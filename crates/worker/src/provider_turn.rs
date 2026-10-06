use super::*;

#[cfg(test)]
thread_local! {
    /// Hermetic transport seam: profile resolution and request digests stay bound to the proved
    /// route, while the unit test sends those already-prepared bytes to a loopback fixture.
    static PROVIDER_TEST_REDIRECT: RefCell<Option<String>> = const { RefCell::new(None) };
}

#[cfg(test)]
pub(crate) fn set_provider_test_redirect(endpoint: Option<String>) {
    PROVIDER_TEST_REDIRECT.with(|slot| *slot.borrow_mut() = endpoint);
}

pub(crate) struct ProviderOutcome {
    pub(crate) seq: u64,
    pub(crate) settle: Option<(&'static str, Option<&'static str>)>,
    pub(crate) retry: bool,
    pub(crate) retry_classification: Option<&'static str>,
    /// Provider-declared admission delay for a rate-limited retry.
    pub(crate) retry_after_seconds: Option<u64>,
    pub(crate) compact_retry: bool,
    pub(crate) tool_calls: Vec<provider::ToolCall>,
}

pub(crate) struct ProviderContext {
    pub(crate) items: Vec<IJsonValue>,
    pub(crate) admits: Vec<u64>,
    pub(crate) continuation_id: Option<String>,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn run_provider_turn(
    ledger: &mut LockedLedger,
    options: &Options,
    profile: &RuntimeProfile,
    selected: &Selected,
    credential: &mut Option<Arc<Mutex<CredentialClient>>>,
    lines: &mut impl Iterator<Item = io::Result<String>>,
    stdout: &mut impl Write,
    cancellation: &RuntimeCancellation,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(hooks) = &options.lifecycle_hooks {
        ledger.sync_prefix()?;
        if let Some(turn) = ledger.projection().and_then(|p| p.latest_turn) {
            if !ledger.projection().is_some_and(|p| p.terminal_tail) {
                hooks.before_turn(ledger, turn);
            }
        }
    }
    let mut execution_profile = profile.clone();
    validation_runtime::activate_validator_profile(ledger, &mut execution_profile)?;
    let profile = &execution_profile;
    let wall_deadline = profile
        .config
        .workspace
        .policy
        .max_wall_seconds
        .map(|seconds| Instant::now() + Duration::from_secs(seconds));
    let result = run_provider_turn_inner(
        ledger,
        ProviderRunContext {
            options,
            profile,
            selected,
        },
        credential,
        lines,
        stdout,
        ProviderLoopBudget {
            retries_left: PROVIDER_RETRIES,
            compactions_left: 1,
            trims_left: 1,
            nonfinal_responses_left: 32,
            wall_deadline,
        },
        cancellation,
    );
    match result {
        Ok(()) => Ok(()),
        Err(error) if is_protocol_failure(error.as_ref()) => Err(error),
        // Stop owns its receipt and interrupted settlement in post_turn_exit.
        // Child launch/wait cancellation must not preempt it with internal error.
        Err(_) if cancellation.stop_requested() => Ok(()),
        Err(error) => match settle_internal_worker_failure(ledger, options, error.as_ref()) {
            Ok(()) => Ok(()),
            Err(_) => Err(error),
        },
    }
}

#[derive(Clone, Copy)]
struct ProviderLoopBudget {
    retries_left: u8,
    compactions_left: u8,
    trims_left: u8,
    nonfinal_responses_left: u8,
    wall_deadline: Option<Instant>,
}

#[derive(Clone, Copy)]
pub(crate) struct ProviderRunContext<'a> {
    pub(crate) options: &'a Options,
    pub(crate) profile: &'a RuntimeProfile,
    pub(crate) selected: &'a Selected,
}

