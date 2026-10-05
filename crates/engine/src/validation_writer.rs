//! Writer-owned validation records. Each decision contains both its negative
//! round and routing/settlement disposition in one synced JSONL record. The
//! worker must recover unmaterialized decisions before requesting another model
//! response. No model-returned object is accepted as a control intent here.

use schema::{Event, EventKind, IJsonValue};
use serde_json::{Value, json};
use store::{LockedLedger, StoreError};

use crate::{
    ValidationBinding, ValidationDecision, ValidationSignal, ValidationVerdict, validation_decision,
};

fn invalid(message: &str) -> StoreError {
    StoreError::Corruption(message.to_owned())
}

fn raw(event: &Event) -> Result<Value, StoreError> {
    serde_json::to_value(event.raw()).map_err(|error| invalid(&error.to_string()))
}

/// Bind a final to an immutable host snapshot. The exact candidate is durable
/// before any judge may be started. A restart recovers this snapshot, never a
/// replacement computed from whatever happens to be the latest output.
pub fn begin_validation(
    ledger: &mut LockedLedger,
    timestamp: &str,
    binding: &ValidationBinding,
) -> Result<u64, StoreError> {
    let projection = ledger
        .projection()
        .ok_or_else(|| invalid("missing validation ledger"))?;
    let thread = projection
        .events
        .first()
        .and_then(|event| event.string_field("thread"));
    if thread != Some(binding.thread.as_str())
        || binding.worker != binding.thread
        || projection.latest_turn != Some(binding.turn)
    {
        return Err(invalid(
            "validation candidate has foreign turn or worker binding",
        ));
    }
    let output = projection
        .events
        .iter()
        .find(|event| event.seq() == binding.output_seq)
        .ok_or_else(|| invalid("validation candidate output is missing"))?;
    if output.kind() != &EventKind::Output
        || output.turn() != Some(binding.turn)
        || raw(output)?.get("final_answer") != Some(&Value::Bool(true))
    {
        return Err(invalid(
            "validation requires a final output in the exact turn",
        ));
    }
    for event in &projection.events {
        if event.kind() == &EventKind::State
            && event.string_field("subkind") == Some("validation.candidate")
        {
            let value = raw(event)?;
            let stored: ValidationBinding =
                serde_json::from_value(value["payload"]["binding"].clone())
                    .map_err(|error| invalid(&error.to_string()))?;
            if stored.output_seq == binding.output_seq {
                if &stored != binding {
                    return Err(invalid("validation candidate snapshot is immutable"));
                }
                return Ok(event.seq());
            }
        }
    }
    if projection.terminal_tail {
        return Err(invalid("cannot validate a settled turn"));
    }
    let seq = ledger.next_seq();
    append_state(
        ledger,
        timestamp,
        binding.turn,
        "validation.candidate",
        json!({"binding":binding}),
    )?;
    Ok(seq)
}

