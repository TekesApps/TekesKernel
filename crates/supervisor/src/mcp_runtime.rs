use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, Weak};
use std::thread;

use mcp::{
    CredentialValue, HttpRequestAuthorization, HttpTransport, McpBroker, McpBrokerHandle,
    McpCancellationToken, McpClient, McpCredentialFieldOperation, McpCredentialReferenceState,
    McpError, McpImplementation, McpManagementMutation, McpMutationReceipt, McpPeer, McpPoolKey,
    McpRegistryError, McpRegistryStore, McpResource, McpScope, McpServerConfig, McpServerReference,
    McpTool, McpToolCallContext, McpTransportConfig, ProtocolMode, StdioTransport, project_catalog,
};
use plugins::{
    ComponentKind, HostEnvironment, MacOsNativeHelperVerifier, NativeHelperVerifier,
    OperatingSystem, PluginComponentReference, PluginStore, PluginVersion,
    ResolvedPluginExecutable, SignaturePolicy,
};
use profile::{DynamicTool, DynamicToolCatalog, DynamicToolSourceKind};
use provider::{SecretMutationAuthority, SecretRecord, SecretResolution, SecretStore};
use schema::IJsonValue;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use worker_control::ToolControl;
use zeroize::Zeroize;

use crate::continuation_journal::{
    ContinuationBinding, ContinuationJournal, ContinuationJournalError,
};
use crate::endpoint_host::{ProductionEndpointRoutes, ProductionRouteFailure};
use crate::mcp_continuation::{resolve_task, task_authority};
use crate::production_tool_control::{
    DynamicExecutionOutcome, DynamicSupervisorAuthority, SupervisorOperationError,
};
use crate::tool_control::ExternalEffectResolution;
use endpoint::EndpointHostCall;
use worker_control::continuation::{ToolContinuationRequest, ToolContinuationResponse};

/// One launch-frozen MCP catalog and its supervisor execution routes.
#[derive(Clone)]
pub struct McpWorkspaceBinding {
    pub catalog: DynamicToolCatalog,
    pub authority: Arc<McpDynamicSupervisorAuthority>,
    pub failures: Vec<McpServerFailure>,
    /// The standalone registry could not be read or resolved, so no
    /// registry-configured server joined this binding (plugin servers still
    /// do). A worker launches degraded rather than not at all; the message is
    /// for the session notice.
    pub registry_failure: Option<String>,
}

/// One server that did not join a binding. `code` is the stable readiness
/// state (`credential-unavailable`, `refresh-failed`, `projection-failed`);
/// `detail` is the human sentence for the session notice — for a projection
/// failure it names the tool and the schema keyword that was refused.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpServerFailure {
    pub server: McpServerReference,
    pub code: &'static str,
    pub detail: Option<String>,
}

#[derive(Clone, Debug)]
pub struct McpResourceBinding {
    pub server: McpServerReference,
    pub binding_digest: String,
    pub resource: McpResource,
}

/// Supervisor-owned MCP generation authority. The registry is durable config;
/// the broker/pool is rebuildable process state.
pub struct McpRuntime {
    registry: McpRegistryStore,
    secret_store: Arc<dyn SecretStore>,
    /// Write path for OAuth grants the Kernel minted (mcp-runtime §oauth);
    /// without it an `oauth` binding can only carry a platform-installed token.
    secret_mutation: Option<Arc<dyn SecretMutationAuthority>>,
    plugin_resolver: Option<Arc<dyn PluginMcpResolver>>,
    broker: McpBroker,
    preparation: Mutex<()>,
    bindings: Mutex<BTreeMap<String, CachedBinding>>,
    failure_states: Mutex<BTreeMap<McpServerReference, &'static str>>,
    /// Per-process generation prevents a resource reference issued by a dead
    /// runtime from being rebound to a freshly connected peer after restart.
    resource_instance: String,
}

pub type ProductionPluginStore = Arc<Mutex<PluginStore<MacOsNativeHelperVerifier>>>;
pub type ProductionMcpAuthorities = (McpRuntime, Option<ProductionPluginStore>);

pub trait PluginMcpResolver: Send + Sync {
    fn resolve(
        &self,
        reference: &PluginComponentReference,
    ) -> Result<Option<ResolvedPluginExecutable>, String>;

    fn components(&self) -> Result<Vec<ResolvedPluginExecutable>, String> {
        Ok(Vec::new())
    }
}

pub struct PluginStoreMcpResolver<V> {
    store: Arc<Mutex<PluginStore<V>>>,
}

impl<V> PluginStoreMcpResolver<V> {
    #[must_use]
    pub fn new(store: PluginStore<V>) -> Self {
        Self {
            store: Arc::new(Mutex::new(store)),
        }
    }

    #[must_use]
    pub fn from_shared(store: Arc<Mutex<PluginStore<V>>>) -> Self {
        Self { store }
    }
}

impl<V: NativeHelperVerifier> PluginMcpResolver for PluginStoreMcpResolver<V> {
    fn resolve(
        &self,
        reference: &PluginComponentReference,
    ) -> Result<Option<ResolvedPluginExecutable>, String> {
        self.store
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .resolve_executable_component(reference)
            .map_err(|error| error.to_string())
    }

    fn components(&self) -> Result<Vec<ResolvedPluginExecutable>, String> {
        let mut store = self
            .store
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let components = store.components().map_err(|error| error.to_string())?;
        let references = components
            .into_iter()
            .filter(|component| component.component_type == ComponentKind::McpServer)
            .map(|component| PluginComponentReference {
                plugin_id: component.owner_plugin_id,
                component_id: component.component_id,
            })
            .collect::<Vec<_>>();
        references
            .into_iter()
            .map(|reference| {
                store
                    .resolve_executable_component(&reference)
                    .map_err(|error| error.to_string())?
                    .ok_or_else(|| {
                        format!(
                            "plugin MCP component {}/{} lost runtime eligibility",
                            reference.plugin_id, reference.component_id
                        )
                    })
            })
            .collect()
    }
}

/// Registry-configured servers, their plugin resolutions, and the registry
/// failure that emptied the first two when the standalone registry could not
/// be read (see `servers_with_plugins_degraded`).
type DegradedWorkspaceServers = (
    Vec<McpServerConfig>,
    Vec<Option<ResolvedPluginExecutable>>,
    Option<String>,
);

struct CachedBinding {
    fingerprint: String,
    catalog: DynamicToolCatalog,
    authority: Weak<McpDynamicSupervisorAuthority>,
    /// The bound routes outlive any one worker's authority: a later run of the
    /// same workspace (a parked remote continuation resuming) rebinds them to
    /// the still-pooled peers instead of re-preparing and restarting servers.
    routes: BTreeMap<String, McpRoute>,
    catalog_generations: BTreeMap<McpPoolKey, u64>,
    failures: Vec<McpServerFailure>,
    registry_failure: Option<String>,
}

impl McpRuntime {
    pub fn open(
        config_root: impl Into<PathBuf>,
        secret_store: Arc<dyn SecretStore>,
    ) -> Result<Self, McpError> {
        Self::open_with_plugin_resolver(config_root, secret_store, None)
    }

    pub fn open_with_plugin_resolver(
        config_root: impl Into<PathBuf>,
        secret_store: Arc<dyn SecretStore>,
        plugin_resolver: Option<Arc<dyn PluginMcpResolver>>,
    ) -> Result<Self, McpError> {
        Self::open_with_authorities(config_root, secret_store, None, plugin_resolver)
    }

    pub fn open_with_authorities(
        config_root: impl Into<PathBuf>,
        secret_store: Arc<dyn SecretStore>,
        secret_mutation: Option<Arc<dyn SecretMutationAuthority>>,
        plugin_resolver: Option<Arc<dyn PluginMcpResolver>>,
    ) -> Result<Self, McpError> {
        Ok(Self {
            registry: McpRegistryStore::new(config_root),
            secret_store,
            secret_mutation,
            plugin_resolver,
            broker: McpBroker::start()?,
            preparation: Mutex::new(()),
            bindings: Mutex::new(BTreeMap::new()),
            failure_states: Mutex::new(BTreeMap::new()),
            resource_instance: uuid::Uuid::new_v4().to_string(),
        })
    }

    pub fn open_production_authorities(
        config_root: impl Into<PathBuf>,
        plugin_root: impl Into<PathBuf>,
        build: &str,
        secret_store: Arc<dyn SecretStore>,
    ) -> Result<ProductionMcpAuthorities, McpError> {
        Self::open_production_authorities_with_mutation(
            config_root,
            plugin_root,
            build,
            secret_store,
            None,
        )
    }

    pub fn open_production_authorities_with_mutation(
        config_root: impl Into<PathBuf>,
        plugin_root: impl Into<PathBuf>,
        build: &str,
        secret_store: Arc<dyn SecretStore>,
        secret_mutation: Option<Arc<dyn SecretMutationAuthority>>,
    ) -> Result<ProductionMcpAuthorities, McpError> {
        let Ok(host_version) = build.parse::<PluginVersion>() else {
            // Non-release test/development hosts may use an opaque build id.
            // They can run standalone MCP rows but cannot claim plugin host
            // compatibility without a SemVer authority.
            return Self::open_with_authorities(config_root, secret_store, secret_mutation, None)
                .map(|runtime| (runtime, None));
        };
        let operating_system = if cfg!(target_os = "macos") {
            OperatingSystem::MacOs
        } else if cfg!(target_os = "windows") {
            OperatingSystem::Windows
        } else {
            OperatingSystem::Linux
        };
        let store = PluginStore::open(
            plugin_root,
            HostEnvironment {
                host_version,
                operating_system,
                operating_system_version: current_operating_system_version(),
                architecture: current_plugin_architecture(),
            },
            SignaturePolicy {
                allow_unsigned_local: true,
                trusted_publishers: BTreeMap::new(),
            },
            MacOsNativeHelperVerifier,
        )
        .map_err(|error| {
            McpError::Conflict(format!("plugin MCP authority unavailable: {error}"))
        })?;
        let store = Arc::new(Mutex::new(store));
        Self::open_with_authorities(
            config_root,
            secret_store,
            secret_mutation,
            Some(Arc::new(PluginStoreMcpResolver::from_shared(Arc::clone(
                &store,
            )))),
        )
        .map(|runtime| (runtime, Some(store)))
    }

    fn secret_access(&self) -> SecretAccess {
        SecretAccess {
            store: Arc::clone(&self.secret_store),
            mutation: self.secret_mutation.clone(),
        }
    }

    pub fn list_servers(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<McpServerConfig>, McpRegistryError> {
        let mut servers = self.registry.list(workspace_id)?;
        servers.retain(|server| server.reference.scope != McpScope::Plugin);
        let projected = self
            .project_plugin_servers(workspace_id)
            .map_err(|error| McpRegistryError::Invalid(error.to_string()))?;
        servers.extend(projected.into_iter().map(|(server, _)| server));
        servers.sort_by(|left, right| left.reference.cmp(&right.reference));
        Ok(servers)
    }

    fn server_readiness(&self, reference: &McpServerReference) -> Option<&'static str> {
        self.failure_states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(reference)
            .copied()
    }

    pub fn get_server(
        &self,
        reference: &McpServerReference,
    ) -> Result<Option<McpServerConfig>, McpRegistryError> {
        if reference.scope != McpScope::Plugin {
            return self.registry.get(reference);
        }
        self.project_plugin_servers(&reference.workspace_id)
            .map_err(|error| McpRegistryError::Invalid(error.to_string()))
            .map(|servers| {
                servers
                    .into_iter()
                    .map(|(server, _)| server)
                    .find(|server| &server.reference == reference)
            })
    }

    pub fn mutate(
        &self,
        rpc_id: &str,
        mutation: &McpManagementMutation,
    ) -> Result<McpMutationReceipt, McpRegistryError> {
        if matches!(mutation, McpManagementMutation::OauthBind { .. }) {
            return Err(McpRegistryError::Invalid(
                "OAuth bind requires the trusted completion authority".to_owned(),
            ));
        }
        let receipt = self.registry.mutate_idempotent(rpc_id, mutation)?;
        self.close_mutation_generations(mutation);
        Ok(receipt)
    }

    /// Completes a pending OAuth ownership transition only after the platform
    /// SecretStore proves the exact credential id is currently active.
    pub fn bind_oauth(
        &self,
        rpc_id: &str,
        reference: &McpServerReference,
        credential_id: &str,
    ) -> Result<McpMutationReceipt, McpRegistryError> {
        let mutation = McpManagementMutation::OauthBind {
            reference: reference.clone(),
            credential_id: credential_id.to_owned(),
        };
        if let Some(receipt) = self.registry.committed_receipt(rpc_id, &mutation)? {
            return Ok(receipt);
        }
        if !matches!(
            self.secret_store.resolve(credential_id),
            Ok(SecretResolution::Active(SecretRecord::Active { .. }))
        ) {
            return Err(McpRegistryError::Invalid(
                "OAuth credential is not active in SecretStore".to_owned(),
            ));
        }
        let receipt = self.registry.mutate_idempotent(rpc_id, &mutation)?;
        self.close_mutation_generations(&mutation);
        Ok(receipt)
    }

    fn close_mutation_generations(&self, mutation: &McpManagementMutation) {
        let reference = match mutation {
            McpManagementMutation::Save { server, .. } => &server.reference,
            McpManagementMutation::Remove { reference }
            | McpManagementMutation::OauthStart { reference, .. }
            | McpManagementMutation::OauthRemove { reference }
            | McpManagementMutation::OauthBind { reference, .. } => reference,
        };
        let keys = {
            let mut bindings = self
                .bindings
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(cached) = bindings.get_mut(&reference.workspace_id) else {
                return;
            };
            cached.fingerprint.clear();
            cached
                .authority
                .upgrade()
                .map_or_else(BTreeSet::new, |authority| {
                    authority
                        .routes
                        .values()
                        .filter(|route| route.server.reference == *reference)
                        .map(|route| route.key.clone())
                        .collect()
                })
        };
        for key in keys {
            let _ = self.broker.handle().remove(key);
        }
    }

    /// Reconciles externally-owned plugin and SecretStore lifecycle changes.
    /// The Slice-14F plugin-management owner must call this after its durable
    /// disable, grant, update, or uninstall commit. Until that owner exists,
    /// execution independently performs the same check before every effect so
    /// a stale authority can never be used.
    pub fn reconcile_authority(&self) -> Result<usize, McpError> {
        let bindings = self
            .bindings
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .iter()
            .filter_map(|(workspace, binding)| {
                binding
                    .authority
                    .upgrade()
                    .map(|authority| (workspace.clone(), authority))
            })
            .collect::<Vec<_>>();
        let mut stale_workspaces = BTreeSet::new();
        let mut stale_keys = BTreeSet::new();
        for (workspace, authority) in bindings {
            for route in authority.routes.values() {
                if authority.validate_route_generation(route).is_err() {
                    stale_workspaces.insert(workspace.clone());
                    stale_keys.insert(route.key.clone());
                }
            }
        }
        for key in &stale_keys {
            self.broker.handle().remove(key.clone())?;
        }
        let mut cache = self
            .bindings
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for workspace in stale_workspaces {
            if let Some(binding) = cache.get_mut(&workspace) {
                binding.fingerprint.clear();
            }
        }
        Ok(stale_keys.len())
    }

