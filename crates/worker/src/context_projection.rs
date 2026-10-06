use super::*;

/// Immutable identity of the deferred catalog this turn declares natively.
pub(crate) fn dynamic_catalog_revision(deferred: &[CatalogEntry]) -> String {
    let mut digest = Sha256::new();
    digest.update(b"tekes-native-deferred-catalog-v1\0");
    for entry in deferred {
        digest.update(entry.name.as_bytes());
        digest.update([0]);
        digest.update(entry.schema_digest.as_bytes());
        digest.update([0]);
    }
    format!("sha256-{:x}", digest.finalize())
}

/// This turn's durable successful `tool_search` offers, as native references.
/// Only names in the declared deferred catalog with the exact offered schema
/// digest are carried; unbound names and stale digests are dropped, never
/// promoted.
pub(crate) fn native_search_references(
    ledger: &LockedLedger,
    turn: u64,
    before_seq: u64,
    deferred: &[CatalogEntry],
    catalog_revision: &str,
) -> Result<Vec<provider::DeferredToolReference>, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let searches = projection
        .events
        .iter()
        .filter(|event| {
            event.seq() < before_seq
                && event.turn() == Some(turn)
                && *event.kind() == EventKind::ToolCall
                && event.string_field("name") == Some("tool_search")
        })
        .filter_map(|event| event.string_field("call").map(str::to_owned))
        .collect::<Vec<_>>();
    let mut references = Vec::new();
    let mut seen = BTreeSet::new();
    for call in &searches {
        let Some(result) = projection.events.iter().find(|event| {
            event.seq() < before_seq
                && *event.kind() == EventKind::ToolResult
                && event.string_field("call") == Some(call.as_str())
        }) else {
            continue;
        };
        let raw = serde_json::to_value(result.raw())?;
        if raw.get("outcome").and_then(Value::as_str) != Some("ok") {
            continue;
        }
        let Some(content) = raw.get("content") else {
            continue;
        };
        let content = materialize_json(ledger, content)?;
        for block in content.as_array().into_iter().flatten() {
            let Some(text) = block.get("text").and_then(Value::as_str) else {
                continue;
            };
            let Ok(offer) = serde_json::from_str::<Value>(text) else {
                continue;
            };
            for item in offer
                .get("items")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                let (Some(name), Some(digest)) = (
                    item.get("name").and_then(Value::as_str),
                    item.get("schema_digest").and_then(Value::as_str),
                ) else {
                    continue;
                };
                let bound = deferred
                    .iter()
                    .any(|entry| entry.name == name && entry.schema_digest == digest);
                if bound && seen.insert((call.clone(), name.to_owned())) {
                    references.push(provider::DeferredToolReference {
                        tool_name: name.to_owned(),
                        schema_digest: digest.to_owned(),
                        catalog_revision: catalog_revision.to_owned(),
                        source_search_call_id: call.clone(),
                    });
                }
            }
        }
    }
    Ok(references)
}

#[cfg(test)]
pub(crate) fn project_provider_context(
    ledger: &LockedLedger,
    epoch_id: &str,
    dialect: DialectId,
    turn: u64,
) -> Result<ProviderContext, Box<dyn std::error::Error>> {
    project_provider_context_mode(ledger, epoch_id, dialect, turn, false)
}