fn run_provider_turn_inner(
    ledger: &mut LockedLedger,
    run: ProviderRunContext<'_>,
    credential: &mut Option<Arc<Mutex<CredentialClient>>>,
    lines: &mut impl Iterator<Item = io::Result<String>>,
    stdout: &mut impl Write,
    budget: ProviderLoopBudget,
    cancellation: &RuntimeCancellation,
) -> Result<(), Box<dyn std::error::Error>> {
    let ProviderRunContext {
        options,
        profile,
        selected,
    } = run;
    let turn = ledger
        .projection()
        .and_then(|projection| projection.latest_turn)
        .ok_or("provider turn requires turn_open")?;
    if budget
        .wall_deadline
        .is_some_and(|deadline| Instant::now() >= deadline)
    {
        append_settle(
            ledger,
            &options.event_timestamp(),
            turn,
            "interrupted",
            Some("budget_wall"),
        )?;
        return Ok(());
    }
    if ledger
        .projection()
        .is_some_and(|projection| projection.terminal_tail || projection.lifecycle.open_hold)
    {
        return Ok(());
    }
    let recovery = recover_provider_attempt_for_ordinary(ledger, options, stdout)?;
    let RecoveryProgress::Continue { reset_epoch } = recovery else {
        return Ok(());
    };
    if validation_runtime::advance(ledger, run, lines, stdout, cancellation)? {
        return Ok(());
    }
    let (provider_config, model) = match selected_provider(&profile.config) {
        Ok(selected) => selected,
        Err(reason) => {
            settle_provider_unavailable(ledger, options, &reason.detail())?;
            return Ok(());
        }
    };
    let manifest = BuiltinManifest::compiled();
    manifest.validate()?;
    let thread_folder = ledger
        .path()
        .parent()
        .ok_or("worker ledger has no thread folder")?
        .to_path_buf();
    let resolved_profile = match provider::resolve_profile(provider_config, model) {
        Ok(profile) => profile,
        Err(error) => {
            settle_provider_unavailable(ledger, options, &format!("config: {error}"))?;
            return Ok(());
        }
    };
    let dialect = resolved_profile.dialect;
    // Native deferred-tool routing is a property of the exact route (dialect,
    // model, endpoint owner, gateway translation) declared in the reviewed
    // capability catalog, never of the adapter family.
    let native_mode = resolved_profile.native_deferred_tools();
    let mut native_tools: Option<provider::NativeDeferredTools> = None;
    let provider_catalog = {
        let (backend, dynamic_backend) = assemble_tool_backends(
            ToolBackendPlan {
                profile,
                selected: *selected,
                thread_folder: &thread_folder,
                invocation_write_roots: &[],
                credential: credential.as_ref(),
                web_search_ready: options.web_search_ready,
            },
            ToolBackendRuntime {
                lines,
                stdout,
                cancellation,
            },
        )?;
        let dynamic_dispatcher = DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog);
        let deferred = dynamic_dispatcher.deferred_search_entries(&dynamic_backend)?;
        let has_deferred = !deferred.is_empty();
        let catalog_context =
            tool_catalog_context(profile, has_deferred, options.web_search_ready)?;
        let mut catalog =
            ToolDispatcher::new(&manifest, &catalog_context).provider_catalog(&backend)?;
        let before_seq = ledger.next_seq();
        let pipeline = ToolPipeline::new(
            ledger,
            AssetStore::new(thread_folder.join("assets"))?,
            SecretScanner::default(),
        );
        catalog.extend(dynamic_dispatcher.provider_catalog_visible(
            &dynamic_backend,
            &pipeline,
            before_seq,
        )?);
        if native_mode.is_some() && has_deferred {
            // Native mode declares every deferred schema to the provider (the
            // adapter marks them deferred) and carries this turn's durable
            // search offers as bound references; nothing else changes.
            let declared = catalog
                .iter()
                .filter_map(|schema| {
                    serde_json::to_value(schema)
                        .ok()?
                        .get("name")?
                        .as_str()
                        .map(str::to_owned)
                })
                .collect::<BTreeSet<_>>();
            for entry in &deferred {
                if !declared.contains(&entry.name) {
                    catalog.push(entry.content.clone());
                }
            }
            let catalog_revision = dynamic_catalog_revision(&deferred);
            drop(pipeline);
            native_tools = Some(provider::NativeDeferredTools {
                references: native_search_references(
                    ledger,
                    turn,
                    before_seq,
                    &deferred,
                    &catalog_revision,
                )?,
                catalog_revision,
                search_tool_name: "tool_search".to_owned(),
                definitions: deferred
                    .iter()
                    .map(|entry| provider::DeferredToolDefinition {
                        schema: entry.content.clone(),
                        schema_digest: entry.schema_digest.clone(),
                    })
                    .collect(),
            });
        }
        catalog
    };
    let tool_catalog = ijson(Value::Array(
        provider_catalog
            .into_iter()
            .map(|value| serde_json::to_value(value).expect("tool schema is JSON"))
            .collect(),
    ))?;
    let tools_digest = format!("{:x}", Sha256::digest(tool_catalog.canonical_bytes()?));
    let consumed = current_turn_inputs(ledger, turn)?;
    // The root prompt is the runtime-owned identity, profile text and working
    // directory followed by the AGENTS.md scopes (spec builtin-tools
    // §Model-facing tool guidance). The validator
    // keeps its own judge prompt.
    let system = if profile.validator {
        validation_runtime::JUDGE_SYSTEM.to_owned()
    } else {
        let identity = identity::resolve(ledger, &options.event_timestamp(), turn)?;
        let root = tools::root_system_instructions(
            identity,
            resolved_profile.wire_model(),
            profile.config.execution_cwd(),
            &effective_system(&profile.instruction),
        );
        let root = if let Some(bound_id) = profile.bindings.goal_id.as_deref() {
            if let Some((storage_root, session)) = goal_store_location(ledger)? {
                match session_controls::read_goal(&storage_root, &session)? {
                    Some(goal) if goal.id == bound_id => format!(
                        "{root}\n\nSession goal (authoritative, separate from conversation history):\nObjective: {}\nState: {}\nContinue pursuing this objective while it is active. A final answer ends only this turn. Mark the whole goal complete with set_goal_state only when its objective is achieved.",
                        goal.objective, goal.phase
                    ),
                    _ => root,
                }
            } else {
                root
            }
        } else {
            root
        };
        if profile.subagent {
            format!(
                "{root}\n\nYou are a delegated subagent. When your work is complete, call report with one self-contained result for your parent. Final prose alone does not deliver your result to the parent."
            )
        } else {
            root
        }
    };
    let profile_value = provider::epoch_profile(
        &resolved_profile,
        &system,
        profile.config.session_settings.as_ref(),
    )?;
    let profile_digest = format!("{:x}", Sha256::digest(profile_value.canonical_bytes()?));
    let mut epoch = (!reset_epoch)
        .then(|| {
            latest_compatible_epoch(
                ledger,
                dialect,
                &model.id,
                &profile_digest,
                &tools_digest,
                CONTEXT_RENDERER_VERSION,
            )
        })
        .flatten();
    if epoch.is_none() {
        epoch = Some(append_provider_epoch(
            ledger,
            EpochAppend {
                options,
                dialect,
                model: &model.id,
                profile: &profile_value,
                tools: &tool_catalog,
                reason: {
                    let events = ledger
                        .projection()
                        .map(|p| p.events.as_slice())
                        .unwrap_or_default();
                    let previous = events.iter().rev().find(|e| *e.kind() == EventKind::Epoch);
                    let compacted_after = previous.is_some_and(|epoch| {
                        events
                            .iter()
                            .any(|e| *e.kind() == EventKind::Compact && e.seq() > epoch.seq())
                    });
                    epoch_open_reason(
                        previous,
                        compacted_after,
                        reset_epoch,
                        &tools_digest,
                        &profile_digest,
                    )
                },
                pending: &consumed,
            },
        )?);
    }
    let epoch = epoch.expect("epoch established");
    // Native search replay is client-managed: the whole prefix renders and no
    // server continuation chain is used, even on a server-managed dialect.
    if let Some(hooks) = &options.lifecycle_hooks {
        hooks.observe(ledger);
    }
    let mut context =
        project_provider_context_mode(ledger, &epoch, dialect, turn, native_tools.is_some())?;
    if let Some(hooks) = &options.lifecycle_hooks {
        ledger.sync_prefix()?;
        for text in hooks.prepare(ledger, turn, &context.items) {
            context.items.push(IJsonValue::parse(&serde_json::to_vec(&json!({
                "role": "user", "content": [{"type": "text", "text": format!("Reference context from a host extension (not user instructions):\n{text}")}]
            }))?)?);
        }
    }
    if context.items.is_empty() && context.continuation_id.is_none() {
        // Nothing model-visible to send for an open turn. Settling it as an
        // internal failure keeps the tail terminal instead of leaving a live
        // worker with nothing to do.
        return Err(format!("turn {turn} rendered no model-visible context").into());
    }
    let attempt = format!("{}-attempt-{}", options.run_id, ledger.next_seq());
    let prepare_input = PrepareInput {
        attempt_id: attempt.clone(),
        target: resolved_profile.target.clone(),
        endpoint: provider_config.endpoint.clone(),
        epoch_profile: profile_value.clone(),
        continuation_id: context.continuation_id.clone(),
        rendered_items: context.items,
        tool_catalog: tool_catalog.clone(),
        stream: true,
    };
    let prepared_result = if let Some(native) = native_tools.as_ref() {
        provider::prepare_with_native_deferred_tools(&prepare_input, None, native)
    } else {
        #[cfg(not(test))]
        {
            prepare(&prepare_input)
        }
        #[cfg(test)]
        {
            provider_context_tests::prepare_live_choice(ledger, turn, prepare_input)
        }
    };
    let prepared = match prepared_result {
        Ok(prepared) => prepared,
        Err(error) => {
            settle_provider_unavailable(ledger, options, &format!("config: {error}"))?;
            return Ok(());
        }
    };
    let transport_endpoint = options.provider_test_redirect.as_deref();
    // The exact transmitted body is durable thread history (secret-free by
    // the prepare contract): published as a content-addressed asset before the
    // attempt event that references it (provider-runtime §Send ordering).
    let request_asset = {
        let thread_folder = ledger
            .path()
            .parent()
            .ok_or("worker ledger has no thread folder")?;
        store::AssetStore::new(thread_folder.join("assets"))?.publish(&prepared.body)?
    };
    // Live evidence capture (secret-free bodies), enabled only by an explicit
    // environment the supervisor forwards.
    if let Ok(directory) = std::env::var("TEKES_KERNEL_LIVE_ARTIFACT") {
        fs::write(
            PathBuf::from(directory).join(format!("{attempt}.request.json")),
            &prepared.body,
        )?;
    }
    #[cfg(test)]
    let thread_transport_endpoint = PROVIDER_TEST_REDIRECT.with(|slot| slot.borrow().clone());
    #[cfg(test)]
    let transport_endpoint = thread_transport_endpoint.as_deref().or(transport_endpoint);
    let origin = match endpoint_origin(&provider_config.endpoint) {
        Ok(origin) => origin,
        Err(error) => {
            settle_provider_unavailable(ledger, options, &format!("config: {error}"))?;
            return Ok(());
        }
    };
    let cancelled = Arc::clone(&cancellation.provider);
    // Material is resolved for this attempt and dropped (zeroed) when the
    // attempt settles; rotation and revocation reach the next attempt's get.
    let credential_material = match (&provider_config.credential_key, credential.as_ref()) {
        (Some(key), Some(client)) => match client
            .lock()
            .map_err(|_| "credential client lock poisoned")?
            .get(CredentialGet {
                request_id: credential_request_id(&attempt, key, &origin),
                attempt: attempt.clone(),
                credential_id: key.clone(),
                purpose: "provider".to_owned(),
                adapter: provider_config.adapter.clone(),
                endpoint_origin: origin,
            }) {
            Ok(material) => Some(material),
            Err(provider::CredentialClientError::Rejected(code)) => {
                settle_provider_unavailable(ledger, options, &format!("credential_{code}"))?;
                return Ok(());
            }
            Err(error) => {
                settle_provider_unavailable(
                    ledger,
                    options,
                    &format!("config: credential broker: {error}"),
                )?;
                return Ok(());
            }
        },
        (Some(_), None) => {
            settle_provider_unavailable(
                ledger,
                options,
                "config: configured provider requires a credential broker",
            )?;
            return Ok(());
        }
        (None, _) => None,
    };
    drain_parked_deliveries(ledger, options, run.selected, cancellation, stdout)?;
    if latest_compatible_epoch(
        ledger,
        dialect,
        &model.id,
        &profile_digest,
        &tools_digest,
        CONTEXT_RENDERER_VERSION,
    )
    .as_deref()
        != Some(epoch.as_str())
    {
        drop(credential_material);
        return run_provider_turn_inner(
            ledger,
            run,
            credential,
            lines,
            stdout,
            budget,
            cancellation,
        );
    }
    // D-39 preflight: a compatible provider-reported usage anchors the token
    // estimate; each appended byte counts as a possible token. If no such
    // observation exists, retain the conservative byte bound. A candidate
    // above the trigger compacts before send; provider overflow remains the
    // hard limit. The attempt's credential covers the summary request.
    // Before paying for a summary, shorten the tool output that made the
    // request large. Trimming makes no model call and loses no history (the
    // original stays on disk). If the request still does not fit afterwards,
    // the next pass summarizes completed older attempts, including those in
    // the open turn, while retaining anchors and the current batch.
    let compact_due = preflight_compaction_due(ledger, &prepared, model, &epoch);
    if budget.trims_left > 0 && compact_due {
        let trims = engine::plan_tool_result_trim(
            &ledger
                .projection()
                .ok_or("ledger projection missing")?
                .events,
        )?;
        if !trims.is_empty() {
            drop(credential_material);
            for trim in &trims {
                append_tool_result_trim(ledger, options, turn, trim)?;
            }
            append_provider_epoch(
                ledger,
                EpochAppend {
                    options,
                    dialect,
                    model: &model.id,
                    profile: &profile_value,
                    tools: &tool_catalog,
                    reason: "tool_result_trim",
                    pending: &consumed,
                },
            )?;
            announce_appended(ledger, stdout)?;
            return run_provider_turn_inner(
                ledger,
                run,
                credential,
                lines,
                stdout,
                ProviderLoopBudget {
                    trims_left: budget.trims_left - 1,
                    ..budget
                },
                cancellation,
            );
        }
    }
    if budget.compactions_left > 0
        && compact_due
        && !engine::plan_context_compaction(
            &ledger
                .projection()
                .ok_or("ledger projection missing")?
                .events,
            turn,
        )?
        .covers
        .is_empty()
    {
        let summary = run_compaction_summary(
            ledger,
            profile,
            provider_config,
            model,
            &resolved_profile,
            credential_material
                .as_ref()
                .map_or("", |material| material.material.as_str()),
            transport_endpoint,
            &cancelled,
            turn,
            &attempt,
        )?;
        drop(credential_material);
        if auto_compact_for_turn(ledger, options, turn, summary)? {
            append_provider_epoch(
                ledger,
                EpochAppend {
                    options,
                    dialect,
                    model: &model.id,
                    profile: &profile_value,
                    tools: &tool_catalog,
                    reason: "compaction",
                    pending: &consumed,
                },
            )?;
            announce_appended(ledger, stdout)?;
        }
        return run_provider_turn_inner(
            ledger,
            run,
            credential,
            lines,
            stdout,
            ProviderLoopBudget {
                compactions_left: budget.compactions_left - 1,
                ..budget
            },
            cancellation,
        );
    }
    stdout.write_all(&encode_line(
        "lease_request",
        &LeaseRequest {
            attempt: attempt.clone(),
            class: "provider".to_owned(),
        },
    )?)?;
    stdout.flush()?;
    let granted = loop {
        let line = lines
            .next()
            .ok_or_else(|| {
                ProtocolFailure("supervisor EOF while awaiting provider lease".to_owned())
            })?
            .map_err(|error| {
                ProtocolFailure(format!(
                    "supervisor read failed while awaiting lease: {error}"
                ))
            })?;
        match decode_supervisor(line.as_bytes())? {
            SupervisorMessage::Lease(lease) if lease.attempt == attempt => break lease.granted,
            SupervisorMessage::Ping(ping) => {
                stdout.write_all(&encode_line("pong", &worker_control::Pong { id: ping.id })?)?;
                stdout.flush()?;
            }
            SupervisorMessage::Stop(_) => {
                cancellation.cancel(CANCEL_STOP);
                cancellation.defer(line);
                break false;
            }
            SupervisorMessage::QueueTransaction(_) => cancellation.park_delivery(line),
            _ => {
                return Err(Box::new(ProtocolFailure(
                    "unexpected supervisor message while awaiting provider lease".to_owned(),
                )));
            }
        }
    };
    if !granted {
        // Denial is the supervisor draining (or a stop that arrived while waiting).
        // The turn stays open without an attempt; the next sweep classifies it as
        // recovery_needed and a fresh run continues or reconciles it.
        eprintln!(
            "tekes-worker: provider lease denied for turn {turn}; leaving the tail for recovery"
        );
        drop(credential_material);
        return Ok(());
    }

    let attempt_event = make_event(json!({
        "v": 1,
        "seq": ledger.next_seq(),
        "turn": turn,
        "kind": "attempt",
        "ts": options.event_timestamp(),
        "attempt": attempt,
        "epoch": epoch,
        "wire_digest": prepared.request_digest,
        "request": {"asset": request_asset.asset, "bytes": request_asset.bytes},
        "admits": ranges(&context.admits)
    }))?;
    ledger.append_contract(attempt_event, BarrierContext::default())?;
    if prepared.dispatch_marker_required {
        let dispatched = make_event(json!({
            "v": 1,
            "seq": ledger.next_seq(),
            "turn": turn,
            "kind": "attempt_dispatched",
            "ts": options.event_timestamp(),
            "attempt": attempt
        }))?;
        ledger.append_contract(dispatched, BarrierContext::default())?;
    }

    let mut eager = EagerDispatch::default();
    let completion = if cancelled.load(std::sync::atomic::Ordering::Acquire) {
        ProviderCompletion::Failure(ProviderFailure::Cancelled)
    } else {
        // The request runs on its own thread so this thread keeps the ledger and
        // stdout: frames are forwarded as they arrive, and deliveries the reader
        // thread parked are appended and receipted while the provider is still
        // streaming (R2-8; tail-lifecycle audit gap J). A failure on this side
        // cancels the request and is reported after the thread has joined.
        let credential_material = credential_material
            .as_ref()
            .map_or("", |material| material.material.as_str());
        let wall = budget
            .wall_deadline
            .map(|deadline| deadline.saturating_duration_since(Instant::now()));
        let (frame_sender, frame_receiver) = std::sync::mpsc::channel::<ProviderFrame>();
        let prepared = &prepared;
        let capture_path = std::env::var("TEKES_KERNEL_LIVE_ARTIFACT")
            .ok()
            .map(|directory| {
                PathBuf::from(directory).join(format!("{attempt}.response.partial.raw"))
            });
        let cancelled_flag = &cancelled;
        std::thread::scope(
            |scope| -> Result<ProviderCompletion, Box<dyn std::error::Error>> {
                let request = scope.spawn(move || {
                    let runtime = HttpRuntime::new()?;
                    let capture = Arc::new(Mutex::new(Vec::new()));
                    let runtime = if capture_path.is_some() {
                        runtime.with_response_capture(Arc::clone(&capture))
                    } else {
                        runtime
                    };
                    let result = runtime.send_dialect_with_frames_and_wall_transport(
                        dialect,
                        prepared,
                        transport_endpoint,
                        credential_material,
                        cancelled_flag,
                        wall,
                        move |frame| {
                            frame_sender.send(frame).map_err(|_| {
                                provider::HttpRuntimeError::Request(
                                    "provider frame receiver closed".to_owned(),
                                )
                            })
                        },
                    );
                    if let Some(path) = capture_path {
                        let body = capture.lock().expect("response capture lock");
                        fs::write(path, &*body).map_err(|error| {
                            provider::HttpRuntimeError::Request(error.to_string())
                        })?;
                    }
                    result
                });
                let mut block = 0_u64;
                let mut main_failure: Option<Box<dyn std::error::Error>> = None;
                loop {
                    let frame = match frame_receiver.recv_timeout(Duration::from_millis(20)) {
                        Ok(frame) => Some(frame),
                        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => None,
                        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                    };
                    if main_failure.is_some() {
                        continue;
                    }
                    let step = (|| -> Result<(), Box<dyn std::error::Error>> {
                        if let Some(mut frame) = frame {
                            if let ProviderFrame::ToolCallReady(call) = &mut frame {
                                repair_provider_call_arguments(dialect, call);
                            }
                            localize_provider_frame(ledger, &attempt, &mut eager, &mut frame)?;
                            let ledger_seq =
                                ledger.projection().map(|projection| projection.last_seq);
                            if let Some(call) = forward_provider_frame(
                                ledger_seq, &attempt, &mut block, frame, stdout,
                            )? {
                                // Eager dispatch: the call's arguments are complete
                                // and validated, so it executes now, while the
                                // response keeps streaming on its own thread. The
                                // terminal later has to name it unchanged.
                                eager_dispatch_ready_call(
                                    ledger,
                                    EagerReadyCall {
                                        options,
                                        profile,
                                        selected: *selected,
                                        manifest: &manifest,
                                        attempt: &attempt,
                                        turn,
                                        cancellation,
                                    },
                                    call,
                                    &mut eager,
                                    credential.as_ref(),
                                    lines,
                                    stdout,
                                )?;
                            }
                        }
                        drain_parked_deliveries(ledger, options, run.selected, cancellation, stdout)
                    })();
                    if let Err(error) = step {
                        cancelled_flag.store(true, std::sync::atomic::Ordering::Release);
                        main_failure = Some(error);
                    }
                }
                let completion = request
                    .join()
                    .map_err(|_| "provider request thread panicked")??;
                if let Some(failure) = main_failure {
                    return Err(failure);
                }
                Ok(completion)
            },
        )?
    };

    if let (Ok(directory), ProviderCompletion::Terminal(terminal)) =
        (std::env::var("TEKES_KERNEL_LIVE_ARTIFACT"), &completion)
    {
        fs::write(
            PathBuf::from(directory).join(format!("{attempt}.response.raw")),
            &terminal.raw_response,
        )?;
    }
    if let ProviderCompletion::Terminal(terminal) = &completion {
        if let Some(changes) = &terminal.input_transformations {
            record_input_transformations(
                ledger,
                &options.event_timestamp(),
                turn,
                &attempt,
                changes,
            )?;
        }
    }
    let runtime_cancelled_provider = matches!(
        &completion,
        ProviderCompletion::Failure(ProviderFailure::Cancelled)
    );
    let outcome = match completion {
        ProviderCompletion::Terminal(terminal)
            if matches!(
                terminal.finish_reason,
                FinishReason::ContentFilter | FinishReason::ProviderError
            ) =>
        {
            append_provider_terminal_error(ledger, options, turn, &attempt, &terminal)?
        }
        ProviderCompletion::Terminal(terminal)
            if terminal.finish_reason == FinishReason::ContextOverflow =>
        {
            append_context_overflow(ledger, options, turn, &attempt, terminal.usage.as_ref())?
        }
        ProviderCompletion::Terminal(terminal) => append_terminal(
            ledger,
            TerminalAppend {
                options,
                manifest: &manifest,
                eager: &eager,
                dialect,
                server_managed: dialect.server_managed(),
                turn,
                attempt: &attempt,
            },
            terminal,
        )?,
        ProviderCompletion::Failure(failure) => append_provider_failure(
            ledger,
            options,
            turn,
            &attempt,
            failure,
            cancellation,
            dialect,
            &eager,
        )?,
    };
    stdout.write_all(&encode_line(
        "attempt_settled",
        &AttemptSettled {
            attempt: attempt.clone(),
            outcome_seq: outcome.seq,
        },
    )?)?;
    stdout.flush()?;
    // The summary request reuses this attempt's lease before it is released;
    // the compact it feeds is written below on the same plan.
    let compaction_summary = if outcome.compact_retry
        && budget.compactions_left > 0
        && !cancelled.load(std::sync::atomic::Ordering::Acquire)
    {
        run_compaction_summary(
            ledger,
            profile,
            provider_config,
            model,
            &resolved_profile,
            credential_material
                .as_ref()
                .map_or("", |material| material.material.as_str()),
            transport_endpoint,
            &cancelled,
            turn,
            &attempt,
        )?
    } else {
        None
    };
    drop(credential_material);
    if runtime_cancelled_provider && cancellation.supervisor_lost() {
        return Err(Box::new(ProtocolFailure(
            "supervisor EOF during provider request".to_owned(),
        )));
    }
    if runtime_cancelled_provider && cancellation.stop_requested() {
        // The attempt is terminal (usage + error are durable). The settle belongs to
        // `post_turn_exit`, which first echoes the durable `stop_requested` for the
        // stop still sitting in the control channel and then settles `user_stop`
        // (tail-lifecycle audit I11: a stop is durable before it takes semantic effect).
        return Ok(());
    }
    if outcome.compact_retry {
        if budget.compactions_left > 0
            && auto_compact_for_turn(ledger, options, turn, compaction_summary)?
        {
            append_provider_epoch(
                ledger,
                EpochAppend {
                    options,
                    dialect,
                    model: &model.id,
                    profile: &profile_value,
                    tools: &tool_catalog,
                    reason: "compaction",
                    pending: &consumed,
                },
            )?;
            return run_provider_turn_inner(
                ledger,
                run,
                credential,
                lines,
                stdout,
                ProviderLoopBudget {
                    compactions_left: budget.compactions_left - 1,
                    ..budget
                },
                cancellation,
            );
        }
        append_settle(
            ledger,
            &options.event_timestamp(),
            turn,
            "interrupted",
            Some("budget_tokens"),
        )?;
        return Ok(());
    }
    if outcome.retry {
        if budget.retries_left == 0 {
            append_settle(
                ledger,
                &options.event_timestamp(),
                turn,
                "error",
                Some(outcome.retry_classification.unwrap_or("internal")),
            )?;
            return Ok(());
        }
        if let Some(classification @ ("rate_limit" | "transport")) = outcome.retry_classification {
            wait_provider_admission(
                ledger,
                options,
                turn,
                &attempt,
                ProviderAdmissionRetry {
                    classification,
                    retry_after_seconds: outcome.retry_after_seconds,
                    retries_left: budget.retries_left,
                },
                cancellation,
            )?;
            if cancellation.stop_requested() || cancellation.supervisor_lost() {
                return Ok(());
            }
        }
        if dialect.server_managed() {
            append_provider_epoch(
                ledger,
                EpochAppend {
                    options,
                    dialect,
                    model: &model.id,
                    profile: &profile_value,
                    tools: &tool_catalog,
                    reason: "recovery",
                    pending: &consumed,
                },
            )?;
        }
        return run_provider_turn_inner(
            ledger,
            run,
            credential,
            lines,
            stdout,
            ProviderLoopBudget {
                retries_left: budget.retries_left - 1,
                ..budget
            },
            cancellation,
        );
    }
    if !outcome.tool_calls.is_empty() {
        if eager.suspended {
            // Approval can arrive inline before the provider terminal. The
            // earlier park flag is not current lifecycle state: a durable
            // answer and no execution start means this worker can finish the
            // head and its siblings now, without exiting for crash recovery.
            let answered_inline = pending_tool_calls(ledger)?.iter().any(|call| {
                eager.contains(&call.call)
                    && call.execution_tracked
                    && call.approval_response.is_some()
                    && !call.execution_started
            });
            if !answered_inline
                || cancellation.stop_requested()
                || cancellation.supervisor_lost()
                || ledger
                    .projection()
                    .ok_or("ledger projection missing")?
                    .lifecycle
                    .open_hold
            {
                return Ok(());
            }
        }
        // Fan-in: eagerly dispatched calls already own their single result.
        // Only the remainder of the batch executes here, and the continuation
        // below waits for the whole batch either way.
        let paired = ledger
            .projection()
            .ok_or("ledger projection missing")?
            .events
            .iter()
            .filter(|event| *event.kind() == EventKind::ToolResult)
            .filter_map(|event| event.string_field("call").map(str::to_owned))
            .collect::<BTreeSet<_>>();
        let remaining = outcome
            .tool_calls
            .iter()
            .filter(|call| !paired.contains(&call.call_id))
            .cloned()
            .collect::<Vec<_>>();
        if execute_provider_tool_calls(
            ledger,
            ToolCallBatch {
                options,
                profile,
                selected: *selected,
                attempt: &attempt,
                turn,
                calls: &remaining,
                cancellation,
            },
            credential.as_ref(),
            lines,
            stdout,
        )? {
            return Ok(());
        }
        if cancellation.stop_requested() {
            return Ok(());
        }
        if cancellation.supervisor_lost() || cancellation.protocol_failed() {
            return Err(Box::new(ProtocolFailure(
                "supervisor lost during tool execution".to_owned(),
            )));
        }
        drain_parked_deliveries(ledger, options, run.selected, cancellation, stdout)?;
        return run_provider_turn_inner(
            ledger,
            run,
            credential,
            lines,
            stdout,
            budget,
            cancellation,
        );
    }
    if let Some((settle_outcome, reason)) = outcome.settle {
        if settle_outcome == "completed" {
            if validation_runtime::advance(ledger, run, lines, stdout, cancellation)? {
                return Ok(());
            }
            // Failed validation and an unmet validator mandate resume the same
            // session, without treating the provider's final as turn settlement.
            return run_provider_turn_inner(
                ledger,
                run,
                credential,
                lines,
                stdout,
                ProviderLoopBudget {
                    nonfinal_responses_left: budget.nonfinal_responses_left.saturating_sub(1),
                    ..budget
                },
                cancellation,
            );
        }
        append_settle(
            ledger,
            &options.event_timestamp(),
            turn,
            settle_outcome,
            reason,
        )?;
        return Ok(());
    }
    if cancellation.stop_requested() {
        return Ok(());
    }
    if budget.nonfinal_responses_left == 0 {
        settle_provider_unavailable(
            ledger,
            options,
            "provider produced no final answer within the continuation budget",
        )?;
        return Ok(());
    }
    drain_parked_deliveries(ledger, options, run.selected, cancellation, stdout)?;
    run_provider_turn_inner(
        ledger,
        run,
        credential,
        lines,
        stdout,
        ProviderLoopBudget {
            nonfinal_responses_left: budget.nonfinal_responses_left - 1,
            ..budget
        },
        cancellation,
    )
}

