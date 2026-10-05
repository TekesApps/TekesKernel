use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use schema::{Event, EventKind, IJsonValue};
use serde_json::{Map, Value, json};
use thiserror::Error;

use crate::session_event_registry_generated::kernel_may_emit_event_type;
use crate::{EndpointJournal, JournalError, SessionEvent, SurfaceOperation};
use store::AssetStore;

#[derive(Clone, Debug)]
struct AttemptInfo {
    turn: u64,
    step: u64,
    epoch: String,
    kernel_seq: u64,
}

#[derive(Clone, Debug)]
struct EpochInfo {
    adapter: String,
    model: String,
}

#[derive(Default)]
pub struct Projector {
    inputs: BTreeMap<u64, Value>,
    attempts: HashMap<String, AttemptInfo>,
    epochs: HashMap<String, EpochInfo>,
    outputs: HashMap<u64, (u64, String)>,
    open_steps: HashMap<u64, AttemptInfo>,
    /// A tool-result trim is appended in the current turn but replaces a result from an
    /// earlier, already closed step. Keep the original presentation identity by ledger seq.
    tool_result_steps: BTreeMap<u64, (u64, u64)>,
    seen_steers: HashSet<u64>,
    terminal_attempts: HashSet<String>,
    /// Durable reasoning text per open attempt, folded into that attempt's
    /// `assistant/message` as leading `reasoning` blocks (spec: the message
    /// preserves reasoning; no separate thinking event kind is invented).
    reasoning: HashMap<String, Vec<String>>,
    expected_kernel_slots: HashSet<(u64, String)>,
    processed_through: Option<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AcceptedStreamFrame {
    pub arguments_complete: Option<bool>,
    pub attempt: String,
    pub frame: u64,
    pub channel: String,
    pub block: u64,
    pub delta: String,
    pub call_id: Option<String>,
    pub name: Option<String>,
    pub time: f64,
}

impl Projector {
    /// Highest Kernel seq this projector has reconciled, or `None` before the
    /// first reconcile. Comparing it with the ledger's `last_seq` answers
    /// "is the endpoint journal behind the semantic ledger" without a scan.
    #[must_use]
    pub const fn processed_through(&self) -> Option<u64> {
        self.processed_through
    }

    pub fn reconcile(
        &mut self,
        events: &[Event],
        journal: &EndpointJournal,
    ) -> Result<Vec<SessionEvent>, ProjectionError> {
        let mut appended = Vec::new();
        for event in events {
            let mut projected = self.project(event, journal)?;
            for (kernel_seqs, slot, wire) in &mut projected {
                apply_surface_supersedes(event, journal, wire)?;
                let anchor = *kernel_seqs
                    .last()
                    .ok_or(ProjectionError::InvalidField("kernel_seqs"))?;
                self.expected_kernel_slots.insert((anchor, slot.clone()));
            }
            appended.extend(journal.append_kernel_batch(projected)?);
            if matches!(event.kind(), EventKind::Output)
                || matches!(event.kind(), EventKind::Error) && event.has_field("attempt")
            {
                let attempt = event
                    .string_field("attempt")
                    .ok_or(ProjectionError::MissingField("attempt"))?;
                self.terminal_attempts.insert(attempt.to_owned());
            }
            self.processed_through = Some(
                self.processed_through
                    .map_or(event.seq(), |seq| seq.max(event.seq())),
            );
        }
        self.validate_existing_kernel_slots(journal)?;
        Ok(appended)
    }

    /// Projects an adapter frame for the live presentation lane. Chunks are
    /// transient: they are never journaled, and the frame ordinal is only a
    /// per-attempt presentation version.
    pub fn stream_event(
        &self,
        frame: &AcceptedStreamFrame,
    ) -> Result<SessionEvent, ProjectionError> {
        if self.terminal_attempts.contains(&frame.attempt) {
            return Err(ProjectionError::TerminalAttempt(frame.attempt.clone()));
        }
        let info = self
            .attempts
            .get(&frame.attempt)
            .ok_or_else(|| ProjectionError::UnknownAttempt(frame.attempt.clone()))?;
        let chunk_type = match frame.channel.as_str() {
            "text" => "text-delta",
            "reasoning" => "reasoning-delta",
            "tool" => "tool-call-delta",
            "usage" => "usage",
            other => return Err(ProjectionError::StreamChannel(other.to_owned())),
        };
        let data = if frame.channel == "usage" {
            if frame.call_id.is_some() || frame.name.is_some() || frame.arguments_complete.is_some()
            {
                return Err(ProjectionError::InvalidField("call_id/name"));
            }
            let report: Value = serde_json::from_str(&frame.delta)
                .map_err(|_| ProjectionError::InvalidField("usage"))?;
            let usage = wire_usage(&report)?;
            json!({"turn": info.turn, "step": info.step, "chunk": {"type": "usage", "usage": usage}})
        } else if frame.channel == "tool" {
            let call_id = frame
                .call_id
                .as_deref()
                .ok_or(ProjectionError::MissingField("call_id"))?;
            let mut chunk = Map::new();
            chunk.insert("type".to_owned(), json!(chunk_type));
            chunk.insert("index".to_owned(), json!(frame.block));
            chunk.insert("id".to_owned(), json!(call_id));
            chunk.insert("argumentsDelta".to_owned(), json!(frame.delta));
            chunk.insert("assistantFrameId".to_owned(), json!(frame.attempt));
            if let Some(complete) = frame.arguments_complete {
                chunk.insert("argumentsComplete".to_owned(), json!(complete));
            }
            if let Some(name) = &frame.name {
                chunk.insert("name".to_owned(), json!(name));
            }
            json!({"turn": info.turn, "step": info.step, "chunk": chunk})
        } else {
            if frame.call_id.is_some() || frame.name.is_some() || frame.arguments_complete.is_some()
            {
                return Err(ProjectionError::InvalidField("call_id/name"));
            }
            json!({
                "turn": info.turn,
                "step": info.step,
                "chunk": {"type": chunk_type, "index": frame.block, "text": frame.delta}
            })
        };
        let mut event = session_event("assistant/chunk", frame.time, data, false)?;
        event.seq = frame.frame;
        Ok(event)
    }

