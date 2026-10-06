use super::*;

pub(crate) struct ChildSpawnExecution<'a> {
    pub(crate) options: &'a Options,
    pub(crate) profile: &'a RuntimeProfile,
    pub(crate) selected: Selected,
    pub(crate) attempt: &'a str,
    pub(crate) turn: u64,
    pub(crate) call: &'a provider::ToolCall,
    pub(crate) hooks: &'a [HookBinding],
    pub(crate) manifest: &'a BuiltinManifest,
    pub(crate) cancellation: &'a RuntimeCancellation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DurableChildSpawn {
    pub(crate) child: String,
    pub(crate) child_file: String,
    pub(crate) spawn_id: String,
    pub(crate) resume: ResumePolicy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ChildTerminal {
    pub(crate) outcome: String,
    pub(crate) summary: Option<String>,
}

pub(crate) fn execute_child_spawn_tool<I, O>(
    ledger: &mut LockedLedger,
    execution: ChildSpawnExecution<'_>,
    lines: &mut I,
    stdout: &mut O,
) -> Result<bool, Box<dyn std::error::Error>>
where
    I: Iterator<Item = io::Result<String>>,
    O: Write,
{
    let ChildSpawnExecution {
        options,
        profile,
        selected,
        attempt,
        turn,
        call,
        hooks,
        manifest,
        cancellation,
    } = execution;
    if !effective_allowed_tools(profile).contains(&call.name) {
        return Err(format!("child-spawn tool {} is not allowed", call.name).into());
    }
    let thread = ledger
        .projection()
        .and_then(|projection| projection.events.first())
        .and_then(|genesis| genesis.string_field("thread"))
        .ok_or("genesis thread binding is missing")?
        .to_owned();
    let invocation = ToolInvocation {
        thread: thread.clone(),
        turn,
        attempt: attempt.to_owned(),
        call: call.call_id.clone(),
        name: call.name.clone(),
        arguments: IJsonValue::parse(&serde_json::to_vec(&call.arguments)?)?,
        timestamp: options.event_timestamp().clone(),
    };
    let context = tool_catalog_context(profile, false, false)?;
    let dispatcher = ToolDispatcher::new(manifest, &context);
    let assets = AssetStore::new(
        ledger
            .path()
            .parent()
            .ok_or("worker ledger has no thread folder")?
            .join("assets"),
    )?;
    let mut policy = session_permission_policy(
        ledger
            .path()
            .parent()
            .ok_or("worker ledger has no thread folder")?,
    );
    let mut pipeline = ToolPipeline::new(ledger, assets, SecretScanner::default());
    let mut deferred = DeferredChildBackend;
    match dispatcher.dispatch(
        &mut pipeline,
        &invocation,
        hooks,
        &mut policy,
        &mut deferred,
    )? {
        PipelineDecision::Parked { .. } => return Ok(true),
        PipelineDecision::Pending { .. } => {
            return Err("child spawn backend cannot report a remote continuation".into());
        }
        PipelineDecision::Deferred => {}
        PipelineDecision::Completed { .. } => return Ok(false),
    }
    drop(pipeline);

    let effective = durable_invocation(ledger, &call.call_id, &invocation.arguments)?;
    let spawn = match ensure_durable_child(ledger, options, profile, call, &effective) {
        Ok(spawn) => spawn,
        Err(error) if error.downcast_ref::<TaskInputError>().is_some() => {
            append_tool_validation_error(ledger, options, turn, call, &error.to_string())?;
            return Ok(false);
        }
        Err(error) => return Err(error),
    };
    let session = ledger
        .path()
        .parent()
        .and_then(|folder| folder.file_name())
        .and_then(|name| name.to_str())
        .ok_or("thread folder has no UTF-8 session UUID")?
        .to_owned();
    let request = ToolControl::new(
        session,
        thread,
        turn,
        call.call_id.clone(),
        call.name.clone(),
        effective,
    )?;
    let control = exchange_tool_control_runtime(&selected, &request, lines, stdout, cancellation)?;

    if control.error.is_some() {
        append_child_result_once(
            ledger,
            options,
            turn,
            call,
            &spawn,
            &ChildTerminal {
                outcome: "failed".to_owned(),
                summary: control.error.as_ref().map(|error| error.message.clone()),
            },
        )?;
    } else {
        let launch = exchange_launch_child_runtime(&spawn, lines, stdout, cancellation)?;
        if !launch.ok {
            append_child_result_once(
                ledger,
                options,
                turn,
                call,
                &spawn,
                &ChildTerminal {
                    outcome: "failed".to_owned(),
                    summary: launch.error,
                },
            )?;
        } else {
            let terminal =
                wait_for_child_terminal(ledger, &spawn, cancellation, options, &selected, stdout)?;
            append_child_result_once(ledger, options, turn, call, &spawn, &terminal)?;
        }
    }

    let assets = AssetStore::new(
        ledger
            .path()
            .parent()
            .ok_or("worker ledger has no thread folder")?
            .join("assets"),
    )?;
    let mut policy = session_permission_policy(
        ledger
            .path()
            .parent()
            .ok_or("worker ledger has no thread folder")?,
    );
    let mut pipeline = ToolPipeline::new(ledger, assets, SecretScanner::default());
    let mut replayed = ReplayedChildBackend { result: control };
    match dispatcher.dispatch(
        &mut pipeline,
        &invocation,
        hooks,
        &mut policy,
        &mut replayed,
    )? {
        PipelineDecision::Completed { .. } => Ok(false),
        PipelineDecision::Parked { .. } => Ok(true),
        PipelineDecision::Pending { .. } => {
            Err("child result backend cannot report a remote continuation".into())
        }
        PipelineDecision::Deferred => Err("child result backend deferred unexpectedly".into()),
    }
}