/// `stateless` forces full-prefix replay without a server continuation id.
pub(crate) fn project_provider_context_mode(
    ledger: &LockedLedger,
    epoch_id: &str,
    dialect: DialectId,
    turn: u64,
    stateless: bool,
) -> Result<ProviderContext, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let mut attempt_epoch = std::collections::BTreeMap::<String, String>::new();
    let mut attempt_admits = std::collections::BTreeMap::<String, Vec<u64>>::new();
    let mut settled_outputs = std::collections::BTreeSet::<String>::new();
    // Attempts whose response was lost in transport after eager dispatch and
    // whose recoverable `error` seals the completed native items: the carrier
    // stands in for the output as the model-visible assistant turn.
    let mut partial_carriers = std::collections::BTreeSet::<String>::new();
    // Where an attempt's assistant carrier (output or partial) renders; a
    // result of a call dispatched before that carrier renders right after it.
    let mut output_positions = std::collections::BTreeMap::<String, u64>::new();
    let mut call_attempts = std::collections::BTreeMap::<String, String>::new();
    let mut superseded = std::collections::BTreeSet::<u64>::new();
    let mut compacted = std::collections::BTreeSet::<u64>::new();
    let mut input_turn = std::collections::BTreeMap::<u64, u64>::new();
    let mut input_position = std::collections::BTreeMap::<u64, u64>::new();
    let mut call_turn = std::collections::BTreeMap::<String, u64>::new();
    let mut call_name = std::collections::BTreeMap::<String, String>::new();
    let mut spawned_calls = std::collections::BTreeSet::<String>::new();
    let mut open_turn = None;
    let mut continuation_id = None;
    let mut epoch_seq = None;
    let mut epoch_pending = std::collections::BTreeSet::<u64>::new();

    for event in &projection.events {
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("event is not an object")?;
        for seq in seqs_from_ranges(object.get("supersedes"))? {
            superseded.insert(seq);
        }
        if *event.kind() == EventKind::Compact {
            for seq in seqs_from_ranges(object.get("covers"))? {
                compacted.insert(seq);
            }
        }
        match event.kind() {
            EventKind::Epoch if event.string_field("id") == Some(epoch_id) => {
                epoch_seq = Some(event.seq());
                for seq in object
                    .get("pending")
                    .and_then(Value::as_object)
                    .into_iter()
                    .flat_map(|pending| [pending.get("eligible"), pending.get("withheld")])
                    .flatten()
                    .flat_map(|value| value.as_array().into_iter().flatten())
                    .filter_map(Value::as_u64)
                {
                    epoch_pending.insert(seq);
                }
            }
            EventKind::TurnOpen => {
                let event_turn = event.turn().ok_or("turn_open lacks turn")?;
                open_turn = Some(event_turn);
                for seq in object
                    .get("trigger")
                    .and_then(|trigger| trigger.get("inputs"))
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_u64)
                {
                    input_turn.insert(seq, event_turn);
                    input_position.insert(seq, event.seq());
                }
            }
            EventKind::Settle => {
                if open_turn == event.turn() {
                    open_turn = None;
                }
            }
            EventKind::Input if object.get("steer").and_then(Value::as_bool) == Some(true) => {
                if let Some(target) = open_turn {
                    input_turn.insert(event.seq(), target);
                }
            }
            EventKind::Attempt => {
                let id = event
                    .string_field("attempt")
                    .ok_or("attempt lacks id")?
                    .to_owned();
                attempt_epoch.insert(
                    id.clone(),
                    event
                        .string_field("epoch")
                        .ok_or("attempt lacks epoch")?
                        .to_owned(),
                );
                attempt_admits.insert(id, seqs_from_ranges(object.get("admits"))?);
            }
            EventKind::Output => {
                let attempt = event
                    .string_field("attempt")
                    .ok_or("output lacks attempt")?;
                settled_outputs.insert(attempt.to_owned());
                output_positions
                    .entry(attempt.to_owned())
                    .or_insert(event.seq());
                if attempt_epoch.get(attempt).is_some_and(|id| id == epoch_id) {
                    continuation_id = object
                        .get("continuation")
                        .and_then(|continuation| continuation.get("id"))
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                        .or(continuation_id);
                }
            }
            EventKind::Error if object.contains_key("sealed") => {
                let attempt = event
                    .string_field("attempt")
                    .ok_or("sealed error lacks attempt")?;
                partial_carriers.insert(attempt.to_owned());
                output_positions
                    .entry(attempt.to_owned())
                    .or_insert(event.seq());
            }
            EventKind::ToolCall => {
                let call = event.string_field("call").ok_or("tool_call lacks call")?;
                if let Some(attempt) = event.string_field("attempt") {
                    call_attempts.insert(call.to_owned(), attempt.to_owned());
                }
                call_turn.insert(call.to_owned(), event.turn().ok_or("tool_call lacks turn")?);
                call_name.insert(
                    call.to_owned(),
                    event
                        .string_field("name")
                        .ok_or("tool_call lacks name")?
                        .to_owned(),
                );
            }
            EventKind::Spawn => {
                spawned_calls.insert(
                    event
                        .string_field("call")
                        .ok_or("spawn lacks call")?
                        .to_owned(),
                );
            }
            _ => {}
        }
    }

    let admitted_all = attempt_admits
        .iter()
        .filter(|(attempt, _)| settled_outputs.contains(*attempt))
        .flat_map(|(_, seqs)| seqs.iter().copied())
        .collect::<std::collections::BTreeSet<_>>();
    let admitted = attempt_admits
        .iter()
        .filter(|(attempt, _)| {
            settled_outputs.contains(*attempt)
                && attempt_epoch.get(*attempt).is_some_and(|id| id == epoch_id)
        })
        .flat_map(|(_, seqs)| seqs.iter().copied())
        .collect::<std::collections::BTreeSet<_>>();
    let server_managed = dialect.server_managed();
    let incremental_continuation = server_managed && continuation_id.is_some() && !stateless;
    let epoch_seq = epoch_seq.ok_or("active epoch is missing")?;
    let mut items = Vec::new();
    if !incremental_continuation {
        items.extend(seed_provider_items(ledger)?);
    }
    for event in &projection.events {
        if incremental_continuation
            || *event.kind() != EventKind::Compact
            || superseded.contains(&event.seq())
            || compacted.contains(&event.seq())
        {
            continue;
        }
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("compact is not an object")?;
        let summary = materialize_string(
            ledger,
            object.get("summary").ok_or("compact lacks summary")?,
        )?;
        items.push(ijson(json!({
            "role":"user",
            "content":[{"type":"text","text":summary}]
        }))?);
    }
    // Freeze resurrected semantic anchors at the epoch boundary. Re-selecting
    // "latest" after each response could retract an older, already-admitted
    // result that a legacy compact marker covered.
    let boundary = projection
        .events
        .partition_point(|event| event.seq() <= epoch_seq);
    let semantic_anchors = compaction_anchor_sequences(&projection.events[..boundary])?;
    let mut admits = Vec::new();
    // Queued inputs are persisted while the preceding turn is still running.
    // Replay them at admission into their own turn, never between an earlier
    // assistant tool call and the corresponding result. Steers retain chronology.
    // An eagerly dispatched call's result is durable before its response's
    // output. The model must still see the assistant call (sealed in that
    // output) before the result: such results render right after the output.
    // A trimmed result renders where the result it retracts rendered, not at
    // its own seq: it answers the same assistant call, and a provider rejects
    // a call whose output has drifted away from it. `supersedes` points
    // strictly backward (event constraint 7), so the walk terminates.
    let mut replaced_result = std::collections::BTreeMap::<u64, u64>::new();
    for event in &projection.events {
        if *event.kind() != EventKind::ToolResult {
            continue;
        }
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("event is not an object")?;
        if let Some(previous) = seqs_from_ranges(object.get("supersedes"))?
            .into_iter()
            .max()
        {
            replaced_result.insert(event.seq(), previous);
        }
    }
    let render_anchor = |seq: u64| -> u64 {
        let mut anchor = seq;
        while let Some(previous) = replaced_result.get(&anchor) {
            anchor = *previous;
        }
        anchor
    };
    let deferred_result_position = |event: &Event| -> Option<u64> {
        if !matches!(event.kind(), EventKind::ToolResult | EventKind::ChildResult) {
            return None;
        }
        let anchor = render_anchor(event.seq());
        let output = output_positions.get(call_attempts.get(event.string_field("call")?)?)?;
        (*output > anchor).then_some(*output)
    };
    let mut ordered_events = projection.events.iter().collect::<Vec<_>>();
    ordered_events.sort_by_key(|event| {
        let anchor = render_anchor(event.seq());
        if let Some(position) = input_position.get(&anchor) {
            (*position, 0_u8, anchor)
        } else if let Some(position) = deferred_result_position(event) {
            (position, 1, anchor)
        } else {
            (anchor, 0, anchor)
        }
    });
    for event in ordered_events {
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("event is not an object")?;
        let anchored = semantic_anchors.contains(&event.seq());
        if superseded.contains(&event.seq()) || (compacted.contains(&event.seq()) && !anchored) {
            continue;
        }
        // The error stays runtime-visible; only its sealed partial carrier
        // is projected, as the assistant items the lost response produced.
        let partial_carrier = *event.kind() == EventKind::Error
            && object.contains_key("sealed")
            && event
                .string_field("attempt")
                .is_some_and(|attempt| partial_carriers.contains(attempt));
        if !partial_carrier
            && !matches!(
                event.effective_visibility(),
                Visibility::Model | Visibility::DialectDependent
            )
        {
            continue;
        }
        if event.kind() == &EventKind::ToolResult
            && event
                .string_field("call")
                .is_some_and(|call| spawned_calls.contains(call))
        {
            // Spawned calls retain their generic tool_result for the
            // tool_call/tool_result lifecycle fold, but child_result is the
            // sole provider-visible result for that call.
            continue;
        }
        let host_born = matches!(
            event.kind(),
            EventKind::Input | EventKind::ToolResult | EventKind::ChildResult | EventKind::State
        );
        let eligible = match event.kind() {
            EventKind::Input => input_turn.get(&event.seq()).is_some_and(|target| {
                if object.get("steer").and_then(Value::as_bool) == Some(true) {
                    *target == turn || admitted_all.contains(&event.seq())
                } else {
                    *target <= turn
                }
            }),
            EventKind::ToolResult | EventKind::ChildResult => object
                .get("call")
                .and_then(Value::as_str)
                .and_then(|call| call_turn.get(call))
                .is_some_and(|target| *target <= turn),
            EventKind::State => event.turn().is_none_or(|target| target <= turn),
            _ => true,
        };
        let admit_eligible = match event.kind() {
            EventKind::Input => input_turn
                .get(&event.seq())
                .is_some_and(|target| *target == turn),
            EventKind::ToolResult | EventKind::ChildResult => object
                .get("call")
                .and_then(Value::as_str)
                .and_then(|call| call_turn.get(call))
                .is_some_and(|target| *target == turn),
            EventKind::State => event.turn() == Some(turn),
            _ => false,
        };
        let eligible_at_epoch = event.seq() >= epoch_seq || epoch_pending.contains(&event.seq());
        if !eligible
            || (incremental_continuation
                && (!host_born || admitted.contains(&event.seq()) || !eligible_at_epoch))
        {
            continue;
        }
        if !incremental_continuation
            && matches!(event.kind(), EventKind::Reasoning | EventKind::ToolCall)
            && event.string_field("attempt").is_some_and(|attempt| {
                settled_outputs.contains(attempt) || partial_carriers.contains(attempt)
            })
        {
            continue;
        }
        let item =
            render_event_item(ledger, event, object, &call_name, dialect)?.ok_or_else(|| {
                format!(
                    "model-visible {} has no provider projection",
                    event.kind().as_str()
                )
            })?;
        if host_born && admit_eligible && !admitted.contains(&event.seq()) {
            admits.push(event.seq());
        }
        items.push(ijson(item)?);
    }
    admits.sort_unstable();
    admits.dedup();
    Ok(ProviderContext {
        items,
        admits,
        continuation_id: incremental_continuation
            .then_some(continuation_id)
            .flatten(),
    })
}

