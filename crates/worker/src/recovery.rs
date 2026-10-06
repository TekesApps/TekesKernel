use super::*;

pub(crate) struct UnresolvedAttempt {
    pub(crate) id: String,
    turn: u64,
    epoch: String,
    dispatched: bool,
    decision: Option<String>,
    inventory: Vec<String>,
    response_asset: Option<String>,
    outcome_seq: Option<u64>,
    reasoning_present: bool,
    pub(crate) calls: std::collections::BTreeSet<String>,
    pub(crate) results: std::collections::BTreeSet<String>,
}

pub(crate) struct PendingToolCall {
    pub(crate) call: String,
    pub(crate) name: String,
    arguments: Value,
    attempt: String,
    pub(crate) turn: u64,
    pub(crate) approval_requested: bool,
    pub(crate) approval_response: Option<(bool, Option<Value>)>,
    pub(crate) execution_tracked: bool,
    pub(crate) execution_started: bool,
}

pub(crate) enum RecoveryProgress {
    Continue { reset_epoch: bool },
    Stop,
}

pub(crate) fn reconcile_provider_attempt(
    ledger: &mut LockedLedger,
    options: &Options,
    stdout: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(attempt) = unresolved_attempt(ledger)? else {
        if let Some(projection) = ledger.projection() {
            if !projection.terminal_tail
                && !projection.lifecycle.unresolved_work
                && !projection.lifecycle.open_hold
            {
                if let Some(turn) = projection.latest_turn {
                    append_settle(
                        ledger,
                        &options.event_timestamp(),
                        turn,
                        "interrupted",
                        Some("recovered"),
                    )?;
                }
            }
        }
        return Ok(());
    };
    let decision = match attempt.decision.as_deref() {
        Some(decision) => decision.to_owned(),
        None if !attempt.dispatched => "not_dispatched".to_owned(),
        None => "unresolved".to_owned(),
    };
    if attempt.decision.is_none() {
        let event = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":attempt.turn,
            "kind":"attempt_recovery","ts":options.event_timestamp(),
            "attempt":attempt.id,"decision":decision
        }))?;
        ledger.append_contract(event, BarrierContext::default())?;
    }
    if decision == "adopt" {
        complete_adopt(ledger, options, stdout, attempt, true)?;
        return Ok(());
    }
    let unresolved = decision == "unresolved";
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":attempt.turn,"kind":"error",
        "ts":options.event_timestamp(),"attempt":attempt.id,"usage":usage_object(None),
        "recoverable":!unresolved,
        "classification":if unresolved { "unresolved_dispatch" } else { "transport" }
    }))?;
    let outcome_seq = event.seq();
    ledger.append_contract(event, BarrierContext::default())?;
    stdout.write_all(&encode_line(
        "attempt_settled",
        &AttemptSettled {
            attempt: attempt.id,
            outcome_seq,
        },
    )?)?;
    stdout.flush()?;
    append_settle(
        ledger,
        &options.event_timestamp(),
        attempt.turn,
        "interrupted",
        Some("recovered"),
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn recover_unpaired_tool_calls<I, O>(
    ledger: &mut LockedLedger,
    options: &Options,
    profile: Option<&RuntimeProfile>,
    selected: Selected,
    reconcile_only: bool,
    lines: &mut I,
    stdout: &mut O,
    cancellation: &RuntimeCancellation,
) -> Result<bool, Box<dyn std::error::Error>>
where
    I: Iterator<Item = io::Result<String>>,
    O: Write,
{
    let pending = pending_tool_calls(ledger)?;
    // A batch executes in order (`execute_provider_tool_calls`), so within one
    // attempt only the first unpaired call can have started. Every later one
    // is provably unstarted: the previous run parked or died before reaching
    // it. Those are as safe to run now as they were then, and calling them a
    // crash both lies about what happened and throws the work away — a
    // provider batch whose first call needs approval otherwise loses every
    // sibling (live corpus runs on four routes ended this way).
    let mut seen_attempts = BTreeSet::new();
    let unstarted = pending
        .iter()
        .filter(|call| !seen_attempts.insert(call.attempt.clone()))
        .map(|call| call.call.clone())
        .collect::<BTreeSet<_>>();
    let manifest = BuiltinManifest::compiled();
    for call in pending {
        let never_started = if call.execution_tracked {
            !call.execution_started
        } else {
            unstarted.contains(&call.call)
        };
        if call.approval_requested && call.approval_response.is_none() {
            if !reconcile_only {
                return Ok(true);
            }
            append_aborted_tool_result(ledger, options, &call.call, call.turn, "recovered")?;
            continue;
        }
        if let Some(continuation) = continuation_facts(ledger, &call.call)? {
            // A remote continuation survives the worker: the initial side effect
            // already happened once, so recovery only resumes the step ledger.
            if reconcile_only {
                append_aborted_tool_result(ledger, options, &call.call, call.turn, "recovered")?;
                continue;
            }
            let Some(profile) = profile else {
                append_aborted_tool_result(ledger, options, &call.call, call.turn, "crash")?;
                continue;
            };
            if wait_tool_continuation(
                ledger,
                ContinuationWait {
                    options,
                    profile,
                    selected,
                    call: &call.call,
                    continuation_id: &continuation.continuation_id,
                    cancellation,
                },
                lines,
                stdout,
            )? {
                return Ok(true);
            }
            continue;
        }
        let tool = manifest.tools.iter().find(|tool| tool.name == call.name);
        let replay_safe = tool.is_some_and(|tool| !engine::side_effectful(tool.effect));
        let stable_receipt_recovery =
            tool.is_some_and(|tool| tool.backend == Backend::SupervisorControl);
        let durable_approval_resume =
            call.approval_response.is_some() && call.execution_tracked && !call.execution_started;
        if replay_safe || stable_receipt_recovery || durable_approval_resume || never_started {
            if let Some(profile) = profile {
                let provider_call = provider::ToolCall {
                    call_id: call.call.clone(),
                    name: call.name.clone(),
                    arguments: call.arguments.clone(),
                };
                if execute_provider_tool_calls(
                    ledger,
                    ToolCallBatch {
                        options,
                        profile,
                        selected,
                        attempt: &call.attempt,
                        turn: call.turn,
                        calls: std::slice::from_ref(&provider_call),
                        cancellation,
                    },
                    None,
                    lines,
                    stdout,
                )? {
                    if reconcile_only {
                        append_aborted_tool_result(
                            ledger,
                            options,
                            &call.call,
                            call.turn,
                            "recovered",
                        )?;
                    } else {
                        return Ok(true);
                    }
                }
                continue;
            }
        }
        // Without a profile there is nothing to execute with, but an unstarted
        // call is still not a crash.
        let outcome = if never_started { "recovered" } else { "crash" };
        append_aborted_tool_result(ledger, options, &call.call, call.turn, outcome)?;
    }
    Ok(false)
}

pub(crate) fn pending_tool_calls(
    ledger: &LockedLedger,
) -> Result<Vec<PendingToolCall>, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let mut results = BTreeSet::new();
    let mut requests = BTreeSet::new();
    let mut responses = std::collections::BTreeMap::<String, (bool, Option<Value>)>::new();
    for event in &projection.events {
        match event.kind() {
            EventKind::ToolResult => {
                if let Some(call) = event.string_field("call") {
                    results.insert(call.to_owned());
                }
            }
            EventKind::ApprovalRequest => {
                if let Some(call) = event.string_field("call") {
                    requests.insert(call.to_owned());
                }
            }
            EventKind::ApprovalResponse => {
                let raw = serde_json::to_value(event.raw())?;
                if let Some(call) = event.string_field("call") {
                    responses.insert(
                        call.to_owned(),
                        (
                            raw.get("grant").and_then(Value::as_bool).unwrap_or(false),
                            raw.get("answer").cloned(),
                        ),
                    );
                }
            }
            _ => {}
        }
    }
    let mut pending = Vec::new();
    for event in &projection.events {
        if *event.kind() != EventKind::ToolCall {
            continue;
        }
        let call = event.string_field("call").ok_or("tool_call lacks call")?;
        if results.contains(call) {
            continue;
        }
        let raw = serde_json::to_value(event.raw())?;
        pending.push(PendingToolCall {
            call: call.to_owned(),
            name: event
                .string_field("name")
                .ok_or("tool_call lacks name")?
                .to_owned(),
            arguments: materialize_json(ledger, raw.get("args").ok_or("tool_call lacks args")?)?,
            attempt: event
                .string_field("attempt")
                .ok_or("tool_call lacks attempt")?
                .to_owned(),
            turn: event.turn().ok_or("tool_call lacks turn")?,
            approval_requested: requests.contains(call),
            approval_response: responses.get(call).cloned(),
            execution_tracked: raw.get("execution_tracked").and_then(Value::as_bool) == Some(true),
            execution_started: projection
                .events
                .iter()
                .rev()
                .filter(|event| event.string_field("call") == Some(call))
                .find(|event| {
                    matches!(
                        event.kind(),
                        EventKind::ToolExecutionStarted | EventKind::ApprovalRequest
                    )
                })
                .is_some_and(|event| *event.kind() == EventKind::ToolExecutionStarted),
        });
    }
    Ok(pending)
}