    fn project(
        &mut self,
        event: &Event,
        journal: &EndpointJournal,
    ) -> Result<Vec<Projected>, ProjectionError> {
        let value = event_value(event)?;
        let time = timestamp_millis(
            value
                .get("ts")
                .and_then(Value::as_str)
                .ok_or(ProjectionError::MissingField("ts"))?,
        )?;
        let mut output = Vec::new();
        match event.kind() {
            EventKind::Input => {
                self.inputs.insert(event.seq(), value);
            }
            EventKind::Epoch => {
                let id = text(&value, "id")?.to_owned();
                self.epochs.insert(
                    id,
                    EpochInfo {
                        adapter: text(&value, "adapter")?.to_owned(),
                        model: text(&value, "model")?.to_owned(),
                    },
                );
            }
            EventKind::TurnOpen => {
                let turn = turn(event)?;
                output.push((
                    vec![event.seq()],
                    "turn-start".to_owned(),
                    session_event("turn/start", time, json!({"turn": turn}), false)?,
                ));
                let input_seqs = value
                    .get("trigger")
                    .and_then(Value::as_object)
                    .and_then(|trigger| trigger.get("inputs"))
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                for (index, input_seq) in input_seqs.into_iter().enumerate() {
                    let input_seq = input_seq
                        .as_u64()
                        .ok_or(ProjectionError::InvalidField("trigger.inputs"))?;
                    let input = self
                        .inputs
                        .get(&input_seq)
                        .ok_or(ProjectionError::UnknownInput(input_seq))?;
                    output.push((
                        vec![input_seq, event.seq()],
                        format!("user-message-{index}"),
                        user_message(input, input_seq, turn, time)?,
                    ));
                }
            }
            EventKind::Attempt => {
                let turn = turn(event)?;
                let attempt = text(&value, "attempt")?.to_owned();
                if let Some(previous) = self.open_steps.remove(&turn) {
                    output.push((
                        vec![previous.kernel_seq, event.seq()],
                        format!("step-end-before-{attempt}"),
                        session_event(
                            "step/end",
                            time,
                            json!({"turn": turn, "step": previous.step}),
                            false,
                        )?,
                    ));
                }
                let step = self
                    .attempts
                    .values()
                    .filter(|info| info.turn == turn)
                    .count() as u64;
                let info = AttemptInfo {
                    turn,
                    step,
                    epoch: text(&value, "epoch")?.to_owned(),
                    kernel_seq: event.seq(),
                };
                self.attempts.insert(attempt.clone(), info.clone());
                self.open_steps.insert(turn, info);
                output.push((
                    vec![event.seq()],
                    "step-start".to_owned(),
                    session_event(
                        "step/start",
                        time,
                        json!({"turn": turn, "step": step}),
                        false,
                    )?,
                ));

                for range in value
                    .get("admits")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                {
                    let from = range
                        .get("from")
                        .and_then(Value::as_u64)
                        .ok_or(ProjectionError::InvalidField("admits.from"))?;
                    let to = range
                        .get("to")
                        .and_then(Value::as_u64)
                        .ok_or(ProjectionError::InvalidField("admits.to"))?;
                    for input_seq in from..=to {
                        let Some(input) = self.inputs.get(&input_seq) else {
                            continue;
                        };
                        if input.get("steer").and_then(Value::as_bool) == Some(true)
                            && self.seen_steers.insert(input_seq)
                        {
                            output.push((
                                vec![input_seq, event.seq()],
                                format!("steer-message-{input_seq}"),
                                user_message(input, input_seq, turn, time)?,
                            ));
                        }
                    }
                }
            }
            EventKind::Reasoning => {
                let attempt = text(&value, "attempt")?;
                let content = resolve_spill(
                    value
                        .get("content")
                        .ok_or(ProjectionError::MissingField("content"))?,
                    journal,
                )?;
                if let Some(text) = content.as_str().filter(|text| !text.is_empty()) {
                    self.reasoning
                        .entry(attempt.to_owned())
                        .or_default()
                        .push(text.to_owned());
                }
            }
            EventKind::Output => {
                let attempt = text(&value, "attempt")?;
                let info = self.attempt(attempt)?.clone();
                let epoch = self
                    .epochs
                    .get(&info.epoch)
                    .ok_or_else(|| ProjectionError::UnknownEpoch(info.epoch.clone()))?;
                let mut data = Map::new();
                data.insert("turn".to_owned(), json!(info.turn));
                data.insert("step".to_owned(), json!(info.step));
                if let Some(final_answer) = value.get("final_answer") {
                    data.insert("sessionFinal".to_owned(), final_answer.clone());
                }
                if value.get("final_answer").and_then(Value::as_bool) == Some(true) {
                    self.outputs
                        .insert(event.seq(), (info.turn, format!("assistant-{attempt}")));
                }
                let mut source = Map::new();
                source.insert("kind".to_owned(), json!("model"));
                source.insert("provider".to_owned(), json!(epoch.adapter));
                source.insert("model".to_owned(), json!(epoch.model));
                if let Some(continuation) = value.get("continuation") {
                    source.insert("replayState".to_owned(), continuation.clone());
                }
                let mut content = self
                    .reasoning
                    .remove(attempt)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|text| json!({"type": "reasoning", "text": text}))
                    .collect::<Vec<_>>();
                if let Some(blocks) = value.get("content").and_then(Value::as_array) {
                    content.extend(blocks.iter().cloned());
                }
                data.insert(
                    "message".to_owned(),
                    json!({
                        "id": format!("assistant-{attempt}"),
                        "role": "assistant",
                        "content": content,
                        "source": source
                    }),
                );
                if let Some(usage) = value.get("usage") {
                    data.insert("usage".to_owned(), wire_usage(usage)?);
                }
                output.push((
                    vec![event.seq()],
                    "assistant-message".to_owned(),
                    session_event("assistant/message", time, Value::Object(data), true)?,
                ));
            }
            EventKind::ToolCall => {
                let attempt = text(&value, "attempt")?;
                let info = self.attempt(attempt)?;
                let arguments = resolve_spill(
                    value
                        .get("args")
                        .ok_or(ProjectionError::MissingField("args"))?,
                    journal,
                )?;
                output.push((
                    vec![event.seq()],
                    "tool-call".to_owned(),
                    session_event(
                        "tool/call",
                        time,
                        json!({
                            "turn": info.turn,
                            "step": info.step,
                            "callId": text(&value, "call")?,
                            "name": text(&value, "name")?,
                            "arguments": arguments
                        }),
                        false,
                    )?,
                ));
            }
            EventKind::ToolResult => {
                let call = text(&value, "call")?;
                let turn = turn(event)?;
                let replaced_step =
                    value
                        .get("supersedes")
                        .and_then(Value::as_array)
                        .and_then(|ranges| {
                            ranges.iter().find_map(|range| {
                                let from = range.get("from")?.as_u64()?;
                                let to = range.get("to")?.as_u64()?;
                                self.tool_result_steps
                                    .range(from..=to)
                                    .next()
                                    .map(|(_, step)| *step)
                            })
                        });
                let (surface_turn, step) = replaced_step
                    .or_else(|| self.open_steps.get(&turn).map(|info| (turn, info.step)))
                    .ok_or(ProjectionError::NoOpenStep(turn))?;
                let (content, error, meta) = tool_result_wire(&value, journal)?;
                self.tool_result_steps
                    .insert(event.seq(), (surface_turn, step));
                let mut payload = json!({
                    "turn": surface_turn,
                    "step": step,
                    "message": {
                        "id": format!("tool-{call}"),
                        "role": "user",
                        "content": content,
                        "source": {"kind": "tool", "callId": call}
                    },
                    "meta": meta
                });
                // Same convention as every other producer: `error` is absent on
                // success and a structured `{code, name}` on failure. A bare
                // `false` would read as a failure to a Client that only checks
                // presence.
                if let Some(error) = error {
                    payload["error"] = error;
                }
                output.push((
                    vec![event.seq()],
                    "tool-result".to_owned(),
                    session_event("tool/result", time, payload, true)?,
                ));
            }
            EventKind::Error => {
                let turn = turn(event)?;
                let classification = text(&value, "classification")?;
                let detail = value.get("detail").and_then(Value::as_str);
                output.push((
                    vec![event.seq()],
                    "response-error".to_owned(),
                    session_event(
                        "response/error",
                        time,
                        json!({
                            "turn": turn,
                            "message": response_error_message(classification, detail),
                            "classification": classification,
                            "recoverable": value.get("recoverable").and_then(Value::as_bool)
                                .unwrap_or(false)
                        }),
                        true,
                    )?,
                ));
            }
            EventKind::Settle => {
                let turn = turn(event)?;
                if let Some(previous) = self.open_steps.remove(&turn) {
                    output.push((
                        vec![previous.kernel_seq, event.seq()],
                        "step-end-at-settle".to_owned(),
                        session_event(
                            "step/end",
                            time,
                            json!({"turn": turn, "step": previous.step}),
                            false,
                        )?,
                    ));
                }
                let mut turn_end = json!({"turn": turn, "reason": settle_reason(&value)?});
                if let Some(validation) = value.get("validation") {
                    let output_seq = validation
                        .get("promoted_output_seq")
                        .and_then(Value::as_u64)
                        .ok_or(ProjectionError::InvalidField("promoted_output_seq"))?;
                    let (output_turn, message_id) = self
                        .outputs
                        .get(&output_seq)
                        .ok_or(ProjectionError::InvalidField("promoted_output_seq"))?;
                    if *output_turn != turn {
                        return Err(ProjectionError::InvalidField("promoted_output_seq"));
                    }
                    turn_end["promotedMessageID"] = json!(message_id);
                    turn_end["validationOutcome"] = validation["outcome"].clone();
                } else if let Some(output_seq) =
                    value.get("promoted_output_seq").and_then(Value::as_u64)
                {
                    let (output_turn, message_id) = self
                        .outputs
                        .get(&output_seq)
                        .ok_or(ProjectionError::InvalidField("promoted_output_seq"))?;
                    if *output_turn != turn {
                        return Err(ProjectionError::InvalidField("promoted_output_seq"));
                    }
                    turn_end["promotedMessageID"] = json!(message_id);
                }
                if value.get("outcome").and_then(Value::as_str) == Some("error") {
                    turn_end["classification"] = Value::String(
                        value
                            .get("classification")
                            .and_then(Value::as_str)
                            .ok_or(ProjectionError::InvalidField("classification"))?
                            .to_owned(),
                    );
                }
                output.push((
                    vec![event.seq()],
                    "turn-end".to_owned(),
                    session_event("turn/end", time, turn_end, false)?,
                ));
            }
            EventKind::Meta if value.get("notice").is_some() => {
                let notice = value
                    .get("notice")
                    .and_then(Value::as_object)
                    .ok_or(ProjectionError::InvalidField("notice"))?;
                let field = |name: &'static str| {
                    notice
                        .get(name)
                        .and_then(Value::as_str)
                        .ok_or(ProjectionError::MissingField(name))
                };
                let severity = field("severity")?;
                if !matches!(severity, "warning" | "error") {
                    return Err(ProjectionError::InvalidField("severity"));
                }
                output.push((
                    vec![event.seq()],
                    "session-notice".to_owned(),
                    session_event(
                        "session/notice",
                        time,
                        json!({
                            "severity": severity,
                            "classification": field("classification")?,
                            "operation": field("operation")?,
                            "message": field("message")?,
                        }),
                        true,
                    )?,
                ));
            }
            EventKind::Meta if value.get("title").is_some() => {
                output.push((
                    vec![event.seq()],
                    "session-title".to_owned(),
                    session_event(
                        "session/title",
                        time,
                        json!({"title": text(&value, "title")?}),
                        false,
                    )?,
                ));
            }
            _ => {}
        }
        Ok(output)
    }

    fn attempt(&self, attempt: &str) -> Result<&AttemptInfo, ProjectionError> {
        self.attempts
            .get(attempt)
            .ok_or_else(|| ProjectionError::UnknownAttempt(attempt.to_owned()))
    }

    fn validate_existing_kernel_slots(
        &self,
        journal: &EndpointJournal,
    ) -> Result<(), ProjectionError> {
        let Some(processed_through) = self.processed_through else {
            return Ok(());
        };
        for record in journal.records()? {
            let slot = record.slot;
            let Some(anchor) = record.kernel_seqs.last().copied() else {
                continue;
            };
            if anchor <= processed_through
                && !self.expected_kernel_slots.contains(&(anchor, slot.clone()))
            {
                return Err(ProjectionError::UnexpectedKernelSlot { anchor, slot });
            }
        }
        Ok(())
    }
}

