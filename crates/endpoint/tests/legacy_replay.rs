//! Kernel import and replay of the legacy AppServer SQLite corpus.
//!
//! Port of the pinned `ValidationLoopReplayLiveTests.legacy_production_database_replays_under_the_loop`:
//! `scripts/replay-legacy-sqlite.py` proves the schema, lineage grammar and
//! reserved-candidate invariants on a read-only copy of a production
//! `appserver.sqlite3` and exports every workspace's root threads (twelve per
//! workspace, the pinned bound) as neutral turn transcripts. This test imports
//! each transcript as a Kernel ledger (genesis, input, turn_open, epoch,
//! attempt, tool_call/tool_result, usage, output, settle), validates it under
//! the Kernel schema, projects it through the endpoint journal, pages the
//! history, and proves the projection is idempotent — the Kernel equivalents
//! of the legacy `thread/list` + `thread/read` projection pass.
//!
//! Gated on `TEKES_LEGACY_REPLAY_EXPORT` (the exporter's output directory);
//! live fixtures are machine-local. Writes `kernel-replay-receipt.json` there.
use std::fs;
use std::path::PathBuf;

use endpoint::{EndpointJournal, Projector, history_page};
use schema::validate_ledger;
use serde_json::{Value, json};
use store::AssetStore;

#[test]
#[ignore = "needs TEKES_LEGACY_REPLAY_EXPORT from scripts/replay-legacy-sqlite.py"]
fn legacy_corpus_export_imports_and_projects_through_kernel() {
    let export = PathBuf::from(
        std::env::var("TEKES_LEGACY_REPLAY_EXPORT").expect("TEKES_LEGACY_REPLAY_EXPORT"),
    );
    let report: Value =
        serde_json::from_slice(&fs::read(export.join("report.json")).expect("report")).unwrap();
    assert_eq!(report["integrity"], "ok");
    assert!(
        !report["repair_kinds"].as_object().unwrap().is_empty(),
        "historical repair lineage"
    );
    assert!(
        report["reserved_candidates"]
            .as_object()
            .unwrap()
            .values()
            .all(|n| n == 0)
    );
    let mut receipts = Vec::new();
    for entry in report["threads"].as_array().expect("threads") {
        let session = entry["session"].as_str().unwrap();
        let thread: Value = serde_json::from_slice(
            &fs::read(export.join("threads").join(format!("{session}.json"))).expect("thread"),
        )
        .unwrap();
        let folder = tempfile::tempdir().expect("thread folder");
        let assets = AssetStore::new(folder.path().join("assets")).expect("assets");
        let (events, expected) = import_thread(&thread, &assets);
        let mut bytes = Vec::new();
        for event in &events {
            bytes.extend_from_slice(&serde_json_canonicalizer::to_vec(event).unwrap());
            bytes.push(b'\n');
        }
        fs::write(folder.path().join("main.jsonl"), &bytes).unwrap();
        let projection = validate_ledger(&bytes, 1)
            .unwrap_or_else(|error| panic!("{session}: imported ledger invalid: {error}"));
        assert_eq!(
            projection.events.len(),
            events.len(),
            "{session}: every imported event projects"
        );
        let journal = EndpointJournal::open(folder.path()).expect("journal");
        Projector::default()
            .reconcile(&projection.events, &journal)
            .unwrap_or_else(|error| panic!("{session}: projection failed: {error}"));
        let first = fs::read(journal.path()).unwrap();
        Projector::default()
            .reconcile(&projection.events, &journal)
            .unwrap();
        assert_eq!(
            fs::read(journal.path()).unwrap(),
            first,
            "{session}: projection is idempotent"
        );
        let records = journal.records().expect("records");
        let page = history_page(&records, None, Some(10_000)).expect("history page");
        let count = |kind: &str| {
            page.events
                .iter()
                .filter(|entry| entry.event.event_type == kind)
                .count()
        };
        let observed = json!({
            "user/message": count("user/message"),
            "assistant/message": count("assistant/message"),
            "turn/start": count("turn/start"),
            "turn/end": count("turn/end"),
            "tool/call": count("tool/call"),
        });
        assert_eq!(
            observed["user/message"], expected["inputs"],
            "{session}: every legacy input projects as a user message"
        );
        assert_eq!(
            observed["turn/start"], expected["turns"],
            "{session}: every legacy turn opens"
        );
        assert_eq!(
            observed["turn/end"], expected["settled"],
            "{session}: every settled legacy turn ends"
        );
        assert!(
            observed["assistant/message"].as_u64().unwrap() >= expected["finals"].as_u64().unwrap(),
            "{session}: legacy final answers project as assistant messages"
        );
        receipts.push(json!({"session":session,"events":events.len(),"journal_records":records.len(),"expected":expected,"observed":observed}));
    }
    assert!(!receipts.is_empty());
    fs::write(
        export.join("kernel-replay-receipt.json"),
        serde_json::to_vec_pretty(&json!({"status":"passed","threads":receipts})).unwrap(),
    )
    .unwrap();
}

