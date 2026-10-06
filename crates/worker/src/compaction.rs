use super::*;

pub(crate) const CONTEXT_RENDERER_VERSION: u64 = 3;

pub(crate) fn auto_compact_for_turn(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    summary: Option<engine::CompactionSummary>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let Some(object) = build_compaction_event(ledger, options, turn, false, summary)? else {
        return Ok(false);
    };
    ledger.append_contract(
        make_event(Value::Object(object))?,
        BarrierContext::default(),
    )?;
    Ok(true)
}

/// `summary` is the model summary frozen over this plan (event §compact
/// `summary_request`): its admitted continuation is the compact summary and
/// the deterministic quoted history the fallback. A worker-authored manual
/// compact at a yield holds no provider lease and carries none.
pub(crate) fn build_compaction_event(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    manual: bool,
    summary: Option<engine::CompactionSummary>,
) -> Result<Option<Map<String, Value>>, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let plan = engine::plan_context_compaction(&projection.events, turn)?;
    if plan.covers.is_empty() && !manual {
        return Ok(None);
    }
    let covered = plan.covers;
    if let Some(hooks) = &options.lifecycle_hooks {
        ledger.sync_prefix()?;
        hooks.before_compact(ledger, turn, &covered, manual);
    }
    let (summary_record, summary) = match summary.as_ref() {
        Some(model_summary) => {
            let (record, text) = model_summary.apply(&covered, plan.summary);
            (Some(record), text)
        }
        None => (None, plan.summary),
    };
    ledger.create_checkpoint(&options.event_timestamp(), "auto-compaction boundary")?;
    let summary = sealed_fragments(ledger, summary.as_bytes())?;
    let mut object = json!({
        "v":1,"seq":ledger.next_seq(),"kind":"compact","ts":options.event_timestamp(),
        "covers":ranges(&covered),"summary":summary
    })
    .as_object()
    .unwrap()
    .clone();
    if let Some(record) = summary_record {
        object.insert("summary_request".to_owned(), record);
    }
    Ok(Some(object))
}

/// The model summary for the compaction this turn is about to write: freeze
/// the planned covers, run one bounded compactor request (tools:
/// `summary_artifact` only) inside the attempt's lease window, admit the
/// artifact against the frozen bundle. Never fails the turn: every failure is
/// the record's rejection reason and the deterministic quoted history stands.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_compaction_summary(
    ledger: &LockedLedger,
    profile: &RuntimeProfile,
    provider_config: &profile::Provider,
    model: &profile::Model,
    resolved: &provider::ResolvedDialectProfile,
    credential_material: &str,
    transport_endpoint: Option<&str>,
    cancelled: &Arc<std::sync::atomic::AtomicBool>,
    turn: u64,
    attempt: &str,
) -> Result<Option<engine::CompactionSummary>, Box<dyn std::error::Error>> {
    let _ = profile;
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let plan = engine::plan_context_compaction(&projection.events, turn)?;
    if plan.covers.is_empty() {
        return Ok(None);
    }
    let bundle = engine::freeze_source_bundle(
        &projection.events,
        &plan.covers,
        engine::summary_request_bytes(model.context_window_tokens),
    )?;
    let outcome = (|| -> Result<_, Box<dyn std::error::Error>> {
        let schema =
            tools::fixed_schema("summary_artifact").ok_or("summary_artifact schema missing")?;
        let catalog = ijson(json!([schema.model_schema()]))?;
        let prepared = provider::prepare_summary_request(
            &provider_config.endpoint,
            resolved,
            engine::SUMMARY_SYSTEM,
            &bundle.rendered,
            catalog,
            format!("{attempt}-summary"),
        )?;
        let runtime = HttpRuntime::new()?;
        Ok(runtime.send_dialect_with_frames_and_wall_transport(
            resolved.dialect,
            &prepared,
            transport_endpoint,
            credential_material,
            cancelled,
            Some(Duration::from_secs(COMPACTION_SUMMARY_TIMEOUT_SECONDS)),
            |_| Ok(()),
        ))
    })();
    let (admission, usage) = match outcome {
        Ok(completion) => {
            let (artifact, usage) = provider::summary_completion_artifact(completion);
            (
                artifact.and_then(|arguments| engine::admit_summary_artifact(&bundle, &arguments)),
                usage,
            )
        }
        Err(error) => (Err(format!("summary request: {error}")), None),
    };
    Ok(Some(engine::CompactionSummary::from_outcome(
        &model.id, bundle, admission, usage,
    )))
}

