use super::*;

/// Provider retries per turn under the rate-limit and transport budget.
pub(crate) const PROVIDER_RETRIES: u8 = 3;
const PROVIDER_ADMISSION_FLOOR: Duration = Duration::from_secs(1);
const PROVIDER_ADMISSION_CEILING: Duration = Duration::from_secs(30);
pub(crate) const PROVIDER_ADMISSION_SUBKIND: &str = "provider_admission";

/// The response terminal may name a call again only when this attempt already
/// dispatched it eagerly, with the same name and materialized arguments. Any
/// other durable id — another attempt's call, a replayed id, a duplicate within
/// the manifest — and any eager call the manifest omits fail the terminal.
pub(crate) fn validate_terminal_tool_calls(
    ledger: &LockedLedger,
    attempt: &str,
    eager: &EagerDispatch,
    calls: &[provider::ToolCall],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut durable = std::collections::BTreeMap::<&str, (&str, &str, &Value)>::new();
    let events = &ledger
        .projection()
        .ok_or("ledger projection missing")?
        .events;
    let mut raws = Vec::with_capacity(events.len());
    for event in events {
        if *event.kind() != EventKind::ToolCall {
            continue;
        }
        raws.push((event, serde_json::to_value(event.raw())?));
    }
    for (event, raw) in &raws {
        durable.insert(
            event
                .string_field("call")
                .ok_or("durable call identity missing")?,
            (
                event
                    .string_field("attempt")
                    .ok_or("durable call lacks attempt")?,
                event
                    .string_field("name")
                    .ok_or("durable call lacks name")?,
                raw.get("args").ok_or("durable call args missing")?,
            ),
        );
    }
    let mut incoming = BTreeSet::new();
    for call in calls {
        if call.call_id.is_empty() || !incoming.insert(call.call_id.as_str()) {
            return Err(format!(
                "provider tool call id {} is empty or duplicated",
                call.call_id
            )
            .into());
        }
        let Some((owner, name, args)) = durable.get(call.call_id.as_str()) else {
            continue;
        };
        if *owner != attempt || !eager.contains(&call.call_id) {
            return Err(format!(
                "provider tool call id {} reuses a durable call",
                call.call_id
            )
            .into());
        }
        if *name != call.name || materialize_json(ledger, args)? != call.arguments {
            return Err(format!(
                "response terminal changed eagerly dispatched call {}",
                call.call_id
            )
            .into());
        }
    }
    for call in &eager.calls {
        if !incoming.contains(call.call_id.as_str()) {
            return Err(format!(
                "response terminal omitted eagerly dispatched call {}",
                call.call_id
            )
            .into());
        }
    }
    Ok(())
}

pub(crate) struct TerminalAppend<'a> {
    pub(crate) options: &'a Options,
    pub(crate) manifest: &'a BuiltinManifest,
    pub(crate) eager: &'a EagerDispatch,
    pub(crate) dialect: DialectId,
    pub(crate) server_managed: bool,
    pub(crate) turn: u64,
    pub(crate) attempt: &'a str,
}

