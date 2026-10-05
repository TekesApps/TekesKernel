//! The compaction summary (event §compact `summary_request`): the frozen
//! source bundle, admission of the model's `summary_artifact` against it, and
//! the telemetry recorded on the compact event. The admitted continuation is
//! the compact summary; the deterministic quoted history is the fallback when
//! the request fails, the artifact is rejected or the plan moved before the
//! write. Pure; the worker and the supervisor own the provider request and
//! the durable write.
use schema::Event;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

/// The continuation bound; the same bound the deterministic summary honours.
pub const MAX_CONTINUATION_BYTES: usize = 12 * 1024;

/// Bytes of rendered history the summary request may carry: 60 % of the
/// model window at the conservative four bytes per token.
pub fn summary_request_bytes(context_window_tokens: u64) -> usize {
    usize::try_from(context_window_tokens)
        .unwrap_or(usize::MAX)
        .saturating_mul(6)
        .saturating_div(10)
        .saturating_mul(4)
        .max(4096)
}

/// The planned covers frozen before the summary request: exact `seq`
/// addresses and a digest over their canonical bytes, plus the rendering the
/// compactor reads (one `seq <n> <kind> <json>` line per record).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceBundle {
    pub covers: Vec<u64>,
    pub sha256: String,
    pub rendered: String,
    pub rendered_bytes: usize,
    pub truncated: bool,
}

pub fn freeze_source_bundle(
    events: &[Event],
    covers: &[u64],
    max_rendered_bytes: usize,
) -> Result<SourceBundle, String> {
    let wanted = covers.iter().copied().collect::<BTreeSet<_>>();
    if wanted.is_empty() {
        return Err("a source bundle covers at least one record".to_owned());
    }
    let mut digest = Sha256::new();
    digest.update(b"tekes-compaction-v5-bundle\0");
    let mut lines = Vec::new();
    for event in events.iter().filter(|event| wanted.contains(&event.seq())) {
        let raw = serde_json::to_value(event.raw()).map_err(|error| error.to_string())?;
        let canonical =
            serde_json_canonicalizer::to_string(&raw).map_err(|error| error.to_string())?;
        digest.update(canonical.as_bytes());
        digest.update([b'\n']);
        lines.push(format!(
            "seq {} {} {}",
            event.seq(),
            event.kind().as_str(),
            canonical
        ));
    }
    if lines.len() != wanted.len() {
        return Err(format!(
            "bundle covers {} records but {} are present",
            wanted.len(),
            lines.len()
        ));
    }
    let mut rendered = lines.join("\n");
    let rendered_bytes = rendered.len();
    let truncated = rendered.len() > max_rendered_bytes;
    if truncated {
        let mut end = max_rendered_bytes;
        while !rendered.is_char_boundary(end) {
            end -= 1;
        }
        rendered.truncate(end);
        rendered.push_str("\n[bundle truncated]");
    }
    Ok(SourceBundle {
        covers: wanted.into_iter().collect(),
        sha256: format!("sha256-{:x}", digest.finalize()),
        rendered,
        rendered_bytes,
        truncated,
    })
}

/// System text of the compactor request (tools: `summary_artifact` only).
pub const SUMMARY_SYSTEM: &str = "You are the compaction step of a coding assistant. The user message is the frozen accepted history that is about to be compacted, one record per line as `seq <n> <kind> <json>`. Call summary_artifact exactly once with the minimal continuation a future assistant needs to carry this conversation on: keep every exact identifier, code, secret word, name, path, number and decision verbatim; drop chatter and tool mechanics. evidence_refs lists the seq numbers of the records the continuation is drawn from, using only seq numbers that appear in the history.";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SummaryArtifact {
    pub continuation: String,
    pub evidence_refs: Vec<u64>,
}

/// Evidence-address admission: every reference is a `seq` inside the frozen
/// bundle, unique and non-empty; the continuation is non-empty and bounded.
/// `Err` is the rejection reason recorded in telemetry.
pub fn admit_summary_artifact(
    bundle: &SourceBundle,
    artifact: &Value,
) -> Result<SummaryArtifact, String> {
    let object = artifact.as_object().ok_or("artifact is not an object")?;
    let continuation = object
        .get("continuation")
        .and_then(Value::as_str)
        .ok_or("continuation is missing")?
        .trim();
    if continuation.is_empty() {
        return Err("continuation is empty".to_owned());
    }
    if continuation.len() > MAX_CONTINUATION_BYTES {
        return Err(format!(
            "continuation exceeds {MAX_CONTINUATION_BYTES} bytes"
        ));
    }
    let refs = object
        .get("evidence_refs")
        .and_then(Value::as_array)
        .ok_or("evidence_refs is missing")?;
    if refs.is_empty() {
        return Err("evidence_refs is empty".to_owned());
    }
    let mut seen = BTreeSet::new();
    for reference in refs {
        let seq = reference
            .as_u64()
            .or_else(|| reference.get("seq").and_then(Value::as_u64))
            .ok_or_else(|| format!("evidence ref {reference} is not a seq address"))?;
        if !bundle.covers.contains(&seq) {
            return Err(format!(
                "evidence ref seq {seq} is outside the frozen bundle"
            ));
        }
        if !seen.insert(seq) {
            return Err(format!("evidence ref seq {seq} repeats"));
        }
    }
    Ok(SummaryArtifact {
        continuation: continuation.to_owned(),
        evidence_refs: seen.into_iter().collect(),
    })
}