pub(crate) fn seed_provider_items(
    ledger: &LockedLedger,
) -> Result<Vec<IJsonValue>, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let Some(genesis) = projection.events.first() else {
        return Ok(Vec::new());
    };
    let raw = serde_json::to_value(genesis.raw())?;
    let Some(asset) = raw.pointer("/seed/snapshot/asset").and_then(Value::as_str) else {
        return Ok(Vec::new());
    };
    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
    let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(asset)?;
    let mut items = Vec::new();
    for line in bytes.split_inclusive(|byte| *byte == b'\n') {
        let canonical = line.strip_suffix(b"\n").ok_or("seed line lacks LF")?;
        if canonical.is_empty() {
            return Err("seed snapshot has an empty line".into());
        }
        let event = Event::decode_canonical(canonical)?;
        if !matches!(
            event.effective_visibility(),
            Visibility::Model | Visibility::DialectDependent
        ) {
            continue;
        }
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("seed event is not an object")?;
        let item = match event.kind() {
            EventKind::Input => json!({
                "role":"user",
                "content":render_blocks(ledger, object.get("content").ok_or("seed input lacks content")?)?
            }),
            EventKind::Output => json!({
                "role":"assistant",
                "content":render_blocks(ledger, object.get("content").ok_or("seed output lacks content")?)?
            }),
            EventKind::Reasoning => json!({
                "role":"assistant",
                "content":[{"type":"reasoning","text":materialize_string(ledger, object.get("content").ok_or("seed reasoning lacks content")?)?}]
            }),
            EventKind::ToolResult => {
                let call = event
                    .string_field("call")
                    .ok_or("seed tool_result lacks call")?;
                let outcome = object
                    .get("outcome")
                    .ok_or("seed tool_result lacks outcome")?;
                json!({
                    "role":"tool",
                    "content":[{
                        "type":"tool_result","call_id":provider_wire_call_id(ledger, call),"name":"tool",
                        "result":object.get("content").map(|value| materialize_json(ledger, value)).transpose()?.unwrap_or_else(|| json!({"outcome":outcome})),
                        "error":outcome != "ok"
                    }]
                })
            }
            EventKind::ChildResult => json!({
                "role":"tool",
                "content":[{
                    "type":"tool_result",
                    "call_id":event.string_field("call").ok_or("seed child_result lacks call")?,
                    "name":"subagent","result":object,
                    "error":object.get("outcome").and_then(Value::as_str) != Some("completed")
                }]
            }),
            EventKind::State if event.string_field("subkind") == Some("delegation") => {
                let payload = object
                    .get("payload")
                    .and_then(Value::as_object)
                    .ok_or("seed delegation state lacks payload")?;
                json!({
                    "role":"user",
                    "content":[{
                        "type":"text",
                        "text":"You are the worker assigned the following delegation. Execute its goal and instructions and produce the specified output. This is your assignment, not a request to call the delegation tool with the same arguments. Use the available tools to do the work; delegate only distinct subtasks when needed. When resolved_inputs binds an input source name, read its path (not the task name); for inline outputs use its summary. These bindings identify completed upstream tasks, not filesystem paths inferred from task names."
                    }, {
                        "type":"text",
                        "text":serde_json_canonicalizer::to_string(&json!({
                            "delegation":payload.get("name").and_then(Value::as_str).ok_or("seed delegation state lacks name")?,
                            "arguments":payload.get("invocation").ok_or("seed delegation state lacks invocation")?,
                            "resolved_inputs":payload.get("resolved_inputs").cloned().unwrap_or_else(|| json!([]))
                        }))?
                    }]
                })
            }
            EventKind::State => json!({
                "role":"user",
                "content":[{"type":"text","text":serde_json_canonicalizer::to_string(object)?}]
            }),
            other => {
                return Err(format!(
                    "model-visible seed kind {} has no provider projection",
                    other.as_str()
                )
                .into());
            }
        };
        items.push(ijson(item)?);
    }
    Ok(items)
}

