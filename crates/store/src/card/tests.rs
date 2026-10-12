use super::*;
use crate::{LockedLedger, scan_valid_prefix};
use schema::Event;
use serde_json::{Value, json};

const THREAD: &str = "018f0000-0000-7000-8000-000000000003";

fn event(mut value: Value, seq: u64) -> Event {
    value["v"] = json!(1);
    value["seq"] = json!(seq);
    value["ts"] = json!("2026-09-04T00:00:00.000Z");
    Event::decode_canonical(&serde_json_canonicalizer::to_vec(&value).unwrap()).unwrap()
}

fn genesis() -> Event {
    event(
        json!({"kind":"genesis","format":1,"min_reader":1,"min_writer":1,
        "thread":THREAD,"workspace":"ws","resume":"never",
        "config":{"digest":"cfg"},"origin_key":"create",
        "origin_tuple":{"principal":"p","client":"c","target":THREAD,"op":"create","key":"create"}}),
        1,
    )
}

fn title(text: &str, seq: u64) -> Event {
    event(json!({"kind":"meta","title":text}), seq)
}

fn line(event: &Event) -> Vec<u8> {
    let mut bytes = event.canonical_bytes().unwrap();
    bytes.push(b'\n');
    bytes
}

fn summary() -> CardSummary {
    CardSummary {
        workspace: "ws".into(),
        folder_binding: None,
        created: "2026-09-04T00:00:00.000Z".into(),
        updated_ms: 1,
        title: None,
        identity_profile: Some("coding".into()),
        ephemeral: false,
        blank: true,
    }
}

/// A thread folder named `THREAD` holding a ledger of `titles.len() + 1` events.
fn thread_with(titles: &[&str]) -> (tempfile::TempDir, std::path::PathBuf) {
    let root = tempfile::tempdir().unwrap();
    let folder = root.path().join(THREAD);
    std::fs::create_dir(&folder).unwrap();
    let mut bytes = line(&genesis());
    for (index, text) in titles.iter().enumerate() {
        bytes.extend(line(&title(text, index as u64 + 2)));
    }
    std::fs::write(folder.join("main.jsonl"), bytes).unwrap();
    (root, folder)
}

/// Folds `main.jsonl` and writes its card (like the supervisor does after a decode).
fn write_for(folder: &Path) -> WriteOutcome {
    let bytes = std::fs::read(folder.join("main.jsonl")).unwrap();
    let scan = scan_valid_prefix(&bytes, 1);
    let projection = scan.projection.clone().unwrap();
    let input = CardInput {
        thread: THREAD,
        kernel: "test",
        written: "2026-09-04T00:00:00.000Z".into(),
        lines: vec![LineInput {
            file: "main.jsonl",
            verified_prefix: &bytes[..scan.valid_bytes as usize],
            last_seq: projection.last_seq,
            line: THREAD.into(),
            parent_file: None,
            facts: projection.lifecycle,
            holds: vec![Hold {
                hold_seq: 9,
                kind: "approval".into(),
                resolved: Some(HoldResolution {
                    seq: 10,
                    outcome: "allowed_once".into(),
                }),
            }],
        }],
        summary: summary(),
        journal: None,
    };
    write_card(folder, &input).unwrap()
}

fn input_for<'a>(prefix: &'a [u8], projection: &schema::LedgerProjection) -> CardInput<'a> {
    CardInput {
        thread: THREAD,
        kernel: "test",
        written: "t".into(),
        lines: vec![LineInput {
            file: "main.jsonl",
            verified_prefix: prefix,
            last_seq: projection.last_seq,
            line: THREAD.into(),
            parent_file: None,
            facts: projection.lifecycle.clone(),
            holds: Vec::new(),
        }],
        summary: summary(),
        journal: None,
    }
}

fn verdict(folder: &Path) -> Freshness {
    let card = read_card(folder).expect("card");
    classify(folder, &card).unwrap().freshness()
}

