//! Worker-side adapter for fixed supervisor-owned tool transports.

use schema::IJsonValue;
use tools::{BackendTerminal, ToolExecution};
use worker_control::{ToolControl, ToolControlErrorCode, ToolControlResult};

use crate::ToolBackend;

pub struct SupervisorControlBackend<F> {
    session: String,
    exchange: F,
}

impl<F> SupervisorControlBackend<F> {
    #[must_use]
    pub fn new(session: impl Into<String>, exchange: F) -> Self {
        Self {
            session: session.into(),
            exchange,
        }
    }
}

impl<F> ToolBackend for SupervisorControlBackend<F>
where
    F: FnMut(&ToolControl) -> Result<ToolControlResult, String>,
{
    fn supports(&self, name: &str) -> bool {
        matches!(
            name,
            "context" | "context_get" | "job" | "report" | "subagent" | "task"
        )
    }

    fn execute(&mut self, execution: &ToolExecution, invocation: &IJsonValue) -> BackendTerminal {
        let request = match ToolControl::new(
            self.session.clone(),
            execution.thread.clone(),
            execution.turn,
            execution.call.clone(),
            execution.name.clone(),
            invocation.clone(),
        ) {
            Ok(request) => request,
            Err(error) => return unavailable("protocol", error.to_string(), false),
        };
        let result = match (self.exchange)(&request) {
            Ok(result) => result,
            Err(message) => return unavailable("transport", message, true),
        };
        if let Err(error) = result.validate_for(&request) {
            return unavailable("protocol", error.to_string(), false);
        }
        match (result.value, result.error) {
            (Some(value), None) => BackendTerminal::Completed(value),
            (None, Some(error)) => unavailable(
                control_error_code(error.code),
                error.message,
                error.retryable,
            ),
            _ => unavailable("protocol", "invalid tool-control result union", false),
        }
    }
}

const fn control_error_code(code: ToolControlErrorCode) -> &'static str {
    match code {
        ToolControlErrorCode::Unsupported => "unsupported",
        ToolControlErrorCode::Denied => "denied",
        ToolControlErrorCode::NotFound => "not_found",
        ToolControlErrorCode::Conflict => "conflict",
        ToolControlErrorCode::Unavailable => "unavailable",
        ToolControlErrorCode::Timeout => "timeout",
        ToolControlErrorCode::EffectUnknown => "effect_unknown",
        ToolControlErrorCode::EffectConflicted => "effect_conflicted",
        ToolControlErrorCode::Internal => "internal",
    }
}

fn unavailable(code: &str, message: impl Into<String>, retryable: bool) -> BackendTerminal {
    BackendTerminal::Unavailable {
        code: code.to_owned(),
        message: message.into(),
        retryable,
    }
}