/// Mirrors one accepted provider frame to the supervisor. Runs on the worker's
/// main thread, the only stdout writer, while the request itself streams on its
/// own thread. Returns the validated call when the frame was the provider's
/// per-call argument completion, so the caller can dispatch it before the
/// response terminal arrives; the presentation marker itself has already left
/// the pipe by then, ahead of the durable `tool_call`.
fn forward_provider_frame(
    ledger_seq: Option<u64>,
    attempt: &str,
    block: &mut u64,
    frame: ProviderFrame,
    stdout: &mut impl Write,
) -> Result<Option<provider::ToolCall>, Box<dyn std::error::Error>> {
    let mut arguments_complete = None;
    let mut ready = None;
    let (channel, delta, call_id, name) = match frame {
        ProviderFrame::TextDelta(delta) => (FrameChannel::Text, delta, None, None),
        ProviderFrame::ReasoningDelta(delta) => (FrameChannel::Reasoning, delta, None, None),
        ProviderFrame::ToolDelta {
            call_id,
            name,
            delta,
        } => (FrameChannel::Tool, delta, Some(call_id), name),
        ProviderFrame::ToolCallReady(call) => {
            arguments_complete = Some(true);
            let (call_id, name) = (call.call_id.clone(), call.name.clone());
            ready = Some(call);
            (FrameChannel::Tool, String::new(), Some(call_id), Some(name))
        }
        // Live reports are presentation-only; the terminal remains the accounting authority.
        ProviderFrame::UsageDelta(usage) => (
            FrameChannel::Usage,
            serde_json::to_string(&usage_object(Some(&usage)))?,
            None,
            None,
        ),
        ProviderFrame::Status(_) => return Ok(None),
    };
    stdout.write_all(&encode_line(
        "frame",
        &Frame {
            arguments_complete,
            ledger_seq,
            attempt: attempt.to_owned(),
            channel,
            block: *block,
            delta,
            call_id,
            name,
        },
    )?)?;
    stdout.flush()?;
    *block = block.saturating_add(1);
    Ok(ready)
}

