//! Non-activating, read-only bounded child history. Never opens a writable ledger or journal.
use crate::client_extensions::failure;
use crate::endpoint_host::ProductionRouteFailure;
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path};
const MAX_LEDGER: u64 = 32 * 1024 * 1024;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Request {
    parent_session_id: String,
    child_session_id: String,
    through_sequence: Option<i64>,
    before_sequence: Option<i64>,
    max_messages: usize,
    history_revision: Option<String>,
}
fn request(value: &Value) -> Result<Request, String> {
    let request: Request = serde_json::from_value(value.clone())
        .map_err(|_| "invalid child history request".to_owned())?;
    endpoint::validate_session_id(&request.parent_session_id)
        .map_err(|_| "invalid parent identity")?;
    endpoint::validate_session_id(&request.child_session_id)
        .map_err(|_| "invalid child identity")?;
    if request.parent_session_id == request.child_session_id
        || request.max_messages == 0
        || request.max_messages > 200
        || request.through_sequence.is_some_and(|s| s < -1)
        || request.before_sequence.is_some_and(|s| s < 0)
        || request.before_sequence.is_some() && request.through_sequence.is_none()
        || request.history_revision.is_some() && request.through_sequence.is_none()
    {
        return Err("invalid child history range".into());
    }
    Ok(request)
}
pub(crate) fn validate(value: &Value) -> Result<(), String> {
    request(value).map(|_| ())
}
fn corrupt(message: &str) -> ProductionRouteFailure {
    failure(
        "ledger-corrupt",
        "Child history is unavailable",
        json!({"reason":message}),
    )
}
fn bytes(path: &Path) -> Result<Vec<u8>, ProductionRouteFailure> {
    let metadata = fs::symlink_metadata(path).map_err(|_| corrupt("ledger unreadable"))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > MAX_LEDGER {
        return Err(failure(
            "history-limit",
            "Invalid or oversized child ledger",
            json!({}),
        ));
    }
    // O_NOFOLLOW closes a replacement/symlink race after metadata validation.
    use std::os::unix::fs::OpenOptionsExt;
    let file = fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(|_| corrupt("ledger unreadable"))?;
    let mut bytes = Vec::new();
    file.take(MAX_LEDGER + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| corrupt("ledger unreadable"))?;
    if bytes.len() as u64 > MAX_LEDGER {
        return Err(failure(
            "history-limit",
            "Child ledger exceeds limit",
            json!({}),
        ));
    }
    // A live writer can be between writes. Read only the complete durable prefix, without repair.
    let end = bytes.iter().rposition(|b| *b == b'\n').map_or(0, |i| i + 1);
    bytes.truncate(end);
    Ok(bytes)
}
pub(crate) fn read(folder: &Path, payload: &Value) -> Result<Value, ProductionRouteFailure> {
    let request =
        request(payload).map_err(|message| failure("bad-request", &message, json!({})))?;
    if fs::symlink_metadata(folder)
        .map_err(|_| corrupt("session folder unavailable"))?
        .file_type()
        .is_symlink()
    {
        return Err(corrupt("session folder is redirected"));
    }
    let parent =
        schema::validate_ledger(&bytes(&folder.join("main.jsonl"))?, schema::READER_VERSION)
            .map_err(|_| corrupt("parent ledger invalid"))?;
    if parent.events.first().and_then(|e| e.string_field("thread"))
        != Some(&request.parent_session_id)
    {
        return Err(corrupt("parent identity mismatch"));
    }
    let child_file = format!("{}.jsonl", request.child_session_id);
    let spawn = parent
        .events
        .iter()
        .find(|e| {
            *e.kind() == schema::EventKind::Spawn
                && e.string_field("child") == Some(child_file.as_str())
        })
        .ok_or_else(|| {
            failure(
                "child-unavailable",
                "Child is not owned by this parent",
                json!({}),
            )
        })?;
    let child = schema::validate_ledger(&bytes(&folder.join(&child_file))?, schema::READER_VERSION)
        .map_err(|_| corrupt("child ledger invalid"))?;
    let genesis = child
        .events
        .first()
        .ok_or_else(|| corrupt("child genesis missing"))?;
    let raw = serde_json::to_value(genesis.raw()).map_err(|_| corrupt("child genesis invalid"))?;
    if genesis.string_field("thread") != Some(request.child_session_id.as_str())
        || raw["parent"]["file"] != "main.jsonl"
        || raw["parent"]["seq"].as_u64() != Some(spawn.seq())
        || raw["parent"]["spawn_id"].as_str() != spawn.string_field("spawn_id")
    {
        return Err(failure(
            "child-unavailable",
            "Child parent binding does not match",
            json!({}),
        ));
    }
    let records = endpoint::Projector::read_only_history(&child.events, folder)
        .map_err(|_| corrupt("child projection failed"))?;
    let latest = records.last().map_or(-1, |r| r.event.seq as i64);
    let cut = request.through_sequence.unwrap_or(latest);
    if cut > latest
        || request
            .before_sequence
            .is_some_and(|b| b > cut.saturating_add(1))
    {
        return Err(failure(
            "history-stale",
            "Child history cut is unavailable",
            json!({}),
        ));
    }
    if cut >= 0
        && records
            .get(cut as usize)
            .zip(records.get(cut as usize + 1))
            .is_some_and(|(a, b)| a.kernel_seqs.iter().any(|seq| b.kernel_seqs.contains(seq)))
    {
        return Err(failure(
            "history-stale",
            "Child history cut splits one causal event",
            json!({}),
        ));
    }
    let prefix: Vec<_> = records
        .into_iter()
        .filter(|r| r.event.seq as i64 <= cut)
        .collect();
    let mut hasher = Sha256::new();
    hasher.update(
        genesis
            .canonical_bytes()
            .map_err(|_| corrupt("child genesis invalid"))?,
    );
    for row in &prefix {
        hasher.update(
            row.canonical_bytes()
                .map_err(|_| corrupt("child projection invalid"))?,
        );
    }
    let revision = format!("{:x}", hasher.finalize());
    if request
        .history_revision
        .as_ref()
        .is_some_and(|r| r != &revision)
    {
        return Err(failure(
            "history-stale",
            "Child history prefix changed",
            json!({}),
        ));
    }
    let page = endpoint::history_page(
        &prefix,
        request.before_sequence.map(|s| s as u64),
        Some(request.max_messages),
    )
    .map_err(|_| corrupt("invalid child page"))?;
    let mut run_open = !child
        .events
        .iter()
        .any(|e| matches!(e.kind(), schema::EventKind::Settle));
    for row in &prefix {
        match row.event.event_type.as_str() {
            "turn/start" => run_open = true,
            "turn/end" => run_open = false,
            _ => (),
        }
    }
    let call = spawn.string_field("call").unwrap_or_default();
    let source = parent
        .events
        .iter()
        .find(|e| *e.kind() == schema::EventKind::ToolCall && e.string_field("call") == Some(call))
        .map(|e| serde_json::to_value(e.raw()).unwrap_or(Value::Null));
    let effective = parent
        .events
        .iter()
        .filter_map(|e| serde_json::to_value(e.raw()).ok())
        .find(|v| {
            v["kind"] == "state" && v["subkind"] == "delegation" && v["payload"]["call"] == call
        });
    let source=source.map(|v| {
        let (name, arguments)=effective.as_ref().map_or((&v["name"],&v["args"]),|v|(&v["payload"]["name"],&v["payload"]["invocation"]));
        let outcome=child.events.iter().rev().find(|e| matches!(e.kind(),schema::EventKind::Settle)).and_then(|e|e.string_field("outcome"));
        json!({"callId":call,"name":name,"arguments":arguments,"phase":match outcome {Some("completed")=>"done",Some("interrupted")=>"stopped",Some(_)=>"error",None=>"running"}})
    });
    let value = json!({"parentSessionId":request.parent_session_id,"childSessionId":request.child_session_id,"throughSequence":cut,"historyRevision":revision,"entries":page.events,"hasMoreBefore":page.has_more,"runOpen":run_open,"sourceCall":source});
    if serde_json::to_vec(&value)
        .map_err(|_| corrupt("child response invalid"))?
        .len()
        > 4 * 1024 * 1024
    {
        return Err(failure(
            "history-limit",
            "Child page exceeds limit",
            json!({}),
        ));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    const PARENT: &str = "018f0000-0000-7000-8000-000000000003";
    const CHILD: &str = "018f0000-0000-7000-8000-000000000004";
    fn fixture() -> tempfile::TempDir {
        let root = tempfile::tempdir().unwrap();
        let corpus = test_support::FixtureRoot::discover().unwrap();
        let parent = fs::read_to_string(corpus.join("endpoint/projection-source.jsonl")).unwrap();
        let mut events: Vec<Value> = parent
            .lines()
            .take(6)
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        let time = events[0]["ts"].clone();
        events.push(json!({"v":1,"seq":7,"ts":time,"turn":1,"kind":"tool_call","attempt":"a1","call":"c1","source":"provider","name":"task","args":{"task_name":"audit"}}));
        events.push(json!({"v":1,"seq":8,"ts":time,"turn":1,"kind":"state","subkind":"delegation","payload":{"call":"c1","name":"task","invocation":{"task_name":"effective audit"}}}));
        events.push(json!({"v":1,"seq":9,"ts":time,"turn":1,"kind":"spawn","call":"c1","spawn_id":"s1","child":format!("{CHILD}.jsonl"),"resume":"never","seed":{"kinds":["state"]}}));
        let text = events
            .into_iter()
            .map(|v| v.to_string() + "\n")
            .collect::<String>();
        schema::validate_ledger(text.as_bytes(), schema::READER_VERSION).unwrap();
        fs::write(root.path().join("main.jsonl"), text).unwrap();
        let child = fs::read_to_string(corpus.join("replay/cancel-before-start.jsonl")).unwrap();
        let mut child: Vec<Value> = child
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        child[0]["parent"]["seq"] = json!(9);
        let text = child
            .into_iter()
            .map(|v| v.to_string() + "\n")
            .collect::<String>();
        schema::validate_ledger(text.as_bytes(), schema::READER_VERSION).unwrap();
        fs::write(root.path().join(format!("{CHILD}.jsonl")), text).unwrap();
        root
    }
    #[test]
    fn owned_child_read_is_frozen_and_creates_no_files() {
        let root = fixture();
        let before = fs::read_dir(root.path()).unwrap().count();
        let request = json!({"parentSessionId":PARENT,"childSessionId":CHILD,"maxMessages":1});
        let page = read(root.path(), &request).unwrap();
        assert_eq!(
            page["sourceCall"]["arguments"]["task_name"],
            "effective audit"
        );
        assert_eq!(page["sourceCall"]["phase"], "stopped");
        assert!(
            page["entries"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v["event"]["type"] == "turn/end")
        );
        assert_eq!(fs::read_dir(root.path()).unwrap().count(), before);
        let mut earlier = request.clone();
        earlier["throughSequence"] = page["throughSequence"].clone();
        earlier["historyRevision"] = page["historyRevision"].clone();
        assert!(read(root.path(), &earlier).is_ok());
        earlier["historyRevision"] = json!("0".repeat(64));
        assert_eq!(
            read(root.path(), &earlier).unwrap_err().code,
            "history-stale"
        );
        earlier["historyRevision"] = page["historyRevision"].clone();
        earlier["throughSequence"] = json!(1000);
        assert_eq!(
            read(root.path(), &earlier).unwrap_err().code,
            "history-stale"
        );
    }
    #[test]
    fn ownership_binding_partial_tails_and_symlinks_fail_closed() {
        let root = fixture();
        let request = json!({"parentSessionId":PARENT,"childSessionId":CHILD,"maxMessages":50});
        let page = read(root.path(), &request).unwrap();
        let file = root.path().join(format!("{CHILD}.jsonl"));
        let original = fs::read(&file).unwrap();
        let mut torn = original.clone();
        torn.extend(b"{\"partial\":");
        fs::write(&file, &torn).unwrap();
        assert_eq!(
            read(root.path(), &request).unwrap()["historyRevision"],
            page["historyRevision"]
        );
        fs::write(&file, &original).unwrap();
        let mut lines: Vec<Value> = String::from_utf8(original)
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        lines[0]["parent"]["spawn_id"] = json!("unrelated");
        fs::write(
            &file,
            lines
                .into_iter()
                .map(|v| v.to_string() + "\n")
                .collect::<String>(),
        )
        .unwrap();
        assert_eq!(
            read(root.path(), &request).unwrap_err().code,
            "child-unavailable"
        );
        fs::remove_file(&file).unwrap();
        std::os::unix::fs::symlink("main.jsonl", &file).unwrap();
        assert!(read(root.path(), &request).is_err());
        let mut bad = request;
        bad["childSessionId"] = json!("../main");
        assert!(validate(&bad).is_err());
    }
}
