use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Read;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};
use store::{AssetRef, AssetStore};
use unicode_normalization::UnicodeNormalization;

use crate::config::WorkspacePolicy;
use crate::{ProfileError, canonical_line, digest, parse_canonical};

const FORMAT: u64 = 1;
const MAX_FILES: usize = 4096;
const MAX_FILE_BYTES: u64 = 1024 * 1024;
const MAX_TOTAL_BYTES: u64 = 4 * 1024 * 1024;
const MAX_SCANS: usize = 8;
const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstructionPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writable_roots: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_wall_seconds: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstructionSettings {
    pub format: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<InstructionPolicy>,
}

impl InstructionSettings {
    pub fn decode(bytes: &[u8]) -> Result<Self, ProfileError> {
        let value: Self = parse_canonical("<instruction-settings>", bytes)?;
        validate_settings(&value, Path::new("<instruction-settings>"))?;
        Ok(value)
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ProfileError> {
        validate_settings(self, Path::new("<instruction-settings>"))?;
        canonical_line("<instruction-settings>", self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstructionKind {
    Agents,
    Skill,
    Command,
    Hook,
    Settings,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "scope", rename_all = "snake_case", deny_unknown_fields)]
pub enum InstructionOrigin {
    User,
    Workspace,
    Project { workspace_index: u64 },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstructionSource {
    pub origin: InstructionOrigin,
    pub path: String,
    pub kind: InstructionKind,
    pub content: String,
    pub content_sha256: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectiveInstructions {
    pub agents: Vec<u64>,
    pub skills: BTreeMap<String, u64>,
    pub commands: BTreeMap<String, u64>,
    pub hooks: BTreeMap<String, u64>,
    pub policy: InstructionPolicy,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstructionSnapshot {
    pub format: u64,
    pub sources: Vec<InstructionSource>,
    pub effective: EffectiveInstructions,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectivePolicy {
    pub network: bool,
    pub allowed_tools: Vec<String>,
    pub writable_roots: Vec<String>,
    pub max_wall_seconds: Option<u64>,
}

impl EffectivePolicy {
    pub fn digest(&self) -> Result<String, ProfileError> {
        Ok(digest(&canonical_line("<effective-policy>", self)?))
    }
}

#[derive(Clone, Debug)]
pub struct LaunchProfile {
    pub config: crate::ConfigSnapshot,
    pub instruction: InstructionSnapshot,
    pub effective_policy: EffectivePolicy,
    pub config_digest: String,
    pub instruction_digest: String,
    pub config_asset: AssetRef,
    pub instruction_asset: AssetRef,
}

impl LaunchProfile {
    /// Resolves the launch profile for a session's immutable folder binding.
    /// The selected root becomes the default cwd while instruction capture
    /// still includes every bound folder in the workspace.
    pub fn resolve_and_publish_for_binding(
        repository: &crate::ConfigRepository,
        workspace_id: &str,
        user_agent_dir: impl AsRef<Path>,
        assets: &AssetStore,
        folder_binding: Option<&str>,
    ) -> Result<Self, ProfileError> {
        let session_folder = assets
            .root()
            .parent()
            .ok_or_else(|| ProfileError::InvalidPath {
                path: assets.root().to_path_buf(),
                reason: "thread asset directory has no session folder".to_owned(),
            })?;
        let config = match folder_binding {
            Some(binding) => {
                repository.resolve_for_session_binding(workspace_id, session_folder, binding)?
            }
            None => repository.resolve_for_session(workspace_id, session_folder)?,
        };
        let resolver = InstructionResolver::new_scoped(
            user_agent_dir.as_ref(),
            repository.workspace_data_dir(workspace_id)?,
            config.workspace.cwd.iter().map(Path::new),
        );
        let instruction = resolver.capture()?;
        instruction.validate_against_config(&config)?;
        let effective_policy = instruction.meet_workspace_policy(&config.workspace.policy);
        let (config_digest, config_asset) = config.publish(assets)?;
        let (instruction_digest, instruction_asset) = instruction.publish(assets)?;
        Ok(Self {
            config,
            instruction,
            effective_policy,
            config_digest,
            instruction_digest,
            config_asset,
            instruction_asset,
        })
    }
}

impl InstructionSnapshot {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ProfileError> {
        validate_snapshot(self)?;
        canonical_line("<instruction-snapshot>", self)
    }

    pub fn digest(&self) -> Result<String, ProfileError> {
        Ok(digest(&self.canonical_bytes()?))
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ProfileError> {
        let snapshot: Self = parse_canonical("<instruction-snapshot>", bytes)?;
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

    pub fn validate_against_config(
        &self,
        config: &crate::ConfigSnapshot,
    ) -> Result<(), ProfileError> {
        for source in &self.sources {
            if let InstructionOrigin::Project { workspace_index } = source.origin {
                let index = usize::try_from(workspace_index).map_err(|_| {
                    ProfileError::InvalidReference {
                        path: PathBuf::from("<instruction-snapshot>"),
                        reason: format!(
                            "project source index {workspace_index} is not addressable"
                        ),
                    }
                })?;
                if index >= config.workspace.cwd.len() {
                    return Err(ProfileError::InvalidReference {
                        path: PathBuf::from("<instruction-snapshot>"),
                        reason: format!(
                            "project source index {workspace_index} exceeds config cwd count"
                        ),
                    });
                }
            }
        }
        if let Some(roots) = &self.effective.policy.writable_roots {
            for root in roots {
                let canonical =
                    fs::canonicalize(root).map_err(|error| ProfileError::InvalidPath {
                        path: PathBuf::from(root),
                        reason: error.to_string(),
                    })?;
                if canonical.to_str() != Some(root.as_str()) {
                    return Err(ProfileError::InvalidPath {
                        path: PathBuf::from(root),
                        reason: "instruction writable root is not canonical".to_owned(),
                    });
                }
                if !config
                    .workspace
                    .cwd
                    .iter()
                    .any(|cwd| canonical.starts_with(cwd))
                {
                    return Err(ProfileError::InvalidReference {
                        path: PathBuf::from("<instruction-snapshot>"),
                        reason: format!("instruction writable root {root:?} is outside config cwd"),
                    });
                }
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn meet_workspace_policy(&self, workspace: &WorkspacePolicy) -> EffectivePolicy {
        let instruction = &self.effective.policy;
        EffectivePolicy {
            network: workspace.network && instruction.network.unwrap_or(true),
            allowed_tools: meet_required_set(
                &workspace.allowed_tools,
                instruction.allowed_tools.as_deref(),
            ),
            writable_roots: meet_required_set(
                &workspace.writable_roots,
                instruction.writable_roots.as_deref(),
            ),
            max_wall_seconds: meet_optional_cap(
                workspace.max_wall_seconds,
                instruction.max_wall_seconds,
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstructionResolver {
    user_data_dir: PathBuf,
    workspace_data_dir: Option<PathBuf>,
    cwd: Vec<PathBuf>,
}

impl InstructionResolver {
    #[must_use]
    pub fn new(
        user_agent_dir: impl AsRef<Path>,
        cwd: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Self {
        Self {
            user_data_dir: user_agent_dir.as_ref().to_path_buf(),
            workspace_data_dir: None,
            cwd: cwd
                .into_iter()
                .map(|value| value.as_ref().to_path_buf())
                .collect(),
        }
    }

    #[must_use]
    pub fn new_scoped(
        user_data_dir: impl AsRef<Path>,
        workspace_data_dir: impl AsRef<Path>,
        cwd: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> Self {
        Self {
            user_data_dir: user_data_dir.as_ref().to_path_buf(),
            workspace_data_dir: Some(workspace_data_dir.as_ref().to_path_buf()),
            cwd: cwd
                .into_iter()
                .map(|value| value.as_ref().to_path_buf())
                .collect(),
        }
    }

    pub fn capture(&self) -> Result<InstructionSnapshot, ProfileError> {
        self.capture_with_hook(|_| {})
    }

    pub fn capture_with_hook(
        &self,
        mut after_scan: impl FnMut(usize),
    ) -> Result<InstructionSnapshot, ProfileError> {
        let mut previous = None;
        for scan in 1..=MAX_SCANS {
            let current = match self.scan() {
                Ok(current) => current,
                Err(ProfileError::UnstableSource { .. }) => {
                    after_scan(scan);
                    previous = None;
                    continue;
                }
                Err(error) => return Err(error),
            };
            after_scan(scan);
            if previous.as_ref() == Some(&current) {
                return build_snapshot(current);
            }
            previous = Some(current);
        }
        Err(ProfileError::UnstableSource {
            attempts: MAX_SCANS,
        })
    }

    fn scan(&self) -> Result<Vec<InstructionSource>, ProfileError> {
        let mut sources = Vec::new();
        scan_origin(
            &self.user_data_dir,
            true,
            InstructionOrigin::User,
            &mut sources,
        )?;
        if let Some(workspace) = &self.workspace_data_dir {
            scan_origin(workspace, true, InstructionOrigin::Workspace, &mut sources)?;
        }
        for (index, cwd) in self.cwd.iter().enumerate() {
            let canonical = fs::canonicalize(cwd).map_err(|error| ProfileError::InvalidPath {
                path: cwd.clone(),
                reason: error.to_string(),
            })?;
            if !canonical.is_dir() {
                return Err(ProfileError::InvalidPath {
                    path: canonical,
                    reason: "workspace cwd is not a directory".to_owned(),
                });
            }
            scan_origin(
                &canonical,
                false,
                InstructionOrigin::Project {
                    workspace_index: index as u64,
                },
                &mut sources,
            )?;
        }
        if sources.len() > MAX_FILES {
            return Err(ProfileError::LimitExceeded("more than 4096 files"));
        }
        let total = sources
            .iter()
            .map(|source| source.content.len() as u64)
            .sum::<u64>();
        if total > MAX_TOTAL_BYTES {
            return Err(ProfileError::LimitExceeded(
                "more than 4 MiB of instruction bytes",
            ));
        }
        let mut identities = BTreeSet::new();
        for source in &sources {
            if !identities.insert((
                source.origin.clone(),
                source.kind.clone(),
                source.path.clone(),
            )) {
                return Err(ProfileError::InvalidPath {
                    path: PathBuf::from(&source.path),
                    reason: "instruction paths collide after NFC normalization".to_owned(),
                });
            }
        }
        sources.sort_by_key(source_order_key);
        Ok(sources)
    }
}

fn scan_origin(
    root: &Path,
    user: bool,
    origin: InstructionOrigin,
    sources: &mut Vec<InstructionSource>,
) -> Result<(), ProfileError> {
    if !require_optional_directory(root)? {
        return Ok(());
    }
    let agents = root.join("AGENTS.md");
    maybe_source(
        &agents,
        "AGENTS.md",
        InstructionKind::Agents,
        &origin,
        sources,
    )?;
    let agent = if user {
        root.to_path_buf()
    } else {
        let authored = root.join(".agents");
        if require_optional_directory(&authored)? {
            authored
        } else {
            let legacy = root.join(".tekes");
            if require_optional_directory(&legacy)? {
                legacy
            } else {
                root.join(".agent")
            }
        }
    };
    if !require_optional_directory(&agent)? {
        return Ok(());
    }
    maybe_source(
        &agent.join("settings.json"),
        "settings.json",
        InstructionKind::Settings,
        &origin,
        sources,
    )?;
    for (directory, kind) in [
        ("commands", InstructionKind::Command),
        ("hooks", InstructionKind::Hook),
        ("skills", InstructionKind::Skill),
    ] {
        let path = agent.join(directory);
        let mut files = Vec::new();
        collect_files(&path, &path, &mut files)?;
        files.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
        for (relative, file) in files {
            if kind == InstructionKind::Skill && !skill_text_resource(&relative) {
                continue;
            }
            maybe_source(
                &file,
                &format!("{directory}/{relative}"),
                kind.clone(),
                &origin,
                sources,
            )?;
        }
    }
    if user
        || agent
            .file_name()
            .is_some_and(|name| name == ".tekes" || name == ".agents")
    {
        for directory in ["instructions", "rules"] {
            let path = agent.join(directory);
            let mut files = Vec::new();
            collect_files(&path, &path, &mut files)?;
            files.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
            for (relative, file) in files {
                maybe_source(
                    &file,
                    &format!("{directory}/{relative}"),
                    InstructionKind::Agents,
                    &origin,
                    sources,
                )?;
            }
        }
    }
    Ok(())
}

fn skill_text_resource(relative: &str) -> bool {
    if relative.split('/').any(|part| part.starts_with('.')) {
        return false;
    }
    matches!(
        Path::new(relative)
            .extension()
            .and_then(|value| value.to_str()),
        Some(
            "md" | "txt"
                | "json"
                | "yaml"
                | "yml"
                | "toml"
                | "sh"
                | "py"
                | "js"
                | "ts"
                | "mjs"
                | "cjs"
                | "html"
                | "css"
                | "csv"
                | "xml"
        )
    )
}

fn collect_files(
    root: &Path,
    current: &Path,
    output: &mut Vec<(String, PathBuf)>,
) -> Result<(), ProfileError> {
    if !require_optional_directory(current)? {
        return Ok(());
    }
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            return Err(ProfileError::Symlink(path));
        }
        if metadata.is_dir() {
            collect_files(root, &path, output)?;
        } else if metadata.is_file() {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| ProfileError::InvalidPath {
                    path: path.clone(),
                    reason: "instruction path escaped its kind directory".to_owned(),
                })?;
            output.push((normalize_relative(relative)?, path));
        } else {
            return Err(ProfileError::NotRegular(path));
        }
    }
    Ok(())
}

fn maybe_source(
    file: &Path,
    logical_path: &str,
    kind: InstructionKind,
    origin: &InstructionOrigin,
    sources: &mut Vec<InstructionSource>,
) -> Result<(), ProfileError> {
    match fs::symlink_metadata(file) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                return Err(ProfileError::Symlink(file.to_path_buf()));
            }
            if !metadata.is_file() {
                return Err(ProfileError::NotRegular(file.to_path_buf()));
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    }
    let bytes = read_stable_file(file)?;
    let content = String::from_utf8(bytes.clone()).map_err(|_| ProfileError::InvalidSchema {
        path: file.to_path_buf(),
        reason: "instruction content is not UTF-8".to_owned(),
    })?;
    if content.contains('\0') {
        return Err(ProfileError::InvalidSchema {
            path: file.to_path_buf(),
            reason: "instruction content contains NUL".to_owned(),
        });
    }
    if matches!(kind, InstructionKind::Settings) {
        let settings: InstructionSettings = parse_canonical(file, &bytes)?;
        validate_settings(&settings, file)?;
    }
    let path = normalize_logical(logical_path)?;
    sources.push(InstructionSource {
        origin: origin.clone(),
        path,
        kind,
        content,
        content_sha256: digest(&bytes),
    });
    Ok(())
}

fn read_stable_file(path: &Path) -> Result<Vec<u8>, ProfileError> {
    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)?;
    let before = file.metadata()?;
    if !before.is_file() {
        return Err(ProfileError::NotRegular(path.to_path_buf()));
    }
    if before.len() > MAX_FILE_BYTES {
        return Err(ProfileError::LimitExceeded(
            "instruction file exceeds 1 MiB",
        ));
    }
    let mut bytes = Vec::with_capacity(before.len() as usize);
    file.read_to_end(&mut bytes)?;
    let after = file.metadata()?;
    let named = fs::symlink_metadata(path)?;
    if named.file_type().is_symlink() {
        return Err(ProfileError::UnstableSource { attempts: 1 });
    }
    if before.dev() != after.dev()
        || before.ino() != after.ino()
        || before.len() != after.len()
        || before.modified()? != after.modified()?
        || after.dev() != named.dev()
        || after.ino() != named.ino()
    {
        return Err(ProfileError::UnstableSource { attempts: 1 });
    }
    Ok(bytes)
}

fn reject_directory_symlink(path: &Path) -> Result<(), ProfileError> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() {
        Err(ProfileError::Symlink(path.to_path_buf()))
    } else if !metadata.is_dir() {
        Err(ProfileError::InvalidPath {
            path: path.to_path_buf(),
            reason: "expected instruction directory".to_owned(),
        })
    } else {
        Ok(())
    }
}

fn require_optional_directory(path: &Path) -> Result<bool, ProfileError> {
    match fs::symlink_metadata(path) {
        Ok(_) => {
            reject_directory_symlink(path)?;
            Ok(true)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}

fn normalize_relative(path: &Path) -> Result<String, ProfileError> {
    let mut values = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => {
                let value = value.to_str().ok_or_else(|| ProfileError::InvalidPath {
                    path: path.to_path_buf(),
                    reason: "instruction path is not UTF-8".to_owned(),
                })?;
                values.push(value.nfc().collect::<String>());
            }
            _ => {
                return Err(ProfileError::InvalidPath {
                    path: path.to_path_buf(),
                    reason: "instruction path contains a non-normal component".to_owned(),
                });
            }
        }
    }
    normalize_logical(&values.join("/"))
}

fn normalize_logical(path: &str) -> Result<String, ProfileError> {
    if path.is_empty()
        || path.contains('\0')
        || path
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
    {
        return Err(ProfileError::InvalidPath {
            path: PathBuf::from(path),
            reason: "invalid logical instruction path".to_owned(),
        });
    }
    Ok(path.nfc().collect())
}

fn build_snapshot(sources: Vec<InstructionSource>) -> Result<InstructionSnapshot, ProfileError> {
    let effective = derive_effective(&sources)?;
    let snapshot = InstructionSnapshot {
        format: FORMAT,
        sources,
        effective,
    };
    validate_snapshot(&snapshot)?;
    Ok(snapshot)
}

fn derive_effective(sources: &[InstructionSource]) -> Result<EffectiveInstructions, ProfileError> {
    let mut effective = EffectiveInstructions::default();
    let mut policies = Vec::new();
    for (index, source) in sources.iter().enumerate() {
        let index = index as u64;
        match source.kind {
            InstructionKind::Agents => effective.agents.push(index),
            InstructionKind::Skill => {
                effective
                    .skills
                    .insert(resource_key(source, "skills/")?, index);
            }
            InstructionKind::Command => {
                effective
                    .commands
                    .insert(resource_key(source, "commands/")?, index);
            }
            InstructionKind::Hook => {
                effective
                    .hooks
                    .insert(resource_key(source, "hooks/")?, index);
            }
            InstructionKind::Settings => {
                let settings: InstructionSettings =
                    parse_canonical(PathBuf::from(&source.path), source.content.as_bytes())?;
                if let Some(policy) = settings.policy {
                    policies.push(policy);
                }
            }
        }
    }
    effective.policy = meet_policies(policies);
    Ok(effective)
}

fn resource_key(source: &InstructionSource, prefix: &str) -> Result<String, ProfileError> {
    source
        .path
        .strip_prefix(prefix)
        .map(ToOwned::to_owned)
        .ok_or_else(|| ProfileError::InvalidSchema {
            path: PathBuf::from(&source.path),
            reason: format!("resource path lacks {prefix}"),
        })
}

fn meet_policies(policies: Vec<InstructionPolicy>) -> InstructionPolicy {
    let mut network = None;
    let mut tools: Option<BTreeSet<String>> = None;
    let mut roots: Option<BTreeSet<String>> = None;
    let mut cap = None;
    for policy in policies {
        if let Some(value) = policy.network {
            network = Some(network.unwrap_or(true) && value);
        }
        meet_optional_set(&mut tools, policy.allowed_tools);
        meet_optional_set(&mut roots, policy.writable_roots);
        if let Some(value) = policy.max_wall_seconds {
            cap = Some(cap.map_or(value, |current: u64| current.min(value)));
        }
    }
    InstructionPolicy {
        network,
        allowed_tools: tools.map(|value| value.into_iter().collect()),
        writable_roots: roots.map(|value| value.into_iter().collect()),
        max_wall_seconds: cap,
    }
}

fn meet_optional_set(target: &mut Option<BTreeSet<String>>, value: Option<Vec<String>>) {
    let Some(value) = value else {
        return;
    };
    let value = value.into_iter().collect::<BTreeSet<_>>();
    *target = Some(match target.take() {
        Some(current) => current.intersection(&value).cloned().collect(),
        None => value,
    });
}

fn meet_required_set(required: &[String], constraint: Option<&[String]>) -> Vec<String> {
    let required = required.iter().cloned().collect::<BTreeSet<_>>();
    match constraint {
        Some(constraint) => {
            let constraint = constraint.iter().cloned().collect::<BTreeSet<_>>();
            required.intersection(&constraint).cloned().collect()
        }
        None => required.into_iter().collect(),
    }
}

fn meet_optional_cap(left: Option<u64>, right: Option<u64>) -> Option<u64> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

fn validate_settings(value: &InstructionSettings, path: &Path) -> Result<(), ProfileError> {
    if value.format != FORMAT {
        return Err(ProfileError::UnsupportedFormat {
            path: path.to_path_buf(),
            format: value.format,
        });
    }
    if let Some(policy) = &value.policy {
        if policy.max_wall_seconds == Some(0) {
            return Err(ProfileError::InvalidSchema {
                path: path.to_path_buf(),
                reason: "max_wall_seconds must be at least 1".to_owned(),
            });
        }
        if policy
            .max_wall_seconds
            .is_some_and(|value| value > MAX_SAFE_INTEGER)
        {
            return Err(ProfileError::InvalidSchema {
                path: path.to_path_buf(),
                reason: "max_wall_seconds exceeds the I-JSON safe-integer range".to_owned(),
            });
        }
        for values in [&policy.allowed_tools, &policy.writable_roots]
            .into_iter()
            .flatten()
        {
            if values.iter().collect::<BTreeSet<_>>().len() != values.len() {
                return Err(ProfileError::InvalidSchema {
                    path: path.to_path_buf(),
                    reason: "instruction policy set contains duplicates".to_owned(),
                });
            }
        }
        for root in policy.writable_roots.as_deref().unwrap_or_default() {
            if root.contains('\0') || !Path::new(root).is_absolute() {
                return Err(ProfileError::InvalidPath {
                    path: path.to_path_buf(),
                    reason: "instruction writable root must be absolute".to_owned(),
                });
            }
        }
    }
    Ok(())
}

fn validate_snapshot(snapshot: &InstructionSnapshot) -> Result<(), ProfileError> {
    let path = Path::new("<instruction-snapshot>");
    if snapshot.format != FORMAT {
        return Err(ProfileError::UnsupportedFormat {
            path: path.to_path_buf(),
            format: snapshot.format,
        });
    }
    if snapshot.sources.len() > MAX_FILES {
        return Err(ProfileError::LimitExceeded("more than 4096 files"));
    }
    let total = snapshot
        .sources
        .iter()
        .map(|source| source.content.len() as u64)
        .sum::<u64>();
    if total > MAX_TOTAL_BYTES {
        return Err(ProfileError::LimitExceeded(
            "more than 4 MiB of instruction bytes",
        ));
    }
    let mut previous_key = None;
    for (index, source) in snapshot.sources.iter().enumerate() {
        if source.content.len() as u64 > MAX_FILE_BYTES {
            return Err(ProfileError::LimitExceeded(
                "instruction file exceeds 1 MiB",
            ));
        }
        if source.content.contains('\0') {
            return Err(ProfileError::InvalidSchema {
                path: PathBuf::from(&source.path),
                reason: "instruction content contains NUL".to_owned(),
            });
        }
        if source.content_sha256.len() != 64
            || !source
                .content_sha256
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(ProfileError::InvalidSchema {
                path: path.to_path_buf(),
                reason: format!("source {index} content_sha256 is not lowercase SHA-256"),
            });
        }
        let actual = digest(source.content.as_bytes());
        if source.content_sha256 != actual {
            return Err(ProfileError::DigestMismatch {
                expected: source.content_sha256.clone(),
                actual,
            });
        }
        if normalize_logical(&source.path)? != source.path {
            return Err(ProfileError::InvalidPath {
                path: PathBuf::from(&source.path),
                reason: "logical path is not NFC-normalized".to_owned(),
            });
        }
        validate_source_path(source)?;
        if let InstructionOrigin::Project { workspace_index } = source.origin {
            if workspace_index > MAX_SAFE_INTEGER {
                return Err(ProfileError::InvalidSchema {
                    path: path.to_path_buf(),
                    reason: format!("source {index} workspace_index exceeds safe integer"),
                });
            }
        }
        let key = source_order_key(source);
        if previous_key
            .as_ref()
            .is_some_and(|previous| previous >= &key)
        {
            return Err(ProfileError::InvalidSchema {
                path: path.to_path_buf(),
                reason: "sources are not in the deterministic discovery order".to_owned(),
            });
        }
        previous_key = Some(key);
        if matches!(source.kind, InstructionKind::Settings) {
            let settings: InstructionSettings =
                parse_canonical(PathBuf::from(&source.path), source.content.as_bytes())?;
            validate_settings(&settings, Path::new(&source.path))?;
        }
        if matches!(source.kind, InstructionKind::Hook) {
            let logical = resource_key(source, "hooks/")?;
            tools::validate_instruction_hook(source.content.as_bytes(), &logical).map_err(
                |error| ProfileError::InvalidSchema {
                    path: PathBuf::from(&source.path),
                    reason: error.to_string(),
                },
            )?;
        }
    }
    let expected = derive_effective(&snapshot.sources)?;
    if snapshot.effective != expected {
        return Err(ProfileError::InvalidSchema {
            path: path.to_path_buf(),
            reason: "effective projection does not equal the deterministic source fold".to_owned(),
        });
    }
    Ok(())
}

fn validate_source_path(source: &InstructionSource) -> Result<(), ProfileError> {
    match source.kind {
        InstructionKind::Agents
            if source.path == "AGENTS.md"
                || source.path.starts_with("instructions/")
                || source.path.starts_with("rules/") =>
        {
            Ok(())
        }
        InstructionKind::Settings if source.path == "settings.json" => Ok(()),
        InstructionKind::Skill => resource_key(source, "skills/").map(|_| ()),
        InstructionKind::Command => resource_key(source, "commands/").map(|_| ()),
        InstructionKind::Hook => resource_key(source, "hooks/").map(|_| ()),
        _ => Err(ProfileError::InvalidSchema {
            path: PathBuf::from(&source.path),
            reason: "source kind does not match its logical path".to_owned(),
        }),
    }
}

fn source_order_key(source: &InstructionSource) -> (u8, u64, u8, Vec<u8>) {
    let (scope, workspace) = match source.origin {
        InstructionOrigin::User => (0, 0),
        InstructionOrigin::Workspace => (1, 0),
        InstructionOrigin::Project { workspace_index } => (2, workspace_index),
    };
    let kind = match source.kind {
        InstructionKind::Agents => 0,
        InstructionKind::Settings => 1,
        InstructionKind::Command => 2,
        InstructionKind::Hook => 3,
        InstructionKind::Skill => 4,
    };
    (scope, workspace, kind, source.path.as_bytes().to_vec())
}