/// Calls this attempt dispatched before its response terminal (eager
/// dispatch). Each entry is already a durable `tool_call`; execution ran, or
/// was suspended, through the same pipeline the post-terminal batch uses.
#[derive(Default)]
pub(crate) struct EagerDispatch {
    pub(crate) wire_ids: std::collections::BTreeMap<String, String>,
    pub(crate) calls: Vec<provider::ToolCall>,
    /// A dispatched call parked the turn (hold, spawn) or a stop arrived. No
    /// further call executes in this run; the terminal still lands durably so
    /// the resumed run finds the whole batch exactly as a late batch would.
    pub(crate) suspended: bool,
}

impl EagerDispatch {
    pub(crate) fn contains(&self, call_id: &str) -> bool {
        self.calls.iter().any(|call| call.call_id == call_id)
    }
}

/// Provider IDs identify calls within a response, not across a durable ledger.
/// A new response may reuse a completed call's wire ID, even in visible history.
/// Keep native carriers untouched and use a separate durable identity locally.
pub(crate) fn local_provider_call_id(
    ledger: &LockedLedger,
    attempt: &str,
    wire_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let events = &ledger
        .projection()
        .ok_or("ledger projection missing")?
        .events;
    let previous = events
        .iter()
        .filter(|event| {
            *event.kind() == EventKind::ToolCall
                && event
                    .string_field("provider_call")
                    .or_else(|| event.string_field("call"))
                    == Some(wire_id)
        })
        .collect::<Vec<_>>();
    if let Some(call) = previous
        .iter()
        .find(|event| event.string_field("attempt") == Some(attempt))
    {
        return Ok(call
            .string_field("call")
            .ok_or("tool_call lacks call")?
            .to_owned());
    }
    if previous.is_empty() {
        return Ok(wire_id.to_owned());
    }
    for call in previous {
        let id = call.string_field("call").ok_or("tool_call lacks call")?;
        let owner = call.string_field("attempt");
        let related = events
            .iter()
            .filter(|event| {
                event.seq() == call.seq()
                    || event.string_field("call") == Some(id)
                        && matches!(event.kind(), EventKind::ToolResult | EventKind::ChildResult)
                    || *event.kind() == EventKind::Output && event.string_field("attempt") == owner
            })
            .collect::<Vec<_>>();
        if !related
            .iter()
            .any(|event| *event.kind() == EventKind::ToolResult)
            || !related
                .iter()
                .any(|event| *event.kind() == EventKind::Output)
        {
            return Err(format!(
                "provider tool call id {wire_id} reuses an unresolved durable call"
            )
            .into());
        }
    }
    let digest = Sha256::digest(serde_json_canonicalizer::to_vec(&json!([
        attempt, wire_id
    ]))?);
    let local_id = format!("provider-call-{digest:x}");
    if events.iter().any(|event| {
        *event.kind() == EventKind::ToolCall && event.string_field("call") == Some(&local_id)
    }) {
        return Err("scoped provider call identity collision".into());
    }
    Ok(local_id)
}

