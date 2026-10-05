use std::fs;
use std::io::Write as _;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use schema::Event;
use serde_json::json;
use store::{BarrierContext, LockedLedger, NamedLock, StoreError, ThreadStore};
use tempfile::TempDir;
use thread_search::{ArchiveVisibility, SearchError, SearchRequest, ThreadSearchAuthority};

const A: &str = "018f0000-0000-7000-8000-000000000001";
const B: &str = "018f0000-0000-7000-8000-000000000002";
const C: &str = "018f0000-0000-7000-8000-000000000003";
const D: &str = "018f0000-0000-7000-8000-000000000004";
const E: &str = "018f0000-0000-7000-8000-000000000005";
const F: &str = "018f0000-0000-7000-8000-000000000006";

#[test]
fn slice14d_gate_105_contract_oracle_and_stable_identity() {
    let fixture = fixture_json("cases.canonical.json");
    let root = fixture_catalog();
    let authority = ThreadSearchAuthority::open(root.path()).expect("search authority");
    let page = authority
        .search(&request(
            "ws-main",
            "ＫERNEL\u{3000} DESIGN",
            10,
            ArchiveVisibility::All,
        ))
        .expect("search");
    let actual = serde_json::to_value(&page).expect("page JSON");
    assert_eq!(actual["results"], fixture["all_results"]);
    assert_eq!(page.results[0].session_id, A);
    assert_eq!(page.results[1].session_id, B);
    assert_eq!(page.results[2].session_id, C);
    assert_eq!(page.results[0].score, 3_000);
    assert_eq!(page.results[1].score, 2_000);
    assert_eq!(page.results[2].score, 1_000);
    assert!(page.reached_end);
    assert!(page.next_cursor.is_none());
    assert_eq!(page.index_use.missing, 4);
    assert_eq!(
        fs::read(
            root.path()
                .join("threads")
                .join(A)
                .join(".thread-search.json")
        )
        .expect("A index"),
        fs::read(fixture_path("index.canonical.json")).expect("index fixture")
    );

    let second = authority
        .search(&request(
            "ws-main",
            "kernel design",
            10,
            ArchiveVisibility::All,
        ))
        .expect("cache hit");
    assert_eq!(second.results, page.results);
    assert_eq!(second.index_use.hits, 4);
    assert!(
        authority
            .search(&request(
                "ws-other",
                "discarded",
                10,
                ArchiveVisibility::Active,
            ))
            .expect("superseded title search")
            .results
            .is_empty()
    );
}

