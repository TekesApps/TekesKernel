use std::collections::{HashMap, VecDeque};

use schema::IJsonValue;
use tools::{BackendTerminal, DurableApprovalResponse, ToolExecution};

use crate::{AdapterCapabilities, QueryResult};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderTerminal {
    pub response_identity: String,
    pub raw_response: Vec<u8>,
    pub input_tokens: Option<String>,
    pub output_tokens: Option<String>,
}

pub trait ProviderAdapter {
    fn capabilities(&self) -> AdapterCapabilities;
    fn render(&self, prefix_digest: &str, epoch_profile: &str) -> Vec<u8>;
    fn query(&mut self, attempt: &str) -> QueryResult;
    fn adopted_response(&self, attempt: &str) -> Option<&ProviderTerminal>;
}

#[derive(Clone, Debug)]
pub struct FakeProvider {
    capabilities: AdapterCapabilities,
    query_results: VecDeque<QueryResult>,
    responses: HashMap<String, ProviderTerminal>,
}

impl FakeProvider {
    #[must_use]
    pub fn new(capabilities: AdapterCapabilities) -> Self {
        Self {
            capabilities,
            query_results: VecDeque::new(),
            responses: HashMap::new(),
        }
    }

    pub fn push_query_result(&mut self, result: QueryResult) {
        self.query_results.push_back(result);
    }

    pub fn insert_response(&mut self, attempt: impl Into<String>, response: ProviderTerminal) {
        self.responses.insert(attempt.into(), response);
    }
}

impl ProviderAdapter for FakeProvider {
    fn capabilities(&self) -> AdapterCapabilities {
        self.capabilities
    }

    fn render(&self, prefix_digest: &str, epoch_profile: &str) -> Vec<u8> {
        format!("prefix={prefix_digest};epoch={epoch_profile}").into_bytes()
    }

    fn query(&mut self, _attempt: &str) -> QueryResult {
        self.query_results
            .pop_front()
            .unwrap_or(QueryResult::Unsupported)
    }

    fn adopted_response(&self, attempt: &str) -> Option<&ProviderTerminal> {
        self.responses.get(attempt)
    }
}

pub trait ToolBackend {
    fn supports(&self, _name: &str) -> bool {
        true
    }

    fn execute(&mut self, execution: &ToolExecution, invocation: &IJsonValue) -> BackendTerminal;

    /// Continue a backend after a matching durable approval response.
    ///
    /// Policy-gated tools have not reached `execute` yet, so the safe generic
    /// behavior is to execute the bound invocation once.  Worker-hold
    /// backends override this to consume the answer instead of fabricating a
    /// result or reissuing their hold.
    fn resume_after_approval(
        &mut self,
        execution: &ToolExecution,
        invocation: &IJsonValue,
        _approval: &DurableApprovalResponse,
    ) -> BackendTerminal {
        self.execute(execution, invocation)
    }
}

#[derive(Default)]
pub struct ToolBackendRouter<'a> {
    backends: Vec<Box<dyn ToolBackend + 'a>>,
}

impl<'a> ToolBackendRouter<'a> {
    #[must_use]
    pub fn new(backends: Vec<Box<dyn ToolBackend + 'a>>) -> Self {
        Self { backends }
    }

    pub fn push(&mut self, backend: impl ToolBackend + 'a) {
        self.backends.push(Box::new(backend));
    }
}

