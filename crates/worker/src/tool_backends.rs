use super::*;

pub(crate) struct AllowedDynamicBackend<'a> {
    allowed: BTreeSet<String>,
    inner: Vec<Box<dyn DynamicBackend + 'a>>,
}

impl DynamicBackend for AllowedDynamicBackend<'_> {
    fn supports(&self, tool: &DynamicTool) -> bool {
        self.allowed.contains(&tool.name) && self.inner.iter().any(|backend| backend.supports(tool))
    }

    fn execute(
        &mut self,
        tool: &DynamicTool,
        execution: &ToolExecution,
        invocation: &IJsonValue,
    ) -> BackendTerminal {
        self.inner
            .iter_mut()
            .find(|backend| backend.supports(tool))
            .map_or_else(
                || BackendTerminal::Unavailable {
                    code: "dynamic_backend_unavailable".to_owned(),
                    message: format!("dynamic route is unavailable for {}", tool.name),
                    retryable: false,
                },
                |backend| backend.execute(tool, execution, invocation),
            )
    }
}

pub(crate) struct AllowedToolBackend<'a> {
    allowed: BTreeSet<String>,
    inner: ToolBackendRouter<'a>,
}

impl ToolBackend for AllowedToolBackend<'_> {
    fn supports(&self, name: &str) -> bool {
        self.allowed.contains(name) && self.inner.supports(name)
    }

    fn execute(&mut self, execution: &ToolExecution, invocation: &IJsonValue) -> BackendTerminal {
        self.inner.execute(execution, invocation)
    }

    fn resume_after_approval(
        &mut self,
        execution: &ToolExecution,
        invocation: &IJsonValue,
        approval: &DurableApprovalResponse,
    ) -> BackendTerminal {
        self.inner
            .resume_after_approval(execution, invocation, approval)
    }
}

/// Child creation is worker-owned, while process launch is supervisor-owned.
/// The first pipeline pass deliberately stops after hooks and the durable
/// approval gate so the worker can complete that transaction before a
/// `tool_result` becomes possible.
pub(crate) struct DeferredChildBackend;

impl ToolBackend for DeferredChildBackend {
    fn supports(&self, name: &str) -> bool {
        matches!(name, "task" | "subagent")
    }

    fn execute(&mut self, _execution: &ToolExecution, _invocation: &IJsonValue) -> BackendTerminal {
        BackendTerminal::Deferred
    }
}

pub(crate) struct ReplayedChildBackend {
    pub(crate) result: ToolControlResult,
}

impl ToolBackend for ReplayedChildBackend {
    fn supports(&self, name: &str) -> bool {
        matches!(name, "task" | "subagent")
    }

    fn execute(&mut self, execution: &ToolExecution, _invocation: &IJsonValue) -> BackendTerminal {
        if self.result.call_id != execution.call {
            return BackendTerminal::Unavailable {
                code: "protocol".to_owned(),
                message: "cached child launch result has the wrong call id".to_owned(),
                retryable: false,
            };
        }
        match (&self.result.value, &self.result.error) {
            (Some(value), None) => BackendTerminal::Completed(value.clone()),
            (None, Some(error)) => BackendTerminal::Unavailable {
                code: match error.code {
                    ToolControlErrorCode::Unsupported => "unsupported",
                    ToolControlErrorCode::Denied => "denied",
                    ToolControlErrorCode::NotFound => "not_found",
                    ToolControlErrorCode::Conflict => "conflict",
                    ToolControlErrorCode::Unavailable => "unavailable",
                    ToolControlErrorCode::Timeout => "timeout",
                    ToolControlErrorCode::EffectUnknown => "effect_unknown",
                    ToolControlErrorCode::EffectConflicted => "effect_conflicted",
                    ToolControlErrorCode::Internal => "internal",
                }
                .to_owned(),
                message: error.message.clone(),
                retryable: error.retryable,
            },
            _ => BackendTerminal::Unavailable {
                code: "protocol".to_owned(),
                message: "cached child launch result has an invalid union".to_owned(),
                retryable: false,
            },
        }
    }
}

