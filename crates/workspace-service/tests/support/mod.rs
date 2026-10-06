//! Helpers shared by the workspace-service process tests.

use std::process::Command;

/// Run a freshly written fixture script once, untimed, before a timed test uses
/// it. macOS assesses a new executable on its first exec and serializes those
/// assessments (about 0.3 s each; 32 concurrent first runs took up to 8.5 s), so
/// under a full parallel test run the first exec alone could outlast a 2-5 s
/// bound. Later execs of the same file start in milliseconds.
///
/// On Linux the first exec can also fail with ETXTBSY ("Text file busy"): while
/// this thread wrote the script, another test thread forked a child, and that
/// child holds the write fd until it execs (O_CLOEXEC closes it only at exec).
/// Retry until the inherited fd is gone. Nothing reopens the script for
/// writing, so once an exec succeeds the timed exec that follows cannot hit it.
#[cfg(unix)]
pub fn warm_first_exec(script: &std::path::Path) {
    let mut attempt = 0;
    let status = loop {
        match Command::new(script).arg("--warm").status() {
            Err(error)
                if error.kind() == std::io::ErrorKind::ExecutableFileBusy && attempt < 50 =>
            {
                attempt += 1;
                std::thread::sleep(std::time::Duration::from_millis(10 * attempt.min(10)));
            }
            result => break result.unwrap(),
        }
    };
    assert!(status.success(), "fixture script failed its warm-up run");
}
