//! Validation settlement decisions. The ledger writer owns admission and commit;
//! a provider final is only a candidate, never a settlement decision by itself.
//!
//! Mirrors TekesRuntime Docs/validation-loop-charter.md C1-C3. Snapshot coverage
//! is an exact, host-computed identity comparison, not the model's assertion.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Immutable host binding. A validator never supplies or chooses this scope.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationBinding {
    pub thread: String,
    pub turn: u64,
    pub worker: String,
    pub output_seq: u64,
    pub snapshot: BTreeMap<String, String>,
}

impl ValidationBinding {
    /// Coverage belongs to the exact candidate answer and artifact snapshot.
    /// Unchanged files cannot carry a verdict across a changed final answer.
    #[must_use]
    pub fn covered_by(&self, previous: &Self) -> bool {
        self.thread == previous.thread
            && self.turn == previous.turn
            && self.worker == previous.worker
            && self.output_seq == previous.output_seq
            && self.snapshot == previous.snapshot
    }

    /// C5 admission: even a genuine verdict for an earlier candidate must not
    /// promote a later output with identical artifact bytes.
    #[must_use]
    pub fn admits_verdict(&self, host_binding: &Self) -> bool {
        self == host_binding
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationVerdict {
    Pass,
    Fail,
    Inconclusive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationOutcome {
    NotRequired,
    Pass,
    Inconclusive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationSignal {
    Candidate {
        has_artifacts: bool,
        covering_verdict: Option<ValidationVerdict>,
    },
    Verdict(ValidationVerdict),
    ValidatorDeath,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationNegative {
    ValidatorFail,
    UnchangedStandoff,
    ValidatorDeath,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ValidationDecision {
    StandDown,
    ForkValidator,
    Feedback {
        ordinal: u64,
        reason: ValidationNegative,
    },
    Settle {
        outcome: ValidationOutcome,
        negative_round: Option<(u64, ValidationNegative)>,
    },
}

/// Call only after exact turn/output/validator provenance admission, while the
/// writer lock is held. Persist a negative round and its route or settlement in
/// one commit. Replayed signals and an occupied settlement slot are no-ops.
#[must_use]
pub fn validation_decision(
    settled: bool,
    signal_seen: bool,
    negative_rounds: u64,
    signal: ValidationSignal,
) -> ValidationDecision {
    if settled || signal_seen {
        return ValidationDecision::StandDown;
    }
    let reason = match signal {
        ValidationSignal::Candidate {
            has_artifacts: false,
            ..
        }
        | ValidationSignal::Candidate {
            covering_verdict: Some(ValidationVerdict::Pass | ValidationVerdict::Inconclusive),
            ..
        } => {
            return ValidationDecision::Settle {
                outcome: ValidationOutcome::NotRequired,
                negative_round: None,
            };
        }
        ValidationSignal::Candidate {
            covering_verdict: None,
            ..
        } => {
            return ValidationDecision::ForkValidator;
        }
        ValidationSignal::Candidate {
            covering_verdict: Some(ValidationVerdict::Fail),
            ..
        } => ValidationNegative::UnchangedStandoff,
        ValidationSignal::Verdict(ValidationVerdict::Pass) => {
            return ValidationDecision::Settle {
                outcome: ValidationOutcome::Pass,
                negative_round: None,
            };
        }
        ValidationSignal::Verdict(ValidationVerdict::Inconclusive) => {
            return ValidationDecision::Settle {
                outcome: ValidationOutcome::Inconclusive,
                negative_round: None,
            };
        }
        ValidationSignal::Verdict(ValidationVerdict::Fail) => ValidationNegative::ValidatorFail,
        ValidationSignal::ValidatorDeath => ValidationNegative::ValidatorDeath,
    };
    let ordinal = negative_rounds.saturating_add(1);
    // The legacy charter gives one repair opportunity. Death is immediately
    // terminal, still recording its negative signal in the same settlement batch.
    if ordinal >= 2 || reason == ValidationNegative::ValidatorDeath {
        ValidationDecision::Settle {
            outcome: ValidationOutcome::Inconclusive,
            negative_round: Some((ordinal, reason)),
        }
    } else {
        ValidationDecision::Feedback { ordinal, reason }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coverage_and_verdict_binding_have_different_identity_requirements() {
        let original = ValidationBinding {
            thread: "root".into(),
            turn: 7,
            worker: "worker".into(),
            output_seq: 21,
            snapshot: BTreeMap::from([("proof.txt".into(), "sha256-a".into())]),
        };
        let mut later = original.clone();
        later.output_seq = 30;
        assert!(original.covered_by(&original));
        assert!(!later.covered_by(&original));
        assert!(!later.admits_verdict(&original));
        later.snapshot.insert("proof.txt".into(), "sha256-b".into());
        assert!(!later.covered_by(&original));
        let mut other_turn = original.clone();
        other_turn.turn += 1;
        assert!(!other_turn.covered_by(&original));
        assert!(!other_turn.admits_verdict(&original));
        let mut other_worker = original.clone();
        other_worker.worker = "replacement".into();
        assert!(!other_worker.admits_verdict(&original));
    }

    #[test]
    fn candidate_requires_validation_even_when_no_judge_is_needed() {
        assert_eq!(
            validation_decision(
                false,
                false,
                0,
                ValidationSignal::Candidate {
                    has_artifacts: false,
                    covering_verdict: None,
                }
            ),
            ValidationDecision::Settle {
                outcome: ValidationOutcome::NotRequired,
                negative_round: None
            }
        );
        assert_eq!(
            validation_decision(
                false,
                false,
                0,
                ValidationSignal::Candidate {
                    has_artifacts: true,
                    covering_verdict: None,
                }
            ),
            ValidationDecision::ForkValidator
        );
    }

    #[test]
    fn failed_snapshot_is_not_accepted_or_rejudged_unchanged() {
        assert_eq!(
            validation_decision(
                false,
                false,
                1,
                ValidationSignal::Candidate {
                    has_artifacts: true,
                    covering_verdict: Some(ValidationVerdict::Fail),
                }
            ),
            ValidationDecision::Settle {
                outcome: ValidationOutcome::Inconclusive,
                negative_round: Some((2, ValidationNegative::UnchangedStandoff)),
            }
        );
    }

    #[test]
    fn fail_repairs_original_worker_once_and_never_becomes_pass_at_cap() {
        assert_eq!(
            validation_decision(
                false,
                false,
                0,
                ValidationSignal::Verdict(ValidationVerdict::Fail)
            ),
            ValidationDecision::Feedback {
                ordinal: 1,
                reason: ValidationNegative::ValidatorFail
            }
        );
        assert_eq!(
            validation_decision(
                false,
                false,
                1,
                ValidationSignal::Verdict(ValidationVerdict::Fail)
            ),
            ValidationDecision::Settle {
                outcome: ValidationOutcome::Inconclusive,
                negative_round: Some((2, ValidationNegative::ValidatorFail))
            }
        );
    }

    #[test]
    fn duplicate_and_late_signals_cannot_release_or_repair_again() {
        for signal in [
            ValidationSignal::Verdict(ValidationVerdict::Fail),
            ValidationSignal::Verdict(ValidationVerdict::Pass),
            ValidationSignal::ValidatorDeath,
        ] {
            assert_eq!(
                validation_decision(true, false, 0, signal),
                ValidationDecision::StandDown
            );
            assert_eq!(
                validation_decision(false, true, 0, signal),
                ValidationDecision::StandDown
            );
        }
    }

    #[test]
    fn direct_inconclusive_has_no_repair_round_but_death_does() {
        assert_eq!(
            validation_decision(
                false,
                false,
                0,
                ValidationSignal::Verdict(ValidationVerdict::Inconclusive)
            ),
            ValidationDecision::Settle {
                outcome: ValidationOutcome::Inconclusive,
                negative_round: None
            }
        );
        assert_eq!(
            validation_decision(false, false, 0, ValidationSignal::ValidatorDeath),
            ValidationDecision::Settle {
                outcome: ValidationOutcome::Inconclusive,
                negative_round: Some((1, ValidationNegative::ValidatorDeath))
            }
        );
    }
}