pub(crate) fn repair_provider_call_arguments(dialect: DialectId, call: &mut provider::ToolCall) {
    if !matches!(
        dialect,
        DialectId::GlmChatV1 | DialectId::DeepseekResponsesV1
    ) {
        return;
    }
    if let Some(arguments) = tools::fixed_schema(&call.name)
        .and_then(|schema| schema.repair_quoted_null_exclusive(&call.arguments))
    {
        // Only the normalized invocation changes. The terminal's sealed native
        // carrier retains the provider's original argument bytes for replay.
        call.arguments = arguments;
    }
}

pub(crate) fn localize_provider_frame(
    ledger: &LockedLedger,
    attempt: &str,
    eager: &mut EagerDispatch,
    frame: &mut ProviderFrame,
) -> Result<(), Box<dyn std::error::Error>> {
    let id = match frame {
        ProviderFrame::ToolDelta { call_id, .. } => call_id,
        ProviderFrame::ToolCallReady(call) => &mut call.call_id,
        _ => return Ok(()),
    };
    let wire_id = id.clone();
    if let Some((local_id, _)) = eager.wire_ids.iter().find(|(_, wire)| *wire == &wire_id) {
        *id = local_id.clone();
    } else {
        *id = local_provider_call_id(ledger, attempt, &wire_id)?;
        eager.wire_ids.insert(id.clone(), wire_id);
    }
    Ok(())
}