    pub fn prepare_workspace(&self, workspace: &str) -> Result<McpWorkspaceBinding, McpError> {
        let _preparation = self
            .preparation
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (servers, plugin_resolutions, registry_failure) =
            self.servers_with_plugins_degraded(workspace)?;
        let fingerprint =
            binding_fingerprint(&servers, &plugin_resolutions, self.secret_store.as_ref())?;
        let cached = self
            .bindings
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(workspace);
        if let Some(binding) = cached {
            let catalog_current = binding.catalog_generations.iter().all(|(key, expected)| {
                self.broker
                    .handle()
                    .catalog_generation(key.clone())
                    .ok()
                    .flatten()
                    == Some(*expected)
            });
            if binding.failures.is_empty()
                && binding.registry_failure.is_none()
                && binding.fingerprint == fingerprint
                && catalog_current
            {
                // The previous worker's authority may be gone; the peers are
                // still pooled and the routes still valid, so rebind rather
                // than restart the servers (a stdio server's task state would
                // not survive a restart).
                let authority = binding.authority.upgrade().unwrap_or_else(|| {
                    Arc::new(McpDynamicSupervisorAuthority::new(
                        self.broker.handle(),
                        binding.routes.clone(),
                        self.secret_access(),
                        self.registry.clone(),
                        self.plugin_resolver.clone(),
                    ))
                });
                let catalog = binding.catalog.clone();
                let failures = binding.failures.clone();
                let mut binding = binding;
                binding.authority = Arc::downgrade(&authority);
                self.bindings
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .insert(workspace.to_owned(), binding);
                return Ok(McpWorkspaceBinding {
                    catalog,
                    authority,
                    failures,
                    registry_failure: None,
                });
            }
            for key in binding.catalog_generations.into_keys() {
                let _ = self.broker.handle().remove(key);
            }
        }
        let (tools, routes, catalog_generations, failures) = with_preparation_runtime(|runtime| {
            let mut tools = Vec::new();
            let mut routes = BTreeMap::new();
            let mut catalog_generations = BTreeMap::new();
            let mut failures = Vec::new();
            for (server, plugin) in servers.iter().zip(&plugin_resolutions) {
                if let Err(error) = self.require_bound_credentials(server) {
                    eprintln!(
                        "mcp-server-refresh-failed: server={} code=credential-unavailable",
                        server.reference.name
                    );
                    let _ = error;
                    failures.push(McpServerFailure {
                        server: server.reference.clone(),
                        code: "credential-unavailable",
                        detail: None,
                    });
                    self.failure_states
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .insert(server.reference.clone(), "credential-unavailable");
                    continue;
                }
                let prepared =
                    match prepare_server(server, plugin.as_ref(), self.secret_access(), runtime) {
                        Ok(prepared) => prepared,
                        Err(error) => {
                            eprintln!(
                                "mcp-server-refresh-failed: server={} code=refresh-failed",
                                server.reference.name
                            );
                            let _ = error;
                            failures.push(McpServerFailure {
                                server: server.reference.clone(),
                                code: "refresh-failed",
                                detail: None,
                            });
                            self.failure_states
                                .lock()
                                .unwrap_or_else(std::sync::PoisonError::into_inner)
                                .insert(server.reference.clone(), "refresh-failed");
                            continue;
                        }
                    };
                self.failure_states
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .remove(&server.reference);
                // One server's catalog that cannot be projected (a schema
                // keyword the profile refuses, a broken external-effect
                // contract) drops that server alone: the worker still launches
                // with every other server's tools, and the session learns
                // which server, which tool and why through the notice.
                match self.bind_prepared_server(server, plugin.as_ref(), prepared, &routes) {
                    Ok(bound) => {
                        routes.extend(bound.routes);
                        catalog_generations.insert(bound.key, bound.generation);
                        tools.extend(bound.tools);
                    }
                    Err(error) => {
                        let detail = error.to_string();
                        eprintln!(
                            "mcp-server-projection-failed: server={} code=projection-failed reason={}",
                            server.reference.name,
                            detail.replace('\n', " ")
                        );
                        failures.push(McpServerFailure {
                            server: server.reference.clone(),
                            code: "projection-failed",
                            detail: Some(detail),
                        });
                        self.failure_states
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner)
                            .insert(server.reference.clone(), "projection-failed");
                    }
                }
            }
            Ok((tools, routes, catalog_generations, failures))
        })?;
        let catalog = DynamicToolCatalog::resolve(tools)
            .map_err(|error| McpError::Conflict(error.to_string()))?;
        let authority = Arc::new(McpDynamicSupervisorAuthority::new(
            self.broker.handle(),
            routes.clone(),
            self.secret_access(),
            self.registry.clone(),
            self.plugin_resolver.clone(),
        ));
        let (stable_servers, stable_plugins, _) = self.servers_with_plugins_degraded(workspace)?;
        let stable_fingerprint =
            binding_fingerprint(&stable_servers, &stable_plugins, self.secret_store.as_ref())?;
        if stable_fingerprint != fingerprint {
            return Err(McpError::Conflict(
                "MCP authority changed while its generation was prepared".to_owned(),
            ));
        }
        self.bindings
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(
                workspace.to_owned(),
                CachedBinding {
                    fingerprint,
                    catalog: catalog.clone(),
                    authority: Arc::downgrade(&authority),
                    routes,
                    catalog_generations,
                    failures: failures.clone(),
                    registry_failure: registry_failure.clone(),
                },
            );
        Ok(McpWorkspaceBinding {
            catalog,
            authority,
            failures,
            registry_failure,
        })
    }

    /// Project one prepared server's catalog, capture its routes and pool the
    /// peer. Any failure leaves the peer unregistered (it is dropped with
    /// `prepared`) and the shared `routes` untouched.
    fn bind_prepared_server(
        &self,
        server: &McpServerConfig,
        plugin: Option<&ResolvedPluginExecutable>,
        prepared: PreparedServer,
        existing_routes: &BTreeMap<String, McpRoute>,
    ) -> Result<BoundServer, McpError> {
        let projected = project_catalog(&server.reference.name, &prepared.tools, server.always_on)
            .map_err(|error| McpError::Conflict(error.to_string()))?;
        let remote_names = prepared
            .tools
            .iter()
            .map(|remote| {
                mcp::project_name(&server.reference.name, &remote.name)
                    .map(|projected| (projected, remote.name.clone()))
                    .map_err(|error| McpError::Conflict(error.to_string()))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        let mut routes = BTreeMap::new();
        for dynamic in &projected.tools {
            let remote_name = remote_names.get(&dynamic.name).ok_or_else(|| {
                McpError::Conflict(format!(
                    "projected MCP tool {} lost its remote identity",
                    dynamic.name
                ))
            })?;
            let effect_tool = prepared
                .tools
                .iter()
                .find(|tool| tool.name == *remote_name)
                .ok_or_else(|| {
                    McpError::Conflict(format!(
                        "projected MCP tool {} lost its source contract",
                        dynamic.name
                    ))
                })?;
            let reconcile_tool = dynamic
                .external_effect
                .as_ref()
                .map(|binding| {
                    prepared
                        .tools
                        .iter()
                        .find(|tool| tool.name == binding.reconcile_tool)
                        .cloned()
                        .ok_or_else(|| {
                            McpError::Conflict(format!(
                                "projected MCP tool {} lost reconciliation contract {}",
                                dynamic.name, binding.reconcile_tool
                            ))
                        })
                })
                .transpose()?;
            let contract = McpRouteContract::capture(
                prepared.peer.as_ref(),
                effect_tool.clone(),
                reconcile_tool,
            )?;
            if existing_routes.contains_key(&dynamic.name)
                || routes
                    .insert(
                        dynamic.name.clone(),
                        McpRoute {
                            key: prepared.key.clone(),
                            remote_name: remote_name.clone(),
                            credential_floors: prepared.credential_floors.clone(),
                            server: server.clone(),
                            plugin_generation: plugin
                                .map(|plugin| plugin.plugin_generation.clone()),
                            contract,
                        },
                    )
                    .is_some()
            {
                return Err(McpError::Conflict(format!(
                    "duplicate MCP route {}",
                    dynamic.name
                )));
            }
        }
        self.broker
            .handle()
            .register(prepared.key.clone(), prepared.peer, server.always_on)?;
        let generation = self
            .broker
            .handle()
            .catalog_generation(prepared.key.clone())?
            .ok_or_else(|| McpError::Transport("MCP peer disappeared after register".to_owned()))?;
        Ok(BoundServer {
            key: prepared.key,
            generation,
            routes,
            tools: projected.tools,
        })
    }

    fn resolve_plugins(
        &self,
        servers: &[McpServerConfig],
    ) -> Result<Vec<Option<ResolvedPluginExecutable>>, McpError> {
        servers
            .iter()
            .map(|server| {
                let Some(reference) = &server.plugin_component else {
                    return Ok(None);
                };
                let resolver = self.plugin_resolver.as_ref().ok_or_else(|| {
                    McpError::Conflict("plugin MCP resolver is unavailable".to_owned())
                })?;
                let resolved = resolver
                    .resolve(reference)
                    .map_err(McpError::Conflict)?
                    .ok_or_else(|| {
                        McpError::Conflict(format!(
                            "plugin MCP component {}/{} is not enabled and fully granted",
                            reference.plugin_id, reference.component_id
                        ))
                    })?;
                if resolved.component_type != ComponentKind::McpServer {
                    return Err(McpError::Conflict(
                        "plugin component is not an MCP server".to_owned(),
                    ));
                }
                Ok(Some(resolved))
            })
            .collect()
    }

    fn project_plugin_servers(
        &self,
        workspace: &str,
    ) -> Result<Vec<(McpServerConfig, ResolvedPluginExecutable)>, McpError> {
        let Some(resolver) = &self.plugin_resolver else {
            return Ok(Vec::new());
        };
        let mut projected = resolver.components().map_err(McpError::Conflict)?;
        projected.sort_by(|left, right| left.reference.cmp(&right.reference));
        let mut names = BTreeSet::new();
        projected
            .into_iter()
            .map(|resolved| {
                if resolved.component_type != ComponentKind::McpServer {
                    return Err(McpError::Conflict(
                        "plugin projection returned a non-MCP component".to_owned(),
                    ));
                }
                let name = resolved.reference.component_id.clone();
                if !names.insert(name.clone()) {
                    return Err(McpError::Conflict(format!(
                        "plugin MCP server name {name} is duplicated"
                    )));
                }
                let server = McpServerConfig {
                    reference: McpServerReference {
                        workspace_id: workspace.to_owned(),
                        scope: McpScope::Plugin,
                        name,
                    },
                    transport: McpTransportConfig::Stdio {
                        command: Vec::new(),
                        cwd: None,
                        environment: BTreeMap::new(),
                    },
                    enabled: true,
                    always_on: false,
                    protocol_mode: ProtocolMode::Auto,
                    owner: Some(resolved.reference.plugin_id.clone()),
                    plugin_component: Some(resolved.reference.clone()),
                    project_trusted: false,
                };
                Ok((server, resolved))
            })
            .collect()
    }

    fn servers_with_plugins(
        &self,
        workspace: &str,
    ) -> Result<(Vec<McpServerConfig>, Vec<Option<ResolvedPluginExecutable>>), McpError> {
        let (servers, plugins, registry_failure) = self.servers_with_plugins_degraded(workspace)?;
        match registry_failure {
            Some(message) => Err(McpError::Conflict(message)),
            None => Ok((servers, plugins)),
        }
    }

    /// Like `servers_with_plugins`, but an unreadable or unresolvable
    /// standalone registry degrades to "no registry-configured servers" and
    /// returns the failure message instead of refusing the workspace. A
    /// worker launch must not be blocked by a hand-edited registry file; the
    /// supervisor turns the message into a session notice.
    fn servers_with_plugins_degraded(
        &self,
        workspace: &str,
    ) -> Result<DegradedWorkspaceServers, McpError> {
        let (configured, registry_failure) = match self.registry.load() {
            Ok(mut registry) => {
                // Plugin rows are never trusted from the standalone registry.
                // They are synthesized exclusively from the verified Slice-12
                // lifecycle store.
                registry
                    .servers
                    .retain(|server| server.reference.scope != McpScope::Plugin);
                match registry.resolve(workspace) {
                    Ok(resolved) => (resolved.into_iter().cloned().collect::<Vec<_>>(), None),
                    Err(error) => (Vec::new(), Some(error.to_string())),
                }
            }
            Err(error) => (Vec::new(), Some(error.to_string())),
        };
        if let Some(message) = &registry_failure {
            eprintln!("mcp-registry-unavailable: workspace={workspace} error={message}");
        }
        let configured_plugins = self.resolve_plugins(&configured)?;
        let mut pairs = configured
            .into_iter()
            .zip(configured_plugins)
            .collect::<Vec<_>>();
        for (server, resolution) in self.project_plugin_servers(workspace)? {
            if pairs
                .iter()
                .any(|(existing, _)| existing.reference.name == server.reference.name)
            {
                return Err(McpError::Conflict(format!(
                    "plugin MCP server {} collides with another scope",
                    server.reference.name
                )));
            }
            pairs.push((server, Some(resolution)));
        }
        pairs.sort_by(|left, right| left.0.reference.name.cmp(&right.0.reference.name));
        let (servers, plugins): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
        Ok((servers, plugins, registry_failure))
    }

    fn require_bound_credentials(&self, server: &McpServerConfig) -> Result<(), McpError> {
        for (field, credential_id) in credential_fields(server) {
            let state = self
                .registry
                .credential_reference_state(&server.reference, &field, &credential_id)
                .map_err(|error| McpError::Conflict(error.to_string()))?;
            if state != Some(McpCredentialReferenceState::Bound) {
                return Err(McpError::Transport(format!(
                    "MCP credential reference {field} is not bound"
                )));
            }
        }
        Ok(())
    }

    pub fn probe_server(&self, reference: &McpServerReference) -> Result<IJsonValue, McpError> {
        let server = self
            .get_server(reference)
            .map_err(|error| McpError::Conflict(error.to_string()))?
            .ok_or_else(|| McpError::Conflict("MCP server was not found".to_owned()))?;
        self.require_bound_credentials(&server)?;
        let plugins = self.resolve_plugins(std::slice::from_ref(&server))?;
        with_preparation_runtime(|runtime| {
            let mut prepared =
                prepare_server(&server, plugins[0].as_ref(), self.secret_access(), runtime)?;
            let capabilities = prepared.peer.capabilities().clone();
            let server_name = prepared.peer.server_name().to_owned();
            let tools = prepared
                .tools
                .iter()
                .map(|tool| tool.name.clone())
                .collect::<Vec<_>>();
            runtime.block_on(prepared.peer.close())?;
            json_to_ijson(&json!({
                "server":{"name":server_name},
                "capabilities":capabilities,
                "catalog":{"toolCount":tools.len(),"tools":tools}
            }))
            .map_err(McpError::Protocol)
        })
    }

    /// Reads one live MCP resource generation without publishing a worker
    /// binding. The caller must keep the returned generation token in its
    /// page cursor; a later call with a different token is stale.
    pub fn list_resource_catalog(
        &self,
        workspace: &str,
    ) -> Result<(String, Vec<McpResourceBinding>), McpError> {
        let _preparation = self
            .preparation
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (servers, plugins) = self.servers_with_plugins(workspace)?;
        let generation = binding_fingerprint(&servers, &plugins, self.secret_store.as_ref())?;
        let mut items = with_preparation_runtime(|runtime| {
            let mut items = Vec::new();
            for (server, plugin) in servers.iter().zip(&plugins) {
                self.require_bound_credentials(server)?;
                let mut prepared =
                    prepare_server(server, plugin.as_ref(), self.secret_access(), runtime)?;
                items.extend(collect_prepared_resources(
                    runtime,
                    &mut prepared,
                    server,
                    &self.resource_instance,
                )?);
            }
            Ok(items)
        })?;
        let (stable_servers, stable_plugins) = self.servers_with_plugins(workspace)?;
        if binding_fingerprint(&stable_servers, &stable_plugins, self.secret_store.as_ref())?
            != generation
        {
            return Err(McpError::Conflict(
                "MCP resource generation changed during the read".to_owned(),
            ));
        }
        items.sort_by(|left, right| {
            left.server
                .cmp(&right.server)
                .then_with(|| left.resource.uri.cmp(&right.resource.uri))
        });
        let page_generation = format!(
            "sha256-{:x}",
            Sha256::digest(
                serde_json_canonicalizer::to_vec(
                    &items
                        .iter()
                        .map(|item| (&item.server, &item.binding_digest))
                        .collect::<Vec<_>>()
                )
                .map_err(|error| McpError::Protocol(error.to_string()))?
            )
        );
        Ok((format!("{generation}.{page_generation}"), items))
    }

    pub fn read_resource(
        &self,
        reference: &McpServerReference,
        binding_digest: &str,
        uri: &str,
    ) -> Result<IJsonValue, McpError> {
        let _preparation = self
            .preparation
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let server = self
            .get_server(reference)
            .map_err(|error| McpError::Conflict(error.to_string()))?
            .ok_or_else(|| McpError::Conflict("MCP server was not found".to_owned()))?;
        self.require_bound_credentials(&server)?;
        let plugin = self.resolve_plugins(std::slice::from_ref(&server))?;
        with_preparation_runtime(|runtime| {
            let mut prepared =
                prepare_server(&server, plugin[0].as_ref(), self.secret_access(), runtime)?;
            read_prepared_resource(
                runtime,
                &mut prepared,
                reference,
                binding_digest,
                uri,
                &self.resource_instance,
            )
        })
    }
}

fn close_prepared_result<T>(
    runtime: &tokio::runtime::Runtime,
    prepared: &mut PreparedServer,
    result: Result<T, McpError>,
) -> Result<T, McpError> {
    let close = runtime.block_on(prepared.peer.close());
    match (result, close) {
        (Err(error), _) => Err(error),
        (Ok(value), Ok(())) => Ok(value),
        (Ok(_), Err(error)) => Err(error),
    }
}

fn collect_prepared_resources(
    runtime: &tokio::runtime::Runtime,
    prepared: &mut PreparedServer,
    server: &McpServerConfig,
    runtime_instance: &str,
) -> Result<Vec<McpResourceBinding>, McpError> {
    let result = (|| {
        if !prepared.peer.capabilities().resources {
            return Ok(Vec::new());
        }
        let resources = runtime.block_on(prepared.peer.list_resources())?;
        let catalog_generation = resource_catalog_generation(
            runtime_instance,
            prepared.peer.catalog_generation(),
            &resources,
        )?;
        let binding_digest =
            resource_binding_digest(&server.reference, prepared, &catalog_generation)?;
        Ok(resources
            .into_iter()
            .map(|resource| McpResourceBinding {
                server: server.reference.clone(),
                binding_digest: binding_digest.clone(),
                resource,
            })
            .collect())
    })();
    close_prepared_result(runtime, prepared, result)
}

fn read_prepared_resource(
    runtime: &tokio::runtime::Runtime,
    prepared: &mut PreparedServer,
    reference: &McpServerReference,
    binding_digest: &str,
    uri: &str,
    runtime_instance: &str,
) -> Result<IJsonValue, McpError> {
    let result = (|| {
        if !prepared.peer.capabilities().resources {
            return Err(McpError::Conflict(
                "MCP server does not advertise resources".to_owned(),
            ));
        }
        let resources = runtime.block_on(prepared.peer.list_resources())?;
        let catalog_generation = resource_catalog_generation(
            runtime_instance,
            prepared.peer.catalog_generation(),
            &resources,
        )?;
        if resource_binding_digest(reference, prepared, &catalog_generation)? != binding_digest {
            return Err(McpError::Conflict(
                "MCP resource binding is stale".to_owned(),
            ));
        }
        if !resources.iter().any(|resource| resource.uri == uri) {
            return Err(McpError::Conflict(
                "MCP resource binding is stale".to_owned(),
            ));
        }
        runtime.block_on(prepared.peer.read_resource(uri))
    })();
    close_prepared_result(runtime, prepared, result)
}

fn resource_binding_digest(
    server: &McpServerReference,
    prepared: &PreparedServer,
    catalog_generation: &str,
) -> Result<String, McpError> {
    let server_identity = prepared
        .peer
        .server_identity()
        .ok_or_else(|| McpError::Protocol("MCP handshake omitted server identity".to_owned()))?;
    let preimage = json!({
        "server":server,
        "configDigest":prepared.key.config_digest,
        "protocolVersion":prepared.peer.protocol_version(),
        "serverIdentity":server_identity,
        "authorizationIdentity":authorization_identity(&prepared.credential_floors),
        "catalogGeneration":catalog_generation,
    });
    Ok(format!(
        "sha256-{:x}",
        Sha256::digest(
            serde_json_canonicalizer::to_vec(&preimage)
                .map_err(|error| McpError::Protocol(error.to_string()))?
        )
    ))
}

fn resource_catalog_generation(
    runtime_instance: &str,
    peer_generation: u64,
    resources: &[McpResource],
) -> Result<String, McpError> {
    let preimage = json!({
        "runtimeInstance":runtime_instance,
        "peerGeneration":peer_generation,
        "resources":resources,
    });
    Ok(format!(
        "sha256-{:x}",
        Sha256::digest(
            serde_json_canonicalizer::to_vec(&preimage)
                .map_err(|error| McpError::Protocol(error.to_string()))?
        )
    ))
}

/// Runs the short-lived MCP connection runtime on its own OS thread. Supervisor
/// calls arrive from Tokio tasks, where constructing, blocking on, or dropping
/// another runtime would panic. The scoped thread also guarantees every early
/// error path drops the preparation runtime outside any async executor.
fn with_preparation_runtime<T, F>(operation: F) -> Result<T, McpError>
where
    T: Send,
    F: FnOnce(&tokio::runtime::Runtime) -> Result<T, McpError> + Send,
{
    thread::scope(|scope| {
        scope
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(|error| McpError::Transport(error.to_string()))?;
                operation(&runtime)
            })
            .join()
            .map_err(|_| McpError::Transport("MCP preparation runtime failed".to_owned()))?
    })
}