pub(crate) fn append_terminal(
    ledger: &mut LockedLedger,
    append: TerminalAppend<'_>,
    mut terminal: ProviderTerminal,
) -> Result<ProviderOutcome, Box<dyn std::error::Error>> {
    let TerminalAppend {
        options,
        manifest,
        eager,
        dialect,
        server_managed,
        turn,
        attempt,
    } = append;
    let mut wire_ids = std::collections::BTreeMap::new();
    for call in &mut terminal.tool_calls {
        repair_provider_call_arguments(dialect, call);
        let wire_id = call.call_id.clone();
        call.call_id = local_provider_call_id(ledger, attempt, &wire_id)?;
        if call.call_id != wire_id {
            wire_ids.insert(call.call_id.clone(), wire_id);
        }
    }
    validate_terminal_tool_calls(ledger, attempt, eager, &terminal.tool_calls)?;
    if server_managed && terminal.response_identity.is_none() {
        return Err("server-managed provider terminal lacks continuation identity".into());
    }
    let mut output_content = Vec::new();
    for content in &terminal.content {
        match content {
            ContentBlock::Text(text) => output_content.push(json!({"type":"text","text":text})),
            ContentBlock::Reasoning(text) => {
                let content = sealed_fragments(ledger, text.as_bytes())?;
                let event = make_event(json!({
                    "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"reasoning",
                    "ts":options.event_timestamp(),"attempt":attempt,"content":content
                }))?;
                ledger.append_contract(event, BarrierContext::default())?;
            }
        }
    }
    for call in &terminal.tool_calls {
        // An eagerly dispatched call is already the durable write-ahead intent
        // (validated above to match); writing it again would reuse its id.
        if eager.contains(&call.call_id) {
            continue;
        }
        append_provider_tool_call_with_wire_id(
            ledger,
            options,
            manifest,
            turn,
            attempt,
            call,
            wire_ids.get(&call.call_id).map(String::as_str),
        )?;
    }
    let native = serde_json_canonicalizer::to_vec(&terminal.sealed_fragments)?;
    let fragments = sealed_fragments(ledger, &native)?;
    let mut output = json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"output",
        "ts":options.event_timestamp(),"attempt":attempt,"content":output_content,
        "final_answer":terminal.is_final_answer(),"usage":usage_object(terminal.usage.as_ref()),
        "sealed":{"version":1,"adapter":dialect.as_str(),"fragments":fragments}
    });
    if server_managed {
        if let Some(identity) = terminal.response_identity.as_deref() {
            output
                .as_object_mut()
                .expect("output object")
                .insert("continuation".to_owned(), json!({"id":identity}));
        }
    }
    let event = make_event(output)?;
    let outcome_seq = event.seq();
    ledger.append_contract(event, BarrierContext::default())?;
    let settle = match terminal.finish_reason {
        FinishReason::Completed if terminal.is_final_answer() => Some(("completed", None)),
        FinishReason::Completed => None,
        FinishReason::Length => Some(("interrupted", Some("budget_tokens"))),
        // A paused turn is resent with its output in history, like a
        // non-final completion; the continuation budget bounds the loop.
        FinishReason::ToolCalls | FinishReason::Paused => None,
        FinishReason::ContextOverflow
        | FinishReason::ContentFilter
        | FinishReason::ProviderError => unreachable!("handled above"),
    };
    Ok(ProviderOutcome {
        seq: outcome_seq,
        settle,
        retry: false,
        retry_classification: None,
        retry_after_seconds: None,
        compact_retry: false,
        tool_calls: terminal.tool_calls,
    })
}

pub(crate) fn append_provider_terminal_error(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    attempt: &str,
    terminal: &ProviderTerminal,
) -> Result<ProviderOutcome, Box<dyn std::error::Error>> {
    let mut value = json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"error",
        "ts":options.event_timestamp(),"attempt":attempt,"recoverable":false,
        "classification":"provider_terminal","usage":usage_object(terminal.usage.as_ref())
    });
    let detail = if terminal.finish_reason == FinishReason::ContentFilter {
        // Anthropic refusals name a category (`cyber`, `bio`, ...); keep it
        // beside the classification so the fallback decision has its input.
        Some(
            match terminal
                .stop_details
                .as_ref()
                .and_then(|details| details.get("category"))
                .and_then(Value::as_str)
            {
                Some(category) => bounded_ledger_detail(&format!("content_filter: {category}")),
                None => "content_filter".to_owned(),
            },
        )
    } else {
        terminal
            .provider_error
            .as_ref()
            .and_then(provider_error_detail)
            .or_else(|| terminal.incomplete_reason.clone())
            .map(|value| bounded_ledger_detail(&value))
    };
    let detail = match (terminal.http_status, detail) {
        (Some(status), Some(detail)) => {
            Some(bounded_ledger_detail(&format!("http_{status}: {detail}")))
        }
        (Some(status), None) => Some(format!("http_{status}")),
        (None, detail) => detail,
    };
    if let Some(detail) = detail {
        value["detail"] = Value::String(detail);
    }
    let event = make_event(value)?;
    let outcome_seq = event.seq();
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(ProviderOutcome {
        seq: outcome_seq,
        settle: Some(("error", Some("provider_terminal"))),
        retry: false,
        retry_classification: None,
        retry_after_seconds: None,
        compact_retry: false,
        tool_calls: Vec::new(),
    })
}