const COMPACTION_SUMMARY_TIMEOUT_SECONDS: u64 = 120;

/// Estimate new tokens from the provider's last reported input count when
/// the current request preserves that request as an exact prefix. Each newly
/// serialized byte is charged as one possible token, with room for framing.
/// Without a compatible observation, retain the conservative byte bound.
pub(crate) fn preflight_compaction_due(
    ledger: &LockedLedger,
    prepared: &provider::PreparedRequest,
    model: &profile::Model,
    epoch: &str,
) -> bool {
    if prepared.candidate_bytes <= model.compact_trigger_tokens {
        return false;
    }
    observed_prefix_token_bound(ledger, &prepared.body, epoch)
        .is_none_or(|bound| bound > model.compact_trigger_tokens)
}

pub(crate) fn observed_prefix_token_bound(
    ledger: &LockedLedger,
    body: &[u8],
    epoch: &str,
) -> Option<u64> {
    let events = &ledger.projection()?.events;
    let output = events.iter().rev().find_map(|event| {
        if *event.kind() != EventKind::Output {
            return None;
        }
        let raw = serde_json::to_value(event.raw()).ok()?;
        let usage = raw.get("usage")?;
        if usage.get("availability")?.as_str()? != "reported" {
            return None;
        }
        let input = usage.get("input_tokens")?;
        Some((
            raw.get("attempt")?.as_str()?.to_owned(),
            input
                .as_u64()
                .or_else(|| input.as_str()?.parse::<u64>().ok())?,
        ))
    })?;
    let attempt = events.iter().rev().find_map(|event| {
        if *event.kind() != EventKind::Attempt {
            return None;
        }
        let raw = serde_json::to_value(event.raw()).ok()?;
        if raw.get("attempt")?.as_str()? != output.0 || raw.get("epoch")?.as_str()? != epoch {
            return None;
        }
        raw.get("request")?
            .get("asset")?
            .as_str()
            .map(ToOwned::to_owned)
    })?;
    let folder = ledger.path().parent()?;
    let prior_bytes = store::AssetStore::new(folder.join("assets"))
        .ok()?
        .read_verified(&attempt)
        .ok()?;
    prefix_preserving_token_bound(&prior_bytes, output.1, body)
}

pub(crate) fn prefix_preserving_token_bound(
    prior_bytes: &[u8],
    prior_tokens: u64,
    body: &[u8],
) -> Option<u64> {
    if body.len() < prior_bytes.len() {
        return None;
    }
    let previous: Value = serde_json::from_slice(prior_bytes).ok()?;
    let current: Value = serde_json::from_slice(body).ok()?;
    if !request_preserves_input_prefix(&previous, &current) {
        return None;
    }
    const FRAMING_MARGIN_TOKENS: u64 = 8 * 1024;
    Some(
        prior_tokens
            .saturating_add((body.len() - prior_bytes.len()) as u64)
            .saturating_add(FRAMING_MARGIN_TOKENS),
    )
}

fn request_preserves_input_prefix(previous: &Value, current: &Value) -> bool {
    let (Some(before), Some(after)) = (previous.as_object(), current.as_object()) else {
        return false;
    };
    if before.len() != after.len() {
        return false;
    }
    let input_key = ["input", "messages", "contents"]
        .into_iter()
        .find(|key| before.contains_key(*key) && after.contains_key(*key));
    let Some(input_key) = input_key else {
        return false;
    };
    let (Some(old_input), Some(new_input)) = (
        before.get(input_key).and_then(Value::as_array),
        after.get(input_key).and_then(Value::as_array),
    ) else {
        return false;
    };
    new_input.starts_with(old_input)
        && before
            .iter()
            .all(|(key, value)| key == input_key || after.get(key) == Some(value))
}