pub const MCP_MANAGEMENT_METHODS: [&str; 7] = [
    "mcp.list",
    "mcp.get",
    "mcp.save",
    "mcp.remove",
    "mcp.probe",
    "mcp.oauth.start",
    "mcp.oauth.remove",
];

pub struct McpManagementRoutes {
    runtime: Arc<McpRuntime>,
}

impl McpManagementRoutes {
    #[must_use]
    pub fn new(runtime: Arc<McpRuntime>) -> Self {
        Self { runtime }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct McpListRequest {
    workspace_id: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct McpReferenceRequest {
    reference: McpServerReference,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct McpSaveRequest {
    server: McpServerConfig,
    credential_fields: BTreeMap<String, McpCredentialFieldOperation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct McpOauthStartRequest {
    reference: McpServerReference,
    credential_id: String,
    authorization_url: String,
}

impl ProductionEndpointRoutes for McpManagementRoutes {
    fn capabilities(&self) -> BTreeSet<String> {
        MCP_MANAGEMENT_METHODS
            .into_iter()
            .map(str::to_owned)
            .collect()
    }

    fn extension_method_class(&self, method: &str) -> Option<endpoint::MethodClass> {
        match method {
            "mcp.list" | "mcp.get" | "mcp.probe" => Some(endpoint::MethodClass::ReadOnly),
            "mcp.save" | "mcp.remove" | "mcp.oauth.start" | "mcp.oauth.remove" => {
                Some(endpoint::MethodClass::Mutation)
            }
            _ => None,
        }
    }

    fn validate_extension_payload(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        match operation {
            "mcp.list" => parse::<McpListRequest>(payload).map(|_| ()),
            "mcp.get" | "mcp.remove" | "mcp.probe" | "mcp.oauth.remove" => {
                parse::<McpReferenceRequest>(payload).map(|_| ())
            }
            "mcp.save" => parse::<McpSaveRequest>(payload).map(|_| ()),
            "mcp.oauth.start" => parse::<McpOauthStartRequest>(payload).map(|_| ()),
            _ => Err(route_failure(
                "unsupported-capability",
                "MCP management method is unavailable",
                json!({"operation":operation}),
            )),
        }
    }

    fn extension_failure_is_exact(
        &self,
        _operation: &str,
        failure: &ProductionRouteFailure,
    ) -> bool {
        matches!(
            failure.code.as_str(),
            "bad-request"
                | "unsupported-capability"
                | "internal"
                | "mcp-not-found"
                | "mcp-read-only"
                | "idempotency-conflict"
                | "mcp-unavailable"
        )
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
        _principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        match request.operation.as_str() {
            "mcp.list" => {
                let input: McpListRequest = parse(payload)?;
                let servers = self
                    .runtime
                    .list_servers(&input.workspace_id)
                    .map_err(map_registry_failure)?;
                let servers = servers.into_iter().map(|server| {
                    let readiness = self.runtime.server_readiness(&server.reference).map_or_else(
                        || json!({"state":"ready"}),
                        |code| json!({"state":"refresh-failed","code":code}),
                    );
                    json!({"editable":server.reference.scope != mcp::McpScope::Plugin,"readiness":readiness,"server":server})
                }).collect::<Vec<_>>();
                json_to_ijson(&json!({"servers":servers})).map_err(internal_failure)
            }
            "mcp.get" => {
                let input: McpReferenceRequest = parse(payload)?;
                let server = self
                    .runtime
                    .get_server(&input.reference)
                    .map_err(map_registry_failure)?
                    .ok_or_else(not_found)?;
                json_to_ijson(&json!({"editable":server.reference.scope != mcp::McpScope::Plugin,"server":server})).map_err(internal_failure)
            }
            "mcp.save" => {
                let input: McpSaveRequest = parse(payload)?;
                mutation_response(
                    request,
                    self.runtime.mutate(
                        &request.rpc_id,
                        &McpManagementMutation::Save {
                            server: input.server,
                            credential_fields: input.credential_fields,
                        },
                    ),
                )
            }
            "mcp.remove" => {
                let input: McpReferenceRequest = parse(payload)?;
                mutation_response(
                    request,
                    self.runtime.mutate(
                        &request.rpc_id,
                        &McpManagementMutation::Remove {
                            reference: input.reference,
                        },
                    ),
                )
            }
            "mcp.probe" => {
                let input: McpReferenceRequest = parse(payload)?;
                self.runtime
                    .probe_server(&input.reference)
                    .map_err(|error| {
                        route_failure(
                            "mcp-unavailable",
                            "MCP probe failed",
                            json!({"reason":error.to_string()}),
                        )
                    })
            }
            "mcp.oauth.start" => {
                let input: McpOauthStartRequest = parse(payload)?;
                mutation_response(
                    request,
                    self.runtime.mutate(
                        &request.rpc_id,
                        &McpManagementMutation::OauthStart {
                            reference: input.reference,
                            credential_id: input.credential_id,
                            authorization_url: input.authorization_url,
                        },
                    ),
                )
            }
            "mcp.oauth.remove" => {
                let input: McpReferenceRequest = parse(payload)?;
                mutation_response(
                    request,
                    self.runtime.mutate(
                        &request.rpc_id,
                        &McpManagementMutation::OauthRemove {
                            reference: input.reference,
                        },
                    ),
                )
            }
            _ => Err(route_failure(
                "unsupported-capability",
                "MCP management method is unavailable",
                json!({"operation":request.operation}),
            )),
        }
    }
}

fn parse<T: DeserializeOwned>(payload: &Value) -> Result<T, ProductionRouteFailure> {
    serde_json::from_value(payload.clone()).map_err(|error| {
        route_failure(
            "bad-request",
            "Invalid MCP management request",
            json!({"reason":error.to_string()}),
        )
    })
}

fn mutation_response(
    request: &EndpointHostCall,
    result: Result<McpMutationReceipt, McpRegistryError>,
) -> Result<IJsonValue, ProductionRouteFailure> {
    let receipt = result.map_err(map_registry_failure)?;
    request
        .handoff
        .mark_handed_off(endpoint::DurableHandoffProof {
            delivery: "mcp-registry-barrier".to_owned(),
            durable_identity: Some(endpoint::RpcDurableIdentity {
                kind: "mcp-rpc".to_owned(),
                id: request.rpc_id.clone(),
                seq: None,
            }),
        })
        .map_err(|_| internal_failure("MCP durable handoff proof failed".to_owned()))?;
    json_to_ijson(&json!({"receipt":receipt})).map_err(internal_failure)
}

fn map_registry_failure(error: McpRegistryError) -> ProductionRouteFailure {
    match error {
        McpRegistryError::Invalid(reason) => route_failure(
            "bad-request",
            "Invalid MCP management request",
            json!({"reason":reason}),
        ),
        McpRegistryError::Conflict(reason) => route_failure(
            "idempotency-conflict",
            "MCP management request conflicts with durable authority",
            json!({"reason":reason}),
        ),
        McpRegistryError::ReadOnlyPlugin => route_failure(
            "mcp-read-only",
            "Plugin-owned MCP configuration is read-only",
            json!({}),
        ),
        McpRegistryError::NotFound => not_found(),
        McpRegistryError::Io(_)
        | McpRegistryError::Store(_)
        | McpRegistryError::InjectedFault(_) => route_failure(
            "mcp-unavailable",
            "MCP management authority is unavailable",
            json!({}),
        ),
    }
}

fn not_found() -> ProductionRouteFailure {
    route_failure("mcp-not-found", "MCP server was not found", json!({}))
}
fn internal_failure(reason: String) -> ProductionRouteFailure {
    route_failure(
        "mcp-unavailable",
        "MCP response encoding failed",
        json!({"reason":reason}),
    )
}
fn route_failure(code: &str, message: &str, details: Value) -> ProductionRouteFailure {
    ProductionRouteFailure::new(
        code,
        message,
        json_to_ijson(&details).expect("static route failure is I-JSON"),
    )
}
fn json_to_ijson(value: &Value) -> Result<IJsonValue, String> {
    IJsonValue::parse(&serde_json::to_vec(value).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())
}

fn binding_fingerprint(
    servers: &[McpServerConfig],
    plugins: &[Option<ResolvedPluginExecutable>],
    secrets: &dyn SecretStore,
) -> Result<String, McpError> {
    let bytes = serde_json_canonicalizer::to_vec(&servers)
        .map_err(|error| McpError::Protocol(error.to_string()))?;
    let mut credential_ids = BTreeSet::new();
    for server in servers {
        match &server.transport {
            McpTransportConfig::Stdio { environment, .. } => {
                for value in environment.values() {
                    if let CredentialValue::Credential { credential } = value {
                        credential_ids.insert(credential.as_str());
                    }
                }
            }
            McpTransportConfig::Http { headers, oauth, .. } => {
                for value in headers.values() {
                    if let CredentialValue::Credential { credential } = value {
                        credential_ids.insert(credential.as_str());
                    }
                }
                if let Some(credential) = oauth {
                    credential_ids.insert(credential.as_str());
                }
            }
        }
    }
    let mut hasher = Sha256::new();
    hasher.update(b"tekes-mcp-binding-v1\0");
    hasher.update(bytes);
    for plugin in plugins.iter().flatten() {
        hasher.update(plugin.plugin_generation.as_bytes());
        hasher.update([0]);
        hasher.update(plugin.executable_path.as_os_str().as_encoded_bytes());
        hasher.update([0]);
    }
    for credential in credential_ids {
        hasher.update(credential.as_bytes());
        hasher.update([0]);
        match secrets.resolve(credential) {
            Ok(SecretResolution::Active(record)) => {
                hasher.update(b"active\0");
                hasher.update(record.generation().to_be_bytes());
            }
            Ok(SecretResolution::Revoked { generation }) => {
                hasher.update(b"revoked\0");
                hasher.update(generation.to_be_bytes());
            }
            Ok(SecretResolution::NotFound) => hasher.update(b"not-found\0"),
            Err(_) => hasher.update(b"unavailable\0"),
        }
        hasher.update([0]);
    }
    Ok(format!("sha256-{:x}", hasher.finalize()))
}

/// One server's share of a workspace binding once its catalog projected.
struct BoundServer {
    key: McpPoolKey,
    generation: u64,
    routes: BTreeMap<String, McpRoute>,
    tools: Vec<DynamicTool>,
}

struct PreparedServer {
    key: McpPoolKey,
    peer: Box<dyn McpPeer>,
    tools: Vec<mcp::McpTool>,
    credential_floors: Vec<(String, u64)>,
}

fn prepare_server(
    config: &McpServerConfig,
    plugin: Option<&ResolvedPluginExecutable>,
    secrets: SecretAccess,
    runtime: &tokio::runtime::Runtime,
) -> Result<PreparedServer, McpError> {
    prepare_server_cancellable(
        config,
        plugin,
        secrets,
        runtime,
        McpCancellationToken::default(),
    )
}

fn prepare_server_cancellable(
    config: &McpServerConfig,
    plugin: Option<&ResolvedPluginExecutable>,
    secrets: SecretAccess,
    runtime: &tokio::runtime::Runtime,
    cancellation: McpCancellationToken,
) -> Result<PreparedServer, McpError> {
    if cancellation.is_cancelled() {
        return Err(McpError::Cancelled);
    }
    let _runtime_guard = runtime.enter();
    let config_bytes = serde_json_canonicalizer::to_vec(config)
        .map_err(|error| McpError::Protocol(error.to_string()))?;
    let config_digest = format!("{:x}", Sha256::digest(&config_bytes));
    let (mut peer, credential_floors): (Box<dyn McpPeer>, Vec<(String, u64)>) = match &config
        .transport
    {
        McpTransportConfig::Stdio {
            command,
            cwd,
            environment,
        } => {
            let mut material = SecretMaterial::resolve_map(environment, secrets.store.as_ref())?;
            let plugin_command;
            let plugin_cwd;
            let (command, cwd) = if let Some(plugin) = plugin {
                plugin_command = vec![plugin.executable_path.to_string_lossy().into_owned()];
                plugin_cwd = plugin.data_path.clone();
                (&plugin_command, Some(plugin_cwd))
            } else {
                (command, cwd.as_ref().map(PathBuf::from))
            };
            let transport = StdioTransport::spawn(command, cwd, &material.values)?;
            material.zeroize();
            let mut client = McpClient::new(config.reference.name.clone(), transport);
            runtime
                .block_on(client.connect_cancellable(config.protocol_mode, cancellation.clone()))?;
            (Box::new(client), material.authority)
        }
        McpTransportConfig::Http {
            url,
            headers,
            oauth,
        } => {
            let mut material = SecretMaterial::resolve_map(headers, secrets.store.as_ref())?;
            let mut authority = material.authority.clone();
            material.zeroize();
            // A Kernel-minted OAuth grant behind the `oauth` field is
            // exchanged for access tokens (mcp-runtime §oauth); any
            // other active material is a platform-installed token and is
            // injected as-is.
            let mut exchange: Option<Arc<provider::OAuthTokenExchange>> = None;
            if let Some(credential) = oauth {
                let (mut transient, generation) =
                    resolve_secret(credential, secrets.store.as_ref())?;
                let kernel_grant = provider::OAuthGrant::decode_material(&transient).is_some();
                transient.zeroize();
                if kernel_grant {
                    let mutation = secrets.mutation.clone().ok_or_else(|| {
                        McpError::Transport(
                            "OAuth refresh grant requires the Kernel secret mutation authority"
                                .to_owned(),
                        )
                    })?;
                    exchange = Some(Arc::new(provider::OAuthTokenExchange::new(
                        credential.clone(),
                        Arc::clone(&secrets.store),
                        mutation,
                    )));
                }
                authority.push((credential.clone(), generation));
            }
            authority.sort();
            authority.dedup();
            let parsed = url
                .parse()
                .map_err(|error: url::ParseError| McpError::Transport(error.to_string()))?;
            let configured_headers = headers.clone();
            let oauth_id = oauth.clone();
            let request_oauth = oauth.clone();
            let request_secrets = Arc::clone(&secrets.store);
            let request_exchange = exchange.clone();
            let transport = HttpTransport::new_with_request_authorization_provider(
                parsed,
                &BTreeMap::new(),
                move || {
                    http_request_authorization(
                        &configured_headers,
                        request_oauth.as_deref(),
                        request_secrets.as_ref(),
                        request_exchange.as_deref(),
                    )
                },
                false,
            )?;
            let mut client = McpClient::new(config.reference.name.clone(), transport);
            runtime
                .block_on(client.connect_cancellable(config.protocol_mode, cancellation.clone()))?;
            if let (Some(exchange), Some(credential)) = (exchange.as_ref(), oauth_id.as_ref()) {
                // The connect exchanged (and may have rotated) the grant:
                // the credential floor of this peer is the generation the
                // store holds now, so a reconnect on the same grant matches.
                if let Some(generation) = exchange.cached_generation() {
                    for entry in &mut authority {
                        if entry.0 == *credential {
                            entry.1 = generation;
                        }
                    }
                }
            }
            (Box::new(client), authority)
        }
    };
    let tools = match runtime.block_on(async {
        tokio::select! {
            biased;
            () = cancellation.cancelled() => Err(McpError::Cancelled),
            result = peer.list_tools() => result,
        }
    }) {
        Ok(tools) => tools,
        Err(error) => {
            let _ = runtime.block_on(peer.close());
            return Err(error);
        }
    };
    let authorization_identity = authorization_identity(&credential_floors);
    let key = McpPoolKey {
        workspace: config.reference.workspace_id.clone(),
        scope: scope_name(config.reference.scope).to_owned(),
        server: config.reference.name.clone(),
        config_digest,
        authorization_identity,
        protocol_mode: protocol_name(config.protocol_mode).to_owned(),
        plugin_generation: plugin.map(|value| value.plugin_generation.clone()),
    };
    Ok(PreparedServer {
        key,
        peer,
        tools,
        credential_floors,
    })
}

fn http_request_authorization(
    configured_headers: &BTreeMap<String, CredentialValue>,
    oauth: Option<&str>,
    secrets: &dyn SecretStore,
    exchange: Option<&provider::OAuthTokenExchange>,
) -> Result<HttpRequestAuthorization, McpError> {
    let mut material = SecretMaterial::resolve_map(configured_headers, secrets)?;
    let mut authority = material.authority.clone();
    let mut bearer = None;
    let mut oauth_identity = None;
    if let Some(credential) = oauth {
        match exchange {
            Some(exchange) => {
                // Refresh exchange, cached per stored generation; a 400 from
                // the token endpoint is the legacy exact failure and closes
                // the record (no retry).
                let (identity, token, generation) = exchange
                    .bearer()
                    .map_err(|error| McpError::Transport(error.to_string()))?;
                authority.push((credential.to_owned(), generation));
                bearer = Some(token);
                oauth_identity = Some(identity);
            }
            None => {
                let (value, generation) = resolve_secret(credential, secrets)?;
                authority.push((credential.to_owned(), generation));
                bearer = Some(value);
            }
        }
    }
    authority.sort();
    authority.dedup();
    Ok(HttpRequestAuthorization {
        identity: oauth_identity.unwrap_or_else(|| authorization_identity(&authority)),
        headers: std::mem::take(&mut material.values),
        bearer,
    })
}

fn credential_fields(server: &McpServerConfig) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    match &server.transport {
        McpTransportConfig::Stdio { environment, .. } => {
            for (name, value) in environment {
                if let CredentialValue::Credential { credential } = value {
                    fields.insert(format!("environment.{name}"), credential.clone());
                }
            }
        }
        McpTransportConfig::Http { headers, oauth, .. } => {
            for (name, value) in headers {
                if let CredentialValue::Credential { credential } = value {
                    fields.insert(format!("headers.{name}"), credential.clone());
                }
            }
            if let Some(credential) = oauth {
                fields.insert("oauth".to_owned(), credential.clone());
            }
        }
    }
    fields
}

pub(crate) fn current_operating_system_version() -> String {
    #[cfg(target_os = "macos")]
    if let Ok(output) = std::process::Command::new("/usr/bin/sw_vers")
        .arg("-productVersion")
        .output()
    {
        if output.status.success() {
            let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            if !value.is_empty() {
                return value;
            }
        }
    }
    "0".to_owned()
}

fn current_plugin_architecture() -> String {
    if cfg!(target_os = "macos") && std::env::consts::ARCH == "aarch64" {
        "arm64".to_owned()
    } else {
        std::env::consts::ARCH.to_owned()
    }
}

struct SecretMaterial {
    values: BTreeMap<String, String>,
    authority: Vec<(String, u64)>,
}

impl SecretMaterial {
    fn resolve_map(
        configured: &BTreeMap<String, CredentialValue>,
        secrets: &dyn SecretStore,
    ) -> Result<Self, McpError> {
        let mut values = BTreeMap::new();
        let mut authority = Vec::new();
        for (name, value) in configured {
            match value {
                CredentialValue::Literal { literal } => {
                    values.insert(name.clone(), literal.clone());
                }
                CredentialValue::Credential { credential } => {
                    let (material, generation) = resolve_secret(credential, secrets)?;
                    values.insert(name.clone(), material);
                    authority.push((credential.clone(), generation));
                }
            }
        }
        authority.sort();
        authority.dedup();
        Ok(Self { values, authority })
    }

    fn zeroize(&mut self) {
        for value in self.values.values_mut() {
            value.zeroize();
        }
    }
}

fn resolve_secret(credential: &str, secrets: &dyn SecretStore) -> Result<(String, u64), McpError> {
    match secrets
        .resolve(credential)
        .map_err(|_| McpError::Transport("MCP credential store unavailable".to_owned()))?
    {
        SecretResolution::Active(record @ SecretRecord::Active { .. }) => match &record {
            SecretRecord::Active {
                generation,
                material,
            } => Ok((material.clone(), *generation)),
            SecretRecord::Revoked { .. } => unreachable!(),
        },
        SecretResolution::Revoked { .. }
        | SecretResolution::NotFound
        | SecretResolution::Active(SecretRecord::Revoked { .. }) => Err(McpError::Transport(
            format!("MCP credential {credential} is unavailable"),
        )),
    }
}

fn authorization_identity(parts: &[(String, u64)]) -> String {
    if parts.is_empty() {
        return "anonymous".to_owned();
    }
    let mut hasher = Sha256::new();
    hasher.update(b"tekes-mcp-authorization-v1\0");
    for (credential, generation) in parts {
        hasher.update(credential.as_bytes());
        hasher.update([0]);
        hasher.update(generation.to_be_bytes());
        hasher.update([0]);
    }
    format!("sha256-{:x}", hasher.finalize())
}

const fn scope_name(scope: mcp::McpScope) -> &'static str {
    match scope {
        mcp::McpScope::User => "user",
        mcp::McpScope::Project => "project",
        mcp::McpScope::Plugin => "plugin",
    }
}

const fn protocol_name(mode: ProtocolMode) -> &'static str {
    match mode {
        ProtocolMode::Auto => "auto",
        ProtocolMode::Legacy => "legacy",
        ProtocolMode::Modern => "modern",
    }
}

/// The read store and, when the Kernel holds one, the write path for the
/// OAuth grants it minted; both travel together into peer preparation.
#[derive(Clone)]
pub struct SecretAccess {
    pub store: Arc<dyn SecretStore>,
    pub mutation: Option<Arc<dyn SecretMutationAuthority>>,
}

impl SecretAccess {
    #[must_use]
    pub fn read_only(store: Arc<dyn SecretStore>) -> Self {
        Self {
            store,
            mutation: None,
        }
    }
}

#[derive(Clone)]
pub struct McpDynamicSupervisorAuthority {
    broker: McpBrokerHandle,
    routes: BTreeMap<String, McpRoute>,
    secrets: SecretAccess,
    registry: McpRegistryStore,
    plugin_resolver: Option<Arc<dyn PluginMcpResolver>>,
    cancellation: Arc<Mutex<McpCancellationToken>>,
}

#[derive(Clone)]
pub struct McpRoute {
    pub key: McpPoolKey,
    pub remote_name: String,
    pub credential_floors: Vec<(String, u64)>,
    pub server: McpServerConfig,
    pub plugin_generation: Option<String>,
    pub contract: McpRouteContract,
}

#[derive(Clone, Debug, PartialEq)]
pub struct McpRouteContract {
    pub protocol_version: String,
    pub server_identity: McpImplementation,
    pub catalog_generation: u64,
    pub effect_tool: McpTool,
    pub reconcile_tool: Option<McpTool>,
    pub digest: String,
}

impl McpRouteContract {
    fn capture(
        peer: &dyn McpPeer,
        effect_tool: McpTool,
        reconcile_tool: Option<McpTool>,
    ) -> Result<Self, McpError> {
        let protocol_version = peer.protocol_version().to_owned();
        let server_identity = peer.server_identity().cloned().ok_or_else(|| {
            McpError::Protocol("MCP handshake omitted server identity".to_owned())
        })?;
        let catalog_generation = peer.catalog_generation();
        let digest = route_contract_digest(
            &protocol_version,
            &server_identity,
            catalog_generation,
            &effect_tool,
            reconcile_tool.as_ref(),
        )?;
        Ok(Self {
            protocol_version,
            server_identity,
            catalog_generation,
            effect_tool,
            reconcile_tool,
            digest,
        })
    }
}

fn route_contract_digest(
    protocol_version: &str,
    server_identity: &McpImplementation,
    catalog_generation: u64,
    effect_tool: &McpTool,
    reconcile_tool: Option<&McpTool>,
) -> Result<String, McpError> {
    let preimage = json!({
        "domain":"io.tekes/mcp-route-contract/v1",
        "protocolVersion":protocol_version,
        "serverIdentity":server_identity,
        "catalogGeneration":catalog_generation,
        "effectTool":effect_tool,
        "reconcileTool":reconcile_tool,
    });
    Ok(format!(
        "sha256-{:x}",
        Sha256::digest(
            serde_json_canonicalizer::to_vec(&preimage)
                .map_err(|error| McpError::Protocol(error.to_string()))?
        )
    ))
}

#[derive(Debug)]
enum RoutePeerError {
    Unavailable(String),
    ContractChanged(String),
}

fn route_preparation_error(error: McpError) -> RoutePeerError {
    match error {
        McpError::Transport(message) | McpError::Timeout(message) => {
            RoutePeerError::Unavailable(message)
        }
        other => RoutePeerError::ContractChanged(other.to_string()),
    }
}

fn validate_reconnected_contract(
    expected: &McpRouteContract,
    reconnected: &McpRouteContract,
) -> Result<(), RoutePeerError> {
    if reconnected.protocol_version != expected.protocol_version {
        return Err(RoutePeerError::ContractChanged(
            "negotiated protocol version changed".to_owned(),
        ));
    }
    if reconnected.server_identity != expected.server_identity {
        return Err(RoutePeerError::ContractChanged(
            "negotiated server identity changed".to_owned(),
        ));
    }
    if reconnected.catalog_generation != expected.catalog_generation {
        return Err(RoutePeerError::ContractChanged(
            "catalog generation changed during reconnect".to_owned(),
        ));
    }
    if reconnected.effect_tool != expected.effect_tool {
        return Err(RoutePeerError::ContractChanged(
            "effect tool canonical contract changed".to_owned(),
        ));
    }
    if reconnected.reconcile_tool != expected.reconcile_tool {
        return Err(RoutePeerError::ContractChanged(
            "reconciliation tool canonical contract changed".to_owned(),
        ));
    }
    if reconnected.digest != expected.digest {
        return Err(RoutePeerError::ContractChanged(
            "MCP route contract digest changed".to_owned(),
        ));
    }
    Ok(())
}

impl McpDynamicSupervisorAuthority {
    #[must_use]
    pub fn new(
        broker: McpBrokerHandle,
        routes: BTreeMap<String, McpRoute>,
        secrets: SecretAccess,
        registry: McpRegistryStore,
        plugin_resolver: Option<Arc<dyn PluginMcpResolver>>,
    ) -> Self {
        Self {
            broker,
            routes,
            secrets,
            registry,
            plugin_resolver,
            cancellation: Arc::new(Mutex::new(McpCancellationToken::default())),
        }
    }
}

impl Drop for McpDynamicSupervisorAuthority {
    fn drop(&mut self) {
        let keys = self
            .routes
            .values()
            .map(|route| route.key.clone())
            .collect::<std::collections::BTreeSet<_>>();
        for key in keys {
            let _ = self.broker.release(key);
        }
    }
}

impl McpDynamicSupervisorAuthority {
    /// Peer generation, catalog contract and credential floors must hold before
    /// any remote operation on a route.
    fn prepare_route(
        &self,
        tool: &DynamicTool,
    ) -> Result<(&McpRoute, McpCancellationToken), SupervisorOperationError> {
        let route = self.routes.get(&tool.name).ok_or_else(|| {
            SupervisorOperationError::Unsupported(format!("MCP route {} is not bound", tool.name))
        })?;
        let cancellation = self.current_cancellation();
        if let Err(error) = self.ensure_route_peer(route, tool, cancellation.clone()) {
            let _ = self.broker.remove(route.key.clone());
            return Err(match error {
                RoutePeerError::Unavailable(reason) => SupervisorOperationError::Unavailable(
                    format!("MCP authority unavailable; peer generation closed: {reason}"),
                ),
                RoutePeerError::ContractChanged(reason) if tool.external_effect.is_some() => {
                    SupervisorOperationError::EffectConflicted(format!(
                        "MCP authority changed; peer generation closed: {reason}"
                    ))
                }
                RoutePeerError::ContractChanged(reason) => SupervisorOperationError::Conflict(
                    format!("MCP authority changed; peer generation closed: {reason}"),
                ),
            });
        }
        for (credential, expected_generation) in &route.credential_floors {
            let current = self.secrets.store.resolve(credential).map_err(|_| {
                SupervisorOperationError::Unavailable("MCP credential store unavailable".to_owned())
            })?;
            let valid = matches!(
                current,
                SecretResolution::Active(ref record)
                    if record.generation() == *expected_generation
            );
            if !valid {
                let _ = self.broker.remove(route.key.clone());
                return Err(SupervisorOperationError::Unavailable(format!(
                    "MCP credential {credential} changed; peer generation closed"
                )));
            }
        }
        Ok((route, cancellation))
    }

    fn call_route(
        &self,
        tool: &DynamicTool,
        request: &ToolControl,
    ) -> Result<Value, SupervisorOperationError> {
        let (route, cancellation) = self.prepare_route(tool)?;
        // A tool whose server declares execution.taskSupport = required only
        // runs as a task; the augmented call binds the remote continuation.
        let result = if route.contract.effect_tool.requires_task() {
            self.broker.call_tool_augmented(
                route.key.clone(),
                route.remote_name.clone(),
                request.arguments.clone(),
                tool.external_effect.as_ref().map(|_| McpToolCallContext {
                    idempotency_key: request.request_id.clone(),
                }),
                REQUIRED_TASK_TTL_MS,
                cancellation,
            )
        } else if tool.external_effect.is_some() {
            self.broker.call_tool_with_context(
                route.key.clone(),
                route.remote_name.clone(),
                request.arguments.clone(),
                McpToolCallContext {
                    idempotency_key: request.request_id.clone(),
                },
                cancellation,
            )
        } else {
            self.broker.call_tool(
                route.key.clone(),
                route.remote_name.clone(),
                request.arguments.clone(),
                cancellation,
            )
        };
        let result = result.map_err(map_mcp_error)?;
        serde_json::to_value(&result)
            .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))
    }
}

