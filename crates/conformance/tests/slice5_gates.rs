use std::fs;

use schema::{Event, EventKind, OriginTuple, validate_ledger};
use serde_json::json;
use store::{AssetStore, RewriteKind, RewriteOperation, RewritePhase, ThreadStore};
use test_support::{FixtureRoot, read};

const SOURCE: &str = "018f0000-0000-7000-8000-000000000003";
const SOURCE_TWO: &str = "018f0000-0000-7000-8000-000000000004";
const FORK_DEST: &str = "018f0000-0000-7000-8000-000000000010";
const REDACT_DEST: &str = "018f0000-0000-7000-8000-000000000011";
const CHILD: &str = "018f0000-0000-7000-8000-000000000099";
const TS: &str = "2026-08-27T09:00:00.000Z";

#[test]
fn slice5_gate_15_redact_rewrite_closure() {
    let fixtures = FixtureRoot::discover().expect("fixtures");
    for name in [
        "op-fork-prepared.canonical.json",
        "op-redact-published.canonical.json",
    ] {
        let bytes = read(&fixtures.join("rewrite").join(name)).expect("operation fixture");
        let operation = RewriteOperation::decode_canonical(&bytes).expect("operation decode");
        assert_eq!(
            operation.canonical_bytes().expect("operation encode"),
            bytes
        );
    }

    let root = tempfile::tempdir().expect("storage root");
    let store = ThreadStore::open(root.path()).expect("store");
    let (_poisoned, safe) = prepare_source(&store, SOURCE, b"SECRET");
    let operation = store
        .begin_redact(
            "redact-closure",
            SOURCE,
            REDACT_DEST,
            TS,
            &[b"SECRET".to_vec()],
        )
        .expect("begin redact");
    assert_eq!(operation.kind, RewriteKind::Redact);
    let record =
        fs::read(root.path().join("staging/redact-closure/op.json")).expect("operation record");
    assert!(!record.windows(6).any(|window| window == b"SECRET"));
    store.recover_rewrites().expect("finish redact");

    assert!(!root.path().join("threads").join(SOURCE).exists());
    let destination = root.path().join("threads").join(REDACT_DEST);
    let bytes = fs::read(destination.join("main.jsonl")).expect("redacted ledger");
    let projection = validate_ledger(&bytes, 1).expect("redacted ledger validates");
    assert!(!bytes.windows(6).any(|window| window == b"SECRET"));
    assert!(!bytes.windows(8).any(|window| window == b"\"sealed\""));
    assert!(projection.events.iter().all(|event| {
        !matches!(
            event.kind(),
            EventKind::Attempt
                | EventKind::AttemptDispatched
                | EventKind::AttemptRecovery
                | EventKind::Checkpoint
                | EventKind::Compact
        )
    }));
    assert!(projection.events.iter().any(|event| {
        matches!(event.kind(), EventKind::Epoch) && event.string_field("reason") == Some("redact")
    }));
    let destination_text = String::from_utf8(bytes.clone()).expect("destination UTF-8");
    assert!(destination_text.contains(r#""title":"source title""#));
    assert!(destination_text.contains(r#""labels":["one","two"]"#));
    assert!(destination.join("assets").join(&safe).is_file());
    assert_eq!(
        fs::read(destination.join("assets").join(safe)).expect("safe asset"),
        b"safe\n"
    );
    let copied_assets = fs::read_dir(destination.join("assets"))
        .expect("destination assets")
        .filter_map(Result::ok)
        .map(|entry| fs::read(entry.path()).expect("copied asset"))
        .collect::<Vec<_>>();
    assert!(copied_assets.iter().any(|bytes| bytes == b"config\n"));
    assert!(copied_assets.iter().any(|bytes| bytes == b"instruction\n"));
    let child_files = fs::read_dir(&destination)
        .expect("destination files")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().extension().and_then(|value| value.to_str()) == Some("jsonl")
                && entry.file_name() != "main.jsonl"
        })
        .collect::<Vec<_>>();
    assert_eq!(child_files.len(), 1);
    let child = validate_ledger(
        &fs::read(child_files[0].path()).expect("rewritten child ledger"),
        1,
    )
    .expect("rewritten child validates");
    assert!(child.events[0].has_field("parent"));
    let child_genesis: serde_json::Value = serde_json::from_slice(
        &child.events[0]
            .canonical_bytes()
            .expect("rewritten child genesis"),
    )
    .expect("child genesis JSON");
    let seed_asset = child_genesis["seed"]["snapshot"]["asset"]
        .as_str()
        .expect("seed snapshot retained");
    assert!(destination.join("assets").join(seed_asset).is_file());

    prepare_source(&store, SOURCE_TWO, b"SECRET");
    store
        .begin_fork("fork-audit", SOURCE_TWO, FORK_DEST, TS)
        .expect("begin fork");
    store.recover_rewrites().expect("finish fork");
    assert!(root.path().join("threads").join(SOURCE_TWO).is_dir());
    assert!(root.path().join("threads").join(FORK_DEST).is_dir());
}

#[test]
fn slice5_gate_16_rewrite_publication_crash_matrix() {
    for kind in [RewriteKind::Fork, RewriteKind::Redact] {
        let transition_count = if kind == RewriteKind::Fork { 4 } else { 7 };
        for crash_after in 0..transition_count {
            let root = tempfile::tempdir().expect("storage root");
            let store = ThreadStore::open(root.path()).expect("store");
            prepare_source(&store, SOURCE, b"SECRET");
            let destination = if kind == RewriteKind::Fork {
                FORK_DEST
            } else {
                REDACT_DEST
            };
            let operation_id = format!("{:?}-{crash_after}", kind).to_ascii_lowercase();
            match kind {
                RewriteKind::Fork => store
                    .begin_fork(&operation_id, SOURCE, destination, TS)
                    .expect("begin fork"),
                RewriteKind::Redact => store
                    .begin_redact(
                        &operation_id,
                        SOURCE,
                        destination,
                        TS,
                        &[b"SECRET".to_vec()],
                    )
                    .expect("begin redact"),
            };
            for _ in 0..crash_after {
                let progress = store
                    .advance_rewrite(&operation_id)
                    .expect("advance before crash");
                if progress.phase.is_none() {
                    break;
                }
            }
            drop(store);
            let recovered = ThreadStore::open(root.path()).expect("reopen");
            recovered.recover_rewrites().expect("recover operation");
            assert!(root.path().join("threads").join(destination).is_dir());
            assert_eq!(
                root.path().join("threads").join(SOURCE).is_dir(),
                kind == RewriteKind::Fork
            );
            assert!(!root.path().join("staging").join(operation_id).exists());
            assert_eq!(
                fs::read_dir(root.path().join(".rewrite-trash"))
                    .expect("trash")
                    .count(),
                0
            );
        }
    }

    let root = tempfile::tempdir().expect("publish action gap root");
    let store = ThreadStore::open(root.path()).expect("store");
    prepare_source(&store, SOURCE, b"SECRET");
    store
        .begin_fork("publish-gap", SOURCE, FORK_DEST, TS)
        .expect("begin fork");
    assert!(matches!(
        store.archive(SOURCE),
        Err(store::StoreError::RewriteInProgress)
    ));
    let delivery_origin = OriginTuple {
        principal: "p".to_owned(),
        client: "cli".to_owned(),
        target: SOURCE.to_owned(),
        op: "submit".to_owned(),
        key: "blocked-during-rewrite".to_owned(),
    };
    let event_origin = delivery_origin.clone();
    let event_key = event_origin.key.clone();
    assert!(matches!(
        store.append_keyed(SOURCE, &delivery_origin, move |seq| {
            let value = json!({
                "v":1,"seq":seq,"kind":"input","ts":TS,
                "origin_key":event_key,
                "origin_tuple":event_origin,
                "content":[{"type":"text","text":"must reject"}]
            });
            Event::decode_canonical(
                &serde_json_canonicalizer::to_vec(&value)
                    .map_err(|error| store::StoreError::Corruption(error.to_string()))?,
            )
            .map_err(store::StoreError::from)
        }),
        Err(store::StoreError::RewriteInProgress)
    ));
    prepare_source(&store, SOURCE_TWO, b"SECRET");
    assert!(matches!(
        store.begin_fork("destination-conflict", SOURCE_TWO, FORK_DEST, TS),
        Err(store::StoreError::RewriteDestinationExists)
    ));
    store.advance_rewrite("publish-gap").expect("build");
    store.advance_rewrite("publish-gap").expect("validate");
    fs::rename(
        root.path().join("staging/publish-gap/payload"),
        root.path().join("threads").join(FORK_DEST),
    )
    .expect("publish action before record");
    drop(store);
    let recovered = ThreadStore::open(root.path()).expect("reopen");
    recovered
        .recover_rewrites()
        .expect("infer published action");
    assert!(root.path().join("threads").join(SOURCE).is_dir());
    assert!(root.path().join("threads").join(FORK_DEST).is_dir());

    let root = tempfile::tempdir().expect("retire action gap root");
    let store = ThreadStore::open(root.path()).expect("store");
    prepare_source(&store, SOURCE, b"SECRET");
    store
        .begin_redact("retire-gap", SOURCE, REDACT_DEST, TS, &[b"SECRET".to_vec()])
        .expect("begin redact");
    for _ in 0..3 {
        store
            .advance_rewrite("retire-gap")
            .expect("reach published");
    }
    fs::rename(
        root.path().join("threads").join(SOURCE),
        root.path()
            .join(".rewrite-trash")
            .join(format!("retire-gap-{SOURCE}")),
    )
    .expect("retire action before record");
    drop(store);
    let recovered = ThreadStore::open(root.path()).expect("reopen");
    recovered
        .recover_rewrites()
        .expect("infer retirement action");
    assert!(!root.path().join("threads").join(SOURCE).exists());
    assert!(root.path().join("threads").join(REDACT_DEST).is_dir());

    let root = tempfile::tempdir().expect("remove action gap root");
    let store = ThreadStore::open(root.path()).expect("store");
    prepare_source(&store, SOURCE, b"SECRET");
    store
        .begin_redact("remove-gap", SOURCE, REDACT_DEST, TS, &[b"SECRET".to_vec()])
        .expect("begin redact");
    for _ in 0..4 {
        store.advance_rewrite("remove-gap").expect("reach retired");
    }
    fs::remove_dir_all(
        root.path()
            .join(".rewrite-trash")
            .join(format!("remove-gap-{SOURCE}")),
    )
    .expect("remove action before record");
    drop(store);
    let recovered = ThreadStore::open(root.path()).expect("reopen");
    recovered.recover_rewrites().expect("infer removal action");
    assert!(!root.path().join("threads").join(SOURCE).exists());
    assert!(root.path().join("threads").join(REDACT_DEST).is_dir());
}

#[test]
fn slice5_gate_24_staging_ownership_gc() {
    let root = tempfile::tempdir().expect("storage root");
    let store = ThreadStore::open(root.path()).expect("store");
    prepare_source(&store, SOURCE, b"SECRET");
    store
        .begin_fork("live-op", SOURCE, FORK_DEST, TS)
        .expect("begin fork");
    assert_eq!(
        store.advance_rewrite("live-op").expect("build").phase,
        Some(RewritePhase::Built)
    );
    assert_eq!(
        store.advance_rewrite("live-op").expect("validate").phase,
        Some(RewritePhase::Validated)
    );
    assert_eq!(
        store.advance_rewrite("live-op").expect("publish").phase,
        Some(RewritePhase::Published)
    );
    fs::create_dir(root.path().join("staging/debris-dir")).expect("debris dir");
    fs::write(root.path().join("staging/debris-file"), b"orphan").expect("debris file");
    fs::create_dir(root.path().join(".create-staging/create-live")).expect("create stage");

    let removed = store.gc_rewrite_debris().expect("rewrite GC");
    assert_eq!(removed.len(), 2);
    assert!(root.path().join("staging/live-op/op.json").is_file());
    assert!(root.path().join(".create-staging/create-live").is_dir());
    store.recover_rewrites().expect("resume live op");
    assert!(root.path().join("threads").join(FORK_DEST).is_dir());
    assert!(root.path().join("threads").join(SOURCE).is_dir());
}

fn prepare_source(store: &ThreadStore, thread: &str, secret: &[u8]) -> (String, String) {
    let folder = store.root().join("threads").join(thread);
    fs::create_dir(&folder).expect("thread folder");
    let assets = AssetStore::new(folder.join("assets")).expect("assets");
    let poisoned = assets
        .publish(&[secret, b"\n"].concat())
        .expect("poisoned asset");
    let safe = assets.publish(b"safe\n").expect("safe asset");
    let system = assets.publish(b"system\n").expect("system asset");
    let config = assets.publish(b"config\n").expect("config asset");
    let instruction = assets.publish(b"instruction\n").expect("instruction asset");
    let config_digest = config.asset.strip_prefix("sha256-").expect("config digest");
    let instruction_digest = instruction
        .asset
        .strip_prefix("sha256-")
        .expect("instruction digest");
    let seed_source = json!({
        "v":1,"seq":10,"kind":"state","ts":TS,"turn":1,
        "subkind":"seed.fact","payload":{"asset":safe.asset}
    });
    let mut seed_bytes =
        serde_json_canonicalizer::to_vec(&seed_source).expect("canonical seed source");
    seed_bytes.push(b'\n');
    let seed_snapshot = assets.publish(&seed_bytes).expect("seed snapshot");
    let seed_digest = seed_snapshot
        .asset
        .strip_prefix("sha256-")
        .expect("seed digest");

    let mut events = vec![
        json!({"v":1,"seq":1,"kind":"genesis","ts":TS,"format":1,"min_reader":1,"min_writer":1,"thread":thread,"workspace":"ws","origin_key":"create","origin_tuple":{"principal":"p","client":"cli","target":thread,"op":"create","key":"create"},"resume":"never","config":{"digest":config_digest},"instruction":{"digest":instruction_digest}}),
        json!({"v":1,"seq":2,"kind":"input","ts":TS,"origin_key":"i1","origin_tuple":{"principal":"p","client":"cli","target":thread,"op":"submit","key":"i1"},"content":[{"type":"text","text":"hello"}]}),
        json!({"v":1,"seq":3,"kind":"run_start","ts":TS,"run":"r1","mode":"ordinary","recovery_ordinal":0,"binary":"worker","config_digest":"cfg","instruction_digest":"ins","policy":"default"}),
        json!({"v":1,"seq":4,"kind":"turn_open","ts":TS,"turn":1,"trigger":{"inputs":[2]}}),
        json!({"v":1,"seq":5,"kind":"epoch","ts":TS,"id":"e1","reason":"initial","adapter":"fake","model":"model","system":{"asset":system.asset,"digest":"sys"},"tools":{"asset":"sha256-tools","digest":"tools"},"renderer":1}),
        json!({"v":1,"seq":6,"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"ts":TS,"turn":1,"attempt":"a1","epoch":"e1","wire_digest":"wire","admits":[{"from":2,"to":2}]}),
        json!({"v":1,"seq":7,"kind":"spawn","ts":TS,"turn":1,"child":CHILD,"call":"child-call","spawn_id":"spawn-1","resume":"never","seed":{"kinds":[]}}),
        json!({"v":1,"seq":8,"kind":"child_result","ts":TS,"turn":1,"child":CHILD,"call":"child-call","spawn_id":"spawn-1","outcome":"completed"}),
        json!({"v":1,"seq":9,"kind":"state","ts":TS,"turn":1,"subkind":"artifact","payload":{"asset":poisoned.asset}}),
        json!({"v":1,"seq":10,"kind":"state","ts":TS,"turn":1,"subkind":"artifact","payload":{"asset":safe.asset}}),
        json!({"v":1,"seq":11,"kind":"output","ts":TS,"turn":1,"attempt":"a1","content":[{"type":"text","text":format!("answer {}",String::from_utf8_lossy(secret))}],"sealed":{"version":1,"adapter":"fake","fragments":format!("sealed {}",String::from_utf8_lossy(secret))},"usage":{"availability":"reported","input_tokens":"1","output_tokens":"1"}}),
        json!({"v":1,"seq":12,"kind":"settle","ts":TS,"turn":1,"outcome":"completed"}),
        json!({"v":1,"seq":13,"kind":"checkpoint","ts":TS,"covers":12,"summary":"checkpoint"}),
        json!({"v":1,"seq":14,"kind":"compact","ts":TS,"covers":[{"from":6,"to":6},{"from":11,"to":11}],"summary":"compacted provider attempt"}),
        json!({"v":1,"seq":15,"kind":"meta","ts":TS,"title":"source title"}),
        json!({"v":1,"seq":16,"kind":"meta","ts":TS,"labels":["one","two"]}),
    ];
    let mut bytes = Vec::new();
    for event in &mut events {
        bytes.extend_from_slice(
            &serde_json_canonicalizer::to_vec(event).expect("canonical source event"),
        );
        bytes.push(b'\n');
    }
    validate_ledger(&bytes, 1).expect("source ledger validates");
    fs::write(folder.join("main.jsonl"), bytes).expect("source ledger");
    let child_events = [
        json!({"v":1,"seq":1,"kind":"genesis","ts":TS,"format":1,"min_reader":1,"min_writer":1,"thread":CHILD,"workspace":"ws","origin_key":"child-create","origin_tuple":{"principal":"kernel","client":"spawn","target":CHILD,"op":"create","key":"child-create"},"resume":"never","parent":{"file":"main.jsonl","seq":7,"spawn_id":"spawn-1"},"seed":{"source":"parent","kinds":["state"],"snapshot":{"asset":seed_snapshot.asset,"digest":seed_digest}},"config":{"digest":config_digest},"instruction":{"digest":instruction_digest}}),
        json!({"v":1,"seq":2,"kind":"turn_open","ts":TS,"turn":1,"trigger":"genesis"}),
        json!({"v":1,"seq":3,"kind":"state","ts":TS,"turn":1,"subkind":"child.summary","payload":{"text":"done"}}),
        json!({"v":1,"seq":4,"kind":"settle","ts":TS,"turn":1,"outcome":"completed"}),
    ];
    let mut child_bytes = Vec::new();
    for event in child_events {
        child_bytes.extend_from_slice(
            &serde_json_canonicalizer::to_vec(&event).expect("canonical child event"),
        );
        child_bytes.push(b'\n');
    }
    validate_ledger(&child_bytes, 1).expect("child ledger validates");
    fs::write(folder.join(format!("{CHILD}.jsonl")), child_bytes).expect("child ledger");
    (poisoned.asset, safe.asset)
}