#[test]
fn slice14d_gate_106_paging_archive_visibility_and_membership_lock() {
    let fixture = fixture_json("cases.canonical.json");
    let root = fixture_catalog();
    let authority = ThreadSearchAuthority::open(root.path()).expect("search authority");
    let first = authority
        .search(&request(
            "ws-main",
            "kernel design",
            1,
            ArchiveVisibility::Active,
        ))
        .expect("first page");
    assert_eq!(first.results[0].session_id, A);
    assert_eq!(
        first.next_cursor.as_deref(),
        fixture["active_first_cursor"].as_str()
    );
    assert!(!first.reached_end);
    let second = authority
        .search(&SearchRequest {
            after: first.next_cursor.clone(),
            ..request("ws-main", "kernel design", 1, ArchiveVisibility::Active)
        })
        .expect("second page");
    assert_eq!(second.results[0].session_id, B);
    assert!(second.reached_end);

    let archived = authority
        .search(&request(
            "ws-main",
            "kernel design",
            10,
            ArchiveVisibility::Archived,
        ))
        .expect("archived search");
    assert_eq!(archived.results.len(), 1);
    assert_eq!(archived.results[0].session_id, C);

    let store = ThreadStore::open(root.path()).expect("store");
    let rewrite = NamedLock::exclusive(root.path().join(".rewrite.lock"))
        .expect("hold rewrite before search");
    let (started, start_wait) = mpsc::channel();
    let (finished, finish_wait) = mpsc::channel();
    let blocked_authority = authority.clone();
    let blocked_search = thread::spawn(move || {
        started.send(()).expect("started signal");
        blocked_authority
            .search(&request("ws-main", "kernel", 10, ArchiveVisibility::All))
            .expect("search after lock release");
        finished.send(()).expect("finished signal");
    });
    start_wait
        .recv_timeout(Duration::from_secs(2))
        .expect("search started");
    let catalog = NamedLock::try_exclusive(root.path().join(".thread-catalog.lock"))
        .expect("search blocked on rewrite has not inverted into catalog-first");
    assert!(finish_wait.recv_timeout(Duration::from_millis(50)).is_err());
    drop(catalog);
    drop(rewrite);
    finish_wait
        .recv_timeout(Duration::from_secs(2))
        .expect("ordered search completes");
    blocked_search.join().expect("blocked search");

    let membership =
        NamedLock::shared(root.path().join(".thread-catalog.lock")).expect("catalog snapshot lock");
    let (sent, received) = mpsc::channel();
    let store_for_move = store.clone();
    let mover = thread::spawn(move || {
        store_for_move
            .archive_nonblocking(B)
            .expect("nonblocking lifecycle archive after catalog reader");
        sent.send(()).expect("signal");
    });
    assert!(received.recv_timeout(Duration::from_millis(50)).is_err());
    drop(membership);
    received
        .recv_timeout(Duration::from_secs(2))
        .expect("move completes");
    mover.join().expect("mover");

    assert!(matches!(
        authority.search(&SearchRequest {
            after: first.next_cursor,
            ..request("ws-main", "kernel design", 1, ArchiveVisibility::Active)
        }),
        Err(SearchError::CursorStale)
    ));
    let archived_after_move = authority
        .search(&request(
            "ws-main",
            "kernel design",
            10,
            ArchiveVisibility::Archived,
        ))
        .expect("archived search after move");
    assert_eq!(
        archived_after_move
            .results
            .iter()
            .map(|result| result.session_id.as_str())
            .collect::<Vec<_>>(),
        vec![B, C]
    );
    assert_eq!(archived_after_move.index_use.hits, 4);
    let moved_index: serde_json::Value = serde_json::from_slice(
        &fs::read(
            root.path()
                .join("archive")
                .join(B)
                .join(".thread-search.json"),
        )
        .expect("moved cache repaired during stale-cursor query"),
    )
    .expect("moved index JSON");
    assert_eq!(moved_index["entry"]["archived"], true);

    let membership =
        NamedLock::shared(root.path().join(".thread-catalog.lock")).expect("second catalog lock");
    let (sent, received) = mpsc::channel();
    let store_for_create = store.clone();
    let creator = thread::spawn(move || {
        let genesis = event(json!({
            "v":1,"seq":1,"kind":"genesis","ts":"2026-08-26T09:00:10.000Z",
            "format":1,"min_reader":1,"min_writer":1,"thread":E,"workspace":"ws-main",
            "origin_key":"create-5",
            "origin_tuple":{"client":"fixture","key":"create-5","op":"create","principal":"test","target":E},
            "config":{"digest":"cfg-1"},"resume":"never"
        }));
        store_for_create
            .create_thread(E, genesis)
            .expect("create after reader");
        sent.send(()).expect("signal");
    });
    assert!(received.recv_timeout(Duration::from_millis(50)).is_err());
    drop(membership);
    received
        .recv_timeout(Duration::from_secs(2))
        .expect("create completes");
    creator.join().expect("creator");
    assert!(root.path().join("threads").join(E).is_dir());
}