/// Time-to-live requested for a task-augmented call whose tool requires tasks.
const REQUIRED_TASK_TTL_MS: u64 = 600_000;

/// Domain-separated host continuation identity for one bound remote task.
fn continuation_identity(request_id: &str, task_id: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(b"tekes-mcp-continuation-v1\0");
    digest.update(request_id.as_bytes());
    digest.update([0]);
    digest.update(task_id.as_bytes());
    format!("{:x}", digest.finalize())
}

fn map_journal_error(error: ContinuationJournalError) -> SupervisorOperationError {
    match error {
        ContinuationJournalError::Conflict(message) => SupervisorOperationError::Conflict(message),
        other => SupervisorOperationError::Unavailable(other.to_string()),
    }
}

fn terminal_tool_value(raw: Value) -> Result<IJsonValue, SupervisorOperationError> {
    match raw.get("isError") {
        Some(Value::Bool(true)) => Err(SupervisorOperationError::ToolFailed(raw.to_string())),
        Some(Value::Bool(false)) | None => {
            json_to_ijson(&raw).map_err(SupervisorOperationError::Protocol)
        }
        Some(_) => Err(SupervisorOperationError::Protocol(
            "MCP tool result isError must be boolean".into(),
        )),
    }
}

impl DynamicSupervisorAuthority for McpDynamicSupervisorAuthority {
    fn supports(&self, tool: &DynamicTool) -> bool {
        tool.source.kind == DynamicToolSourceKind::Mcp && self.routes.contains_key(&tool.name)
    }

    fn execute(
        &self,
        tool: &DynamicTool,
        request: &ToolControl,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        let raw = self.call_route(tool, request)?;
        terminal_tool_value(raw)
    }

    /// `tools/call` whose result is a task (the MCP tasks extension's
    /// `CreateTaskResult`) binds a durable continuation instead of returning a
    /// value: the worker will drive it through `continue_task`. Any other
    /// result keeps the synchronous mapping.
    fn execute_outcome(
        &self,
        tool: &DynamicTool,
        request: &ToolControl,
        continuation_root: &Path,
    ) -> Result<DynamicExecutionOutcome, SupervisorOperationError> {
        let raw = self.call_route(tool, request)?;
        let Some(task) = raw.get("task").filter(|task| task.is_object()) else {
            return terminal_tool_value(raw).map(DynamicExecutionOutcome::Value);
        };
        let task_id = task["taskId"]
            .as_str()
            .filter(|id| !id.is_empty())
            .ok_or_else(|| {
                SupervisorOperationError::Protocol("MCP task result lacks taskId".to_owned())
            })?;
        let task_value = json_to_ijson(task).map_err(SupervisorOperationError::Protocol)?;
        let validated = mcp::McpTask::validate_result(&task_value, task_id)
            .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))?;
        match validated.status.as_str() {
            "working" | "input_required" => {}
            "completed" => {
                return terminal_tool_value(task["result"].clone())
                    .map(DynamicExecutionOutcome::Value);
            }
            other => {
                return Err(SupervisorOperationError::ToolFailed(format!(
                    "MCP task {task_id} was created {other}: {}",
                    task.get("error")
                        .map(ToString::to_string)
                        .unwrap_or_default()
                )));
            }
        }
        let route = self.routes.get(&tool.name).ok_or_else(|| {
            SupervisorOperationError::Unsupported(format!("MCP route {} is not bound", tool.name))
        })?;
        let continuation_id = continuation_identity(&request.request_id, task_id);
        let journal = ContinuationJournal::open(continuation_root).map_err(map_journal_error)?;
        journal
            .bind(&ContinuationBinding {
                original: request.clone(),
                continuation_id: continuation_id.clone(),
                authority: task_authority(&route.key).map_err(map_journal_error)?,
                initial_state: task_value.clone(),
            })
            .map_err(map_journal_error)?;
        Ok(DynamicExecutionOutcome::Pending {
            continuation_id,
            state: task_value,
        })
    }

    fn continue_task(
        &self,
        tool: &DynamicTool,
        request: &ToolContinuationRequest,
        continuation_root: &Path,
    ) -> Result<ToolContinuationResponse, SupervisorOperationError> {
        let (route, cancellation) = self.prepare_route(tool)?;
        let journal = ContinuationJournal::open(continuation_root).map_err(map_journal_error)?;
        resolve_task(&journal, &self.broker, &route.key, request, cancellation)
            .map_err(map_journal_error)
    }

    fn reconcile(&self, tool: &DynamicTool, request: &ToolControl) -> ExternalEffectResolution {
        let Some(binding) = &tool.external_effect else {
            return ExternalEffectResolution::Conflicted {
                message: format!("MCP tool {} lost its external-effect contract", tool.name),
            };
        };
        let Some(route) = self.routes.get(&tool.name) else {
            return ExternalEffectResolution::Conflicted {
                message: format!("MCP route {} is not bound", tool.name),
            };
        };
        let cancellation = self.current_cancellation();
        if let Err(error) = self.ensure_route_peer(route, tool, cancellation.clone()) {
            return match error {
                RoutePeerError::Unavailable(reason) => ExternalEffectResolution::Unknown {
                    message: format!("MCP reconciliation authority is unavailable: {reason}"),
                },
                RoutePeerError::ContractChanged(reason) => ExternalEffectResolution::Conflicted {
                    message: format!("MCP reconciliation authority changed: {reason}"),
                },
            };
        }
        let arguments = match IJsonValue::parse(
            &serde_json::to_vec(&json!({"idempotencyKey":request.request_id}))
                .expect("idempotency reconciliation request is JSON"),
        ) {
            Ok(arguments) => arguments,
            Err(error) => {
                return ExternalEffectResolution::Conflicted {
                    message: format!("MCP reconciliation request is invalid: {error}"),
                };
            }
        };
        match self.broker.call_tool(
            route.key.clone(),
            binding.reconcile_tool.clone(),
            arguments,
            cancellation,
        ) {
            Ok(value) => parse_external_effect_resolution(request, value),
            Err(error) => ExternalEffectResolution::Unknown {
                message: format!("MCP reconciliation query failed: {error}"),
            },
        }
    }

    fn cancel_inflight(&self) {
        let cancelled = {
            let mut current = self
                .cancellation
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            std::mem::take(&mut *current)
        };
        cancelled.cancel();
    }
}

impl McpDynamicSupervisorAuthority {
    fn current_cancellation(&self) -> McpCancellationToken {
        self.cancellation
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }

