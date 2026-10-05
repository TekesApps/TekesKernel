use std::collections::BTreeSet;
use std::fs;
use std::os::fd::OwnedFd;
#[cfg(target_os = "macos")]
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use rustix::fs::{Mode, OFlags, fchmod, fstat, mkdirat, open, openat};
use serde::{Deserialize, Serialize};
use store::{AssetRef, AssetStore, AtomicPublisher, NamedLock};

use crate::{ProfileError, canonical_line, digest, parse_canonical};

const FORMAT: u64 = 1;
const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspacePolicy {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub writable_roots: Vec<String>,
    /// Explicit read-only toolchain installations; their bin directories form tool PATH.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub toolchain_roots: Vec<String>,
    /// Network authority for workspace tools. Provider transport is governed by its configured
    /// connection and reports its own HTTP, authentication, and timeout failures.
    #[serde(default)]
    pub network: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_tools: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_wall_seconds: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceFolder {
    pub id: String,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceConfig {
    pub format: u64,
    pub revision: u64,
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cwd: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<WorkspaceFolder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<WorkspacePolicy>,
}

impl WorkspaceConfig {
    #[must_use]
    pub fn folder_paths(&self) -> Vec<&str> {
        if self.folders.is_empty() {
            self.cwd.iter().map(String::as_str).collect()
        } else {
            self.folders
                .iter()
                .map(|folder| folder.path.as_str())
                .collect()
        }
    }

    #[must_use]
    pub fn binding_for_path(&self, path: &str) -> Option<String> {
        self.folders
            .iter()
            .find(|folder| folder.path == path)
            .map(|folder| folder.id.clone())
            .or_else(|| {
                self.cwd
                    .iter()
                    .position(|candidate| candidate == path)
                    .map(|index| format!("folder-{:04}", index + 1))
            })
    }

    #[must_use]
    pub fn path_for_binding(&self, binding: &str) -> Option<&str> {
        if self.folders.is_empty() {
            self.cwd
                .iter()
                .enumerate()
                .find(|(index, _)| binding == format!("folder-{:04}", index + 1))
                .map(|(_, path)| path.as_str())
        } else {
            self.folders
                .iter()
                .find(|folder| folder.id == binding)
                .map(|folder| folder.path.as_str())
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Model {
    pub id: String,
    pub profile: String,
    pub enabled: bool,
    pub context_window_tokens: u64,
    pub compact_trigger_tokens: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provider {
    pub id: String,
    /// User-facing provider name carried from the model-list item. `id`
    /// remains the stable connection identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub adapter: String,
    pub dialect: String,
    pub endpoint_owner: String,
    pub gateway_translation: String,
    pub evidence_revision: String,
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_key: Option<String>,
    pub models: Vec<Model>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebSearch {
    pub adapter: String,
    pub endpoint: String,
    pub credential_key: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvidersConfig {
    pub format: u64,
    pub revision: u64,
    pub providers: Vec<Provider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search: Option<WebSearch>,
}

impl Default for ProvidersConfig {
    fn default() -> Self {
        Self {
            format: FORMAT,
            revision: 0,
            providers: Vec::new(),
            web_search: None,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_workers: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_provider_leases: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettingsConfig {
    pub format: u64,
    pub revision: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<Limits>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionSettings {
    pub format: u64,
    pub revision: u64,
    pub provider: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            format: FORMAT,
            revision: 0,
            default_provider: None,
            default_model: None,
            limits: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedWorkspace {
    pub format: u64,
    pub revision: u64,
    pub id: String,
    pub name: String,
    /// The stable folder selected for this session snapshot. A workspace-wide
    /// snapshot leaves this absent and retains authored folder order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_binding: Option<String>,
    /// Canonical execution/default cwd paired with `folder_binding`. The
    /// complete ordered `cwd` list remains authored workspace authority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_cwd: Option<String>,
    pub cwd: Vec<String>,
    pub policy: WorkspacePolicy,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RevisionVector {
    pub workspace: u64,
    pub providers: u64,
    pub settings: u64,
    /// Snapshots published before the integrations file was removed carry an
    /// `integrations` revision; it is read and ignored, never written.
    #[serde(
        rename = "integrations",
        default,
        skip_serializing,
        deserialize_with = "ignore_legacy_field"
    )]
    pub legacy_integrations: (),
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_settings: Option<u64>,
}

fn ignore_legacy_field<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
    serde::de::IgnoredAny::deserialize(deserializer).map(|_| ())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigSnapshot {
    pub format: u64,
    pub workspace: ResolvedWorkspace,
    pub providers: ProvidersConfig,
    /// Snapshots published before the integrations file was removed carry an
    /// `integrations` document; it is read and ignored, never written.
    #[serde(
        rename = "integrations",
        default,
        skip_serializing,
        deserialize_with = "ignore_legacy_field"
    )]
    pub legacy_integrations: (),
    pub settings: SettingsConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_settings: Option<SessionSettings>,
    pub revisions: RevisionVector,
}

impl ConfigSnapshot {
    #[must_use]
    pub fn execution_cwd(&self) -> Option<&str> {
        self.workspace
            .selected_cwd
            .as_deref()
            .or_else(|| self.workspace.cwd.first().map(String::as_str))
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ProfileError> {
        validate_snapshot(self)?;
        canonical_line("<config-snapshot>", self)
    }

    pub fn digest(&self) -> Result<String, ProfileError> {
        Ok(digest(&self.canonical_bytes()?))
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ProfileError> {
        let snapshot: Self = parse_canonical("<config-snapshot>", bytes)?;
        if snapshot.format != FORMAT {
            return Err(ProfileError::UnsupportedFormat {
                path: PathBuf::from("<config-snapshot>"),
                format: snapshot.format,
            });
        }
        validate_snapshot(&snapshot)?;
        Ok(snapshot)
    }

    pub fn publish(&self, assets: &AssetStore) -> Result<(String, AssetRef), ProfileError> {
        let bytes = self.canonical_bytes()?;
        let digest = digest(&bytes);
        let reference = assets.publish(&bytes)?;
        if reference.asset != format!("sha256-{digest}") {
            return Err(ProfileError::DigestMismatch {
                expected: digest,
                actual: reference.asset,
            });
        }
        Ok((digest, reference))
    }

    #[must_use]
    pub fn requires_respawn_from(&self, previous: &Self) -> bool {
        let current = &self.workspace.policy;
        let old = &previous.workspace.policy;
        if (old.network && !current.network)
            || removed(&old.allowed_tools, &current.allowed_tools)
            || old.toolchain_roots != current.toolchain_roots
            || removed(&old.writable_roots, &current.writable_roots)
            || lowered(old.max_wall_seconds, current.max_wall_seconds)
            || old.provider != current.provider
            || old.model != current.model
        {
            return true;
        }
        let current_providers = enabled_provider_models(&self.providers);
        let old_providers = enabled_provider_models(&previous.providers);
        !old_providers.is_subset(&current_providers)
            || previous
                .providers
                .web_search
                .as_ref()
                .is_some_and(|old| self.providers.web_search.as_ref() != Some(old))
    }
}

macro_rules! config_document {
    ($type:ty, $validate:ident, $label:literal) => {
        impl $type {
            pub fn decode(bytes: &[u8]) -> Result<Self, ProfileError> {
                let value: Self = parse_canonical($label, bytes)?;
                $validate(&value, false, Path::new($label))?;
                Ok(value)
            }

            pub fn canonical_bytes(&self) -> Result<Vec<u8>, ProfileError> {
                $validate(self, false, Path::new($label))?;
                canonical_line($label, self)
            }
        }
    };
}

config_document!(WorkspaceConfig, validate_workspace, "<workspace-config>");
config_document!(ProvidersConfig, validate_providers, "<providers-config>");
config_document!(SettingsConfig, validate_settings, "<settings-config>");
config_document!(
    SessionSettings,
    validate_session_settings,
    "<session-settings>"
);

#[derive(Clone, Debug)]
pub struct ConfigRepository {
    root: PathBuf,
}

fn invalid_private_directory(path: &Path, reason: &str) -> ProfileError {
    ProfileError::InvalidPath {
        path: path.to_path_buf(),
        reason: reason.to_owned(),
    }
}

fn validate_private_directory_fd(
    descriptor: &OwnedFd,
    path: &Path,
    expected_uid: libc::uid_t,
) -> Result<(), ProfileError> {
    let metadata = fstat(descriptor).map_err(rustix_profile_error)?;
    if metadata.st_mode & libc::S_IFMT != libc::S_IFDIR {
        return Err(invalid_private_directory(
            path,
            "private data directory is not a real directory",
        ));
    }
    if metadata.st_uid != expected_uid {
        return Err(invalid_private_directory(
            path,
            "private data directory has the wrong owner",
        ));
    }
    if metadata.st_mode & 0o022 != 0 {
        return Err(invalid_private_directory(
            path,
            "private data directory is group- or world-writable",
        ));
    }
    // Read-only but privately owned directories are safe to harden in place.
    // Writable-by-others directories were rejected above before this mutation.
    fchmod(descriptor, Mode::RWXU).map_err(rustix_profile_error)?;
    Ok(())
}

fn rustix_profile_error(error: rustix::io::Errno) -> ProfileError {
    std::io::Error::from_raw_os_error(error.raw_os_error()).into()
}

fn open_directory_at(
    parent: &OwnedFd,
    name: &str,
    path: &Path,
    create: bool,
    expected_uid: libc::uid_t,
) -> Result<OwnedFd, ProfileError> {
    let flags = OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC | OFlags::NOFOLLOW;
    let mut result = openat(parent, name, flags, Mode::empty());
    if result
        .as_ref()
        .is_err_and(|error| *error == rustix::io::Errno::NOENT)
        && create
    {
        match mkdirat(parent, name, Mode::RWXU) {
            Ok(()) | Err(rustix::io::Errno::EXIST) => {}
            Err(error) => return Err(rustix_profile_error(error)),
        }
        result = openat(parent, name, flags, Mode::empty());
    }
    let descriptor = match result {
        Ok(descriptor) => descriptor,
        Err(error) if error == rustix::io::Errno::LOOP || error == rustix::io::Errno::NOTDIR => {
            return Err(invalid_private_directory(
                path,
                "private data directory is not a real directory",
            ));
        }
        Err(error) => return Err(rustix_profile_error(error)),
    };
    validate_private_directory_fd(&descriptor, path, expected_uid)?;
    Ok(descriptor)
}

fn open_private_root(
    path: &Path,
    expected_uid: libc::uid_t,
    create_leaf: bool,
) -> Result<OwnedFd, ProfileError> {
    #[cfg(target_os = "macos")]
    if path.is_absolute() {
        return open_private_root_darwin(path, expected_uid, create_leaf);
    }
    #[cfg(target_os = "macos")]
    let traversal_path = path.strip_prefix("/var").map_or_else(
        |_| path.to_path_buf(),
        |suffix| Path::new("/private/var").join(suffix),
    );
    #[cfg(not(target_os = "macos"))]
    let traversal_path = path.to_path_buf();
    let flags = OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC | OFlags::NOFOLLOW;
    let parent = if path.is_absolute() {
        open("/", flags, Mode::empty()).map_err(rustix_profile_error)?
    } else {
        open(".", flags, Mode::empty()).map_err(rustix_profile_error)?
    };
    let mut components = Vec::new();
    for component in traversal_path.components() {
        match component {
            std::path::Component::RootDir | std::path::Component::CurDir => {}
            std::path::Component::Normal(value) => {
                let value = value.to_str().ok_or_else(|| {
                    invalid_private_directory(path, "private data root must be UTF-8")
                })?;
                components.push(value.to_owned());
            }
            std::path::Component::ParentDir | std::path::Component::Prefix(_) => {
                return Err(invalid_private_directory(
                    path,
                    "private data root must be lexically normalized",
                ));
            }
        }
    }
    if components.is_empty() {
        return Err(invalid_private_directory(
            path,
            "private data root must name a directory",
        ));
    }
    let mut descriptor = parent;
    let mut traversed = if path.is_absolute() {
        PathBuf::from("/")
    } else {
        PathBuf::from(".")
    };
    let component_count = components.len();
    for (index, component) in components.into_iter().enumerate() {
        traversed.push(&component);
        let mut result = openat(&descriptor, &component, flags, Mode::empty());
        if result
            .as_ref()
            .is_err_and(|error| *error == rustix::io::Errno::NOENT)
            && create_leaf
            && index + 1 == component_count
        {
            match mkdirat(&descriptor, &component, Mode::RWXU) {
                Ok(()) | Err(rustix::io::Errno::EXIST) => {}
                Err(error) => return Err(rustix_profile_error(error)),
            }
            result = openat(&descriptor, &component, flags, Mode::empty());
        }
        match result {
            Ok(next) => descriptor = next,
            Err(error)
                if error == rustix::io::Errno::LOOP || error == rustix::io::Errno::NOTDIR =>
            {
                return Err(invalid_private_directory(
                    &traversed,
                    "private data root traverses a symlink or non-directory",
                ));
            }
            Err(error) => return Err(rustix_profile_error(error)),
        }
    }
    validate_private_directory_fd(&descriptor, path, expected_uid)?;
    Ok(descriptor)
}

#[cfg(target_os = "macos")]
fn open_private_root_darwin(
    path: &Path,
    expected_uid: libc::uid_t,
    create_leaf: bool,
) -> Result<OwnedFd, ProfileError> {
    let path = path.strip_prefix("/var").map_or_else(
        |_| path.to_path_buf(),
        |suffix| Path::new("/private/var").join(suffix),
    );
    if path.components().any(|component| {
        matches!(
            component,
            std::path::Component::ParentDir | std::path::Component::CurDir
        )
    }) {
        return Err(invalid_private_directory(
            &path,
            "private data root must be lexically normalized",
        ));
    }
    if path.file_name().is_none() {
        return Err(invalid_private_directory(
            &path,
            "private data root must name a directory",
        ));
    }
    // App Sandbox grants the container by its full path but denies reading
    // ancestor directories such as /Users. O_NOFOLLOW_ANY checks the entire
    // path atomically without opening each ancestor for directory listing.
    let open_directory = |path: &Path| -> std::io::Result<OwnedFd> {
        let file = fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW_ANY | libc::O_CLOEXEC)
            .open(path)?;
        Ok(file.into())
    };
    let descriptor = match open_directory(&path) {
        Ok(descriptor) => descriptor,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && create_leaf => {
            let parent_path = path.parent().ok_or_else(|| {
                invalid_private_directory(&path, "private data root has no parent")
            })?;
            let parent = open_directory(parent_path).map_err(ProfileError::from)?;
            let name = path
                .file_name()
                .ok_or_else(|| invalid_private_directory(&path, "private data root has no name"))?;
            match mkdirat(&parent, name, Mode::RWXU) {
                Ok(()) | Err(rustix::io::Errno::EXIST) => {}
                Err(error) => return Err(rustix_profile_error(error)),
            }
            open_directory(&path).map_err(ProfileError::from)?
        }
        Err(error) if error.raw_os_error() == Some(libc::ELOOP) => {
            return Err(invalid_private_directory(
                &path,
                "private data root traverses a symlink",
            ));
        }
        Err(error) => return Err(ProfileError::from(error)),
    };
    validate_private_directory_fd(&descriptor, &path, expected_uid)?;
    Ok(descriptor)
}

fn prepare_repository_directories(root: &Path) -> Result<(), ProfileError> {
    let expected_uid = rustix::process::geteuid().as_raw();
    let root_descriptor = open_private_root(root, expected_uid, true)?;
    for name in ["config", "workspaces"] {
        let _ = open_directory_at(&root_descriptor, name, &root.join(name), true, expected_uid)?;
    }
    Ok(())
}

fn prepare_private_directory(path: &Path) -> Result<(), ProfileError> {
    let parent_path = path
        .parent()
        .ok_or_else(|| invalid_private_directory(path, "private data directory has no parent"))?;
    let name = path
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or_else(|| {
            invalid_private_directory(path, "private data directory name must be UTF-8")
        })?;
    let expected_uid = rustix::process::geteuid().as_raw();
    let parent = open_private_root(parent_path, expected_uid, false)?;
    let _ = open_directory_at(&parent, name, path, true, expected_uid)?;
    Ok(())
}

fn prepare_workspace_directory(root: &Path, workspace_id: &str) -> Result<(), ProfileError> {
    validate_workspace_id(workspace_id)?;
    let directory = root.join("workspaces").join(workspace_id);
    prepare_private_directory(&directory)?;
    for name in ["state", "skills"] {
        prepare_private_directory(&directory.join(name))?;
    }
    Ok(())
}

fn prepare_existing_workspace_directories(root: &Path) -> Result<(), ProfileError> {
    let workspaces = root.join("workspaces");
    for entry in fs::read_dir(&workspaces)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_symlink() || !entry.file_type()?.is_dir() {
            return Err(ProfileError::InvalidPath {
                path,
                reason: "workspace authority entry must be a directory".to_owned(),
            });
        }
        let id = entry
            .file_name()
            .into_string()
            .map_err(|_| ProfileError::InvalidPath {
                path: entry.path(),
                reason: "workspace authority directory must be UTF-8".to_owned(),
            })?;
        prepare_workspace_directory(root, &id)?;
    }
    Ok(())
}

impl ConfigRepository {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, ProfileError> {
        let root = root.as_ref().to_path_buf();
        prepare_repository_directories(&root)?;
        prepare_existing_workspace_directories(&root)?;
        Ok(Self { root })
    }

    pub fn workspace_data_dir(&self, workspace_id: &str) -> Result<PathBuf, ProfileError> {
        validate_workspace_id(workspace_id)?;
        Ok(self.root.join("workspaces").join(workspace_id))
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn resolve(&self, workspace_id: &str) -> Result<ConfigSnapshot, ProfileError> {
        validate_workspace_id(workspace_id)?;
        let _lock = NamedLock::shared(self.root.join("config/.lock"))?;
        self.resolve_unlocked(workspace_id, None, None)
    }

    /// Resolves a workspace snapshot with a selected stable execution folder.
    /// The authored `workspace.cwd` order is unchanged so instruction/resource
    /// capture still spans the complete workspace with stable precedence.
    pub fn resolve_for_binding(
        &self,
        workspace_id: &str,
        folder_binding: &str,
    ) -> Result<ConfigSnapshot, ProfileError> {
        validate_workspace_id(workspace_id)?;
        let _lock = NamedLock::shared(self.root.join("config/.lock"))?;
        self.resolve_unlocked(workspace_id, None, Some(folder_binding))
    }

    pub fn resolve_for_session(
        &self,
        workspace_id: &str,
        session_folder: impl AsRef<Path>,
    ) -> Result<ConfigSnapshot, ProfileError> {
        validate_workspace_id(workspace_id)?;
        let _lock = NamedLock::shared(self.root.join("config/.lock"))?;
        let snapshot = self.resolve_unlocked(workspace_id, Some(session_folder.as_ref()), None)?;
        if snapshot.workspace.cwd.len() != 1 {
            return Err(ProfileError::InvalidReference {
                path: session_folder.as_ref().to_path_buf(),
                reason: "multi-folder session has no stable folder binding".to_owned(),
            });
        }
        Ok(snapshot)
    }

    /// Session-local equivalent of `resolve_for_binding`; session settings are
    /// read from `session_folder` while the stable binding selects the default
    /// execution root.
    pub fn resolve_for_session_binding(
        &self,
        workspace_id: &str,
        session_folder: impl AsRef<Path>,
        folder_binding: &str,
    ) -> Result<ConfigSnapshot, ProfileError> {
        validate_workspace_id(workspace_id)?;
        let _lock = NamedLock::shared(self.root.join("config/.lock"))?;
        self.resolve_unlocked(
            workspace_id,
            Some(session_folder.as_ref()),
            Some(folder_binding),
        )
    }

    /// Reads the durable provider authority without synthesizing or writing a
    /// missing file. The revision-zero default is returned in memory.
    pub fn providers(&self) -> Result<ProvidersConfig, ProfileError> {
        let _lock = NamedLock::shared(self.root.join("config/.lock"))?;
        let path = self.root.join("config/providers.json");
        let value = self.read_optional(&path)?.unwrap_or_default();
        validate_providers(&value, true, &path)?;
        Ok(value)
    }

    /// Reads the durable global settings authority without creating it.
    pub fn settings(&self) -> Result<SettingsConfig, ProfileError> {
        let _lock = NamedLock::shared(self.root.join("config/.lock"))?;
        let path = self.root.join("config/settings.json");
        let value = self.read_optional(&path)?.unwrap_or_default();
        validate_settings(&value, true, &path)?;
        Ok(value)
    }

    /// Reads one authored workspace document rather than its resolved spawn
    /// projection.
    pub fn workspace(&self, workspace_id: &str) -> Result<WorkspaceConfig, ProfileError> {
        validate_workspace_id(workspace_id)?;
        let _lock = NamedLock::shared(self.root.join("config/.lock"))?;
        let path = workspace_document_path(&self.root, workspace_id);
        let value: WorkspaceConfig = self.read_required(&path)?;
        validate_workspace(&value, false, &path)?;
        if value.id != workspace_id {
            return invalid(&path, "workspace id does not match filename");
        }
        Ok(value)
    }

    /// Reads every authored workspace document in stable workspace-id order.
    /// Directory names are part of the authority and must match the document
    /// identity; malformed entries fail the complete read.
    pub fn workspaces(&self) -> Result<Vec<WorkspaceConfig>, ProfileError> {
        let _lock = NamedLock::shared(self.root.join("config/.lock"))?;
        let mut entries =
            fs::read_dir(self.root.join("workspaces"))?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(fs::DirEntry::file_name);
        let mut values = Vec::with_capacity(entries.len());
        for entry in entries {
            let directory = entry.path();
            if entry.file_type()?.is_symlink() || !entry.file_type()?.is_dir() {
                return invalid(&directory, "workspace authority entry must be a directory");
            }
            let workspace_id =
                entry
                    .file_name()
                    .into_string()
                    .map_err(|_| ProfileError::InvalidPath {
                        path: directory.clone(),
                        reason: "workspace authority directory must be UTF-8".to_owned(),
                    })?;
            validate_workspace_id(&workspace_id)?;
            let path = directory.join("workspace.json");
            let value: WorkspaceConfig = self.read_required(&path)?;
            validate_workspace(&value, false, &path)?;
            if value.id != workspace_id {
                return invalid(&path, "workspace id does not match directory name");
            }
            values.push(value);
        }
        Ok(values)
    }

    /// Returns the same canonical roots without erasing workspace ownership.
    /// Client resource references use this view so a project skill cannot be
    /// rebound under another workspace id.
    pub fn resource_workspace_roots(&self) -> Result<Vec<(String, Vec<PathBuf>)>, ProfileError> {
        let _lock = NamedLock::shared(self.root.join("config/.lock"))?;
        let directory = self.root.join("workspaces");
        let mut entries = fs::read_dir(&directory)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(fs::DirEntry::file_name);
        let mut workspaces = Vec::new();
        for entry in entries {
            let directory_path = entry.path();
            if !entry.file_type()?.is_dir() {
                return Err(ProfileError::InvalidPath {
                    path: directory_path,
                    reason: "workspace authority entry must be a directory".to_owned(),
                });
            }
            let workspace_id = directory_path
                .file_name()
                .and_then(|value| value.to_str())
                .ok_or_else(|| ProfileError::InvalidPath {
                    path: directory_path.clone(),
                    reason: "workspace authority directory must be UTF-8".to_owned(),
                })?;
            validate_workspace_id(workspace_id)?;
            let path = workspace_document_path(&self.root, workspace_id);
            let workspace: WorkspaceConfig = self.read_required(&path)?;
            validate_workspace(&workspace, false, &path)?;
            if workspace.id != workspace_id {
                return Err(ProfileError::InvalidSchema {
                    path,
                    reason: "workspace id does not match filename".to_owned(),
                });
            }
            let roots = resolve_workspace(&workspace, &path)?
                .cwd
                .into_iter()
                .map(PathBuf::from)
                .collect::<Vec<_>>();
            workspaces.push((workspace_id.to_owned(), roots));
        }
        Ok(workspaces)
    }

    pub fn publish_workspace(
        &self,
        expected_revision: u64,
        value: &WorkspaceConfig,
    ) -> Result<(), ProfileError> {
        validate_workspace_id(&value.id)?;
        let path = workspace_document_path(&self.root, &value.id);
        let _lock = NamedLock::exclusive(self.root.join("config/.lock"))?;
        prepare_workspace_directory(&self.root, &value.id)?;
        let actual = match self.read_optional::<WorkspaceConfig>(&path)? {
            Some(current) => {
                validate_workspace(&current, false, &path)?;
                current.revision
            }
            None => 0,
        };
        check_publication_revision(expected_revision, actual, value.revision)?;
        validate_workspace(value, false, &path)?;
        AtomicPublisher::replace(&path, &canonical_line(&path, value)?)?;
        Ok(())
    }

    pub fn publish_providers(
        &self,
        expected_revision: u64,
        value: &ProvidersConfig,
    ) -> Result<(), ProfileError> {
        let path = self.root.join("config/providers.json");
        self.publish_global(expected_revision, value.revision, &path, value, |value| {
            validate_providers(value, false, &path)
        })
    }

    /// Publishes a provider replacement only when every workspace, global
    /// default, and active/archived session reference remains valid. The scan
    /// and replacement share the single config lock, closing delete/reference
    /// races for management callers.
    pub fn publish_providers_checked(
        &self,
        expected_revision: u64,
        value: &ProvidersConfig,
    ) -> Result<(), ProfileError> {
        let path = self.root.join("config/providers.json");
        // Folder moves take the management lock before their own config work.
        // Holding its shared side across the active+archive scan prevents an
        // unarchive from moving a referencing session between the two scans.
        let management_path = store::endpoint_management_root(&self.root)?.join("lock");
        let _management = management_path
            .exists()
            .then(|| NamedLock::shared(&management_path))
            .transpose()?;
        let _lock = NamedLock::exclusive(self.root.join("config/.lock"))?;
        let actual = self
            .read_optional::<ProvidersConfig>(&path)?
            .unwrap_or_default();
        validate_providers(&actual, true, &path)?;
        check_publication_revision(expected_revision, actual.revision, value.revision)?;
        validate_providers(value, false, &path)?;
        self.validate_all_provider_references_unlocked(value)?;
        AtomicPublisher::replace(&path, &canonical_line(&path, value)?)?;
        Ok(())
    }

    pub fn publish_settings(
        &self,
        expected_revision: u64,
        value: &SettingsConfig,
    ) -> Result<(), ProfileError> {
        let path = self.root.join("config/settings.json");
        self.publish_global(expected_revision, value.revision, &path, value, |value| {
            validate_settings(value, false, &path)
        })
    }

    /// Publishes settings with its provider/model reference checked under the
    /// same config lock as the replace.
    pub fn publish_settings_checked(
        &self,
        expected_revision: u64,
        value: &SettingsConfig,
    ) -> Result<(), ProfileError> {
        let path = self.root.join("config/settings.json");
        let providers_path = self.root.join("config/providers.json");
        let _lock = NamedLock::exclusive(self.root.join("config/.lock"))?;
        let actual = self
            .read_optional::<SettingsConfig>(&path)?
            .unwrap_or_default();
        validate_settings(&actual, true, &path)?;
        check_publication_revision(expected_revision, actual.revision, value.revision)?;
        validate_settings(value, false, &path)?;
        let providers = self
            .read_optional::<ProvidersConfig>(&providers_path)?
            .unwrap_or_default();
        validate_providers(&providers, true, &providers_path)?;
        let empty_workspace = WorkspaceConfig {
            format: FORMAT,
            revision: 1,
            id: "reference-check".to_owned(),
            name: "reference-check".to_owned(),
            cwd: vec!["/".to_owned()],
            folders: Vec::new(),
            policy: None,
        };
        validate_references(&empty_workspace, &providers, value, &path)?;
        AtomicPublisher::replace(&path, &canonical_line(&path, value)?)?;
        Ok(())
    }

    /// Replaces one workspace-local policy while preserving all other authored
    /// fields. Provider references and the publication revision are checked
    /// under the same exclusive lock.
    pub fn publish_workspace_policy(
        &self,
        workspace_id: &str,
        expected_revision: u64,
        policy: WorkspacePolicy,
    ) -> Result<WorkspaceConfig, ProfileError> {
        validate_workspace_id(workspace_id)?;
        let path = workspace_document_path(&self.root, workspace_id);
        let providers_path = self.root.join("config/providers.json");
        let settings_path = self.root.join("config/settings.json");
        let _lock = NamedLock::exclusive(self.root.join("config/.lock"))?;
        let mut value: WorkspaceConfig = self.read_required(&path)?;
        validate_workspace(&value, false, &path)?;
        let next_revision =
            value
                .revision
                .checked_add(1)
                .ok_or_else(|| ProfileError::InvalidSchema {
                    path: path.clone(),
                    reason: "workspace revision overflow".to_owned(),
                })?;
        check_publication_revision(expected_revision, value.revision, next_revision)?;
        value.revision = next_revision;
        value.policy = Some(policy);
        validate_workspace(&value, false, &path)?;
        let providers = self
            .read_optional::<ProvidersConfig>(&providers_path)?
            .unwrap_or_default();
        let settings = self
            .read_optional::<SettingsConfig>(&settings_path)?
            .unwrap_or_default();
        validate_providers(&providers, true, &providers_path)?;
        validate_settings(&settings, true, &settings_path)?;
        validate_references(&value, &providers, &settings, &path)?;
        AtomicPublisher::replace(&path, &canonical_line(&path, &value)?)?;
        Ok(value)
    }

    pub fn publish_session_settings(
        &self,
        session_folder: impl AsRef<Path>,
        expected_revision: u64,
        value: &SessionSettings,
    ) -> Result<(), ProfileError> {
        let path = store::session_settings_path(session_folder.as_ref())?;
        let _lock = NamedLock::exclusive(self.root.join("config/.lock"))?;
        let actual = match self.read_optional::<SessionSettings>(&path)? {
            Some(current) => {
                validate_session_settings(&current, false, &path)?;
                current.revision
            }
            None => 0,
        };
        check_publication_revision(expected_revision, actual, value.revision)?;
        validate_session_settings(value, false, &path)?;
        let providers_path = self.root.join("config/providers.json");
        let providers = self.read_optional(&providers_path)?.unwrap_or_default();
        validate_providers(&providers, true, &providers_path)?;
        validate_session_reference(&Some(value.clone()), &providers, &path)?;
        AtomicPublisher::replace(&path, &canonical_line(&path, value)?)?;
        Ok(())
    }

    pub fn session_settings(
        &self,
        session_folder: impl AsRef<Path>,
    ) -> Result<Option<SessionSettings>, ProfileError> {
        let _lock = NamedLock::shared(self.root.join("config/.lock"))?;
        let path = store::session_settings_path(session_folder.as_ref())?;
        let value = self.read_optional::<SessionSettings>(&path)?;
        if let Some(value) = &value {
            validate_session_settings(value, false, &path)?;
        }
        Ok(value)
    }

    fn publish_global<T: Serialize + for<'de> Deserialize<'de>>(
        &self,
        expected_revision: u64,
        next_revision: u64,
        path: &Path,
        value: &T,
        validate: impl Fn(&T) -> Result<(), ProfileError>,
    ) -> Result<(), ProfileError> {
        let _lock = NamedLock::exclusive(self.root.join("config/.lock"))?;
        let actual = match fs::read(path) {
            Ok(bytes) => {
                let current: T = parse_canonical(path, &bytes)?;
                validate(&current)?;
                revision_from_value(path, &current)?
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => 0,
            Err(error) => return Err(error.into()),
        };
        check_publication_revision(expected_revision, actual, next_revision)?;
        validate(value)?;
        AtomicPublisher::replace(path, &canonical_line(path, value)?)?;
        Ok(())
    }

    fn resolve_unlocked(
        &self,
        workspace_id: &str,
        session_folder: Option<&Path>,
        folder_binding: Option<&str>,
    ) -> Result<ConfigSnapshot, ProfileError> {
        let workspace_path = workspace_document_path(&self.root, workspace_id);
        let workspace: WorkspaceConfig = self.read_required(&workspace_path)?;
        if workspace.id != workspace_id {
            return Err(ProfileError::InvalidSchema {
                path: workspace_path,
                reason: "workspace id does not match filename".to_owned(),
            });
        }
        let providers_path = self.root.join("config/providers.json");
        let settings_path = self.root.join("config/settings.json");
        let providers = self.read_optional(&providers_path)?.unwrap_or_default();
        let settings = self.read_optional(&settings_path)?.unwrap_or_default();
        let session_settings_path = session_folder
            .map(store::session_settings_path)
            .transpose()?;
        let session_settings = match session_settings_path.as_ref() {
            Some(path) => self.read_optional::<SessionSettings>(path)?,
            None => None,
        };

        validate_workspace(&workspace, false, &workspace_path)?;
        validate_providers(&providers, true, &providers_path)?;
        validate_settings(&settings, true, &settings_path)?;
        if let (Some(value), Some(path)) = (&session_settings, &session_settings_path) {
            validate_session_settings(value, false, path)?;
        }
        validate_references(&workspace, &providers, &settings, &workspace_path)?;
        validate_session_reference(
            &session_settings,
            &providers,
            session_settings_path.as_deref().unwrap_or(&workspace_path),
        )?;
        let mut resolved = resolve_workspace(&workspace, &workspace_path)?;
        if let Some(binding) = folder_binding {
            validate_id(binding, "folder binding id", &workspace_path)?;
            let selected = workspace.path_for_binding(binding).ok_or_else(|| {
                ProfileError::InvalidReference {
                    path: workspace_path.clone(),
                    reason: format!("unknown folder binding {binding}"),
                }
            })?;
            let selected = canonical_directory(selected, &workspace_path)?;
            if !resolved.cwd.contains(&selected) {
                return Err(ProfileError::InvalidReference {
                    path: workspace_path.clone(),
                    reason: format!("folder binding {binding} has no resolved cwd"),
                });
            }
            resolved.folder_binding = Some(binding.to_owned());
            resolved.selected_cwd = Some(selected);
        }
        Ok(ConfigSnapshot {
            format: FORMAT,
            revisions: RevisionVector {
                workspace: workspace.revision,
                providers: providers.revision,
                settings: settings.revision,
                session_settings: session_settings.as_ref().map(|value| value.revision),
                legacy_integrations: (),
            },
            workspace: resolved,
            providers,
            legacy_integrations: (),
            settings,
            session_settings,
        })
    }

    fn read_required<T: for<'de> Deserialize<'de>>(&self, path: &Path) -> Result<T, ProfileError> {
        parse_canonical(path, &fs::read(path)?)
    }

    fn read_optional<T: for<'de> Deserialize<'de>>(
        &self,
        path: &Path,
    ) -> Result<Option<T>, ProfileError> {
        match fs::read(path) {
            Ok(bytes) => Ok(Some(parse_canonical(path, &bytes)?)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    fn validate_all_provider_references_unlocked(
        &self,
        providers: &ProvidersConfig,
    ) -> Result<(), ProfileError> {
        let settings_path = self.root.join("config/settings.json");
        let settings = self
            .read_optional::<SettingsConfig>(&settings_path)?
            .unwrap_or_default();
        validate_settings(&settings, true, &settings_path)?;

        let mut workspace_entries =
            fs::read_dir(self.root.join("workspaces"))?.collect::<Result<Vec<_>, _>>()?;
        workspace_entries.sort_by_key(fs::DirEntry::file_name);
        for entry in workspace_entries {
            if !entry.file_type()?.is_dir() {
                return invalid(
                    &entry.path(),
                    "workspace authority entry must be a directory",
                );
            }
            let workspace_path = entry.path().join("workspace.json");
            let workspace: WorkspaceConfig = self.read_required(&workspace_path)?;
            validate_workspace(&workspace, false, &workspace_path)?;
            validate_references(&workspace, providers, &settings, &workspace_path)?;
        }

        for area in ["threads", "archive"] {
            let root = self.root.join(area);
            let entries = match fs::read_dir(&root) {
                Ok(entries) => entries.collect::<Result<Vec<_>, _>>()?,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => return Err(error.into()),
            };
            for entry in entries {
                if !entry.file_type()?.is_dir() {
                    continue;
                }
                let settings_path = store::session_settings_path(&entry.path())?;
                if let Some(session) = self.read_optional::<SessionSettings>(&settings_path)? {
                    validate_session_settings(&session, false, &settings_path)?;
                    validate_session_reference(&Some(session), providers, &settings_path)?;
                }
            }
        }
        Ok(())
    }
}

fn check_publication_revision(expected: u64, actual: u64, next: u64) -> Result<(), ProfileError> {
    validate_safe_integer(expected, "expected revision", Path::new("<publication>"))?;
    validate_safe_integer(actual, "actual revision", Path::new("<publication>"))?;
    validate_safe_integer(next, "next revision", Path::new("<publication>"))?;
    if expected != actual {
        return Err(ProfileError::StaleRevision { expected, actual });
    }
    let successor = actual
        .checked_add(1)
        .ok_or_else(|| ProfileError::InvalidSchema {
            path: PathBuf::from("<publication>"),
            reason: "revision cannot advance beyond the I-JSON safe-integer range".to_owned(),
        })?;
    if next != successor {
        return Err(ProfileError::InvalidSchema {
            path: PathBuf::from("<publication>"),
            reason: format!("next revision must be {successor}, found {next}"),
        });
    }
    Ok(())
}

fn revision_from_value<T: Serialize>(path: &Path, value: &T) -> Result<u64, ProfileError> {
    #[derive(Deserialize)]
    struct RevisionOnly {
        revision: u64,
    }
    let value = serde_json::to_value(value).map_err(|error| ProfileError::InvalidSchema {
        path: path.to_path_buf(),
        reason: error.to_string(),
    })?;
    serde_json::from_value::<RevisionOnly>(value)
        .map(|value| value.revision)
        .map_err(|error| ProfileError::InvalidSchema {
            path: path.to_path_buf(),
            reason: error.to_string(),
        })
}

fn validate_workspace_id(id: &str) -> Result<(), ProfileError> {
    let valid = !id.is_empty()
        && id.len() <= 128
        && !id.starts_with('.')
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'));
    if valid {
        Ok(())
    } else {
        Err(ProfileError::InvalidPath {
            path: PathBuf::from(id),
            reason: "workspace id is outside [A-Za-z0-9._-]".to_owned(),
        })
    }
}

fn workspace_document_path(root: &Path, workspace_id: &str) -> PathBuf {
    root.join("workspaces")
        .join(workspace_id)
        .join("workspace.json")
}

fn validate_common(
    format: u64,
    revision: u64,
    allow_zero: bool,
    path: &Path,
) -> Result<(), ProfileError> {
    if format != FORMAT {
        return Err(ProfileError::UnsupportedFormat {
            path: path.to_path_buf(),
            format,
        });
    }
    if revision == 0 && !allow_zero {
        return Err(ProfileError::InvalidSchema {
            path: path.to_path_buf(),
            reason: "stored revision must be at least 1".to_owned(),
        });
    }
    validate_safe_integer(revision, "revision", path)?;
    Ok(())
}

fn validate_workspace(
    value: &WorkspaceConfig,
    allow_zero: bool,
    path: &Path,
) -> Result<(), ProfileError> {
    validate_common(value.format, value.revision, allow_zero, path)?;
    validate_workspace_id(&value.id)?;
    validate_id(&value.name, "workspace name", path)?;
    if value.cwd.is_empty() && value.folders.is_empty() {
        return invalid(path, "workspace folders must be nonempty");
    }
    if !value.cwd.is_empty() && !value.folders.is_empty() {
        return invalid(path, "workspace cannot contain both legacy cwd and folders");
    }
    let mut binding_ids = BTreeSet::new();
    for folder in &value.folders {
        validate_id(&folder.id, "folder binding id", path)?;
        if !binding_ids.insert(folder.id.as_str()) {
            return invalid(path, "duplicate folder binding id");
        }
        validate_absolute(&folder.path, path)?;
    }
    for cwd in value.folder_paths() {
        validate_absolute(cwd, path)?;
    }
    if let Some(policy) = &value.policy {
        for root in policy.writable_roots.iter().chain(&policy.toolchain_roots) {
            validate_absolute(root, path)?;
        }
        validate_unique(&policy.allowed_tools, "allowed_tools", path)?;
        if policy.max_wall_seconds == Some(0) {
            return invalid(path, "max_wall_seconds must be at least 1");
        }
        if let Some(value) = policy.max_wall_seconds {
            validate_safe_integer(value, "max_wall_seconds", path)?;
        }
    }
    Ok(())
}

fn validate_providers(
    value: &ProvidersConfig,
    allow_zero: bool,
    path: &Path,
) -> Result<(), ProfileError> {
    validate_common(value.format, value.revision, allow_zero, path)?;
    let mut ids = BTreeSet::new();
    for provider in &value.providers {
        validate_id(&provider.id, "provider id", path)?;
        if let Some(name) = &provider.name {
            validate_id(name, "provider name", path)?;
        }
        validate_id(&provider.adapter, "provider adapter", path)?;
        validate_id(&provider.dialect, "provider dialect", path)?;
        validate_id(&provider.endpoint_owner, "provider endpoint owner", path)?;
        validate_id(
            &provider.gateway_translation,
            "provider gateway translation",
            path,
        )?;
        validate_id(
            &provider.evidence_revision,
            "provider evidence revision",
            path,
        )?;
        validate_id(&provider.endpoint, "provider endpoint", path)?;
        if !ids.insert(provider.id.as_str()) {
            return invalid(path, "duplicate provider id");
        }
        if let Some(key) = &provider.credential_key {
            validate_id(key, "credential key id", path)?;
        }
        let mut models = BTreeSet::new();
        for model in &provider.models {
            validate_id(&model.id, "model id", path)?;
            validate_id(&model.profile, "model profile", path)?;
            validate_safe_integer(model.context_window_tokens, "context_window_tokens", path)?;
            validate_safe_integer(model.compact_trigger_tokens, "compact_trigger_tokens", path)?;
            if model.context_window_tokens < 2
                || model.compact_trigger_tokens == 0
                || model.compact_trigger_tokens >= model.context_window_tokens
            {
                return invalid(
                    path,
                    "compact_trigger_tokens must be positive and less than context_window_tokens",
                );
            }
            if !models.insert(model.id.as_str()) {
                return invalid(path, "duplicate model id");
            }
        }
    }
    if let Some(search) = &value.web_search {
        if search.adapter != "tavily_v1" {
            return invalid(path, "web_search adapter must be tavily_v1");
        }
        validate_id(&search.credential_key, "web_search credential key id", path)?;
        validate_web_search_origin(&search.endpoint, path)?;
    }
    Ok(())
}

fn validate_web_search_origin(value: &str, path: &Path) -> Result<(), ProfileError> {
    let parsed = url::Url::parse(value).map_err(|_| ProfileError::InvalidPath {
        path: path.to_path_buf(),
        reason: "web_search endpoint is not an absolute URL".to_owned(),
    })?;
    if parsed.scheme() != "https"
        || parsed.username() != ""
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || parsed.path() != "/"
        || parsed.host_str().is_none()
        || value.ends_with('/')
    {
        return invalid(
            path,
            "web_search endpoint must be an HTTPS origin without path, query, fragment, userinfo, or trailing slash",
        );
    }
    let host = parsed.host_str().expect("checked above");
    let address_literal = host
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .unwrap_or(host);
    if host.eq_ignore_ascii_case("localhost")
        || host.ends_with(".localhost")
        || address_literal
            .parse::<std::net::IpAddr>()
            .is_ok_and(|address| !tools::is_public_internet_address(address))
    {
        return invalid(path, "web_search endpoint must name a public origin");
    }
    Ok(())
}

fn validate_settings(
    value: &SettingsConfig,
    allow_zero: bool,
    path: &Path,
) -> Result<(), ProfileError> {
    validate_common(value.format, value.revision, allow_zero, path)?;
    if let Some(limits) = &value.limits {
        if limits.max_workers == Some(0) || limits.max_provider_leases == Some(0) {
            return invalid(path, "settings limits must be at least 1");
        }
        for (label, limit) in [
            ("max_workers", limits.max_workers),
            ("max_provider_leases", limits.max_provider_leases),
        ] {
            if let Some(limit) = limit {
                validate_safe_integer(limit, label, path)?;
            }
        }
    }
    Ok(())
}

fn validate_session_settings(
    value: &SessionSettings,
    _allow_default: bool,
    path: &Path,
) -> Result<(), ProfileError> {
    validate_common(value.format, value.revision, false, path)?;
    validate_id(&value.provider, "session provider", path)?;
    validate_id(&value.model, "session model", path)?;
    if value.reasoning_effort.as_deref().is_some_and(str::is_empty) {
        return invalid(path, "reasoning_effort must be nonempty");
    }
    Ok(())
}

fn validate_session_reference(
    settings: &Option<SessionSettings>,
    providers: &ProvidersConfig,
    path: &Path,
) -> Result<(), ProfileError> {
    let Some(settings) = settings else {
        return Ok(());
    };
    let provider = providers
        .providers
        .iter()
        .find(|provider| provider.id == settings.provider)
        .ok_or_else(|| ProfileError::InvalidReference {
            path: path.to_path_buf(),
            reason: format!("unknown provider {}", settings.provider),
        })?;
    if !provider
        .models
        .iter()
        .any(|model| model.id == settings.model && model.enabled)
    {
        return invalid_reference(path, "session model is absent or disabled");
    }
    Ok(())
}

fn validate_references(
    workspace: &WorkspaceConfig,
    providers: &ProvidersConfig,
    settings: &SettingsConfig,
    path: &Path,
) -> Result<(), ProfileError> {
    for (provider, model) in [
        (
            settings.default_provider.as_deref(),
            settings.default_model.as_deref(),
        ),
        (
            workspace
                .policy
                .as_ref()
                .and_then(|value| value.provider.as_deref()),
            workspace
                .policy
                .as_ref()
                .and_then(|value| value.model.as_deref()),
        ),
    ] {
        let Some(provider_id) = provider else {
            if model.is_some() {
                return invalid_reference(path, "model is set without a provider");
            }
            continue;
        };
        let provider = providers
            .providers
            .iter()
            .find(|value| value.id == provider_id)
            .ok_or_else(|| ProfileError::InvalidReference {
                path: path.to_path_buf(),
                reason: format!("unknown provider {provider_id}"),
            })?;
        if let Some(model_id) = model {
            if !provider
                .models
                .iter()
                .any(|value| value.id == model_id && value.enabled)
            {
                return invalid_reference(path, "model is absent or disabled");
            }
        }
    }
    Ok(())
}

fn resolve_workspace(
    value: &WorkspaceConfig,
    path: &Path,
) -> Result<ResolvedWorkspace, ProfileError> {
    let mut cwd = Vec::new();
    for item in value.folder_paths() {
        let resolved = canonical_directory(item, path)?;
        if cwd.contains(&resolved) {
            return invalid(path, "duplicate canonical cwd");
        }
        cwd.push(resolved);
    }
    let mut policy = value.policy.clone().unwrap_or_default();
    let mut roots = Vec::new();
    for item in &policy.writable_roots {
        let resolved = canonical_directory(item, path)?;
        if !cwd
            .iter()
            .any(|base| Path::new(&resolved).starts_with(base))
        {
            return invalid(path, "writable root is outside every cwd");
        }
        roots.push(resolved);
    }
    roots.sort();
    roots.dedup();
    policy.writable_roots = roots;
    let mut toolchains = Vec::new();
    for root in &policy.toolchain_roots {
        let canonical = canonical_directory(root, path)?;
        if canonical.contains(':') {
            return invalid(path, "toolchain root cannot contain PATH separator");
        }
        toolchains.push(canonical);
    }
    toolchains.sort();
    toolchains.dedup();
    policy.toolchain_roots = toolchains;
    policy.allowed_tools.sort();
    policy.allowed_tools.dedup();
    Ok(ResolvedWorkspace {
        format: value.format,
        revision: value.revision,
        id: value.id.clone(),
        name: value.name.clone(),
        folder_binding: None,
        selected_cwd: None,
        cwd,
        policy,
    })
}

fn validate_snapshot(snapshot: &ConfigSnapshot) -> Result<(), ProfileError> {
    let path = Path::new("<config-snapshot>");
    if snapshot.format != FORMAT {
        return Err(ProfileError::UnsupportedFormat {
            path: path.to_path_buf(),
            format: snapshot.format,
        });
    }
    validate_common(
        snapshot.workspace.format,
        snapshot.workspace.revision,
        false,
        path,
    )?;
    validate_workspace_id(&snapshot.workspace.id)?;
    validate_id(&snapshot.workspace.name, "workspace name", path)?;
    if let Some(binding) = &snapshot.workspace.folder_binding {
        validate_id(binding, "folder binding id", path)?;
    }
    match (
        snapshot.workspace.folder_binding.as_ref(),
        snapshot.workspace.selected_cwd.as_ref(),
    ) {
        (Some(_), Some(selected)) => {
            validate_absolute(selected, path)?;
            if canonical_directory(selected, path)? != *selected {
                return invalid(path, "selected cwd is not canonical");
            }
            if !snapshot.workspace.cwd.contains(selected) {
                return invalid(path, "selected cwd is not a bound workspace cwd");
            }
        }
        (None, None) => {}
        _ => return invalid(path, "folder binding and selected cwd must appear together"),
    }
    if snapshot.workspace.cwd.is_empty() {
        return invalid(path, "snapshot workspace cwd must be nonempty");
    }
    validate_unique(&snapshot.workspace.cwd, "snapshot cwd", path)?;
    for cwd in &snapshot.workspace.cwd {
        validate_absolute(cwd, path)?;
        if canonical_directory(cwd, path)? != *cwd {
            return invalid(path, "snapshot cwd is not canonical");
        }
    }
    if !is_sorted_unique(&snapshot.workspace.policy.toolchain_roots)
        || !is_sorted_unique(&snapshot.workspace.policy.writable_roots)
        || !is_sorted_unique(&snapshot.workspace.policy.allowed_tools)
    {
        return invalid(path, "resolved policy sets must be sorted and unique");
    }
    for root in &snapshot.workspace.policy.toolchain_roots {
        validate_absolute(root, path)?;
        if root.contains(':') || canonical_directory(root, path)? != *root {
            return invalid(
                path,
                "resolved toolchain root is not canonical or contains PATH separator",
            );
        }
    }
    for root in &snapshot.workspace.policy.writable_roots {
        validate_absolute(root, path)?;
        if canonical_directory(root, path)? != *root {
            return invalid(path, "resolved writable root is not canonical");
        }
        if !snapshot
            .workspace
            .cwd
            .iter()
            .any(|cwd| Path::new(root).starts_with(cwd))
        {
            return invalid(path, "resolved writable root is outside every cwd");
        }
    }
    if let Some(value) = snapshot.workspace.policy.max_wall_seconds {
        if value == 0 {
            return invalid(path, "max_wall_seconds must be at least 1");
        }
        validate_safe_integer(value, "max_wall_seconds", path)?;
    }
    validate_providers(&snapshot.providers, true, path)?;
    validate_settings(&snapshot.settings, true, path)?;
    if let Some(session) = &snapshot.session_settings {
        validate_session_settings(session, false, path)?;
    }
    if snapshot.providers.revision == 0 && snapshot.providers != ProvidersConfig::default() {
        return invalid(
            path,
            "revision-0 providers must equal the synthesized default",
        );
    }
    if snapshot.settings.revision == 0 && snapshot.settings != SettingsConfig::default() {
        return invalid(
            path,
            "revision-0 settings must equal the synthesized default",
        );
    }
    let workspace = WorkspaceConfig {
        format: snapshot.workspace.format,
        revision: snapshot.workspace.revision,
        id: snapshot.workspace.id.clone(),
        name: snapshot.workspace.name.clone(),
        cwd: snapshot.workspace.cwd.clone(),
        folders: Vec::new(),
        policy: Some(snapshot.workspace.policy.clone()),
    };
    validate_references(&workspace, &snapshot.providers, &snapshot.settings, path)?;
    validate_session_reference(&snapshot.session_settings, &snapshot.providers, path)?;
    if snapshot.revisions.workspace != snapshot.workspace.revision
        || snapshot.revisions.providers != snapshot.providers.revision
        || snapshot.revisions.settings != snapshot.settings.revision
        || snapshot.revisions.session_settings
            != snapshot
                .session_settings
                .as_ref()
                .map(|settings| settings.revision)
    {
        return invalid(
            path,
            "snapshot revision vector does not match embedded documents",
        );
    }
    Ok(())
}

fn validate_safe_integer(value: u64, label: &str, path: &Path) -> Result<(), ProfileError> {
    if value <= MAX_SAFE_INTEGER {
        Ok(())
    } else {
        invalid(
            path,
            &format!("{label} exceeds the I-JSON safe-integer range"),
        )
    }
}

fn is_sorted_unique(values: &[String]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn canonical_directory(value: &str, source: &Path) -> Result<String, ProfileError> {
    validate_absolute(value, source)?;
    let canonical = fs::canonicalize(value).map_err(|error| ProfileError::InvalidPath {
        path: PathBuf::from(value),
        reason: error.to_string(),
    })?;
    if !canonical.is_dir() {
        return Err(ProfileError::InvalidPath {
            path: canonical,
            reason: "expected a directory".to_owned(),
        });
    }
    canonical
        .to_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| ProfileError::InvalidPath {
            path: canonical,
            reason: "path is not UTF-8".to_owned(),
        })
}

fn validate_absolute(value: &str, source: &Path) -> Result<(), ProfileError> {
    if value.contains('\0') || !Path::new(value).is_absolute() {
        return Err(ProfileError::InvalidPath {
            path: source.to_path_buf(),
            reason: format!("path is not absolute UTF-8: {value:?}"),
        });
    }
    Ok(())
}

fn validate_id(value: &str, label: &str, path: &Path) -> Result<(), ProfileError> {
    if value.is_empty() || value.chars().any(char::is_control) {
        invalid(
            path,
            &format!("{label} is empty or contains control characters"),
        )
    } else {
        Ok(())
    }
}

fn validate_unique(values: &[String], label: &str, path: &Path) -> Result<(), ProfileError> {
    let unique = values.iter().collect::<BTreeSet<_>>();
    if unique.len() == values.len() {
        Ok(())
    } else {
        invalid(path, &format!("duplicate {label}"))
    }
}

fn invalid<T>(path: &Path, reason: &str) -> Result<T, ProfileError> {
    Err(ProfileError::InvalidSchema {
        path: path.to_path_buf(),
        reason: reason.to_owned(),
    })
}

fn invalid_reference<T>(path: &Path, reason: &str) -> Result<T, ProfileError> {
    Err(ProfileError::InvalidReference {
        path: path.to_path_buf(),
        reason: reason.to_owned(),
    })
}

fn removed(previous: &[String], current: &[String]) -> bool {
    let current = current.iter().collect::<BTreeSet<_>>();
    previous.iter().any(|value| !current.contains(value))
}

fn lowered(previous: Option<u64>, current: Option<u64>) -> bool {
    match (previous, current) {
        (Some(previous), Some(current)) => current < previous,
        (None, Some(_)) => true,
        (Some(_), None) | (None, None) => false,
    }
}

fn enabled_provider_models(providers: &ProvidersConfig) -> BTreeSet<(&str, &str)> {
    providers
        .providers
        .iter()
        .flat_map(|provider| {
            provider
                .models
                .iter()
                .filter(|model| model.enabled)
                .map(move |model| (provider.id.as_str(), model.id.as_str()))
        })
        .collect()
}

#[cfg(test)]
mod private_directory_tests {
    use super::{open_private_root, validate_private_directory_fd};

    #[test]
    fn private_directory_rejects_wrong_owner() {
        let root = tempfile::tempdir().expect("root");
        let actual_uid = rustix::process::geteuid().as_raw();
        let descriptor = open_private_root(root.path(), actual_uid, false).expect("trusted root");
        let error =
            validate_private_directory_fd(&descriptor, root.path(), actual_uid.wrapping_add(1))
                .expect_err("wrong owner");
        assert!(error.to_string().contains("wrong owner"));
    }
}

#[cfg(test)]
mod toolchain_tests {
    use super::*;

    #[test]
    fn toolchain_roots_are_canonical_deduplicated_and_never_writable_roots() {
        let root = tempfile::tempdir().unwrap();
        let workspace = root.path().join("workspace");
        let chain = root.path().join("python");
        fs::create_dir_all(&workspace).unwrap();
        fs::create_dir_all(chain.join("bin")).unwrap();
        let alias = root.path().join("alias");
        std::os::unix::fs::symlink(&chain, &alias).unwrap();
        let mut config = WorkspaceConfig {
            format: 1,
            revision: 1,
            id: "ws".into(),
            name: "ws".into(),
            cwd: vec![workspace.to_string_lossy().into_owned()],
            folders: vec![],
            policy: Some(WorkspacePolicy {
                toolchain_roots: vec![
                    alias.to_string_lossy().into_owned(),
                    chain.to_string_lossy().into_owned(),
                ],
                ..Default::default()
            }),
        };
        let resolved = resolve_workspace(&config, Path::new("test.json")).unwrap();
        assert_eq!(
            resolved.policy.toolchain_roots,
            vec![
                fs::canonicalize(&chain)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            ]
        );
        assert!(resolved.policy.writable_roots.is_empty());
        assert_eq!(resolved.cwd.len(), 1, "toolchains are not project folders");
        let ambiguous = root.path().join("bad:path");
        fs::create_dir(&ambiguous).unwrap();
        config.policy.as_mut().unwrap().toolchain_roots =
            vec![ambiguous.to_string_lossy().into_owned()];
        assert!(resolve_workspace(&config, Path::new("test.json")).is_err());
    }
}

#[cfg(test)]
mod legacy_snapshot_tests {
    use super::*;

    #[test]
    fn a_snapshot_published_with_the_integrations_file_still_decodes() {
        let root = tempfile::tempdir().unwrap();
        let cwd = std::fs::canonicalize(root.path()).unwrap();
        let legacy = serde_json::json!({
            "format": 1,
            "workspace": {
                "format": 1, "revision": 1, "id": "ws", "name": "ws",
                "cwd": [cwd.to_str().unwrap()], "policy": {"network": false}
            },
            "providers": {"format": 1, "revision": 0, "providers": []},
            "integrations": {"format": 1, "revision": 0, "integrations": []},
            "settings": {"format": 1, "revision": 0},
            "revisions": {"workspace": 1, "providers": 0, "integrations": 0, "settings": 0}
        });
        let mut bytes = schema::IJsonValue::parse(&serde_json::to_vec(&legacy).unwrap())
            .expect("i-json")
            .canonical_bytes()
            .expect("canonical");
        bytes.push(b'\n');
        let snapshot = ConfigSnapshot::decode(&bytes).expect("legacy snapshot decodes");
        let reencoded = snapshot.canonical_bytes().expect("encode");
        assert!(
            !String::from_utf8(reencoded)
                .unwrap()
                .contains("integrations")
        );
    }
}