pub(crate) fn exchange_tool_control_runtime(
    selected: &Selected,
    request: &ToolControl,
    lines: &mut impl Iterator<Item = io::Result<String>>,
    stdout: &mut impl Write,
    cancellation: &RuntimeCancellation,
) -> Result<ToolControlResult, Box<dyn std::error::Error>> {
    require_version(selected)?;
    if cancellation.stop_requested() {
        return Err("tool-control cancelled by stop".into());
    }
    if cancellation.supervisor_lost() {
        cancellation.mark_protocol_failed();
        return Err(Box::new(ProtocolFailure(
            "supervisor EOF before tool-control request".to_owned(),
        )));
    }
    stdout.write_all(&encode_tool_control(request)?)?;
    stdout.flush()?;
    loop {
        let Some(line) = lines.next() else {
            cancellation.cancel(CANCEL_SUPERVISOR_LOSS);
            cancellation.mark_protocol_failed();
            return Err(Box::new(ProtocolFailure(
                "supervisor EOF while awaiting tool-control result".to_owned(),
            )));
        };
        let line = line.map_err(|error| {
            cancellation.cancel(CANCEL_SUPERVISOR_LOSS);
            cancellation.mark_protocol_failed();
            ProtocolFailure(format!("supervisor read failed: {error}"))
        })?;
        match decode_tool_control_result(line.as_bytes()) {
            Ok(result) => {
                if let Err(error) = result.validate_for(request) {
                    cancellation.mark_protocol_failed();
                    return Err(Box::new(error));
                }
                return Ok(result);
            }
            Err(_) => match decode_supervisor(line.as_bytes()) {
                Ok(SupervisorMessage::Ping(ping)) => {
                    stdout
                        .write_all(&encode_line("pong", &worker_control::Pong { id: ping.id })?)?;
                    stdout.flush()?;
                }
                Ok(SupervisorMessage::Stop(_)) => {
                    cancellation.cancel(CANCEL_STOP);
                    cancellation.defer(line);
                    return Err("tool-control cancelled by stop".into());
                }
                Ok(SupervisorMessage::QueueTransaction(_)) => cancellation.park_delivery(line),
                Ok(_) => {
                    cancellation.mark_protocol_failed();
                    return Err(Box::new(ProtocolFailure(
                        "unexpected supervisor message while awaiting tool-control result"
                            .to_owned(),
                    )));
                }
                Err(error) => {
                    cancellation.mark_protocol_failed();
                    return Err(Box::new(error));
                }
            },
        }
    }
}

pub(crate) fn tool_catalog_context(
    profile: &RuntimeProfile,
    has_deferred_catalog: bool,
    web_search_ready: bool,
) -> Result<CatalogContext, Box<dyn std::error::Error>> {
    let selected_tools = effective_allowed_tools(profile);
    let selectors = [
        ("plan", CatalogRole::Plan),
        ("summary_artifact", CatalogRole::Compactor),
        ("verify", CatalogRole::Validator),
    ]
    .into_iter()
    .filter(|(name, _)| selected_tools.contains(*name))
    .collect::<Vec<_>>();
    if selected_tools.contains("report") && !profile.subagent {
        return Err("report requires a durably delegated child role".into());
    }
    if selectors.len() > 1 && !profile.validator && !profile.subagent {
        return Err(format!(
            "effective tool selection contains conflicting role selectors: {}",
            selectors
                .iter()
                .map(|(name, _)| *name)
                .collect::<Vec<_>>()
                .join(", ")
        )
        .into());
    }
    Ok(CatalogContext {
        role: if profile.validator {
            CatalogRole::Validator
        } else if profile.subagent {
            CatalogRole::Subagent
        } else {
            selectors
                .first()
                .map_or(CatalogRole::Ordinary, |(_, role)| *role)
        },
        has_web_credential: web_search_ready
            && profile.config.providers.web_search.is_some()
            && profile
                .instruction
                .meet_workspace_policy(&profile.config.workspace.policy)
                .network,
        has_deferred_catalog,
        selected_tools,
    })
}