fn provider_error_detail(value: &Value) -> Option<String> {
    let value = value.get("error").unwrap_or(value);
    let fields = ["code", "type", "message"]
        .into_iter()
        .filter_map(|field| {
            value
                .get(field)
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .map(|value| format!("{field}={value}"))
        })
        .collect::<Vec<_>>();
    (!fields.is_empty()).then(|| fields.join(" "))
}

pub(crate) fn append_context_overflow(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    attempt: &str,
    usage: Option<&provider::Usage>,
) -> Result<ProviderOutcome, Box<dyn std::error::Error>> {
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"error",
        "ts":options.event_timestamp(),"attempt":attempt,"recoverable":true,
        "classification":"transport","detail":"context_overflow","usage":usage_object(usage)
    }))?;
    let outcome_seq = event.seq();
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(ProviderOutcome {
        seq: outcome_seq,
        settle: None,
        retry: false,
        retry_classification: None,
        retry_after_seconds: None,
        compact_retry: true,
        tool_calls: Vec::new(),
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn append_provider_failure(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    attempt: &str,
    failure: ProviderFailure,
    cancellation: &RuntimeCancellation,
    dialect: DialectId,
    eager: &EagerDispatch,
) -> Result<ProviderOutcome, Box<dyn std::error::Error>> {
    let (classification, detail, recoverable, retry, settle) = match &failure {
        ProviderFailure::AuthFailure { status, detail } => (
            "provider_terminal",
            detail.as_ref().map_or_else(
                || format!("http_{status}"),
                |detail| format!("http_{status}: {detail}"),
            ),
            false,
            false,
            Some(("error", Some("provider_terminal"))),
        ),
        ProviderFailure::RateLimited {
            status,
            detail,
            retry_after_seconds,
        } => (
            "rate_limit",
            format!(
                "http_{status}{}; retry_after_seconds={}",
                detail
                    .as_ref()
                    .map(|detail| format!(": {detail}"))
                    .unwrap_or_default(),
                retry_after_seconds.map_or_else(|| "unknown".to_owned(), |value| value.to_string())
            ),
            true,
            true,
            None,
        ),
        ProviderFailure::Transport { status, detail, .. } => (
            "transport",
            status.map_or_else(
                || detail.clone(),
                |status| format!("http_{status}: {detail}"),
            ),
            true,
            true,
            None,
        ),
        ProviderFailure::Malformed { detail } => (
            "provider_terminal",
            format!("malformed_response: {detail}"),
            false,
            false,
            Some(("error", Some("provider_terminal"))),
        ),
        ProviderFailure::Cancelled if cancellation.supervisor_lost() => {
            ("transport", "supervisor_lost".to_owned(), true, false, None)
        }
        ProviderFailure::Cancelled if cancellation.stop_requested() => (
            "transport",
            "user_stop".to_owned(),
            false,
            false,
            Some(("interrupted", Some("user_stop"))),
        ),
        ProviderFailure::Cancelled => ("transport", "cancelled".to_owned(), true, false, None),
    };
    let detail = bounded_ledger_detail(&detail);
    let mut error = json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"error",
        "ts":options.event_timestamp(),"attempt":attempt,"recoverable":recoverable,
        "classification":classification,"detail":detail,"usage":usage_object(None)
    });
    if let ProviderFailure::Transport {
        partial_fragments: Some(partial),
        ..
    } = &failure
    {
        if let Some(sealed) = partial_carrier_for_eager_calls(ledger, dialect, eager, partial)? {
            error
                .as_object_mut()
                .expect("error object")
                .insert("sealed".to_owned(), sealed);
        }
    }
    let event = make_event(error)?;
    let outcome_seq = event.seq();
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(ProviderOutcome {
        seq: outcome_seq,
        settle,
        retry,
        retry_classification: retry.then_some(classification),
        retry_after_seconds: match &failure {
            ProviderFailure::RateLimited {
                retry_after_seconds,
                ..
            } => *retry_after_seconds,
            _ => None,
        },
        compact_retry: false,
        tool_calls: Vec::new(),
    })
}