fn durable_invocation(
    ledger: &LockedLedger,
    call: &str,
    original: &IJsonValue,
) -> Result<IJsonValue, Box<dyn std::error::Error>> {
    let Some(event) = ledger.projection().and_then(|projection| {
        projection.events.iter().find(|event| {
            event.kind() == &EventKind::EffectiveExecution
                && event.string_field("call") == Some(call)
        })
    }) else {
        return Ok(original.clone());
    };
    let raw = serde_json::to_value(event.raw())?;
    let value = raw
        .get("invocation")
        .ok_or("effective_execution invocation is missing")?;
    Ok(IJsonValue::parse(&serde_json::to_vec(&materialize_json(
        ledger, value,
    )?)?)?)
}

pub(crate) fn ensure_durable_child(
    ledger: &mut LockedLedger,
    options: &Options,
    profile: &RuntimeProfile,
    call: &provider::ToolCall,
    effective: &IJsonValue,
) -> Result<DurableChildSpawn, Box<dyn std::error::Error>> {
    if let Some(spawn) = ledger.projection().and_then(|projection| {
        projection.events.iter().find(|event| {
            event.kind() == &EventKind::Spawn
                && event.string_field("call") == Some(call.call_id.as_str())
        })
    }) {
        let raw = serde_json::to_value(spawn.raw())?;
        let child_file = spawn
            .string_field("child")
            .ok_or("spawn child is missing")?
            .to_owned();
        let child_path = ledger
            .path()
            .parent()
            .ok_or("parent ledger has no folder")?
            .join(&child_file);
        let child_line = child_path
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or("spawn child file has no UTF-8 stem")?
            .to_owned();
        if child_path.exists() {
            let child = read_child_projection(&child_path)?;
            let genesis = child.events.first().ok_or("child genesis is missing")?;
            if genesis.string_field("thread") != Some(child_line.as_str()) {
                return Err("spawn child file does not match child genesis".into());
            }
        }
        return Ok(DurableChildSpawn {
            child: child_line,
            child_file,
            spawn_id: spawn
                .string_field("spawn_id")
                .ok_or("spawn id is missing")?
                .to_owned(),
            resume: serde_json::from_value(
                raw.get("resume")
                    .cloned()
                    .ok_or("spawn resume is missing")?,
            )?,
        });
    }

    let delegation = ensure_delegation_state(ledger, options, call, effective)?;
    let spawn_seq = ledger.next_seq();
    let parent_thread = ledger
        .projection()
        .and_then(|projection| projection.events.first())
        .and_then(|genesis| genesis.string_field("thread"))
        .ok_or("parent genesis thread is missing")?;
    let (child, spawn_id) = child_identity(parent_thread, &call.call_id, spawn_seq);
    let child_file = format!("{child}.jsonl");
    let resume = ResumePolicy::Bounded(3);
    let mut seed_bytes = delegation.canonical_bytes()?;
    seed_bytes.push(b'\n');
    let folder = ledger
        .path()
        .parent()
        .ok_or("parent ledger has no folder")?;
    let assets = AssetStore::new(folder.join("assets"))?;
    let seed = assets.publish(&seed_bytes)?;
    let digest = seed
        .asset
        .strip_prefix("sha256-")
        .ok_or("seed asset name has no sha256 prefix")?;
    let parent_file = ledger
        .path()
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("parent ledger file name is not UTF-8")?;
    let origin_key = format!("spawn:{spawn_id}");
    let mut genesis = json!({
        "v":1,"seq":1,"kind":"genesis","ts":options.event_timestamp(),
        "format":1,"min_reader":1,"min_writer":1,
        "thread":child,"workspace":profile.config.workspace.id,
        "origin_key":origin_key,
        "origin_tuple":{
            "principal":"kernel-worker","client":"worker",
            "target":child,"op":"spawn","key":origin_key
        },
        "parent":{"file":parent_file,"seq":spawn_seq,"spawn_id":spawn_id},
        "seed":{"source":parent_thread,"kinds":["state"],
            "snapshot":{"asset":seed.asset,"digest":digest}},
        "resume":resume,
        "config":{"digest":options.config_digest}
    });
    if let Some(identity) = identity::selected(ledger)? {
        genesis
            .as_object_mut()
            .expect("genesis object")
            .insert("identity_profile".to_owned(), json!(identity.as_str()));
    }
    if !options.instruction_digest.is_empty() {
        genesis.as_object_mut().expect("genesis object").insert(
            "instruction".to_owned(),
            json!({"digest":options.instruction_digest}),
        );
    }
    publish_child_genesis(folder, &child_file, make_event(genesis)?)?;
    let spawn = make_event(json!({
        "v":1,"seq":spawn_seq,"turn":call_turn(ledger, &call.call_id)?,
        "kind":"spawn","ts":options.event_timestamp(),"child":child_file,
        "call":call.call_id,"spawn_id":spawn_id,"resume":resume,
        "seed":{"kinds":["state"]}
    }))?;
    ledger.append_contract(spawn, BarrierContext::default())?;
    Ok(DurableChildSpawn {
        child,
        child_file,
        spawn_id,
        resume,
    })
}

