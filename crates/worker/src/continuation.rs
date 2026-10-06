use super::*;

/// The durable facts of one call's remote continuation, folded from its
/// `state{subkind: tool_continuation}` records and approval pair.
pub(crate) struct ContinuationFacts {
    pub(crate) continuation_id: String,
    /// Highest durable step (0 = the initial pending binding).
    step: u64,
    /// Remote task status at that step.
    status: String,
    state: Value,
    /// Seq of the latest `update` step record, if any.
    updated_at: Option<u64>,
    /// Latest park record: when the next poll is due and the interval used.
    poll_after: Option<String>,
    interval_ms: Option<u64>,
    approval_request: Option<u64>,
    approval_response: Option<(u64, bool, Option<Value>)>,
    result: bool,
}

pub(crate) fn continuation_facts(
    ledger: &LockedLedger,
    call: &str,
) -> Result<Option<ContinuationFacts>, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let mut facts: Option<ContinuationFacts> = None;
    for event in &projection.events {
        match event.kind() {
            EventKind::State
                if event.string_field("subkind") == Some(tools::TOOL_CONTINUATION_SUBKIND) =>
            {
                let raw = serde_json::to_value(event.raw())?;
                let payload = raw
                    .get("payload")
                    .ok_or("tool_continuation lacks payload")?;
                if payload.get("call").and_then(Value::as_str) != Some(call) {
                    continue;
                }
                let continuation_id = payload
                    .get("continuation_id")
                    .and_then(Value::as_str)
                    .ok_or("tool_continuation lacks continuation_id")?
                    .to_owned();
                let step = payload
                    .get("step")
                    .and_then(Value::as_u64)
                    .ok_or("tool_continuation lacks step")?;
                let action = payload
                    .get("action")
                    .and_then(Value::as_str)
                    .unwrap_or("query");
                let state = materialize_json(
                    ledger,
                    payload
                        .get("state")
                        .ok_or("tool_continuation lacks state")?,
                )?;
                let status = state
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned();
                let updated_at = (action == "update").then_some(event.seq());
                let poll_after = payload
                    .get("poll_after")
                    .and_then(Value::as_str)
                    .map(str::to_owned);
                let interval_ms = payload.get("interval_ms").and_then(Value::as_u64);
                match facts.as_mut() {
                    Some(existing) => {
                        if existing.continuation_id != continuation_id {
                            return Err("call has records for two continuations".into());
                        }
                        if step < existing.step {
                            return Err("tool_continuation steps regress".into());
                        }
                        existing.step = step;
                        existing.status = status;
                        existing.state = state;
                        existing.updated_at = updated_at.or(existing.updated_at);
                        // Only a park record carries a due instant; any later
                        // receipt clears it.
                        existing.poll_after = poll_after;
                        existing.interval_ms = interval_ms.or(existing.interval_ms);
                    }
                    None => {
                        facts = Some(ContinuationFacts {
                            continuation_id,
                            step,
                            status,
                            state,
                            updated_at,
                            poll_after,
                            interval_ms,
                            approval_request: None,
                            approval_response: None,
                            result: false,
                        });
                    }
                }
            }
            EventKind::ApprovalRequest if event.string_field("call") == Some(call) => {
                if let Some(existing) = facts.as_mut() {
                    existing.approval_request = Some(event.seq());
                }
            }
            EventKind::ApprovalResponse if event.string_field("call") == Some(call) => {
                if let Some(existing) = facts.as_mut() {
                    let raw = serde_json::to_value(event.raw())?;
                    existing.approval_response = Some((
                        event.seq(),
                        raw.get("grant").and_then(Value::as_bool).unwrap_or(false),
                        raw.get("answer").cloned(),
                    ));
                }
            }
            EventKind::ToolResult if event.string_field("call") == Some(call) => {
                if let Some(existing) = facts.as_mut() {
                    existing.result = true;
                }
            }
            _ => {}
        }
    }
    Ok(facts)
}

/// Scope of the public question a remote task asks through `input_required`.
pub(crate) const CONTINUATION_INPUT_SCOPE: &str = "mcp_task_input";
const CONTINUATION_POLL_FLOOR: Duration = Duration::from_millis(250);
/// Polls shorter than this stay resident; at this interval the worker parks
/// (durable `poll_after`, line lock released) and the supervisor's sweep
/// resumes the line when the poll is due.
const CONTINUATION_PARK_THRESHOLD: Duration = Duration::from_secs(1);
const CONTINUATION_POLL_CEILING: Duration = Duration::from_secs(30);
pub(crate) fn rfc3339_after(delay: Duration) -> String {
    (chrono::Utc::now() + chrono::Duration::from_std(delay).unwrap_or_default())
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

pub(crate) struct ContinuationWait<'a> {
    pub(crate) options: &'a Options,
    pub(crate) profile: &'a RuntimeProfile,
    pub(crate) selected: Selected,
    pub(crate) call: &'a str,
    pub(crate) continuation_id: &'a str,
    pub(crate) cancellation: &'a RuntimeCancellation,
}