fn provider_wire_call_id<'a>(ledger: &'a LockedLedger, call: &'a str) -> &'a str {
    ledger
        .projection()
        .and_then(|projection| {
            projection.events.iter().find(|event| {
                *event.kind() == EventKind::ToolCall && event.string_field("call") == Some(call)
            })
        })
        .and_then(|event| event.string_field("provider_call"))
        .unwrap_or(call)
}

/// The provider-native assistant items an `output` (or a transport-lost
/// attempt's `error`) sealed, replayed verbatim through the active adapter.
fn render_sealed_carrier(
    ledger: &LockedLedger,
    object: &Map<String, Value>,
    dialect: DialectId,
    kind: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let sealed = object
        .get("sealed")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("{kind} lacks sealed carrier"))?;
    if sealed.get("version").and_then(Value::as_u64) != Some(1)
        || sealed.get("adapter").and_then(Value::as_str) != Some(dialect.as_str())
    {
        return Err(
            format!("{kind} sealed carrier is incompatible with the active adapter").into(),
        );
    }
    let fragments = materialize_string(
        ledger,
        sealed
            .get("fragments")
            .ok_or("sealed carrier lacks fragments")?,
    )?;
    let fragments = serde_json::to_value(IJsonValue::parse(fragments.as_bytes())?)?;
    Ok(json!({"role":"sealed","adapter":dialect.as_str(),"fragments":fragments}))
}