/// The sealed partial carrier for a response that lost its transport after
/// eagerly dispatching calls. Their `tool_call`/`tool_result` records are
/// durable, so the retry replays each result; a provider that binds a call
/// to the native items before it (DeepSeek thinking mode rejects a
/// `function_call` without its `reasoning` item) needs those items verbatim,
/// not the normalized `tool_call` projection. The carrier has the shape of
/// `output.sealed`; it is written only when every eager call is in it, so the
/// renderer can skip the eager `tool_call` records without losing a call.
fn partial_carrier_for_eager_calls(
    ledger: &LockedLedger,
    dialect: DialectId,
    eager: &EagerDispatch,
    partial: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    if eager.calls.is_empty() {
        return Ok(None);
    }
    let Some(items) = partial.as_array() else {
        return Ok(None);
    };
    let sealed_calls = items
        .iter()
        .filter_map(|item| {
            match item.get("type").and_then(Value::as_str)? {
                "tool_use" => item.get("id"),
                _ => item.get("call_id"),
            }
            .and_then(Value::as_str)
        })
        .collect::<BTreeSet<_>>();
    let complete = eager.calls.iter().all(|call| {
        let wire_id = eager.wire_ids.get(&call.call_id).unwrap_or(&call.call_id);
        sealed_calls.contains(wire_id.as_str())
    });
    if !complete {
        return Ok(None);
    }
    let native = serde_json_canonicalizer::to_vec(partial)?;
    let fragments = sealed_fragments(ledger, &native)?;
    Ok(Some(
        json!({"version":1,"adapter":dialect.as_str(),"fragments":fragments}),
    ))
}

/// Why a provider attempt is waiting and how much retry budget is left.
pub(crate) struct ProviderAdmissionRetry<'a> {
    pub(crate) classification: &'a str,
    pub(crate) retry_after_seconds: Option<u64>,
    pub(crate) retries_left: u8,
}

/// Durable admission wait before a rate-limited or transport retry. The delay is the
/// provider's `Retry-After` when declared, otherwise a doubling backoff from
/// one second; both are clamped to `PROVIDER_ADMISSION_CEILING`. The wait is
/// written as a runtime-visible `state{subkind: "provider_admission"}` record
/// carrying `next_attempt_at`, so a worker killed mid-wait leaves the retry as
/// a durable obligation of the open turn (tail-lifecycle `admission_wait_until`)
/// rather than an unsolicited resume, and a stop or supervisor loss cuts the
/// wait short.
pub(crate) fn wait_provider_admission(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    attempt: &str,
    retry: ProviderAdmissionRetry<'_>,
    cancellation: &RuntimeCancellation,
) -> Result<(), Box<dyn std::error::Error>> {
    let ProviderAdmissionRetry {
        classification,
        retry_after_seconds,
        retries_left,
    } = retry;
    let exhausted = PROVIDER_RETRIES.saturating_sub(retries_left);
    let backoff = Duration::from_secs(1u64 << u32::from(exhausted.min(8)));
    let wait = retry_after_seconds
        .map_or(backoff, Duration::from_secs)
        .clamp(PROVIDER_ADMISSION_FLOOR, PROVIDER_ADMISSION_CEILING);
    let next_attempt_at = rfc3339_after(wait);
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"state",
        "ts":options.event_timestamp(),"visibility":"runtime",
        "subkind":PROVIDER_ADMISSION_SUBKIND,
        "payload":{
            "attempt":attempt,"classification":classification,
            "next_attempt_at":next_attempt_at,"wait_ms":wait.as_millis() as u64,
            "retries_left":retries_left,"declared_retry_after":retry_after_seconds.is_some()
        }
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    let deadline = Instant::now() + wait;
    while Instant::now() < deadline {
        if cancellation.stop_requested() || cancellation.supervisor_lost() {
            break;
        }
        std::thread::sleep((deadline - Instant::now()).min(Duration::from_millis(100)));
    }
    Ok(())
}