/// The `compact.summary_request` telemetry object: the frozen bundle, the
/// admission verdict (evidence addresses or the rejection reason), the
/// request's usage and the model. The continuation itself is the compact
/// `summary`, never duplicated here.
pub fn summary_record(
    bundle: &SourceBundle,
    outcome: &Result<SummaryArtifact, String>,
    usage: Option<Value>,
    model: &str,
) -> Value {
    let mut record = json!({
        "bundle": {"covers": seq_ranges(&bundle.covers), "sha256": bundle.sha256, "bytes": bundle.rendered_bytes, "truncated": bundle.truncated},
        "accepted": outcome.is_ok(),
        "model": model,
    });
    match outcome {
        Ok(artifact) => record["evidence_refs"] = json!(artifact.evidence_refs),
        Err(reason) => record["reason"] = Value::String(reason.clone()),
    }
    if let Some(usage) = usage {
        record["usage"] = usage;
    }
    record
}

/// A record for a compaction whose summary request could not run or was
/// voided (no provider lease, provider failure, stale bundle).
pub fn summary_unavailable(bundle: &SourceBundle, reason: &str, model: &str) -> Value {
    summary_record(bundle, &Err(reason.to_owned()), None, model)
}

/// One summary result carried from the request to the compact write.
#[derive(Clone, Debug)]
pub struct CompactionSummary {
    pub model: String,
    pub bundle: SourceBundle,
    pub record: Value,
    /// The admitted continuation as the compact summary text; `None` when the
    /// artifact was rejected or the request failed.
    pub continuation: Option<String>,
}

impl CompactionSummary {
    pub fn from_outcome(
        model: &str,
        bundle: SourceBundle,
        outcome: Result<SummaryArtifact, String>,
        usage: Option<Value>,
    ) -> Self {
        let record = summary_record(&bundle, &outcome, usage, model);
        let continuation = outcome
            .ok()
            .map(|artifact| format!("[compacted history]\n{}", artifact.continuation));
        Self {
            model: model.to_owned(),
            bundle,
            record,
            continuation,
        }
    }

    /// The record and summary text for a compact whose plan covers `covers`:
    /// the admitted continuation when the bundle is still the plan, otherwise
    /// the deterministic quoted history with a stale-bundle record.
    pub fn apply(&self, covers: &[u64], deterministic: String) -> (Value, String) {
        if covers == self.bundle.covers.as_slice() {
            (
                self.record.clone(),
                self.continuation.clone().unwrap_or(deterministic),
            )
        } else {
            (
                summary_unavailable(
                    &self.bundle,
                    "stale bundle: the plan changed before the compact was written",
                    &self.model,
                ),
                deterministic,
            )
        }
    }
}