pub(crate) fn ensure_delegation_state(
    ledger: &mut LockedLedger,
    options: &Options,
    call: &provider::ToolCall,
    effective: &IJsonValue,
) -> Result<Event, Box<dyn std::error::Error>> {
    let invocation = serde_json::to_value(effective)?;
    if let Some(existing) = ledger.projection().and_then(|projection| {
        projection.events.iter().find(|event| {
            if event.kind() != &EventKind::State
                || event.string_field("subkind") != Some("delegation")
            {
                return false;
            }
            serde_json::to_value(event.raw())
                .ok()
                .and_then(|value| {
                    value
                        .pointer("/payload/call")
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                })
                .as_deref()
                == Some(call.call_id.as_str())
        })
    }) {
        let raw = serde_json::to_value(existing.raw())?;
        if raw.pointer("/payload/name").and_then(Value::as_str) != Some(call.name.as_str())
            || raw.pointer("/payload/invocation") != Some(&invocation)
        {
            return Err("durable delegation state does not match effective invocation".into());
        }
        return Ok(existing.clone());
    }

    let resolved_inputs = resolve_completed_task_inputs(ledger, &invocation, &call.call_id)?;
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":call_turn(ledger, &call.call_id)?,
        "kind":"state","ts":options.event_timestamp(),"subkind":"delegation",
        "payload":{"call":call.call_id,"name":call.name,"invocation":invocation,
            "resolved_inputs":resolved_inputs}
    }))?;
    ledger.append_contract(event.clone(), BarrierContext::default())?;
    Ok(event)
}

