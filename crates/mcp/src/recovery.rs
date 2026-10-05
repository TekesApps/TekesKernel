#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpLossState {
    Handshake,
    Catalog,
    ReadBeforeResponse,
    ToolWithoutProof,
    TaskMutationWithoutProof,
    ResumableOperation,
    AuthorityChanged,
    StdioExit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryAction {
    ReconnectOnce,
    UnknownEffect,
    QueryIdentity,
    FreshGeneration,
    FailAndRestartOnDemand,
}

#[must_use]
pub const fn recovery_action(state: McpLossState) -> RecoveryAction {
    match state {
        McpLossState::Handshake | McpLossState::Catalog | McpLossState::ReadBeforeResponse => {
            RecoveryAction::ReconnectOnce
        }
        McpLossState::ToolWithoutProof | McpLossState::TaskMutationWithoutProof => {
            RecoveryAction::UnknownEffect
        }
        McpLossState::ResumableOperation => RecoveryAction::QueryIdentity,
        McpLossState::AuthorityChanged => RecoveryAction::FreshGeneration,
        McpLossState::StdioExit => RecoveryAction::FailAndRestartOnDemand,
    }
}