pub(crate) struct EagerReadyCall<'a> {
    pub(crate) options: &'a Options,
    pub(crate) profile: &'a RuntimeProfile,
    pub(crate) selected: Selected,
    pub(crate) manifest: &'a BuiltinManifest,
    pub(crate) attempt: &'a str,
    pub(crate) turn: u64,
    pub(crate) cancellation: &'a RuntimeCancellation,
}

/// Dispatches one provider call as soon as its arguments are complete, while the
/// response is still streaming. The durable `tool_call` is the write-ahead
/// intent exactly as for a post-terminal batch; the response terminal must later
/// name this call with the same identity, name and arguments, and the
/// post-terminal batch skips it. A parked or stopped dispatch suspends eager
/// execution for the rest of the response without cancelling the request.
pub(crate) fn eager_dispatch_ready_call<I, O>(
    ledger: &mut LockedLedger,
    ready: EagerReadyCall<'_>,
    call: provider::ToolCall,
    eager: &mut EagerDispatch,
    credential: Option<&Arc<Mutex<CredentialClient>>>,
    lines: &mut I,
    stdout: &mut O,
) -> Result<(), Box<dyn std::error::Error>>
where
    I: Iterator<Item = io::Result<String>>,
    O: Write,
{
    if eager.suspended {
        return Ok(());
    }
    let EagerReadyCall {
        options,
        profile,
        selected,
        manifest,
        attempt,
        turn,
        cancellation,
    } = ready;
    if call.call_id.is_empty() || eager.contains(&call.call_id) {
        return Err(format!(
            "provider tool call id {} is empty or duplicated",
            call.call_id
        )
        .into());
    }
    let reused = ledger
        .projection()
        .ok_or("ledger projection missing")?
        .events
        .iter()
        .any(|event| {
            *event.kind() == EventKind::ToolCall
                && event.string_field("call") == Some(call.call_id.as_str())
        });
    if reused {
        return Err(format!(
            "provider tool call id {} reuses a durable call",
            call.call_id
        )
        .into());
    }
    append_provider_tool_call_with_wire_id(
        ledger,
        options,
        manifest,
        turn,
        attempt,
        &call,
        eager.wire_ids.get(&call.call_id).map(String::as_str),
    )?;
    announce_appended(ledger, stdout)?;
    let parked = execute_provider_tool_calls(
        ledger,
        ToolCallBatch {
            options,
            profile,
            selected,
            attempt,
            turn,
            calls: std::slice::from_ref(&call),
            cancellation,
        },
        credential,
        lines,
        stdout,
    )?;
    eager.calls.push(call);
    if parked {
        eager.suspended = true;
    }
    announce_appended(ledger, stdout)?;
    Ok(())
}