#[derive(Debug)]
pub(crate) struct TaskInputError(String);
impl std::fmt::Display for TaskInputError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}
impl std::error::Error for TaskInputError {}

/// Resolves task-name inputs only from earlier, completed sibling delegations.
/// Paths supplied directly by the model remain ordinary paths and are not
/// rewritten. The durable mapping travels in the child seed so recovery does
/// not depend on re-reading mutable parent state.
pub(crate) fn resolve_completed_task_inputs(
    ledger: &LockedLedger,
    invocation: &Value,
    current_call: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let Some(sources) = invocation.get("input_sources").and_then(Value::as_array) else {
        return Ok(json!([]));
    };
    let projection = ledger.projection().ok_or("task ledger has no projection")?;
    let current_seq = projection
        .events
        .iter()
        .find(|event| {
            event.kind() == &EventKind::ToolCall && event.string_field("call") == Some(current_call)
        })
        .map(Event::seq)
        .ok_or("task call is absent while resolving input sources")?;
    let mut resolved = Vec::new();
    for source in sources {
        let source = source.as_str().ok_or("task input source is not a string")?;
        let matches = projection
            .events
            .iter()
            .filter(|event| {
                event.seq() < current_seq
                    && event.kind() == &EventKind::State
                    && event.string_field("subkind") == Some("delegation")
            })
            .filter_map(|event| {
                let raw = serde_json::to_value(event.raw()).ok()?;
                (raw.pointer("/payload/name").and_then(Value::as_str) == Some("task")
                    && raw
                        .pointer("/payload/invocation/task_name")
                        .and_then(Value::as_str)
                        == Some(source))
                .then_some((event, raw))
            })
            .collect::<Vec<_>>();
        if matches.is_empty() {
            continue;
        }
        // Re-executing a named task supersedes its older version. Never fall
        // back to an old completion while the latest version is unfinished.
        let (delegation, raw) = matches.last().expect("nonempty task matches");
        let prior_call = raw
            .pointer("/payload/call")
            .and_then(Value::as_str)
            .ok_or("prior task delegation lacks call")?;
        let completion = projection.events.iter().find(|event| {
            event.seq() < current_seq
                && event.kind() == &EventKind::ChildResult
                && event.string_field("call") == Some(prior_call)
                && event.string_field("outcome") == Some("completed")
        });
        let Some(completion) = completion else {
            return Err(Box::new(TaskInputError(format!(
                "task input source {source:?} is not completed"
            ))));
        };
        let output = raw
            .pointer("/payload/invocation/output")
            .cloned()
            .ok_or("prior task delegation lacks output contract")?;
        let path = output
            .get("path")
            .and_then(Value::as_str)
            .ok_or("prior task output path is not a string")?;
        resolved.push(json!({
            "source":source,"path":path,"output":output,"summary":completion.string_field("summary"),
            "delegation_seq":delegation.seq(),"completion_seq":completion.seq()
        }));
    }
    Ok(Value::Array(resolved))
}