/// Drives one bound remote continuation to exactly one terminal `tool_result`.
/// Each request step is a distinct receipt-bound identity; every pending
/// response is a durable step record before the next request; `input_required`
/// parks the turn on an ordinary public hold whose answer becomes exactly one
/// input update; a stop cancels the remote task and terminalizes the call as
/// an error. Returns `true` when the turn parked or stopped.
pub(crate) fn wait_tool_continuation<I, O>(
    ledger: &mut LockedLedger,
    wait: ContinuationWait<'_>,
    lines: &mut I,
    stdout: &mut O,
) -> Result<bool, Box<dyn std::error::Error>>
where
    I: Iterator<Item = io::Result<String>>,
    O: Write,
{
    let ContinuationWait {
        options,
        profile,
        selected,
        call,
        continuation_id,
        cancellation,
    } = wait;
    let thread_folder = ledger
        .path()
        .parent()
        .ok_or("worker ledger has no thread folder")?
        .to_path_buf();
    let session = thread_folder
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("thread folder has no UTF-8 session UUID")?
        .to_owned();
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let thread = projection
        .events
        .first()
        .and_then(|genesis| genesis.string_field("thread"))
        .ok_or("genesis thread binding is missing")?
        .to_owned();
    let tool_call = projection
        .events
        .iter()
        .find(|event| {
            *event.kind() == EventKind::ToolCall && event.string_field("call") == Some(call)
        })
        .ok_or("continuation call is not durable")?;
    let turn = tool_call.turn().ok_or("tool_call lacks turn")?;
    let attempt = tool_call
        .string_field("attempt")
        .ok_or("tool_call lacks attempt")?
        .to_owned();
    let name = tool_call
        .string_field("name")
        .ok_or("tool_call lacks name")?
        .to_owned();
    let raw_call = serde_json::to_value(tool_call.raw())?;
    let mut arguments =
        materialize_json(ledger, raw_call.get("args").ok_or("tool_call lacks args")?)?;
    if let Some(effective) = projection.events.iter().rev().find(|event| {
        *event.kind() == EventKind::EffectiveExecution && event.string_field("call") == Some(call)
    }) {
        let raw = serde_json::to_value(effective.raw())?;
        arguments = materialize_json(
            ledger,
            raw.get("invocation")
                .ok_or("effective_execution lacks invocation")?,
        )?;
    }
    let arguments = IJsonValue::parse(&serde_json::to_vec(&arguments)?)?;
    let original = ToolControl::new(
        session,
        thread.clone(),
        turn,
        call,
        name.clone(),
        arguments.clone(),
    )?;
    let side_effectful = profile
        .bindings
        .dynamic_catalog
        .tools
        .iter()
        .find(|tool| tool.name == name)
        .is_none_or(|tool| engine::dynamic_side_effectful(tool.effect));
    let execution = ToolExecution {
        thread,
        call: call.to_owned(),
        name,
        attempt,
        invocation: arguments,
        side_effectful,
        turn,
        timestamp: options.event_timestamp().clone(),
    };
    let hooks = frozen_hook_bindings(&profile.instruction)?;
    let assets = store::AssetStore::new(thread_folder.join("assets"))?;
    let mut interval = CONTINUATION_POLL_FLOOR;
    let mut resumed_from_park = false;
    // After a park the due instant has been honored: the next query goes out
    // immediately, and only later polls may park again.
    let mut poll_immediately = false;
    loop {
        let facts =
            continuation_facts(ledger, call)?.ok_or("continuation has no durable step record")?;
        if facts.continuation_id != continuation_id {
            return Err("continuation identity changed under the call".into());
        }
        if facts.result {
            return Ok(false);
        }
        if !resumed_from_park {
            resumed_from_park = true;
            if let Some(parked_interval) = facts.interval_ms {
                // Resume after a park: honor the durable due instant, then keep
                // growing the interval from where the previous run left it.
                if let Some(due) = facts
                    .poll_after
                    .as_deref()
                    .and_then(|value| chrono::DateTime::parse_from_rfc3339(value).ok())
                {
                    let remaining = (due.with_timezone(&chrono::Utc) - chrono::Utc::now())
                        .to_std()
                        .unwrap_or_default();
                    std::thread::sleep(remaining.min(CONTINUATION_POLL_CEILING));
                }
                interval = Duration::from_millis(parked_interval.saturating_mul(2))
                    .min(CONTINUATION_POLL_CEILING);
                poll_immediately = true;
            }
        }
        let terminalize = |ledger: &mut LockedLedger, terminal: BackendTerminal| {
            let mut pipeline = ToolPipeline::new(ledger, assets.clone(), SecretScanner::default());
            pipeline
                .complete_continuation(&execution, &hooks, &execution.invocation, terminal)
                .map_err(|error| error.to_string())
        };
        if cancellation.stop_requested() {
            // Best effort remote cancellation is its own receipt; the local
            // outcome never claims the remote acknowledged it.
            let cancel = ToolContinuationRequest::new(
                original.clone(),
                continuation_id.to_owned(),
                facts.step + 1,
                ContinuationOperation::Cancel,
            )?;
            let remote = exchange_tool_continuation_runtime(
                &selected,
                &cancel,
                lines,
                stdout,
                cancellation,
                true,
            )
            .map(|response| match response.result {
                ToolContinuationOutcome::Failed { error } => format!(
                    "remote {}: {}",
                    control_error_code_name(&error.code),
                    error.message
                ),
                ToolContinuationOutcome::Pending { .. } => "remote still pending".to_owned(),
                ToolContinuationOutcome::Completed { .. } => {
                    "remote completed before cancellation".to_owned()
                }
            })
            .unwrap_or_else(|error| format!("cancellation not confirmed: {error}"));
            terminalize(
                ledger,
                BackendTerminal::Unavailable {
                    code: "cancelled".to_owned(),
                    message: format!("stopped while the remote task was pending; {remote}"),
                    retryable: false,
                },
            )?;
            return Ok(true);
        }
        let action = if facts.status == "input_required" {
            match (facts.approval_request, facts.approval_response) {
                (None, _) => {
                    // The task contract carries a non-empty `inputRequests`
                    // object; it is the public question verbatim.
                    let question = facts
                        .state
                        .get("inputRequests")
                        .cloned()
                        .unwrap_or_else(|| facts.state.clone());
                    let event = make_event(json!({
                        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"approval_request",
                        "ts":options.event_timestamp(),"call":call,
                        "scope":CONTINUATION_INPUT_SCOPE,"question":question
                    }))?;
                    ledger.append_contract(event, BarrierContext::default())?;
                    announce_appended(ledger, stdout)?;
                    return Ok(true);
                }
                (Some(_), None) => return Ok(true),
                (Some(_), Some((response_seq, grant, answer))) => {
                    if !grant {
                        terminalize(
                            ledger,
                            BackendTerminal::Unavailable {
                                code: "denied".to_owned(),
                                message: "remote task input was declined".to_owned(),
                                retryable: false,
                            },
                        )?;
                        return Ok(false);
                    }
                    if facts.updated_at.is_some_and(|seq| seq > response_seq) {
                        ContinuationOperation::Query
                    } else {
                        ContinuationOperation::Update {
                            input_responses: IJsonValue::parse(&serde_json::to_vec(
                                &answer.unwrap_or_else(|| json!({})),
                            )?)?,
                        }
                    }
                }
            }
        } else {
            ContinuationOperation::Query
        };
        if matches!(action, ContinuationOperation::Query) && std::mem::take(&mut poll_immediately) {
            // The parked interval already elapsed before this run resumed.
        } else if matches!(action, ContinuationOperation::Query) {
            let remote_hint = facts
                .state
                .get("pollIntervalMs")
                .and_then(Value::as_u64)
                .map(Duration::from_millis);
            let wait = remote_hint
                .map_or(interval, |hint| hint.max(interval))
                .min(CONTINUATION_POLL_CEILING);
            if wait >= CONTINUATION_PARK_THRESHOLD && !cancellation.stop_requested() {
                // Park: the due instant is durable, the turn stays open with
                // its unpaired call, and this run releases the line lock. The
                // supervisor spawns the resume when `poll_after` has passed.
                let poll_after = rfc3339_after(wait);
                let state = IJsonValue::parse(&serde_json::to_vec(&facts.state)?)?;
                let mut pipeline =
                    ToolPipeline::new(ledger, assets.clone(), SecretScanner::default());
                pipeline.append_continuation_step(
                    &execution,
                    continuation_id,
                    facts.step,
                    "park",
                    &state,
                    Some((&poll_after, wait.as_millis() as u64)),
                )?;
                announce_appended(ledger, stdout)?;
                return Ok(true);
            }
            std::thread::sleep(wait);
            interval = (interval * 2).min(CONTINUATION_POLL_CEILING);
        }
        let request = ToolContinuationRequest::new(
            original.clone(),
            continuation_id.to_owned(),
            facts.step + 1,
            action.clone(),
        )?;
        let response = match exchange_tool_continuation_runtime(
            &selected,
            &request,
            lines,
            stdout,
            cancellation,
            false,
        ) {
            Ok(response) => response,
            Err(error) if cancellation.stop_requested() && !cancellation.protocol_failed() => {
                let _ = error;
                continue;
            }
            Err(error) => return Err(error),
        };
        match response.result {
            ToolContinuationOutcome::Pending { state, .. } => {
                let action_name = match action {
                    ContinuationOperation::Query => "query",
                    ContinuationOperation::Update { .. } => "update",
                    ContinuationOperation::Cancel => "cancel",
                };
                let mut pipeline =
                    ToolPipeline::new(ledger, assets.clone(), SecretScanner::default());
                pipeline.append_continuation_step(
                    &execution,
                    continuation_id,
                    facts.step + 1,
                    action_name,
                    &state,
                    None,
                )?;
                if action_name == "update" {
                    interval = CONTINUATION_POLL_FLOOR;
                }
            }
            ToolContinuationOutcome::Completed { value } => {
                terminalize(ledger, BackendTerminal::Completed(value))?;
                announce_appended(ledger, stdout)?;
                return Ok(false);
            }
            ToolContinuationOutcome::Failed { error } => {
                terminalize(
                    ledger,
                    BackendTerminal::Unavailable {
                        code: control_error_code_name(&error.code).to_owned(),
                        message: error.message,
                        retryable: error.retryable,
                    },
                )?;
                announce_appended(ledger, stdout)?;
                return Ok(false);
            }
        }
    }
}

