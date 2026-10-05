//! Pure context-compaction policy; the worker owns checkpoints and durable writes.
use schema::{Event, EventKind};
use serde_json::{Value, json};

pub struct ContextCompactionPlan {
    pub covers: Vec<u64>,
    pub summary: String,
}

/// Derive semantic anchors without rewriting the accepted history. Preserve
/// complete provider tool batches when retaining a result, so native replay
/// never sees a result whose assistant call was compacted away.
pub fn compaction_anchor_sequences(
    events: &[Event],
) -> Result<std::collections::BTreeSet<u64>, Box<dyn std::error::Error>> {
    let rows = events
        .iter()
        .map(|event| serde_json::to_value(event.raw()))
        .collect::<Result<Vec<_>, _>>()?;
    let mut superseded = std::collections::BTreeSet::new();
    let mut admitted_inputs = std::collections::BTreeSet::new();
    for row in &rows {
        superseded.extend(seqs_from_ranges(row.get("supersedes"))?);
        if row["kind"] == "turn_open" {
            if let Some(inputs) = row["trigger"]["inputs"].as_array() {
                admitted_inputs.extend(inputs.iter().filter_map(Value::as_u64));
            }
        }
    }
    let live = rows
        .iter()
        .filter(|row| !superseded.contains(&row["seq"].as_u64().unwrap()));
    let mut anchors = std::collections::BTreeSet::new();
    let mut calls = std::collections::BTreeMap::new();
    let mut latest = std::collections::BTreeMap::new();
    let mut initial_input = false;
    for row in live.clone() {
        let seq = row["seq"].as_u64().unwrap();
        if row["anchor"] == true {
            anchors.insert(seq);
        }
        if row["kind"] == "input" && !initial_input && admitted_inputs.contains(&seq) {
            anchors.insert(seq);
            initial_input = true;
        }
        if row["kind"] == "tool_call" {
            calls.insert(row["call"].as_str().unwrap(), row);
        }
        if matches!(row["kind"].as_str(), Some("tool_result" | "child_result")) {
            let Some(call) = row["call"].as_str().and_then(|id| calls.get(id)) else {
                continue;
            };
            let key = match call["name"].as_str() {
                Some("ask_user_questions") if row["outcome"] == "ok" => {
                    call["name"].as_str().unwrap().to_owned()
                }
                Some("task") if row["kind"] == "child_result" => format!(
                    "task:{}",
                    call["args"]["task_name"]
                        .as_str()
                        .unwrap_or(call["call"].as_str().unwrap())
                ),
                _ => continue,
            };
            latest.insert(key, seq);
        }
    }
    anchors.extend(latest.into_values());
    let mut attempts = std::collections::BTreeSet::new();
    for row in live
        .clone()
        .filter(|row| anchors.contains(&row["seq"].as_u64().unwrap()))
    {
        if let Some(attempt) = row["attempt"].as_str() {
            attempts.insert(attempt);
        }
        if let Some(call) = row["call"].as_str().and_then(|id| calls.get(id)) {
            if let Some(attempt) = call["attempt"].as_str() {
                attempts.insert(attempt);
            }
        }
    }
    let retained_calls = calls
        .iter()
        .filter_map(|(id, row)| {
            row["attempt"]
                .as_str()
                .is_some_and(|attempt| attempts.contains(attempt))
                .then_some(*id)
        })
        .collect::<std::collections::BTreeSet<_>>();
    for row in live {
        if row["attempt"]
            .as_str()
            .is_some_and(|attempt| attempts.contains(attempt))
            || row["call"]
                .as_str()
                .is_some_and(|call| retained_calls.contains(call))
        {
            anchors.insert(row["seq"].as_u64().unwrap());
        }
    }
    Ok(anchors)
}

