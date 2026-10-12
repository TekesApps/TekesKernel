//! Process-wide work counters used by boot tracing and the "reads nothing" tests.
//!
//! The counters are monotonic relaxed atomics: they never influence behaviour and
//! cost one uncontended increment per ledger line or journal row.
use std::sync::atomic::{AtomicU64, Ordering};

static LEDGER_DECODES: AtomicU64 = AtomicU64::new(0);
static LEDGER_DECODE_BYTES: AtomicU64 = AtomicU64::new(0);
static JOURNAL_PARSES: AtomicU64 = AtomicU64::new(0);
static JOURNAL_PARSE_BYTES: AtomicU64 = AtomicU64::new(0);

/// A point-in-time copy of the counters.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorkCounters {
    /// Calls to `Event::decode_canonical` (one per ledger line).
    pub ledger_decodes: u64,
    pub ledger_decode_bytes: u64,
    /// Endpoint-journal passes (one per `parse_records` call).
    pub journal_parses: u64,
    pub journal_parse_bytes: u64,
}

impl WorkCounters {
    /// Counters accumulated since `earlier`.
    #[must_use]
    pub fn since(self, earlier: Self) -> Self {
        Self {
            ledger_decodes: self.ledger_decodes.saturating_sub(earlier.ledger_decodes),
            ledger_decode_bytes: self
                .ledger_decode_bytes
                .saturating_sub(earlier.ledger_decode_bytes),
            journal_parses: self.journal_parses.saturating_sub(earlier.journal_parses),
            journal_parse_bytes: self
                .journal_parse_bytes
                .saturating_sub(earlier.journal_parse_bytes),
        }
    }
}

#[must_use]
pub fn snapshot() -> WorkCounters {
    WorkCounters {
        ledger_decodes: LEDGER_DECODES.load(Ordering::Relaxed),
        ledger_decode_bytes: LEDGER_DECODE_BYTES.load(Ordering::Relaxed),
        journal_parses: JOURNAL_PARSES.load(Ordering::Relaxed),
        journal_parse_bytes: JOURNAL_PARSE_BYTES.load(Ordering::Relaxed),
    }
}

pub(crate) fn note_ledger_decode(bytes: usize) {
    LEDGER_DECODES.fetch_add(1, Ordering::Relaxed);
    LEDGER_DECODE_BYTES.fetch_add(bytes as u64, Ordering::Relaxed);
}

/// Called by the endpoint journal once per `parse_records` pass.
pub fn note_journal_parse(bytes: usize) {
    JOURNAL_PARSES.fetch_add(1, Ordering::Relaxed);
    JOURNAL_PARSE_BYTES.fetch_add(bytes as u64, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counters_are_monotonic_and_since_subtracts() {
        let before = snapshot();
        note_ledger_decode(10);
        note_journal_parse(7);
        let delta = snapshot().since(before);
        assert!(delta.ledger_decodes >= 1 && delta.ledger_decode_bytes >= 10);
        assert!(delta.journal_parses >= 1 && delta.journal_parse_bytes >= 7);
    }
}