pub fn seq_ranges(seqs: &[u64]) -> Vec<Value> {
    let mut ranges: Vec<Value> = Vec::new();
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

#[cfg(test)]
mod tests {
    use super::*;

    fn event(seq: u64, kind: &str, extra: Value) -> Event {
        let mut object = json!({"v":1,"seq":seq,"kind":kind,"ts":"2026-09-05T00:00:00.000Z"});
        for (key, value) in extra.as_object().unwrap() {
            object[key] = value.clone();
        }
        let parsed = schema::IJsonValue::parse(&serde_json::to_vec(&object).unwrap()).unwrap();
        Event::from_value(parsed).unwrap()
    }

    fn history() -> Vec<Event> {
        vec![
            event(
                2,
                "input",
                json!({"content":[{"type":"text","text":"code AZURE-17"}],"origin_key":"i1","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i1"}}),
            ),
            event(
                3,
                "output",
                json!({"turn":1,"attempt":"a1","usage":{"availability":"unavailable"},"content":[{"type":"text","text":"OK"}],"sealed":{"version":1,"adapter":"fake","fragments":"calls"}}),
            ),
            event(
                4,
                "input",
                json!({"content":[{"type":"text","text":"code CORAL-42"}],"origin_key":"i2","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i2"}}),
            ),
        ]
    }

    #[test]
    fn bundle_is_frozen_over_exact_covers_and_digested() {
        let events = history();
        let a = freeze_source_bundle(&events, &[3, 2], 1 << 20).unwrap();
        let b = freeze_source_bundle(&events, &[2, 3], 1 << 20).unwrap();
        assert_eq!(a, b, "covers are a set");
        assert_eq!(a.covers, vec![2, 3]);
        assert!(a.rendered.contains("seq 2 input") && a.rendered.contains("AZURE-17"));
        assert!(
            !a.rendered.contains("CORAL-42"),
            "uncovered records are not rendered"
        );
        assert!(!a.truncated);
        let other = freeze_source_bundle(&events, &[2, 4], 1 << 20).unwrap();
        assert_ne!(a.sha256, other.sha256);
        assert!(
            freeze_source_bundle(&events, &[9], 1 << 20).is_err(),
            "missing covers are refused"
        );
        let small = freeze_source_bundle(&events, &[2, 3], 40).unwrap();
        assert!(small.truncated && small.rendered.ends_with("[bundle truncated]"));
        assert_eq!(
            small.sha256, a.sha256,
            "the digest is over the full bundle, not the rendering"
        );
    }

    #[test]
    fn admission_checks_evidence_addresses_against_the_bundle() {
        let bundle = freeze_source_bundle(&history(), &[2, 3], 1 << 20).unwrap();
        let ok = admit_summary_artifact(
            &bundle,
            &json!({"continuation":"User gave code AZURE-17.","evidence_refs":[3,2]}),
        )
        .unwrap();
        assert_eq!(ok.evidence_refs, vec![2, 3]);
        let objects = admit_summary_artifact(
            &bundle,
            &json!({"continuation":"x","evidence_refs":[{"seq":2}]}),
        )
        .unwrap();
        assert_eq!(objects.evidence_refs, vec![2]);
        for (bad, needle) in [
            (
                json!({"continuation":"x","evidence_refs":[4]}),
                "outside the frozen bundle",
            ),
            (json!({"continuation":"x","evidence_refs":[2,2]}), "repeats"),
            (json!({"continuation":"x","evidence_refs":[]}), "empty"),
            (
                json!({"continuation":"  ","evidence_refs":[2]}),
                "continuation is empty",
            ),
            (
                json!({"continuation":"x","evidence_refs":["2"]}),
                "not a seq address",
            ),
            (json!({"continuation":"x"}), "evidence_refs is missing"),
            (
                json!({"continuation":"y".repeat(MAX_CONTINUATION_BYTES+1),"evidence_refs":[2]}),
                "exceeds",
            ),
        ] {
            let reason = admit_summary_artifact(&bundle, &bad).unwrap_err();
            assert!(reason.contains(needle), "{bad}: {reason}");
        }
    }

    #[test]
    fn a_stale_bundle_voids_the_artifact() {
        let bundle = freeze_source_bundle(&history(), &[2, 3], 1 << 20).unwrap();
        let accepted = admit_summary_artifact(
            &bundle,
            &json!({"continuation":"AZURE-17","evidence_refs":[2]}),
        );
        let summary = CompactionSummary::from_outcome("m", bundle, accepted, None);
        let (record, text) = summary.apply(&[2, 3], "[compacted history]\ndet".to_owned());
        assert_eq!(record["accepted"], true);
        assert_eq!(text, "[compacted history]\nAZURE-17");
        let (record, text) = summary.apply(&[2, 3, 4], "[compacted history]\ndet".to_owned());
        assert_eq!(record["accepted"], false);
        assert!(
            record["reason"]
                .as_str()
                .unwrap()
                .starts_with("stale bundle")
        );
        assert_eq!(text, "[compacted history]\ndet");
    }

    #[test]
    fn the_record_carries_the_verdict_and_the_summary_is_the_admitted_continuation() {
        let bundle = freeze_source_bundle(&history(), &[2, 3], 1 << 20).unwrap();
        let accepted = admit_summary_artifact(
            &bundle,
            &json!({"continuation":"AZURE-17","evidence_refs":[2]}),
        );
        let rejected = admit_summary_artifact(
            &bundle,
            &json!({"continuation":"AZURE-17","evidence_refs":[7]}),
        );
        let record = summary_record(&bundle, &rejected, Some(json!({"input_tokens":"10"})), "m");
        assert_eq!(record["accepted"], false);
        assert!(record["reason"].as_str().unwrap().contains("outside"));
        assert_eq!(record["bundle"]["covers"], json!([{"from":2,"to":3}]));
        assert_eq!(record["usage"]["input_tokens"], "10");
        let record = summary_record(&bundle, &accepted, None, "m");
        assert_eq!(record["evidence_refs"], json!([2]));
        assert!(
            record.get("continuation").is_none(),
            "the continuation lives in compact.summary only"
        );
        let fallback = CompactionSummary::from_outcome("m", bundle.clone(), rejected, None);
        assert_eq!(
            fallback.apply(&[2, 3], "det".to_owned()).1,
            "det",
            "rejection falls back to the deterministic summary"
        );
        let promoted = CompactionSummary::from_outcome("m", bundle, accepted, None);
        assert_eq!(
            promoted.continuation.as_deref(),
            Some("[compacted history]\nAZURE-17")
        );
    }
}
