//! Compact transition gate: the first provider attempt that belongs to the
//! post-compaction generation and admits the inputs queued behind the compact
//! request. Pure over the ledger; the worker and the supervisor own the writes.
//!
//! Legacy AppServer expressed the same acceptance rule over digested records
//! (`compact.request` pending → post input → new projection generation →
//! `compact.request.settlement` effective → first frame of the new generation
//! referencing the post input). In Kernel the manually originated `compact`
//! event is the request and its settlement at once (ack-bearing, receipted),
//! the `epoch{reason: "compaction"}` appended after it is the new generation,
//! and the `attempt` bound to that epoch is the frame. A frame bound to an
//! earlier epoch is stale even when it admits the same inputs, and without the
//! originated compact there is no gate to pass.
use schema::{Event, EventKind};
use serde_json::Value;

/// Seq of the first attempt on the compaction epoch that follows the manual
/// compact originated with `origin_key` and admits every seq in `post_inputs`.
#[must_use]
pub fn first_post_compact_attempt(
    events: &[Event],
    origin_key: &str,
    post_inputs: &[u64],
) -> Option<u64> {
    let compact = events.iter().find(|event| {
        *event.kind() == EventKind::Compact && event.string_field("origin_key") == Some(origin_key)
    })?;
    let epoch = events.iter().find(|event| {
        event.seq() > compact.seq()
            && *event.kind() == EventKind::Epoch
            && event.string_field("reason") == Some("compaction")
    })?;
    let epoch_id = epoch.string_field("id")?;
    events
        .iter()
        .find(|event| {
            event.seq() > epoch.seq()
                && *event.kind() == EventKind::Attempt
                && event.string_field("epoch") == Some(epoch_id)
                && admits_all(event, post_inputs)
        })
        .map(Event::seq)
}

fn admits_all(attempt: &Event, post_inputs: &[u64]) -> bool {
    let Ok(raw) = serde_json::to_value(attempt.raw()) else {
        return false;
    };
    let ranges = raw
        .get("admits")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    post_inputs.iter().all(|input| {
        ranges.iter().any(|range| {
            range
                .get("from")
                .and_then(Value::as_u64)
                .is_some_and(|from| from <= *input)
                && range
                    .get("to")
                    .and_then(Value::as_u64)
                    .is_some_and(|to| *input <= to)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::first_post_compact_attempt;
    use schema::{Event, IJsonValue};
    use serde_json::{Value, json};

    fn event(value: Value) -> Event {
        Event::from_value(IJsonValue::parse(&serde_json::to_vec(&value).unwrap()).unwrap()).unwrap()
    }

    fn epoch(seq: u64, id: &str, reason: &str) -> Value {
        json!({"v":1,"seq":seq,"kind":"epoch","ts":"2026-09-05T00:00:00.000Z","id":id,"reason":reason,
            "adapter":"openai_responses_v1","model":"m","system":{"asset":"a","digest":"d"},"tools":{"asset":"sha256-t","digest":"t"},"renderer":1})
    }

    fn attempt(seq: u64, id: &str, epoch: &str, admits: Value) -> Value {
        json!({"v":1,"seq":seq,"turn":2,"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"ts":"2026-09-05T00:00:00.000Z","attempt":id,
            "epoch":epoch,"wire_digest":"w","admits":admits})
    }

    const ORIGIN: &str = "compact:client:1";

    fn chain() -> Vec<Event> {
        let origin = json!({"principal":"user","client":"client","target":"thread","op":"compact","key":ORIGIN});
        [
            json!({"v":1,"seq":1,"kind":"input","ts":"2026-09-05T00:00:00.000Z","content":[{"type":"text","text":"PRE"}],
                "origin_key":"input:0","origin_tuple":{"principal":"user","client":"client","target":"thread","op":"input","key":"input:0"}}),
            json!({"v":1,"seq":2,"turn":1,"kind":"turn_open","ts":"2026-09-05T00:00:00.000Z","trigger":{"inputs":[1]}}),
            epoch(3, "e1", "initial"),
            json!({"v":1,"seq":4,"kind":"compact","ts":"2026-09-05T00:00:00.000Z","covers":[{"from":1,"to":3}],
                "summary":"pre","origin_key":ORIGIN,"origin_tuple":origin}),
            json!({"v":1,"seq":5,"kind":"input","ts":"2026-09-05T00:00:00.000Z","content":[{"type":"text","text":"POST"}],
                "origin_key":"input:1","origin_tuple":{"principal":"user","client":"client","target":"thread","op":"input","key":"input:1"}}),
            epoch(6, "e2", "compaction"),
            attempt(7, "frame", "e2", json!([{"from":5,"to":5}])),
        ]
        .into_iter()
        .map(event)
        .collect()
    }

    #[test]
    fn first_frame_of_the_compaction_generation_admitting_the_post_input_passes() {
        assert_eq!(first_post_compact_attempt(&chain(), ORIGIN, &[5]), Some(7));
    }

    #[test]
    fn absent_originated_compact_request_never_passes() {
        let without_request: Vec<Event> = chain()
            .into_iter()
            .filter(|event| event.seq() != 4)
            .collect();
        assert_eq!(
            first_post_compact_attempt(&without_request, ORIGIN, &[5]),
            None
        );
        assert_eq!(
            first_post_compact_attempt(&chain(), "compact:client:other", &[5]),
            None
        );
    }

    #[test]
    fn frame_bound_to_the_pre_compact_epoch_is_stale() {
        let mut stale = chain();
        stale.pop();
        stale.push(event(attempt(7, "stale", "e1", json!([{"from":5,"to":5}]))));
        assert_eq!(first_post_compact_attempt(&stale, ORIGIN, &[5]), None);
        let mut unadmitted = chain();
        unadmitted.pop();
        unadmitted.push(event(attempt(7, "partial", "e2", json!([]))));
        assert_eq!(first_post_compact_attempt(&unadmitted, ORIGIN, &[5]), None);
    }
}