fn append_aborted_tool_result(
    ledger: &mut LockedLedger,
    options: &Options,
    call: &str,
    turn: u64,
    reason: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"tool_result",
        "ts":options.event_timestamp(),"call":call,"outcome":{"aborted":reason}
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(())
}

pub(crate) fn recover_provider_attempt_for_ordinary(
    ledger: &mut LockedLedger,
    options: &Options,
    stdout: &mut impl Write,
) -> Result<RecoveryProgress, Box<dyn std::error::Error>> {
    let Some(attempt) = unresolved_attempt(ledger)? else {
        return Ok(RecoveryProgress::Continue { reset_epoch: false });
    };
    let adapter_name = ledger
        .projection()
        .ok_or("ledger projection missing")?
        .events
        .iter()
        .find(|event| {
            *event.kind() == EventKind::Epoch
                && event.string_field("id") == Some(attempt.epoch.as_str())
        })
        .and_then(|event| event.string_field("adapter"))
        .ok_or("recovered attempt epoch is missing")?;
    let dialect = DialectId::from_str(adapter_name)?;
    let capabilities = dialect.family().capabilities();
    let decision = match attempt.decision.as_deref() {
        Some("not_dispatched") => RecoveryDecision::NotDispatched,
        Some("adopt") => RecoveryDecision::Adopt,
        Some("resend") => RecoveryDecision::Resend,
        Some("recovery_epoch") => RecoveryDecision::RecoveryEpoch,
        Some("unresolved") => RecoveryDecision::Unresolved,
        Some(other) => return Err(format!("unknown recorded recovery decision {other}").into()),
        None => decide_recovery(
            AdapterCapabilities {
                continuation: if dialect.server_managed() {
                    Continuation::ServerManaged
                } else {
                    Continuation::Stateless
                },
                query_by_identity: if capabilities.query_by_identity {
                    QueryCapability::Available
                } else {
                    QueryCapability::None
                },
                dispatch_marker_required: capabilities.dispatch_marker_required,
            },
            sent_state(attempt.dispatched, capabilities.dispatch_marker_required),
            QueryResult::Unsupported,
            RunMode::Ordinary,
        ),
    };
    if attempt.decision.is_none() {
        let decision_name = match decision {
            RecoveryDecision::NotDispatched => "not_dispatched",
            RecoveryDecision::Adopt => "adopt",
            RecoveryDecision::Resend => "resend",
            RecoveryDecision::RecoveryEpoch => "recovery_epoch",
            RecoveryDecision::Unresolved => "unresolved",
        };
        let event = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":attempt.turn,
            "kind":"attempt_recovery","ts":options.event_timestamp(),
            "attempt":attempt.id,"decision":decision_name
        }))?;
        ledger.append_contract(event, BarrierContext::default())?;
    }
    if matches!(decision, RecoveryDecision::Adopt) {
        return if complete_adopt(ledger, options, stdout, attempt, false)? {
            Ok(RecoveryProgress::Continue { reset_epoch: false })
        } else {
            Ok(RecoveryProgress::Stop)
        };
    }

    let unresolved = matches!(decision, RecoveryDecision::Unresolved);
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":attempt.turn,"kind":"error",
        "ts":options.event_timestamp(),"attempt":attempt.id,"usage":usage_object(None),
        "recoverable":!unresolved,
        "classification":if unresolved { "unresolved_dispatch" } else { "transport" }
    }))?;
    let outcome_seq = event.seq();
    ledger.append_contract(event, BarrierContext::default())?;
    stdout.write_all(&encode_line(
        "attempt_settled",
        &AttemptSettled {
            attempt: attempt.id,
            outcome_seq,
        },
    )?)?;
    stdout.flush()?;
    if unresolved {
        append_settle(
            ledger,
            &options.event_timestamp(),
            attempt.turn,
            "interrupted",
            Some("recovered"),
        )?;
        return Ok(RecoveryProgress::Stop);
    }
    Ok(RecoveryProgress::Continue {
        reset_epoch: matches!(decision, RecoveryDecision::RecoveryEpoch),
    })
}

