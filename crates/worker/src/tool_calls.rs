use super::*;

pub(crate) fn execute_provider_tool_calls<I, O>(
    ledger: &mut LockedLedger,
    batch: ToolCallBatch<'_>,
    credential: Option<&Arc<Mutex<CredentialClient>>>,
    lines: &mut I,
    stdout: &mut O,
) -> Result<bool, Box<dyn std::error::Error>>
where
    I: Iterator<Item = io::Result<String>>,
    O: Write,
{
    let ToolCallBatch {
        options,
        profile,
        selected,
        attempt,
        turn,
        calls,
        cancellation,
    } = batch;
    let thread_folder = ledger
        .path()
        .parent()
        .ok_or("worker ledger has no thread folder")?
        .to_path_buf();
    let thread = ledger
        .projection()
        .and_then(|projection| projection.events.first())
        .and_then(|genesis| genesis.string_field("thread"))
        .ok_or("genesis thread binding is missing")?
        .to_owned();
    let manifest = BuiltinManifest::compiled();
    let assets = store::AssetStore::new(thread_folder.join("assets"))?;
    let hooks = frozen_hook_bindings(&profile.instruction)?;
    // Re-read per batch: a mode selected through the endpoint applies to the
    // next tool call of a running session without a restart.
    let mut policy = session_permission_policy(&thread_folder);
    for call in calls {
        if cancellation.supervisor_lost() {
            return Err(Box::new(ProtocolFailure(
                "supervisor lost before tool execution".to_owned(),
            )));
        }
        if cancellation.stop_requested() {
            return Ok(true);
        }
        if matches!(call.name.as_str(), "task" | "subagent") {
            if execute_child_spawn_tool(
                ledger,
                ChildSpawnExecution {
                    options,
                    profile,
                    selected,
                    attempt,
                    turn,
                    call,
                    hooks: &hooks,
                    manifest: &manifest,
                    cancellation,
                },
                lines,
                stdout,
            )? {
                return Ok(true);
            }
            continue;
        }
        if let Some(detail) = provider::invalid_arguments_detail(&call.arguments) {
            // The model's arguments were not JSON; the call is refused with a
            // durable error result so the model can resend it, never executed.
            append_tool_validation_error(ledger, options, turn, call, &detail)?;
            continue;
        }
        let write_roots = match invocation_write_roots(profile, call) {
            Ok(roots) => roots,
            Err(error) => {
                // Invalid model-supplied authority is a tool rejection, not
                // a worker crash. Otherwise eager dispatch exits with an
                // unpaired call and every recovery repeats the same failure.
                append_tool_validation_error(ledger, options, turn, call, &error.to_string())?;
                continue;
            }
        };
        let (mut backend, mut dynamic_backend) = assemble_tool_backends(
            ToolBackendPlan {
                profile,
                selected,
                thread_folder: &thread_folder,
                invocation_write_roots: &write_roots,
                credential,
                web_search_ready: options.web_search_ready,
            },
            ToolBackendRuntime {
                lines,
                stdout,
                cancellation,
            },
        )?;
        let arguments = IJsonValue::parse(&serde_json::to_vec(&call.arguments)?)?;
        let invocation = ToolInvocation {
            thread: thread.clone(),
            turn,
            attempt: attempt.to_owned(),
            call: call.call_id.clone(),
            name: call.name.clone(),
            arguments,
            timestamp: options.event_timestamp().clone(),
        };
        let has_deferred = !DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)
            .deferred_search_entries(&dynamic_backend)?
            .is_empty();
        let context = tool_catalog_context(profile, has_deferred, options.web_search_ready)?;
        let mut pipeline = ToolPipeline::new(ledger, assets.clone(), SecretScanner::default());
        let decision = if manifest
            .tools
            .iter()
            .any(|tool| tool.name == invocation.name)
        {
            ToolDispatcher::new(&manifest, &context)
                .dispatch(
                    &mut pipeline,
                    &invocation,
                    &hooks,
                    &mut policy,
                    &mut backend,
                )
                .map_err(|error| error.to_string())
        } else {
            DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)
                .dispatch(
                    &mut pipeline,
                    &invocation,
                    &hooks,
                    &mut policy,
                    &mut dynamic_backend,
                )
                .map_err(|error| error.to_string())
        };
        let decision = match decision {
            Ok(decision) => decision,
            Err(error) => {
                drop(pipeline);
                append_tool_validation_error(ledger, options, turn, call, &error)?;
                continue;
            }
        };
        if cancellation.protocol_failed() {
            return Err(Box::new(ProtocolFailure(
                "tool-control protocol failure".to_owned(),
            )));
        }
        if cancellation.stop_requested() {
            return Ok(true);
        }
        if cancellation.supervisor_lost() {
            return Err(Box::new(ProtocolFailure(
                "supervisor lost during tool execution".to_owned(),
            )));
        }
        if matches!(decision, PipelineDecision::Parked { .. }) {
            return Ok(true);
        }
        if let PipelineDecision::Pending {
            continuation_id, ..
        } = decision
        {
            // The supervisor bound a remote continuation (an MCP task). The
            // durable step-0 state is written; drive it to its single terminal.
            // The backends hold the control channel; release them first.
            drop(backend);
            drop(dynamic_backend);
            if wait_tool_continuation(
                ledger,
                ContinuationWait {
                    options,
                    profile,
                    selected,
                    call: &call.call_id,
                    continuation_id: &continuation_id,
                    cancellation,
                },
                lines,
                stdout,
            )? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub(crate) fn append_tool_validation_error(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    call: &provider::ToolCall,
    detail: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"tool_result",
        "ts":options.event_timestamp(),"call":call.call_id,"outcome":"error",
        "content":[{"type":"text","text":bounded_ledger_detail(detail)}],
        "meta":{"classification":"validation","tool":call.name}
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(())
}

pub(crate) fn frozen_hook_bindings(
    instruction: &InstructionSnapshot,
) -> Result<Vec<HookBinding>, Box<dyn std::error::Error>> {
    instruction
        .effective
        .hooks
        .iter()
        .filter(|(_, index)| {
            instruction
                .sources
                .get(**index as usize)
                .and_then(|source| serde_json::from_str::<Value>(&source.content).ok())
                .is_some_and(|value| value["format"] == 1)
        })
        .map(|(logical_name, index)| {
            let source = instruction
                .sources
                .get(*index as usize)
                .ok_or_else(|| format!("effective hook {logical_name:?} has an invalid index"))?;
            decode_hook_binding(source.content.as_bytes(), logical_name)
                .map_err(|error| Box::new(error) as Box<dyn std::error::Error>)
        })
        .collect()
}

pub(crate) fn frozen_skill_catalog(
    instruction: &InstructionSnapshot,
) -> Result<Vec<CatalogEntry>, Box<dyn std::error::Error>> {
    let resources = ResourceCatalog::from_snapshot(instruction)?;
    let mut entries = resources
        .skill_summaries()
        .into_iter()
        .map(|summary| {
            let package = resources.skill(&summary.name)?;
            let content = json!({
                "format": 1,
                "name": summary.name,
                "description": summary.description,
                "body": package.body,
                "resources": package.resources,
            });
            Ok(CatalogEntry {
                name: summary.name,
                summary: summary.description,
                aliases: Vec::new(),
                schema_digest: summary.content_digest,
                content: IJsonValue::parse(&serde_json::to_vec(&content)?)?,
            })
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    let legacy = instruction
        .effective
        .skills
        .iter()
        .filter(|(logical_name, _)| !logical_name.contains('/'))
        .map(|(logical_name, index)| {
            let source = instruction
                .sources
                .get(*index as usize)
                .ok_or_else(|| format!("effective skill {logical_name:?} has an invalid index"))?;
            let summary = source
                .content
                .lines()
                .find(|line| !line.trim().is_empty())
                .map_or_else(|| logical_name.clone(), |line| line.trim().to_owned());
            Ok(CatalogEntry {
                name: logical_name.clone(),
                summary,
                aliases: Vec::new(),
                schema_digest: source.content_sha256.clone(),
                content: IJsonValue::from(source.content.clone()),
            })
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    entries.extend(legacy);
    entries.sort_by(|left, right| left.name.as_bytes().cmp(right.name.as_bytes()));
    Ok(entries)
}

pub(crate) struct ToolCallBatch<'a> {
    pub(crate) options: &'a Options,
    pub(crate) profile: &'a RuntimeProfile,
    pub(crate) selected: Selected,
    pub(crate) attempt: &'a str,
    pub(crate) turn: u64,
    pub(crate) calls: &'a [provider::ToolCall],
    pub(crate) cancellation: &'a RuntimeCancellation,
}
