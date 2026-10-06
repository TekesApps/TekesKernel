use super::*;

pub(crate) fn append_run_start(
    ledger: &mut LockedLedger,
    options: &Options,
    mode: RunMode,
    ordinal: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mode = match mode {
        RunMode::Ordinary => "ordinary",
        RunMode::Reconcile => "reconcile",
    };
    let mut value = json!({
        "v": 1,
        "seq": ledger.next_seq(),
        "kind": "run_start",
        "ts": options.event_timestamp(),
        "run": options.run_id,
        "mode": mode,
        "recovery_ordinal": ordinal,
        "binary": options.binary,
        "config_digest": options.config_digest,
        "instruction_digest": options.instruction_digest,
        "policy": options.policy
    });
    if let Some(digest) = &options.launch_bindings_digest {
        value
            .as_object_mut()
            .expect("run_start is an object")
            .insert(
                "launch_bindings_digest".to_owned(),
                Value::String(digest.clone()),
            );
    }
    let event = make_event(value)?;
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(())
}

/// The production queue gate: a provider final or validation decision alone
/// never consumes queued input; only a settled root turn releases it.
pub(crate) fn open_ready_turn(
    ledger: &mut LockedLedger,
    timestamp: &str,
    mode: RunMode,
) -> Result<Option<u64>, Box<dyn std::error::Error>> {
    let facts = ledger
        .projection()
        .ok_or("missing queue projection")?
        .lifecycle
        .clone();
    if matches!(classify(&facts, LockFacts::CALLER), TailState::Unstarted) {
        append_turn_open(ledger, timestamp, "genesis", &[], 1)?;
        return Ok(Some(1));
    }
    if matches!(mode, RunMode::Ordinary)
        && (facts.latest_turn.is_none() || facts.terminal_tail)
        && !facts.turn_open_inputs.is_empty()
    {
        let turn = facts.latest_turn.map_or(1, |value| value + 1);
        append_turn_open(ledger, timestamp, "inputs", &facts.turn_open_inputs, turn)?;
        return Ok(Some(turn));
    }
    Ok(None)
}

/// A completed turn is not a completed goal. Admit the next turn only from
/// the separately persisted goal state, after all queued user input is drained.
pub(crate) fn open_goal_continuation(
    ledger: &mut LockedLedger,
    options: &Options,
    profile: &RuntimeProfile,
    cancellation: &RuntimeCancellation,
) -> Result<bool, Box<dyn std::error::Error>> {
    let Some(bound_id) = profile.bindings.goal_id.as_deref() else {
        return Ok(false);
    };
    if cancellation.stop_requested() {
        return Ok(false);
    }
    let projection = ledger
        .projection()
        .ok_or("goal continuation has no projection")?;
    let facts = &projection.lifecycle;
    if !facts.terminal_tail || facts.stop_active || !facts.turn_open_inputs.is_empty() {
        return Ok(false);
    }
    let Some(turn) = facts.latest_turn else {
        return Ok(false);
    };
    let completed = projection
        .events
        .iter()
        .rev()
        .find(|event| event.kind() == &EventKind::Settle)
        .is_some_and(|event| {
            event.turn() == Some(turn) && event.string_field("outcome") == Some("completed")
        });
    if !completed {
        return Ok(false);
    }
    let Some((root, session)) = goal_store_location(ledger)? else {
        return Ok(false);
    };
    let Some(goal) = session_controls::read_goal(&root, &session)? else {
        return Ok(false);
    };
    if goal.id != bound_id || goal.phase != session_controls::PHASE_ACTIVE {
        return Ok(false);
    }
    if goal.rounds_started >= goal.max_goal_rounds {
        return Ok(false);
    }
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn + 1,"kind":"turn_open",
        "ts":options.event_timestamp(),"trigger":{"goal":bound_id}
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(true)
}

pub(crate) fn goal_store_location(
    ledger: &LockedLedger,
) -> Result<Option<(PathBuf, String)>, Box<dyn std::error::Error>> {
    let folder = ledger.path().parent().ok_or("goal thread folder missing")?;
    let Some(session) = folder.file_name().and_then(|name| name.to_str()) else {
        return Ok(None);
    };
    let root = folder
        .parent()
        .and_then(|threads| threads.parent())
        .ok_or("goal storage root missing")?;
    Ok(Some((root.to_path_buf(), session.to_owned())))
}

pub(crate) fn append_turn_open(
    ledger: &mut LockedLedger,
    timestamp: &str,
    trigger: &str,
    inputs: &[u64],
    turn: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let trigger = if trigger == "genesis" {
        Value::String("genesis".to_owned())
    } else {
        json!({"inputs": inputs})
    };
    let event = make_event(json!({
        "v": 1,
        "seq": ledger.next_seq(),
        "turn": turn,
        "kind": "turn_open",
        "ts": timestamp,
        "trigger": trigger
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(())
}

pub(crate) fn origin_event(
    ledger: &LockedLedger,
    timestamp: &str,
    kind: &str,
    origin: &schema::OriginTuple,
) -> Map<String, Value> {
    json!({
        "v": 1,
        "seq": ledger.next_seq(),
        "kind": kind,
        "ts": timestamp,
        "origin_key": origin.key,
        "origin_tuple": origin
    })
    .as_object()
    .expect("object literal")
    .clone()
}

pub(crate) fn append_and_receipt(
    ledger: &mut LockedLedger,
    stdout: &mut impl Write,
    delivery: String,
    origin: &schema::OriginTuple,
    object: Map<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(projection) = ledger.projection() {
        if let Some(existing_seq) = projection.origin_tuples.get(origin) {
            stdout.write_all(&encode_line(
                "receipt",
                &Receipt {
                    delivery,
                    seq: *existing_seq,
                    deduplicated: true,
                },
            )?)?;
            stdout.flush()?;
            return Ok(());
        }
    }
    let event = make_event(Value::Object(object))?;
    let seq = event.seq();
    ledger.append_contract(event, BarrierContext::default())?;
    stdout.write_all(&encode_line(
        "receipt",
        &Receipt {
            delivery,
            seq,
            deduplicated: false,
        },
    )?)?;
    stdout.flush()?;
    announce_appended(ledger, stdout)?;
    Ok(())
}

pub(crate) fn announce_appended(
    ledger: &LockedLedger,
    stdout: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(seq) = ledger.projection().map(|projection| projection.last_seq) else {
        return Ok(());
    };
    stdout.write_all(&encode_line("appended", &Appended { seq })?)?;
    stdout.flush()?;
    Ok(())
}

pub(crate) fn event_at(ledger: &LockedLedger, seq: u64) -> Result<&Event, StoreError> {
    ledger
        .projection()
        .and_then(|projection| projection.events.iter().find(|event| event.seq() == seq))
        .ok_or_else(|| StoreError::Corruption(format!("origin tuple names missing seq {seq}")))
}

pub(crate) fn event_json(event: &Event) -> Result<Value, Box<dyn std::error::Error>> {
    Ok(serde_json::from_slice(&event.canonical_bytes()?)?)
}

pub(crate) fn make_event(value: Value) -> Result<Event, Box<dyn std::error::Error>> {
    Ok(Event::decode(&serde_json::to_vec(&value)?)?)
}

pub(crate) fn ijson(value: Value) -> Result<IJsonValue, Box<dyn std::error::Error>> {
    Ok(IJsonValue::parse(&serde_json::to_vec(&value)?)?)
}