pub(crate) struct ToolBackendPlan<'a> {
    pub(crate) profile: &'a RuntimeProfile,
    pub(crate) selected: Selected,
    pub(crate) thread_folder: &'a std::path::Path,
    pub(crate) invocation_write_roots: &'a [String],
    pub(crate) credential: Option<&'a Arc<Mutex<CredentialClient>>>,
    pub(crate) web_search_ready: bool,
}

pub(crate) struct ToolBackendRuntime<'a, I, O> {
    pub(crate) lines: &'a mut I,
    pub(crate) stdout: &'a mut O,
    pub(crate) cancellation: &'a RuntimeCancellation,
}

pub(crate) fn assemble_tool_backends<'a, I, O>(
    plan: ToolBackendPlan<'_>,
    runtime: ToolBackendRuntime<'a, I, O>,
) -> Result<(AllowedToolBackend<'a>, AllowedDynamicBackend<'a>), Box<dyn std::error::Error>>
where
    I: Iterator<Item = io::Result<String>>,
    O: Write,
{
    let ToolBackendPlan {
        profile,
        selected,
        thread_folder,
        invocation_write_roots,
        credential,
        web_search_ready,
    } = plan;
    let ToolBackendRuntime {
        lines,
        stdout,
        cancellation,
    } = runtime;
    let allowed = effective_allowed_tools(profile);
    let mut router = ToolBackendRouter::default();
    let storage_root = storage_root_for_thread_folder(thread_folder)?;

    let effective = profile
        .instruction
        .meet_workspace_policy(&profile.config.workspace.policy);
    let workspace = profile
        .config
        .workspace
        .cwd
        .first()
        .ok_or("runtime profile has no workspace root")?;
    let helper_path = sibling_helper_path()?;
    let mut read_roots = profile.config.workspace.cwd.clone();
    read_roots.extend(
        profile
            .config
            .workspace
            .policy
            .toolchain_roots
            .iter()
            .cloned(),
    );
    read_roots.sort();
    read_roots.dedup();
    let mut write_roots = invocation_write_roots.to_vec();
    write_roots.sort();
    write_roots.dedup();
    let sandbox_policy = SandboxPolicy {
        format: 1,
        read_roots: read_roots.clone(),
        write_roots,
        network: if effective.network {
            NetworkPolicy::All
        } else {
            NetworkPolicy::Deny
        },
        allow_process: true,
        scratch: None,
    };
    #[cfg(target_os = "macos")]
    let sandbox_backend = SandboxBackend::DarwinSeatbeltV1;
    #[cfg(target_os = "linux")]
    let sandbox_backend = SandboxBackend::LinuxLandlockSeccompV1;
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let sandbox_backend = SandboxBackend::LinuxLandlockSeccompV1;
    let probe = probe_backend(sandbox_backend);
    let roots = profile
        .config
        .workspace
        .cwd
        .iter()
        .enumerate()
        .map(|(index, path)| {
            (
                if index == 0 {
                    "workspace".to_owned()
                } else {
                    format!("workspace-{index}")
                },
                PathBuf::from(path),
            )
        })
        .collect::<Vec<_>>();
    let (helper, scratch_path) =
        if helper_path.is_file() && matches!(probe, ProbeStatus::Available { .. }) {
            let client =
                HelperClient::sandboxed_with_scratch(helper_path, roots, sandbox_policy, probe)?;
            let scratch_path = client.scratch_path().map(str::to_owned);
            (
                Some(Arc::new(client) as Arc<dyn HelperInvoker>),
                scratch_path,
            )
        } else {
            (None, None)
        };
    let mut dynamic_backends: Vec<Box<dyn DynamicBackend + 'a>> = Vec::new();
    let artifact_versions = helper
        .as_ref()
        .map(|_| DurableArtifactVersions::new(storage_root.join("tool-state")))
        .transpose()?
        .map(|authority| Arc::new(authority) as Arc<dyn engine::ArtifactVersionAuthority>);
    let http = effective
        .network
        .then(|| BoundedHttpClient::new(HttpLimits::default()))
        .transpose()?;
    let search = match (
        web_search_ready,
        effective.network,
        profile.config.providers.web_search.as_ref(),
        credential,
    ) {
        (true, true, Some(config), Some(credential)) if config.adapter == "tavily_v1" => {
            Some(Arc::new(TavilySearchProvider {
                config: config.clone(),
                credential: Arc::clone(credential),
                transport: Arc::new(ProductionTavilyTransport),
            }) as Arc<dyn SearchProvider>)
        }
        _ => None,
    };
    let mut system_config = SystemToolConfig::workspace("workspace", workspace);
    if profile.validator {
        system_config
            .environment
            .insert("PYTHONDONTWRITEBYTECODE".into(), "1".into());
    }
    if let Some(scratch_path) = scratch_path {
        system_config
            .environment
            .insert("TMPDIR".into(), format!("{scratch_path}/"));
    }
    if !profile.config.workspace.policy.toolchain_roots.is_empty() {
        let mut bins = profile
            .config
            .workspace
            .policy
            .toolchain_roots
            .iter()
            .map(|root| {
                std::path::Path::new(root)
                    .join("bin")
                    .to_string_lossy()
                    .into_owned()
            })
            .collect::<Vec<_>>();
        bins.extend(
            ["/usr/bin", "/bin", "/usr/sbin", "/sbin"]
                .into_iter()
                .map(str::to_owned),
        );
        system_config
            .environment
            .insert("PATH".into(), bins.join(":"));
    }
    system_config.extra_roots = profile
        .config
        .workspace
        .cwd
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index, path)| engine::RootMount {
            name: format!("workspace-{index}"),
            path: PathBuf::from(path),
        })
        .collect();
    let mut system_backend = SystemToolBackend::new(
        system_config,
        helper,
        http,
        search,
        artifact_versions,
        cancellation.tools.clone(),
    );
    let workspace_helper = std::env::current_exe()?
        .parent()
        .ok_or("worker executable has no parent directory")?
        .join("tekes-workspace-service");
    if workspace_helper.is_file() && !profile.validator {
        system_backend =
            system_backend.with_edit_recorder(Arc::new(workspace_edits::WorkspaceEditRecorder {
                executable: workspace_helper,
                workspace: PathBuf::from(workspace),
                workspace_roots: profile
                    .config
                    .workspace
                    .cwd
                    .iter()
                    .map(PathBuf::from)
                    .collect(),
                state_root: storage_root.join("workspace-service"),
                workspace_id: profile.config.workspace.id.clone(),
                session_id: thread_folder
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or("thread folder has no session identity")?
                    .to_owned(),
            }));
    }
    router.push(system_backend);
    {
        let session = thread_folder
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or("thread folder has no UTF-8 session UUID")?
            .to_owned();
        let exchange = Rc::new(RefCell::new(move |request: &ToolControl| {
            exchange_tool_control_runtime(&selected, request, lines, stdout, cancellation)
                .map_err(|error| error.to_string())
        }));
        let fixed_exchange = Rc::clone(&exchange);
        router.push(SupervisorControlBackend::new(
            session.clone(),
            move |request: &ToolControl| (fixed_exchange.borrow_mut())(request),
        ));
        dynamic_backends.push(Box::new(DynamicSupervisorBackend::new(
            session,
            move |request: &ToolControl| (exchange.borrow_mut())(request),
        )));
    }
    let dynamic = AllowedDynamicBackend {
        allowed: allowed.clone(),
        inner: dynamic_backends,
    };
    let deferred_tools = DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)
        .deferred_search_entries(&dynamic)?;
    router.push(WorkflowBackend::new(
        &storage_root,
        profile.config.workspace.id.clone(),
        profile.bindings.goal_id.clone(),
        frozen_skill_catalog(&profile.instruction)?,
        deferred_tools,
    )?);
    Ok((
        AllowedToolBackend {
            allowed,
            inner: router,
        },
        dynamic,
    ))
}