pub(crate) fn settle_internal_worker_failure(
    ledger: &mut LockedLedger,
    options: &Options,
    error: &(dyn std::error::Error + 'static),
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(projection) = ledger.projection() else {
        return Err("worker ledger projection missing while reporting failure".into());
    };
    if projection.terminal_tail
        || projection.lifecycle.open_hold
        || unresolved_attempt(ledger)?.is_some()
    {
        return Err(error.to_string().into());
    }
    let Some(turn) = projection.latest_turn else {
        return Err(error.to_string().into());
    };
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"error",
        "ts":options.event_timestamp(),"recoverable":false,
        "classification":"internal","detail":bounded_ledger_detail(&error.to_string())
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    append_settle(
        ledger,
        &options.event_timestamp(),
        turn,
        "error",
        Some("internal"),
    )
}

pub(crate) fn bounded_ledger_detail(value: &str) -> String {
    let sanitized = value.replace(['\r', '\n', '\t'], " ");
    let scanned = IJsonValue::parse(
        &serde_json::to_vec(&sanitized).expect("diagnostic string is JSON serializable"),
    )
    .map(|value| SecretScanner::default().scan(&value));
    let sanitized = match scanned {
        Ok(SecretScan::Clean(value) | SecretScan::Redacted(value)) => serde_json::to_value(value)
            .ok()
            .and_then(|value| value.as_str().map(str::to_owned))
            .unwrap_or_else(|| "diagnostic unavailable".to_owned()),
        Ok(SecretScan::Withheld(_)) | Err(_) => "diagnostic withheld".to_owned(),
    };
    let mut end = sanitized.len().min(512);
    while !sanitized.is_char_boundary(end) {
        end -= 1;
    }
    sanitized[..end].to_owned()
}

/// The `usage` object carried by the `output`/`error` that settles an
/// attempt: `unavailable` when the provider reported no figures.
pub(crate) fn usage_object(usage: Option<&provider::Usage>) -> Value {
    let mut value = json!({
        "availability": if usage.is_some() { "reported" } else { "unavailable" }
    });
    if let Some(usage) = usage {
        let object = value.as_object_mut().expect("usage object");
        for (field, figure) in [
            ("input_tokens", usage.input_tokens.as_ref()),
            ("output_tokens", usage.output_tokens.as_ref()),
            ("cache_read", usage.cache_read.as_ref()),
            ("cache_miss", usage.cache_miss.as_ref()),
            ("cache_write", usage.cache_write.as_ref()),
            ("reasoning_tokens", usage.reasoning_tokens.as_ref()),
        ] {
            if let Some(figure) = figure {
                object.insert(field.to_owned(), Value::String(figure.clone()));
            }
        }
    }
    value
}

pub(crate) fn record_input_transformations(
    ledger: &mut LockedLedger,
    timestamp: &str,
    turn: u64,
    attempt: &str,
    changes: &Value,
) -> Result<(), Box<dyn std::error::Error>> {
    // Runtime visibility keeps diagnostics out of the cached prompt. Large
    // reports use the existing spill carrier instead of exceeding ledger limits.
    let changes = sealed_fragments(ledger, &serde_json_canonicalizer::to_vec(changes)?)?;
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"state",
        "ts":timestamp,"visibility":"runtime",
        "subkind":"provider_input_transformations",
        "payload":{"attempt":attempt,"input_transformations":changes}
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(())
}

pub(crate) fn sealed_fragments(
    ledger: &LockedLedger,
    bytes: &[u8],
) -> Result<Value, Box<dyn std::error::Error>> {
    let text = std::str::from_utf8(bytes)?;
    let encoded = serde_json_canonicalizer::to_vec(&Value::String(text.to_owned()))?;
    if encoded.len() <= 16 * 1024 {
        return Ok(Value::String(text.to_owned()));
    }
    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
    let asset = store::AssetStore::new(folder.join("assets"))?.publish(&encoded)?;
    Ok(json!({"$spill":{"asset":asset.asset,"bytes":asset.bytes}}))
}

pub(crate) fn spill_json(
    ledger: &LockedLedger,
    value: &Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let bytes = serde_json_canonicalizer::to_vec(value)?;
    if bytes.len() <= 16 * 1024 {
        return Ok(value.clone());
    }
    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
    let asset = AssetStore::new(folder.join("assets"))?.publish(&bytes)?;
    Ok(json!({"$spill":{"asset":asset.asset,"bytes":asset.bytes}}))
}

pub(crate) fn append_settle(
    ledger: &mut LockedLedger,
    timestamp: &str,
    turn: u64,
    outcome: &str,
    reason: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut value = json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"settle",
        "ts":timestamp,"outcome":outcome
    });
    if let Some(reason) = reason {
        value.as_object_mut().expect("settle object").insert(
            if outcome == "error" {
                "classification"
            } else {
                "reason"
            }
            .to_owned(),
            Value::String(reason.to_owned()),
        );
    }
    ledger.append_contract(make_event(value)?, BarrierContext::default())?;
    Ok(())
}