/// Commit the decision and round atomically as a single record. Callers admit
/// their source evidence before entering this writer API. References must still
/// exist in the exact turn while the ledger lock is held. Replays return the
/// original immutable decision record, including after subsequent projection.
pub fn commit_validation_decision(
    ledger: &mut LockedLedger,
    timestamp: &str,
    candidate_seq: u64,
    source_seq: u64,
) -> Result<u64, StoreError> {
    let projection = ledger
        .projection()
        .ok_or_else(|| invalid("missing validation ledger"))?;
    let candidate = projection
        .events
        .iter()
        .find(|event| event.seq() == candidate_seq)
        .ok_or_else(|| invalid("validation candidate is missing"))?;
    if candidate.kind() != &EventKind::State
        || candidate.string_field("subkind") != Some("validation.candidate")
    {
        return Err(invalid(
            "validation decision target is not a host candidate",
        ));
    }
    let turn = candidate
        .turn()
        .ok_or_else(|| invalid("validation candidate has no turn"))?;
    let source = projection
        .events
        .iter()
        .find(|event| event.seq() == source_seq)
        .ok_or_else(|| invalid("validation signal is missing"))?;
    if source.turn() != Some(turn) || projection.latest_turn != Some(turn) {
        return Err(invalid("validation signal belongs to a different turn"));
    }
    if source_seq != candidate_seq
        && (source.kind() != &EventKind::State
            || !matches!(
                source.string_field("subkind"),
                Some("validation.verdict" | "validation.death")
            )
            || raw(source)?["payload"]["candidate_seq"].as_u64() != Some(candidate_seq))
    {
        return Err(invalid("validation signal is not bound host evidence"));
    }
    let mut negative_rounds = 0;
    let mut terminal_decision_exists = false;
    for event in &projection.events {
        if event.kind() != &EventKind::State
            || event.string_field("subkind") != Some("validation.decision")
        {
            continue;
        }
        let value = raw(event)?;
        if value["payload"]["source_seq"].as_u64() == Some(source_seq) {
            if value["payload"]["candidate_seq"].as_u64() != Some(candidate_seq) {
                return Err(invalid(
                    "validation signal already has a different decision",
                ));
            }
            return Ok(event.seq());
        }
        if event.turn() == Some(turn) {
            let previous: ValidationDecision =
                serde_json::from_value(value["payload"]["decision"].clone())
                    .map_err(|error| invalid(&error.to_string()))?;
            terminal_decision_exists |= matches!(previous, ValidationDecision::Settle { .. });
            match previous {
                ValidationDecision::Feedback { ordinal, .. }
                | ValidationDecision::Settle {
                    negative_round: Some((ordinal, _)),
                    ..
                } => {
                    negative_rounds = negative_rounds.max(ordinal);
                }
                _ => {}
            }
        }
    }
    if projection.terminal_tail || terminal_decision_exists {
        return Err(invalid("cannot add a validation decision after settlement"));
    }
    let signal = if source_seq == candidate_seq {
        let binding: ValidationBinding =
            serde_json::from_value(raw(candidate)?["payload"]["binding"].clone())
                .map_err(|error| invalid(&error.to_string()))?;
        let mut covering_verdict = None;
        for event in projection.events.iter().rev() {
            if event.turn() != Some(turn)
                || event.kind() != &EventKind::State
                || event.string_field("subkind") != Some("validation.verdict")
            {
                continue;
            }
            let value = raw(event)?;
            let prior_seq = value["payload"]["candidate_seq"]
                .as_u64()
                .ok_or_else(|| invalid("verdict has no candidate binding"))?;
            let prior = projection
                .events
                .iter()
                .find(|event| event.seq() == prior_seq)
                .ok_or_else(|| invalid("verdict candidate is missing"))?;
            let prior_binding: ValidationBinding =
                serde_json::from_value(raw(prior)?["payload"]["binding"].clone())
                    .map_err(|error| invalid(&error.to_string()))?;
            if binding.covered_by(&prior_binding) {
                covering_verdict = Some(
                    serde_json::from_value::<ValidationVerdict>(
                        value["payload"]["verdict"].clone(),
                    )
                    .map_err(|error| invalid(&error.to_string()))?,
                );
                break;
            }
        }
        ValidationSignal::Candidate {
            has_artifacts: !binding.snapshot.is_empty(),
            covering_verdict,
        }
    } else if source.string_field("subkind") == Some("validation.death") {
        ValidationSignal::ValidatorDeath
    } else {
        ValidationSignal::Verdict(
            serde_json::from_value(raw(source)?["payload"]["verdict"].clone())
                .map_err(|error| invalid(&error.to_string()))?,
        )
    };
    let decision = validation_decision(false, false, negative_rounds, signal);
    let seq = ledger.next_seq();
    append_state(
        ledger,
        timestamp,
        turn,
        "validation.decision",
        json!({
            "candidate_seq":candidate_seq,"source_seq":source_seq,"decision":decision,
        }),
    )?;
    Ok(seq)
}

fn append_state(
    ledger: &mut LockedLedger,
    timestamp: &str,
    turn: u64,
    subkind: &str,
    payload: Value,
) -> Result<(), StoreError> {
    let value = json!({"v":1,"seq":ledger.next_seq(),"kind":"state","turn":turn,
        "ts":timestamp,"visibility":"runtime","subkind":subkind,"payload":payload});
    let bytes = serde_json::to_vec(&value).map_err(|error| invalid(&error.to_string()))?;
    let event = Event::from_value(IJsonValue::parse(&bytes)?)?;
    ledger.append(event, true)
}