#[test]
fn card_roundtrips_and_rejects_every_kind_of_damage() {
    let (_root, folder) = thread_with(&["a", "b"]);
    assert_eq!(write_for(&folder), WriteOutcome::Written);
    let card = read_card(&folder).expect("valid card");
    assert_eq!(card.thread, THREAD);
    assert_eq!(card.lines[0].last_seq, 3);
    assert_eq!(card.lines[0].holds[0].resolved.as_ref().unwrap().seq, 10);
    let bytes = std::fs::read(folder.join(CARD_FILE)).unwrap();
    assert!(bytes.ends_with(b"\n"));
    assert_eq!(ThreadCard::decode(&bytes).unwrap(), card);

    // bit flip, truncation, zero-fill, altered number with the digest left as it was,
    // unknown field, wrong version: all read as "no card".
    let mut flipped = bytes.clone();
    flipped[bytes.len() / 2] ^= 0x01;
    assert!(ThreadCard::decode(&flipped).is_none());
    assert!(ThreadCard::decode(&bytes[..bytes.len() / 2]).is_none());
    assert!(ThreadCard::decode(&vec![0_u8; bytes.len()]).is_none());
    let text = String::from_utf8(bytes.clone()).unwrap();
    let altered = text.replacen("\"valid_bytes\":", "\"valid_bytes\":1", 1);
    assert!(ThreadCard::decode(altered.as_bytes()).is_none());
    let extra = text.replacen("{\"", "{\"extra\":1,\"", 1);
    assert!(ThreadCard::decode(extra.as_bytes()).is_none());
    let mut other = card.clone();
    other.v = 2;
    assert!(ThreadCard::decode(&other.encode().unwrap()).is_none());

    // the same damage on disk after the rename: read_card says None, nothing panics
    std::fs::write(folder.join(CARD_FILE), &flipped).unwrap();
    assert!(read_card(&folder).is_none());
    std::fs::write(folder.join(CARD_FILE), b"").unwrap();
    assert!(read_card(&folder).is_none());
}

#[test]
fn freshness_follows_content_not_inode_or_mtime() {
    let (root, folder) = thread_with(&["a", "b", "c"]);
    write_for(&folder);
    assert_eq!(verdict(&folder), Freshness::Fresh);

    // append: Ahead, and the verified prefix is still trusted
    let ledger = folder.join("main.jsonl");
    let before = std::fs::read(&ledger).unwrap();
    {
        use std::io::Write as _;
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&ledger)
            .unwrap();
        file.write_all(&line(&title("d", 5))).unwrap();
    }
    assert_eq!(verdict(&folder), Freshness::Ahead);

    // a copy under another parent keeps the folder name and the content: still Ahead/Fresh
    // although every inode and mtime differ (cp -c, rsync, tar, backup restore).
    std::fs::write(&ledger, &before).unwrap();
    let clone_parent = tempfile::tempdir().unwrap();
    let clone = clone_parent.path().join(THREAD);
    std::fs::create_dir(&clone).unwrap();
    for entry in std::fs::read_dir(&folder).unwrap() {
        let entry = entry.unwrap();
        std::fs::copy(entry.path(), clone.join(entry.file_name())).unwrap();
    }
    assert_eq!(verdict(&clone), Freshness::Fresh);
    // moved into an archive-like sibling directory: same name, still Fresh
    let archive = root.path().join("archive");
    std::fs::create_dir(&archive).unwrap();
    std::fs::rename(&folder, archive.join(THREAD)).unwrap();
    assert_eq!(verdict(&archive.join(THREAD)), Freshness::Fresh);

    // a copy under a different folder name does not belong to the card
    let renamed = clone_parent.path().join("someone-else");
    std::fs::rename(&clone, &renamed).unwrap();
    assert_eq!(verdict(&renamed), Freshness::Cold);
}