    fn ensure_route_peer(
        &self,
        route: &McpRoute,
        expected_tool: &DynamicTool,
        cancellation: McpCancellationToken,
    ) -> Result<(), RoutePeerError> {
        if cancellation.is_cancelled() {
            return Err(RoutePeerError::Unavailable(
                "MCP operation was cancelled".to_owned(),
            ));
        }
        self.validate_route_generation(route)
            .map_err(RoutePeerError::ContractChanged)?;
        if let Some(generation) = self
            .broker
            .catalog_generation(route.key.clone())
            .map_err(|error| RoutePeerError::Unavailable(error.to_string()))?
        {
            if generation == route.contract.catalog_generation {
                return Ok(());
            }
            let _ = self.broker.remove(route.key.clone());
            return Err(RoutePeerError::ContractChanged(
                "MCP catalog generation changed".to_owned(),
            ));
        }
        let plugin = if let Some(reference) = &route.server.plugin_component {
            Some(
                self.plugin_resolver
                    .as_ref()
                    .ok_or_else(|| {
                        RoutePeerError::Unavailable("plugin MCP resolver is unavailable".to_owned())
                    })?
                    .resolve(reference)
                    .map_err(RoutePeerError::Unavailable)?
                    .ok_or_else(|| {
                        RoutePeerError::ContractChanged(
                            "plugin MCP component is unavailable".to_owned(),
                        )
                    })?,
            )
        } else {
            None
        };
        let prepared = with_preparation_runtime(|runtime| {
            prepare_server_cancellable(
                &route.server,
                plugin.as_ref(),
                self.secrets.clone(),
                runtime,
                cancellation,
            )
        })
        .map_err(route_preparation_error)?;
        if prepared.key != route.key || prepared.credential_floors != route.credential_floors {
            return Err(RoutePeerError::ContractChanged(
                "reconnected MCP pool or credential identity changed".to_owned(),
            ));
        }
        let effect_tool = prepared
            .tools
            .iter()
            .find(|tool| tool.name == route.remote_name)
            .ok_or_else(|| {
                RoutePeerError::ContractChanged(
                    "reconnected MCP catalog lost the effect tool".to_owned(),
                )
            })?
            .clone();
        let reconcile_tool = route
            .contract
            .reconcile_tool
            .as_ref()
            .map(|expected| {
                prepared
                    .tools
                    .iter()
                    .find(|tool| tool.name == expected.name)
                    .cloned()
                    .ok_or_else(|| {
                        RoutePeerError::ContractChanged(
                            "reconnected MCP catalog lost the reconciliation tool".to_owned(),
                        )
                    })
            })
            .transpose()?;
        let reconnected_contract =
            McpRouteContract::capture(prepared.peer.as_ref(), effect_tool, reconcile_tool)
                .map_err(route_preparation_error)?;
        validate_reconnected_contract(&route.contract, &reconnected_contract)?;
        let projected = project_catalog(
            &route.server.reference.name,
            &prepared.tools,
            route.server.always_on,
        )
        .map_err(|error| RoutePeerError::ContractChanged(error.to_string()))?;
        let reconnected_tool = projected
            .tools
            .iter()
            .find(|tool| tool.name == expected_tool.name)
            .ok_or_else(|| {
                RoutePeerError::ContractChanged(
                    "reconnected MCP catalog lost the effectful tool".to_owned(),
                )
            })?;
        if reconnected_tool != expected_tool {
            return Err(RoutePeerError::ContractChanged(
                "reconnected MCP catalog changed the projected effect contract".to_owned(),
            ));
        }
        self.broker
            .register(prepared.key, prepared.peer, route.server.always_on)
            .map_err(|error| RoutePeerError::Unavailable(error.to_string()))
    }

    fn validate_route_generation(&self, route: &McpRoute) -> Result<(), String> {
        if let Some(reference) = &route.server.plugin_component {
            let resolver = self
                .plugin_resolver
                .as_ref()
                .ok_or_else(|| "plugin MCP resolver is unavailable".to_owned())?;
            let resolved = resolver.resolve(reference)?.ok_or_else(|| {
                "plugin component is disabled, uninstalled, or under-granted".to_owned()
            })?;
            if resolved.component_type != ComponentKind::McpServer
                || Some(&resolved.plugin_generation) != route.plugin_generation.as_ref()
            {
                return Err("plugin package or grant generation changed".to_owned());
            }
        } else {
            let current = self
                .registry
                .get(&route.server.reference)
                .map_err(|error| error.to_string())?
                .ok_or_else(|| "server was removed".to_owned())?;
            if current != route.server || !current.enabled {
                return Err("server configuration generation changed".to_owned());
            }
            if current.reference.scope == McpScope::Project && !current.project_trusted {
                return Err("project server trust was revoked".to_owned());
            }
        }
        for (field, credential_id) in credential_fields(&route.server) {
            let state = self
                .registry
                .credential_reference_state(&route.server.reference, &field, &credential_id)
                .map_err(|error| error.to_string())?;
            if state != Some(McpCredentialReferenceState::Bound) {
                return Err(format!("credential reference {field} is not bound"));
            }
        }
        for (credential_id, expected_generation) in &route.credential_floors {
            let current = self
                .secrets
                .store
                .resolve(credential_id)
                .map_err(|_| "credential store is unavailable".to_owned())?;
            if !matches!(
                current,
                SecretResolution::Active(ref record)
                    if record.generation() == *expected_generation
            ) {
                return Err(format!(
                    "credential {credential_id} generation changed or was revoked"
                ));
            }
        }
        Ok(())
    }
}

fn map_mcp_error(error: McpError) -> SupervisorOperationError {
    match error {
        McpError::Timeout(message) => SupervisorOperationError::Timeout(message),
        McpError::Unsupported(message) => SupervisorOperationError::Unsupported(message),
        McpError::NoMutualProtocol(versions) => {
            SupervisorOperationError::Unsupported(format!("no mutual MCP protocol: {versions:?}"))
        }
        McpError::Protocol(message) | McpError::Conflict(message) => {
            SupervisorOperationError::Protocol(message)
        }
        McpError::Remote { code, message, .. } => {
            SupervisorOperationError::Unavailable(format!("MCP remote error {code}: {message}"))
        }
        McpError::UnknownEffect => {
            SupervisorOperationError::AmbiguousEffect("MCP tool effect is unknown".to_owned())
        }
        McpError::CatalogLimit => {
            SupervisorOperationError::Limit("MCP catalog bound exceeded".to_owned())
        }
        McpError::Cancelled => {
            SupervisorOperationError::Unavailable("MCP call cancelled".to_owned())
        }
        McpError::Transport(message) => SupervisorOperationError::Unavailable(message),
    }
}