pub(crate) fn render_event_item(
    ledger: &LockedLedger,
    event: &Event,
    object: &Map<String, Value>,
    call_names: &std::collections::BTreeMap<String, String>,
    dialect: DialectId,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    let item = match event.kind() {
        EventKind::Input => json!({
            "role":"user",
            "content":render_blocks(ledger, object.get("content").ok_or("input lacks content")?)?
        }),
        EventKind::Output => render_sealed_carrier(ledger, object, dialect, "output")?,
        // An error projects only through its sealed partial carrier.
        EventKind::Error if object.contains_key("sealed") => {
            render_sealed_carrier(ledger, object, dialect, "error")?
        }
        EventKind::Reasoning => json!({
            "role":"assistant",
            "content":[{"type":"reasoning","text":materialize_string(ledger, object.get("content").ok_or("reasoning lacks content")?)?}]
        }),
        EventKind::ToolCall => json!({
            "role":"assistant",
            "content":[{
                "type":"tool_call",
                "call_id":event.string_field("provider_call").or_else(|| event.string_field("call")).ok_or("tool_call lacks call")?,
                "name":event.string_field("name").ok_or("tool_call lacks name")?,
                "arguments":materialize_json(ledger, object.get("args").ok_or("tool_call lacks args")?)?
            }]
        }),
        EventKind::ToolResult => {
            let call = event.string_field("call").ok_or("tool_result lacks call")?;
            let outcome = object.get("outcome").ok_or("tool_result lacks outcome")?;
            let name = call_names.get(call).map(String::as_str).unwrap_or("tool");
            let result = if let Some(content) = object.get("content") {
                let mut blocks = render_blocks(ledger, content)?;
                // The ledger body stays canonical JSON; the model reads the
                // fixed tool's plain-text projection of it.
                if let [block] = blocks.as_mut_slice() {
                    if let Some(text) = block
                        .get("text")
                        .and_then(Value::as_str)
                        .filter(|_| block.get("type").and_then(Value::as_str) == Some("text"))
                        .and_then(|text| result_presentation::present(name, text, outcome == "ok"))
                    {
                        *block = json!({"type":"text","text":text});
                    }
                }
                Value::Array(blocks)
            } else {
                json!({"outcome":outcome})
            };
            json!({
                "role":"tool",
                "content":[{
                    "type":"tool_result","call_id":provider_wire_call_id(ledger, call),
                    "name":name,
                    "result":result,
                    "error":outcome != "ok"
                }]
            })
        }
        EventKind::ChildResult => {
            let call = event
                .string_field("call")
                .ok_or("child_result lacks call")?;
            json!({
                "role":"tool",
                "content":[{
                    "type":"tool_result","call_id":provider_wire_call_id(ledger, call),
                    "name":call_names.get(call).map(String::as_str).unwrap_or("subagent"),
                    "result":object,
                    "error":object.get("outcome").and_then(Value::as_str) != Some("completed")
                }]
            })
        }
        EventKind::State => json!({
            "role":"user",
            "content":[{"type":"text","text":serde_json_canonicalizer::to_string(object)?}]
        }),
        _ => return Ok(None),
    };
    Ok(Some(item))
}