fn control_error_code_name(code: &ToolControlErrorCode) -> String {
    serde_json::to_value(code)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_else(|| "internal".to_owned())
}

/// One continuation step over the control channel. `allow_stop` lets a
/// cancellation step through after a stop was requested.
fn exchange_tool_continuation_runtime(
    selected: &Selected,
    request: &ToolContinuationRequest,
    lines: &mut impl Iterator<Item = io::Result<String>>,
    stdout: &mut impl Write,
    cancellation: &RuntimeCancellation,
    allow_stop: bool,
) -> Result<ToolContinuationResponse, Box<dyn std::error::Error>> {
    require_version(selected)?;
    if cancellation.stop_requested() && !allow_stop {
        return Err("tool continuation cancelled by stop".into());
    }
    if cancellation.supervisor_lost() {
        cancellation.mark_protocol_failed();
        return Err(Box::new(ProtocolFailure(
            "supervisor EOF before tool continuation".to_owned(),
        )));
    }
    stdout.write_all(&encode_tool_continuation(request)?)?;
    stdout.flush()?;
    loop {
        let Some(line) = lines.next() else {
            cancellation.cancel(CANCEL_SUPERVISOR_LOSS);
            cancellation.mark_protocol_failed();
            return Err(Box::new(ProtocolFailure(
                "supervisor EOF while awaiting tool continuation result".to_owned(),
            )));
        };
        let line = line.map_err(|error| {
            cancellation.cancel(CANCEL_SUPERVISOR_LOSS);
            cancellation.mark_protocol_failed();
            ProtocolFailure(format!("supervisor read failed: {error}"))
        })?;
        match decode_tool_continuation_result(line.as_bytes()) {
            Ok(response) => {
                if let Err(error) = response.validate_for(request) {
                    cancellation.mark_protocol_failed();
                    return Err(Box::new(ProtocolFailure(error)));
                }
                return Ok(response);
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
                    if !allow_stop {
                        return Err("tool continuation cancelled by stop".into());
                    }
                }
                Ok(SupervisorMessage::QueueTransaction(_)) => cancellation.park_delivery(line),
                Ok(_) => {
                    cancellation.mark_protocol_failed();
                    return Err(Box::new(ProtocolFailure(
                        "unexpected supervisor message while awaiting tool continuation result"
                            .to_owned(),
                    )));
                }
                Err(error) => {
                    cancellation.mark_protocol_failed();
                    return Err(Box::new(ProtocolFailure(format!(
                        "invalid supervisor line while awaiting tool continuation result: {error}"
                    ))));
                }
            },
        }
    }
}
