use super::*;
use std::sync::atomic::AtomicUsize;

const SESSION: &str = "018f0000-0000-7000-8000-0000000000b1";

fn ledger_lines(count: u64) -> Vec<String> {
    let mut lines = vec![format!(
        r#"{{"config":{{"digest":"cfg"}},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{{"client":"cli","key":"create","op":"create","principal":"p","target":"{SESSION}"}},"resume":"never","seq":1,"thread":"{SESSION}","ts":"2026-08-26T09:00:00.000Z","v":1,"workspace":"ws"}}"#
    )];
    for seq in 2..=count {
        lines.push(format!(
            r#"{{"kind":"meta","seq":{seq},"title":"t{seq}","ts":"2026-08-26T09:00:00.000Z","v":1}}"#
        ));
    }
    lines
}

fn folder_with(events: u64) -> (tempfile::TempDir, PathBuf) {
    let root = tempfile::tempdir().unwrap();
    let folder = root.path().join(SESSION);
    std::fs::create_dir(&folder).unwrap();
    std::fs::write(
        folder.join("main.jsonl"),
        ledger_lines(events).join("\n") + "\n",
    )
    .unwrap();
    (root, folder)
}

fn fold(folder: &std::path::Path, session: &str) -> PreparedCard {
    let bytes = std::fs::read(folder.join("main.jsonl")).unwrap();
    let scan = store::scan_valid_prefix(&bytes, 1);
    let valid = scan.valid_bytes as usize;
    let projection = scan.projection.unwrap();
    prepare(
        session,
        folder.to_path_buf(),
        bytes,
        valid,
        &projection,
        None,
    )
    .unwrap()
}

fn tuning(debounce_ms: u64, max_per_sec: u32, budget_ms: u64) -> Tuning {
    Tuning {
        debounce: Duration::from_millis(debounce_ms),
        max_per_sec,
        shutdown_budget: Duration::from_millis(budget_ms),
    }
}

#[test]
fn is_due_claims_one_window_per_thread() {
    let writer = CardWriter::spawn_with(tuning(150, 0, 500), Arc::new(|_| WriteOutcome::Written));
    assert!(writer.is_due("a"));
    assert!(!writer.is_due("a"), "inside the window nothing is prepared");
    assert!(writer.is_due("b"), "windows are per thread");
    std::thread::sleep(Duration::from_millis(200));
    assert!(writer.is_due("a"), "a new window opens after the debounce");
    writer.shutdown();
}

#[test]
fn doorbells_inside_the_window_converge_to_the_newest_state_without_preparing() {
    let (_root, folder) = folder_with(3);
    let writer = CardWriter::spawn(tuning(80, 0, 1_000));
    assert!(writer.is_due(SESSION));
    writer.submit(fold(&folder, SESSION));
    // more events arrive while the window is open: only dirty marks, no preparation
    {
        use std::io::Write as _;
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(folder.join("main.jsonl"))
            .unwrap();
        for seq in 4..=9 {
            let line = format!(
                r#"{{"kind":"meta","seq":{seq},"title":"t{seq}","ts":"2026-08-26T09:00:00.000Z","v":1}}"#
            );
            file.write_all((line + "\n").as_bytes()).unwrap();
            assert!(!writer.is_due(SESSION));
            writer.mark_dirty(SESSION, folder.clone(), None);
        }
    }
    writer.shutdown();
    let card = store::card::read_card(&folder).expect("card");
    assert_eq!(
        card.lines[0].last_seq, 9,
        "the dirty re-fold wrote the final state"
    );
    assert_eq!(
        store::card::classify(&folder, &card).unwrap().freshness(),
        store::card::Freshness::Fresh
    );
}

#[test]
fn writes_are_paced_to_the_global_cap() {
    let calls = Arc::new(Mutex::new(Vec::<Instant>::new()));
    let seen = Arc::clone(&calls);
    // 20 per second => at least 50 ms between writes (the debounce is irrelevant: distinct threads)
    let writer = CardWriter::spawn_with(
        tuning(0, 20, 2_000),
        Arc::new(move |_| {
            seen.lock().unwrap().push(Instant::now());
            WriteOutcome::Written
        }),
    );
    let (_root, folder) = folder_with(3);
    for index in 0..6 {
        let mut card = fold(&folder, SESSION);
        card.session = format!("s{index}");
        writer.submit(card);
    }
    let started = Instant::now();
    while calls.lock().unwrap().len() < 6 && started.elapsed() < Duration::from_secs(5) {
        std::thread::sleep(Duration::from_millis(10));
    }
    writer.shutdown();
    let calls = calls.lock().unwrap();
    assert_eq!(calls.len(), 6);
    for pair in calls.windows(2) {
        assert!(
            pair[1].duration_since(pair[0]) >= Duration::from_millis(40),
            "writes closer than the 20/s cap: {:?}",
            pair[1].duration_since(pair[0])
        );
    }
}

#[test]
fn shutdown_flush_is_bounded_by_its_budget_even_with_a_slow_disk() {
    let written = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&written);
    let writer = CardWriter::spawn_with(
        tuning(0, 0, 300),
        Arc::new(move |_| {
            std::thread::sleep(Duration::from_millis(120));
            counter.fetch_add(1, Ordering::Relaxed);
            WriteOutcome::Written
        }),
    );
    let (_root, folder) = folder_with(3);
    for index in 0..40 {
        let mut card = fold(&folder, SESSION);
        card.session = format!("s{index}");
        card.lines[0].last_seq = index; // distinct fingerprints
        writer.submit(card);
    }
    let started = Instant::now();
    writer.shutdown();
    assert!(
        started.elapsed() < Duration::from_millis(1_500),
        "quit must not wait for 40 slow writes: {:?}",
        started.elapsed()
    );
    assert!(written.load(Ordering::Relaxed) < 40);
    assert!(writer.stats.dropped_at_shutdown.load(Ordering::Relaxed) > 0);
}

#[test]
fn children_are_folded_on_the_writer_thread_not_by_prepare() {
    let (_root, folder) = folder_with(3);
    let child = "018f0000-0000-7000-8000-0000000000b2";
    std::fs::write(
        folder.join(format!("{child}.jsonl")),
        format!(
            r#"{{"config":{{"digest":"cfg"}},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"c","origin_tuple":{{"client":"cli","key":"c","op":"create","principal":"p","target":"{child}"}},"resume":"never","seq":1,"thread":"{child}","ts":"2026-08-26T09:00:00.000Z","v":1,"workspace":"ws"}}"#
        ) + "\n",
    )
    .unwrap();
    let prepared = fold(&folder, SESSION);
    assert_eq!(prepared.lines.len(), 1, "prepare reads only the main line");
    let writer = CardWriter::spawn(tuning(0, 0, 1_000));
    writer.submit(prepared);
    writer.shutdown();
    let card = store::card::read_card(&folder).expect("card");
    assert_eq!(
        card.lines.len(),
        2,
        "the writer thread added the child line"
    );
    assert_eq!(card.lines[1].file, format!("{child}.jsonl"));
}