pub(crate) fn render_blocks(
    ledger: &LockedLedger,
    value: &Value,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let value = materialize_json(ledger, value)?;
    let blocks = value.as_array().ok_or("content is not a block array")?;
    blocks
        .iter()
        .map(|block| {
            let object = block.as_object().ok_or("block is not an object")?;
            Ok(match object.get("type").and_then(Value::as_str) {
                Some("text") => json!({"type":"text","text":object.get("text").and_then(Value::as_str).ok_or("text block lacks text")?}),
                Some("reasoning") => json!({"type":"reasoning","text":object.get("text").and_then(Value::as_str).ok_or("reasoning block lacks text")?}),
                Some("image") => {
                    let asset = object.get("asset").and_then(Value::as_str).ok_or("image block lacks asset")?;
                    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
                    let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(asset)?;
                    json!({
                        "type":"file","name":asset,
                        "data":base64::engine::general_purpose::STANDARD.encode(bytes),
                        "mime":object.get("mime").and_then(Value::as_str).ok_or("image block lacks mime")?
                    })
                },
                Some("file") => {
                    let asset = object.get("asset").and_then(Value::as_str).ok_or("file block lacks asset")?;
                    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
                    let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(asset)?;
                    json!({
                        "type":"file",
                        "name":object.get("name").and_then(Value::as_str).ok_or("file block lacks name")?,
                        "mime":object.get("mime").and_then(Value::as_str).ok_or("file block lacks mime")?,
                        "bytes":bytes.len(),
                        "data":base64::engine::general_purpose::STANDARD.encode(bytes)
                    })
                },
                Some("tool-call") => json!({
                    "type":"tool_call","call_id":object.get("call").and_then(Value::as_str).ok_or("nested call lacks id")?,
                    "name":object.get("name").and_then(Value::as_str).ok_or("nested call lacks name")?,
                    "arguments":object.get("args").cloned().ok_or("nested call lacks args")?
                }),
                Some("tool-result") => json!({
                    "type":"tool_result","call_id":object.get("call").and_then(Value::as_str).ok_or("nested result lacks call")?,
                    "name":"tool","result":object.get("content").cloned().ok_or("nested result lacks content")?,
                    "error":object.get("error").and_then(Value::as_bool).unwrap_or(false)
                }),
                _ => return Err("unknown block kind".into()),
            })
        })
        .collect()
}