#[test]
fn slice14d_gate_107_rebuild_and_stale_corrupt_fallback() {
    let root = fixture_catalog();
    let authority = ThreadSearchAuthority::open(root.path()).expect("search authority");
    let baseline = authority
        .search(&request("ws-main", "kernel", 50, ArchiveVisibility::All))
        .expect("baseline");

    append_title(root.path(), B, "Kernel revised", "2026-08-26T09:00:09.000Z");
    fs::write(
        root.path()
            .join("threads")
            .join(A)
            .join(".thread-search.json"),
        b"{\"format\":1}\n",
    )
    .expect("corrupt cache");
    let repaired = authority
        .search(&request("ws-main", "kernel", 50, ArchiveVisibility::All))
        .expect("fallback");
    assert_eq!(repaired.index_use.corrupt, 1);
    assert_eq!(repaired.index_use.stale, 1);
    assert_eq!(repaired.results.len(), baseline.results.len());
    assert_eq!(repaired.results[1].title, "Kernel revised");

    for area in ["threads", "archive"] {
        for entry in fs::read_dir(root.path().join(area)).expect("area") {
            let path = entry.expect("entry").path().join(".thread-search.json");
            let _ = fs::remove_file(path);
        }
    }
    let deleted = authority
        .search(&request("ws-main", "kernel", 50, ArchiveVisibility::All))
        .expect("cacheless scan");
    assert_eq!(deleted.results, repaired.results);
    assert_eq!(deleted.index_use.missing, 4);
    let digest = authority.rebuild_index().expect("explicit rebuild");
    assert!(!digest.is_empty());
    let hit = authority
        .search(&request("ws-main", "kernel", 50, ArchiveVisibility::All))
        .expect("rebuilt hit");
    assert_eq!(hit.index_use.hits, 4);

    let blocked_cache = root
        .path()
        .join("threads")
        .join(A)
        .join(".thread-search.json");
    fs::remove_file(&blocked_cache).expect("remove A cache");
    fs::create_dir(&blocked_cache).expect("make cache destination unwritable");
    let fallback = authority
        .search(&request("ws-main", "kernel", 50, ArchiveVisibility::All))
        .expect("cache publication failure is query-soft");
    assert_eq!(fallback.results, repaired.results);
    assert_eq!(fallback.index_use.corrupt, 1);
    assert_eq!(fallback.index_use.write_failed, 1);
}

