use std::path::Path;

use serde_json::Value;
use uuid::Uuid;

use crate::error::SelectorError;
use crate::fs::read_regular;

pub(crate) fn verify_canary_ledger(
    storage_root: &Path,
    version: &str,
    session: &str,
    run: &str,
) -> Result<(), SelectorError> {
    let uuid = Uuid::parse_str(session)
        .map_err(|_| SelectorError::invalid_state("canary-session-invalid"))?;
    if uuid.hyphenated().to_string() != session {
        return Err(SelectorError::invalid_state("canary-session-invalid"));
    }
    let path = storage_root.join(session).join("main.jsonl");
    let bytes =
        read_regular(&path).map_err(|_| SelectorError::invalid_state("canary-ledger-missing"))?;
    if !bytes.ends_with(b"\n") {
        return Err(SelectorError::invalid_state("canary-ledger-invalid"));
    }
    let mut last_seq = 0_u64;
    let mut genesis_ok = false;
    let mut matching_runs = 0_u64;
    let mut matching_binary = None;
    let mut settled_after = false;
    let mut active_run = None;
    for line in bytes.split_inclusive(|byte| *byte == b'\n') {
        let body = line
            .strip_suffix(b"\n")
            .ok_or_else(|| SelectorError::invalid_state("canary-ledger-invalid"))?;
        let value: Value = serde_json::from_slice(body)
            .map_err(|_| SelectorError::invalid_state("canary-ledger-invalid"))?;
        if serde_json_canonicalizer::to_vec(&value)
            .map_err(|_| SelectorError::invalid_state("canary-ledger-invalid"))?
            != body
        {
            return Err(SelectorError::invalid_state("canary-ledger-invalid"));
        }
        let seq = value
            .get("seq")
            .and_then(Value::as_u64)
            .ok_or_else(|| SelectorError::invalid_state("canary-ledger-invalid"))?;
        if seq != last_seq + 1 {
            return Err(SelectorError::invalid_state("canary-ledger-invalid"));
        }
        last_seq = seq;
        match value.get("kind").and_then(Value::as_str) {
            Some("genesis") if seq == 1 => {
                genesis_ok = value.get("thread").and_then(Value::as_str) == Some(session);
            }
            Some("run_start") => {
                active_run = value.get("run").and_then(Value::as_str).map(str::to_owned);
                if active_run.as_deref() == Some(run) {
                    matching_runs += 1;
                    matching_binary = value
                        .get("binary")
                        .and_then(Value::as_str)
                        .map(str::to_owned);
                    settled_after = false;
                }
            }
            Some("settle") if matching_runs == 1 && active_run.as_deref() == Some(run) => {
                settled_after = true;
            }
            _ => {}
        }
    }
    let binary_matches = matching_binary
        .as_deref()
        .is_some_and(|binary| binary == version || binary.ends_with(&format!("-{version}")));
    if !genesis_ok || matching_runs != 1 || !settled_after || !binary_matches {
        return Err(SelectorError::invalid_state("canary-attribution-failed"));
    }
    Ok(())
}
