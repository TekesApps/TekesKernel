use schema::LifecycleFacts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LockFacts {
    pub held_by_other: bool,
    pub held_by_caller: bool,
}

impl LockFacts {
    pub const FREE: Self = Self {
        held_by_other: false,
        held_by_caller: false,
    };
    pub const CALLER: Self = Self {
        held_by_other: false,
        held_by_caller: true,
    };
    pub const OTHER: Self = Self {
        held_by_other: true,
        held_by_caller: false,
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TailState {
    Running,
    StoppedActive,
    AnsweredHold,
    ParkedHold,
    RecoveryNeeded,
    Unstarted,
    Settled,
}

impl TailState {
    /// The wire spelling of a tail state: the spec/tail-lifecycle row names.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::StoppedActive => "stopped_active",
            Self::AnsweredHold => "answered_hold",
            Self::ParkedHold => "parked_hold",
            Self::RecoveryNeeded => "recovery_needed",
            Self::Unstarted => "unstarted",
            Self::Settled => "settled",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunMode {
    Ordinary,
    Reconcile,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunDecision {
    Start(RunMode),
    ExitClean,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnsureAction {
    None,
    SpawnCandidate,
    SpawnRecoveryCandidate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeliveryAction {
    Forward,
    AppendAckNoSpawn,
    AppendAckSpawn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArchiveAction {
    Allow,
    Queue,
}

#[must_use]
pub fn classify(facts: &LifecycleFacts, locks: LockFacts) -> TailState {
    if locks.held_by_other && !locks.held_by_caller {
        return TailState::Running;
    }
    if facts.stop_active {
        TailState::StoppedActive
    } else if facts.answered_hold {
        TailState::AnsweredHold
    } else if facts.open_hold && !facts.unresolved_work {
        TailState::ParkedHold
    } else if !facts.terminal_tail
        && facts.latest_turn.is_some()
        && (facts.unresolved_work || !facts.open_hold)
    {
        TailState::RecoveryNeeded
    } else if facts.unstarted {
        TailState::Unstarted
    } else {
        TailState::Settled
    }
}

#[must_use]
pub fn ensure_action(state: TailState, facts: &LifecycleFacts) -> EnsureAction {
    ensure_action_at(state, facts, None)
}

/// `ensure_action` with a clock: a `recovery_needed` line whose only pending
/// work is a parked remote continuation or a durable provider-admission wait
/// is not spawned before that due instant (RFC 3339 UTC; lexical order).
/// Without a clock the wait counts as due.
#[must_use]
pub fn ensure_action_at(
    state: TailState,
    facts: &LifecycleFacts,
    now: Option<&str>,
) -> EnsureAction {
    match state {
        TailState::StoppedActive => EnsureAction::SpawnRecoveryCandidate,
        TailState::RecoveryNeeded
            if facts
                .durable_wait_until()
                .zip(now)
                .is_some_and(|(due, now)| due > now) =>
        {
            EnsureAction::None
        }
        TailState::AnsweredHold | TailState::RecoveryNeeded | TailState::Unstarted => {
            EnsureAction::SpawnCandidate
        }
        TailState::Settled if !facts.runnable_inputs.is_empty() => EnsureAction::SpawnCandidate,
        TailState::Settled | TailState::Running | TailState::ParkedHold => EnsureAction::None,
    }
}

#[must_use]
pub const fn delivery_action(
    state: TailState,
    matching_answer: bool,
    becomes_runnable: bool,
) -> DeliveryAction {
    match state {
        TailState::Running => DeliveryAction::Forward,
        TailState::ParkedHold if matching_answer => DeliveryAction::AppendAckSpawn,
        TailState::ParkedHold | TailState::StoppedActive | TailState::Unstarted => {
            DeliveryAction::AppendAckNoSpawn
        }
        TailState::Settled if !becomes_runnable => DeliveryAction::AppendAckNoSpawn,
        TailState::AnsweredHold | TailState::RecoveryNeeded | TailState::Settled => {
            DeliveryAction::AppendAckSpawn
        }
    }
}

#[must_use]
pub fn run_decision(state: TailState, facts: &LifecycleFacts) -> RunDecision {
    match state {
        TailState::StoppedActive => RunDecision::Start(RunMode::Reconcile),
        TailState::AnsweredHold | TailState::Unstarted => RunDecision::Start(RunMode::Ordinary),
        TailState::RecoveryNeeded => {
            // Finishing a parked remote continuation is a durable obligation of
            // the open turn, like a pending validation: never an unsolicited
            // resume governed by the spawn's resume policy.
            if !facts.runnable_inputs.is_empty()
                || facts.durable_wait_until().is_some()
                || facts.resume_policy.permits(facts.recovery_ordinal)
            {
                RunDecision::Start(RunMode::Ordinary)
            } else {
                RunDecision::Start(RunMode::Reconcile)
            }
        }
        TailState::Settled if !facts.runnable_inputs.is_empty() => {
            RunDecision::Start(RunMode::Ordinary)
        }
        TailState::Running | TailState::ParkedHold | TailState::Settled => RunDecision::ExitClean,
    }
}

#[must_use]
pub const fn archive_action(state: TailState) -> ArchiveAction {
    match state {
        TailState::ParkedHold | TailState::Settled => ArchiveAction::Allow,
        TailState::Running
        | TailState::StoppedActive
        | TailState::AnsweredHold
        | TailState::RecoveryNeeded
        | TailState::Unstarted => ArchiveAction::Queue,
    }
}