type Projected = (Vec<u64>, String, SessionEvent);

fn user_message(
    input: &Value,
    input_seq: u64,
    _turn: u64,
    time: f64,
) -> Result<SessionEvent, ProjectionError> {
    let rpc_id = input
        .get("origin_key")
        .and_then(Value::as_str)
        .unwrap_or("kernel");
    let mut data = json!({
        "id": format!("input-{input_seq}"),
        "role": "user",
        "content": input
            .get("content")
            .map_or_else(|| json!([]), crate::attachment::project_content_blocks),
        "source": {"kind": "user", "rpcId": rpc_id}
    });
    if let Some(submission) = input.get("submission") {
        data["submission"] = submission.clone();
    }
    session_event("user/message", time, data, true)
}

fn apply_surface_supersedes(
    kernel_event: &Event,
    journal: &EndpointJournal,
    wire: &mut SessionEvent,
) -> Result<(), ProjectionError> {
    if wire.surface_op.is_none() {
        return Ok(());
    }
    let value = event_value(kernel_event)?;
    let Some(ranges) = value.get("supersedes").and_then(Value::as_array) else {
        return Ok(());
    };
    let mut endpoint_sources = wire
        .source_event_seqs
        .take()
        .unwrap_or_default()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let records = journal.records()?;
    let mut replaced = Vec::new();
    for record in records {
        let surface = matches!(
            record.event.event_type.as_str(),
            "user/message" | "assistant/message" | "tool/result"
        );
        if surface
            && record.kernel_seqs.iter().any(|seq| {
                ranges.iter().any(|range| {
                    let from = range.get("from").and_then(Value::as_u64);
                    let to = range.get("to").and_then(Value::as_u64);
                    from.zip(to)
                        .is_some_and(|(from, to)| from <= *seq && *seq <= to)
                })
            })
        {
            replaced.push(record.event.seq);
        }
    }
    if let (Some(start), Some(end)) = (replaced.first().copied(), replaced.last().copied()) {
        endpoint_sources.extend(replaced);
        wire.source_event_seqs = Some(endpoint_sources.into_iter().collect());
        wire.surface_op = Some(SurfaceOperation::Replace {
            op: "replace".to_owned(),
            start,
            end,
        });
    } else if !endpoint_sources.is_empty() {
        wire.source_event_seqs = Some(endpoint_sources.into_iter().collect());
    }
    Ok(())
}

