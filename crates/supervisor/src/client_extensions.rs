//! Slice-14F Client extension registry.
//!
//! The frozen Session Endpoint table remains in `endpoint_host`. Each value
//! below is one atomic capability group: construction installs every method
//! in the group or none of them, and dispatch is delegated to exactly one
//! durable authority backend.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use base64::Engine as _;
use chrono::Utc;
use endpoint::{DurableHandoffProof, EndpointHostCall, MethodClass, RpcDurableIdentity};
use plugins::{
    InstallOptions, MacOsNativeHelperVerifier, PluginReceipt, PluginSource, PluginStore,
};
use profile::ResourceCatalog;
use schema::{EventKind, IJsonValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Digest as _;
use thread_search::{ArchiveVisibility, SearchRequest, ThreadSearchAuthority};
use tools::{BuiltinManifest, fixed_schema};

use crate::endpoint_host::{ProductionEndpointRoutes, ProductionRouteFailure};
use crate::mcp_runtime::{McpManagementRoutes, McpRuntime};
use crate::process_host::ProductionProcessHost;
use crate::resource_capability::{
    ClientResourceService, CommandRunRequest, EndpointCommandInputAuthority,
    map_prompt_materialize_error, map_resource_failure,
};

pub const RESOURCE_METHODS: [(&str, MethodClass); 5] = [
    ("skills/list", MethodClass::ReadOnly),
    ("commands/list", MethodClass::ReadOnly),
    ("commands/run", MethodClass::Mutation),
    ("resources/list", MethodClass::ReadOnly),
    ("resources/read", MethodClass::ReadOnly),
];
pub const INITIAL_PRESET_METHODS: [(&str, MethodClass); 1] =
    [("session.initialPresets", MethodClass::ReadOnly)];
pub const RECOVERY_METHODS: [(&str, MethodClass); 1] =
    [("sessions.recover", MethodClass::Mutation)];
pub const ATTACHMENT_METHODS: [(&str, MethodClass); 3] = [
    ("attachments.policy", MethodClass::ReadOnly),
    ("session.uploadFile", MethodClass::Mutation),
    ("session.fileAttachment", MethodClass::ReadOnly),
];
pub const APPROVAL_METHODS: [(&str, MethodClass); 3] = [
    ("approvals.policy", MethodClass::ReadOnly),
    ("approvals.mode", MethodClass::ReadOnly),
    ("approvals.select", MethodClass::Mutation),
];
pub const FEEDBACK_METHODS: [(&str, MethodClass); 3] = [
    ("feedback.list", MethodClass::ReadOnly),
    ("feedback.put", MethodClass::Mutation),
    ("feedback.delete", MethodClass::Mutation),
];
pub const SETTINGS_METHODS: [(&str, MethodClass); 4] = [
    ("settings.describe", MethodClass::ReadOnly),
    ("settings.mutate", MethodClass::Mutation),
    ("settings.update", MethodClass::Mutation),
    ("settings.document", MethodClass::ReadOnly),
];
pub const GOAL_METHODS: [(&str, MethodClass); 5] = [
    ("goals.get", MethodClass::ReadOnly),
    ("goals.edit", MethodClass::Mutation),
    ("goals.clear", MethodClass::Mutation),
    ("goals.pause", MethodClass::Mutation),
    ("goals.resume", MethodClass::Mutation),
];
pub const SUBAGENT_METHODS: [(&str, MethodClass); 1] = [("subagents.list", MethodClass::ReadOnly)];
pub const HOST_FILE_METHODS: [(&str, MethodClass); 4] = [
    ("directory.list", MethodClass::ReadOnly),
    ("directory.create", MethodClass::Mutation),
    ("session.references.files", MethodClass::ReadOnly),
    ("session.references.sessions", MethodClass::ReadOnly),
];
pub const FILE_METHODS: [(&str, MethodClass); 3] = [
    ("session.files.stat", MethodClass::ReadOnly),
    ("session.files.read", MethodClass::ReadOnly),
    ("session.files.readBytes", MethodClass::ReadOnly),
];

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FilePageRequest {
    session_id: String,
    path: String,
    offset: Option<u64>,
    limit: Option<usize>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AttachmentPolicyRequest {
    session_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ApprovalModeRequest {
    session_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ApprovalSelectRequest {
    session_id: String,
    mode: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct UploadFileRequest {
    session_id: String,
    name: String,
    media_type: Option<String>,
    data: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FileAttachmentRequest {
    session_id: String,
    attachment_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RecoveryRequest {
    session_ids: Vec<String>,
}
pub const TOOL_METHODS: [(&str, MethodClass); 2] = [
    ("tools/list", MethodClass::ReadOnly),
    ("tools/resolve", MethodClass::ReadOnly),
];
pub const PLUGIN_METHODS: [(&str, MethodClass); 8] = [
    ("plugin/list", MethodClass::ReadOnly),
    ("plugin/get", MethodClass::ReadOnly),
    ("plugin/inspect", MethodClass::ReadOnly),
    ("plugin/install", MethodClass::Mutation),
    ("plugin/setEnabled", MethodClass::Mutation),
    ("plugin/setGrants", MethodClass::Mutation),
    ("plugin/remove", MethodClass::Mutation),
    ("plugin/components", MethodClass::ReadOnly),
];
pub const MCP_METHODS: [(&str, MethodClass); 7] = [
    ("mcp.list", MethodClass::ReadOnly),
    ("mcp.get", MethodClass::ReadOnly),
    ("mcp.save", MethodClass::Mutation),
    ("mcp.remove", MethodClass::Mutation),
    ("mcp.probe", MethodClass::ReadOnly),
    ("mcp.oauth.start", MethodClass::Mutation),
    ("mcp.oauth.remove", MethodClass::Mutation),
];
pub const SCHEDULE_METHODS: [(&str, MethodClass); 4] = [
    ("schedule.list", MethodClass::ReadOnly),
    ("schedule.save", MethodClass::Mutation),
    ("schedule.delete", MethodClass::Mutation),
    ("schedule.runNow", MethodClass::Mutation),
];
pub const THREAD_SEARCH_METHODS: [(&str, MethodClass); 2] = [
    ("thread.search", MethodClass::ReadOnly),
    ("session.search", MethodClass::ReadOnly),
];
pub const USAGE_METHODS: [(&str, MethodClass); 2] = [
    ("usage.summary", MethodClass::ReadOnly),
    ("usage.cacheAttribution", MethodClass::ReadOnly),
];

/// Semantic authority behind the transport-neutral registry. Mutation
/// implementations receive the endpoint rpc id unchanged; dropping or
/// replacing it would break their durable idempotency namespace.
pub trait ClientExtensionAuthority: Send + Sync {
    fn execute(
        &self,
        operation: &str,
        payload: &Value,
        principal: &str,
        rpc_id: &str,
        _recovering: bool,
    ) -> Result<IJsonValue, ProductionRouteFailure>;
}

pub struct AtomicCapabilityRoutes {
    id: &'static str,
    methods: &'static [(&'static str, MethodClass)],
    authority: Arc<dyn ClientExtensionAuthority>,
}

impl AtomicCapabilityRoutes {
    #[must_use]
    pub fn new(
        id: &'static str,
        methods: &'static [(&'static str, MethodClass)],
        authority: Arc<dyn ClientExtensionAuthority>,
    ) -> Self {
        assert!(
            !methods.is_empty(),
            "an advertised capability cannot be empty"
        );
        Self {
            id,
            methods,
            authority,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &'static str {
        self.id
    }

    fn class(&self, operation: &str) -> Option<MethodClass> {
        self.methods
            .iter()
            .find_map(|(name, class)| (*name == operation).then_some(*class))
    }
}

impl ProductionEndpointRoutes for AtomicCapabilityRoutes {
    fn capabilities(&self) -> BTreeSet<String> {
        self.methods
            .iter()
            .map(|(name, _)| (*name).to_owned())
            .collect()
    }

    fn extension_method_class(&self, method: &str) -> Option<MethodClass> {
        self.class(method)
    }

    fn validate_extension_payload(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        if self.class(operation).is_none() || !payload.is_object() {
            return Err(failure(
                "bad-request",
                "Invalid Client extension request",
                serde_json::json!({"operation":operation}),
            ));
        }
        validate_payload(operation, payload)
    }

    fn extension_failure_is_exact(
        &self,
        operation: &str,
        failure: &ProductionRouteFailure,
    ) -> bool {
        // Every backend is responsible for emitting only the closed contract
        // vocabulary. This registry additionally blocks accidental raw Rust
        // and transport errors by requiring the public kebab-case shape.
        error_allowed(operation, self.class(operation), &failure.code)
    }

    fn execute(
        &self,
        request: &EndpointHostCall,
        payload: &Value,
        principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        if self.class(&request.operation).is_none() {
            return Err(failure(
                "unsupported-capability",
                "Capability is unavailable",
                serde_json::json!({"operation":request.operation}),
            ));
        }
        let result = self.authority.execute(
            &request.operation,
            payload,
            principal,
            &request.rpc_id,
            request.recovering,
        )?;
        if self.class(&request.operation) == Some(MethodClass::Mutation) {
            request
                .handoff
                .mark_handed_off(DurableHandoffProof {
                    delivery: "extension-authority-barrier".to_owned(),
                    durable_identity: Some(RpcDurableIdentity {
                        kind: "extension-rpc".to_owned(),
                        id: request.rpc_id.clone(),
                        seq: None,
                    }),
                })
                .map_err(|_| {
                    failure(
                        "internal",
                        "Durable handoff proof failed",
                        serde_json::json!({}),
                    )
                })?;
        }
        Ok(result)
    }
}

#[must_use]
pub fn common_capability_routes(
    authority: Arc<dyn ClientExtensionAuthority>,
) -> Vec<Arc<dyn ProductionEndpointRoutes>> {
    [
        ("resources.v1", RESOURCE_METHODS.as_slice()),
        ("initialPresets.v1", INITIAL_PRESET_METHODS.as_slice()),
        ("recovery.v1", RECOVERY_METHODS.as_slice()),
        ("attachments.v1", ATTACHMENT_METHODS.as_slice()),
        ("approvals.v1", APPROVAL_METHODS.as_slice()),
        ("hostFiles.v1", HOST_FILE_METHODS.as_slice()),
        ("feedback.v1", FEEDBACK_METHODS.as_slice()),
        ("settings.v1", SETTINGS_METHODS.as_slice()),
        ("goals.v1", GOAL_METHODS.as_slice()),
        ("subagents.v1", SUBAGENT_METHODS.as_slice()),
        ("sessionFiles.v1", FILE_METHODS.as_slice()),
        ("tools.v1", TOOL_METHODS.as_slice()),
        ("plugins.v1", PLUGIN_METHODS.as_slice()),
        ("schedule.v1", SCHEDULE_METHODS.as_slice()),
        ("threadSearch.v1", THREAD_SEARCH_METHODS.as_slice()),
        ("usage.v1", USAGE_METHODS.as_slice()),
    ]
    .into_iter()
    .map(|(id, methods)| {
        Arc::new(AtomicCapabilityRoutes::new(
            id,
            methods,
            Arc::clone(&authority),
        )) as Arc<dyn ProductionEndpointRoutes>
    })
    .collect()
}

/// Concrete binding to the Slice-11--14 authorities. There is deliberately no
/// Product-specific branch: plugin and MCP rows go through their generic stores.
pub struct ProductionClientExtensions {
    root: PathBuf,
    workspace_resources: BTreeMap<String, ResourceCatalog>,
    commands: ClientResourceService<EndpointCommandInputAuthority>,
    mcp: Arc<McpRuntime>,
    plugins: Option<Arc<Mutex<PluginStore<MacOsNativeHelperVerifier>>>>,
    search: ThreadSearchAuthority,
    process: Arc<ProductionProcessHost>,
}

impl ProductionClientExtensions {
    pub fn open(
        root: impl AsRef<Path>,
        resources: ResourceCatalog,
        workspace_resources: BTreeMap<String, ResourceCatalog>,
        commands: EndpointCommandInputAuthority,
        process: Arc<ProductionProcessHost>,
    ) -> Result<Self, ProductionRouteFailure> {
        let root = root.as_ref().to_path_buf();
        let plugins = process.plugin_store();
        let mcp = process.mcp_runtime();
        let commands =
            commands.with_attachment_authority(endpoint::AttachmentAuthority::open(&root));
        Ok(Self {
            commands: ClientResourceService::new(resources, commands),
            workspace_resources,
            mcp,
            plugins,
            search: ThreadSearchAuthority::open(&root).map_err(map_search_error)?,
            root,
            process,
        })
    }

    /// The same session-scoped attachment authority `session.prompt` and
    /// `session.attachment` use: one storage root, content-addressed assets.
    fn attachments(&self) -> endpoint::AttachmentAuthority {
        endpoint::AttachmentAuthority::open(&self.root)
    }

    pub fn routes(self: &Arc<Self>) -> Vec<Arc<dyn ProductionEndpointRoutes>> {
        let authority: Arc<dyn ClientExtensionAuthority> = self.clone();
        let mut groups = vec![
            ("resources.v1", RESOURCE_METHODS.as_slice()),
            ("initialPresets.v1", INITIAL_PRESET_METHODS.as_slice()),
            ("recovery.v1", RECOVERY_METHODS.as_slice()),
            ("attachments.v1", ATTACHMENT_METHODS.as_slice()),
            ("approvals.v1", APPROVAL_METHODS.as_slice()),
            ("hostFiles.v1", HOST_FILE_METHODS.as_slice()),
            ("feedback.v1", FEEDBACK_METHODS.as_slice()),
            ("settings.v1", SETTINGS_METHODS.as_slice()),
            ("goals.v1", GOAL_METHODS.as_slice()),
            ("subagents.v1", SUBAGENT_METHODS.as_slice()),
            ("sessionFiles.v1", FILE_METHODS.as_slice()),
            ("tools.v1", TOOL_METHODS.as_slice()),
            ("schedule.v1", SCHEDULE_METHODS.as_slice()),
            ("threadSearch.v1", THREAD_SEARCH_METHODS.as_slice()),
            ("usage.v1", USAGE_METHODS.as_slice()),
        ];
        if self.plugins.is_some() {
            groups.push(("plugins.v1", PLUGIN_METHODS.as_slice()));
        }
        let mut routes = groups
            .into_iter()
            .map(|(id, methods)| {
                Arc::new(AtomicCapabilityRoutes::new(
                    id,
                    methods,
                    Arc::clone(&authority),
                )) as Arc<dyn ProductionEndpointRoutes>
            })
            .collect::<Vec<_>>();
        routes.push(Arc::new(McpManagementRoutes::new(
            self.process.mcp_runtime(),
        )));
        routes
    }

    fn plugin_store(
        &self,
    ) -> Result<&Mutex<PluginStore<MacOsNativeHelperVerifier>>, ProductionRouteFailure> {
        self.plugins.as_deref().ok_or_else(|| {
            failure(
                "unsupported-capability",
                "Plugin capability is unavailable",
                serde_json::json!({}),
            )
        })
    }

    fn workspace_snapshot(
        &self,
        workspace_id: &str,
    ) -> Result<profile::ConfigSnapshot, ProductionRouteFailure> {
        profile::ConfigRepository::open(&self.root)
            .and_then(|repository| repository.resolve(workspace_id))
            .map_err(|_| {
                failure(
                    "workspace-not-found",
                    "Workspace was not found",
                    serde_json::json!({"workspaceId":workspace_id}),
                )
            })
    }

    fn workspace_resource_catalog(
        &self,
        workspace_id: &str,
    ) -> Result<&ResourceCatalog, ProductionRouteFailure> {
        self.workspace_resources.get(workspace_id).ok_or_else(|| {
            failure(
                "resource-not-found",
                "Workspace resource catalog was not found",
                serde_json::json!({}),
            )
        })
    }

    fn plugin_operation(
        &self,
        operation: &str,
        payload: &Value,
        rpc_id: &str,
    ) -> Result<PluginEndpointOperation, ProductionRouteFailure> {
        let authority = self.root.join("plugins");
        for directory in [
            authority.join("endpoint-rpc"),
            authority.join("endpoint-sources"),
        ] {
            std::fs::create_dir_all(&directory).map_err(|error| {
                failure(
                    "internal",
                    "Plugin endpoint journal layout failed",
                    serde_json::json!({"reason":error.to_string()}),
                )
            })?;
            std::fs::File::open(&directory)
                .and_then(|file| file.sync_all())
                .map_err(|error| {
                    failure(
                        "internal",
                        "Plugin endpoint journal layout is not durable",
                        serde_json::json!({"reason":error.to_string()}),
                    )
                })?;
        }
        std::fs::File::open(&authority)
            .and_then(|file| file.sync_all())
            .map_err(|error| {
                failure(
                    "internal",
                    "Plugin endpoint journal root is not durable",
                    serde_json::json!({"reason":error.to_string()}),
                )
            })?;
        let lock =
            store::NamedLock::exclusive(authority.join(".endpoint-rpc.lock")).map_err(|error| {
                failure(
                    "internal",
                    "Plugin receipt lock failed",
                    serde_json::json!({"reason":error.to_string()}),
                )
            })?;
        self.recover_plugin_endpoint_journal_locked(&authority)?;
        let stem = format!("{:x}", sha2::Sha256::digest(rpc_id.as_bytes()));
        let path = authority.join("endpoint-rpc").join(format!("{stem}.json"));
        let stage = authority.join("endpoint-sources").join(&stem);
        let request_digest = format!(
            "sha256-{:x}",
            sha2::Sha256::digest(
                serde_json_canonicalizer::to_vec(&serde_json::json!({
                    "operation":operation,
                    "payload":payload,
                }))
                .map_err(|_| failure(
                    "internal",
                    "Plugin request cannot be canonicalized",
                    serde_json::json!({})
                ))?
            )
        );
        let record = if path.is_file() {
            let record = read_plugin_endpoint_record(&path)?;
            if record.rpc_id != rpc_id
                || record.operation != operation
                || record.request_digest != request_digest
            {
                return Err(failure(
                    "idempotency-conflict",
                    "Plugin rpcId was reused with different bytes",
                    serde_json::json!({"rpcId":rpc_id}),
                ));
            }
            record
        } else {
            PluginEndpointRecord {
                format: 1,
                rpc_id: rpc_id.to_owned(),
                operation: operation.to_owned(),
                request_digest,
                phase: PluginEndpointPhase::Prepared,
                payload: payload.clone(),
                originally_present: None,
                result: None,
                failure: None,
            }
        };
        Ok(PluginEndpointOperation {
            _lock: Some(lock),
            path,
            stage,
            record,
        })
    }

    fn recover_plugin_endpoint_journal(&self) -> Result<(), ProductionRouteFailure> {
        let authority = self.root.join("plugins");
        let _lock =
            store::NamedLock::exclusive(authority.join(".endpoint-rpc.lock")).map_err(|error| {
                failure(
                    "internal",
                    "Plugin receipt lock failed",
                    serde_json::json!({"reason":error.to_string()}),
                )
            })?;
        self.recover_plugin_endpoint_journal_locked(&authority)
    }

    fn recover_plugin_endpoint_journal_locked(
        &self,
        authority: &Path,
    ) -> Result<(), ProductionRouteFailure> {
        let directory = authority.join("endpoint-rpc");
        let mut prepared = Vec::new();
        if directory.is_dir() {
            for entry in std::fs::read_dir(&directory).map_err(|error| {
                failure(
                    "internal",
                    "Plugin receipt directory is unreadable",
                    serde_json::json!({"reason":error.to_string()}),
                )
            })? {
                let path = entry
                    .map_err(|error| {
                        failure(
                            "internal",
                            "Plugin receipt directory is unreadable",
                            serde_json::json!({"reason":error.to_string()}),
                        )
                    })?
                    .path();
                if path.extension().and_then(|value| value.to_str()) != Some("json") {
                    return Err(failure(
                        "internal",
                        "Plugin receipt directory contains an unknown entry",
                        serde_json::json!({}),
                    ));
                }
                let record = read_plugin_endpoint_record(&path)?;
                if record.phase == PluginEndpointPhase::Prepared {
                    prepared.push((path, record));
                }
            }
        }
        if prepared.len() > 1 {
            return Err(failure(
                "internal",
                "Plugin mutation journal has multiple prepared operations",
                serde_json::json!({}),
            ));
        }
        if let Some((path, record)) = prepared.pop() {
            let stem = format!("{:x}", sha2::Sha256::digest(record.rpc_id.as_bytes()));
            if path.file_stem().and_then(|value| value.to_str()) != Some(&stem) {
                return Err(failure(
                    "internal",
                    "Plugin receipt path does not match rpcId",
                    serde_json::json!({}),
                ));
            }
            let mut operation = PluginEndpointOperation {
                _lock: None,
                path,
                stage: authority.join("endpoint-sources").join(stem),
                record,
            };
            self.complete_prepared_plugin_operation(&mut operation)?;
        }
        Ok(())
    }

    fn complete_prepared_plugin_operation(
        &self,
        operation: &mut PluginEndpointOperation,
    ) -> Result<(), ProductionRouteFailure> {
        let result = match operation.record.operation.as_str() {
            "plugin/install" => {
                let request: PluginInstallRequest = decode(&operation.record.payload)?;
                let source = PluginSource::from_path(&operation.stage).map_err(map_plugin_error)?;
                let mut store = lock(self.plugin_store()?)?;
                let inspection = store.inspect(&source).map_err(map_plugin_error)?;
                if inspection.package_digest != request.package_digest {
                    return Err(failure(
                        "internal",
                        "Frozen plugin source no longer matches its receipt",
                        serde_json::json!({}),
                    ));
                }
                let install = store.install(
                    &source,
                    InstallOptions {
                        grants: request.grants.into_iter().collect(),
                        enable: request.enable,
                        allow_downgrade: request.allow_downgrade,
                        allow_same_version_replacement: request.allow_same_version_replacement,
                    },
                );
                let plugin = plugin_mutation_result(operation, install)?;
                drop(store);
                self.plugin_mutation_value(plugin)
            }
            "plugin/setEnabled" => {
                let request: PluginEnabledRequest = decode(&operation.record.payload)?;
                let mut store = lock(self.plugin_store()?)?;
                let update = store.set_enabled(&request.plugin_id, request.enabled);
                let plugin = plugin_mutation_result(operation, update)?;
                drop(store);
                self.plugin_mutation_value(plugin)
            }
            "plugin/setGrants" => {
                let request: PluginGrantsRequest = decode(&operation.record.payload)?;
                let mut store = lock(self.plugin_store()?)?;
                let update =
                    store.set_grants(&request.plugin_id, request.grants.into_iter().collect());
                let plugin = plugin_mutation_result(operation, update)?;
                drop(store);
                self.plugin_mutation_value(plugin)
            }
            "plugin/remove" => {
                let request: PluginIdRequest = decode(&operation.record.payload)?;
                let originally_present = operation.record.originally_present.ok_or_else(|| {
                    failure(
                        "internal",
                        "Prepared plugin removal omitted original presence",
                        serde_json::json!({}),
                    )
                })?;
                let mut store = lock(self.plugin_store()?)?;
                let removal = store.remove(&request.plugin_id);
                let _ = plugin_mutation_result(operation, removal)?;
                drop(store);
                serde_json::json!({"format":1,"removed":originally_present,"readiness":self.plugin_readiness()})
            }
            _ => {
                return Err(failure(
                    "internal",
                    "Plugin receipt names an unknown mutation",
                    serde_json::json!({}),
                ));
            }
        };
        operation.complete(result)?;
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct OptionalWorkspace {
    workspace_id: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EmptyRequest {}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PluginIdRequest {
    plugin_id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PluginInspectRequest {
    archive_path: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PluginInstallRequest {
    archive_path: String,
    package_digest: String,
    enable: bool,
    grants: Vec<String>,
    allow_downgrade: bool,
    allow_same_version_replacement: bool,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PluginEnabledRequest {
    plugin_id: String,
    enabled: bool,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PluginGrantsRequest {
    plugin_id: String,
    grants: Vec<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ResourceListRequest {
    workspace_id: String,
    source: String,
    after: Option<String>,
    limit: Option<usize>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ResourceReadRequest {
    workspace_id: String,
    reference: Value,
    uri: String,
}
#[derive(Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
enum ResourceReference {
    #[serde(rename = "skill")]
    Skill {
        #[serde(rename = "workspaceId")]
        workspace_id: String,
        name: String,
        #[serde(rename = "contentDigest")]
        content_digest: String,
    },
    #[serde(rename = "mcp")]
    Mcp {
        server: mcp::McpServerReference,
        #[serde(rename = "bindingDigest")]
        binding_digest: String,
    },
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ToolListRequest {
    workspace_id: String,
    session_id: Option<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ToolResolveRequest {
    workspace_id: String,
    session_id: Option<String>,
    name: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ScheduleSaveRequest {
    definition: schedule::ScheduleDefinition,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TaskIdRequest {
    task_id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SessionSearchRequest {
    query: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ThreadSearchRequest {
    workspace_id: String,
    query: String,
    limit: usize,
    visibility: ArchiveVisibility,
    after: Option<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SessionIdRequest {
    session_id: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum PluginEndpointPhase {
    Prepared,
    Complete,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PluginEndpointFailure {
    code: String,
    message: String,
    details: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PluginEndpointRecord {
    format: u32,
    rpc_id: String,
    operation: String,
    request_digest: String,
    payload: Value,
    phase: PluginEndpointPhase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    originally_present: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    failure: Option<PluginEndpointFailure>,
}

struct PluginEndpointOperation {
    _lock: Option<store::NamedLock>,
    path: PathBuf,
    stage: PathBuf,
    record: PluginEndpointRecord,
}

impl PluginEndpointOperation {
    fn publish(&self) -> Result<(), ProductionRouteFailure> {
        let bytes = serde_json_canonicalizer::to_vec(&self.record).map_err(|_| {
            failure(
                "internal",
                "Plugin receipt cannot be canonicalized",
                serde_json::json!({}),
            )
        })?;
        store::AtomicPublisher::replace(&self.path, &bytes).map_err(|error| {
            failure(
                "internal",
                "Plugin receipt cannot be published",
                serde_json::json!({"reason":error.to_string()}),
            )
        })
    }

    fn prepare(&mut self, originally_present: Option<bool>) -> Result<(), ProductionRouteFailure> {
        self.record.originally_present = originally_present;
        self.publish()
    }

    fn complete(&mut self, result: Value) -> Result<IJsonValue, ProductionRouteFailure> {
        self.record.phase = PluginEndpointPhase::Complete;
        self.record.result = Some(result.clone());
        self.publish()?;
        if self.stage.is_dir() {
            let _ = std::fs::remove_dir_all(&self.stage);
        }
        encode(&result)
    }

    fn fail(
        &mut self,
        error: ProductionRouteFailure,
    ) -> Result<ProductionRouteFailure, ProductionRouteFailure> {
        let details = ijson_value(&error.details)?;
        self.record.phase = PluginEndpointPhase::Failed;
        self.record.failure = Some(PluginEndpointFailure {
            code: error.code.clone(),
            message: error.message.clone(),
            details,
        });
        self.publish()?;
        if self.stage.is_dir() {
            let _ = std::fs::remove_dir_all(&self.stage);
        }
        Ok(error)
    }

    fn completed(&self) -> Result<Option<IJsonValue>, ProductionRouteFailure> {
        match self.record.phase {
            PluginEndpointPhase::Prepared => Ok(None),
            PluginEndpointPhase::Complete => {
                if self.stage.is_dir() {
                    let _ = std::fs::remove_dir_all(&self.stage);
                }
                self.record
                    .result
                    .as_ref()
                    .map(encode)
                    .transpose()
                    .and_then(|result| {
                        result.ok_or_else(|| {
                            failure(
                                "internal",
                                "Complete plugin receipt omitted result",
                                serde_json::json!({}),
                            )
                        })
                    })
                    .map(Some)
            }
            PluginEndpointPhase::Failed => {
                let error = self.record.failure.as_ref().ok_or_else(|| {
                    failure(
                        "internal",
                        "Failed plugin receipt omitted its error",
                        serde_json::json!({}),
                    )
                })?;
                Err(failure(&error.code, &error.message, error.details.clone()))
            }
        }
    }
}

fn read_plugin_endpoint_record(
    path: &Path,
) -> Result<PluginEndpointRecord, ProductionRouteFailure> {
    let bytes = std::fs::read(path).map_err(|error| {
        failure(
            "internal",
            "Plugin receipt is unreadable",
            serde_json::json!({"reason":error.to_string()}),
        )
    })?;
    let record: PluginEndpointRecord = serde_json::from_slice(&bytes).map_err(|_| {
        failure(
            "internal",
            "Plugin receipt is corrupt",
            serde_json::json!({}),
        )
    })?;
    let canonical = serde_json_canonicalizer::to_vec(&record).map_err(|_| {
        failure(
            "internal",
            "Plugin receipt is corrupt",
            serde_json::json!({}),
        )
    })?;
    let phase_valid = match record.phase {
        PluginEndpointPhase::Prepared => record.result.is_none() && record.failure.is_none(),
        PluginEndpointPhase::Complete => record.result.is_some() && record.failure.is_none(),
        PluginEndpointPhase::Failed => record.result.is_none() && record.failure.is_some(),
    };
    if bytes != canonical || record.format != 1 || !phase_valid {
        return Err(failure(
            "internal",
            "Plugin receipt is corrupt",
            serde_json::json!({}),
        ));
    }
    let digest = format!(
        "sha256-{:x}",
        sha2::Sha256::digest(
            serde_json_canonicalizer::to_vec(&serde_json::json!({
                "operation":record.operation,
                "payload":record.payload,
            }))
            .map_err(|_| failure(
                "internal",
                "Plugin receipt is corrupt",
                serde_json::json!({})
            ))?
        )
    );
    if digest != record.request_digest {
        return Err(failure(
            "internal",
            "Plugin receipt request digest does not match its intent",
            serde_json::json!({}),
        ));
    }
    Ok(record)
}

fn plugin_view(receipt: &PluginReceipt) -> Value {
    serde_json::json!({
        "pluginId":receipt.plugin_id,"version":receipt.version,"displayName":receipt.display_name,
        "packageDigest":receipt.package_digest,"integrity":receipt.integrity,
        "requestedCapabilities":receipt.requested_capabilities,"grantedCapabilities":receipt.granted_capabilities,
        "enabled":receipt.enabled
    })
}

fn decode<T: for<'de> Deserialize<'de>>(value: &Value) -> Result<T, ProductionRouteFailure> {
    serde_json::from_value(value.clone()).map_err(|error| bad_request(&error.to_string()))
}
fn encode(value: &impl serde::Serialize) -> Result<IJsonValue, ProductionRouteFailure> {
    IJsonValue::parse(&serde_json::to_vec(value).map_err(|_| {
        failure(
            "internal",
            "Response serialization failed",
            serde_json::json!({}),
        )
    })?)
    .map_err(|_| failure("internal", "Response is not I-JSON", serde_json::json!({})))
}
fn ijson_value(value: &IJsonValue) -> Result<Value, ProductionRouteFailure> {
    serde_json::from_slice(&value.canonical_bytes().map_err(|_| {
        failure(
            "internal",
            "I-JSON serialization failed",
            serde_json::json!({}),
        )
    })?)
    .map_err(|_| failure("internal", "I-JSON decoding failed", serde_json::json!({})))
}
fn bad_request(reason: &str) -> ProductionRouteFailure {
    failure(
        "bad-request",
        "Invalid Client extension request",
        serde_json::json!({"reason":reason}),
    )
}
fn lock<T>(value: &Mutex<T>) -> Result<std::sync::MutexGuard<'_, T>, ProductionRouteFailure> {
    value.lock().map_err(|_| {
        failure(
            "internal",
            "Authority lock is poisoned",
            serde_json::json!({}),
        )
    })
}
fn decode_resource_cursor(value: &str, generation: &str) -> Result<usize, ProductionRouteFailure> {
    let (found, offset) = value.rsplit_once(':').ok_or_else(|| {
        failure(
            "resource-stale",
            "Resource cursor is malformed",
            serde_json::json!({}),
        )
    })?;
    if found != generation {
        return Err(failure(
            "resource-stale",
            "Resource generation changed",
            serde_json::json!({}),
        ));
    }
    offset.parse().map_err(|_| {
        failure(
            "resource-stale",
            "Resource cursor is malformed",
            serde_json::json!({}),
        )
    })
}
fn normalize_mcp_resource(value: &Value) -> Result<Value, ProductionRouteFailure> {
    let contents = value
        .get("contents")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            failure(
                "resource-not-found",
                "MCP resource result is invalid",
                serde_json::json!({}),
            )
        })?;
    if contents.len() != 1 {
        return Err(failure(
            "resource-too-large",
            "MCP resource must contain one bounded item",
            serde_json::json!({}),
        ));
    }
    let item = contents[0].as_object().ok_or_else(|| {
        failure(
            "resource-not-found",
            "MCP resource result is invalid",
            serde_json::json!({}),
        )
    })?;
    if let Some(text) = item.get("text").and_then(Value::as_str) {
        if text.len() > 4 * 1024 * 1024 {
            return Err(failure(
                "resource-too-large",
                "Resource exceeds 4 MiB",
                serde_json::json!({}),
            ));
        }
        let mut content = serde_json::json!({"text":text});
        if let Some(mime) = item.get("mimeType").and_then(Value::as_str) {
            content["mimeType"] = Value::String(mime.to_owned());
        }
        return Ok(content);
    }
    let blob = item.get("blob").and_then(Value::as_str).ok_or_else(|| {
        failure(
            "resource-not-found",
            "MCP resource result is invalid",
            serde_json::json!({}),
        )
    })?;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(blob)
        .map_err(|_| {
            failure(
                "resource-not-found",
                "MCP resource base64 is invalid",
                serde_json::json!({}),
            )
        })?;
    if decoded.len() > 4 * 1024 * 1024 {
        return Err(failure(
            "resource-too-large",
            "Resource exceeds 4 MiB",
            serde_json::json!({}),
        ));
    }
    Ok(
        serde_json::json!({"blobBase64":blob,"mimeType":item.get("mimeType").and_then(Value::as_str).unwrap_or("application/octet-stream")}),
    )
}
fn numeric_token(value: Option<&Value>) -> Result<u64, ProductionRouteFailure> {
    match value.and_then(Value::as_str) {
        None => Ok(0),
        Some(value) => value.parse().map_err(|_| {
            failure(
                "source-corrupt",
                "Usage token value is invalid",
                serde_json::json!({}),
            )
        }),
    }
}

#[derive(Debug)]
struct CacheAttributionGroup {
    target: Value,
    attempts: BTreeSet<String>,
    input_tokens: String,
    output_tokens: String,
    cache_read_tokens: String,
    cost: Option<String>,
    cost_members: usize,
}

impl CacheAttributionGroup {
    fn new(target: Value) -> Self {
        Self {
            target,
            attempts: BTreeSet::new(),
            input_tokens: "0".to_owned(),
            output_tokens: "0".to_owned(),
            cache_read_tokens: "0".to_owned(),
            cost: None,
            cost_members: 0,
        }
    }

    fn add(
        &mut self,
        attempt: &str,
        input: &str,
        output: &str,
        cache_read: &str,
        cost: Option<&str>,
    ) -> Result<(), ProductionRouteFailure> {
        if !self.attempts.insert(attempt.to_owned()) {
            return Err(failure(
                "source-corrupt",
                "Effective usage repeats an attempt in one target group",
                serde_json::json!({}),
            ));
        }
        self.input_tokens = add_unsigned_decimal(&self.input_tokens, input)?;
        self.output_tokens = add_unsigned_decimal(&self.output_tokens, output)?;
        self.cache_read_tokens = add_unsigned_decimal(&self.cache_read_tokens, cache_read)?;
        match (&mut self.cost, cost) {
            (None, Some(value)) if self.cost_members == 0 => {
                self.cost = Some(canonical_cost(value)?);
            }
            (Some(total), Some(value)) => {
                *total = add_canonical_cost(total, value)?;
            }
            (None, None) => {}
            (Some(_), None) | (None, Some(_)) => {
                return Err(failure(
                    "source-corrupt",
                    "Usage cost availability changed within one exact target",
                    serde_json::json!({}),
                ));
            }
        }
        self.cost_members += 1;
        Ok(())
    }

    fn value(self) -> Result<Value, ProductionRouteFailure> {
        let attempts = u64::try_from(self.attempts.len()).map_err(|_| {
            failure(
                "source-corrupt",
                "Usage attempt count overflowed",
                serde_json::json!({}),
            )
        })?;
        if attempts > 9_007_199_254_740_991 {
            return Err(failure(
                "source-corrupt",
                "Usage attempt count exceeds I-JSON",
                serde_json::json!({}),
            ));
        }
        let mut value = serde_json::json!({
            "target":self.target,
            "attempts":attempts,
            "inputTokens":self.input_tokens,
            "outputTokens":self.output_tokens,
            "cacheReadTokens":self.cache_read_tokens,
        });
        if let Some(cost) = self.cost {
            value["cost"] = Value::String(cost);
        }
        Ok(value)
    }
}

fn canonical_unsigned(value: &str) -> Result<&str, ProductionRouteFailure> {
    if value == "0" || (!value.starts_with('0') && value.bytes().all(|byte| byte.is_ascii_digit()))
    {
        Ok(value)
    } else {
        Err(failure(
            "source-corrupt",
            "Usage token value is not canonical unsigned decimal",
            serde_json::json!({}),
        ))
    }
}

fn add_unsigned_decimal(left: &str, right: &str) -> Result<String, ProductionRouteFailure> {
    let left = canonical_unsigned(left)?;
    let right = canonical_unsigned(right)?;
    add_decimal_digits(left, right)
}

fn add_decimal_digits(left: &str, right: &str) -> Result<String, ProductionRouteFailure> {
    if !left.bytes().all(|byte| byte.is_ascii_digit())
        || !right.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(failure(
            "source-corrupt",
            "Usage decimal contains a non-digit",
            serde_json::json!({}),
        ));
    }
    let mut carry = 0u8;
    let mut result = Vec::with_capacity(left.len().max(right.len()) + 1);
    let mut left = left.bytes().rev();
    let mut right = right.bytes().rev();
    loop {
        let a = left.next().map(|byte| byte - b'0');
        let b = right.next().map(|byte| byte - b'0');
        if a.is_none() && b.is_none() && carry == 0 {
            break;
        }
        let sum = a.unwrap_or(0) + b.unwrap_or(0) + carry;
        result.push(b'0' + sum % 10);
        carry = sum / 10;
    }
    result.reverse();
    let result = String::from_utf8(result).map_err(|_| {
        failure(
            "internal",
            "Usage decimal addition failed",
            serde_json::json!({}),
        )
    })?;
    let result = result.trim_start_matches('0');
    Ok(if result.is_empty() {
        "0".to_owned()
    } else {
        result.to_owned()
    })
}

fn canonical_cost(value: &str) -> Result<String, ProductionRouteFailure> {
    let (integer, fraction) = match value.split_once('.') {
        Some((integer, fraction)) => (integer, Some(fraction)),
        None => (value, None),
    };
    canonical_unsigned(integer)?;
    if let Some(fraction) = fraction {
        if fraction.is_empty()
            || !fraction.bytes().all(|byte| byte.is_ascii_digit())
            || fraction.ends_with('0')
        {
            return Err(failure(
                "source-corrupt",
                "Usage cost is not canonical decimal",
                serde_json::json!({}),
            ));
        }
    }
    if integer == "0" && fraction.is_some_and(|digits| digits.bytes().all(|byte| byte == b'0')) {
        return Err(failure(
            "source-corrupt",
            "Usage zero cost is not canonical",
            serde_json::json!({}),
        ));
    }
    Ok(value.to_owned())
}

fn add_canonical_cost(left: &str, right: &str) -> Result<String, ProductionRouteFailure> {
    let left = canonical_cost(left)?;
    let right = canonical_cost(right)?;
    let left_scale = left.split_once('.').map_or(0, |(_, digits)| digits.len());
    let right_scale = right.split_once('.').map_or(0, |(_, digits)| digits.len());
    let scale = left_scale.max(right_scale);
    let left_digits = left.replace('.', "") + &"0".repeat(scale - left_scale);
    let right_digits = right.replace('.', "") + &"0".repeat(scale - right_scale);
    let mut sum = add_decimal_digits(&left_digits, &right_digits)?;
    if scale == 0 {
        return Ok(sum);
    }
    if sum.len() <= scale {
        sum = format!("{}{}", "0".repeat(scale + 1 - sum.len()), sum);
    }
    let point = sum.len() - scale;
    sum.insert(point, '.');
    while sum.ends_with('0') {
        sum.pop();
    }
    if sum.ends_with('.') {
        sum.pop();
    }
    Ok(sum)
}

fn usage_route_target(
    epoch: &Value,
    assets: &store::AssetStore,
) -> Result<Value, ProductionRouteFailure> {
    let asset = epoch
        .pointer("/system/asset")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            failure(
                "source-corrupt",
                "Epoch profile asset is absent",
                serde_json::json!({}),
            )
        })?;
    let digest = epoch
        .pointer("/system/digest")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            failure(
                "source-corrupt",
                "Epoch profile digest is absent",
                serde_json::json!({}),
            )
        })?;
    if asset != format!("sha256-{digest}") {
        return Err(failure(
            "source-corrupt",
            "Epoch profile asset and digest disagree",
            serde_json::json!({}),
        ));
    }
    let bytes = assets.read_verified(asset).map_err(|_| {
        failure(
            "source-corrupt",
            "Epoch profile asset is corrupt",
            serde_json::json!({}),
        )
    })?;
    let profile = IJsonValue::parse(&bytes).map_err(|_| {
        failure(
            "source-corrupt",
            "Epoch profile asset is not I-JSON",
            serde_json::json!({}),
        )
    })?;
    if profile.canonical_bytes().map_err(|_| {
        failure(
            "source-corrupt",
            "Epoch profile is not canonical",
            serde_json::json!({}),
        )
    })? != bytes
    {
        return Err(failure(
            "source-corrupt",
            "Epoch profile asset is not canonical",
            serde_json::json!({}),
        ));
    }
    let profile = ijson_value(&profile)?;
    let target = profile
        .get("target")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            failure(
                "source-corrupt",
                "Epoch profile target is absent",
                serde_json::json!({}),
            )
        })?;
    let route = target
        .get("route")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            failure(
                "source-corrupt",
                "Epoch route evidence is absent",
                serde_json::json!({}),
            )
        })?;
    let field = |object: &serde_json::Map<String, Value>, name: &str| {
        object
            .get(name)
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .ok_or_else(|| {
                failure(
                    "source-corrupt",
                    "Epoch target field is absent",
                    serde_json::json!({"field":name}),
                )
            })
    };
    let protocol_family = field(target, "protocol_family")?;
    let dialect_id = field(target, "dialect_id")?;
    let model_profile_id = field(target, "model_profile_id")?;
    let endpoint_owner = field(route, "endpoint_owner")?;
    let gateway_translation = field(route, "gateway_translation")?;
    let exact_sku = field(route, "exact_sku")?;
    let evidence_revision = field(route, "evidence_revision")?;
    if epoch.get("adapter").and_then(Value::as_str) != Some(dialect_id.as_str())
        || epoch.get("model").and_then(Value::as_str) != Some(exact_sku.as_str())
    {
        return Err(failure(
            "source-corrupt",
            "Epoch summary disagrees with its exact target",
            serde_json::json!({}),
        ));
    }
    Ok(serde_json::json!({
        "protocolFamily":protocol_family,
        "dialectId":dialect_id,
        "modelProfileId":model_profile_id,
        "endpointOwner":endpoint_owner,
        "gatewayTranslation":gateway_translation,
        "exactSku":exact_sku,
        "evidenceRevision":evidence_revision,
    }))
}
fn mcp_readiness_code(error: &mcp::McpError) -> &'static str {
    match error {
        mcp::McpError::Timeout(_) => "timeout",
        mcp::McpError::Transport(_) => "transport",
        _ => "invalid",
    }
}
fn map_mcp_resource_error(error: mcp::McpError) -> ProductionRouteFailure {
    let code = if matches!(&error, mcp::McpError::Conflict(reason) if reason.contains("stale")) {
        "resource-stale"
    } else {
        "resource-not-found"
    };
    failure(
        code,
        "MCP resource operation failed",
        serde_json::json!({"reason":error.to_string()}),
    )
}
fn map_plugin_error(error: plugins::PluginError) -> ProductionRouteFailure {
    use plugins::PluginError::*;
    let code = match &error {
        NotInstalled(_) => "plugin-not-found",
        InvalidArchive(_) | InvalidManifest(_) | InvalidVersion(_) | Incompatible(_) => {
            "plugin-invalid"
        }
        Signature(_) | PublisherTrustRequired(_) => "plugin-untrusted",
        UndeclaredGrant(_) | MissingGrant(_) => "plugin-grants",
        ComponentCollision { .. } => "plugin-collision",
        DowngradeRejected { .. } | SameVersionChanged(_) => "plugin-version-conflict",
        InjectedCrash(_) => "plugin-busy",
        CorruptRegistry(_) | Storage(_) => "internal",
    };
    failure(
        code,
        "Plugin operation failed",
        serde_json::json!({"reason":error.to_string()}),
    )
}

fn plugin_mutation_result<T>(
    operation: &mut PluginEndpointOperation,
    result: Result<T, plugins::PluginError>,
) -> Result<T, ProductionRouteFailure> {
    match result {
        Ok(value) => Ok(value),
        Err(error) if plugin_error_is_precommit(&error) => {
            let public = map_plugin_error(error);
            Err(operation.fail(public)?)
        }
        Err(error) => Err(map_plugin_error(error)),
    }
}

fn plugin_error_is_precommit(error: &plugins::PluginError) -> bool {
    use plugins::PluginError::*;
    matches!(
        error,
        InvalidVersion(_)
            | InvalidManifest(_)
            | Incompatible(_)
            | InvalidArchive(_)
            | Signature(_)
            | PublisherTrustRequired(_)
            | NotInstalled(_)
            | DowngradeRejected { .. }
            | SameVersionChanged(_)
            | UndeclaredGrant(_)
            | MissingGrant(_)
            | ComponentCollision { .. }
    )
}
fn map_schedule_error(error: schedule::ScheduleError) -> ProductionRouteFailure {
    use schedule::ScheduleError::*;
    let code = match &error {
        OriginCollision => "idempotency-conflict",
        NotFound(_) => "schedule-not-found",
        Active(_) => "schedule-active",
        Invalid(_) | InvalidCron | InvalidTimeZone(_) | NoOccurrence => "schedule-invalid",
        ClaimMismatch | Corrupt(_) => "schedule-corrupt",
        Io(_) | Store(_) | Json(_) => "internal",
    };
    failure(
        code,
        "Schedule operation failed",
        serde_json::json!({"reason":error.to_string()}),
    )
}
fn map_search_error(error: thread_search::SearchError) -> ProductionRouteFailure {
    use thread_search::SearchError::*;
    let code = match &error {
        InvalidWorkspace => "invalid-workspace",
        InvalidQuery => "invalid-query",
        InvalidLimit => "invalid-limit",
        MalformedCursor => "malformed-cursor",
        CursorScope => "cursor-scope",
        CursorStale => "cursor-stale",
        CursorPosition => "cursor-position",
        SourceIdentity => "source-identity",
        SourceCorrupt(_) | IndexCorrupt(_) | Canonical(_) | Store(_) | Io(_) => "source-corrupt",
    };
    failure(
        code,
        "Thread search failed",
        serde_json::json!({"reason":error.to_string()}),
    )
}

impl ProductionClientExtensions {
    /// The directory picker browses the user's home, not the Kernel state root.
    fn picker_home(&self) -> PathBuf {
        std::env::var_os("HOME")
            .map(PathBuf::from)
            .filter(|home| home.is_absolute())
            .unwrap_or_else(|| self.root.clone())
    }

    /// `(folder, archived)` for a session that exists in either area.
    fn session_folder(&self, session_id: &str) -> Result<(PathBuf, bool), ProductionRouteFailure> {
        endpoint::validate_session_id(session_id)
            .map_err(|_| bad_request("invalid session identity"))?;
        [("threads", false), ("archive", true)]
            .into_iter()
            .map(|(area, archived)| (self.root.join(area).join(session_id), archived))
            .find(|(folder, _)| folder.is_dir())
            .ok_or_else(|| {
                failure(
                    "session-not-found",
                    "Session was not found",
                    serde_json::json!({"sessionId":session_id}),
                )
            })
    }

    fn session_controls(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let map = |error: session_controls::Failure| {
            failure(error.code(), &error.to_string(), error.details())
        };
        let text = |key: &str| payload.get(key).and_then(Value::as_str).unwrap_or_default();
        let value = if operation == "subagents.list" {
            let (folder, archived) = self.session_folder(text("parentSessionId"))?;
            let bytes = std::fs::read(folder.join("main.jsonl")).map_err(|error| {
                failure(
                    "ledger-corrupt",
                    "Session ledger is unreadable",
                    serde_json::json!({"reason":error.to_string()}),
                )
            })?;
            session_controls::subagent_catalog(&bytes, text("parentSessionId"), archived)
                .map_err(map)?
        } else {
            self.session_folder(text("sessionId"))?;
            // Production never binds a host goal id; the record owns its identity.
            let value = session_controls::execute_goal(&self.root, operation, payload, None)
                .map_err(map)?;
            if operation == "goals.get" {
                serde_json::json!({"goal": value})
            } else {
                value
            }
        };
        encode(&value)
    }

    fn host_files(
        &self,
        operation: &str,
        payload: &Value,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let map = |error: host_files::Failure| {
            failure(error.code(), error.message(), serde_json::json!({}))
        };
        let text = |key: &str| payload.get(key).and_then(Value::as_str).unwrap_or_default();
        let value = match operation {
            "directory.list" => host_files::list_directory(
                &self.picker_home(),
                payload.get("path").and_then(Value::as_str),
            )
            .map_err(map)?,
            "directory.create" => {
                let path = host_files::create_directory(text("path"), text("name")).map_err(map)?;
                serde_json::json!({"path": path})
            }
            "session.references.files" => {
                let roots = self
                    .process
                    .client_session_roots(text("sessionId"))
                    .map_err(|error| {
                        failure(
                            "session-not-found",
                            "Session was not found",
                            serde_json::json!({"reason":error.to_string()}),
                        )
                    })?;
                let items = host_files::file_references(&roots, text("query")).map_err(map)?;
                serde_json::json!({"items": items})
            }
            "session.references.sessions" => {
                let requesting = text("sessionId");
                let internal = |error: String| {
                    failure(
                        "internal",
                        "Session inventory is unavailable",
                        serde_json::json!({"reason":error}),
                    )
                };
                let native = endpoint::NativeEndpoint::open(&self.root)
                    .map_err(|error| internal(error.to_string()))?;
                let items = native
                    .list_sessions(&HashSet::new())
                    .map_err(|error| internal(error.to_string()))?;
                let management = endpoint::ManagementStore::open(&self.root)
                    .map_err(|error| internal(error.to_string()))?;
                let workspace = items
                    .iter()
                    .find(|item| item.session_id == requesting)
                    .map(|item| item.workspace_id.clone())
                    .ok_or_else(|| {
                        failure(
                            "session-not-found",
                            "Session was not found",
                            serde_json::json!({"sessionId":requesting}),
                        )
                    })?;
                let candidates = items
                    .iter()
                    .map(|item| host_files::SessionCandidate {
                        session_id: item.session_id.clone(),
                        title: item.title.clone(),
                        cwd: management
                            .workspace_path(&item.workspace_id)
                            .unwrap_or_default(),
                        workspace_id: item.workspace_id.clone(),
                        created_at_ms: item.updated_at as f64,
                        archived: item.archived,
                    })
                    .collect::<Vec<_>>();
                let items = host_files::session_references(
                    requesting,
                    &workspace,
                    &candidates,
                    text("query"),
                );
                serde_json::json!({"items": items})
            }
            other => return Err(bad_request(&format!("unknown host file method {other}"))),
        };
        encode(&value)
    }
}

impl ClientExtensionAuthority for ProductionClientExtensions {
    fn execute(
        &self,
        operation: &str,
        payload: &Value,
        principal: &str,
        rpc_id: &str,
        _recovering: bool,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        match operation {
            "session.initialPresets" => encode(&serde_json::json!({
                "presets": [
                    {"id":"coding", "title":"Coding", "configurationText":tools::CODING_PROFILE},
                    {"id":"general", "title":"General", "configurationText":tools::GENERAL_PROFILE}
                ],
                "defaultID": null
            })),
            "session.files.stat" | "session.files.read" | "session.files.readBytes" => {
                let request: FilePageRequest = decode(payload)?;
                let text = operation == "session.files.read";
                let mut page = self
                    .process
                    .client_file_page(
                        &request.session_id,
                        &request.path,
                        text,
                        request.offset.unwrap_or(if text { 1 } else { 0 }),
                        request.limit.unwrap_or(if text {
                            200
                        } else if operation == "session.files.stat" {
                            1
                        } else {
                            256 * 1024
                        }),
                    )
                    .map_err(|error| {
                        failure(
                            "file-unavailable",
                            "Session file is unavailable",
                            serde_json::json!({"reason":error.to_string()}),
                        )
                    })?;
                if operation == "session.files.stat" {
                    page.as_object_mut().unwrap().retain(|key, _| {
                        ["absolutePath", "version", "bytes"].contains(&key.as_str())
                    });
                }
                encode(&page)
            }
            "attachments.policy" => encode(&endpoint::AttachmentPolicy::default()),
            "session.uploadFile" => {
                let request: UploadFileRequest = decode(payload)?;
                let receipt = self
                    .attachments()
                    .upload_file(
                        &request.session_id,
                        &request.name,
                        request.media_type.as_deref(),
                        &request.data,
                    )
                    .map_err(|error| map_prompt_materialize_error(&request.session_id, error))?;
                encode(&receipt)
            }
            "session.fileAttachment" => {
                let request: FileAttachmentRequest = decode(payload)?;
                let attachment = self
                    .attachments()
                    .read_authorized_file(&request.session_id, &request.attachment_id)
                    .map_err(|error| map_attachment_read_error(&request.session_id, error))?;
                encode(&attachment)
            }
            "approvals.policy" => encode(&endpoint::ApprovalPolicy::default()),
            "approvals.mode" => {
                let request: ApprovalModeRequest = decode(payload)?;
                let (folder, _archived) = self.session_folder(&request.session_id)?;
                let read = engine::read_permission_mode(&folder);
                if let Some(diagnostic) = read.diagnostic {
                    eprintln!("tekes-supervisor: permission mode {diagnostic}");
                }
                encode(&serde_json::json!({"mode": read.mode.as_str()}))
            }
            "approvals.select" => {
                let request: ApprovalSelectRequest = decode(payload)?;
                let mode = engine::PermissionMode::parse(&request.mode)
                    .ok_or_else(|| bad_request("unknown permission mode"))?;
                let (folder, archived) = self.session_folder(&request.session_id)?;
                if archived {
                    return Err(failure(
                        "archived",
                        "Session is archived",
                        serde_json::json!({"sessionId": request.session_id}),
                    ));
                }
                engine::write_permission_mode(&folder, mode).map_err(|error| {
                    failure(
                        "internal",
                        "Permission mode could not be written",
                        serde_json::json!({"reason": error.to_string()}),
                    )
                })?;
                // The mode rides on the session's inventory summary: re-announce
                // the session so every subscriber's composer follows it.
                crate::endpoint_host::SessionDeliveryAuthority::session_metadata_changed(
                    self.process.as_ref(),
                    &request.session_id,
                );
                encode(&serde_json::json!({"mode": mode.as_str()}))
            }
            "directory.list"
            | "directory.create"
            | "session.references.files"
            | "session.references.sessions" => self.host_files(operation, payload),
            operation if session_controls::METHODS.contains(&operation) => {
                self.session_controls(operation, payload)
            }
            operation if user_documents::METHODS.contains(&operation) => {
                let value = user_documents::execute(&self.root, operation, payload)
                    .map_err(|error| failure(error.code, &error.message, error.details))?;
                encode(&value)
            }
            "sessions.recover" => {
                let request: RecoveryRequest = decode(payload)?;
                self.process
                    .recover_client_sessions(&request.session_ids)
                    .map_err(|error| {
                        failure(
                            "session-not-found",
                            "Session recovery failed",
                            serde_json::json!({"reason":error.to_string()}),
                        )
                    })?;
                encode(&serde_json::json!({"recoveredSessionIds":request.session_ids}))
            }
            "skills/list" | "commands/list" => {
                if let Some(session_id) = payload.get("sessionId").and_then(Value::as_str) {
                    let catalog =
                        self.process
                            .client_resource_catalog(session_id)
                            .map_err(|error| {
                                failure(
                                    "resource-not-found",
                                    "Session resources are unavailable",
                                    serde_json::json!({"reason":error.to_string()}),
                                )
                            })?;
                    if operation == "skills/list" {
                        encode(
                            &serde_json::json!({"skills": catalog.skill_summaries().iter().map(|skill| serde_json::json!({
                            "name":skill.name, "description":skill.description, "modelInvocable":true
                        })).collect::<Vec<_>>()}),
                        )
                    } else {
                        encode(
                            &serde_json::json!({"commands":catalog.command_summaries().iter().map(|command| serde_json::json!({
                            "name":command.name, "description":command.description, "argumentHint":command.argument_hint, "supportsAttachments":true
                        })).collect::<Vec<_>>()}),
                        )
                    }
                } else if operation == "skills/list" {
                    encode(&self.commands.skills_list())
                } else {
                    encode(&self.commands.commands_list())
                }
            }
            "commands/run" => {
                let input: CommandRunRequest = decode(payload)?;
                self.commands
                    .validate_session(&input.session_id)
                    .map_err(map_resource_failure)?;
                let catalog = self
                    .process
                    .client_resource_catalog(&input.session_id)
                    .map_err(|error| {
                        failure(
                            "invalid-command",
                            "Session command catalog is unavailable",
                            serde_json::json!({"reason":error.to_string()}),
                        )
                    })?;
                encode(
                    &self
                        .commands
                        .commands_run_with_catalog(&catalog, &input, principal)
                        .map_err(map_resource_failure)?,
                )
            }
            "resources/list" => self.resources_list(payload),
            "resources/read" => self.resources_read(payload),
            "tools/list" => {
                let request: ToolListRequest = decode(payload)?;
                self.tools(&request.workspace_id, request.session_id.as_deref(), None)
            }
            "tools/resolve" => {
                let request: ToolResolveRequest = decode(payload)?;
                self.tools(
                    &request.workspace_id,
                    request.session_id.as_deref(),
                    Some(&request.name),
                )
            }
            "plugin/list" => {
                let _: EmptyRequest = decode(payload)?;
                self.recover_plugin_endpoint_journal()?;
                let mut store = lock(self.plugin_store()?)?;
                let plugins = store.list().map_err(map_plugin_error)?;
                encode(
                    &serde_json::json!({"format":1,"plugins":plugins.iter().map(plugin_view).collect::<Vec<_>>() }),
                )
            }
            "plugin/get" => {
                let request: PluginIdRequest = decode(payload)?;
                self.recover_plugin_endpoint_journal()?;
                let mut store = lock(self.plugin_store()?)?;
                let plugin = store
                    .list()
                    .map_err(map_plugin_error)?
                    .into_iter()
                    .find(|item| item.plugin_id == request.plugin_id)
                    .ok_or_else(|| {
                        failure(
                            "plugin-not-found",
                            "Plugin was not found",
                            serde_json::json!({}),
                        )
                    })?;
                encode(&serde_json::json!({"format":1,"plugin":plugin_view(&plugin)}))
            }
            "plugin/inspect" => {
                let request: PluginInspectRequest = decode(payload)?;
                self.recover_plugin_endpoint_journal()?;
                let source =
                    PluginSource::from_path(&request.archive_path).map_err(map_plugin_error)?;
                let store = lock(self.plugin_store()?)?;
                let inspection = store.inspect(&source).map_err(map_plugin_error)?;
                encode(&serde_json::json!({
                    "format":1,"manifest":inspection.manifest,
                    "packageDigest":inspection.package_digest,"integrity":inspection.integrity,
                    "requestedCapabilities":inspection.requested_capabilities
                }))
            }
            "plugin/install" => {
                let request: PluginInstallRequest = decode(payload)?;
                let mut operation = self.plugin_operation(operation, payload, rpc_id)?;
                if let Some(result) = operation.completed()? {
                    return Ok(result);
                }
                let store = lock(self.plugin_store()?)?;
                if !operation.path.is_file() {
                    let source =
                        PluginSource::from_path(&request.archive_path).map_err(map_plugin_error)?;
                    let inspection = store
                        .freeze_source(&source, &operation.stage)
                        .map_err(map_plugin_error)?;
                    if inspection.package_digest != request.package_digest {
                        let _ = std::fs::remove_dir_all(&operation.stage);
                        return Err(failure(
                            "plugin-invalid",
                            "Plugin archive digest does not match",
                            serde_json::json!({"expected":request.package_digest,"actual":inspection.package_digest}),
                        ));
                    }
                    operation.prepare(None)?;
                }
                drop(store);
                self.complete_prepared_plugin_operation(&mut operation)?;
                operation.completed()?.ok_or_else(|| {
                    failure(
                        "internal",
                        "Plugin install recovery did not complete",
                        serde_json::json!({}),
                    )
                })
            }
            "plugin/setEnabled" => {
                let _: PluginEnabledRequest = decode(payload)?;
                let mut operation = self.plugin_operation(operation, payload, rpc_id)?;
                if let Some(result) = operation.completed()? {
                    return Ok(result);
                }
                if !operation.path.is_file() {
                    operation.prepare(None)?;
                }
                self.complete_prepared_plugin_operation(&mut operation)?;
                operation.completed()?.ok_or_else(|| {
                    failure(
                        "internal",
                        "Plugin enable recovery did not complete",
                        serde_json::json!({}),
                    )
                })
            }
            "plugin/setGrants" => {
                let _: PluginGrantsRequest = decode(payload)?;
                let mut operation = self.plugin_operation(operation, payload, rpc_id)?;
                if let Some(result) = operation.completed()? {
                    return Ok(result);
                }
                if !operation.path.is_file() {
                    operation.prepare(None)?;
                }
                self.complete_prepared_plugin_operation(&mut operation)?;
                operation.completed()?.ok_or_else(|| {
                    failure(
                        "internal",
                        "Plugin grants recovery did not complete",
                        serde_json::json!({}),
                    )
                })
            }
            "plugin/remove" => {
                let request: PluginIdRequest = decode(payload)?;
                let mut operation = self.plugin_operation(operation, payload, rpc_id)?;
                if let Some(result) = operation.completed()? {
                    return Ok(result);
                }
                let mut store = lock(self.plugin_store()?)?;
                if !operation.path.is_file() {
                    let present = store
                        .list()
                        .map_err(map_plugin_error)?
                        .iter()
                        .any(|plugin| plugin.plugin_id == request.plugin_id);
                    operation.prepare(Some(present))?;
                }
                drop(store);
                self.complete_prepared_plugin_operation(&mut operation)?;
                operation.completed()?.ok_or_else(|| {
                    failure(
                        "internal",
                        "Plugin removal recovery did not complete",
                        serde_json::json!({}),
                    )
                })
            }
            "plugin/components" => {
                let _: EmptyRequest = decode(payload)?;
                self.recover_plugin_endpoint_journal()?;
                let mut store = lock(self.plugin_store()?)?;
                let generations = store
                    .list()
                    .map_err(map_plugin_error)?
                    .into_iter()
                    .map(|receipt| (receipt.plugin_id, receipt.package_digest))
                    .collect::<std::collections::BTreeMap<_, _>>();
                let components = store.components().map_err(map_plugin_error)?;
                let mut views = Vec::new();
                for item in components {
                    let generation = generations.get(&item.owner_plugin_id).ok_or_else(|| {
                        failure(
                            "internal",
                            "Plugin component lost its owning generation",
                            serde_json::json!({}),
                        )
                    })?;
                    let launchable = store
                        .resolve_executable_component(&plugins::PluginComponentReference {
                            plugin_id: item.owner_plugin_id.clone(),
                            component_id: item.component_id.clone(),
                        })
                        .map_err(map_plugin_error)?
                        .is_some();
                    views.push(serde_json::json!({
                        "ownerPluginId":item.owner_plugin_id,"ownerVersion":item.owner_version,
                        "componentId":item.component_id,"componentKind":item.component_type,
                        "pluginGeneration":generation,"grantedCapabilities":item.granted_capabilities,
                        "launchable":launchable
                    }));
                }
                encode(&serde_json::json!({"format":1,"components":views}))
            }
            "schedule.list" => self.schedule_list(payload),
            "schedule.save" => self.schedule_save(payload, principal, rpc_id),
            "schedule.delete" => self.schedule_delete(payload, principal, rpc_id),
            "schedule.runNow" => self.schedule_run_now(payload, principal, rpc_id),
            "thread.search" => self.thread_search(payload),
            "session.search" => {
                let request: SessionSearchRequest = decode(payload)?;
                encode(
                    &self
                        .search
                        .search_sessions(&request.query)
                        .map_err(map_search_error)?,
                )
            }
            "usage.summary" => self.usage(payload, false),
            "usage.cacheAttribution" => self.usage(payload, true),
            _ => Err(failure(
                "unsupported-capability",
                "Capability is unavailable",
                serde_json::json!({"operation":operation}),
            )),
        }
    }
}

impl ProductionClientExtensions {
    fn resources_list(&self, payload: &Value) -> Result<IJsonValue, ProductionRouteFailure> {
        let request: ResourceListRequest = decode(payload)?;
        let limit = request.limit.unwrap_or(100);
        if !(1..=100).contains(&limit) {
            return Err(bad_request("limit must be 1...100"));
        }
        match request.source.as_str() {
            "skill" => {
                let catalog = self.workspace_resource_catalog(&request.workspace_id)?;
                let mut items = Vec::new();
                for package in catalog.skill_packages() {
                    for resource in &package.resources {
                        items.push(serde_json::json!({
                            "reference":{"kind":"skill","workspaceId":request.workspace_id,"name":package.summary.name,"contentDigest":package.summary.content_digest},
                            "uri":resource.path,
                            "name":resource.path,"mimeType":"text/markdown",
                            "description":package.summary.description
                        }));
                    }
                }
                items.sort_by(|left, right| left["uri"].as_str().cmp(&right["uri"].as_str()));
                let generation = format!(
                    "sha256-{:x}",
                    sha2::Sha256::digest(serde_json_canonicalizer::to_vec(&items).map_err(
                        |_| failure(
                            "internal",
                            "Skill resources cannot be canonicalized",
                            serde_json::json!({})
                        )
                    )?)
                );
                let start = match request.after {
                    None => 0,
                    Some(cursor) => decode_resource_cursor(&cursor, &generation)?,
                };
                if start > items.len() {
                    return Err(failure(
                        "resource-stale",
                        "Skill resource cursor is stale",
                        serde_json::json!({}),
                    ));
                }
                let end = start.saturating_add(limit).min(items.len());
                let next = (end < items.len()).then(|| format!("{generation}:{end}"));
                let mut value = serde_json::json!({"format":1,"items":items[start..end]});
                if let Some(next) = next {
                    value["next"] = Value::String(next);
                }
                encode(&value)
            }
            "mcp" => {
                let (generation, all) = self
                    .mcp
                    .list_resource_catalog(&request.workspace_id)
                    .map_err(map_mcp_resource_error)?;
                let start = match request.after {
                    None => 0,
                    Some(cursor) => decode_resource_cursor(&cursor, &generation)?,
                };
                if start > all.len() {
                    return Err(failure(
                        "resource-stale",
                        "MCP resource cursor is stale",
                        serde_json::json!({}),
                    ));
                }
                let end = start.saturating_add(limit).min(all.len());
                let items = all[start..end].iter().map(|item| {
                    let mut row = serde_json::json!({
                        "reference":{"kind":"mcp","server":item.server,"bindingDigest":item.binding_digest},
                        "uri":item.resource.uri,"name":item.resource.name
                    });
                    if let Some(mime) = &item.resource.mime_type {
                        row["mimeType"] = Value::String(mime.clone());
                    }
                    row
                }).collect::<Vec<_>>();
                let mut value = serde_json::json!({"format":1,"items":items});
                if end < all.len() {
                    value["next"] = Value::String(format!("{generation}:{end}"));
                }
                encode(&value)
            }
            _ => Err(bad_request("source is not skill or mcp")),
        }
    }

    fn resources_read(&self, payload: &Value) -> Result<IJsonValue, ProductionRouteFailure> {
        let request: ResourceReadRequest = decode(payload)?;
        match serde_json::from_value::<ResourceReference>(request.reference)
            .map_err(|_| bad_request("reference is invalid"))?
        {
            ResourceReference::Skill {
                workspace_id,
                name,
                content_digest,
            } => {
                if workspace_id != request.workspace_id {
                    return Err(failure(
                        "resource-not-found",
                        "Resource was not found",
                        serde_json::json!({}),
                    ));
                }
                let catalog = self.workspace_resource_catalog(&request.workspace_id)?;
                let package = catalog.skill(&name).map_err(|_| {
                    failure(
                        "resource-not-found",
                        "Resource was not found",
                        serde_json::json!({}),
                    )
                })?;
                if package.summary.content_digest != content_digest {
                    return Err(failure(
                        "resource-stale",
                        "Skill package generation changed",
                        serde_json::json!({}),
                    ));
                }
                let text = self
                    .workspace_resource_catalog(&request.workspace_id)?
                    .skill_resource(&name, &request.uri)
                    .map_err(|_| {
                        failure(
                            "resource-not-found",
                            "Resource was not found",
                            serde_json::json!({}),
                        )
                    })?;
                if text.len() > 4 * 1024 * 1024 {
                    return Err(failure(
                        "resource-too-large",
                        "Resource exceeds 4 MiB",
                        serde_json::json!({}),
                    ));
                }
                encode(
                    &serde_json::json!({"format":1,"content":{"text":text,"mimeType":"text/markdown"}}),
                )
            }
            ResourceReference::Mcp {
                server,
                binding_digest,
            } => {
                if server.workspace_id != request.workspace_id {
                    return Err(failure(
                        "resource-not-found",
                        "Resource was not found",
                        serde_json::json!({}),
                    ));
                }
                let raw = self
                    .mcp
                    .read_resource(&server, &binding_digest, &request.uri)
                    .map_err(map_mcp_resource_error)?;
                let raw = ijson_value(&raw)?;
                let content = normalize_mcp_resource(&raw)?;
                encode(&serde_json::json!({"format":1,"content":content}))
            }
        }
    }

    fn tools(
        &self,
        workspace_id: &str,
        session_id: Option<&str>,
        resolve: Option<&str>,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let dynamic = self
            .process
            .client_tool_catalog(workspace_id, session_id)
            .map_err(|_| {
                failure(
                    "internal",
                    "Tool catalog is unavailable",
                    serde_json::json!({}),
                )
            })?;
        let mut rows = Vec::new();
        let fixed = BuiltinManifest::compiled();
        for tool in &fixed.tools {
            let schema = fixed_schema(&tool.name).ok_or_else(|| {
                failure(
                    "internal",
                    "Compiled tool schema is absent",
                    serde_json::json!({}),
                )
            })?;
            rows.push(serde_json::json!({
                "name":tool.name,"description":schema.description,
                "arguments":schema.parameters.to_json_schema(),"availability":tool.availability,
                "backend":tool.backend,"effect":tool.effect,"source":{"kind":"builtin"}
            }));
        }
        for tool in dynamic.tools {
            let schema = ijson_value(&tool.schema)?;
            rows.push(serde_json::json!({
                "name":tool.name,"description":schema.get("description").and_then(Value::as_str).unwrap_or(""),
                "arguments":schema.get("parameters").cloned().unwrap_or_else(|| serde_json::json!({})),
                "availability":if tool.always_on {"always"} else {"deferred"},
                "backend":"supervisor_control","effect":tool.effect,"source":tool.source
            }));
        }
        rows.sort_by(|left, right| left["name"].as_str().cmp(&right["name"].as_str()));
        let digest = format!(
            "sha256-{:x}",
            sha2::Sha256::digest(serde_json_canonicalizer::to_vec(&rows).map_err(|_| failure(
                "internal",
                "Tool catalog cannot be canonicalized",
                serde_json::json!({})
            ))?)
        );
        if let Some(name) = resolve {
            let tool = rows
                .into_iter()
                .find(|row| row["name"] == name)
                .ok_or_else(|| {
                    failure(
                        "tool-not-found",
                        "Tool was not found",
                        serde_json::json!({"name":name}),
                    )
                })?;
            encode(&serde_json::json!({"format":1,"catalogDigest":digest,"tool":tool}))
        } else {
            encode(&serde_json::json!({"format":1,"catalogDigest":digest,"tools":rows}))
        }
    }

    fn plugin_mutation_value(&self, plugin: PluginReceipt) -> Value {
        let readiness = self.plugin_readiness();
        serde_json::json!({"format":1,"plugin":plugin_view(&plugin),"readiness":readiness})
    }

    fn plugin_readiness(&self) -> Value {
        match self.mcp.reconcile_authority() {
            Ok(_) => serde_json::json!({"state":"ready"}),
            Err(error) => {
                serde_json::json!({"state":"refresh-failed","code":mcp_readiness_code(&error)})
            }
        }
    }

    fn schedule_list(&self, payload: &Value) -> Result<IJsonValue, ProductionRouteFailure> {
        let request: OptionalWorkspace = decode(payload)?;
        encode(
            &serde_json::json!({"format":1,"schedules":self.process.schedule_authority().list(request.workspace_id.as_deref()).map_err(map_schedule_error)?}),
        )
    }

    fn schedule_save(
        &self,
        payload: &Value,
        principal: &str,
        rpc_id: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let request: ScheduleSaveRequest = decode(payload)?;
        if request.definition.permission_mode != "inherit" {
            return Err(failure(
                "schedule-policy",
                "Unsupported schedule permission policy",
                serde_json::json!({}),
            ));
        }
        if request.definition.model_id.is_some() {
            return Err(failure(
                "schedule-model",
                "Bare schedule model id is ambiguous",
                serde_json::json!({}),
            ));
        }
        self.workspace_snapshot(&request.definition.workspace_id)?;
        let schedule = self
            .process
            .schedule_authority()
            .save(
                schedule::OriginTuple {
                    client_id: principal.to_owned(),
                    key: rpc_id.to_owned(),
                },
                request.definition,
                Utc::now(),
            )
            .map_err(map_schedule_error)?;
        if schedule.definition.enabled {
            self.process.start_schedule_timer().map_err(|_| {
                failure(
                    "internal",
                    "Schedule timer is unavailable",
                    serde_json::json!({}),
                )
            })?;
        }
        encode(&serde_json::json!({"format":1,"schedule":schedule}))
    }

    fn schedule_delete(
        &self,
        payload: &Value,
        principal: &str,
        rpc_id: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let request: TaskIdRequest = decode(payload)?;
        let deleted = self
            .process
            .schedule_authority()
            .delete(
                schedule::OriginTuple {
                    client_id: principal.to_owned(),
                    key: rpc_id.to_owned(),
                },
                &request.task_id,
                Utc::now(),
            )
            .map_err(map_schedule_error)?;
        encode(&serde_json::json!({"format":1,"deleted":deleted}))
    }

    fn schedule_run_now(
        &self,
        payload: &Value,
        principal: &str,
        rpc_id: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let request: TaskIdRequest = decode(payload)?;
        let claim = self
            .process
            .schedule_authority()
            .run_now(
                schedule::OriginTuple {
                    client_id: principal.to_owned(),
                    key: rpc_id.to_owned(),
                },
                &request.task_id,
                Utc::now(),
            )
            .map_err(map_schedule_error)?;
        self.process.start_schedule_timer().map_err(|_| {
            failure(
                "internal",
                "Schedule timer is unavailable",
                serde_json::json!({}),
            )
        })?;
        encode(&serde_json::json!({"format":1,"claim":claim}))
    }

    fn thread_search(&self, payload: &Value) -> Result<IJsonValue, ProductionRouteFailure> {
        let request: ThreadSearchRequest = decode(payload)?;
        let page = self
            .search
            .search(&SearchRequest {
                workspace_id: request.workspace_id,
                query: request.query,
                limit: request.limit,
                visibility: request.visibility,
                after: request.after,
            })
            .map_err(map_search_error)?;
        let mut result = serde_json::json!({"format":1,"results":page.results,"reachedEnd":page.reached_end,"catalogDigest":page.catalog_digest});
        if let Some(next) = page.next_cursor {
            result["nextCursor"] = Value::String(next);
        }
        encode(&result)
    }

    fn usage(&self, payload: &Value, cache: bool) -> Result<IJsonValue, ProductionRouteFailure> {
        let request: SessionIdRequest = decode(payload)?;
        let folder = ["threads", "archive"]
            .into_iter()
            .map(|area| self.root.join(area).join(&request.session_id))
            .find(|path| path.is_dir())
            .ok_or_else(|| {
                failure(
                    "session-not-found",
                    "Session was not found",
                    serde_json::json!({}),
                )
            })?;
        let bytes = std::fs::read(folder.join("main.jsonl")).map_err(|_| {
            failure(
                "source-corrupt",
                "Usage source is unreadable",
                serde_json::json!({}),
            )
        })?;
        let assets = store::AssetStore::new(folder.join("assets")).map_err(|_| {
            failure(
                "source-corrupt",
                "Usage assets are corrupt",
                serde_json::json!({}),
            )
        })?;
        let scan = store::scan_valid_prefix(&bytes, 1);
        if scan.needs_repair() {
            return Err(failure(
                "source-corrupt",
                "Usage source has an invalid tail",
                serde_json::json!({}),
            ));
        }
        let projection = scan.projection.ok_or_else(|| {
            failure(
                "source-corrupt",
                "Usage source has no projection",
                serde_json::json!({}),
            )
        })?;
        let mut epoch_targets = BTreeMap::<String, Value>::new();
        let mut attempt_epochs = BTreeMap::<String, String>::new();
        if cache {
            for event in &projection.events {
                match event.kind() {
                    EventKind::Epoch => {
                        let raw = ijson_value(event.raw())?;
                        let id = event.string_field("id").ok_or_else(|| {
                            failure(
                                "source-corrupt",
                                "Epoch identity is absent",
                                serde_json::json!({}),
                            )
                        })?;
                        let target = usage_route_target(&raw, &assets)?;
                        if epoch_targets.insert(id.to_owned(), target).is_some() {
                            return Err(failure(
                                "source-corrupt",
                                "Epoch identity is duplicated",
                                serde_json::json!({}),
                            ));
                        }
                    }
                    EventKind::Attempt => {
                        let attempt = event.string_field("attempt").ok_or_else(|| {
                            failure(
                                "source-corrupt",
                                "Attempt identity is absent",
                                serde_json::json!({}),
                            )
                        })?;
                        let epoch = event.string_field("epoch").ok_or_else(|| {
                            failure(
                                "source-corrupt",
                                "Attempt epoch is absent",
                                serde_json::json!({}),
                            )
                        })?;
                        if attempt_epochs
                            .insert(attempt.to_owned(), epoch.to_owned())
                            .is_some()
                        {
                            return Err(failure(
                                "source-corrupt",
                                "Attempt identity is duplicated",
                                serde_json::json!({}),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        let mut input = 0u64;
        let mut output = 0u64;
        let mut groups = BTreeMap::<Vec<u8>, CacheAttributionGroup>::new();
        for event in &projection.events {
            // The settling output/error carries its attempt's usage.
            let settles_attempt = matches!(event.kind(), EventKind::Output)
                || matches!(event.kind(), EventKind::Error) && event.has_field("attempt");
            if !settles_attempt {
                continue;
            }
            let outcome = ijson_value(event.raw())?;
            let raw = outcome.get("usage").cloned().unwrap_or(Value::Null);
            let availability = raw
                .get("availability")
                .and_then(Value::as_str)
                .unwrap_or("unavailable");
            let parsed_input = numeric_token(raw.get("input_tokens"))?;
            let parsed_output = numeric_token(raw.get("output_tokens"))?;
            input = input.checked_add(parsed_input).ok_or_else(|| {
                failure(
                    "source-corrupt",
                    "Usage total overflowed",
                    serde_json::json!({}),
                )
            })?;
            output = output.checked_add(parsed_output).ok_or_else(|| {
                failure(
                    "source-corrupt",
                    "Usage total overflowed",
                    serde_json::json!({}),
                )
            })?;
            if cache && availability == "reported" {
                let (Some(input_tokens), Some(output_tokens), Some(cache_read_tokens)) = (
                    raw.get("input_tokens").and_then(Value::as_str),
                    raw.get("output_tokens").and_then(Value::as_str),
                    raw.get("cache_read").and_then(Value::as_str),
                ) else {
                    continue;
                };
                canonical_unsigned(input_tokens)?;
                canonical_unsigned(output_tokens)?;
                canonical_unsigned(cache_read_tokens)?;
                let attempt = event.string_field("attempt").ok_or_else(|| {
                    failure(
                        "source-corrupt",
                        "Usage attempt is absent",
                        serde_json::json!({}),
                    )
                })?;
                let epoch = attempt_epochs.get(attempt).ok_or_else(|| {
                    failure(
                        "source-corrupt",
                        "Usage attempt has no epoch binding",
                        serde_json::json!({}),
                    )
                })?;
                let target = epoch_targets.get(epoch).ok_or_else(|| {
                    failure(
                        "source-corrupt",
                        "Usage epoch has no exact target",
                        serde_json::json!({}),
                    )
                })?;
                let key = serde_json_canonicalizer::to_vec(target).map_err(|_| {
                    failure(
                        "source-corrupt",
                        "Usage target cannot be canonicalized",
                        serde_json::json!({}),
                    )
                })?;
                groups
                    .entry(key)
                    .or_insert_with(|| CacheAttributionGroup::new(target.clone()))
                    .add(
                        attempt,
                        input_tokens,
                        output_tokens,
                        cache_read_tokens,
                        raw.get("cost").and_then(Value::as_str),
                    )?;
            }
        }
        if cache {
            let entries = groups
                .into_values()
                .map(CacheAttributionGroup::value)
                .collect::<Result<Vec<_>, _>>()?;
            encode(
                &serde_json::json!({"format":1,"sessionId":request.session_id,"asOfSeq":projection.last_seq,"entries":entries}),
            )
        } else {
            let total = input.checked_add(output).ok_or_else(|| {
                failure(
                    "source-corrupt",
                    "Usage total overflowed",
                    serde_json::json!({}),
                )
            })?;
            if [input, output, total]
                .into_iter()
                .any(|value| value > 9_007_199_254_740_991)
            {
                return Err(failure(
                    "source-corrupt",
                    "Usage total exceeds the I-JSON safe integer range",
                    serde_json::json!({}),
                ));
            }
            encode(
                &serde_json::json!({"format":1,"sessionId":request.session_id,"asOfSeq":projection.last_seq,"inputTokens":input,"outputTokens":output,"totalTokens":total}),
            )
        }
    }
}

/// Mirrors `session.attachment` in `endpoint_host`: lifecycle codes keep the
/// session identity, validation is the closed `attachment-error {reason}`,
/// and storage defects fold to `internal`.
fn map_attachment_read_error(
    session_id: &str,
    error: endpoint::AttachmentReadError,
) -> ProductionRouteFailure {
    match error {
        endpoint::AttachmentReadError::SessionNotFound(_) => failure(
            "session-not-found",
            "Session was not found",
            serde_json::json!({"sessionId": session_id}),
        ),
        endpoint::AttachmentReadError::Archived(_) => failure(
            "archived",
            "Session is archived",
            serde_json::json!({"sessionId": session_id}),
        ),
        endpoint::AttachmentReadError::Attachment(reason) => failure(
            "attachment-error",
            "Image attachment is unavailable",
            serde_json::json!({"reason": reason}),
        ),
        _ => failure(
            "internal",
            "Endpoint operation failed",
            serde_json::json!({}),
        ),
    }
}

fn error_allowed(operation: &str, class: Option<MethodClass>, code: &str) -> bool {
    if matches!(code, "bad-request" | "unsupported-capability" | "internal")
        || (class == Some(MethodClass::Mutation) && code == "idempotency-conflict")
    {
        return true;
    }
    match operation {
        "session.files.stat" | "session.files.read" | "session.files.readBytes" => {
            code == "file-unavailable"
        }
        "sessions.recover" => code == "session-not-found",
        "approvals.mode" | "approvals.select" => matches!(code, "session-not-found" | "archived"),
        "session.uploadFile" | "session.fileAttachment" => {
            matches!(code, "attachment-error" | "session-not-found" | "archived")
        }
        "directory.list" | "directory.create" => matches!(
            code,
            "directory-not-found" | "directory-exists" | "directory-forbidden" | "io-error"
        ),
        "session.references.files" | "session.references.sessions" => {
            matches!(code, "session-not-found" | "io-error")
        }
        value if value.starts_with("feedback.") => matches!(code, "version-conflict" | "io"),
        value if value.starts_with("settings.") => matches!(
            code,
            "unknown-namespace" | "stale-revision" | "settings-invalid" | "io"
        ),
        value if value.starts_with("goals.") => matches!(
            code,
            "goal-stale"
                | "goal-not-found"
                | "goal-invalid-transition"
                | "session-not-found"
                | "ledger-corrupt"
        ),
        "subagents.list" => matches!(code, "session-not-found" | "ledger-corrupt"),
        "skills/list" | "commands/list" => code == "resource-not-found",
        "commands/run" => matches!(
            code,
            "invalid-command"
                | "invalid-arguments"
                | "command-not-found"
                | "session-not-found"
                | "session-archived"
                | "session-busy"
                | "delivery-timeout"
                | "delivery-rejected"
                | "attachment-error"
        ),
        "resources/list" | "resources/read" => matches!(
            code,
            "resource-not-found" | "resource-stale" | "resource-too-large"
        ),
        "tools/list" | "tools/resolve" => code == "tool-not-found",
        value if value.starts_with("plugin/") => matches!(
            code,
            "plugin-not-found"
                | "plugin-invalid"
                | "plugin-untrusted"
                | "plugin-grants"
                | "plugin-collision"
                | "plugin-version-conflict"
                | "plugin-busy"
        ),
        value if value.starts_with("schedule.") => matches!(
            code,
            "schedule-invalid"
                | "schedule-not-found"
                | "schedule-active"
                | "schedule-policy"
                | "schedule-model"
                | "schedule-corrupt"
        ),
        "thread.search" | "session.search" => matches!(
            code,
            "invalid-workspace"
                | "invalid-query"
                | "invalid-limit"
                | "malformed-cursor"
                | "cursor-scope"
                | "cursor-stale"
                | "cursor-position"
                | "source-identity"
                | "source-corrupt"
        ),
        "usage.summary" | "usage.cacheAttribution" => {
            matches!(code, "session-not-found" | "source-corrupt")
        }
        _ => false,
    }
}

fn validate_payload(operation: &str, payload: &Value) -> Result<(), ProductionRouteFailure> {
    match operation {
        "session.initialPresets" => {
            if payload.as_object().is_some_and(serde_json::Map::is_empty) {
                Ok(())
            } else {
                Err(bad_request("request must be empty"))
            }
        }
        "session.files.stat" | "session.files.read" | "session.files.readBytes" => {
            let request: FilePageRequest = decode(payload)?;
            endpoint::validate_session_id(&request.session_id)
                .map_err(|_| bad_request("Invalid session identity"))?;
            if request.path.is_empty()
                || request.path.contains('\0')
                || request.limit == Some(0)
                || (operation == "session.files.read" && request.offset == Some(0))
                || (operation == "session.files.stat"
                    && (request.offset.is_some() || request.limit.is_some()))
                || request.limit.is_some_and(|limit| {
                    limit
                        > if operation == "session.files.read" {
                            10000
                        } else {
                            4 * 1024 * 1024
                        }
                })
            {
                return Err(bad_request("Invalid file range"));
            }
            Ok(())
        }
        operation if session_controls::METHODS.contains(&operation) => {
            session_controls::validate(operation, payload).map_err(|reason| bad_request(&reason))
        }
        operation if user_documents::METHODS.contains(&operation) => {
            user_documents::validate(operation, payload).map_err(|reason| bad_request(&reason))
        }
        operation if host_files::METHODS.contains(&operation) => {
            host_files::validate(operation, payload).map_err(|reason| bad_request(&reason))
        }
        "attachments.policy" | "approvals.policy" => {
            let request: AttachmentPolicyRequest = decode(payload)?;
            if let Some(id) = request.session_id {
                endpoint::validate_session_id(&id)
                    .map_err(|_| bad_request("Invalid session identity"))?;
            }
            Ok(())
        }
        "approvals.mode" => {
            let request: ApprovalModeRequest = decode(payload)?;
            endpoint::validate_session_id(&request.session_id)
                .map_err(|_| bad_request("Invalid session identity"))?;
            Ok(())
        }
        "approvals.select" => {
            let request: ApprovalSelectRequest = decode(payload)?;
            endpoint::validate_session_id(&request.session_id)
                .map_err(|_| bad_request("Invalid session identity"))?;
            if engine::PermissionMode::parse(&request.mode).is_none() {
                return Err(bad_request("unknown permission mode"));
            }
            Ok(())
        }
        "session.uploadFile" => {
            let request: UploadFileRequest = decode(payload)?;
            endpoint::validate_session_id(&request.session_id)
                .map_err(|_| bad_request("Invalid session identity"))?;
            if request.name.is_empty() || request.data.is_empty() {
                return Err(bad_request("Invalid upload request"));
            }
            Ok(())
        }
        "session.fileAttachment" => {
            let request: FileAttachmentRequest = decode(payload)?;
            endpoint::validate_session_id(&request.session_id)
                .map_err(|_| bad_request("Invalid session identity"))?;
            if request.attachment_id.is_empty() {
                return Err(bad_request("Invalid attachment identity"));
            }
            Ok(())
        }
        "sessions.recover" => decode::<RecoveryRequest>(payload).map(|_| ()),
        "skills/list" | "commands/list" => {
            let object = payload
                .as_object()
                .ok_or_else(|| bad_request("object required"))?;
            if object.is_empty()
                || (object.len() == 1
                    && object
                        .get("sessionId")
                        .and_then(Value::as_str)
                        .is_some_and(|id| !id.is_empty()))
            {
                Ok(())
            } else {
                Err(bad_request("request must be empty"))
            }
        }
        "commands/run" => decode::<CommandRunRequest>(payload).map(|_| ()),
        "resources/list" => decode::<ResourceListRequest>(payload).map(|_| ()),
        "resources/read" => decode::<ResourceReadRequest>(payload).map(|_| ()),
        "tools/list" => decode::<ToolListRequest>(payload).map(|_| ()),
        "tools/resolve" => {
            let request = decode::<ToolResolveRequest>(payload)?;
            if !request.name.is_empty() {
                Ok(())
            } else {
                Err(bad_request("tools/resolve requires name"))
            }
        }
        "plugin/list" | "plugin/components" => decode::<EmptyRequest>(payload).map(|_| ()),
        "plugin/get" | "plugin/remove" => decode::<PluginIdRequest>(payload).map(|_| ()),
        "plugin/inspect" => decode::<PluginInspectRequest>(payload).map(|_| ()),
        "plugin/install" => decode::<PluginInstallRequest>(payload).map(|_| ()),
        "plugin/setEnabled" => decode::<PluginEnabledRequest>(payload).map(|_| ()),
        "plugin/setGrants" => decode::<PluginGrantsRequest>(payload).map(|_| ()),
        "schedule.list" => decode::<OptionalWorkspace>(payload).map(|_| ()),
        "schedule.save" => decode::<ScheduleSaveRequest>(payload).map(|_| ()),
        "schedule.delete" | "schedule.runNow" => decode::<TaskIdRequest>(payload).map(|_| ()),
        "thread.search" => decode::<ThreadSearchRequest>(payload).map(|_| ()),
        "session.search" => decode::<SessionSearchRequest>(payload).map(|_| ()),
        "usage.summary" | "usage.cacheAttribution" => {
            decode::<SessionIdRequest>(payload).map(|_| ())
        }
        _ => Err(failure(
            "unsupported-capability",
            "Capability is unavailable",
            serde_json::json!({"operation":operation}),
        )),
    }
}

pub(crate) fn failure(code: &str, message: &str, details: Value) -> ProductionRouteFailure {
    let details = IJsonValue::parse(
        &serde_json::to_vec(&details).expect("extension failure details are JSON"),
    )
    .expect("extension failure details are I-JSON");
    ProductionRouteFailure::new(code, message, details)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use endpoint::{EndpointHost, SessionHostDescription};

    use crate::endpoint_host::{SessionDeliveryAuthority, SessionInputAdmissionAuthority};
    use crate::host_runtime::assemble_production_endpoint_host;

    use super::*;

    fn production_extensions(
        root: &Path,
    ) -> (Arc<ProductionClientExtensions>, Arc<ProductionProcessHost>) {
        let agent = root.join(".agent");
        fs::create_dir_all(&agent).expect("agent root");
        let process = ProductionProcessHost::open(
            root,
            std::env::current_exe().expect("test executable"),
            "1.0.0",
            &agent,
        )
        .expect("process host");
        let admission = Arc::new(SessionInputAdmissionAuthority::new(root.to_path_buf()));
        let commands = EndpointCommandInputAuthority::new(
            Arc::clone(&process) as Arc<dyn SessionDeliveryAuthority>,
            Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
            admission,
        );
        let extensions = Arc::new(
            ProductionClientExtensions::open(
                root,
                ResourceCatalog::default(),
                BTreeMap::new(),
                commands,
                Arc::clone(&process),
            )
            .expect("extensions"),
        );
        (extensions, process)
    }

    #[test]
    fn initial_presets_disclose_source_text_without_creating_a_session() {
        let root = tempfile::tempdir().expect("root");
        let (extensions, _process) = production_extensions(root.path());
        let catalog = execute(
            &extensions,
            "session.initialPresets",
            &serde_json::json!({}),
            "presets",
        )
        .expect("catalog");
        assert_eq!(catalog["defaultID"], Value::Null);
        assert_eq!(catalog["presets"][0]["id"], "coding");
        assert_eq!(
            catalog["presets"][0]["configurationText"],
            tools::CODING_PROFILE
        );
        assert_eq!(
            catalog["presets"][1]["configurationText"],
            tools::GENERAL_PROFILE
        );
        assert!(
            catalog["presets"][0]["configurationText"]
                .as_str()
                .unwrap()
                .contains("{model}")
        );
        assert!(
            !root.path().join("threads").exists()
                || fs::read_dir(root.path().join("threads"))
                    .unwrap()
                    .next()
                    .is_none()
        );
        assert!(
            validate_payload(
                "session.initialPresets",
                &serde_json::json!({"sessionId":"x"})
            )
            .is_err()
        );
    }

    fn write_plugin(root: &Path, version: &str, capabilities: &[&str]) -> PathBuf {
        let package = root.join(format!("package-{version}"));
        fs::create_dir_all(&package).expect("package");
        let operating_system = if cfg!(target_os = "macos") {
            "macos"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            "linux"
        };
        let architecture = if cfg!(target_os = "macos") && std::env::consts::ARCH == "aarch64" {
            "arm64"
        } else {
            std::env::consts::ARCH
        };
        let capabilities = capabilities
            .iter()
            .map(|id| serde_json::json!({"id":id}))
            .collect::<Vec<_>>();
        fs::create_dir(package.join("assets")).expect("assets component");
        fs::write(
            package.join("tekes-plugin.json"),
            serde_json::to_vec(&serde_json::json!({
                "manifestVersion":1,
                "id":"com.example.journal",
                "version":version,
                "displayName":"Journal fixture",
                "platforms":[{"os":operating_system,"architectures":[architecture]}],
                "capabilities":capabilities,
                "components":[{"id":"assets","type":"assets","path":"assets","capabilities":[]}]
            }))
            .expect("manifest JSON"),
        )
        .expect("manifest");
        package
    }

    fn package_digest(extensions: &ProductionClientExtensions, package: &Path) -> String {
        let source = PluginSource::from_path(package).expect("source");
        lock(extensions.plugin_store().expect("plugin store"))
            .expect("lock")
            .inspect(&source)
            .expect("inspect")
            .package_digest
    }

    fn install_payload(package: &Path, digest: &str, allow_replace: bool) -> Value {
        serde_json::json!({
            "archivePath":package.to_string_lossy(),
            "packageDigest":digest,
            "enable":false,
            "grants":[],
            "allowDowngrade":false,
            "allowSameVersionReplacement":allow_replace,
        })
    }

    fn execute(
        extensions: &ProductionClientExtensions,
        operation: &str,
        payload: &Value,
        rpc_id: &str,
    ) -> Result<Value, ProductionRouteFailure> {
        let result = ClientExtensionAuthority::execute(
            extensions,
            operation,
            payload,
            "test-client",
            rpc_id,
            false,
        )?;
        ijson_value(&result)
    }

    /// `approvals.select` durably writes the per-session mode record,
    /// `approvals.mode` reads it back (absent = workspace-write), the option
    /// ids published by `approvals.policy` are exactly the engine's modes,
    /// an unknown mode is `bad-request`, and an archived session is refused.
    #[test]
    fn approvals_select_persists_the_session_mode_and_refuses_archived_sessions() {
        let root = tempfile::tempdir().expect("root");
        let (extensions, _process) = production_extensions(root.path());
        let session = "018f0000-0000-7000-8000-000000000031";
        let archived = "018f0000-0000-7000-8000-000000000032";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(&folder).expect("thread folder");
        fs::create_dir_all(root.path().join("archive").join(archived)).expect("archive folder");

        let policy = execute(
            &extensions,
            "approvals.policy",
            &serde_json::json!({}),
            "policy",
        )
        .expect("policy");
        let published: Vec<&str> = policy["threadLevel"]
            .as_array()
            .expect("thread options")
            .iter()
            .map(|option| option["value"].as_str().expect("value"))
            .collect();
        assert_eq!(
            published,
            engine::PermissionMode::ALL.map(engine::PermissionMode::as_str)
        );
        assert_eq!(
            policy["serverLevel"],
            serde_json::json!({"options":[],"currentValue":null})
        );

        let absent = execute(
            &extensions,
            "approvals.mode",
            &serde_json::json!({"sessionId":session}),
            "mode-0",
        )
        .expect("mode");
        assert_eq!(absent, serde_json::json!({"mode":"workspace-write"}));

        let selected = execute(
            &extensions,
            "approvals.select",
            &serde_json::json!({"sessionId":session,"mode":"read-only"}),
            "select-1",
        )
        .expect("select");
        assert_eq!(selected, serde_json::json!({"mode":"read-only"}));
        assert_eq!(
            fs::read(folder.join("permission-mode.json")).expect("record"),
            b"{\"format\":1,\"mode\":\"read-only\"}\n"
        );
        let read = execute(
            &extensions,
            "approvals.mode",
            &serde_json::json!({"sessionId":session}),
            "mode-1",
        )
        .expect("mode");
        assert_eq!(read, serde_json::json!({"mode":"read-only"}));

        let unknown = execute(
            &extensions,
            "approvals.select",
            &serde_json::json!({"sessionId":session,"mode":"yolo"}),
            "select-2",
        )
        .expect_err("unknown mode");
        assert_eq!(unknown.code, "bad-request");
        let refused = execute(
            &extensions,
            "approvals.select",
            &serde_json::json!({"sessionId":archived,"mode":"read-only"}),
            "select-3",
        )
        .expect_err("archived");
        assert_eq!(refused.code, "archived");
        assert!(
            !root
                .path()
                .join("archive")
                .join(archived)
                .join("permission-mode.json")
                .exists()
        );
        let missing = execute(
            &extensions,
            "approvals.select",
            &serde_json::json!({"sessionId":"018f0000-0000-7000-8000-000000000033","mode":"read-only"}),
            "select-4",
        )
        .expect_err("missing");
        assert_eq!(missing.code, "session-not-found");
        for (operation, class, code) in [
            ("approvals.select", MethodClass::Mutation, "archived"),
            ("approvals.mode", MethodClass::ReadOnly, "session-not-found"),
        ] {
            assert!(
                error_allowed(operation, Some(class), code),
                "{operation} {code}"
            );
        }
    }

    #[test]
    fn tools_list_is_closed_and_includes_every_fixed_tool() {
        assert!(
            validate_payload(
                "tools/list",
                &serde_json::json!({"workspaceId":"ws","name":"web_search"}),
            )
            .is_err(),
            "the list DTO must reject resolve-only fields"
        );
        let root = tempfile::tempdir().expect("root");
        let workspace = serde_json::json!({
            "format":1,"id":"ws","name":"ws","revision":1,
            "cwd":[root.path().to_string_lossy()],
            "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
        });
        let mut workspace_bytes =
            serde_json_canonicalizer::to_vec(&workspace).expect("workspace JSON");
        workspace_bytes.push(b'\n');
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        fs::write(
            root.path().join("workspaces/ws/workspace.json"),
            workspace_bytes,
        )
        .expect("workspace config");
        let (extensions, _) = production_extensions(root.path());
        let result = execute(
            &extensions,
            "tools/list",
            &serde_json::json!({"workspaceId":"ws"}),
            "rpc-tools-list",
        )
        .expect("tool list");
        let tools = result["tools"].as_array().expect("tools");
        let fixed = BuiltinManifest::compiled();
        for expected in &fixed.tools {
            assert!(tools.iter().any(|tool| tool["name"] == expected.name));
        }
        assert_eq!(fixed.tools.len(), 25);

        let resolved = execute(
            &extensions,
            "tools/resolve",
            &serde_json::json!({"workspaceId":"ws","name":"web_search"}),
            "rpc-tools-resolve",
        )
        .expect("conditional fixed tool resolves for introspection");
        assert_eq!(resolved["tool"]["name"], "web_search");
    }

    #[test]
    fn schedule_save_accepts_a_valid_multi_root_workspace() {
        let root = tempfile::tempdir().expect("root");
        let first = root.path().join("first");
        let second = root.path().join("second");
        fs::create_dir_all(&first).expect("first root");
        fs::create_dir_all(&second).expect("second root");
        fs::create_dir_all(root.path().join("workspaces/multi")).expect("workspaces");
        let workspace = serde_json::json!({
            "format":1,"id":"multi","name":"multi","revision":1,
            "cwd":[first.to_string_lossy(),second.to_string_lossy()],
            "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
        });
        let mut workspace_bytes =
            serde_json_canonicalizer::to_vec(&workspace).expect("workspace JSON");
        workspace_bytes.push(b'\n');
        fs::write(
            root.path().join("workspaces/multi/workspace.json"),
            workspace_bytes,
        )
        .expect("workspace config");

        let (extensions, process) = production_extensions(root.path());
        let result = execute(
            &extensions,
            "schedule.save",
            &serde_json::json!({"definition":{
                "id":"018f0000-0000-7000-8000-000000000011",
                "name":"Daily","workspace_id":"multi","cron":"0 9 * * 1-5",
                "time_zone":"Asia/Shanghai","prompt":"review",
                "permission_mode":"inherit","enabled":false,
                "missed_policy":"skip_and_record"
            }}),
            "rpc-schedule-multi-root",
        )
        .expect("multi-root schedule save");
        assert_eq!(result["schedule"]["definition"]["workspace_id"], "multi");
        process.shutdown();
    }

    #[test]
    fn plugin_journal_recovers_a_before_b_and_old_retry_cannot_overwrite_b() {
        let root = tempfile::tempdir().expect("root");
        let (extensions, process) = production_extensions(root.path());
        let v1 = write_plugin(root.path(), "1.0.0", &[]);
        let digest = package_digest(&extensions, &v1);
        execute(
            &extensions,
            "plugin/install",
            &install_payload(&v1, &digest, false),
            "install-first",
        )
        .expect("install v1");
        execute(
            &extensions,
            "plugin/setEnabled",
            &serde_json::json!({"pluginId":"com.example.journal","enabled":true}),
            "enable-initial",
        )
        .expect("enable initial");

        let payload_a = serde_json::json!({"pluginId":"com.example.journal","enabled":false});
        let mut operation = extensions
            .plugin_operation("plugin/setEnabled", &payload_a, "enable-a")
            .expect("operation A");
        operation.prepare(None).expect("prepare A");
        lock(extensions.plugin_store().expect("plugin store"))
            .expect("lock")
            .set_enabled("com.example.journal", false)
            .expect("semantic A commit");
        drop(operation);

        execute(
            &extensions,
            "plugin/setEnabled",
            &serde_json::json!({"pluginId":"com.example.journal","enabled":true}),
            "enable-b",
        )
        .expect("B recovers A then commits");
        execute(&extensions, "plugin/setEnabled", &payload_a, "enable-a")
            .expect("A exact retry re-acks");
        let listed =
            execute(&extensions, "plugin/list", &serde_json::json!({}), "list").expect("list");
        assert_eq!(listed["plugins"][0]["enabled"], true);
        process.shutdown();
    }

    #[test]
    fn plugin_journal_orders_remove_reinstall_and_install_update() {
        let root = tempfile::tempdir().expect("root");
        let (extensions, process) = production_extensions(root.path());
        let v1 = write_plugin(root.path(), "1.0.0", &[]);
        let digest_v1 = package_digest(&extensions, &v1);
        execute(
            &extensions,
            "plugin/install",
            &install_payload(&v1, &digest_v1, false),
            "install-first",
        )
        .expect("install v1");

        let remove = serde_json::json!({"pluginId":"com.example.journal"});
        let mut operation = extensions
            .plugin_operation("plugin/remove", &remove, "remove-a")
            .expect("remove operation");
        operation.prepare(Some(true)).expect("prepare remove");
        lock(extensions.plugin_store().expect("plugin store"))
            .expect("lock")
            .remove("com.example.journal")
            .expect("semantic remove");
        drop(operation);

        execute(
            &extensions,
            "plugin/install",
            &install_payload(&v1, &digest_v1, false),
            "reinstall-b",
        )
        .expect("reinstall follows recovered remove");
        execute(&extensions, "plugin/remove", &remove, "remove-a")
            .expect("old remove retry re-acks");
        assert_eq!(
            execute(&extensions, "plugin/list", &serde_json::json!({}), "list-1").expect("list")["plugins"]
                [0]["version"],
            "1.0.0"
        );

        let v2 = write_plugin(root.path(), "2.0.0", &[]);
        let digest_v2 = package_digest(&extensions, &v2);
        let payload_v2 = install_payload(&v2, &digest_v2, false);
        let mut operation = extensions
            .plugin_operation("plugin/install", &payload_v2, "update-a")
            .expect("update A");
        let source = PluginSource::from_path(&v2).expect("source v2");
        let mut store = lock(extensions.plugin_store().expect("plugin store")).expect("lock");
        store
            .freeze_source(&source, &operation.stage)
            .expect("freeze v2");
        operation.prepare(None).expect("prepare v2");
        let staged = PluginSource::from_path(&operation.stage).expect("staged v2");
        store
            .install(&staged, InstallOptions::default())
            .expect("semantic v2 commit");
        drop(store);
        drop(operation);

        let v3 = write_plugin(root.path(), "3.0.0", &[]);
        let digest_v3 = package_digest(&extensions, &v3);
        execute(
            &extensions,
            "plugin/install",
            &install_payload(&v3, &digest_v3, false),
            "update-b",
        )
        .expect("v3 follows recovered v2");
        execute(&extensions, "plugin/install", &payload_v2, "update-a")
            .expect("old update retry re-acks");
        assert_eq!(
            execute(&extensions, "plugin/list", &serde_json::json!({}), "list-2").expect("list")["plugins"]
                [0]["version"],
            "3.0.0"
        );
        process.shutdown();
    }

    #[test]
    fn plugin_precommit_failures_are_terminal_and_do_not_wedge_management() {
        let root = tempfile::tempdir().expect("root");
        let (extensions, process) = production_extensions(root.path());
        let missing = execute(
            &extensions,
            "plugin/setEnabled",
            &serde_json::json!({"pluginId":"com.example.missing","enabled":true}),
            "missing-enable",
        )
        .expect_err("missing plugin");
        assert_eq!(missing.code, "plugin-not-found");
        execute(
            &extensions,
            "plugin/list",
            &serde_json::json!({}),
            "list-after-missing",
        )
        .expect("read after failed receipt");

        let v2 = write_plugin(root.path(), "2.0.0", &[]);
        let digest_v2 = package_digest(&extensions, &v2);
        execute(
            &extensions,
            "plugin/install",
            &install_payload(&v2, &digest_v2, false),
            "install-v2",
        )
        .expect("install v2");
        let grants = execute(
            &extensions,
            "plugin/setGrants",
            &serde_json::json!({"pluginId":"com.example.journal","grants":["not.requested"]}),
            "bad-grants",
        )
        .expect_err("bad grants");
        assert_eq!(grants.code, "plugin-grants");
        execute(
            &extensions,
            "plugin/setEnabled",
            &serde_json::json!({"pluginId":"com.example.journal","enabled":false}),
            "valid-after-grants",
        )
        .expect("valid mutation after failed grants");

        let v1 = write_plugin(root.path(), "1.0.0", &[]);
        let digest_v1 = package_digest(&extensions, &v1);
        let downgrade = execute(
            &extensions,
            "plugin/install",
            &install_payload(&v1, &digest_v1, false),
            "bad-downgrade",
        )
        .expect_err("downgrade");
        assert_eq!(downgrade.code, "plugin-version-conflict");
        execute(
            &extensions,
            "plugin/list",
            &serde_json::json!({}),
            "list-after-downgrade",
        )
        .expect("list after downgrade failure");

        fs::write(v1.join("extra"), b"changed").expect("mutate source");
        let changed = execute(
            &extensions,
            "plugin/install",
            &install_payload(&v1, &digest_v1, false),
            "wrong-source",
        )
        .expect_err("wrong source digest");
        assert_eq!(changed.code, "plugin-invalid");
        process.shutdown();
    }

    #[test]
    fn skill_resources_never_cross_configured_workspace_catalogs() {
        let root = tempfile::tempdir().expect("root");
        let agent = root.path().join(".agent");
        fs::create_dir_all(&agent).expect("agent root");
        let repository = profile::ConfigRepository::open(root.path()).expect("config repository");
        let mut catalogs = BTreeMap::new();
        for id in ["workspace-a", "workspace-b"] {
            let skill = "shared";
            let project = root.path().join(format!("project-{id}"));
            let package = project.join(format!(".agent/skills/{skill}"));
            fs::create_dir_all(&package).expect("skill package");
            fs::write(
                package.join("SKILL.md"),
                format!(
                    "---\nname: {skill}\ndescription: {id} skill\n---\nresource owned by {id}\n"
                ),
            )
            .expect("skill");
            repository
                .publish_workspace(
                    0,
                    &profile::WorkspaceConfig {
                        format: 1,
                        revision: 1,
                        id: id.to_owned(),
                        name: id.to_owned(),
                        cwd: vec![project.to_string_lossy().into_owned()],
                        folders: Vec::new(),
                        policy: Some(profile::WorkspacePolicy::default()),
                    },
                )
                .expect("workspace config");
        }
        for (workspace_id, roots) in repository
            .resource_workspace_roots()
            .expect("workspace roots")
        {
            let snapshot = profile::InstructionResolver::new(&agent, &roots)
                .capture()
                .expect("snapshot");
            catalogs.insert(
                workspace_id,
                ResourceCatalog::from_snapshot(&snapshot).expect("catalog"),
            );
        }
        let process = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "1.0.0",
            &agent,
        )
        .expect("process");
        let production = assemble_production_endpoint_host(
            root.path(),
            SessionHostDescription {
                version: "1.0.0".to_owned(),
                cwd: root.path().display().to_string(),
                provider: None,
                model: None,
                attached_sessions: 0,
                home: root.path().display().to_string(),
                can_open_path: false,
            },
            Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
            &agent,
            Arc::clone(&process),
        )
        .expect("production assembly must isolate unrelated workspace skill names");
        assert!(
            production
                .extension_capabilities()
                .contains("resources/list")
        );
        let commands = EndpointCommandInputAuthority::new(
            Arc::clone(&process) as Arc<dyn SessionDeliveryAuthority>,
            Arc::new(|| Ok("2026-08-29T00:00:00.000Z".to_owned())),
            Arc::new(SessionInputAdmissionAuthority::new(
                root.path().to_path_buf(),
            )),
        );
        let extensions = ProductionClientExtensions::open(
            root.path(),
            ResourceCatalog::default(),
            catalogs,
            commands,
            Arc::clone(&process),
        )
        .expect("extensions");
        let workspace_a = execute(
            &extensions,
            "resources/list",
            &serde_json::json!({"workspaceId":"workspace-a","source":"skill","limit":100}),
            "resources-a",
        )
        .expect("workspace-a resources");
        let workspace_b = execute(
            &extensions,
            "resources/list",
            &serde_json::json!({"workspaceId":"workspace-b","source":"skill","limit":100}),
            "resources-b",
        )
        .expect("workspace-b resources");
        assert!(workspace_a.to_string().contains("shared"));
        assert!(workspace_b.to_string().contains("shared"));
        assert_ne!(
            workspace_a, workspace_b,
            "same-name skills retain workspace-local identities and digests"
        );
        process.shutdown();
    }

    fn append_canonical(lines: &mut Vec<u8>, value: Value) {
        lines.extend(serde_json_canonicalizer::to_vec(&value).expect("canonical event"));
        lines.push(b'\n');
    }

    fn epoch_profile_asset(
        assets: &store::AssetStore,
        dialect: &str,
        profile: &str,
        model: &str,
        owner: &str,
    ) -> store::AssetRef {
        let value = serde_json::json!({
            "controls":{},
            "serializer_revision":format!("{dialect}-serializer-1"),
            "system":"system",
            "target":{
                "protocol_family":if dialect.contains("anthropic") {"anthropic_messages"} else {"responses"},
                "dialect_id":dialect,
                "model_profile_id":profile,
                "route":{
                    "endpoint_owner":owner,
                    "gateway_translation":"direct",
                    "exact_sku":model,
                    "evidence_revision":format!("{owner}-evidence-1")
                }
            }
        });
        assets
            .publish(&serde_json_canonicalizer::to_vec(&value).expect("profile bytes"))
            .expect("publish profile")
    }

    #[test]
    fn usage_cache_attribution_joins_effective_attempts_to_exact_epoch_targets() {
        let root = tempfile::tempdir().expect("root");
        let (extensions, process) = production_extensions(root.path());
        let session = "018f0000-0000-7000-8000-000000000003";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(&folder).expect("thread folder");
        let assets = store::AssetStore::new(folder.join("assets")).expect("assets");
        let anthropic = epoch_profile_asset(
            &assets,
            "anthropic_messages_v1",
            "anthropic_messages_v1:claude",
            "claude",
            "anthropic",
        );
        let openai = epoch_profile_asset(
            &assets,
            "openai_responses_v1",
            "openai_responses_v1:gpt-5",
            "gpt-5",
            "openai",
        );
        let mut ledger = Vec::new();
        append_canonical(
            &mut ledger,
            serde_json::json!({"config":{"digest":"cfg"},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"client":"cli","key":"create","op":"create","principal":"p","target":session},"resume":"never","seq":1,"thread":session,"ts":"2026-08-29T00:00:00.000Z","v":1,"workspace":"ws"}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"content":[{"text":"one","type":"text"}],"kind":"input","origin_key":"i1","origin_tuple":{"client":"cli","key":"i1","op":"submit","principal":"p","target":session},"seq":2,"ts":"2026-08-29T00:00:01.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"binary":"worker","config_digest":"cfg","instruction_digest":"ins","kind":"run_start","mode":"ordinary","policy":"default","recovery_ordinal":0,"run":"r1","seq":3,"ts":"2026-08-29T00:00:02.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"turn_open","seq":4,"trigger":{"inputs":[2]},"ts":"2026-08-29T00:00:03.000Z","turn":1,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"adapter":"anthropic_messages_v1","id":"e1","kind":"epoch","model":"claude","reason":"initial","renderer":1,"seq":5,"system":{"asset":anthropic.asset,"digest":anthropic.asset.strip_prefix("sha256-").expect("digest")},"tools":{"asset":"sha256-td1","digest":"td1"},"ts":"2026-08-29T00:00:04.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"admits":[{"from":2,"to":2}],"attempt":"a1","epoch":"e1","kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"seq":6,"ts":"2026-08-29T00:00:05.000Z","turn":1,"v":1,"wire_digest":"wd1"}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"attempt":"a1","content":[{"text":"done","type":"text"}],"kind":"output","sealed":{"adapter":"anthropic_messages_v1","fragments":"f1","version":1},"seq":7,"ts":"2026-08-29T00:00:07.000Z","turn":1,"v":1,"usage":{"availability":"reported","input_tokens":"10","output_tokens":"5","cache_read":"2","cost":"0.01"}}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"settle","outcome":"completed","seq":8,"ts":"2026-08-29T00:00:08.000Z","turn":1,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"content":[{"text":"two","type":"text"}],"kind":"input","origin_key":"i2","origin_tuple":{"client":"cli","key":"i2","op":"submit","principal":"p","target":session},"seq":9,"ts":"2026-08-29T00:00:09.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"turn_open","seq":10,"trigger":{"inputs":[9]},"ts":"2026-08-29T00:00:10.000Z","turn":2,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"adapter":"openai_responses_v1","id":"e2","kind":"epoch","model":"gpt-5","reason":"profile-change","renderer":1,"seq":11,"system":{"asset":openai.asset,"digest":openai.asset.strip_prefix("sha256-").expect("digest")},"tools":{"asset":"sha256-td2","digest":"td2"},"ts":"2026-08-29T00:00:11.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"admits":[{"from":9,"to":9}],"attempt":"a2","epoch":"e2","kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"seq":12,"ts":"2026-08-29T00:00:12.000Z","turn":2,"v":1,"wire_digest":"wd2"}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"attempt":"a2","content":[{"text":"done","type":"text"}],"kind":"output","sealed":{"adapter":"openai_responses_v1","fragments":"f2","version":1},"seq":13,"ts":"2026-08-29T00:00:15.000Z","turn":2,"v":1,"usage":{"availability":"reported","input_tokens":"20","output_tokens":"7","cache_read":"3","cost":"0.02"}}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"settle","outcome":"completed","seq":14,"ts":"2026-08-29T00:00:16.000Z","turn":2,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"content":[{"text":"three","type":"text"}],"kind":"input","origin_key":"i3","origin_tuple":{"client":"cli","key":"i3","op":"submit","principal":"p","target":session},"seq":15,"ts":"2026-08-29T00:00:17.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"turn_open","seq":16,"trigger":{"inputs":[15]},"ts":"2026-08-29T00:00:18.000Z","turn":3,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"admits":[{"from":15,"to":15}],"attempt":"a3","epoch":"e2","kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"seq":17,"ts":"2026-08-29T00:00:19.000Z","turn":3,"v":1,"wire_digest":"wd3"}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"attempt":"a3","content":[{"text":"done","type":"text"}],"kind":"output","sealed":{"adapter":"openai_responses_v1","fragments":"f3","version":1},"seq":18,"ts":"2026-08-29T00:00:21.000Z","turn":3,"v":1,"usage":{"availability":"reported","input_tokens":"1","output_tokens":"1","cache_read":"4","cost":"0.03"}}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"settle","outcome":"completed","seq":19,"ts":"2026-08-29T00:00:22.000Z","turn":3,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"content":[{"text":"four","type":"text"}],"kind":"input","origin_key":"i4","origin_tuple":{"client":"cli","key":"i4","op":"submit","principal":"p","target":session},"seq":20,"ts":"2026-08-29T00:00:23.000Z","v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"turn_open","seq":21,"trigger":{"inputs":[20]},"ts":"2026-08-29T00:00:24.000Z","turn":4,"v":1}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"admits":[{"from":20,"to":20}],"attempt":"a4","epoch":"e2","kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"seq":22,"ts":"2026-08-29T00:00:25.000Z","turn":4,"v":1,"wire_digest":"wd4"}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"attempt":"a4","content":[{"text":"done","type":"text"}],"kind":"output","sealed":{"adapter":"openai_responses_v1","fragments":"f4","version":1},"seq":23,"ts":"2026-08-29T00:00:27.000Z","turn":4,"v":1,"usage":{"availability":"reported","input_tokens":"2","output_tokens":"2"}}),
        );
        append_canonical(
            &mut ledger,
            serde_json::json!({"kind":"settle","outcome":"completed","seq":24,"ts":"2026-08-29T00:00:28.000Z","turn":4,"v":1}),
        );
        fs::write(folder.join("main.jsonl"), ledger).expect("ledger");

        let attribution = execute(
            &extensions,
            "usage.cacheAttribution",
            &serde_json::json!({"sessionId":session}),
            "usage-cache",
        )
        .expect("cache attribution");
        assert_eq!(attribution["entries"].as_array().expect("entries").len(), 2);
        assert_eq!(
            attribution["entries"][0]["target"]["dialectId"],
            "anthropic_messages_v1"
        );
        assert_eq!(attribution["entries"][0]["inputTokens"], "10");
        assert_eq!(
            attribution["entries"][1]["target"]["dialectId"],
            "openai_responses_v1"
        );
        assert_eq!(attribution["entries"][1]["attempts"], 2);
        assert_eq!(attribution["entries"][1]["inputTokens"], "21");
        assert_eq!(attribution["entries"][1]["outputTokens"], "8");
        assert_eq!(attribution["entries"][1]["cacheReadTokens"], "7");
        assert_eq!(attribution["entries"][1]["cost"], "0.05");
        let summary = execute(
            &extensions,
            "usage.summary",
            &serde_json::json!({"sessionId":session}),
            "usage-summary",
        )
        .expect("usage summary");
        assert_eq!(summary["inputTokens"], 33);
        assert_eq!(summary["outputTokens"], 15);
        process.shutdown();
    }
}