#[test]
fn any_edit_of_a_verified_byte_makes_the_card_cold() {
    let (_root, folder) = thread_with(&["alpha", "bravo", "charlie", "delta"]);
    write_for(&folder);
    let ledger = folder.join("main.jsonl");
    let original = std::fs::read(&ledger).unwrap();
    let card = read_card(&folder).unwrap();
    let first = &card.lines[0];

    // shorter than the card claims
    std::fs::write(&ledger, &original[..original.len() - 1]).unwrap();
    assert_eq!(verdict(&folder), Freshness::Cold);
    // same length, last line altered
    let mut tampered = original.clone();
    let at = original.len() - 3;
    tampered[at] = if tampered[at] == b'x' { b'y' } else { b'x' };
    std::fs::write(&ledger, &tampered).unwrap();
    assert_eq!(verdict(&folder), Freshness::Cold);
    // same length, first line altered
    let mut tampered = original.clone();
    tampered[3] ^= 0x20;
    std::fs::write(&ledger, &tampered).unwrap();
    assert_eq!(verdict(&folder), Freshness::Cold);
    // same length, one sampled window altered (not the first or the last line)
    for sample in &first.samples {
        let inside_first = sample.off < first.genesis_len;
        let inside_last = sample.off + sample.len > first.valid_bytes - first.tail_len;
        if inside_first || inside_last {
            continue;
        }
        let mut tampered = original.clone();
        tampered[sample.off as usize] ^= 0x01;
        std::fs::write(&ledger, &tampered).unwrap();
        assert_eq!(
            verdict(&folder),
            Freshness::Cold,
            "window at {}",
            sample.off
        );
    }
    // missing ledger
    std::fs::remove_file(&ledger).unwrap();
    assert_eq!(verdict(&folder), Freshness::Cold);
    // restored: Fresh again
    std::fs::write(&ledger, &original).unwrap();
    assert_eq!(verdict(&folder), Freshness::Fresh);
    // a child ledger that appeared after the card makes the thread Ahead
    std::fs::write(
        folder.join("018f0000-0000-7000-8000-000000000004.jsonl"),
        line(&genesis()),
    )
    .unwrap();
    assert_eq!(verdict(&folder), Freshness::Ahead);
}

#[test]
fn writer_skips_instead_of_claiming_bytes_it_cannot_pin() {
    let (_root, folder) = thread_with(&["a", "b"]);
    let ledger = folder.join("main.jsonl");
    let bytes = std::fs::read(&ledger).unwrap();
    let scan = scan_valid_prefix(&bytes, 1);
    let projection = scan.projection.clone().unwrap();
    let build = |prefix| input_for(prefix, &projection);
    // the file was rewritten after the fold: the pinned descriptor no longer matches
    let mut different = bytes.clone();
    let at = bytes.len() - 3;
    different[at] ^= 0x01;
    std::fs::write(&ledger, &different).unwrap();
    assert_eq!(
        write_card(&folder, &build(&bytes)).unwrap(),
        WriteOutcome::Skipped(SkipReason::FileChanged)
    );
    assert!(!folder.join(CARD_FILE).exists());
    // the file is shorter than what was folded
    std::fs::write(&ledger, &bytes[..bytes.len() - 5]).unwrap();
    assert_eq!(
        write_card(&folder, &build(&bytes)).unwrap(),
        WriteOutcome::Skipped(SkipReason::FileChanged)
    );
    // the ledger vanished (archive / discard race) and the folder vanished
    std::fs::remove_file(&ledger).unwrap();
    assert_eq!(
        write_card(&folder, &build(&bytes)).unwrap(),
        WriteOutcome::Skipped(SkipReason::FileGone)
    );
    assert_eq!(
        write_card(&folder.join("missing"), &build(&bytes)).unwrap(),
        WriteOutcome::Skipped(SkipReason::FolderGone)
    );
    // no temp file is left behind by any of the above
    let leftovers = std::fs::read_dir(&folder)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".tmp"))
        .count();
    assert_eq!(leftovers, 0);
}

#[test]
fn journal_identity_is_classified_without_parsing_rows() {
    let (_root, folder) = thread_with(&["a"]);
    let journal = folder.join("endpoint.jsonl");
    let rows = b"{\"row\":0,\"pad\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}\n{\"row\":1,\"pad\":\"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\"}\n";
    std::fs::write(&journal, rows).unwrap();
    let bytes = std::fs::read(folder.join("main.jsonl")).unwrap();
    let scan = scan_valid_prefix(&bytes, 1);
    let projection = scan.projection.clone().unwrap();
    let input = CardInput {
        thread: THREAD,
        kernel: "test",
        written: "t".into(),
        lines: vec![LineInput {
            file: "main.jsonl",
            verified_prefix: &bytes,
            last_seq: projection.last_seq,
            line: THREAD.into(),
            parent_file: None,
            facts: projection.lifecycle,
            holds: Vec::new(),
        }],
        summary: summary(),
        journal: Some(JournalInput {
            len: rows.len() as u64,
            rows: 2,
            last_endpoint_seq: 1,
            last_kernel_seq: 2,
        }),
    };
    assert_eq!(write_card(&folder, &input).unwrap(), WriteOutcome::Written);
    let card = read_card(&folder).unwrap();
    let classified = classify(&folder, &card).unwrap();
    assert_eq!(classified.journal, JournalVerdict::Fresh);
    {
        use std::io::Write as _;
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&journal)
            .unwrap();
        file.write_all(b"{\"row\":2}\n").unwrap();
    }
    assert_eq!(
        classify(&folder, &card).unwrap().journal,
        JournalVerdict::Ahead
    );
    std::fs::write(&journal, b"{\"row\":0,\"pad\":\"changed\"}\n").unwrap();
    assert_eq!(
        classify(&folder, &card).unwrap().journal,
        JournalVerdict::Stale
    );
}