#[test]
fn slice14d_gate_108_fail_closed_and_semantic_non_mutation() {
    let invalid = fixture_json("invalid.canonical.json");
    let root = fixture_catalog();
    let authority = ThreadSearchAuthority::open(root.path()).expect("search authority");
    let ledger = root.path().join("threads").join(A).join("main.jsonl");
    let endpoint = root.path().join("threads").join(A).join("endpoint.jsonl");
    fs::write(&endpoint, b"endpoint-sentinel\n").expect("endpoint sentinel");
    let before_ledger = fs::read(&ledger).expect("ledger before");
    let before_endpoint = fs::read(&endpoint).expect("endpoint before");

    assert!(matches!(
        authority.search(&request("ws-main", "   ", 5, ArchiveVisibility::Active)),
        Err(SearchError::InvalidQuery)
    ));
    assert!(matches!(
        authority.search(&request("ws-main", "kernel", 0, ArchiveVisibility::Active)),
        Err(SearchError::InvalidLimit)
    ));
    for cursor in invalid["malformed_cursors"]
        .as_array()
        .expect("cursor cases")
    {
        assert!(matches!(
            authority.search(&SearchRequest {
                after: Some(cursor.as_str().expect("cursor string").to_owned()),
                ..request("ws-main", "kernel", 5, ArchiveVisibility::Active)
            }),
            Err(SearchError::MalformedCursor)
        ));
    }

    let first = authority
        .search(&request("ws-main", "kernel", 1, ArchiveVisibility::Active))
        .expect("cursor source");
    assert!(matches!(
        authority.search(&SearchRequest {
            after: first.next_cursor,
            ..request("ws-main", "different", 1, ArchiveVisibility::Active)
        }),
        Err(SearchError::CursorScope)
    ));
    assert_eq!(fs::read(ledger).expect("ledger after"), before_ledger);
    assert_eq!(fs::read(endpoint).expect("endpoint after"), before_endpoint);

    let source_cache = root
        .path()
        .join("threads")
        .join(A)
        .join(".thread-search.json");
    fs::write(&source_cache, b"cache-must-not-enter-materialized-fork\n")
        .expect("search cache sentinel");
    let store = ThreadStore::open(root.path()).expect("rewrite store");
    let catalog =
        NamedLock::shared(root.path().join(".thread-catalog.lock")).expect("catalog observer");
    let (finished, finish_wait) = mpsc::channel();
    let rewrite_store = store.clone();
    let rewriter = thread::spawn(move || {
        rewrite_store
            .begin_fork("slice14d-cache-fork", A, F, "2026-08-26T10:00:00.000Z")
            .expect("prepare fork after catalog observer releases");
        finished.send(()).expect("rewrite finished signal");
    });
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        match NamedLock::try_exclusive(root.path().join(".rewrite.lock")) {
            Err(StoreError::Busy) => break,
            Ok(lock) => drop(lock),
            Err(error) => panic!("rewrite lock probe failed: {error}"),
        }
        assert!(
            Instant::now() < deadline,
            "rewrite never acquired its first lock"
        );
        thread::yield_now();
    }
    assert!(finish_wait.recv_timeout(Duration::from_millis(50)).is_err());
    drop(catalog);
    finish_wait
        .recv_timeout(Duration::from_secs(2))
        .expect("rewrite proceeds after catalog release");
    rewriter.join().expect("rewriter");
    let colliding_genesis = event(json!({
        "v":1,"seq":1,"kind":"genesis","ts":"2026-08-26T10:00:01.000Z",
        "format":1,"min_reader":1,"min_writer":1,"thread":F,"workspace":"ws-main",
        "origin_key":"colliding-create",
        "origin_tuple":{"client":"fixture","key":"colliding-create","op":"create","principal":"test","target":F},
        "config":{"digest":"cfg-1"},"resume":"never"
    }));
    assert!(matches!(
        store.create_thread(F, colliding_genesis),
        Err(StoreError::ThreadCollision)
    ));
    store
        .recover_rewrites()
        .expect("materialize fork through rewrite-exclusive path");
    assert!(root.path().join("threads").join(F).is_dir());
    assert!(
        !root
            .path()
            .join("threads")
            .join(F)
            .join(".thread-search.json")
            .exists(),
        "materialized projection must not copy the source search cache"
    );

    let ledger = root.path().join("threads").join(A).join("main.jsonl");
    std::fs::OpenOptions::new()
        .append(true)
        .open(&ledger)
        .expect("open corrupt tail")
        .write_all(b"{\"complete\":false}\n")
        .expect("append corrupt complete line");
    let corrupt_source = fs::read(&ledger).expect("corrupt source bytes");
    assert!(matches!(
        authority.search(&request("ws-main", "kernel", 5, ArchiveVisibility::Active)),
        Err(SearchError::SourceCorrupt(_))
    ));
    assert_eq!(
        fs::read(ledger).expect("source after rejection"),
        corrupt_source
    );
}

fn request(
    workspace_id: &str,
    query: &str,
    limit: usize,
    visibility: ArchiveVisibility,
) -> SearchRequest {
    SearchRequest {
        workspace_id: workspace_id.to_owned(),
        query: query.to_owned(),
        limit,
        visibility,
        after: None,
    }
}