/// Build the Kernel ledger for one exported legacy thread. Legacy turns that
/// never finalized are settled `interrupted` (they were stopped or abandoned
/// in the legacy runtime) unless they are the thread's last turn, which stays
/// open so the running tail projects as well.
fn import_thread(thread: &Value, assets: &AssetStore) -> (Vec<Value>, Value) {
    let session = thread["session"].as_str().unwrap().to_ascii_lowercase();
    let mut events = Vec::new();
    let mut seq = 0_u64;
    let mut next = |events: &mut Vec<Value>, mut value: Value| {
        seq += 1;
        value["v"] = json!(1);
        value["seq"] = json!(seq);
        value["ts"] = json!(format!(
            "2026-09-05T00:00:{:02}.{:03}Z",
            (seq / 1000) % 60,
            seq % 1000
        ));
        events.push(value);
        seq
    };
    let origin = |op: &str, key: String| json!({"principal":"legacy-import","client":"replay","target":session,"op":op,"key":key});
    next(
        &mut events,
        json!({"kind":"genesis","format":1,"min_reader":1,"min_writer":1,"thread":session,"workspace":"legacy",
        "origin_key":format!("create-{session}"),"origin_tuple":origin("create",format!("create-{session}")),"resume":"never","config":{"digest":"legacy-import"}}),
    );
    let system = assets
        .publish(b"legacy system prompt")
        .expect("system asset");
    let mut epoch_id = None;
    let turns = thread["turns"].as_array().unwrap();
    let (mut inputs, mut finals, mut settled) = (0, 0, 0);
    for (index, turn) in turns.iter().enumerate() {
        let number = index as u64 + 1;
        let text = turn["inputs"][0].as_str().unwrap_or("").to_owned();
        let text = if text.trim().is_empty() {
            "(empty legacy input)".to_owned()
        } else {
            text
        };
        let input_seq = next(
            &mut events,
            json!({"kind":"input","content":[{"type":"text","text":text}],
            "origin_key":format!("input-{session}-{number}"),"origin_tuple":origin("input",format!("input-{session}-{number}"))}),
        );
        inputs += 1;
        next(
            &mut events,
            json!({"kind":"turn_open","turn":number,"trigger":{"inputs":[input_seq]}}),
        );
        let is_last = index + 1 == turns.len();
        let settlement = turn["settlement"].as_str().unwrap_or("open");
        if settlement == "open" && is_last {
            break;
        }
        if epoch_id.is_none() {
            let id = format!("legacy-epoch-{session}");
            next(
                &mut events,
                json!({"kind":"epoch","id":id,"reason":"initial","adapter":"openai_responses_v1","model":"legacy",
                "system":{"asset":system.asset,"digest":"legacy"},"tools":{"asset":"sha256-legacy","digest":"legacy"},"renderer":1}),
            );
            epoch_id = Some(id);
        }
        let epoch = epoch_id.clone().unwrap();
        let tools = turn["tools"].as_array().cloned().unwrap_or_default();
        let mut attempt_index = 0;
        if !tools.is_empty() {
            attempt_index += 1;
            let attempt = format!("legacy-{number}-{attempt_index}");
            next(
                &mut events,
                json!({"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"turn":number,"attempt":attempt,"epoch":epoch,"wire_digest":"legacy","admits":[{"from":input_seq,"to":input_seq}]}),
            );
            let calls = tools.iter().enumerate().map(|(i, tool)| {
                let call = format!("legacy-call-{number}-{i}");
                let args = tool["arguments"].as_str().and_then(|raw| serde_json::from_str::<Value>(raw).ok()).unwrap_or_else(|| json!({"legacy_arguments": tool["arguments"]}));
                next(&mut events, json!({"kind":"tool_call","turn":number,"attempt":attempt,"call":call,"name":tool["name"],"args":args,"source":"provider"}));
                (call, tool["outcome"].as_str().unwrap_or("ok").to_owned(), tool["summary"].as_str().unwrap_or("").to_owned())
            }).collect::<Vec<_>>();
            next(
                &mut events,
                json!({"kind":"output","turn":number,"attempt":attempt,"content":[],"sealed":{"version":1,"adapter":"openai_responses_v1","fragments":"[]"},"usage":{"availability":"unavailable"}}),
            );
            for (call, outcome, summary) in calls {
                next(
                    &mut events,
                    json!({"kind":"tool_result","turn":number,"call":call,"outcome":outcome,"content":[{"type":"text","text":summary}]}),
                );
            }
        }
        let final_texts = turn["final"].as_array().cloned().unwrap_or_default();
        if !final_texts.is_empty() {
            attempt_index += 1;
            let attempt = format!("legacy-{number}-{attempt_index}");
            let admits = if attempt_index == 1 {
                json!([{"from":input_seq,"to":input_seq}])
            } else {
                json!([])
            };
            next(
                &mut events,
                json!({"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"turn":number,"attempt":attempt,"epoch":epoch,"wire_digest":"legacy","admits":admits}),
            );
            let text = final_texts
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join("\n");
            next(
                &mut events,
                json!({"kind":"output","turn":number,"attempt":attempt,"final_answer":true,"content":[{"type":"text","text":text}],"sealed":{"version":1,"adapter":"openai_responses_v1","fragments":"[]"},"usage":{"availability":"unavailable"}}),
            );
            finals += 1;
        }
        let settle = match settlement {
            "completed" if !final_texts.is_empty() => {
                json!({"kind":"settle","turn":number,"outcome":"completed"})
            }
            _ => {
                json!({"kind":"settle","turn":number,"outcome":"interrupted","reason":"user_stop"})
            }
        };
        next(&mut events, settle);
        settled += 1;
    }
    (
        events,
        json!({"turns":turns.len(),"inputs":inputs,"finals":finals,"settled":settled}),
    )
}