#[test]
fn the_card_module_takes_no_locks() {
    let source = include_str!("../card.rs");
    for forbidden in [
        "flock(",
        "try_lock",
        "LockedLedger::",
        "DirectoryLock::",
        "NamedLock::",
        "RootLock",
        "F_SETLK",
        "lockf(",
    ] {
        assert!(
            !source.contains(forbidden),
            "card.rs must not use `{forbidden}`: the card writer never takes a ledger lock"
        );
    }
}

/// A worker holds the line flock and appends continuously while the card writer folds and
/// writes cards as fast as it can. The worker never sees contention, loses nothing, and every
/// card the writer produced is trustworthy for the final file.
#[test]
fn card_writes_never_disturb_a_worker_appending_under_the_flock() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    let (_root, folder) = thread_with(&[]);
    let ledger = folder.join("main.jsonl");
    let done = Arc::new(AtomicBool::new(false));
    let worker = {
        let ledger = ledger.clone();
        let done = Arc::clone(&done);
        std::thread::spawn(move || {
            // The worker owns the flock for its whole life; a card writer that tried to lock
            // would make this `open` of a later writer fail, and any lock taken by the card
            // module would be visible here as a failed `append`.
            let mut writer = LockedLedger::open(&ledger, 1).expect("worker lock");
            for seq in 2..=400_u64 {
                writer
                    .append(title(&format!("event-{seq}"), seq), seq % 7 == 0)
                    .expect("worker append is never blocked by the card writer");
            }
            done.store(true, Ordering::Release);
            writer.next_seq() - 1
        })
    };
    let mut written = 0_u32;
    while !done.load(Ordering::Acquire) {
        if write_for(&folder) == WriteOutcome::Written {
            written += 1;
        }
        // every card the writer produced describes a durable prefix of the file at that time
        if let Some(card) = read_card(&folder) {
            let classified = classify(&folder, &card).unwrap();
            assert_ne!(
                classified.freshness(),
                Freshness::Cold,
                "card was Cold after write"
            );
        }
    }
    let last = worker.join().unwrap();
    assert_eq!(last, 400);
    assert!(
        written > 0,
        "the writer made progress while the worker appended"
    );
    write_for(&folder);
    assert_eq!(verdict(&folder), Freshness::Fresh);
    let card = read_card(&folder).unwrap();
    assert_eq!(card.lines[0].last_seq, 400);
    let on_disk = std::fs::read(&ledger).unwrap();
    assert_eq!(card.lines[0].valid_bytes, on_disk.len() as u64);
}

/// Manual check against a real root: `TEKES_CARD_CHECK_ROOT=<kernel root> cargo test -p store
/// classify_every_card_of_a_real_root -- --ignored --nocapture`. Prints the verdict counts and
/// fails when any present card is not Fresh.
#[test]
#[ignore = "needs TEKES_CARD_CHECK_ROOT"]
fn classify_every_card_of_a_real_root() {
    let root = std::env::var("TEKES_CARD_CHECK_ROOT").expect("TEKES_CARD_CHECK_ROOT");
    let (mut fresh, mut ahead, mut cold, mut missing) = (0, 0, 0, 0);
    for entry in std::fs::read_dir(Path::new(&root).join("threads")).unwrap() {
        let folder = entry.unwrap().path();
        if !folder.is_dir() {
            continue;
        }
        let started = std::time::Instant::now();
        match read_card(&folder) {
            None => missing += 1,
            Some(card) => match classify(&folder, &card).unwrap().freshness() {
                Freshness::Fresh => fresh += 1,
                Freshness::Ahead => ahead += 1,
                Freshness::Cold => cold += 1,
            },
        }
        assert!(started.elapsed() < std::time::Duration::from_millis(200));
    }
    println!("fresh={fresh} ahead={ahead} cold={cold} missing={missing}");
    assert_eq!(ahead + cold + missing, 0);
}