pub(crate) fn materialize_json(
    ledger: &LockedLedger,
    value: &Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    materialize_json_at(ledger.path(), value)
}

pub(crate) fn materialize_json_at(
    ledger_path: &std::path::Path,
    value: &Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let Some(spill) = value
        .as_object()
        .filter(|object| object.len() == 1)
        .and_then(|object| object.get("$spill"))
        .and_then(Value::as_object)
    else {
        return Ok(value.clone());
    };
    let asset = spill
        .get("asset")
        .and_then(Value::as_str)
        .ok_or("spill lacks asset")?;
    let folder = ledger_path.parent().ok_or("ledger has no folder")?;
    let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(asset)?;
    Ok(serde_json::to_value(IJsonValue::parse(&bytes)?)?)
}

pub(crate) fn materialize_string(
    ledger: &LockedLedger,
    value: &Value,
) -> Result<String, Box<dyn std::error::Error>> {
    let value = materialize_json(ledger, value)?;
    value
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| "spilled value is not a string".into())
}

fn seqs_from_ranges(value: Option<&Value>) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let mut seqs = Vec::new();
    for range in value.and_then(Value::as_array).into_iter().flatten() {
        let from = range
            .get("from")
            .and_then(Value::as_u64)
            .ok_or("range lacks from")?;
        let to = range
            .get("to")
            .and_then(Value::as_u64)
            .ok_or("range lacks to")?;
        seqs.extend(from..=to);
    }
    Ok(seqs)
}

pub(crate) fn effective_system(instruction: &InstructionSnapshot) -> String {
    instruction
        .effective
        .agents
        .iter()
        .filter_map(|index| instruction.sources.get(*index as usize))
        .filter(|source| source.kind == InstructionKind::Agents)
        .map(|source| source.content.as_str())
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub(crate) struct EpochAppend<'a> {
    pub(crate) options: &'a Options,
    pub(crate) dialect: DialectId,
    pub(crate) model: &'a str,
    pub(crate) profile: &'a IJsonValue,
    pub(crate) tools: &'a IJsonValue,
    pub(crate) reason: &'a str,
    pub(crate) pending: &'a [u64],
}

/// Why a new epoch opens when no compatible one exists (context §Epochs):
/// recovery and compaction first, then a renderer upgrade, then — with a
/// previous epoch to compare against — the part of the frozen head that
/// moved: the declared tool catalog, the system profile, or the target
/// model. Only the very first epoch of a file is `initial`.
pub(crate) fn epoch_open_reason(
    previous: Option<&Event>,
    compacted_after: bool,
    reset_epoch: bool,
    tools_digest: &str,
    profile_digest: &str,
) -> &'static str {
    if reset_epoch {
        return "recovery";
    }
    if compacted_after {
        return "compaction";
    }
    let Some(previous) = previous else {
        return "initial";
    };
    if previous.integer_field("renderer") != Some(CONTEXT_RENDERER_VERSION) {
        return "renderer_change";
    }
    let head = serde_json::to_value(previous.raw()).ok();
    let field = |pointer: &str| {
        head.as_ref().and_then(|raw| {
            raw.pointer(pointer)
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
    };
    if field("/tools/digest").as_deref() != Some(tools_digest) {
        return "tool_profile_change";
    }
    if field("/system/digest").as_deref() != Some(profile_digest) {
        return "system_change";
    }
    "model_change"
}

/// Replace one oversized `tool_result` with a bounded head and tail. The
/// replacement retracts the original by `supersedes`, so the call still pairs
/// with exactly one live result, the render skips the original, and the whole
/// output stays on disk for audit, fork and re-compaction.
pub(crate) fn append_tool_result_trim(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    trim: &engine::ToolResultTrim,
) -> Result<(), Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let original = projection
        .events
        .iter()
        .find(|event| event.seq() == trim.seq)
        .ok_or("trim target is not in the ledger")?;
    let raw = serde_json::to_value(original.raw())?;
    let object = raw.as_object().ok_or("tool_result is not an object")?;
    // The replacement is stamped with the turn that writes it, not the turn
    // of the record it retracts: every turn-bound event carries the latest
    // opened turn (event constraint 8). The pairing to the original is the
    // `call`, and eligibility still follows that call's own turn.
    let content = materialize_json(
        ledger,
        object.get("content").ok_or("tool_result lacks content")?,
    )?;
    let trimmed = engine::trimmed_tool_result_content(&content, trim.seq);
    let mut replacement = json!({
        "v":1, "seq": ledger.next_seq(), "turn": turn, "kind": "tool_result",
        "ts": options.event_timestamp(), "call": trim.call,
        "outcome": object.get("outcome").cloned().unwrap_or_else(|| json!("ok")),
        "content": trimmed,
        "supersedes": [{"from": trim.seq, "to": trim.seq}]
    });
    if let Some(meta) = object.get("meta") {
        replacement
            .as_object_mut()
            .expect("replacement object")
            .insert("meta".to_owned(), meta.clone());
    }
    ledger.append_contract(make_event(replacement)?, BarrierContext::default())?;
    Ok(())
}

