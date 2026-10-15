//! P-3 out of process (G-BOOT-1, E832): the real `tekes-supervisor` reaches `ready` on a root
//! whose `threads/` and `archive/` directories cannot be listed and whose thread folders cannot
//! be opened. `sandbox-exec` cannot wrap the supervisor (it probes its own sandbox and macOS
//! refuses a nested one), so the denial is enforced by the kernel through an ACL that denies
//! `list` on both directories (mode stays 0700, which the install check requires) plus mode 000
//! on every thread folder. The first post-ready read of the directory fails with EACCES: that
//! line in the log is the positive control that the denial was in force.
//!
//! What this proves (E833): readiness does not *depend on* reading any thread. It does not prove
//! that no read was attempted: a pre-ready read whose error was swallowed would not show. The
//! boot trace therefore also has to report zero ledger decodes and journal parses at the ready
//! phase, and the first refusal has to come after the ready phase marker. The named exceptions
//! (pending operation recovery, X1-X4) have their own tests and are not part of this proof.
//!
//! Cleanup: if the test process is killed (SIGKILL) the ACL stays on the temporary directories;
//! `chmod -N <dir>/root/threads <dir>/root/archive` removes it, then `chmod 700` the folders.
#![cfg(target_os = "macos")]

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn user() -> String {
    String::from_utf8(Command::new("id").arg("-un").output().expect("id").stdout)
        .expect("utf8")
        .trim()
        .to_owned()
}

struct Restore(Vec<PathBuf>, Vec<PathBuf>);

impl Drop for Restore {
    fn drop(&mut self) {
        for directory in &self.0 {
            let _ = Command::new("chmod").arg("-N").arg(directory).status();
        }
        for folder in &self.1 {
            let _ = fs::set_permissions(folder, fs::Permissions::from_mode(0o700));
        }
    }
}

fn stub_thread(directory: &Path, name: &str) -> PathBuf {
    let folder = directory.join(name);
    fs::create_dir_all(folder.join("assets")).expect("stub thread");
    fs::write(folder.join("main.jsonl"), b"not even a ledger\n").expect("stub ledger");
    folder
}

fn ready_without_reading_threads(threads: usize, archived: usize) {
    let base = fs::canonicalize(tempfile::tempdir().expect("tmp").keep()).expect("canonical");
    let root = base.join("root");
    fs::create_dir(&root).expect("root");
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).expect("root mode");
    let mut folders = Vec::new();
    for (area, count) in [("threads", threads), ("archive", archived)] {
        let directory = root.join(area);
        fs::create_dir(&directory).expect("area");
        fs::set_permissions(&directory, fs::Permissions::from_mode(0o700)).expect("area mode");
        for n in 0..count {
            folders.push(stub_thread(
                &directory,
                &format!(
                    "018f0000-0000-7000-8000-{:012}",
                    n + if area == "archive" { 5000 } else { 0 }
                ),
            ));
        }
    }
    // the supervisor opens (never creates-after-lock) its lock file inside a directory it cannot list
    fs::write(root.join("threads/.root-lock"), b"").expect("lock file");
    fs::set_permissions(
        root.join("threads/.root-lock"),
        fs::Permissions::from_mode(0o600),
    )
    .expect("lock mode");
    let directories = vec![root.join("threads"), root.join("archive")];
    let _restore = Restore(directories.clone(), folders.clone());
    for folder in &folders {
        fs::set_permissions(folder, fs::Permissions::from_mode(0o000)).expect("lock folder");
    }
    for directory in &directories {
        let status = Command::new("chmod")
            .arg("+a")
            .arg(format!("{} deny list", user()))
            .arg(directory)
            .status()
            .expect("chmod +a");
        assert!(status.success(), "ACL could not be applied");
        assert!(
            fs::read_dir(directory).is_err(),
            "the ACL must make the directory unlistable"
        );
    }

    let supervisor = env!("CARGO_BIN_EXE_tekes-supervisor");
    let launch = base.join("launch.json");
    fs::write(
        &launch,
        serde_json::to_vec(&serde_json::json!({
            "format": 1,
            "bootstrap_version": 1,
            "root": root,
            "worker": supervisor,
            "listen": "127.0.0.1:0",
            "providers": {"format": 1, "revision": 1, "providers": []},
            "credential_bindings": {},
        }))
        .expect("launch"),
    )
    .expect("write launch");
    let mut child = Command::new(supervisor)
        .arg("--built-in")
        .arg(&launch)
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("TEKES_KERNEL_BOOT_TRACE", "1")
        .env("HOME", &base)
        .env("TMPDIR", &base)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn supervisor");
    let body =
        serde_json::to_vec(&serde_json::json!({"endpointToken":"a".repeat(64),"credentials":{}}))
            .unwrap();
    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"TEKES-KERNEL-BOOTSTRAP/1\n").unwrap();
    stdin.write_all(&(body.len() as u32).to_be_bytes()).unwrap();
    stdin.write_all(&body).unwrap();
    stdin.flush().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().expect("stdout"));
    let (line_tx, line_rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut line = String::new();
        let _ = stdout.read_line(&mut line);
        let _ = line_tx.send(line);
    });
    let mut stderr = child.stderr.take().expect("stderr");
    let (log_tx, log_rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut text = String::new();
        let mut buffer = [0_u8; 4096];
        while let Ok(count) = stderr.read(&mut buffer) {
            if count == 0 {
                break;
            }
            text.push_str(&String::from_utf8_lossy(&buffer[..count]));
            let _ = log_tx.send(text.clone());
        }
    });
    let started = Instant::now();
    let ready = line_rx
        .recv_timeout(Duration::from_secs(60))
        .unwrap_or_else(|_| {
            let _ = child.kill();
            panic!("no ready line within 60 s")
        });
    let elapsed = started.elapsed();
    assert!(ready.contains(r#""type":"ready""#), "ready line: {ready}");
    // Positive control: the first read of the directory after ready is refused by the kernel.
    let mut log = String::new();
    let deadline = Instant::now() + Duration::from_secs(30);
    while !log.contains("Permission denied") && Instant::now() < deadline {
        if let Ok(text) = log_rx.recv_timeout(Duration::from_millis(200)) {
            log = text;
        }
    }
    drop(child.stdin.take());
    let _ = child.wait();
    assert!(
        log.contains("Permission denied"),
        "the ACL denial was never exercised; stderr: {log}"
    );
    assert!(!log.contains("panicked"), "supervisor panicked: {log}");
    let ready_phase = log
        .find("boot-trace: phase=ready ")
        .unwrap_or_else(|| panic!("no ready phase marker in the trace: {log}"));
    let ready_line = &log[ready_phase
        ..log[ready_phase..]
            .find('\n')
            .map_or(log.len(), |end| ready_phase + end)];
    assert!(
        ready_line.contains("ledger_decodes=0") && ready_line.contains("journal_parses=0"),
        "work done before ready: {ready_line}"
    );
    let first_refusal = log.find("Permission denied").expect("checked above");
    assert!(
        first_refusal > ready_phase,
        "a thread path was refused before the ready phase: {log}"
    );
    eprintln!("ready after {elapsed:?} with {threads} threads / {archived} archived");
    drop(_restore);
    let _ = fs::remove_dir_all(&base);
}

#[test]
fn the_real_supervisor_is_ready_without_listing_or_opening_any_thread() {
    ready_without_reading_threads(40, 10);
}