impl ToolBackend for ToolBackendRouter<'_> {
    fn supports(&self, name: &str) -> bool {
        self.backends.iter().any(|backend| backend.supports(name))
    }

    fn execute(&mut self, execution: &ToolExecution, invocation: &IJsonValue) -> BackendTerminal {
        let Some(backend) = self
            .backends
            .iter_mut()
            .find(|backend| backend.supports(&execution.name))
        else {
            return BackendTerminal::Unavailable {
                code: "unsupported".to_owned(),
                message: format!("no backend registered for {}", execution.name),
                retryable: false,
            };
        };
        backend.execute(execution, invocation)
    }

    fn resume_after_approval(
        &mut self,
        execution: &ToolExecution,
        invocation: &IJsonValue,
        approval: &DurableApprovalResponse,
    ) -> BackendTerminal {
        let Some(backend) = self
            .backends
            .iter_mut()
            .find(|backend| backend.supports(&execution.name))
        else {
            return BackendTerminal::Unavailable {
                code: "unsupported".to_owned(),
                message: format!("no backend registered for {}", execution.name),
                retryable: false,
            };
        };
        backend.resume_after_approval(execution, invocation, approval)
    }
}

#[derive(Clone, Debug, Default)]
pub struct FakeToolBackend {
    outcomes: HashMap<String, BackendTerminal>,
    executions: HashMap<String, usize>,
}

impl FakeToolBackend {
    pub fn set_outcome(&mut self, call: impl Into<String>, outcome: Result<IJsonValue, String>) {
        self.outcomes.insert(
            call.into(),
            match outcome {
                Ok(value) => BackendTerminal::Completed(value),
                Err(message) => BackendTerminal::Unavailable {
                    code: "fake_error".to_owned(),
                    message,
                    retryable: false,
                },
            },
        );
    }

    pub fn set_terminal(&mut self, call: impl Into<String>, terminal: BackendTerminal) {
        self.outcomes.insert(call.into(), terminal);
    }

    #[must_use]
    pub fn execution_count(&self, call: &str) -> usize {
        self.executions.get(call).copied().unwrap_or(0)
    }
}

impl ToolBackend for FakeToolBackend {
    fn execute(&mut self, execution: &ToolExecution, _invocation: &IJsonValue) -> BackendTerminal {
        *self.executions.entry(execution.call.clone()).or_default() += 1;
        self.outcomes
            .get(&execution.call)
            .cloned()
            .unwrap_or_else(|| BackendTerminal::Unavailable {
                code: "unconfigured".to_owned(),
                message: "unconfigured fake tool".to_owned(),
                retryable: false,
            })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessState {
    Running,
    Signaled,
    Exited(i32),
}

pub trait ProcessHost {
    fn spawn(&mut self, key: &str) -> Result<(), String>;
    fn signal(&mut self, key: &str) -> Result<(), String>;
    fn wait(&mut self, key: &str) -> Result<i32, String>;
    fn state(&self, key: &str) -> Option<ProcessState>;
}

#[derive(Clone, Debug, Default)]
pub struct FakeProcessHost {
    processes: HashMap<String, ProcessState>,
}

impl FakeProcessHost {
    pub fn exit(&mut self, key: &str, status: i32) {
        self.processes
            .insert(key.to_owned(), ProcessState::Exited(status));
    }
}

impl ProcessHost for FakeProcessHost {
    fn spawn(&mut self, key: &str) -> Result<(), String> {
        self.processes
            .entry(key.to_owned())
            .or_insert(ProcessState::Running);
        Ok(())
    }

    fn signal(&mut self, key: &str) -> Result<(), String> {
        match self.processes.get_mut(key) {
            Some(state @ ProcessState::Running) => {
                *state = ProcessState::Signaled;
                Ok(())
            }
            Some(ProcessState::Signaled | ProcessState::Exited(_)) => Ok(()),
            None => Err(format!("unknown process {key}")),
        }
    }

    fn wait(&mut self, key: &str) -> Result<i32, String> {
        match self.processes.get(key) {
            Some(ProcessState::Exited(status)) => Ok(*status),
            Some(ProcessState::Running | ProcessState::Signaled) => {
                Err(format!("process {key} is still live"))
            }
            None => Err(format!("unknown process {key}")),
        }
    }

    fn state(&self, key: &str) -> Option<ProcessState> {
        self.processes.get(key).copied()
    }
}