pub(crate) fn invocation_write_roots(
    profile: &RuntimeProfile,
    call: &provider::ToolCall,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let workspace = PathBuf::from(
        profile
            .config
            .workspace
            .cwd
            .first()
            .ok_or("runtime profile has no workspace root")?,
    );
    if profile
        .bindings
        .dynamic_catalog
        .tools
        .iter()
        .any(|tool| tool.name == call.name && tool.effect == DynamicToolEffect::WorkspaceWrite)
    {
        return Ok(profile
            .instruction
            .meet_workspace_policy(&profile.config.workspace.policy)
            .writable_roots);
    }
    let allowed = profile
        .instruction
        .meet_workspace_policy(&profile.config.workspace.policy)
        .writable_roots
        .into_iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    let declared = match call.name.as_str() {
        "shell"
            if call.arguments.get("writable_paths").is_none_or(|value| {
                value.is_null() || value.as_array().is_some_and(Vec::is_empty)
            }) =>
        {
            // The effective user permission policy is the authority. A model
            // need not restate it in every command just to write a file.
            allowed
                .iter()
                .map(|path| path.to_string_lossy().into_owned())
                .collect()
        }
        "shell" => call
            .arguments
            .get("writable_paths")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .map(|value| {
                value
                    .as_str()
                    .map(ToOwned::to_owned)
                    .ok_or("shell writable_paths must contain strings")
            })
            .collect::<Result<Vec<_>, _>>()?,
        "apply_patch" | "edit" | "write" => call
            .arguments
            .get("path")
            .and_then(Value::as_str)
            .map(|path| vec![path.to_owned()])
            .unwrap_or_default(),
        _ => Vec::new(),
    };
    let mut roots = Vec::new();
    for raw in declared {
        let raw_path = PathBuf::from(&raw);
        if raw_path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
        {
            return Err(format!("writable path {raw:?} contains parent traversal").into());
        }
        let target = if raw_path.is_absolute() {
            raw_path
        } else {
            workspace.join(raw_path)
        };
        if !allowed.iter().any(|root| target.starts_with(root)) {
            return Err(format!(
                "writable path {} is outside the effective writable roots",
                target.display()
            )
            .into());
        }
        let authority = if matches!(call.name.as_str(), "apply_patch" | "edit" | "write") {
            target
                .parent()
                .ok_or("apply_patch target has no writable parent")?
                .to_path_buf()
        } else {
            target
        };
        roots.push(authority.to_string_lossy().into_owned());
    }
    roots.sort();
    roots.dedup();
    Ok(roots)
}

fn storage_root_for_thread_folder(
    thread_folder: &std::path::Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let parent = thread_folder
        .parent()
        .ok_or("thread folder has no storage-root parent")?;
    if parent.file_name().is_some_and(|name| name == "threads") {
        return Ok(parent
            .parent()
            .ok_or("threads directory has no storage root")?
            .to_path_buf());
    }
    // Unit tests and embedders may open a standalone thread folder. Its
    // parent is the only honest storage authority available in that layout.
    Ok(parent.to_path_buf())
}

/// The session's durable permission mode, read fresh from the thread folder
/// for every tool batch. A corrupt record is the default mode plus a
/// diagnostic, never a crash.
pub(crate) fn session_permission_policy(thread_folder: &std::path::Path) -> PermissionModePolicy {
    let (policy, diagnostic) = PermissionModePolicy::for_session_folder(thread_folder);
    if let Some(diagnostic) = diagnostic {
        eprintln!("tekes-worker: permission mode {diagnostic}");
    }
    policy
}