#[test]
fn a_file_that_shrinks_during_classification_is_stale_not_an_io_error() {
    // verify_prefix against a descriptor whose file was truncated after the length check:
    // simulate by truncating through a second handle between metadata and pread via a card
    // whose claimed prefix is longer than the (shrunk) file.
    let (_root, folder) = thread_with(&["a", "b", "c"]);
    write_for(&folder);
    let card = read_card(&folder).unwrap();
    let ledger = folder.join("main.jsonl");
    let original = std::fs::read(&ledger).unwrap();
    let file = open_nofollow(&ledger).unwrap();
    std::fs::write(&ledger, &original[..40]).unwrap();
    let first = &card.lines[0];
    // The descriptor still sees the truncated file: every read past EOF is UnexpectedEof.
    let verdict = verify_prefix(
        &file,
        first.valid_bytes,
        first.genesis_len,
        &first.genesis_sha256,
        first.tail_len,
        &first.tail_sha256,
        &first.samples,
    )
    .expect("a shrink is a verdict, not an error");
    assert_eq!(verdict, LineVerdict::Stale);
    // The race window itself (length check passed, file shrank before the read) is the
    // `pread_if_present` contract: EOF is `None`, which every caller maps to Stale/Skipped.
    let short = std::fs::File::open(&ledger).unwrap();
    assert!(pread_if_present(&short, 0, 40).unwrap().is_some());
    assert!(pread_if_present(&short, 0, 41).unwrap().is_none());
    assert!(pread_if_present(&short, 100, 1).unwrap().is_none());
}

#[test]
fn card_line_names_are_plain_jsonl_names() {
    for good in ["main.jsonl", "018f0000-0000-7000-8000-000000000004.jsonl"] {
        assert!(valid_line_name(good), "{good}");
    }
    for bad in [
        "",
        ".jsonl",
        "endpoint.jsonl",
        "../main.jsonl",
        "a/b.jsonl",
        "..",
        ".hidden.jsonl",
        "main.json",
        "main.jsonl\0",
        "a\\b.jsonl",
        "main.JSONL",
    ] {
        assert!(!valid_line_name(bad), "{bad:?}");
    }
    // A hand-edited card that names a path is not a card, and a writer refuses such a name.
    let (_root, folder) = thread_with(&["a"]);
    write_for(&folder);
    let mut card = read_card(&folder).unwrap();
    card.lines[0].file = "../escape.jsonl".into();
    assert!(ThreadCard::decode(&card.encode().unwrap()).is_none());
    let bytes = std::fs::read(folder.join("main.jsonl")).unwrap();
    let scan = scan_valid_prefix(&bytes, 1);
    let projection = scan.projection.unwrap();
    let mut input = input_for(&bytes, &projection);
    input.lines[0].file = "../escape.jsonl";
    assert_eq!(
        write_card(&folder, &input).unwrap(),
        WriteOutcome::Skipped(SkipReason::InvalidName)
    );
}

/// O-final m9: a crashed process's temporary card file is removed; a live process's, ours and
/// anything that is not a card temporary are left alone.
#[test]
fn stale_card_temporaries_of_dead_processes_are_removed() {
    let (_root, folder) = thread_with(&["a"]);
    let mut child = std::process::Command::new("true").spawn().unwrap();
    let dead = child.id();
    child.wait().unwrap();
    let live_parent = std::os::unix::process::parent_id();
    let ours = std::process::id();
    let stale = format!(".{CARD_FILE}.{dead}.3.tmp");
    let live = format!(".{CARD_FILE}.{live_parent}.1.tmp");
    let own = format!(".{CARD_FILE}.{ours}.9.tmp");
    let unrelated = format!(".{CARD_FILE}.notapid.1.tmp");
    let other = "main.jsonl.tmp".to_owned();
    for name in [&stale, &live, &own, &unrelated, &other] {
        std::fs::write(folder.join(name), b"x").unwrap();
    }
    assert_eq!(remove_stale_card_temps(&folder), 1);
    assert!(!folder.join(&stale).exists());
    for kept in [&live, &own, &unrelated, &other] {
        assert!(folder.join(kept).exists(), "{kept} must stay");
    }
    assert_eq!(remove_stale_card_temps(&folder), 0);
}
