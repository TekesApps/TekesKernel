use crate::RunMode;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Continuation {
    Stateless,
    ServerManaged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryCapability {
    None,
    Available,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdapterCapabilities {
    pub continuation: Continuation,
    pub query_by_identity: QueryCapability,
    pub dispatch_marker_required: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SentState {
    NotDispatched,
    MaybeSent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryResult {
    Outcome,
    NotFound,
    Expired,
    TransientExhausted,
    AuthFailure,
    Malformed,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryDecision {
    NotDispatched,
    Adopt,
    Resend,
    RecoveryEpoch,
    Unresolved,
}

#[must_use]
pub const fn sent_state(marker_present: bool, marker_required: bool) -> SentState {
    if marker_present || !marker_required {
        SentState::MaybeSent
    } else {
        SentState::NotDispatched
    }
}

#[must_use]
pub const fn decide_recovery(
    capabilities: AdapterCapabilities,
    state: SentState,
    query: QueryResult,
    mode: RunMode,
) -> RecoveryDecision {
    if matches!(state, SentState::NotDispatched) {
        return RecoveryDecision::NotDispatched;
    }
    if matches!(query, QueryResult::Outcome) {
        return RecoveryDecision::Adopt;
    }
    if matches!(mode, RunMode::Reconcile) {
        return RecoveryDecision::Unresolved;
    }
    match capabilities.continuation {
        Continuation::Stateless => RecoveryDecision::Resend,
        Continuation::ServerManaged => RecoveryDecision::RecoveryEpoch,
    }
}