fn fixture_catalog() -> TempDir {
    let root = tempfile::tempdir().expect("temp root");
    let store = ThreadStore::open(root.path()).expect("store");
    for (id, workspace, title, second) in [
        (A, "ws-main", "Kernel Design", 1_u64),
        (B, "ws-main", "kernel design notes", 2),
        (C, "ws-main", "notes from kernel design", 3),
        (D, "ws-other", "discarded name", 4),
    ] {
        let genesis = event(json!({
            "v":1,"seq":1,"kind":"genesis","ts":format!("2026-08-26T09:00:0{second}.000Z"),
            "format":1,"min_reader":1,"min_writer":1,"thread":id,"workspace":workspace,
            "origin_key":format!("create-{second}"),
            "origin_tuple":{"client":"fixture","key":format!("create-{second}"),"op":"create","principal":"test","target":id},
            "config":{"digest":"cfg-1"},"resume":"never"
        }));
        store.create_thread(id, genesis).expect("create thread");
        append_title(
            root.path(),
            id,
            title,
            &format!("2026-08-26T09:00:0{}.000Z", second + 4),
        );
    }
    append_superseding_title(
        root.path(),
        D,
        "kernel design elsewhere",
        "2026-08-26T09:00:09.000Z",
        2,
    );
    store.archive(C).expect("archive C");
    root
}

fn append_superseding_title(
    root: &std::path::Path,
    id: &str,
    title: &str,
    timestamp: &str,
    superseded_seq: u64,
) {
    let path = root.join("threads").join(id).join("main.jsonl");
    let mut ledger = LockedLedger::open(path, 1).expect("open ledger");
    let seq = ledger.next_seq();
    ledger
        .append_contract(
            event(json!({
                "v":1,"seq":seq,"kind":"meta","ts":timestamp,"title":title,
                "supersedes":[{"from":superseded_seq,"to":superseded_seq}]
            })),
            BarrierContext::default(),
        )
        .expect("append superseding title");
}

fn append_title(root: &std::path::Path, id: &str, title: &str, timestamp: &str) {
    let area = if root.join("threads").join(id).is_dir() {
        "threads"
    } else {
        "archive"
    };
    let path = root.join(area).join(id).join("main.jsonl");
    let mut ledger = LockedLedger::open(path, 1).expect("open ledger");
    let seq = ledger.next_seq();
    ledger
        .append_contract(
            event(json!({"v":1,"seq":seq,"kind":"meta","ts":timestamp,"title":title})),
            BarrierContext::default(),
        )
        .expect("append title");
}

fn event(value: serde_json::Value) -> Event {
    Event::decode_canonical(&serde_json_canonicalizer::to_vec(&value).expect("canonical event"))
        .expect("valid event")
}

fn fixture_json(name: &str) -> serde_json::Value {
    serde_json::from_slice(&fs::read(fixture_path(name)).expect("fixture bytes"))
        .expect("fixture JSON")
}

fn fixture_path(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/thread-search")
        .join(name)
}

#[test]
fn host_search_matches_content_across_workspaces_and_archive() {
    let root = fixture_catalog();
    for (id, area) in [(A, "threads"), (D, "threads"), (C, "archive")] {
        let mut ledger =
            LockedLedger::open(root.path().join(area).join(id).join("main.jsonl"), 1).unwrap();
        let seq = ledger.next_seq();
        ledger.append_contract(event(json!({
            "v":1,"seq":seq,"kind":"input","ts":"2026-08-26T10:00:00.000Z",
            "origin_key":"host-search-input",
            "origin_tuple":{"client":"fixture","key":"host-search-input","op":"submit","principal":"test","target":id},
            "content":[{"type":"text","text":format!("{} UNIQUE 会话正文 suffix", "开头".repeat(100))}]
        })), BarrierContext::default()).unwrap();
    }
    let authority = ThreadSearchAuthority::open(root.path()).unwrap();
    let result = authority.search_sessions("unique 会话正文").unwrap();
    assert_eq!(
        result
            .items
            .iter()
            .map(|item| item.session_id.as_str())
            .collect::<Vec<_>>(),
        vec![A, C, D]
    );
    assert!(!result.has_more);
    assert!(
        result
            .items
            .iter()
            .all(|item| item.snippet.contains("unique 会话正文") && item.snippet.starts_with('…'))
    );
    assert!(
        authority
            .search_sessions("discarded name")
            .unwrap()
            .items
            .is_empty()
    );
    assert!(authority.search_sessions("  ").is_err());
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value["items"][0]["sessionId"], A);
    assert_eq!(value["hasMore"], false);
}