pub(crate) fn unresolved_attempt(
    ledger: &LockedLedger,
) -> Result<Option<UnresolvedAttempt>, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let mut attempts = std::collections::BTreeMap::<String, UnresolvedAttempt>::new();
    let mut call_attempts = std::collections::BTreeMap::<String, String>::new();
    for event in &projection.events {
        match event.kind() {
            EventKind::Attempt => {
                let id = event
                    .string_field("attempt")
                    .ok_or("attempt lacks id")?
                    .to_owned();
                attempts.insert(
                    id.clone(),
                    UnresolvedAttempt {
                        id,
                        turn: event.turn().ok_or("attempt lacks turn")?,
                        epoch: event
                            .string_field("epoch")
                            .ok_or("attempt lacks epoch")?
                            .to_owned(),
                        dispatched: false,
                        decision: None,
                        inventory: Vec::new(),
                        response_asset: None,
                        outcome_seq: None,
                        reasoning_present: false,
                        calls: std::collections::BTreeSet::new(),
                        results: std::collections::BTreeSet::new(),
                    },
                );
            }
            EventKind::AttemptDispatched => {
                if let Some(attempt) = event
                    .string_field("attempt")
                    .and_then(|id| attempts.get_mut(id))
                {
                    attempt.dispatched = true;
                }
            }
            EventKind::AttemptRecovery => {
                if let Some(attempt) = event
                    .string_field("attempt")
                    .and_then(|id| attempts.get_mut(id))
                {
                    attempt.decision = event.string_field("decision").map(str::to_owned);
                    let raw = serde_json::to_value(event.raw())?;
                    attempt.inventory = raw
                        .get("inventory")
                        .and_then(Value::as_array)
                        .into_iter()
                        .flatten()
                        .filter_map(Value::as_str)
                        .map(str::to_owned)
                        .collect();
                    attempt.response_asset = raw
                        .pointer("/response/asset")
                        .and_then(Value::as_str)
                        .map(str::to_owned);
                }
            }
            EventKind::Reasoning => {
                if let Some(attempt) = event
                    .string_field("attempt")
                    .and_then(|id| attempts.get_mut(id))
                {
                    attempt.reasoning_present = true;
                }
            }
            EventKind::Output | EventKind::Error => {
                if let Some(id) = event.string_field("attempt") {
                    if attempts
                        .get(id)
                        .is_some_and(|attempt| attempt.decision.as_deref() == Some("adopt"))
                    {
                        if let Some(attempt) = attempts.get_mut(id) {
                            attempt.outcome_seq = Some(event.seq());
                        }
                    } else {
                        attempts.remove(id);
                    }
                }
            }
            EventKind::ToolCall => {
                if let (Some(attempt_id), Some(call)) =
                    (event.string_field("attempt"), event.string_field("call"))
                {
                    call_attempts.insert(call.to_owned(), attempt_id.to_owned());
                    if let Some(attempt) = attempts.get_mut(attempt_id) {
                        attempt.calls.insert(call.to_owned());
                    }
                }
            }
            EventKind::ToolResult => {
                if let Some(call) = event.string_field("call") {
                    if let Some(attempt_id) = call_attempts.get(call) {
                        if let Some(attempt) = attempts.get_mut(attempt_id) {
                            attempt.results.insert(call.to_owned());
                        }
                    }
                }
            }
            EventKind::Settle => {
                if let Some(turn) = event.turn() {
                    attempts.retain(|_, attempt| attempt.turn != turn);
                }
            }
            _ => {}
        }
    }
    if attempts.len() > 1 {
        return Err("ledger has more than one live provider attempt".into());
    }
    Ok(attempts.into_values().next())
}