fn session_event(
    event_type: &str,
    time: f64,
    data: Value,
    surface: bool,
) -> Result<SessionEvent, ProjectionError> {
    if !kernel_may_emit_event_type(event_type) {
        return Err(ProjectionError::UnregisteredKernelEvent(
            event_type.to_owned(),
        ));
    }
    Ok(SessionEvent {
        event_type: event_type.to_owned(),
        seq: 0,
        time,
        data: ijson(data)?,
        ignorable: None,
        source_event_seqs: None,
        surface_op: surface.then(|| SurfaceOperation::Append("append".to_owned())),
    })
}

fn event_value(event: &Event) -> Result<Value, ProjectionError> {
    serde_json::to_value(event.raw()).map_err(ProjectionError::Json)
}

fn ijson(value: Value) -> Result<IJsonValue, ProjectionError> {
    let bytes = serde_json::to_vec(&value).map_err(ProjectionError::Json)?;
    IJsonValue::parse(&bytes).map_err(ProjectionError::Schema)
}

fn text<'a>(value: &'a Value, field: &'static str) -> Result<&'a str, ProjectionError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or(ProjectionError::MissingField(field))
}

fn turn(event: &Event) -> Result<u64, ProjectionError> {
    event.turn().ok_or(ProjectionError::MissingField("turn"))
}

fn wire_usage(value: &Value) -> Result<Value, ProjectionError> {
    let mut usage = Map::new();
    for (source, destination) in [
        ("input_tokens", "inputTokens"),
        ("output_tokens", "outputTokens"),
        ("cache_read", "cacheReadTokens"),
        ("cache_write", "cacheWriteTokens"),
        ("reasoning_tokens", "reasoningTokens"),
    ] {
        if let Some(raw) = value.get(source).and_then(Value::as_str) {
            let parsed = raw
                .parse::<u64>()
                .map_err(|_| ProjectionError::Usage(raw.to_owned()))?;
            if parsed > 9_007_199_254_740_991 {
                return Err(ProjectionError::Usage(raw.to_owned()));
            }
            usage.insert(destination.to_owned(), json!(parsed));
        }
    }
    Ok(Value::Object(usage))
}

#[cfg(test)]
mod reasoning_usage_projection_tests {
    use super::*;

    #[test]
    fn reasoning_usage_preserves_zero_missing_and_wire_integer_boundary() {
        assert_eq!(
            wire_usage(&json!({"reasoning_tokens":"0"})).unwrap(),
            json!({"reasoningTokens":0})
        );
        assert_eq!(
            wire_usage(&json!({"output_tokens":"2"})).unwrap(),
            json!({"outputTokens":2})
        );
        assert!(wire_usage(&json!({"reasoning_tokens":"9007199254740992"})).is_err());
        assert!(wire_usage(&json!({"reasoning_tokens":"unknown"})).is_err());
    }
}

fn tool_result_wire(
    value: &Value,
    journal: &EndpointJournal,
) -> Result<(Value, Option<Value>, Value), ProjectionError> {
    let outcome = value
        .get("outcome")
        .ok_or(ProjectionError::MissingField("outcome"))?;
    let original_content = match value.get("content") {
        Some(content) => resolve_spill(content, journal)?,
        None => json!([]),
    };
    let source_meta = value.get("meta").cloned();
    let (content, error, kernel_outcome, reason) = match outcome {
        Value::String(value) if value == "ok" => (original_content, None, "ok", None),
        Value::String(value) if value == "error" => {
            // A failed effect's durable content is `{code, message, retryable}`.
            // Passing that object through made the model — and the transcript row
            // — read raw JSON, while every other failure arm here reports a
            // sentence and its own code (2026-09-20: `Shell · {"code":"denied",
            // ...}`). The structured code belongs in `error`, the message in the
            // content. Any other error payload keeps the generic shape.
            match failure_object(&original_content) {
                Some((code, message)) => (
                    text_content(&message),
                    Some(tool_error(&code, &failure_error_name(&code))),
                    "error",
                    None,
                ),
                None => (
                    original_content,
                    Some(tool_error("tool-error", "ToolError")),
                    "error",
                    None,
                ),
            }
        }
        Value::Object(object) if object.get("aborted").and_then(Value::as_str) == Some("crash") => {
            (
                text_content("aborted: crash recovery"),
                Some(tool_error("aborted", "Aborted")),
                "aborted",
                Some("crash"),
            )
        }
        Value::Object(object)
            if object.get("aborted").and_then(Value::as_str) == Some("recovered") =>
        {
            (
                text_content("aborted: recovered, not executed"),
                Some(tool_error("aborted", "Aborted")),
                "aborted",
                Some("recovered"),
            )
        }
        Value::Object(object) if object.get("denied").and_then(Value::as_str).is_some() => {
            let reason = object
                .get("denied")
                .and_then(Value::as_str)
                .expect("guarded");
            (
                text_content(reason),
                Some(tool_error("denied", "Denied")),
                "denied",
                Some(reason),
            )
        }
        Value::Object(object) if object.get("withheld").and_then(Value::as_str).is_some() => {
            let reason = object
                .get("withheld")
                .and_then(Value::as_str)
                .expect("guarded");
            (
                text_content("output withheld"),
                Some(tool_error("withheld", "Withheld")),
                "withheld",
                Some(reason),
            )
        }
        _ => return Err(ProjectionError::InvalidField("outcome")),
    };
    let mut meta = Map::new();
    meta.insert("kernel_outcome".to_owned(), json!(kernel_outcome));
    if let Some(reason) = reason {
        meta.insert("reason".to_owned(), json!(reason));
    }
    if let Some(source) = source_meta {
        meta.insert("source".to_owned(), source);
    }
    Ok((content, error, Value::Object(meta)))
}

