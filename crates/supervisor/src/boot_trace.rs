//! Opt-in boot instrumentation (G-BOOT-1, slice S0). No behaviour change.
//!
//! * `TEKES_KERNEL_BOOT_TRACE=1` prints one stderr line per boot phase with the elapsed time
//!   and the ledger/journal work counters (`schema::metrics`) accumulated since process start.
//! * `TEKES_KERNEL_BOOT_PROGRESS=1` additionally prints `{"type":"boot",...}` records on stdout
//!   before the `ready` record. It is gated because a client that treats the first stdout line as
//!   the ready record would misread it; only a client that opts in sets it.
//!
//! With neither variable set every call is a no-op.
use std::io::Write;
use std::sync::OnceLock;
use std::time::Instant;

struct Trace {
    started: Instant,
    base: schema::metrics::WorkCounters,
    stderr: bool,
    stdout: bool,
}

static TRACE: OnceLock<Trace> = OnceLock::new();

fn flag(name: &str) -> bool {
    std::env::var_os(name).is_some_and(|value| value == "1")
}

/// Starts the clock. Idempotent; only the first call counts.
pub fn start() {
    let _ = TRACE.set(Trace {
        started: Instant::now(),
        base: schema::metrics::snapshot(),
        stderr: flag("TEKES_KERNEL_BOOT_TRACE"),
        stdout: flag("TEKES_KERNEL_BOOT_PROGRESS"),
    });
}

/// Formats one trace line (also used by tests).
#[must_use]
pub fn format_line(
    phase: &str,
    elapsed_ms: u128,
    work: schema::metrics::WorkCounters,
    detail: &str,
) -> String {
    let mut line = format!(
        "boot-trace: phase={phase} t_ms={elapsed_ms} ledger_decodes={} ledger_decode_bytes={} journal_parses={} journal_parse_bytes={}",
        work.ledger_decodes,
        work.ledger_decode_bytes,
        work.journal_parses,
        work.journal_parse_bytes
    );
    if !detail.is_empty() {
        line.push(' ');
        line.push_str(detail);
    }
    line
}

/// Records the end of a boot phase.
pub fn phase(name: &str) {
    detail(name, "");
}

/// Records the end of a boot phase with extra `key=value` text.
pub fn detail(name: &str, detail: &str) {
    let Some(trace) = TRACE.get() else {
        return;
    };
    if !trace.stderr && !trace.stdout {
        return;
    }
    let elapsed = trace.started.elapsed().as_millis();
    let work = schema::metrics::snapshot().since(trace.base);
    if trace.stderr {
        eprintln!("{}", format_line(name, elapsed, work, detail));
    }
    if trace.stdout {
        let record = serde_json::json!({
            "type": "boot",
            "phase": name,
            "elapsedMs": elapsed,
            "ledgerDecodes": work.ledger_decodes,
            "journalParses": work.journal_parses,
        });
        let mut out = std::io::stdout().lock();
        let _ = writeln!(out, "{record}");
        let _ = out.flush();
    }
}

/// True when per-item tracing (for example one line per swept session) is wanted.
#[must_use]
pub fn per_item_enabled() -> bool {
    TRACE.get().is_some_and(|trace| trace.stderr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_names_phase_time_and_work() {
        let work = schema::metrics::WorkCounters {
            ledger_decodes: 3,
            ledger_decode_bytes: 90,
            journal_parses: 2,
            journal_parse_bytes: 50,
        };
        let line = format_line("boot_sweep", 1234, work, "threads=60");
        assert_eq!(
            line,
            "boot-trace: phase=boot_sweep t_ms=1234 ledger_decodes=3 ledger_decode_bytes=90 journal_parses=2 journal_parse_bytes=50 threads=60"
        );
    }

    #[test]
    fn phase_without_start_is_a_no_op() {
        // `start` may or may not have run in this process; both paths must not panic.
        phase("noop");
        detail("noop", "k=v");
    }
}