// Recovery may reuse a completed call, but may never change its durable facts.
pub(crate) fn validate_adopted_calls(
    ledger: &LockedLedger,
    attempt: &str,
    calls: &[provider::ToolCall],
) -> Result<(), Box<dyn std::error::Error>> {
    for event in &ledger
        .projection()
        .ok_or("ledger projection missing")?
        .events
    {
        if event.kind() != &EventKind::ToolCall || event.string_field("attempt") != Some(attempt) {
            continue;
        }
        let call_id = event
            .string_field("call")
            .ok_or("durable call identity missing")?;
        let call = calls
            .iter()
            .find(|call| call.call_id == call_id)
            .ok_or("adopt response omitted a durable tool call")?;
        let raw = serde_json::to_value(event.raw())?;
        let arguments =
            materialize_json(ledger, raw.get("args").ok_or("durable call args missing")?)?;
        if event.string_field("name") != Some(call.name.as_str()) || arguments != call.arguments {
            return Err("adopt response changed a durable tool call".into());
        }
    }
    Ok(())
}

fn complete_adopt(
    ledger: &mut LockedLedger,
    options: &Options,
    stdout: &mut impl Write,
    attempt: UnresolvedAttempt,
    reconcile_only: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let response_asset = attempt
        .response_asset
        .as_deref()
        .ok_or("adopt decision lacks response asset")?;
    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
    let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(response_asset)?;
    let adapter_name = ledger
        .projection()
        .ok_or("ledger projection missing")?
        .events
        .iter()
        .find(|event| {
            *event.kind() == EventKind::Epoch
                && event.string_field("id") == Some(attempt.epoch.as_str())
        })
        .and_then(|event| event.string_field("adapter"))
        .ok_or("adopt attempt epoch is missing")?;
    let dialect = DialectId::from_str(adapter_name)?;
    let mut terminal = provider::normalize_dialect_response(dialect, &bytes)?;
    let final_answer = terminal.is_final_answer();
    let actual_inventory = terminal
        .tool_calls
        .iter()
        .map(|call| call.call_id.as_str())
        .collect::<Vec<_>>();
    let expected_inventory = attempt
        .inventory
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    if actual_inventory != expected_inventory {
        return Err("adopt response call inventory mismatch".into());
    }
    let mut wire_ids = std::collections::BTreeMap::new();
    for call in &mut terminal.tool_calls {
        repair_provider_call_arguments(dialect, call);
        let wire_id = call.call_id.clone();
        call.call_id = local_provider_call_id(ledger, &attempt.id, &wire_id)?;
        if call.call_id != wire_id {
            wire_ids.insert(call.call_id.clone(), wire_id);
        }
    }
    validate_adopted_calls(ledger, &attempt.id, &terminal.tool_calls)?;
    if attempt.outcome_seq.is_none() && !attempt.reasoning_present {
        for content in &terminal.content {
            let ContentBlock::Reasoning(text) = content else {
                continue;
            };
            let content = sealed_fragments(ledger, text.as_bytes())?;
            let event = make_event(json!({
                "v":1,"seq":ledger.next_seq(),"turn":attempt.turn,"kind":"reasoning",
                "ts":options.event_timestamp(),"attempt":attempt.id,"content":content
            }))?;
            ledger.append_contract(event, BarrierContext::default())?;
        }
    }
    if matches!(
        terminal.finish_reason,
        FinishReason::ContentFilter | FinishReason::ProviderError
    ) {
        let outcome_seq = if let Some(seq) = attempt.outcome_seq {
            seq
        } else {
            let event = make_event(json!({
                "v":1,"seq":ledger.next_seq(),"turn":attempt.turn,"kind":"error",
                "ts":options.event_timestamp(),"attempt":attempt.id,"recoverable":false,
                "classification":"provider_terminal","usage":usage_object(terminal.usage.as_ref())
            }))?;
            let seq = event.seq();
            ledger.append_contract(event, BarrierContext::default())?;
            seq
        };
        stdout.write_all(&encode_line(
            "attempt_settled",
            &AttemptSettled {
                attempt: attempt.id,
                outcome_seq,
            },
        )?)?;
        stdout.flush()?;
        if reconcile_only {
            append_settle(
                ledger,
                &options.event_timestamp(),
                attempt.turn,
                "interrupted",
                Some("recovered"),
            )?;
        } else {
            append_settle(
                ledger,
                &options.event_timestamp(),
                attempt.turn,
                "error",
                Some("provider_terminal"),
            )?;
        }
        return Ok(false);
    }
    let outcome_seq = if let Some(seq) = attempt.outcome_seq {
        seq
    } else {
        let mut content = Vec::new();
        for block in &terminal.content {
            if let ContentBlock::Text(text) = block {
                content.push(json!({"type":"text","text":text}));
            }
        }
        let native = serde_json_canonicalizer::to_vec(&terminal.sealed_fragments)?;
        let fragments = sealed_fragments(ledger, &native)?;
        let mut value = json!({
            "v":1,"seq":ledger.next_seq(),"turn":attempt.turn,"kind":"output",
            "ts":options.event_timestamp(),"attempt":attempt.id,"content":content,
            "final_answer":final_answer,"usage":usage_object(terminal.usage.as_ref()),
            "sealed":{"version":1,"adapter":dialect.as_str(),"fragments":fragments}
        });
        if dialect.server_managed() {
            let identity = terminal
                .response_identity
                .as_deref()
                .ok_or("adopted server-managed response lacks continuation identity")?;
            value
                .as_object_mut()
                .expect("output object")
                .insert("continuation".to_owned(), json!({"id":identity}));
        }
        let event = make_event(value)?;
        let seq = event.seq();
        ledger.append_contract(event, BarrierContext::default())?;
        seq
    };
    stdout.write_all(&encode_line(
        "attempt_settled",
        &AttemptSettled {
            attempt: attempt.id.clone(),
            outcome_seq,
        },
    )?)?;
    stdout.flush()?;
    for call in terminal.tool_calls {
        if !attempt.calls.contains(&call.call_id) {
            let mut call_raw = json!({
                "v":1,"seq":ledger.next_seq(),"turn":attempt.turn,"kind":"tool_call",
                "ts":options.event_timestamp(),"attempt":attempt.id,"call":call.call_id,
                "name":call.name,"args":spill_json(ledger, &call.arguments)?,"source":"provider"
            });
            if let Some(wire_id) = wire_ids.get(&call.call_id) {
                call_raw["provider_call"] = json!(wire_id);
            }
            let call_event = make_event(call_raw)?;
            ledger.append_contract(call_event, BarrierContext::default())?;
        }
        if !attempt.results.contains(&call.call_id) {
            let result = make_event(json!({
                "v":1,"seq":ledger.next_seq(),"turn":attempt.turn,"kind":"tool_result",
                "ts":options.event_timestamp(),"call":call.call_id,"outcome":{"aborted":"recovered"}
            }))?;
            ledger.append_contract(result, BarrierContext::default())?;
        }
    }
    if reconcile_only {
        append_settle(
            ledger,
            &options.event_timestamp(),
            attempt.turn,
            "interrupted",
            Some("recovered"),
        )?;
        return Ok(false);
    }
    match terminal.finish_reason {
        FinishReason::Completed if final_answer => {
            // The recovered response is a candidate. The ordinary execution
            // path owns validation, just as for a newly received final.
            Ok(true)
        }
        FinishReason::Length => {
            append_settle(
                ledger,
                &options.event_timestamp(),
                attempt.turn,
                "interrupted",
                Some("budget_tokens"),
            )?;
            Ok(false)
        }
        FinishReason::Completed | FinishReason::ToolCalls | FinishReason::Paused => Ok(true),
        FinishReason::ContextOverflow => {
            unreachable!("adopt query does not classify overflow as a successful outcome")
        }
        FinishReason::ContentFilter | FinishReason::ProviderError => {
            unreachable!("handled before adopted output")
        }
    }
}