/// Reads the `{code, message, retryable}` object a failed backend effect stores
/// as its durable result content. The pipeline serializes that object into a
/// single text block on its way to the ledger, so the JSON arrives as text.
/// Any other shape (a validation sentence, a tool's own payload) is left alone.
fn failure_object(content: &Value) -> Option<(String, String)> {
    let object = match content {
        Value::Object(_) => content.clone(),
        Value::Array(blocks) => {
            let [block] = blocks.as_slice() else {
                return None;
            };
            if block.get("type").and_then(Value::as_str) != Some("text") {
                return None;
            }
            serde_json::from_str::<Value>(block.get("text")?.as_str()?).ok()?
        }
        _ => return None,
    };
    let object = object.as_object()?;
    // `retryable` is what distinguishes this record from a tool that happens to
    // report its own `{code, message}`.
    if !object.contains_key("retryable") {
        return None;
    }
    let code = object.get("code")?.as_str()?;
    let message = object.get("message")?.as_str()?;
    if code.is_empty() || message.is_empty() {
        return None;
    }
    Some((code.to_owned(), message.to_owned()))
}

/// `backend_protocol` -> `BackendProtocol`: the `name` half of the `{code, name}`
/// pair the contract specifies, derived from the code rather than a second list
/// that could drift from it.
fn failure_error_name(code: &str) -> String {
    code.split(['_', '-', '.'])
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut characters = segment.chars();
            match characters.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(characters).collect(),
            }
        })
        .collect()
}

fn tool_error(code: &str, name: &str) -> Value {
    json!({"code": code, "name": name})
}

fn text_content(value: &str) -> Value {
    json!([{"type": "text", "text": value}])
}

fn response_error_message(classification: &str, detail: Option<&str>) -> &'static str {
    match (classification, detail) {
        ("provider_terminal", Some("network_disabled")) => {
            "Network access is disabled for this workspace."
        }
        ("provider_terminal", Some("provider_unavailable")) => {
            "The selected provider is unavailable."
        }
        ("transport", Some("context_overflow")) => "The model context window was exceeded.",
        ("provider_terminal", _) => "The provider ended the response with an error.",
        ("unresolved_dispatch", _) => "The provider request outcome could not be resolved.",
        _ => "The provider request failed.",
    }
}

fn resolve_spill(value: &Value, journal: &EndpointJournal) -> Result<Value, ProjectionError> {
    let Some(spill) = value.get("$spill").and_then(Value::as_object) else {
        return Ok(value.clone());
    };
    let asset = spill
        .get("asset")
        .and_then(Value::as_str)
        .ok_or(ProjectionError::InvalidField("$spill.asset"))?;
    let expected_bytes = spill
        .get("bytes")
        .and_then(Value::as_u64)
        .ok_or(ProjectionError::InvalidField("$spill.bytes"))?;
    let assets = AssetStore::new(journal.thread_folder().join("assets"))?;
    let bytes = assets.read_verified(asset)?;
    if bytes.len() as u64 != expected_bytes {
        return Err(ProjectionError::SpillLength {
            asset: asset.to_owned(),
            expected: expected_bytes,
            actual: bytes.len() as u64,
        });
    }
    let parsed = IJsonValue::parse(&bytes)?;
    serde_json::to_value(parsed).map_err(ProjectionError::Json)
}

fn settle_reason(value: &Value) -> Result<&'static str, ProjectionError> {
    match text(value, "outcome")? {
        "completed" => Ok("completed"),
        "error" => Ok("error"),
        "interrupted" => match value.get("reason").and_then(Value::as_str) {
            Some("user_stop") => Ok("aborted"),
            Some("budget_tokens") => Ok("max-tokens"),
            Some("budget_wall" | "recovered") => Ok("interrupted"),
            _ => Ok("interrupted"),
        },
        other => Err(ProjectionError::SettleOutcome(other.to_owned())),
    }
}