#[cfg(test)]
pub(crate) fn append_provider_tool_call(
    ledger: &mut LockedLedger,
    options: &Options,
    manifest: &BuiltinManifest,
    turn: u64,
    attempt: &str,
    call: &provider::ToolCall,
) -> Result<(), Box<dyn std::error::Error>> {
    append_provider_tool_call_with_wire_id(ledger, options, manifest, turn, attempt, call, None)
}

pub(crate) fn append_provider_tool_call_with_wire_id(
    ledger: &mut LockedLedger,
    options: &Options,
    manifest: &BuiltinManifest,
    turn: u64,
    attempt: &str,
    call: &provider::ToolCall,
    wire_id: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let effect = manifest
        .tools
        .iter()
        .find(|tool| tool.name == call.name)
        .map(|tool| engine::side_effectful(tool.effect))
        .unwrap_or(true);
    let arguments = spill_json(ledger, &call.arguments)?;
    let mut raw = json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"tool_call",
        "ts":options.event_timestamp(),"attempt":attempt,"call":call.call_id,
        "name":call.name,"args":arguments,"source":"provider","execution_tracked":true
    });
    if let Some(wire_id) = wire_id.filter(|id| *id != call.call_id) {
        raw["provider_call"] = json!(wire_id);
    }
    let event = make_event(raw)?;
    ledger.append_contract(
        event,
        BarrierContext {
            side_effectful_tool_call: effect,
        },
    )?;
    Ok(())
}