fn parse_external_effect_resolution(
    request: &ToolControl,
    value: IJsonValue,
) -> ExternalEffectResolution {
    let result: Value = match value
        .canonical_bytes()
        .map_err(|error| error.to_string())
        .and_then(|bytes| serde_json::from_slice(&bytes).map_err(|error| error.to_string()))
    {
        Ok(raw) => raw,
        Err(error) => {
            return ExternalEffectResolution::Conflicted {
                message: format!("MCP reconciliation result is invalid: {error}"),
            };
        }
    };
    // A normal MCP tools/call returns a CallToolResult envelope. Reconcile
    // functions may expose their JSON via structuredContent or the first text
    // content block; older direct peers return the object itself.
    let raw = if result.get("status").is_some() {
        result
    } else if result.get("isError") == Some(&Value::Bool(true)) {
        return ExternalEffectResolution::Unknown {
            message: "MCP reconciliation tool returned an error".to_owned(),
        };
    } else if let Some(structured) = result.get("structuredContent") {
        structured.clone()
    } else if let Some(text) = result
        .get("content")
        .and_then(Value::as_array)
        .and_then(|items| items.first())
        .and_then(|item| item.get("text"))
        .and_then(Value::as_str)
    {
        match serde_json::from_str::<Value>(text) {
            Ok(value) => value,
            Err(_) => {
                return ExternalEffectResolution::Conflicted {
                    message: "MCP reconciliation text is not JSON".to_owned(),
                };
            }
        }
    } else {
        result
    };
    let Some(status) = raw.get("status").and_then(Value::as_str) else {
        return ExternalEffectResolution::Conflicted {
            message: "MCP reconciliation result has no closed status".to_owned(),
        };
    };
    match status {
        "confirmed" => {
            if raw.as_object().is_none_or(|object| object.len() != 2) {
                return ExternalEffectResolution::Conflicted {
                    message: "confirmed reconciliation result has unknown fields".to_owned(),
                };
            }
            let Some(value) = raw.get("value") else {
                return ExternalEffectResolution::Conflicted {
                    message: "confirmed reconciliation result has no value".to_owned(),
                };
            };
            match json_to_ijson(value) {
                Ok(value) => {
                    ExternalEffectResolution::Confirmed(worker_control::ToolControlResult::success(
                        request.request_id.clone(),
                        request.call_id.clone(),
                        value,
                    ))
                }
                Err(error) => ExternalEffectResolution::Conflicted {
                    message: format!("confirmed reconciliation value is invalid: {error}"),
                },
            }
        }
        "not_found" if raw.as_object().is_some_and(|object| object.len() == 1) => {
            ExternalEffectResolution::NotFound
        }
        "unknown" | "conflicted" => {
            let reason = raw
                .get("reason")
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty());
            if raw.as_object().is_none_or(|object| object.len() != 2) || reason.is_none() {
                return ExternalEffectResolution::Conflicted {
                    message: format!(
                        "{status} reconciliation result must contain only status and nonempty reason"
                    ),
                };
            }
            let reason = reason.expect("checked nonempty reason").to_owned();
            if status == "unknown" {
                ExternalEffectResolution::Unknown { message: reason }
            } else {
                ExternalEffectResolution::Conflicted { message: reason }
            }
        }
        other => ExternalEffectResolution::Conflicted {
            message: format!("MCP reconciliation returned unsupported status {other}"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mcp::{McpCapabilities, McpImplementation};

    #[test]
    fn reconcile_accepts_standard_mcp_text_result_envelope() {
        let request = ToolControl::new(
            "018f0000-0000-7000-8000-000000000001",
            "018f0000-0000-7000-8000-000000000002",
            1,
            "call-1",
            "mcp__pdf2cad__create_run",
            IJsonValue::parse_str("{}").unwrap(),
        )
        .unwrap();
        let result = IJsonValue::parse_str(
            r#"{"content":[{"type":"text","text":"{\"status\":\"not_found\"}"}],"isError":false}"#,
        )
        .unwrap();
        assert!(matches!(
            parse_external_effect_resolution(&request, result),
            ExternalEffectResolution::NotFound
        ));
    }

    /// mcp-runtime §oauth: a Kernel-minted grant behind the `oauth` field
    /// is exchanged (identity stable across rotation, bearer = access token,
    /// floor = the rotated generation); a platform-installed token is injected
    /// as-is; the token endpoint's 400 is the exact legacy failure and closes
    /// the record so the next request fails without another exchange.
    #[test]
    fn oauth_binding_exchanges_kernel_grants_and_injects_platform_tokens() {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let token_endpoint = format!("http://{}/token", listener.local_addr().unwrap());
        let server = std::thread::spawn(move || {
            for (status, body) in [
                (
                    200,
                    r#"{"access_token":"access-1","token_type":"bearer","expires_in":3600,"refresh_token":"refresh-2"}"#,
                ),
                (400, r#"{"error":"invalid_grant"}"#),
            ] {
                let (mut stream, _) = listener.accept().unwrap();
                let mut bytes = [0_u8; 8192];
                let mut request = Vec::new();
                loop {
                    let count = stream.read(&mut bytes).unwrap();
                    request.extend_from_slice(&bytes[..count]);
                    if let Some(end) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                        let headers =
                            String::from_utf8_lossy(&request[..end + 4]).to_ascii_lowercase();
                        let length = headers
                            .lines()
                            .find_map(|l| l.strip_prefix("content-length: "))
                            .and_then(|v| v.trim().parse::<usize>().ok())
                            .unwrap_or(0);
                        if request.len() >= end + 4 + length {
                            break;
                        }
                    }
                    if count == 0 {
                        break;
                    }
                }
                write!(stream, "HTTP/1.1 {status} X\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}", body.len()).unwrap();
            }
        });
        let store = Arc::new(MemorySecretStore::new());
        provider::mint_oauth_grant(
            store.as_ref(),
            store.as_ref(),
            "oauth-cf",
            &provider::OAuthGrant::new(token_endpoint, "client-1", "refresh-1"),
        )
        .unwrap();
        store
            .publish(
                "platform-token",
                SecretRecord::Active {
                    generation: 4,
                    material: "installed-access-token".to_owned(),
                },
            )
            .unwrap();
        let mut headers = BTreeMap::new();
        headers.insert(
            "x-tenant".to_owned(),
            CredentialValue::Literal {
                literal: "acme".to_owned(),
            },
        );
        let exchange = provider::OAuthTokenExchange::new("oauth-cf", store.clone(), store.clone());
        let first =
            http_request_authorization(&headers, Some("oauth-cf"), store.as_ref(), Some(&exchange))
                .unwrap();
        assert_eq!(first.identity, "oauth:oauth-cf:client-1");
        assert_eq!(first.bearer.as_deref(), Some("access-1"));
        assert_eq!(
            first.headers.get("x-tenant").map(String::as_str),
            Some("acme")
        );
        assert_eq!(
            exchange.cached_generation(),
            Some(2),
            "the rotated refresh token is generation 2"
        );
        let again =
            http_request_authorization(&headers, Some("oauth-cf"), store.as_ref(), Some(&exchange))
                .unwrap();
        assert_eq!(
            again.bearer.as_deref(),
            Some("access-1"),
            "cached: no second exchange"
        );
        let platform =
            http_request_authorization(&headers, Some("platform-token"), store.as_ref(), None)
                .unwrap();
        assert_eq!(platform.bearer.as_deref(), Some("installed-access-token"));
        assert_ne!(platform.identity, first.identity);
        // A fresh connector after remote revocation: exact legacy text, record closed, no retry.
        let restarted = provider::OAuthTokenExchange::new("oauth-cf", store.clone(), store.clone());
        let error = http_request_authorization(
            &headers,
            Some("oauth-cf"),
            store.as_ref(),
            Some(&restarted),
        )
        .map(|_| ())
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "MCP transport failed: token endpoint returned HTTP 400"
        );
        assert!(matches!(
            store.resolve("oauth-cf").unwrap(),
            SecretResolution::Revoked { generation: 3 }
        ));
        let closed = http_request_authorization(
            &headers,
            Some("oauth-cf"),
            store.as_ref(),
            Some(&restarted),
        )
        .map(|_| ())
        .unwrap_err();
        assert_eq!(
            closed.to_string(),
            "MCP transport failed: token endpoint returned HTTP 400",
            "the retry on the same connector keeps the exact text"
        );
        let later = provider::OAuthTokenExchange::new("oauth-cf", store.clone(), store.clone());
        let later =
            http_request_authorization(&headers, Some("oauth-cf"), store.as_ref(), Some(&later))
                .map(|_| ())
                .unwrap_err();
        assert!(later.to_string().contains("is unavailable"), "{later}");
        server.join().unwrap();
    }
    use provider::MemorySecretStore;
    use std::collections::VecDeque;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use worker_control::continuation::{ContinuationOperation, ToolContinuationOutcome};

    struct CancellationRotationPeer {
        identity: McpImplementation,
        dispatched: Mutex<Option<std::sync::mpsc::Sender<()>>>,
        effect_keys: Arc<Mutex<Vec<String>>>,
        reconcile_keys: Arc<Mutex<Vec<String>>>,
        cancellation_states: Arc<Mutex<Vec<bool>>>,
        reconcile_results: Arc<Mutex<VecDeque<Value>>>,
        catalog_generation: Arc<AtomicUsize>,
    }

    impl McpPeer for CancellationRotationPeer {
        fn server_name(&self) -> &str {
            &self.identity.name
        }

        fn protocol_version(&self) -> &str {
            "2026-07-28"
        }

        fn server_identity(&self) -> Option<&McpImplementation> {
            Some(&self.identity)
        }

        fn capabilities(&self) -> &McpCapabilities {
            static CAPABILITIES: McpCapabilities = McpCapabilities {
                tools: true,
                prompts: false,
                resources: false,
                tasks: false,
                subscriptions: false,
                extensions: BTreeMap::new(),
                tools_list_changed: false,
                prompts_list_changed: false,
                resources_list_changed: false,
                resources_subscribe: false,
            };
            &CAPABILITIES
        }

        fn catalog_generation(&self) -> u64 {
            self.catalog_generation.load(Ordering::Acquire) as u64
        }

        fn list_tools(&mut self) -> mcp::McpPeerFuture<'_, Vec<mcp::McpTool>> {
            Box::pin(async { Ok(Vec::new()) })
        }

        fn list_prompts(&mut self) -> mcp::McpPeerFuture<'_, Vec<mcp::McpPrompt>> {
            Box::pin(async { Ok(Vec::new()) })
        }

        fn list_resources(&mut self) -> mcp::McpPeerFuture<'_, Vec<McpResource>> {
            Box::pin(async { Ok(Vec::new()) })
        }

        fn call_tool(
            &mut self,
            name: &str,
            arguments: IJsonValue,
            cancellation: McpCancellationToken,
        ) -> mcp::McpPeerFuture<'_, IJsonValue> {
            self.cancellation_states
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(cancellation.is_cancelled());
            if name == "orders/get_by_idempotency_key" {
                let raw: Value = serde_json::from_slice(
                    &arguments.canonical_bytes().expect("reconcile arguments"),
                )
                .expect("reconcile JSON");
                self.reconcile_keys
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .push(
                        raw["idempotencyKey"]
                            .as_str()
                            .expect("idempotency key")
                            .to_owned(),
                    );
                let result = self
                    .reconcile_results
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .pop_front()
                    .expect("queued reconciliation result");
                return Box::pin(async move { json_to_ijson(&result).map_err(McpError::Protocol) });
            }
            Box::pin(
                async move { json_to_ijson(&json!({"ordinary":"ok"})).map_err(McpError::Protocol) },
            )
        }

        fn call_tool_with_context(
            &mut self,
            _name: &str,
            _arguments: IJsonValue,
            context: McpToolCallContext,
            cancellation: McpCancellationToken,
        ) -> mcp::McpPeerFuture<'_, IJsonValue> {
            self.effect_keys
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(context.idempotency_key);
            if let Some(dispatched) = self
                .dispatched
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .take()
            {
                let _ = dispatched.send(());
            }
            Box::pin(async move {
                while !cancellation.is_cancelled() {
                    tokio::task::yield_now().await;
                }
                Err(McpError::Cancelled)
            })
        }

        fn get_prompt(
            &mut self,
            _name: &str,
            _arguments: IJsonValue,
        ) -> mcp::McpPeerFuture<'_, IJsonValue> {
            Box::pin(async { Err(McpError::Unsupported("unused".to_owned())) })
        }

        fn read_resource(&mut self, _uri: &str) -> mcp::McpPeerFuture<'_, IJsonValue> {
            Box::pin(async { Err(McpError::Unsupported("unused".to_owned())) })
        }

        fn task_operation(
            &mut self,
            _method: &str,
            _params: IJsonValue,
        ) -> mcp::McpPeerFuture<'_, IJsonValue> {
            Box::pin(async { Err(McpError::Unsupported("unused".to_owned())) })
        }

        fn close(&mut self) -> mcp::McpPeerFuture<'_, ()> {
            Box::pin(async { Ok(()) })
        }
    }

    struct ResourcePeer {
        capabilities: McpCapabilities,
        identity: McpImplementation,
        resources: Vec<McpResource>,
        list_fails: bool,
        closes: Arc<AtomicUsize>,
    }

    impl McpPeer for ResourcePeer {
        fn server_name(&self) -> &str {
            &self.identity.name
        }

        fn protocol_version(&self) -> &str {
            "2025-03-26"
        }

        fn server_identity(&self) -> Option<&McpImplementation> {
            Some(&self.identity)
        }

        fn capabilities(&self) -> &McpCapabilities {
            &self.capabilities
        }

        fn list_tools(&mut self) -> mcp::McpPeerFuture<'_, Vec<mcp::McpTool>> {
            Box::pin(async { Ok(Vec::new()) })
        }

        fn list_prompts(&mut self) -> mcp::McpPeerFuture<'_, Vec<mcp::McpPrompt>> {
            Box::pin(async { Ok(Vec::new()) })
        }

        fn list_resources(&mut self) -> mcp::McpPeerFuture<'_, Vec<McpResource>> {
            let result = if self.list_fails {
                Err(McpError::Transport("injected list failure".to_owned()))
            } else {
                Ok(self.resources.clone())
            };
            Box::pin(async move { result })
        }

        fn call_tool(
            &mut self,
            _name: &str,
            _arguments: IJsonValue,
            _cancellation: McpCancellationToken,
        ) -> mcp::McpPeerFuture<'_, IJsonValue> {
            Box::pin(async { Err(McpError::Unsupported("unused".to_owned())) })
        }

        fn get_prompt(
            &mut self,
            _name: &str,
            _arguments: IJsonValue,
        ) -> mcp::McpPeerFuture<'_, IJsonValue> {
            Box::pin(async { Err(McpError::Unsupported("unused".to_owned())) })
        }

        fn read_resource(&mut self, uri: &str) -> mcp::McpPeerFuture<'_, IJsonValue> {
            let uri = uri.to_owned();
            Box::pin(async move {
                json_to_ijson(&json!({"contents":[{"text":"ok","uri":uri}]}))
                    .map_err(McpError::Protocol)
            })
        }

        fn task_operation(
            &mut self,
            _method: &str,
            _params: IJsonValue,
        ) -> mcp::McpPeerFuture<'_, IJsonValue> {
            Box::pin(async { Err(McpError::Unsupported("unused".to_owned())) })
        }

        fn close(&mut self) -> mcp::McpPeerFuture<'_, ()> {
            self.closes.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }
    }

    #[test]
    fn preparation_cancellation_reaps_real_stdio_during_handshake_and_catalog() {
        use std::time::Duration;
        for stage in ["handshake", "catalog"] {
            let root = tempfile::tempdir().unwrap();
            let marker = root.path().join("reached");
            let script = root.path().join("silent.py");
            std::fs::write(&script, r#"
import json, os, sys, time
stage, marker, version = sys.argv[1:]
for line in sys.stdin:
    request = json.loads(line)
    if stage == 'handshake' or request['method'] == 'tools/list':
        with open(marker, 'w') as f: f.write(str(os.getpid()))
        time.sleep(30)
    elif request['method'] == 'initialize':
        print(json.dumps({'jsonrpc':'2.0','id':request['id'],'result':{'protocolVersion':version,'capabilities':{'tools':{}},'serverInfo':{'name':'silent','version':'1'}}}), flush=True)
"#).unwrap();
            let mut server = resource_test_server();
            server.protocol_mode = ProtocolMode::Legacy;
            server.transport = McpTransportConfig::Stdio {
                command: vec![
                    "/usr/bin/python3".into(),
                    script.to_string_lossy().into_owned(),
                    stage.into(),
                    marker.to_string_lossy().into_owned(),
                    mcp::LEGACY_PROTOCOL_VERSION.into(),
                ],
                cwd: None,
                environment: BTreeMap::new(),
            };
            let cancellation = McpCancellationToken::default();
            let trigger = cancellation.clone();
            let marker_copy = marker.clone();
            let canceller = std::thread::spawn(move || {
                let deadline = std::time::Instant::now() + Duration::from_secs(5);
                while std::fs::read_to_string(&marker_copy)
                    .ok()
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .is_none()
                {
                    assert!(
                        std::time::Instant::now() < deadline,
                        "child did not reach cancellation stage"
                    );
                    std::thread::sleep(Duration::from_millis(10));
                }
                trigger.cancel();
            });
            let started = std::time::Instant::now();
            let result = with_preparation_runtime(|runtime| {
                prepare_server_cancellable(
                    &server,
                    None,
                    SecretAccess::read_only(Arc::new(MemorySecretStore::new())),
                    runtime,
                    cancellation,
                )
            });
            canceller.join().unwrap();
            assert!(matches!(result, Err(McpError::Cancelled)));
            assert!(started.elapsed() < Duration::from_secs(5));
            let pid: i32 = std::fs::read_to_string(marker)
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            assert_eq!(
                unsafe { libc::kill(pid, 0) },
                -1,
                "cancelled MCP child remains alive"
            );
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::ESRCH)
            );
        }
    }

    fn resource_test_server() -> McpServerConfig {
        McpServerConfig {
            reference: McpServerReference {
                workspace_id: "ws".to_owned(),
                scope: McpScope::User,
                name: "resources".to_owned(),
            },
            transport: McpTransportConfig::Stdio {
                command: vec!["unused".to_owned()],
                cwd: None,
                environment: BTreeMap::new(),
            },
            enabled: true,
            always_on: false,
            protocol_mode: ProtocolMode::Modern,
            owner: None,
            plugin_component: None,
            project_trusted: true,
        }
    }

    fn prepared_resource_peer(
        resources_enabled: bool,
        list_fails: bool,
        closes: Arc<AtomicUsize>,
    ) -> PreparedServer {
        let server = resource_test_server();
        PreparedServer {
            key: McpPoolKey {
                workspace: "ws".to_owned(),
                scope: "user".to_owned(),
                server: "resources".to_owned(),
                config_digest: "config".to_owned(),
                authorization_identity: "anonymous".to_owned(),
                protocol_mode: "modern".to_owned(),
                plugin_generation: None,
            },
            peer: Box::new(ResourcePeer {
                capabilities: McpCapabilities {
                    resources: resources_enabled,
                    ..McpCapabilities::default()
                },
                identity: McpImplementation {
                    name: server.reference.name,
                    version: "1".to_owned(),
                    title: None,
                    icons: None,
                    description: None,
                    website_url: None,
                },
                resources: vec![McpResource {
                    uri: "file:///one".to_owned(),
                    name: "one".to_owned(),
                    mime_type: Some("text/plain".to_owned()),
                }],
                list_fails,
                closes,
            }),
            tools: Vec::new(),
            credential_floors: Vec::new(),
        }
    }

    fn cancellation_rotation_tool(name: &str, external: bool) -> DynamicTool {
        let mut tool = DynamicTool::declared(
            name,
            profile::DynamicToolSource {
                kind: DynamicToolSourceKind::Mcp,
                id: "rotation".to_owned(),
            },
            if external {
                profile::DynamicToolEffect::ExternalProcess
            } else {
                profile::DynamicToolEffect::ReadOnly
            },
            true,
            Vec::new(),
            json_to_ijson(&json!({
                "name":name,
                "description":"rotation test tool",
                "parameters":{
                    "type":"object",
                    "properties":{},
                    "required":[],
                    "additionalProperties":false
                }
            }))
            .expect("tool schema"),
        )
        .expect("dynamic tool");
        if external {
            tool.external_effect = Some(profile::ExternalEffectBinding {
                protocol: profile::ExternalEffectProtocol::IdempotencyReconcileV1,
                reconcile_tool: "orders/get_by_idempotency_key".to_owned(),
            });
        }
        tool
    }

    fn cancellation_route_contract(
        remote_name: &str,
        reconcile_name: Option<&str>,
    ) -> McpRouteContract {
        let identity = McpImplementation {
            name: "rotation".to_owned(),
            version: "1".to_owned(),
            title: None,
            icons: None,
            description: None,
            website_url: None,
        };
        let tool = |name: &str, read_only: bool| McpTool {
            name: name.to_owned(),
            description: "rotation test tool".to_owned(),
            input_schema: json_to_ijson(&json!({
                "type":"object",
                "properties":{},
                "required":[],
                "additionalProperties":false
            }))
            .expect("route tool schema"),
            annotations: mcp::McpToolAnnotations {
                read_only_hint: read_only,
                destructive_hint: !read_only,
            },
            metadata: None,
            execution: None,
        };
        let effect_tool = tool(remote_name, reconcile_name.is_none());
        let reconcile_tool = reconcile_name.map(|name| tool(name, true));
        let digest = route_contract_digest(
            "2026-07-28",
            &identity,
            0,
            &effect_tool,
            reconcile_tool.as_ref(),
        )
        .expect("route contract digest");
        McpRouteContract {
            protocol_version: "2026-07-28".to_owned(),
            server_identity: identity,
            catalog_generation: 0,
            effect_tool,
            reconcile_tool,
            digest,
        }
    }

    fn refresh_test_contract_digest(contract: &mut McpRouteContract) {
        contract.digest = route_contract_digest(
            &contract.protocol_version,
            &contract.server_identity,
            contract.catalog_generation,
            &contract.effect_tool,
            contract.reconcile_tool.as_ref(),
        )
        .expect("refreshed route digest");
    }

    #[test]
    fn reconnected_route_contract_rejects_authority_and_reconcile_drift() {
        let expected =
            cancellation_route_contract("orders/create", Some("orders/get_by_idempotency_key"));
        assert!(validate_reconnected_contract(&expected, &expected).is_ok());

        let mut identity = expected.clone();
        identity.server_identity.version = "2".to_owned();
        refresh_test_contract_digest(&mut identity);

        let mut protocol = expected.clone();
        protocol.protocol_version = "2027-01-01".to_owned();
        refresh_test_contract_digest(&mut protocol);

        let mut schema = expected.clone();
        schema
            .reconcile_tool
            .as_mut()
            .expect("reconcile tool")
            .input_schema = json_to_ijson(&json!({
            "type":"object",
            "properties":{
                "idempotencyKey":{"type":"string"},
                "region":{"type":"string"}
            },
            "required":["idempotencyKey"],
            "additionalProperties":false
        }))
        .expect("changed schema");
        refresh_test_contract_digest(&mut schema);

        let mut annotations = expected.clone();
        annotations
            .reconcile_tool
            .as_mut()
            .expect("reconcile tool")
            .annotations
            .destructive_hint = true;
        refresh_test_contract_digest(&mut annotations);

        for changed in [identity, protocol, schema, annotations] {
            assert!(matches!(
                validate_reconnected_contract(&expected, &changed),
                Err(RoutePeerError::ContractChanged(_))
            ));
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum ReconnectDrift {
        Identity,
        Protocol,
        Effect,
        Reconcile,
    }

    #[test]
    #[ignore = "read-only call to public Cloudflare MCP over production HTTPS loader"]
    fn live_https_loader_exposes_and_executes_cloudflare_documentation() {
        let root = tempfile::tempdir().unwrap();
        let runtime = McpRuntime::open(root.path(), Arc::new(MemorySecretStore::new())).unwrap();
        let server = McpServerConfig {
            reference: McpServerReference {
                workspace_id: "ws".into(),
                scope: McpScope::User,
                name: "cfdocs".into(),
            },
            transport: McpTransportConfig::Http {
                url: "https://docs.mcp.cloudflare.com/mcp".into(),
                headers: BTreeMap::new(),
                oauth: None,
            },
            enabled: true,
            always_on: false,
            protocol_mode: ProtocolMode::Auto,
            owner: None,
            plugin_component: None,
            project_trusted: true,
        };
        runtime
            .mutate(
                "save-cfdocs",
                &McpManagementMutation::Save {
                    server,
                    credential_fields: BTreeMap::new(),
                },
            )
            .unwrap();
        let binding = runtime.prepare_workspace("ws").unwrap();
        assert!(binding.failures.is_empty(), "{:?}", binding.failures);
        let tool = binding
            .catalog
            .tools
            .iter()
            .find(|tool| tool.name == "mcp__cfdocs__search_cloudflare_documentation")
            .expect("projected public tool");
        assert!(binding.authority.supports(tool));
        let request = ToolControl::new(
            "018f0000-0000-7000-8000-000000000001",
            "018f0000-0000-7000-8000-000000000001",
            1,
            "call-cfdocs",
            tool.name.clone(),
            json_to_ijson(&json!({"query":"MCP server/discover"})).unwrap(),
        )
        .unwrap();
        let result = binding
            .authority
            .execute(tool, &request)
            .expect("production HTTPS authority call");
        let result: Value = serde_json::from_slice(&result.canonical_bytes().unwrap()).unwrap();
        assert_ne!(result["isError"], true);
        assert!(
            result["content"]
                .as_array()
                .is_some_and(|parts| parts.iter().any(|part| part["type"] == "text"
                    && part["text"]
                        .as_str()
                        .is_some_and(|text| !text.trim().is_empty()))),
            "expected nonempty documentation content"
        );
    }

    #[test]
    fn loader_exposes_and_executes_remote_greeting_through_dynamic_authority() {
        let root = tempfile::tempdir().unwrap();
        let script = root.path().join("greeting-server");
        let log = root.path().join("requests.log");
        fs::write(&script, r#"#!/bin/sh
log=$1
while IFS= read -r line; do
printf '%s\n' "$line" >> "$log"
case "$line" in
*'"method":"initialize"'*) printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"__LEGACY_VERSION__","capabilities":{"tools":{}},"serverInfo":{"name":"greeting","version":"1"}}}' ;;
*'"method":"tools/list"'*) printf '%s\n' '{"jsonrpc":"2.0","id":2,"result":{"tools":[{"name":"greet","description":"Greeting","annotations":{"readOnlyHint":true},"inputSchema":{"type":"object","properties":{"who":{"type":"string"}},"required":["who"],"additionalProperties":false}}]}}' ;;
*'"method":"tools/call"'*) printf '%s\n' '{"jsonrpc":"2.0","id":3,"result":{"content":[{"type":"text","text":"hello, world"}],"isError":false}}' ;;
esac
done
"#.replace("__LEGACY_VERSION__", mcp::LEGACY_PROTOCOL_VERSION)).unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
        let runtime = McpRuntime::open(root.path(), Arc::new(MemorySecretStore::new())).unwrap();
        let server = McpServerConfig {
            reference: McpServerReference {
                workspace_id: "ws".into(),
                scope: McpScope::User,
                name: "remote".into(),
            },
            transport: McpTransportConfig::Stdio {
                command: vec![
                    script.to_string_lossy().into_owned(),
                    log.to_string_lossy().into_owned(),
                ],
                cwd: None,
                environment: BTreeMap::new(),
            },
            enabled: true,
            always_on: false,
            protocol_mode: ProtocolMode::Legacy,
            owner: None,
            plugin_component: None,
            project_trusted: true,
        };
        runtime
            .mutate(
                "save-greeting",
                &McpManagementMutation::Save {
                    server,
                    credential_fields: BTreeMap::new(),
                },
            )
            .unwrap();
        let binding = runtime.prepare_workspace("ws").unwrap();
        assert!(binding.failures.is_empty(), "{:?}", binding.failures);
        assert_eq!(binding.catalog.tools.len(), 1);
        let tool = &binding.catalog.tools[0];
        assert_eq!(tool.name, "mcp__remote__greet");
        assert!(binding.authority.supports(tool));
        let request = ToolControl::new(
            "018f0000-0000-7000-8000-000000000001",
            "018f0000-0000-7000-8000-000000000001",
            1,
            "call-greeting",
            tool.name.clone(),
            json_to_ijson(&json!({"who":"world"})).unwrap(),
        )
        .unwrap();
        let result = binding.authority.execute(tool, &request).unwrap();
        let result: Value = serde_json::from_slice(&result.canonical_bytes().unwrap()).unwrap();
        assert_eq!(result["isError"], false);
        assert_eq!(result["content"][0]["text"], "hello, world");
        let requests = fs::read_to_string(log).unwrap();
        let calls: Vec<Value> = requests
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .filter(|request: &Value| request["method"] == "tools/call")
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0]["params"]["name"], "greet");
        assert_eq!(calls[0]["params"]["arguments"], json!({"who":"world"}));
    }

    fn write_effect_server(
        path: &Path,
        drift: Option<ReconnectDrift>,
        list_change_on_reconcile: bool,
    ) {
        let identity_version = if matches!(drift, Some(ReconnectDrift::Identity)) {
            "2"
        } else {
            "1"
        };
        let protocol_version = if matches!(drift, Some(ReconnectDrift::Protocol)) {
            "1900-01-01"
        } else {
            mcp::LEGACY_PROTOCOL_VERSION
        };
        let effect_description = if matches!(drift, Some(ReconnectDrift::Effect)) {
            "Create one changed order"
        } else {
            "Create one order"
        };
        let reconcile_description = if matches!(drift, Some(ReconnectDrift::Reconcile)) {
            "Find one changed order"
        } else {
            "Find one order"
        };
        let initialize = serde_json::to_string(&json!({
            "jsonrpc":"2.0",
            "id":1,
            "result":{
                "protocolVersion":protocol_version,
                "capabilities":{"tools":{}},
                "serverInfo":{"name":"orders","version":identity_version}
            }
        }))
        .expect("initialize JSON");
        let tools = serde_json::to_string(&json!({
            "jsonrpc":"2.0",
            "id":2,
            "result":{"tools":[
                {
                    "name":"orders/create",
                    "description":effect_description,
                    "inputSchema":{
                        "type":"object",
                        "properties":{"sku":{"type":"string"}},
                        "required":["sku"],
                        "additionalProperties":false
                    },
                    "annotations":{"readOnlyHint":false,"destructiveHint":true},
                    "_meta":{"io.tekes/externalEffect":{
                        "version":1,
                        "reconcileTool":"orders/get_by_idempotency_key"
                    }}
                },
                {
                    "name":"orders/get_by_idempotency_key",
                    "description":reconcile_description,
                    "inputSchema":{
                        "type":"object",
                        "properties":{"idempotencyKey":{"type":"string"}},
                        "required":["idempotencyKey"],
                        "additionalProperties":false
                    },
                    "annotations":{"readOnlyHint":true,"destructiveHint":false}
                }
            ]}
        }))
        .expect("tools JSON");
        let reconcile = serde_json::to_string(&json!({
            "jsonrpc":"2.0",
            "id":3,
            "result":{"status":"not_found"}
        }))
        .expect("reconcile JSON");
        let list_change = if list_change_on_reconcile {
            "printf '%s\\n' '{\"jsonrpc\":\"2.0\",\"method\":\"notifications/tools/list_changed\"}'; "
        } else {
            ""
        };
        let script = format!(
            "#!/bin/sh\nlog=$1\nwhile IFS= read -r line; do\ncase \"$line\" in\n*'\"method\":\"initialize\"'*) printf 'initialize\\n' >> \"$log\"; printf '%s\\n' '{initialize}' ;;\n*'\"method\":\"tools/list\"'*) printf 'tools/list\\n' >> \"$log\"; printf '%s\\n' '{tools}' ;;\n*'\"name\":\"orders/create\"'*) printf 'effect\\n' >> \"$log\"; printf '%s\\n' '{{\"jsonrpc\":\"2.0\",\"id\":3,\"result\":{{\"ok\":true}}}}' ;;\n*'\"name\":\"orders/get_by_idempotency_key\"'*) printf 'reconcile\\n' >> \"$log\"; {list_change}printf '%s\\n' '{reconcile}' ;;\nesac\ndone\n"
        );
        fs::write(path, script).expect("server script");
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).expect("server mode");
    }

    #[test]
    fn prepare_workspace_degrades_when_the_registry_file_is_unreadable() {
        let root = tempfile::tempdir().unwrap();
        fs::write(
            root.path().join("mcp-servers.json"),
            b"{\"format\":1,\"servers\":[]} \n",
        )
        .unwrap();
        let runtime = McpRuntime::open(root.path(), Arc::new(MemorySecretStore::new())).unwrap();
        let binding = runtime
            .prepare_workspace("ws")
            .expect("a broken registry must not block the launch");
        assert!(binding.catalog.tools.is_empty());
        assert!(binding.failures.is_empty());
        let failure = binding
            .registry_failure
            .expect("registry failure is reported");
        assert!(failure.contains("mcp-servers.json"), "{failure}");
        assert!(failure.contains("not canonical"), "{failure}");

        // The degraded binding is never reused: once the file is repaired the
        // next launch sees the registry again.
        fs::write(
            root.path().join("mcp-servers.json"),
            b"{\"format\":1,\"servers\":[]}\n",
        )
        .unwrap();
        let binding = runtime.prepare_workspace("ws").expect("repaired registry");
        assert_eq!(binding.registry_failure, None);
    }

    fn write_stdio_catalog_server(script: &std::path::Path, tools_json: &str) {
        fs::write(
            script,
            format!(
                "#!/bin/sh\nwhile IFS= read -r line; do\ncase \"$line\" in\n*'\"method\":\"initialize\"'*) printf '%s\\n' '{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{{\"protocolVersion\":\"{}\",\"capabilities\":{{\"tools\":{{}}}},\"serverInfo\":{{\"name\":\"catalog\",\"version\":\"1\"}}}}}}' ;;\n*'\"method\":\"tools/list\"'*) printf '%s\\n' '{{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{{\"tools\":{tools_json}}}}}' ;;\nesac\ndone\n",
                mcp::LEGACY_PROTOCOL_VERSION
            ),
        )
        .unwrap();
        fs::set_permissions(script, fs::Permissions::from_mode(0o755)).unwrap();
    }

    fn stdio_user_server(name: &str, script: &std::path::Path) -> McpServerConfig {
        McpServerConfig {
            reference: McpServerReference {
                workspace_id: "ws".into(),
                scope: McpScope::User,
                name: name.into(),
            },
            transport: McpTransportConfig::Stdio {
                command: vec![script.to_string_lossy().into_owned()],
                cwd: None,
                environment: BTreeMap::new(),
            },
            enabled: true,
            always_on: false,
            protocol_mode: ProtocolMode::Legacy,
            owner: None,
            plugin_component: None,
            project_trusted: true,
        }
    }

    /// One server whose catalog the profile refuses (a schema keyword outside
    /// the dynamic-schema contract) is dropped from the binding by itself: the
    /// other server's tools launch, and the failure names the server, the
    /// tool and the reason for the session notice.
    #[test]
    fn prepare_workspace_drops_only_the_server_whose_catalog_fails_projection() {
        let root = tempfile::tempdir().unwrap();
        let good = root.path().join("good-server");
        let bad = root.path().join("bad-server");
        write_stdio_catalog_server(
            &good,
            r#"[{"name":"greet","description":"Greeting","annotations":{"readOnlyHint":true},"inputSchema":{"type":"object","properties":{"who":{"type":"string"}},"required":["who"],"additionalProperties":false}}]"#,
        );
        write_stdio_catalog_server(
            &bad,
            r#"[{"name":"get_entities","description":"Read entities","annotations":{"readOnlyHint":true},"inputSchema":{"type":"object","properties":{"layout":{"type":"string","not":{"const":"model"}}},"required":[],"additionalProperties":false}}]"#,
        );
        let runtime = McpRuntime::open(root.path(), Arc::new(MemorySecretStore::new())).unwrap();
        for (name, script) in [("good", &good), ("bad", &bad)] {
            runtime
                .mutate(
                    &format!("save-{name}"),
                    &McpManagementMutation::Save {
                        server: stdio_user_server(name, script),
                        credential_fields: BTreeMap::new(),
                    },
                )
                .unwrap();
        }
        let binding = runtime
            .prepare_workspace("ws")
            .expect("one refused catalog must not block the launch");
        assert_eq!(
            binding
                .catalog
                .tools
                .iter()
                .map(|tool| tool.name.as_str())
                .collect::<Vec<_>>(),
            vec!["mcp__good__greet"]
        );
        assert!(binding.authority.supports(&binding.catalog.tools[0]));
        assert_eq!(binding.failures.len(), 1, "{:?}", binding.failures);
        let failure = &binding.failures[0];
        assert_eq!(failure.server.name, "bad");
        assert_eq!(failure.code, "projection-failed");
        let detail = failure.detail.as_deref().expect("projection detail");
        assert!(detail.contains("get_entities"), "{detail}");
        assert!(detail.contains("unsupported keyword"), "{detail}");
        assert_eq!(
            runtime.server_readiness(&failure.server),
            Some("projection-failed")
        );
        assert_eq!(binding.registry_failure, None);
    }

    #[test]
    fn loader_rejects_remote_tool_error_through_dynamic_authority() {
        let root = tempfile::tempdir().unwrap();
        let script = root.path().join("greeting-server");
        let log = root.path().join("requests.log");
        fs::write(&script, r#"#!/bin/sh
log=$1
while IFS= read -r line; do
printf '%s\n' "$line" >> "$log"
case "$line" in
*'"method":"initialize"'*) printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"__LEGACY_VERSION__","capabilities":{"tools":{}},"serverInfo":{"name":"greeting","version":"1"}}}' ;;
*'"method":"tools/list"'*) printf '%s\n' '{"jsonrpc":"2.0","id":2,"result":{"tools":[{"name":"greet","description":"Greeting","annotations":{"readOnlyHint":true},"inputSchema":{"type":"object","properties":{"who":{"type":"string"}},"required":["who"],"additionalProperties":false}}]}}' ;;
*'"method":"tools/call"'*) printf '%s\n' '{"jsonrpc":"2.0","id":3,"result":{"content":[{"type":"text","text":"hello, world"}],"isError":true}}' ;;
esac
done
"#.replace("__LEGACY_VERSION__", mcp::LEGACY_PROTOCOL_VERSION)).unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
        let runtime = McpRuntime::open(root.path(), Arc::new(MemorySecretStore::new())).unwrap();
        let server = McpServerConfig {
            reference: McpServerReference {
                workspace_id: "ws".into(),
                scope: McpScope::User,
                name: "remote".into(),
            },
            transport: McpTransportConfig::Stdio {
                command: vec![
                    script.to_string_lossy().into_owned(),
                    log.to_string_lossy().into_owned(),
                ],
                cwd: None,
                environment: BTreeMap::new(),
            },
            enabled: true,
            always_on: false,
            protocol_mode: ProtocolMode::Legacy,
            owner: None,
            plugin_component: None,
            project_trusted: true,
        };
        runtime
            .mutate(
                "save-greeting",
                &McpManagementMutation::Save {
                    server,
                    credential_fields: BTreeMap::new(),
                },
            )
            .unwrap();
        let binding = runtime.prepare_workspace("ws").unwrap();
        assert!(binding.failures.is_empty(), "{:?}", binding.failures);
        assert_eq!(binding.catalog.tools.len(), 1);
        let tool = &binding.catalog.tools[0];
        assert_eq!(tool.name, "mcp__remote__greet");
        assert!(binding.authority.supports(tool));
        let request = ToolControl::new(
            "018f0000-0000-7000-8000-000000000001",
            "018f0000-0000-7000-8000-000000000001",
            1,
            "call-greeting",
            tool.name.clone(),
            json_to_ijson(&json!({"who":"world"})).unwrap(),
        )
        .unwrap();
        assert!(matches!(binding.authority.execute(tool, &request),
            Err(SupervisorOperationError::ToolFailed(message)) if message.contains("hello, world")));
        let requests = fs::read_to_string(log).unwrap();
        let calls: Vec<Value> = requests
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .filter(|request: &Value| request["method"] == "tools/call")
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0]["params"]["name"], "greet");
        assert_eq!(calls[0]["params"]["arguments"], json!({"who":"world"}));
    }

    #[test]
    fn peer_eviction_reconnect_drift_never_registers_or_queries_reconciliation() {
        for drift in [
            ReconnectDrift::Identity,
            ReconnectDrift::Protocol,
            ReconnectDrift::Effect,
            ReconnectDrift::Reconcile,
        ] {
            let root = tempfile::tempdir().expect("root");
            let script = root.path().join("mcp-server");
            let log = root.path().join("server.log");
            write_effect_server(&script, None, false);
            let runtime =
                McpRuntime::open(root.path(), Arc::new(MemorySecretStore::new())).expect("runtime");
            let server = McpServerConfig {
                reference: McpServerReference {
                    workspace_id: "ws".to_owned(),
                    scope: McpScope::User,
                    name: "orders".to_owned(),
                },
                transport: McpTransportConfig::Stdio {
                    command: vec![
                        script.to_string_lossy().into_owned(),
                        log.to_string_lossy().into_owned(),
                    ],
                    cwd: None,
                    environment: BTreeMap::new(),
                },
                enabled: true,
                always_on: false,
                protocol_mode: ProtocolMode::Legacy,
                owner: None,
                plugin_component: None,
                project_trusted: true,
            };
            runtime
                .mutate(
                    "save-orders",
                    &McpManagementMutation::Save {
                        server,
                        credential_fields: BTreeMap::new(),
                    },
                )
                .expect("save server");
            let binding = runtime.prepare_workspace("ws").expect("prepare workspace");
            let tool = binding
                .catalog
                .tools
                .iter()
                .find(|tool| tool.external_effect.is_some())
                .expect("effect tool")
                .clone();
            let route = binding
                .authority
                .routes
                .get(&tool.name)
                .expect("effect route")
                .clone();
            assert!(
                binding
                    .authority
                    .broker
                    .remove(route.key.clone())
                    .expect("evict peer")
            );
            write_effect_server(&script, Some(drift), false);
            let request = ToolControl::new(
                "018f0000-0000-7000-8000-000000000001",
                "018f0000-0000-7000-8000-000000000001",
                1,
                "call-orders",
                tool.name.clone(),
                json_to_ijson(&json!({"sku":"one"})).expect("arguments"),
            )
            .expect("request");

            assert!(matches!(
                binding.authority.reconcile(&tool, &request),
                ExternalEffectResolution::Conflicted { .. }
            ));
            assert_eq!(
                binding
                    .authority
                    .broker
                    .catalog_generation(route.key)
                    .expect("generation"),
                None,
                "drifted peer must not register for {drift:?}"
            );
            let log = fs::read_to_string(&log).expect("server log");
            assert_eq!(log.matches("initialize").count(), 2, "{drift:?}");
            assert!(!log.contains("reconcile"), "{drift:?}");
            assert!(!log.contains("effect"), "{drift:?}");
        }
    }

    #[test]
    fn not_found_then_catalog_drift_blocks_retry_before_effect_dispatch() {
        let root = tempfile::tempdir().expect("root");
        let script = root.path().join("mcp-server");
        let log = root.path().join("server.log");
        write_effect_server(&script, None, true);
        let runtime =
            McpRuntime::open(root.path(), Arc::new(MemorySecretStore::new())).expect("runtime");
        let server = McpServerConfig {
            reference: McpServerReference {
                workspace_id: "ws".to_owned(),
                scope: McpScope::User,
                name: "orders".to_owned(),
            },
            transport: McpTransportConfig::Stdio {
                command: vec![
                    script.to_string_lossy().into_owned(),
                    log.to_string_lossy().into_owned(),
                ],
                cwd: None,
                environment: BTreeMap::new(),
            },
            enabled: true,
            always_on: false,
            protocol_mode: ProtocolMode::Legacy,
            owner: None,
            plugin_component: None,
            project_trusted: true,
        };
        runtime
            .mutate(
                "save-orders",
                &McpManagementMutation::Save {
                    server,
                    credential_fields: BTreeMap::new(),
                },
            )
            .expect("save server");
        let binding = runtime.prepare_workspace("ws").expect("prepare workspace");
        let tool = binding
            .catalog
            .tools
            .iter()
            .find(|tool| tool.external_effect.is_some())
            .expect("effect tool")
            .clone();
        let route = binding
            .authority
            .routes
            .get(&tool.name)
            .expect("effect route")
            .clone();
        let request = ToolControl::new(
            "018f0000-0000-7000-8000-000000000001",
            "018f0000-0000-7000-8000-000000000001",
            1,
            "call-orders",
            tool.name.clone(),
            json_to_ijson(&json!({"sku":"one"})).expect("arguments"),
        )
        .expect("request");

        assert!(matches!(
            binding.authority.reconcile(&tool, &request),
            ExternalEffectResolution::NotFound
        ));
        assert!(matches!(
            binding.authority.execute(&tool, &request),
            Err(SupervisorOperationError::EffectConflicted(_))
        ));
        assert_eq!(
            binding
                .authority
                .broker
                .catalog_generation(route.key)
                .expect("generation"),
            None
        );
        let log = fs::read_to_string(log).expect("server log");
        assert_eq!(log.matches("reconcile").count(), 1);
        assert!(!log.contains("effect"));
    }

    #[test]
    fn cancellation_rotates_generation_before_reconcile_and_later_calls() {
        let root = tempfile::tempdir().expect("root");
        let registry = McpRegistryStore::new(root.path());
        let mut server = resource_test_server();
        server.reference.name = "rotation".to_owned();
        registry
            .mutate_idempotent(
                "save-rotation",
                &McpManagementMutation::Save {
                    server: server.clone(),
                    credential_fields: BTreeMap::new(),
                },
            )
            .expect("publish server");

        let key = McpPoolKey {
            workspace: "ws".to_owned(),
            scope: "user".to_owned(),
            server: "rotation".to_owned(),
            config_digest: "rotation-config".to_owned(),
            authorization_identity: "anonymous".to_owned(),
            protocol_mode: "modern".to_owned(),
            plugin_generation: None,
        };
        let (dispatched_tx, dispatched_rx) = std::sync::mpsc::channel();
        let effect_keys = Arc::new(Mutex::new(Vec::new()));
        let reconcile_keys = Arc::new(Mutex::new(Vec::new()));
        let cancellation_states = Arc::new(Mutex::new(Vec::new()));
        let catalog_generation = Arc::new(AtomicUsize::new(0));
        let reconcile_results = Arc::new(Mutex::new(VecDeque::from([
            json!({"status":"confirmed","value":{"state":"created"}}),
            json!({"status":"not_found"}),
            json!({"status":"unknown","reason":"authority unavailable"}),
        ])));
        let broker = McpBroker::start().expect("broker");
        let handle = broker.handle();
        handle
            .register(
                key.clone(),
                Box::new(CancellationRotationPeer {
                    identity: McpImplementation {
                        name: "rotation".to_owned(),
                        version: "1".to_owned(),
                        title: None,
                        icons: None,
                        description: None,
                        website_url: None,
                    },
                    dispatched: Mutex::new(Some(dispatched_tx)),
                    effect_keys: Arc::clone(&effect_keys),
                    reconcile_keys: Arc::clone(&reconcile_keys),
                    cancellation_states: Arc::clone(&cancellation_states),
                    reconcile_results,
                    catalog_generation: Arc::clone(&catalog_generation),
                }),
                false,
            )
            .expect("register peer");

        let external = cancellation_rotation_tool("mcp__rotation__orders_2fcreate", true);
        let ordinary = cancellation_rotation_tool("mcp__rotation__orders_2fstatus", false);
        let routes = BTreeMap::from([
            (
                external.name.clone(),
                McpRoute {
                    key: key.clone(),
                    remote_name: "orders/create".to_owned(),
                    credential_floors: Vec::new(),
                    server: server.clone(),
                    plugin_generation: None,
                    contract: cancellation_route_contract(
                        "orders/create",
                        Some("orders/get_by_idempotency_key"),
                    ),
                },
            ),
            (
                ordinary.name.clone(),
                McpRoute {
                    key,
                    remote_name: "orders/status".to_owned(),
                    credential_floors: Vec::new(),
                    server,
                    plugin_generation: None,
                    contract: cancellation_route_contract("orders/status", None),
                },
            ),
        ]);
        let authority = Arc::new(McpDynamicSupervisorAuthority::new(
            handle,
            routes,
            SecretAccess::read_only(Arc::new(MemorySecretStore::new())),
            registry,
            None,
        ));
        let request = ToolControl::new(
            "018f0000-0000-7000-8000-000000000001",
            "018f0000-0000-7000-8000-000000000001",
            1,
            "call-rotation",
            external.name.clone(),
            json_to_ijson(&json!({})).expect("arguments"),
        )
        .expect("request");

        let executing = Arc::clone(&authority);
        let execute_tool = external.clone();
        let execute_request = request.clone();
        let execution =
            std::thread::spawn(move || executing.execute(&execute_tool, &execute_request));
        dispatched_rx
            .recv_timeout(std::time::Duration::from_secs(2))
            .expect("effect dispatched");
        authority.cancel_inflight();
        assert!(matches!(
            execution.join().expect("execution thread"),
            Err(SupervisorOperationError::Unavailable(_))
        ));

        assert!(matches!(
            authority.reconcile(&external, &request),
            ExternalEffectResolution::Confirmed(_)
        ));
        assert!(matches!(
            authority.reconcile(&external, &request),
            ExternalEffectResolution::NotFound
        ));
        assert!(matches!(
            authority.reconcile(&external, &request),
            ExternalEffectResolution::Unknown { .. }
        ));
        let ordinary_result = authority
            .execute(&ordinary, &request)
            .expect("later ordinary call uses fresh cancellation generation");
        let ordinary_json: Value =
            serde_json::from_slice(&ordinary_result.canonical_bytes().expect("ordinary bytes"))
                .expect("ordinary JSON");
        assert_eq!(ordinary_json, json!({"ordinary":"ok"}));

        catalog_generation.store(1, Ordering::Release);
        assert!(matches!(
            authority.reconcile(&external, &request),
            ExternalEffectResolution::Conflicted { .. }
        ));

        assert_eq!(
            effect_keys
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .as_slice(),
            [request.request_id.as_str()]
        );
        assert_eq!(
            reconcile_keys
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .as_slice(),
            [
                request.request_id.as_str(),
                request.request_id.as_str(),
                request.request_id.as_str(),
            ]
        );
        assert!(
            cancellation_states
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .iter()
                .all(|cancelled| !cancelled),
            "reconciliation and later calls must use the fresh token generation"
        );
        drop(authority);
        drop(broker);
    }

    #[test]
    fn resource_peers_close_on_every_negative_exit() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime");
        let server = resource_test_server();

        for (resources_enabled, list_fails) in [(false, false), (true, true)] {
            let closes = Arc::new(AtomicUsize::new(0));
            let mut prepared =
                prepared_resource_peer(resources_enabled, list_fails, Arc::clone(&closes));
            let result = collect_prepared_resources(&runtime, &mut prepared, &server, "instance");
            if resources_enabled {
                assert!(result.is_err());
            } else {
                assert_eq!(result.expect("unsupported is an empty catalog").len(), 0);
            }
            assert_eq!(closes.load(Ordering::SeqCst), 1);
        }

        for (binding, uri) in [("stale", "file:///one"), ("valid", "file:///missing")] {
            let closes = Arc::new(AtomicUsize::new(0));
            let mut prepared = prepared_resource_peer(true, false, Arc::clone(&closes));
            let resources = vec![McpResource {
                uri: "file:///one".to_owned(),
                name: "one".to_owned(),
                mime_type: Some("text/plain".to_owned()),
            }];
            let catalog_generation =
                resource_catalog_generation("instance", 0, &resources).expect("catalog generation");
            let valid = resource_binding_digest(&server.reference, &prepared, &catalog_generation)
                .expect("binding digest");
            let selected = if binding == "valid" { &valid } else { binding };
            assert!(
                read_prepared_resource(
                    &runtime,
                    &mut prepared,
                    &server.reference,
                    selected,
                    uri,
                    "instance",
                )
                .is_err()
            );
            assert_eq!(closes.load(Ordering::SeqCst), 1);
        }
    }

    #[test]
    fn production_plugin_architecture_uses_the_package_vocabulary() {
        if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            assert_eq!(current_plugin_architecture(), "arm64");
        } else {
            assert_eq!(current_plugin_architecture(), std::env::consts::ARCH);
        }
    }

    #[test]
    fn production_http_authorization_is_resolved_per_send() {
        let secrets = MemorySecretStore::new();
        secrets
            .publish(
                "header-ref",
                SecretRecord::Active {
                    generation: 1,
                    material: "header-one".to_owned(),
                },
            )
            .expect("header generation one");
        secrets
            .publish(
                "oauth-ref",
                SecretRecord::Active {
                    generation: 1,
                    material: "bearer-one".to_owned(),
                },
            )
            .expect("OAuth generation one");
        let configured = BTreeMap::from([(
            "X-Private".to_owned(),
            CredentialValue::Credential {
                credential: "header-ref".to_owned(),
            },
        )]);
        let first = http_request_authorization(&configured, Some("oauth-ref"), &secrets, None)
            .expect("first send projection");
        let first_identity = first.identity.clone();
        assert_eq!(first.headers["X-Private"], "header-one");
        assert_eq!(first.bearer.as_deref(), Some("bearer-one"));
        drop(first);

        secrets
            .publish(
                "header-ref",
                SecretRecord::Active {
                    generation: 2,
                    material: "header-two".to_owned(),
                },
            )
            .expect("header rotation");
        secrets
            .publish(
                "oauth-ref",
                SecretRecord::Active {
                    generation: 2,
                    material: "bearer-two".to_owned(),
                },
            )
            .expect("OAuth rotation");
        let second = http_request_authorization(&configured, Some("oauth-ref"), &secrets, None)
            .expect("second send projection");
        assert_ne!(second.identity, first_identity);
        assert_eq!(second.headers["X-Private"], "header-two");
        assert_eq!(second.bearer.as_deref(), Some("bearer-two"));
        drop(second);

        secrets
            .publish("oauth-ref", SecretRecord::Revoked { generation: 3 })
            .expect("OAuth revocation");
        assert!(
            http_request_authorization(&configured, Some("oauth-ref"), &secrets, None).is_err(),
            "revoked authorization must fail before an HTTP send"
        );
    }

    /// A task-capable peer: `tools/call` creates a working task; polls walk
    /// working → input_required → (after one input update) completed.
    struct AsyncTaskPeer {
        identity: McpImplementation,
        capabilities: McpCapabilities,
        calls: Arc<AtomicUsize>,
        gets: Arc<AtomicUsize>,
        updates: Arc<Mutex<Vec<Value>>>,
        cancels: Arc<AtomicUsize>,
        polls_after_update: Arc<AtomicUsize>,
        /// TTLs of task-augmented calls; a plain call on a task-required tool panics.
        augmented_ttls: Arc<Mutex<Vec<u64>>>,
    }

    impl AsyncTaskPeer {
        fn state(&self, status: &str) -> Value {
            let mut state = json!({
                "taskId":"task-async","status":status,
                "createdAt":"2026-09-05T00:00:00Z","lastUpdatedAt":"2026-09-05T00:00:01Z"
            });
            if status == "input_required" {
                state["inputRequests"] = json!({"confirm":{"question":"Proceed?"}});
            }
            if status == "completed" {
                state["result"] =
                    json!({"content":[{"type":"text","text":"async done"}],"isError":false});
            }
            state
        }
    }

    impl McpPeer for AsyncTaskPeer {
        fn server_name(&self) -> &str {
            &self.identity.name
        }
        fn protocol_version(&self) -> &str {
            "2026-07-28"
        }
        fn server_identity(&self) -> Option<&McpImplementation> {
            Some(&self.identity)
        }
        fn capabilities(&self) -> &McpCapabilities {
            &self.capabilities
        }
        fn catalog_generation(&self) -> u64 {
            0
        }
        fn list_tools(&mut self) -> mcp::McpPeerFuture<'_, Vec<mcp::McpTool>> {
            Box::pin(async { Ok(Vec::new()) })
        }
        fn list_prompts(&mut self) -> mcp::McpPeerFuture<'_, Vec<mcp::McpPrompt>> {
            Box::pin(async { Ok(Vec::new()) })
        }
        fn list_resources(&mut self) -> mcp::McpPeerFuture<'_, Vec<McpResource>> {
            Box::pin(async { Ok(Vec::new()) })
        }
        fn call_tool(
            &mut self,
            _name: &str,
            _arguments: IJsonValue,
            _cancellation: McpCancellationToken,
        ) -> mcp::McpPeerFuture<'_, IJsonValue> {
            panic!("a task-required tool must be called with task augmentation");
        }
        fn call_tool_augmented(
            &mut self,
            _name: &str,
            _arguments: IJsonValue,
            _context: Option<McpToolCallContext>,
            task_ttl_ms: u64,
            _cancellation: McpCancellationToken,
        ) -> mcp::McpPeerFuture<'_, IJsonValue> {
            self.calls.fetch_add(1, Ordering::AcqRel);
            self.augmented_ttls.lock().unwrap().push(task_ttl_ms);
            let created = json!({"task": self.state("working")});
            Box::pin(async move { json_to_ijson(&created).map_err(McpError::Protocol) })
        }
        fn get_prompt(
            &mut self,
            _name: &str,
            _arguments: IJsonValue,
        ) -> mcp::McpPeerFuture<'_, IJsonValue> {
            Box::pin(async { Err(McpError::Unsupported("unused".to_owned())) })
        }
        fn read_resource(&mut self, _uri: &str) -> mcp::McpPeerFuture<'_, IJsonValue> {
            Box::pin(async { Err(McpError::Unsupported("unused".to_owned())) })
        }
        fn task_operation(
            &mut self,
            method: &str,
            params: IJsonValue,
        ) -> mcp::McpPeerFuture<'_, IJsonValue> {
            let params: Value = serde_json::from_slice(&params.canonical_bytes().expect("params"))
                .expect("params JSON");
            assert_eq!(
                params["taskId"], "task-async",
                "operations name the exact bound task"
            );
            let state = match method {
                "tasks/get" => {
                    let poll = self.gets.fetch_add(1, Ordering::AcqRel);
                    let updated = !self.updates.lock().unwrap().is_empty();
                    if self.cancels.load(Ordering::Acquire) > 0 {
                        self.state("cancelled")
                    } else if updated && self.polls_after_update.fetch_add(1, Ordering::AcqRel) == 0
                    {
                        self.state("working")
                    } else if updated {
                        self.state("completed")
                    } else if poll == 0 {
                        self.state("working")
                    } else {
                        self.state("input_required")
                    }
                }
                "tasks/update" => {
                    self.updates
                        .lock()
                        .unwrap()
                        .push(params["inputResponses"].clone());
                    self.state("working")
                }
                "tasks/cancel" => {
                    self.cancels.fetch_add(1, Ordering::AcqRel);
                    self.state("cancelled")
                }
                other => panic!("unexpected task operation {other}"),
            };
            Box::pin(async move { json_to_ijson(&state).map_err(McpError::Protocol) })
        }
        fn close(&mut self) -> mcp::McpPeerFuture<'_, ()> {
            Box::pin(async { Ok(()) })
        }
    }

    /// A task-shaped `tools/call` result binds a continuation instead of
    /// completing the call; each step is a receipt-bound remote operation;
    /// the input answer is sent exactly once; the terminal value is the
    /// embedded tool result; replays return the immutable receipt without a
    /// remote request; a step after the terminal is refused.
    #[test]
    fn task_result_binds_continuation_and_steps_reach_one_terminal() {
        let root = tempfile::tempdir().expect("root");
        let registry = McpRegistryStore::new(root.path());
        let mut server = resource_test_server();
        server.reference.name = "rotation".to_owned();
        registry
            .mutate_idempotent(
                "save-async",
                &McpManagementMutation::Save {
                    server: server.clone(),
                    credential_fields: BTreeMap::new(),
                },
            )
            .expect("publish server");
        let key = McpPoolKey {
            workspace: "ws".to_owned(),
            scope: "user".to_owned(),
            server: "rotation".to_owned(),
            config_digest: "rotation-config".to_owned(),
            authorization_identity: "anonymous".to_owned(),
            protocol_mode: "modern".to_owned(),
            plugin_generation: None,
        };
        let calls = Arc::new(AtomicUsize::new(0));
        let gets = Arc::new(AtomicUsize::new(0));
        let updates = Arc::new(Mutex::new(Vec::new()));
        let cancels = Arc::new(AtomicUsize::new(0));
        let augmented_ttls = Arc::new(Mutex::new(Vec::new()));
        let broker = McpBroker::start().expect("broker");
        let handle = broker.handle();
        handle
            .register(
                key.clone(),
                Box::new(AsyncTaskPeer {
                    identity: McpImplementation {
                        name: "rotation".to_owned(),
                        version: "1".to_owned(),
                        title: None,
                        icons: None,
                        description: None,
                        website_url: None,
                    },
                    capabilities: McpCapabilities {
                        tools: true,
                        prompts: false,
                        resources: false,
                        tasks: true,
                        subscriptions: false,
                        extensions: BTreeMap::new(),
                        tools_list_changed: false,
                        prompts_list_changed: false,
                        resources_list_changed: false,
                        resources_subscribe: false,
                    },
                    calls: Arc::clone(&calls),
                    gets: Arc::clone(&gets),
                    updates: Arc::clone(&updates),
                    cancels: Arc::clone(&cancels),
                    polls_after_update: Arc::new(AtomicUsize::new(0)),
                    augmented_ttls: Arc::clone(&augmented_ttls),
                }),
                false,
            )
            .expect("register peer");
        let tool = cancellation_rotation_tool("mcp__rotation__async_2frun", false);
        let mut contract = cancellation_route_contract("async/run", None);
        contract.effect_tool.execution = Some(mcp::McpToolExecution {
            task_support: mcp::McpTaskSupport::Required,
        });
        let routes = BTreeMap::from([(
            tool.name.clone(),
            McpRoute {
                key: key.clone(),
                remote_name: "async/run".to_owned(),
                credential_floors: Vec::new(),
                server,
                plugin_generation: None,
                contract,
            },
        )]);
        let authority = McpDynamicSupervisorAuthority::new(
            handle,
            routes,
            SecretAccess::read_only(Arc::new(MemorySecretStore::new())),
            registry,
            None,
        );
        let request = ToolControl::new(
            "018f0000-0000-7000-8000-000000000001",
            "018f0000-0000-7000-8000-000000000001",
            1,
            "call-async",
            tool.name.clone(),
            json_to_ijson(&json!({})).expect("arguments"),
        )
        .expect("request");
        let continuation_root = root.path().join("continuations");

        let outcome = authority
            .execute_outcome(&tool, &request, &continuation_root)
            .expect("initial execution");
        let DynamicExecutionOutcome::Pending {
            continuation_id,
            state,
        } = outcome
        else {
            panic!("task result must be pending")
        };
        assert_eq!(calls.load(Ordering::Acquire), 1);
        assert_eq!(
            augmented_ttls.lock().unwrap().as_slice(),
            &[REQUIRED_TASK_TTL_MS],
            "a task-required tool is called with task augmentation"
        );
        let state: Value = serde_json::from_slice(&state.canonical_bytes().unwrap()).unwrap();
        assert_eq!(state["status"], "working");
        assert!(
            continuation_root
                .join(&continuation_id)
                .join("binding.json")
                .is_file(),
            "the binding is durable before the pending result"
        );

        let step = |n: u64, action: ContinuationOperation| {
            ToolContinuationRequest::new(request.clone(), continuation_id.clone(), n, action)
                .expect("step request")
        };
        let first = authority
            .continue_task(
                &tool,
                &step(1, ContinuationOperation::Query),
                &continuation_root,
            )
            .expect("query 1");
        assert!(matches!(
            &first.result,
            ToolContinuationOutcome::Pending { next_step: 2, .. }
        ));
        let second = authority
            .continue_task(
                &tool,
                &step(2, ContinuationOperation::Query),
                &continuation_root,
            )
            .expect("query 2");
        let ToolContinuationOutcome::Pending { state, .. } = &second.result else {
            panic!("second poll is still pending")
        };
        let state: Value = serde_json::from_slice(&state.canonical_bytes().unwrap()).unwrap();
        assert_eq!(state["status"], "input_required");
        assert_eq!(state["inputRequests"]["confirm"]["question"], "Proceed?");

        let answer = json_to_ijson(&json!({"answer":"yes"})).unwrap();
        let update = step(
            3,
            ContinuationOperation::Update {
                input_responses: answer,
            },
        );
        let third = authority
            .continue_task(&tool, &update, &continuation_root)
            .expect("update");
        assert!(
            matches!(
                &third.result,
                ToolContinuationOutcome::Pending { next_step: 4, .. }
            ),
            "an update returns the post-update state, not a terminal"
        );
        assert_eq!(
            updates.lock().unwrap().as_slice(),
            &[json!({"answer":"yes"})]
        );
        let fourth = authority
            .continue_task(
                &tool,
                &step(4, ContinuationOperation::Query),
                &continuation_root,
            )
            .expect("final poll");
        let ToolContinuationOutcome::Completed { value } = &fourth.result else {
            panic!("completed task yields the embedded result")
        };
        let value: Value = serde_json::from_slice(&value.canonical_bytes().unwrap()).unwrap();
        assert_eq!(value["content"][0]["text"], "async done");

        let remote_before = gets.load(Ordering::Acquire);
        let replay = authority
            .continue_task(
                &tool,
                &step(4, ContinuationOperation::Query),
                &continuation_root,
            )
            .expect("replay");
        assert_eq!(
            replay, fourth,
            "a committed step replays its immutable receipt"
        );
        assert_eq!(
            gets.load(Ordering::Acquire),
            remote_before,
            "replay makes no remote request"
        );
        assert!(
            authority
                .continue_task(
                    &tool,
                    &step(5, ContinuationOperation::Query),
                    &continuation_root
                )
                .is_err(),
            "no step follows a terminal receipt"
        );
        assert!(
            authority
                .continue_task(
                    &tool,
                    &step(3, ContinuationOperation::Cancel),
                    &continuation_root
                )
                .is_err(),
            "a changed action for a committed step conflicts"
        );
        assert_eq!(
            calls.load(Ordering::Acquire),
            1,
            "the initial side effect ran exactly once"
        );
        assert_eq!(cancels.load(Ordering::Acquire), 0);
        drop(authority);
        drop(broker);
    }
}