pub(crate) fn timestamp_millis(value: &str) -> Result<f64, ProjectionError> {
    if value.len() != 24
        || &value[4..5] != "-"
        || &value[7..8] != "-"
        || &value[10..11] != "T"
        || &value[13..14] != ":"
        || &value[16..17] != ":"
        || &value[19..20] != "."
        || &value[23..24] != "Z"
    {
        return Err(ProjectionError::Timestamp(value.to_owned()));
    }
    let parse = |range: std::ops::Range<usize>| {
        value[range]
            .parse::<i64>()
            .map_err(|_| ProjectionError::Timestamp(value.to_owned()))
    };
    let year = parse(0..4)?;
    let month = parse(5..7)?;
    let day = parse(8..10)?;
    let hour = parse(11..13)?;
    let minute = parse(14..16)?;
    let second = parse(17..19)?;
    let millis = parse(20..23)?;
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || !(0..=23).contains(&hour)
        || !(0..=59).contains(&minute)
        || !(0..=59).contains(&second)
    {
        return Err(ProjectionError::Timestamp(value.to_owned()));
    }
    let days = days_from_civil(year, month, day);
    Ok(((days * 86_400 + hour * 3_600 + minute * 60 + second) * 1_000 + millis) as f64)
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - i64::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let adjusted_month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * adjusted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

#[derive(Debug, Error)]
pub enum ProjectionError {
    #[error("endpoint journal failed: {0}")]
    Journal(#[from] JournalError),
    #[error("kernel store failed: {0}")]
    Store(#[from] store::StoreError),
    #[error("kernel schema failed: {0}")]
    Schema(#[from] schema::SchemaError),
    #[error("JSON conversion failed: {0}")]
    Json(serde_json::Error),
    #[error("missing field {0}")]
    MissingField(&'static str),
    #[error("invalid field {0}")]
    InvalidField(&'static str),
    #[error("unknown input seq {0}")]
    UnknownInput(u64),
    #[error("unknown attempt {0}")]
    UnknownAttempt(String),
    #[error("attempt {0} is already terminal")]
    TerminalAttempt(String),
    #[error("unknown epoch {0}")]
    UnknownEpoch(String),
    #[error("turn {0} has no open step")]
    NoOpenStep(u64),
    #[error("unsupported stream channel {0}")]
    StreamChannel(String),
    #[error("invalid wire usage {0}")]
    Usage(String),
    #[error("invalid settle outcome {0}")]
    SettleOutcome(String),
    #[error("invalid timestamp {0}")]
    Timestamp(String),
    #[error("SessionEvent {0} is not registered for the Kernel producer")]
    UnregisteredKernelEvent(String),
    #[error("journal contains unexpected kernel projection {anchor}:{slot}")]
    UnexpectedKernelSlot { anchor: u64, slot: String },
    #[error("spill asset {asset} length is {actual}, expected {expected}")]
    SpillLength {
        asset: String,
        expected: u64,
        actual: u64,
    },
}

#[cfg(test)]
mod tests {
    use store::AssetStore;

    use super::*;

    #[test]
    fn stream_usage_keeps_partial_reports_in_the_transient_lane() {
        let mut projector = Projector::default();
        projector.attempts.insert(
            "a".to_owned(),
            AttemptInfo {
                turn: 1,
                step: 2,
                epoch: "e".to_owned(),
                kernel_seq: 9,
            },
        );
        let frame = AcceptedStreamFrame {
            arguments_complete: None,
            attempt: "a".to_owned(),
            frame: 0,
            channel: "usage".to_owned(),
            block: 0,
            delta: r#"{"availability":"reported","output_tokens":"12"}"#.to_owned(),
            call_id: None,
            name: None,
            time: 1.0,
        };
        let event = projector.stream_event(&frame).unwrap();
        let data = serde_json::to_value(event.data).unwrap();
        assert_eq!(event.event_type, "assistant/chunk");
        assert_eq!(data["chunk"]["type"], "usage");
        assert_eq!(data["chunk"]["usage"], json!({"outputTokens":12}));
        projector.terminal_attempts.insert("a".to_owned());
        assert!(projector.stream_event(&frame).is_err());
    }

    #[test]
    fn user_message_projects_file_blocks_and_keeps_images_verbatim() {
        let digest = "b".repeat(64);
        let input = json!({
            "origin_key":"rpc-1",
            "content":[
                {"type":"image","asset":format!("sha256-{digest}"),"mime":"image/png"},
                {"type":"file","asset":format!("sha256-{digest}"),"mime":"text/plain","name":"a.txt","bytes":7}
            ]
        });
        let event = user_message(&input, 2, 1, 1.0).expect("projected");
        let data: Value =
            serde_json::from_slice(&serde_json::to_vec(&event.data).unwrap()).unwrap();
        assert_eq!(
            data["content"],
            json!([
                {"type":"image","asset":format!("sha256-{digest}"),"mime":"image/png"},
                {"type":"file","attachment":{"attachmentId":format!("sha256:{digest}"),"name":"a.txt","bytes":7},"mediaType":"text/plain"}
            ])
        );
        assert_eq!(data["source"]["rpcId"], "rpc-1");
    }

    #[test]
    fn durable_reasoning_folds_into_its_attempts_assistant_message() {
        let folder = tempfile::tempdir().unwrap();
        let journal = EndpointJournal::open(folder.path()).unwrap();
        let mut projector = Projector::default();
        projector.epochs.insert(
            "epoch".into(),
            EpochInfo {
                adapter: "deepseek_responses_v1".into(),
                model: "deepseek-flash".into(),
            },
        );
        for attempt in ["a1", "a2"] {
            projector.attempts.insert(
                attempt.into(),
                AttemptInfo {
                    turn: 1,
                    step: 1,
                    epoch: "epoch".into(),
                    kernel_seq: 4,
                },
            );
        }
        let mut messages = Vec::new();
        for (index, mut raw) in [
            json!({"kind":"reasoning","turn":1,"attempt":"a1","content":"think first"}),
            json!({"kind":"reasoning","turn":1,"attempt":"a1","content":"then decide"}),
            json!({"kind":"output","turn":1,"attempt":"a1","final_answer":true,"usage":{"availability":"unavailable"},"content":[{"type":"text","text":"answer"}],"sealed":{"version":1,"adapter":"deepseek_responses_v1","fragments":"[]"}}),
            json!({"kind":"output","turn":1,"attempt":"a2","final_answer":true,"usage":{"availability":"unavailable"},"content":[{"type":"text","text":"plain"}],"sealed":{"version":1,"adapter":"deepseek_responses_v1","fragments":"[]"}}),
        ].into_iter().enumerate() {
            raw["seq"] = json!(index + 5); raw["v"] = json!(1);
            raw["ts"] = json!("2026-09-14T00:00:00.000Z");
            let event = Event::decode(&serde_json::to_vec(&raw).unwrap()).unwrap();
            let wire = projector.project(&event, &journal).unwrap();
            messages.extend(wire.into_iter().filter(|(_, _, e)| e.event_type == "assistant/message").map(|(_, _, e)| e));
        }
        assert_eq!(messages.len(), 2);
        let first: Value =
            serde_json::from_slice(&serde_json::to_vec(&messages[0].data).unwrap()).unwrap();
        assert_eq!(
            first["message"]["content"],
            json!([
                {"type":"reasoning","text":"think first"},
                {"type":"reasoning","text":"then decide"},
                {"type":"text","text":"answer"}
            ])
        );
        let second: Value =
            serde_json::from_slice(&serde_json::to_vec(&messages[1].data).unwrap()).unwrap();
        assert_eq!(
            second["message"]["content"],
            json!([{"type":"text","text":"plain"}])
        );
        assert!(
            projector.reasoning.is_empty(),
            "folded reasoning is released with its attempt"
        );
    }

    #[test]
    fn worker_final_and_validation_feedback_do_not_end_the_client_turn() {
        let folder = tempfile::tempdir().unwrap();
        let journal = EndpointJournal::open(folder.path()).unwrap();
        let mut projector = Projector::default();
        projector.epochs.insert(
            "epoch".into(),
            EpochInfo {
                adapter: "responses".into(),
                model: "test".into(),
            },
        );
        let mut ends = Vec::new();
        for (index, mut raw) in [
            json!({"kind":"output","turn":1,"attempt":"first","final_answer":true,"usage":{"availability":"unavailable"},"content":[{"type":"text","text":"draft"}],"sealed":{"version":1,"adapter":"responses","fragments":"[]"}}),
            json!({"kind":"state","turn":1,"subkind":"validation.feedback","visibility":"model","payload":{"instruction":"repair"}}),
            json!({"kind":"output","turn":1,"attempt":"repair","final_answer":true,"usage":{"availability":"unavailable"},"content":[{"type":"text","text":"repaired answer"}],"sealed":{"version":1,"adapter":"responses","fragments":"[]"}}),
            json!({"kind":"settle","turn":1,"outcome":"completed","validation":{"outcome":"pass","candidate_seq":10,"decision_seq":11,"promoted_output_seq":5}}),
        ].into_iter().enumerate() {
            raw["seq"] = json!(if index == 3 { 12 } else { index + 5 }); raw["v"] = json!(1);
            raw["ts"] = json!("2026-09-04T00:00:00.000Z");
            if let Some(attempt) = raw["attempt"].as_str() {
                projector.attempts.insert(attempt.into(), AttemptInfo { turn: 1, step: index as u64 + 1, epoch: "epoch".into(), kernel_seq: index as u64 + 4 });
            }
            let event = Event::decode(&serde_json::to_vec(&raw).unwrap()).unwrap();
            let wire = projector.project(&event, &journal).unwrap();
            let turn_ends = wire.iter().filter(|(_, _, e)| e.event_type == "turn/end").count();
            if index < 3 { assert_eq!(turn_ends, 0, "candidate or feedback cannot complete a turn"); }
            ends.extend(wire.into_iter().filter(|(_, _, e)| e.event_type == "turn/end"));
        }
        assert_eq!(ends.len(), 1);
        assert_eq!(ends[0].0, vec![12], "only settlement owns completion");
        let data = serde_json::to_value(&ends[0].2.data).unwrap();
        assert_eq!(data["promotedMessageID"], "assistant-first");
        assert_eq!(data["validationOutcome"], "pass");
    }

    #[test]
    fn direct_final_settlement_promotes_the_exact_output_without_a_review_outcome() {
        let folder = tempfile::tempdir().unwrap();
        let journal = EndpointJournal::open(folder.path()).unwrap();
        let mut projector = Projector::default();
        projector.epochs.insert(
            "epoch".into(),
            EpochInfo {
                adapter: "responses".into(),
                model: "test".into(),
            },
        );
        projector.attempts.insert(
            "answer".into(),
            AttemptInfo {
                turn: 1,
                step: 1,
                epoch: "epoch".into(),
                kernel_seq: 3,
            },
        );
        let mut events = Vec::new();
        for raw in [
            json!({"v":1,"seq":4,"kind":"output","turn":1,"attempt":"answer",
                "final_answer":true,"usage":{"availability":"unavailable"},
                "content":[{"type":"text","text":"done"}],
                "sealed":{"version":1,"adapter":"responses","fragments":"[]"},
                "ts":"2026-09-04T00:00:00.000Z"}),
            json!({"v":1,"seq":5,"kind":"settle","turn":1,"outcome":"completed",
                "promoted_output_seq":4,"ts":"2026-09-04T00:00:00.000Z"}),
        ] {
            let event = Event::decode(&serde_json::to_vec(&raw).unwrap()).unwrap();
            events.extend(projector.project(&event, &journal).unwrap());
        }
        let end = events
            .iter()
            .find(|(_, _, event)| event.event_type == "turn/end")
            .unwrap();
        let data = serde_json::to_value(&end.2.data).unwrap();
        assert_eq!(data["promotedMessageID"], "assistant-answer");
        assert!(data.get("validationOutcome").is_none());
    }

    #[test]
    fn session_notice_meta_projects_as_a_turn_independent_surface_event() {
        let folder = tempfile::tempdir().unwrap();
        let journal = EndpointJournal::open(folder.path()).unwrap();
        let mut projector = Projector::default();
        let raw = json!({
            "v": 1, "seq": 7, "ts": "2026-09-20T12:00:00.000Z", "kind": "meta",
            "notice": {
                "severity": "error", "classification": "worker_launch", "operation": "prompt",
                "message": "launch bindings could not be derived"
            },
            "origin_key": "session-notice.record:s:input-6"
        });
        let event = Event::decode(&serde_json::to_vec(&raw).unwrap()).unwrap();
        let wire = projector.project(&event, &journal).unwrap();
        assert_eq!(wire.len(), 1);
        let (seqs, slot, projected) = &wire[0];
        assert_eq!(seqs, &vec![7]);
        assert_eq!(slot, "session-notice");
        assert_eq!(projected.event_type, "session/notice");
        assert!(
            projected.surface_op.is_some(),
            "a notice is a transcript row"
        );
        let data = serde_json::to_value(&projected.data).unwrap();
        assert_eq!(
            data,
            json!({
                "severity": "error", "classification": "worker_launch", "operation": "prompt",
                "message": "launch bindings could not be derived"
            })
        );

        // The grade is closed at the schema, before any projection runs.
        let mut bad = raw.clone();
        bad["notice"]["severity"] = json!("fatal");
        assert!(Event::decode(&serde_json::to_vec(&bad).unwrap()).is_err());
    }

    #[test]
    fn trim_of_closed_prior_turn_tool_result_keeps_original_surface_step() {
        let folder = tempfile::tempdir().unwrap();
        let journal = EndpointJournal::open(folder.path()).unwrap();
        let mut projector = Projector::default();
        projector.open_steps.insert(
            1,
            AttemptInfo {
                turn: 1,
                step: 2,
                epoch: "epoch".into(),
                kernel_seq: 10,
            },
        );
        let event = |raw: Value| Event::decode(&serde_json::to_vec(&raw).unwrap()).unwrap();
        let original = event(json!({
            "v":1,"seq":14,"ts":"2026-09-23T07:00:00.000Z","turn":1,
            "kind":"tool_result","call":"old-call","outcome":"ok",
            "content":[{"type":"text","text":"original"}]
        }));
        projector.project(&original, &journal).unwrap();
        projector.open_steps.remove(&1);
        let trimmed = event(json!({
            "v":1,"seq":438,"ts":"2026-09-23T07:04:40.000Z","turn":43,
            "kind":"tool_result","call":"old-call","outcome":"ok",
            "content":[{"type":"text","text":"trimmed"}],
            "supersedes":[{"from":14,"to":14}]
        }));
        let wire = projector.project(&trimmed, &journal).unwrap();
        let data = serde_json::to_value(&wire[0].2.data).unwrap();
        assert_eq!(data["turn"], 1);
        assert_eq!(data["step"], 2);
        assert_eq!(projector.tool_result_steps.get(&438), Some(&(1, 2)));
    }

    #[test]
    fn kernel_producer_rejects_dsh_private_state_events() {
        for event_type in [
            "model/selection",
            "session-log-deepseek/delivery-accepted",
            "subagent/model-selection-policy",
        ] {
            assert!(matches!(
                session_event(event_type, 1.0, json!({}), false),
                Err(ProjectionError::UnregisteredKernelEvent(value)) if value == event_type
            ));
        }
        session_event("turn/start", 1.0, json!({"turn": 0}), false).expect("shared Kernel event");
    }

    #[test]
    fn provider_error_projects_as_turn_scoped_transcript_surface() {
        let folder = tempfile::tempdir().expect("folder");
        let journal = EndpointJournal::open(folder.path()).expect("journal");
        let event = Event::decode(
            br#"{"classification":"provider_terminal","detail":"network_disabled","kind":"error","recoverable":false,"seq":5,"ts":"2026-09-02T14:12:34.029Z","turn":1,"v":1}"#,
        )
        .expect("provider error");

        let projected = Projector::default()
            .project(&event, &journal)
            .expect("projection");
        let wire = &projected.first().expect("response error event").2;
        let data = serde_json::to_value(&wire.data).expect("wire data");
        assert_eq!(wire.event_type, "response/error");
        assert_eq!(
            data.get("message").and_then(Value::as_str),
            Some("Network access is disabled for this workspace.")
        );
        assert_eq!(
            wire.surface_op,
            Some(SurfaceOperation::Append("append".to_owned()))
        );
    }

    /// A failed backend effect stores `{code, message, retryable}` as its durable
    /// content. Handing that object to the model and the transcript row made both
    /// read raw JSON, and flattened every cause to `tool-error` (2026-09-20:
    /// `Shell · {"code":"denied",...}`). The message is the content, the code is
    /// the error.
    #[test]
    fn a_failed_effect_reports_its_message_and_its_own_code() {
        let folder = tempfile::tempdir().expect("folder");
        let journal = EndpointJournal::open(folder.path()).expect("journal");

        // The pipeline serializes the failure object into one text block.
        let value = json!({
            "outcome": "error",
            "content": [{"type": "text", "text": json!({
                "code": "denied",
                "message": "operation denied: path is outside the allowed roots; use a path under /w",
                "retryable": false,
            }).to_string()}],
        });
        let (content, error, meta) = tool_result_wire(&value, &journal).expect("mapping");
        let error = error.expect("a failure carries a structured error");
        assert_eq!(error["code"], "denied");
        assert_eq!(error["name"], "Denied");
        assert_eq!(
            content[0]["text"],
            "operation denied: path is outside the allowed roots; use a path under /w"
        );
        assert_eq!(meta["kernel_outcome"], "error");

        // A multi-word code still reads as one CamelCase name.
        let value = json!({
            "outcome": "error",
            "content": [{"type": "text", "text": json!({
                "code": "backend_protocol", "message": "no", "retryable": true,
            }).to_string()}],
        });
        let (_, error, _) = tool_result_wire(&value, &journal).expect("mapping");
        assert_eq!(error.expect("error")["name"], "BackendProtocol");

        // Anything that is not that object keeps the generic shape: a tool that
        // failed with its own payload is not reinterpreted.
        for content in [
            json!([{"type": "text", "text": "invalid arguments at $.steps: must be an array"}]),
            json!([{"type": "text", "text": json!({
                "code": "denied", "message": "no retryable key",
            }).to_string()}]),
            json!([
                {"type": "text", "text": "one"},
                {"type": "text", "text": json!({
                    "code": "denied", "message": "two blocks", "retryable": false,
                }).to_string()},
            ]),
        ] {
            let value = json!({"outcome": "error", "content": content.clone()});
            let (projected, error, _) = tool_result_wire(&value, &journal).expect("mapping");
            assert_eq!(projected, content);
            assert_eq!(error.expect("error")["code"], "tool-error");
        }
    }

    #[test]
    fn non_success_tool_results_have_closed_wire_shapes_and_spills_resolve() {
        let folder = tempfile::tempdir().expect("folder");
        let journal = EndpointJournal::open(folder.path()).expect("journal");
        let cases = [
            (
                json!({"outcome": {"aborted": "crash"}}),
                "aborted",
                "crash",
                "aborted: crash recovery",
            ),
            (
                json!({"outcome": {"aborted": "recovered"}}),
                "aborted",
                "recovered",
                "aborted: recovered, not executed",
            ),
            (
                json!({"outcome": {"denied": "policy"}}),
                "denied",
                "policy",
                "policy",
            ),
            (
                json!({"outcome": {"withheld": "scan_failed"}}),
                "withheld",
                "scan_failed",
                "output withheld",
            ),
        ];
        for (value, outcome, reason, message) in cases {
            let (content, error, meta) = tool_result_wire(&value, &journal).expect("mapping");
            let error = error.expect("a non-ok outcome carries a structured error");
            assert!(error.get("code").and_then(Value::as_str).is_some());
            assert!(error.get("name").and_then(Value::as_str).is_some());
            assert_eq!(meta["kernel_outcome"], outcome);
            assert_eq!(meta["reason"], reason);
            assert_eq!(content[0]["text"], message);
        }

        let bytes = br#"[{"text":"asset result","type":"text"}]"#;
        let asset = AssetStore::new(folder.path().join("assets"))
            .expect("assets")
            .publish(bytes)
            .expect("publish");
        let value = json!({
            "outcome": "ok",
            "content": {"$spill": {"asset": asset.asset, "bytes": asset.bytes}}
        });
        let (content, error, _) = tool_result_wire(&value, &journal).expect("spill mapping");
        assert!(error.is_none(), "an ok outcome carries no error key");
        assert_eq!(content[0]["text"], "asset result");
    }
}