pub(crate) fn append_provider_epoch(
    ledger: &mut LockedLedger,
    append: EpochAppend<'_>,
) -> Result<String, Box<dyn std::error::Error>> {
    let EpochAppend {
        options,
        dialect,
        model,
        profile,
        tools,
        reason,
        pending,
    } = append;
    let id = format!("{}-epoch-{}", options.run_id, ledger.next_seq());
    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
    let assets = store::AssetStore::new(folder.join("assets"))?;
    let system_bytes = profile.canonical_bytes()?;
    let system_asset = assets.publish(&system_bytes)?;
    // Content-addressed, so an unchanged catalog costs one blob across every
    // epoch of the thread.
    let tools_bytes = tools.canonical_bytes()?;
    let tools_asset = assets.publish(&tools_bytes)?;
    let event = make_event(json!({
        "v": 1,
        "seq": ledger.next_seq(),
        "kind": "epoch",
        "ts": options.event_timestamp(),
        "id": id,
        "reason": reason,
        "adapter": dialect.as_str(),
        "model": model,
        "system": {
            "asset": system_asset.asset,
            "digest": format!("{:x}", Sha256::digest(&system_bytes))
        },
        "tools": {
            "asset": tools_asset.asset,
            "digest": format!("{:x}", Sha256::digest(&tools_bytes))
        },
        "renderer": CONTEXT_RENDERER_VERSION,
        "pending": {"eligible": pending, "withheld": []}
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(id)
}

pub(crate) fn latest_compatible_epoch(
    ledger: &LockedLedger,
    dialect: DialectId,
    model: &str,
    system_digest: &str,
    tools_digest: &str,
    renderer: u64,
) -> Option<String> {
    // A compatible historical epoch is not a continuation of the current
    // chain. Reusing it after a catalog/profile change can pair later tool
    // results with an older response that never contained those calls.
    let event = ledger
        .projection()?
        .events
        .iter()
        .rev()
        .find(|event| *event.kind() == EventKind::Epoch)?;
    if ledger
        .projection()?
        .events
        .iter()
        .any(|later| later.seq() > event.seq() && *later.kind() == EventKind::Compact)
    {
        return None;
    }
    let raw = serde_json::to_value(event.raw()).ok()?;
    (event.string_field("adapter") == Some(dialect.as_str())
        && event.string_field("model") == Some(model)
        && raw.pointer("/system/digest").and_then(Value::as_str) == Some(system_digest)
        && raw.pointer("/tools/digest").and_then(Value::as_str) == Some(tools_digest)
        && event.integer_field("renderer") == Some(renderer))
    .then(|| event.string_field("id").map(str::to_owned))
    .flatten()
}

pub(crate) fn ranges(seqs: &[u64]) -> Vec<Value> {
    let mut ranges = Vec::new();
    for seq in seqs {
        match ranges.last_mut().and_then(Value::as_object_mut) {
            Some(range)
                if range.get("to").and_then(Value::as_u64) == Some(seq.saturating_sub(1)) =>
            {
                range.insert("to".to_owned(), Value::from(*seq));
            }
            _ => ranges.push(json!({"from": seq, "to": seq})),
        }
    }
    ranges
}