pub(crate) fn child_identity(parent: &str, call: &str, spawn_seq: u64) -> (String, String) {
    let mut hasher = Sha256::new();
    hasher.update(b"tekes-child-v1\0");
    hasher.update(parent.as_bytes());
    hasher.update([0]);
    hasher.update(call.as_bytes());
    hasher.update([0]);
    hasher.update(spawn_seq.to_string().as_bytes());
    let digest: [u8; 32] = hasher.finalize().into();
    let mut bytes = [0_u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x50;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let mut hex = String::with_capacity(32);
    for byte in bytes {
        use std::fmt::Write as _;
        write!(&mut hex, "{byte:02x}").expect("writing to String cannot fail");
    }
    let child = format!(
        "{}-{}-{}-{}-{}",
        &hex[0..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..32]
    );
    (child, format!("spawn-{}", &hex[..24]))
}

fn call_turn(ledger: &LockedLedger, call: &str) -> Result<u64, Box<dyn std::error::Error>> {
    ledger
        .projection()
        .and_then(|projection| {
            projection.events.iter().find(|event| {
                event.kind() == &EventKind::ToolCall && event.string_field("call") == Some(call)
            })
        })
        .and_then(Event::turn)
        .ok_or_else(|| "tool_call turn is missing".into())
}

pub(crate) fn publish_child_genesis(
    folder: &std::path::Path,
    child_file: &str,
    genesis: Event,
) -> Result<(), Box<dyn std::error::Error>> {
    let destination = folder.join(child_file);
    let mut expected = genesis.canonical_bytes()?;
    expected.push(b'\n');
    if destination.exists() {
        if fs::read(&destination)? == expected {
            return Ok(());
        }
        return Err(format!("child file {child_file} exists with different genesis").into());
    }
    let temp = folder.join(format!(".{child_file}.{}.tmp", std::process::id()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&temp)?;
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        file.write_all(&expected)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temp, &destination)?;
        File::open(folder)?.sync_all()?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

pub(crate) fn exchange_launch_child_runtime<I, O>(
    spawn: &DurableChildSpawn,
    lines: &mut I,
    stdout: &mut O,
    cancellation: &RuntimeCancellation,
) -> Result<LaunchResult, Box<dyn std::error::Error>>
where
    I: Iterator<Item = io::Result<String>>,
    O: Write,
{
    stdout.write_all(&encode_line(
        "launch_child",
        &LaunchChild {
            child: spawn.child.clone(),
            spawn_id: spawn.spawn_id.clone(),
            resume: spawn.resume,
        },
    )?)?;
    stdout.flush()?;
    loop {
        let line = lines.next().ok_or_else(|| {
            cancellation.cancel(CANCEL_SUPERVISOR_LOSS);
            ProtocolFailure("supervisor EOF while awaiting launch_result".to_owned())
        })??;
        match decode_supervisor(line.as_bytes())? {
            SupervisorMessage::LaunchResult(result)
                if result.child == spawn.child && result.spawn_id == spawn.spawn_id =>
            {
                return Ok(result);
            }
            SupervisorMessage::Ping(ping) => {
                stdout.write_all(&encode_line("pong", &worker_control::Pong { id: ping.id })?)?;
                stdout.flush()?;
            }
            SupervisorMessage::Stop(_) => {
                cancellation.cancel(CANCEL_STOP);
                cancellation.defer(line);
                return Err("child launch cancelled by stop".into());
            }
            SupervisorMessage::QueueTransaction(_) => cancellation.park_delivery(line),
            _ => {
                cancellation.mark_protocol_failed();
                return Err(Box::new(ProtocolFailure(
                    "unexpected supervisor message while awaiting launch_result".to_owned(),
                )));
            }
        }
    }
}

pub(crate) fn wait_for_child_terminal(
    parent: &mut LockedLedger,
    spawn: &DurableChildSpawn,
    cancellation: &RuntimeCancellation,
    options: &Options,
    selected: &Selected,
    stdout: &mut impl Write,
) -> Result<ChildTerminal, Box<dyn std::error::Error>> {
    let path = parent
        .path()
        .parent()
        .ok_or("parent ledger has no folder")?
        .join(&spawn.child_file);
    loop {
        if cancellation.stop_requested() {
            return Err("child wait cancelled by stop".into());
        }
        if cancellation.supervisor_lost() {
            return Err(Box::new(ProtocolFailure(
                "supervisor EOF while waiting for child".to_owned(),
            )));
        }
        // A child wait remains a yield point for durable user input. Receipting
        // the input queues it; turn admission still waits for root settlement.
        drain_parked_deliveries(parent, options, selected, cancellation, stdout)?;
        let projection = read_child_projection(&path)?;
        if projection.terminal_tail {
            let settle = projection
                .events
                .iter()
                .rev()
                .find(|event| event.kind() == &EventKind::Settle)
                .ok_or("terminal child has no settle")?;
            let raw = serde_json::to_value(settle.raw())?;
            let outcome = match raw.get("outcome").and_then(Value::as_str) {
                Some("completed") => "completed",
                Some("interrupted") => "interrupted",
                Some("error") => "error",
                _ => return Err("child settle has unknown outcome".into()),
            };
            return Ok(ChildTerminal {
                outcome: outcome.to_owned(),
                summary: child_report_summary(&path, &projection)?,
            });
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}

pub(crate) fn read_child_projection(
    path: &std::path::Path,
) -> Result<schema::LedgerProjection, Box<dyn std::error::Error>> {
    let bytes = fs::read(path)?;
    scan_valid_prefix(&bytes, 1)
        .projection
        .ok_or_else(|| "child ledger has no projection".into())
}

fn child_report_summary(
    path: &std::path::Path,
    projection: &schema::LedgerProjection,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let assets = AssetStore::new(
        path.parent()
            .ok_or("child ledger has no folder")?
            .join("assets"),
    )?;
    for event in projection.events.iter().rev() {
        if event.kind() != &EventKind::ToolCall || event.string_field("name") != Some("report") {
            continue;
        }
        if event.turn() != projection.lifecycle.latest_turn
            || !projection.events.iter().any(|result| {
                result.kind() == &EventKind::ToolResult
                    && result.seq() > event.seq()
                    && result.turn() == event.turn()
                    && result.string_field("call") == event.string_field("call")
                    && result.string_field("outcome") == Some("ok")
            })
        {
            continue;
        }
        let raw = serde_json::to_value(event.raw())?;
        let mut args = raw.get("args").cloned().ok_or("report args are missing")?;
        if let Some(asset) = args
            .get("$spill")
            .and_then(|spill| spill.get("asset"))
            .and_then(Value::as_str)
        {
            args = serde_json::from_slice(&assets.read_verified(asset)?)?;
        }
        if let Some(result) = args.get("result").and_then(Value::as_str) {
            return Ok(Some(bounded_text(result, 4096)));
        }
    }
    Ok(None)
}

fn bounded_text(value: &str, max: usize) -> String {
    if value.len() <= max {
        return value.to_owned();
    }
    let mut end = max;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}[truncated]", &value[..end])
}

pub(crate) fn append_child_result_once(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    call: &provider::ToolCall,
    spawn: &DurableChildSpawn,
    terminal: &ChildTerminal,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(existing) = ledger.projection().and_then(|projection| {
        projection.events.iter().find(|event| {
            event.kind() == &EventKind::ChildResult
                && event.string_field("call") == Some(call.call_id.as_str())
        })
    }) {
        if existing.string_field("child") == Some(spawn.child_file.as_str())
            && existing.string_field("spawn_id") == Some(spawn.spawn_id.as_str())
        {
            return Ok(());
        }
        return Err("existing child_result does not match spawn".into());
    }
    let mut value = json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"child_result",
        "ts":options.event_timestamp(),"child":spawn.child_file,"call":call.call_id,
        "spawn_id":spawn.spawn_id,"outcome":terminal.outcome
    });
    if let Some(summary) = &terminal.summary {
        value
            .as_object_mut()
            .expect("child_result object")
            .insert("summary".to_owned(), Value::String(summary.clone()));
    }
    ledger.append_contract(make_event(value)?, BarrierContext::default())?;
    Ok(())
}