/// Recoverable projection of a committed terminal decision. The decision owns
/// the outcome and promoted output; neither is recomputed from the latest tail.
/// A crash before this append leaves the decision available for the next writer.
pub fn materialize_validation_settlement(
    ledger: &mut LockedLedger,
    timestamp: &str,
    decision_seq: u64,
) -> Result<u64, StoreError> {
    let projection = ledger
        .projection()
        .ok_or_else(|| invalid("missing validation ledger"))?;
    let event = projection
        .events
        .iter()
        .find(|event| event.seq() == decision_seq)
        .ok_or_else(|| invalid("validation decision is missing"))?;
    if event.kind() != &EventKind::State
        || event.string_field("subkind") != Some("validation.decision")
    {
        return Err(invalid("settlement source is not a validation decision"));
    }
    let turn = event
        .turn()
        .ok_or_else(|| invalid("decision has no turn"))?;
    let value = raw(event)?;
    let decision: ValidationDecision = serde_json::from_value(value["payload"]["decision"].clone())
        .map_err(|error| invalid(&error.to_string()))?;
    let ValidationDecision::Settle { outcome, .. } = decision else {
        return Err(invalid("nonterminal validation decision cannot settle"));
    };
    let candidate_seq = value["payload"]["candidate_seq"]
        .as_u64()
        .ok_or_else(|| invalid("decision has no candidate reference"))?;
    let candidate = projection
        .events
        .iter()
        .find(|event| event.seq() == candidate_seq)
        .ok_or_else(|| invalid("decision candidate is missing"))?;
    if candidate.kind() != &EventKind::State
        || candidate.string_field("subkind") != Some("validation.candidate")
        || candidate.string_field("visibility") != Some("runtime")
        || candidate.turn() != Some(turn)
        || event.string_field("visibility") != Some("runtime")
    {
        return Err(invalid(
            "settlement requires host-owned candidate and decision",
        ));
    }
    let binding: ValidationBinding =
        serde_json::from_value(raw(candidate)?["payload"]["binding"].clone())
            .map_err(|error| invalid(&error.to_string()))?;
    let thread = projection
        .events
        .first()
        .and_then(|e| e.string_field("thread"));
    let output = projection
        .events
        .iter()
        .find(|e| e.seq() == binding.output_seq)
        .ok_or_else(|| invalid("promoted output is missing"))?;
    if thread != Some(binding.thread.as_str())
        || binding.worker != binding.thread
        || binding.turn != turn
        || output.turn() != Some(turn)
        || output.kind() != &EventKind::Output
        || raw(output)?["final_answer"] != true
        || !(binding.output_seq < candidate_seq && candidate_seq < decision_seq)
    {
        return Err(invalid(
            "settlement output is not the exact host-bound final",
        ));
    }
    for existing in &projection.events {
        if existing.turn() == Some(turn) && existing.kind() == &EventKind::Settle {
            if raw(existing)?["validation"]["decision_seq"].as_u64() == Some(decision_seq) {
                return Ok(existing.seq());
            }
            return Err(invalid("turn already settled by another authority"));
        }
        if existing.turn() == Some(turn)
            && existing.seq() < decision_seq
            && existing.string_field("subkind") == Some("validation.decision")
        {
            let previous: ValidationDecision =
                serde_json::from_value(raw(existing)?["payload"]["decision"].clone())
                    .map_err(|error| invalid(&error.to_string()))?;
            if matches!(previous, ValidationDecision::Settle { .. }) {
                return Err(invalid(
                    "earlier terminal validation decision owns settlement",
                ));
            }
        }
    }
    if projection.latest_turn != Some(turn) || binding.turn != turn {
        return Err(invalid("validation settlement belongs to a different turn"));
    }
    let seq = ledger.next_seq();
    let value = json!({"v":1,"seq":seq,"kind":"settle","turn":turn,
    "ts":timestamp,"outcome":"completed","validation":{
        "outcome":outcome,"candidate_seq":candidate_seq,"decision_seq":decision_seq,
        "promoted_output_seq":binding.output_seq
    }});
    let bytes = serde_json::to_vec(&value).map_err(|error| invalid(&error.to_string()))?;
    ledger.append(Event::from_value(IJsonValue::parse(&bytes)?)?, true)?;
    Ok(seq)
}
