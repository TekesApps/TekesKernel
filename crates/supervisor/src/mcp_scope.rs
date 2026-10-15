//! Canonical ownership is independent of the workspace compatibility address.
use mcp::{McpError, McpRegistry, McpRegistryStore, McpScope, McpServerConfig, McpTransportConfig};
use profile::{FolderAuthorityStore, FolderTrust};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

fn refused(reason: impl ToString) -> McpError {
    McpError::Conflict(reason.to_string())
}
pub fn storage_root(registry: &McpRegistryStore) -> Result<&Path, McpError> {
    // Production owns <storage>/config. Never create a guessed parent root.
    if registry
        .root()
        .file_name()
        .is_none_or(|name| name != "config")
    {
        return Err(refused("folder authority root unavailable"));
    }
    registry
        .root()
        .parent()
        .ok_or_else(|| refused("folder authority root unavailable"))
}
pub fn config_digest(server: &McpServerConfig) -> Result<String, McpError> {
    let value = serde_json::json!({"transport":server.transport,"always_on":server.always_on,"protocol_mode":server.protocol_mode,"plugin_owner":server.owner,"plugin_component":server.plugin_component});
    Ok(format!(
        "{:x}",
        Sha256::digest(serde_json_canonicalizer::to_vec(&value).map_err(refused)?)
    ))
}
pub fn folder(
    server: &McpServerConfig,
    registry: &McpRegistryStore,
) -> Result<Option<FolderTrust>, McpError> {
    if server.reference.scope != McpScope::Project {
        return Ok(None);
    }
    let root = storage_root(registry)?;
    let authority = FolderAuthorityStore::new(root);
    if let Some(id) = &server.folder_id {
        if let Some(folder) = profile::FolderCatalog::new(root)
            .folders()
            .map_err(refused)?
            .into_iter()
            .find(|folder| &folder.id == id)
        {
            return authority
                .status(Path::new(&folder.path))
                .map(Some)
                .map_err(refused);
        }
        return authority.record(id).map(Some).map_err(refused);
    }
    let repository = profile::ConfigRepository::open(root).map_err(refused)?;
    let workspace = repository.workspace(&server.reference.workspace_id);
    let paths = match &workspace {
        Ok(workspace) => workspace
            .folder_paths()
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        Err(_) => profile::FolderCatalog::new(root)
            .folders()
            .map_err(refused)?
            .into_iter()
            .filter(|folder| {
                folder
                    .aliases
                    .iter()
                    .any(|(workspace, _)| workspace == &server.reference.workspace_id)
            })
            .map(|folder| folder.path)
            .collect(),
    };
    if paths.len() != 1 {
        return Err(refused("legacy project MCP requires an explicit folder_id"));
    }
    authority
        .status(Path::new(&paths[0]))
        .map(Some)
        .map_err(refused)
}
pub fn owner(
    server: &McpServerConfig,
    registry: Option<&McpRegistryStore>,
) -> Result<(String, Option<FolderTrust>), McpError> {
    match server.reference.scope {
        McpScope::User => Ok(("user".into(), None)),
        McpScope::Plugin => Ok((
            format!(
                "plugin:{}",
                server
                    .owner
                    .as_deref()
                    .ok_or_else(|| refused("missing plugin owner"))?
            ),
            None,
        )),
        McpScope::Project => {
            let folder = folder(
                server,
                registry.ok_or_else(|| refused("missing folder authority"))?,
            )?
            .ok_or_else(|| refused("missing folder identity"))?;
            if matches!(server.transport, McpTransportConfig::Stdio { .. }) && !folder.trusted {
                return Err(refused("project-folder-untrusted"));
            }
            Ok((format!("folder:{}", folder.id), Some(folder)))
        }
    }
}
fn user_identity(
    server: &McpServerConfig,
    store: &McpRegistryStore,
) -> Result<serde_json::Value, McpError> {
    let credentials = super::mcp_runtime::credential_fields(server)
        .into_iter()
        .map(|(field, id)| {
            let state = store
                .credential_reference_state(&server.reference, &field, &id)
                .map_err(refused)?;
            Ok(serde_json::json!({"field":field,"id":id,"state":state}))
        })
        .collect::<Result<Vec<_>, McpError>>()?;
    Ok(serde_json::json!({"config":config_digest(server)?,"credentials":credentials}))
}
pub fn resolve(
    registry: &McpRegistry,
    store: &McpRegistryStore,
    workspace: &str,
    paths: Option<&[String]>,
) -> Result<Vec<McpServerConfig>, McpError> {
    let paths: Vec<PathBuf> = match paths {
        Some(paths) => paths.iter().map(PathBuf::from).collect(),
        None => storage_root(store)
            .ok()
            .and_then(|root| profile::ConfigRepository::open(root).ok())
            .and_then(|repo| repo.workspace(workspace).ok())
            .map(|ws| ws.folder_paths().into_iter().map(PathBuf::from).collect())
            .unwrap_or_default(),
    };
    let mut users: BTreeMap<String, (&McpServerConfig, serde_json::Value)> = BTreeMap::new();
    let mut projects: BTreeMap<String, (&McpServerConfig, String, serde_json::Value)> =
        BTreeMap::new();
    for server in registry.servers.iter().filter(|server| server.enabled) {
        match server.reference.scope {
            McpScope::User => {
                let identity = user_identity(server, store)?;
                if let Some((_, prior)) = users.get(&server.reference.name) {
                    if *prior != identity {
                        return Err(refused(format!(
                            "conflicting global user MCP {}",
                            server.reference.name
                        )));
                    }
                } else {
                    users.insert(server.reference.name.clone(), (server, identity));
                }
            }
            McpScope::Project => {
                let Ok(Some(folder)) = folder(server, store) else {
                    if server.reference.workspace_id == workspace {
                        return Err(refused("project MCP owner unavailable or ambiguous"));
                    }
                    continue;
                };
                if !paths.iter().any(|path| {
                    path.canonicalize().ok().as_deref() == Some(Path::new(&folder.path))
                }) {
                    continue;
                }
                if matches!(server.transport, McpTransportConfig::Stdio { .. }) && !folder.trusted {
                    continue;
                }
                let identity = user_identity(server, store)?;
                if let Some((_, owner, prior)) = projects.get(&server.reference.name) {
                    if *owner != folder.id || *prior != identity {
                        return Err(refused("project MCP owner/name collision"));
                    }
                } else {
                    projects.insert(server.reference.name.clone(), (server, folder.id, identity));
                }
            }
            McpScope::Plugin => {}
        }
    }
    let mut resolved = users
        .into_iter()
        .map(|(name, (server, _))| (name, server.clone()))
        .collect::<BTreeMap<_, _>>();
    // Preserve existing project-over-user precedence, rejecting peer project collisions.
    resolved.extend(
        projects
            .into_iter()
            .map(|(name, (server, _, _))| (name, server.clone())),
    );
    Ok(resolved.into_values().collect())
}

pub fn validate_user(server: &McpServerConfig, store: &McpRegistryStore) -> Result<(), McpError> {
    let identity = user_identity(server, store)?;
    for other in store
        .load()
        .map_err(refused)?
        .servers
        .iter()
        .filter(|other| {
            other.enabled
                && other.reference.scope == McpScope::User
                && other.reference.name == server.reference.name
        })
    {
        if user_identity(other, store)? != identity {
            return Err(refused("conflicting global user MCP definition"));
        }
    }
    Ok(())
}

pub fn validate_project(
    server: &McpServerConfig,
    store: &McpRegistryStore,
) -> Result<(), McpError> {
    let selected = folder(server, store)?.ok_or_else(|| refused("missing folder owner"))?;
    let identity = user_identity(server, store)?;
    for other in store
        .load()
        .map_err(refused)?
        .servers
        .iter()
        .filter(|other| {
            other.enabled
                && other.reference.scope == McpScope::Project
                && other.reference.name == server.reference.name
        })
    {
        if folder(other, store)
            .ok()
            .flatten()
            .is_some_and(|folder| folder.id == selected.id)
            && user_identity(other, store)? != identity
        {
            return Err(refused("conflicting project MCP definition"));
        }
    }
    Ok(())
}