fn settle_provider_unavailable(
    ledger: &mut LockedLedger,
    options: &Options,
    detail: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let turn = ledger
        .projection()
        .and_then(|projection| projection.latest_turn)
        .ok_or("provider failure requires turn_open")?;
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"error",
        "ts":options.event_timestamp(),"recoverable":false,
        "classification":"provider_terminal","detail":bounded_ledger_detail(detail)
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    append_settle(
        ledger,
        &options.event_timestamp(),
        turn,
        "error",
        Some("provider_terminal"),
    )?;
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SelectedProviderFailure {
    ProviderNotSelected,
    ProviderNotConfigured(String),
    ModelNotSelected,
    ModelNotConfigured(String),
    ModelDisabled(String),
}

impl SelectedProviderFailure {
    fn detail(&self) -> String {
        match self {
            Self::ProviderNotSelected => "provider_not_selected".to_owned(),
            Self::ProviderNotConfigured(id) => format!("provider_not_configured:{id}"),
            Self::ModelNotSelected => "model_not_selected".to_owned(),
            Self::ModelNotConfigured(id) => format!("model_not_configured:{id}"),
            Self::ModelDisabled(id) => format!("model_disabled:{id}"),
        }
    }
}

pub(crate) fn selected_provider(
    config: &ConfigSnapshot,
) -> Result<(&Provider, &Model), SelectedProviderFailure> {
    let provider_id = config
        .session_settings
        .as_ref()
        .map(|settings| settings.provider.as_str())
        .or(config.workspace.policy.provider.as_deref())
        .or(config.settings.default_provider.as_deref())
        .ok_or(SelectedProviderFailure::ProviderNotSelected)?;
    let model_id = config
        .session_settings
        .as_ref()
        .map(|settings| settings.model.as_str())
        .or(config.workspace.policy.model.as_deref())
        .or(config.settings.default_model.as_deref())
        .ok_or(SelectedProviderFailure::ModelNotSelected)?;
    let provider = config
        .providers
        .providers
        .iter()
        .find(|provider| provider.id == provider_id)
        .ok_or_else(|| SelectedProviderFailure::ProviderNotConfigured(provider_id.to_owned()))?;
    let model = provider
        .models
        .iter()
        .find(|model| model.id == model_id)
        .ok_or_else(|| SelectedProviderFailure::ModelNotConfigured(model_id.to_owned()))?;
    if !model.enabled {
        return Err(SelectedProviderFailure::ModelDisabled(model_id.to_owned()));
    }
    Ok((provider, model))
}

pub(crate) fn current_turn_inputs(
    ledger: &LockedLedger,
    turn: u64,
) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let open = projection
        .events
        .iter()
        .rev()
        .find(|event| *event.kind() == EventKind::TurnOpen && event.turn() == Some(turn))
        .ok_or("current turn has no turn_open")?;
    let raw = serde_json::to_value(open.raw())?;
    Ok(raw
        .as_object()
        .expect("event object")
        .get("trigger")
        .and_then(Value::as_object)
        .and_then(|trigger| trigger.get("inputs"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_u64)
        .collect())
}