pub fn plan_context_compaction(
    events: &[Event],
    turn: u64,
) -> Result<ContextCompactionPlan, Box<dyn std::error::Error>> {
    let mut input_turns = std::collections::BTreeMap::<u64, u64>::new();
    let mut superseded = std::collections::BTreeSet::<u64>::new();
    let mut already_covered = std::collections::BTreeSet::<u64>::new();
    for event in events {
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("event is not an object")?;
        for seq in seqs_from_ranges(object.get("supersedes"))? {
            superseded.insert(seq);
        }
        if *event.kind() == EventKind::Compact {
            for seq in seqs_from_ranges(object.get("covers"))? {
                already_covered.insert(seq);
            }
        }
        if *event.kind() == EventKind::TurnOpen {
            let event_turn = event.turn().ok_or("turn_open lacks turn")?;
            for seq in object
                .get("trigger")
                .and_then(|trigger| trigger.get("inputs"))
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_u64)
            {
                input_turns.insert(seq, event_turn);
            }
        }
    }
    let semantic_anchors = compaction_anchor_sequences(events)?;
    let call_names = events
        .iter()
        .filter(|event| *event.kind() == EventKind::ToolCall)
        .filter_map(|event| {
            Some((
                event.string_field("call")?.to_owned(),
                event.string_field("name")?.to_owned(),
            ))
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    // A result belongs to the turn of the call it answers, not to the turn of
    // the record that carries it: a trim replacement is stamped with the turn
    // that wrote it (event constraint 8), and judging it by that stamp would
    // let a compaction cover the assistant call while its result survives —
    // a tool output with no tool call, which every adapter rejects.
    let call_turns = events
        .iter()
        .filter(|event| *event.kind() == EventKind::ToolCall)
        .filter_map(|event| Some((event.string_field("call")?.to_owned(), event.turn()?)))
        .collect::<std::collections::BTreeMap<_, _>>();
    // An open turn can contain an entire coding session. Retire complete
    // older attempts as units, keeping the newest attempt and every batch
    // with an outstanding result. Result replacements follow their call,
    // even when the trim record was appended after the retained batch.
    let call_attempts = events
        .iter()
        .filter(|event| *event.kind() == EventKind::ToolCall)
        .filter_map(|event| Some((event.string_field("call")?, event.string_field("attempt")?)))
        .collect::<std::collections::BTreeMap<_, _>>();
    let answered = events
        .iter()
        .filter(|event| {
            *event.kind() == EventKind::ToolResult && !superseded.contains(&event.seq())
        })
        .filter_map(|event| event.string_field("call"))
        .collect::<std::collections::BTreeSet<_>>();
    let latest_attempt = events
        .iter()
        .rev()
        // A failed newer request does not prove that its input batch has
        // been consumed. Keep the last successful output's batch as well.
        .filter(|event| event.turn() == Some(turn) && *event.kind() == EventKind::Output)
        .find_map(|event| event.string_field("attempt"));
    let completed_attempts = events
        .iter()
        .filter(|event| event.turn() == Some(turn) && *event.kind() == EventKind::Output)
        .filter_map(|event| event.string_field("attempt"))
        .filter(|attempt| Some(*attempt) != latest_attempt)
        .filter(|attempt| {
            call_attempts
                .iter()
                .all(|(call, owner)| owner != attempt || answered.contains(call))
        })
        .collect::<std::collections::BTreeSet<_>>();
    let mut covered = Vec::new();
    let mut summary_lines = Vec::new();
    for event in events {
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("event is not an object")?;
        let historical = match event.kind() {
            EventKind::Input => input_turns
                .get(&event.seq())
                .is_some_and(|input_turn| *input_turn < turn),
            EventKind::ToolResult | EventKind::ChildResult => object
                .get("call")
                .and_then(Value::as_str)
                .and_then(|call| call_turns.get(call))
                .is_some_and(|call_turn| *call_turn < turn),
            EventKind::Output | EventKind::Reasoning | EventKind::ToolCall | EventKind::State => {
                event.turn().is_some_and(|event_turn| event_turn < turn)
            }
            _ => false,
        };
        let completed_step = match event.kind() {
            EventKind::Output | EventKind::Reasoning | EventKind::ToolCall => {
                event.turn() == Some(turn)
                    && event
                        .string_field("attempt")
                        .is_some_and(|attempt| completed_attempts.contains(attempt))
            }
            EventKind::ToolResult | EventKind::ChildResult => event
                .string_field("call")
                .and_then(|call| call_attempts.get(call))
                .is_some_and(|attempt| completed_attempts.contains(attempt)),
            _ => false,
        };
        let anchored = semantic_anchors.contains(&event.seq());
        if !(historical || completed_step)
            || anchored
            || superseded.contains(&event.seq())
            || already_covered.contains(&event.seq())
        {
            continue;
        }
        covered.push(event.seq());
        if let Some(line) = summary_line(event, object, &call_names)? {
            summary_lines.push(line);
        }
    }
    Ok(ContextCompactionPlan {
        covers: covered,
        summary: format!("[compacted history]\n{}", fit_summary(summary_lines)),
    })
}

/// Longest a single record may contribute. One oversized tool result must not
/// be able to crowd the rest of the conversation out of the summary.
const MAX_LINE_CHARS: usize = 600;
/// Total budget, matching the model summary's own continuation bound.
const MAX_SUMMARY_BYTES: usize = 12 * 1024;

/// One readable line per covered record, keeping its `seq` so the addresses
/// stay usable as evidence. Returns `None` for a record that carries nothing
/// a reader could use (an empty assistant turn, a runtime marker).
fn summary_line(
    event: &Event,
    object: &serde_json::Map<String, Value>,
    call_names: &std::collections::BTreeMap<String, String>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let seq = event.seq();
    let body = match event.kind() {
        EventKind::Input => format!("user: {}", blocks_text(object.get("content"))),
        EventKind::Output => format!("assistant: {}", blocks_text(object.get("content"))),
        EventKind::ToolCall => {
            let name = object.get("name").and_then(Value::as_str).unwrap_or("tool");
            format!("call {name}({})", compact_value(object.get("args")))
        }
        EventKind::ToolResult => {
            let name = object
                .get("call")
                .and_then(Value::as_str)
                .and_then(|call| call_names.get(call))
                .map_or("tool", String::as_str);
            let outcome = compact_value(object.get("outcome"));
            let text = blocks_text(object.get("content"));
            if text.is_empty() {
                format!("{name} -> {outcome}")
            } else {
                format!("{name} -> {outcome}: {text}")
            }
        }
        EventKind::ChildResult => format!(
            "subagent -> {}: {}",
            compact_value(object.get("outcome")),
            object
                .get("summary")
                .and_then(Value::as_str)
                .unwrap_or_default()
        ),
        EventKind::Reasoning => return Ok(None),
        EventKind::State => format!(
            "state {}: {}",
            object
                .get("subkind")
                .and_then(Value::as_str)
                .unwrap_or("unknown"),
            compact_value(object.get("payload"))
        ),
        // Covered kinds are model-visible by construction; anything else that
        // reaches here is quoted rather than silently dropped.
        _ => serde_json_canonicalizer::to_string(object)?,
    };
    let body = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if body.is_empty() {
        return Ok(None);
    }
    Ok(Some(format!("seq {seq} {}", clip(&body, MAX_LINE_CHARS))))
}

/// The text a reader can see in a block list, with spilled bodies named by
/// size rather than by their storage marker.
fn blocks_text(value: Option<&Value>) -> String {
    let Some(value) = value else {
        return String::new();
    };
    if let Some(spill) = value.get("$spill") {
        let bytes = spill
            .get("bytes")
            .and_then(Value::as_u64)
            .unwrap_or_default();
        return format!("[spilled {bytes} bytes]");
    }
    let Some(blocks) = value.as_array() else {
        return compact_value(Some(value));
    };
    blocks
        .iter()
        .map(|block| match block.get("type").and_then(Value::as_str) {
            Some("text") => block
                .get("text")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned(),
            Some(other) => format!("[{other}]"),
            None => compact_value(Some(block)),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// A JSON value rendered inline, with a spill named by size.
fn compact_value(value: Option<&Value>) -> String {
    match value {
        None => String::new(),
        Some(Value::String(text)) => text.clone(),
        Some(value) => {
            if let Some(spill) = value.get("$spill") {
                let bytes = spill
                    .get("bytes")
                    .and_then(Value::as_u64)
                    .unwrap_or_default();
                return format!("[spilled {bytes} bytes]");
            }
            serde_json_canonicalizer::to_string(value).unwrap_or_default()
        }
    }
}

fn clip(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        return text.to_owned();
    }
    let head = limit.saturating_sub(64).max(1);
    let kept: String = text.chars().take(head).collect();
    let tail: String = text
        .chars()
        .skip(text.chars().count().saturating_sub(48))
        .collect();
    format!("{kept} […] {tail}")
}

/// Join the lines within the total budget. When they do not fit, prune from
/// the middle: the opening of the task and the most recent work are the two
/// halves a continuation actually needs.
fn fit_summary(lines: Vec<String>) -> String {
    let joined = lines.join("\n");
    if joined.len() <= MAX_SUMMARY_BYTES {
        return joined;
    }
    let mut head = Vec::new();
    let mut tail = std::collections::VecDeque::new();
    let mut used = 0usize;
    let mut front = 0usize;
    let mut back = lines.len();
    // Alternate front and back so both ends grow until the budget is spent.
    while front < back {
        let take_front = head.len() <= tail.len();
        let index = if take_front { front } else { back - 1 };
        let cost = lines[index].len() + 1;
        if used + cost > MAX_SUMMARY_BYTES {
            break;
        }
        used += cost;
        if take_front {
            head.push(lines[index].clone());
            front += 1;
        } else {
            tail.push_front(lines[index].clone());
            back -= 1;
        }
    }
    let pruned = back.saturating_sub(front);
    head.push(format!("[... {pruned} records pruned ...]"));
    head.extend(tail);
    head.join("\n")
}

/// One historical `tool_result` whose rendered body is over budget. The plan
/// is pure over the ledger — sizes are read from the record, spilled bodies
/// from their declared `bytes` — and the worker materializes and rewrites the
/// content, because only it can read the asset store.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolResultTrim {
    pub seq: u64,
    pub call: String,
    pub bytes: usize,
}

/// Tool output above this many bytes is worth trimming before paying for a
/// summary compaction. Head and tail retained around the pruned middle.
pub const TRIM_THRESHOLD_BYTES: usize = 8 * 1024;
pub const TRIM_HEAD_BYTES: usize = 4 * 1024;
pub const TRIM_TAIL_BYTES: usize = 1024;

/// Tool results large enough to trim, largest first. Eligibility is bounded
/// by consumption rather than by turn: a result the model has already
/// answered — one preceding the latest `output` — can be shortened, while the
/// results of the batch it is working on right now stay whole. That matters
/// because an agent session is one long turn, and
/// [`plan_context_compaction`] can only cover *earlier* turns: inside the open
/// turn this is the only pressure relief there is. A result already
/// superseded or covered by a compact is left alone; it does not render.
pub fn plan_tool_result_trim(
    events: &[Event],
) -> Result<Vec<ToolResultTrim>, Box<dyn std::error::Error>> {
    let mut hidden = std::collections::BTreeSet::new();
    for event in events {
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("event is not an object")?;
        for seq in seqs_from_ranges(object.get("supersedes"))? {
            hidden.insert(seq);
        }
        if *event.kind() == EventKind::Compact {
            for seq in seqs_from_ranges(object.get("covers"))? {
                hidden.insert(seq);
            }
        }
    }
    let consumed_through = events
        .iter()
        .rev()
        .find(|event| *event.kind() == EventKind::Output)
        .map_or(0, Event::seq);
    let mut targets = Vec::new();
    for event in events {
        if *event.kind() != EventKind::ToolResult
            || hidden.contains(&event.seq())
            || event.seq() > consumed_through
        {
            continue;
        }
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("event is not an object")?;
        let Some(content) = object.get("content") else {
            continue;
        };
        let bytes = rendered_bytes(content)?;
        if bytes > TRIM_THRESHOLD_BYTES {
            targets.push(ToolResultTrim {
                seq: event.seq(),
                call: object
                    .get("call")
                    .and_then(Value::as_str)
                    .ok_or("tool_result lacks call")?
                    .to_owned(),
                bytes,
            });
        }
    }
    // Largest first: the fewest replacements that relieve the most pressure.
    targets.sort_by(|left, right| right.bytes.cmp(&left.bytes).then(left.seq.cmp(&right.seq)));
    Ok(targets)
}

/// Size of a content payload without materializing it: a spilled body
/// declares its own byte count, an inline one is measured canonically.
fn rendered_bytes(content: &Value) -> Result<usize, Box<dyn std::error::Error>> {
    if let Some(spill) = content.get("$spill") {
        return Ok(usize::try_from(
            spill
                .get("bytes")
                .and_then(Value::as_u64)
                .unwrap_or_default(),
        )
        .unwrap_or(usize::MAX));
    }
    Ok(serde_json_canonicalizer::to_string(content)?.len())
}

/// The replacement body for a trimmed result: every non-text block is kept in
/// order, and the text is reduced to a bounded head and tail around one
/// marker naming the original record.
#[must_use]
pub fn trimmed_tool_result_content(content: &Value, original_seq: u64) -> Value {
    let Some(blocks) = content.as_array() else {
        return content.clone();
    };
    let mut output = Vec::new();
    for block in blocks {
        if block.get("type").and_then(Value::as_str) != Some("text") {
            output.push(block.clone());
            continue;
        }
        let text = block
            .get("text")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if text.len() <= TRIM_THRESHOLD_BYTES {
            output.push(block.clone());
            continue;
        }
        let mut head = TRIM_HEAD_BYTES.min(text.len());
        while !text.is_char_boundary(head) {
            head -= 1;
        }
        let mut tail = text.len() - TRIM_TAIL_BYTES.min(text.len());
        while !text.is_char_boundary(tail) {
            tail += 1;
        }
        let pruned = tail.saturating_sub(head);
        output.push(json!({
            "type": "text",
            "text": format!(
                "{}\n\n[... {pruned} bytes of tool output pruned; the whole result is durable at seq {original_seq} ...]\n\n{}",
                &text[..head],
                &text[tail..]
            )
        }));
    }
    Value::Array(output)
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

#[cfg(test)]
mod summary_tests {
    use super::*;
    use serde_json::json;

    fn event(value: Value) -> Event {
        Event::decode_canonical(&serde_json_canonicalizer::to_vec(&value).expect("canonical"))
            .expect("event")
    }

    fn ledger(rows: Vec<Value>) -> Vec<Event> {
        let mut events = vec![
            event(
                json!({"v":1,"seq":1,"kind":"genesis","ts":TS,"format":1,"thread":THREAD,"workspace":"ws","config":{"digest":"cfg"},"resume":"never","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"principal":"p","client":"cli","target":THREAD,"op":"create","key":"create"}}),
            ),
            event(
                json!({"v":1,"seq":2,"kind":"input","ts":TS,"content":[{"type":"text","text":"write the ledger"}],"origin_key":"i1","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i1"}}),
            ),
            event(
                json!({"v":1,"seq":3,"kind":"turn_open","ts":TS,"turn":1,"trigger":{"inputs":[2]}}),
            ),
        ];
        events.extend(rows.into_iter().map(event));
        events
    }

    const TS: &str = "2026-09-06T00:00:00.000Z";
    const THREAD: &str = "018f0000-0000-7000-8000-000000000003";

    #[test]
    fn open_turn_compaction_keeps_current_and_incomplete_batches_and_pairs_trimmed_results() {
        let mut events = ledger(vec![
            json!({"v":1,"seq":4,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c1","name":"shell","args":{},"source":"provider"}),
            json!({"v":1,"seq":5,"turn":1,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok","content":[{"type":"text","text":"original"}]}),
            json!({"v":1,"seq":6,"turn":1,"kind":"output","ts":TS,"attempt":"a1","content":[],"sealed":{"version":1,"adapter":"fake","fragments":"[]"},"usage":{"availability":"unavailable"}}),
            json!({"v":1,"seq":7,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a2","call":"c2","name":"shell","args":{},"source":"provider"}),
            json!({"v":1,"seq":8,"turn":1,"kind":"output","ts":TS,"attempt":"a2","content":[],"sealed":{"version":1,"adapter":"fake","fragments":"[]"},"usage":{"availability":"unavailable"}}),
            json!({"v":1,"seq":9,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a3","call":"c3","name":"read","args":{},"source":"provider"}),
            json!({"v":1,"seq":10,"turn":1,"kind":"tool_result","ts":TS,"call":"c3","outcome":"ok","content":[]}),
            json!({"v":1,"seq":11,"turn":1,"kind":"output","ts":TS,"attempt":"a3","content":[],"sealed":{"version":1,"adapter":"fake","fragments":"[]"},"usage":{"availability":"unavailable"}}),
            json!({"v":1,"seq":12,"turn":1,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok","content":[{"type":"text","text":"trimmed history"}],"supersedes":[{"from":5,"to":5}]}),
        ]);
        let plan = plan_context_compaction(&events, 1).unwrap();
        assert_eq!(plan.covers, vec![4, 6, 12]);
        assert!(plan.summary.contains("trimmed history"));
        assert!(!plan.summary.contains("original"));
        events.push(event(json!({"v":1,"seq":13,"turn":1,"kind":"error","ts":TS,
            "attempt":"a4","classification":"provider_terminal","detail":"context_overflow","recoverable":true,"usage":{"availability":"unavailable"}})));
        assert_eq!(
            plan_context_compaction(&events, 1).unwrap().covers,
            vec![4, 6, 12],
            "a failed newer request does not retire the current batch"
        );
        events.pop();
        // An explicit semantic anchor on the result retains its whole batch.
        let mut anchored = serde_json::to_value(events.pop().unwrap().raw()).unwrap();
        anchored["anchor"] = json!(true);
        events.push(event(anchored));
        assert!(
            plan_context_compaction(&events, 1)
                .unwrap()
                .covers
                .is_empty()
        );
    }

    /// The fallback a failed summary request falls back to must read as
    /// history, not as a transcript of the storage format: the model gets it
    /// as its own conversation.
    #[test]
    fn the_deterministic_summary_quotes_history_rather_than_json() {
        let events = ledger(vec![
            json!({"v":1,"seq":4,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c1","name":"read","args":{"path":"a.txt"},"source":"provider"}),
            json!({"v":1,"seq":5,"turn":1,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok","content":[{"type":"text","text":"file body"}]}),
            json!({"v":1,"seq":6,"turn":1,"kind":"output","ts":TS,"attempt":"a1","content":[{"type":"text","text":"done"}],"sealed":{"version":1,"adapter":"fake","fragments":"[]"},"usage":{"availability":"unavailable"}}),
            json!({"v":1,"seq":7,"turn":1,"kind":"settle","ts":TS,"outcome":"completed"}),
            json!({"v":1,"seq":8,"kind":"input","ts":TS,"content":[{"type":"text","text":"next"}],"origin_key":"i2","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i2"}}),
            json!({"v":1,"seq":9,"kind":"turn_open","ts":TS,"turn":2,"trigger":{"inputs":[8]}}),
        ]);
        let plan = plan_context_compaction(&events, 2).expect("plan");
        let summary = plan.summary.clone();
        assert!(summary.starts_with("[compacted history]\n"), "{summary}");
        assert!(
            !summary.contains("\"kind\":"),
            "no raw event JSON: {summary}"
        );
        // The first admitted input is a semantic anchor: it stays verbatim in
        // the render instead of being summarized, so it is not covered here.
        assert!(!plan.covers.contains(&2), "the opening input is an anchor");
        assert!(summary.contains("seq 4 call read("), "{summary}");
        // The result is named by the tool that produced it, not by its call id.
        assert!(summary.contains("seq 5 read -> ok: file body"), "{summary}");
        assert!(summary.contains("seq 6 assistant: done"), "{summary}");
    }

    /// A trim replacement is stamped with the turn that wrote it, so judging
    /// coverage by that stamp would compact the assistant call while its
    /// result survived — a tool output with no tool call, which every adapter
    /// rejects with a 400. A live corpus run hit exactly that. Coverage
    /// follows the call the result answers.
    #[test]
    fn a_result_is_covered_with_the_call_it_answers_not_with_its_own_stamp() {
        let events = ledger(vec![
            json!({"v":1,"seq":4,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c1","name":"shell","args":{},"source":"provider"}),
            json!({"v":1,"seq":5,"turn":1,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok","content":[{"type":"text","text":"whole"}]}),
            json!({"v":1,"seq":6,"turn":1,"kind":"output","ts":TS,"attempt":"a1","content":[{"type":"text","text":"ok"}],"sealed":{"version":1,"adapter":"fake","fragments":"[]"},"usage":{"availability":"unavailable"}}),
            json!({"v":1,"seq":7,"turn":1,"kind":"settle","ts":TS,"outcome":"completed"}),
            json!({"v":1,"seq":8,"kind":"input","ts":TS,"content":[{"type":"text","text":"next"}],"origin_key":"i2","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i2"}}),
            json!({"v":1,"seq":9,"kind":"turn_open","ts":TS,"turn":2,"trigger":{"inputs":[8]}}),
            // Turn 2 trims turn 1's result: the replacement carries turn 2.
            json!({"v":1,"seq":10,"turn":2,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok","content":[{"type":"text","text":"whole [...]"}],"supersedes":[{"from":5,"to":5}]}),
        ]);
        let plan = plan_context_compaction(&events, 3).expect("plan");
        let covered = plan
            .covers
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        assert!(covered.contains(&4), "the assistant call is history");
        assert!(
            covered.contains(&10),
            "its live result goes with it even though the record says turn 2: {covered:?}"
        );
    }

    /// One oversized record must not be able to spend the whole budget, and
    /// when the whole history still does not fit, the pruning happens in the
    /// middle so the opening of the task and the most recent work survive.
    #[test]
    fn a_huge_tool_result_cannot_crowd_out_the_rest_of_the_history() {
        let mut rows = Vec::new();
        let mut seq = 4;
        for index in 0..40 {
            rows.push(json!({"v":1,"seq":seq,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":format!("c{index}"),"name":"shell","args":{"command":"echo"},"source":"provider"}));
            rows.push(json!({"v":1,"seq":seq+1,"turn":1,"kind":"tool_result","ts":TS,"call":format!("c{index}"),"outcome":"ok","content":[{"type":"text","text":"x".repeat(2_000)}]}));
            seq += 2;
        }
        rows.push(json!({"v":1,"seq":seq,"turn":1,"kind":"output","ts":TS,"attempt":"a1","content":[{"type":"text","text":"finished"}],"sealed":{"version":1,"adapter":"fake","fragments":"[]"},"usage":{"availability":"unavailable"}}));
        rows.push(
            json!({"v":1,"seq":seq+1,"turn":1,"kind":"settle","ts":TS,"outcome":"completed"}),
        );
        rows.push(json!({"v":1,"seq":seq+2,"kind":"input","ts":TS,"content":[{"type":"text","text":"next"}],"origin_key":"i2","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i2"}}));
        rows.push(json!({"v":1,"seq":seq+3,"kind":"turn_open","ts":TS,"turn":2,"trigger":{"inputs":[seq+2]}}));
        let plan = plan_context_compaction(&ledger(rows), 2).expect("plan");
        let summary = plan.summary;
        assert!(
            summary.len() <= 13 * 1024,
            "within budget: {}",
            summary.len()
        );
        assert!(
            summary.contains("seq 4 call shell("),
            "the opening survives: {summary}"
        );
        assert!(
            summary.contains("assistant: finished"),
            "the newest work survives: {}",
            &summary[summary.len().saturating_sub(200)..]
        );
        assert!(
            summary.contains("records pruned"),
            "the middle is what gives way"
        );
        assert!(
            !summary.contains(&"x".repeat(1000)),
            "no single result spends the budget"
        );
    }
}

#[cfg(test)]
mod trim_tests {
    use super::*;
    use serde_json::json;

    const TS: &str = "2026-09-06T00:00:00.000Z";
    const THREAD: &str = "018f0000-0000-7000-8000-000000000003";

    fn event(value: Value) -> Event {
        Event::decode_canonical(&serde_json_canonicalizer::to_vec(&value).expect("canonical"))
            .expect("event")
    }

    fn session(rows: Vec<Value>) -> Vec<Event> {
        let mut events = vec![
            event(
                json!({"v":1,"seq":1,"kind":"genesis","ts":TS,"format":1,"thread":THREAD,"workspace":"ws","config":{"digest":"cfg"},"resume":"never","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{"principal":"p","client":"cli","target":THREAD,"op":"create","key":"create"}}),
            ),
            event(
                json!({"v":1,"seq":2,"kind":"input","ts":TS,"content":[{"type":"text","text":"go"}],"origin_key":"i1","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i1"}}),
            ),
            event(
                json!({"v":1,"seq":3,"kind":"turn_open","ts":TS,"turn":1,"trigger":{"inputs":[2]}}),
            ),
        ];
        events.extend(rows.into_iter().map(event));
        events
    }

    fn big(bytes: usize) -> Value {
        json!([{"type":"text","text":"y".repeat(bytes)}])
    }

    /// An agent session is one long turn, so eligibility follows consumption,
    /// not the turn boundary: a result the model has already answered can be
    /// shortened, the batch it is working on right now cannot.
    #[test]
    fn the_open_turns_answered_results_are_trimmable_and_the_newest_are_not() {
        let events = session(vec![
            json!({"v":1,"seq":4,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c1","name":"shell","args":{},"source":"provider"}),
            json!({"v":1,"seq":5,"turn":1,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok","content":big(12_000)}),
            json!({"v":1,"seq":6,"turn":1,"kind":"output","ts":TS,"attempt":"a1","content":[{"type":"text","text":"ok"}],"sealed":{"version":1,"adapter":"fake","fragments":"[]"},"usage":{"availability":"unavailable"}}),
            json!({"v":1,"seq":7,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a2","call":"c2","name":"shell","args":{},"source":"provider"}),
            json!({"v":1,"seq":8,"turn":1,"kind":"tool_result","ts":TS,"call":"c2","outcome":"ok","content":big(12_000)}),
        ]);
        let plan = plan_tool_result_trim(&events).expect("plan");
        assert_eq!(plan.len(), 1, "only the answered result: {plan:?}");
        assert_eq!(plan[0].seq, 5);
        assert_eq!(plan[0].call, "c1");
        assert!(plan[0].bytes > TRIM_THRESHOLD_BYTES);
    }

    /// Small results, retracted results and compacted results are left alone,
    /// and the largest target comes first.
    #[test]
    fn trim_targets_skip_hidden_records_and_lead_with_the_largest() {
        let events = session(vec![
            json!({"v":1,"seq":4,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c1","name":"shell","args":{},"source":"provider"}),
            json!({"v":1,"seq":5,"turn":1,"kind":"tool_result","ts":TS,"call":"c1","outcome":"ok","content":big(9_000)}),
            json!({"v":1,"seq":6,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c2","name":"shell","args":{},"source":"provider"}),
            // A really large body is spilled in production; the planner reads
            // its declared size instead of materializing it.
            json!({"v":1,"seq":7,"turn":1,"kind":"tool_result","ts":TS,"call":"c2","outcome":"ok","content":{"$spill":{"asset":format!("sha256-{}", "ab".repeat(32)),"bytes":40_000}}}),
            json!({"v":1,"seq":8,"turn":1,"kind":"tool_call","ts":TS,"attempt":"a1","call":"c3","name":"shell","args":{},"source":"provider"}),
            json!({"v":1,"seq":9,"turn":1,"kind":"tool_result","ts":TS,"call":"c3","outcome":"ok","content":[{"type":"text","text":"small"}]}),
            json!({"v":1,"seq":10,"kind":"checkpoint","ts":TS,"covers":9,"summary":"boundary"}),
            json!({"v":1,"seq":11,"kind":"compact","ts":TS,"covers":[{"from":4,"to":5}],"summary":"[compacted history]"}),
            json!({"v":1,"seq":12,"turn":1,"kind":"output","ts":TS,"attempt":"a1","content":[{"type":"text","text":"ok"}],"sealed":{"version":1,"adapter":"fake","fragments":"[]"},"usage":{"availability":"unavailable"}}),
        ]);
        let plan = plan_tool_result_trim(&events).expect("plan");
        assert_eq!(
            plan.iter().map(|t| t.seq).collect::<Vec<_>>(),
            vec![7],
            "{plan:?}"
        );
    }

    /// The replacement keeps a bounded head and tail and names where the whole
    /// output still lives; non-text blocks keep their order.
    #[test]
    fn the_replacement_keeps_a_head_a_tail_and_a_pointer_to_the_original() {
        let content = json!([
            {"type":"text","text":format!("HEAD{}TAIL", "z".repeat(60_000))},
            {"type":"image","data":"..."}
        ]);
        let trimmed = trimmed_tool_result_content(&content, 42);
        let blocks = trimmed.as_array().expect("blocks");
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[1]["type"], "image", "other blocks keep their order");
        let text = blocks[0]["text"].as_str().expect("text");
        assert!(text.starts_with("HEAD"), "the head survives");
        assert!(text.ends_with("TAIL"), "the tail survives");
        assert!(
            text.contains("durable at seq 42"),
            "the original is addressable: {}",
            &text[..120]
        );
        assert!(
            text.len() < 6 * 1024,
            "the replacement is bounded: {}",
            text.len()
        );
        // A result already within budget is returned unchanged.
        let small = json!([{"type":"text","text":"fine"}]);
        assert_eq!(trimmed_tool_result_content(&small, 1), small);
    }
}
