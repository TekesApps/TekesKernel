use schema::{Event, IJsonValue};
use serde_json::{Map, Value, json};
use store::{AssetStore, BarrierContext, LockedLedger, StoreError};
use thiserror::Error;

use crate::hook::{
    HookBinding, HookFailureMode, HookPhase, HookRequest, HookResponse, HookResultView,
    PostVerdict, PreVerdict, ProcessHook,
};

const SPILL_THRESHOLD: usize = 16_384;

#[derive(Clone, Debug, PartialEq)]
pub struct ToolExecution {
    pub thread: String,
    pub call: String,
    pub name: String,
    pub attempt: String,
    pub invocation: IJsonValue,
    pub side_effectful: bool,
    pub turn: u64,
    pub timestamp: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BackendTerminal {
    Completed(IJsonValue),
    /// The durable write-ahead/policy half completed, but the worker must
    /// finish a worker-owned protocol transaction before a terminal result
    /// can be authored.  This is used by child spawning: child genesis,
    /// parent `spawn`, launch/wait, and `child_result` all precede the paired
    /// `tool_result`.
    Deferred,
    Hold {
        scope: String,
        question: IJsonValue,
    },
    /// The supervisor bound a remote continuation (an MCP task) for this call.
    /// The pipeline records the pending step durably; the worker drives the
    /// continuation to exactly one terminal `tool_result` later.
    Pending {
        continuation_id: String,
        state: IJsonValue,
    },
    Unavailable {
        code: String,
        message: String,
        retryable: bool,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ApprovalGate {
    Allow,
    Deny(String),
    Hold { scope: String, question: IJsonValue },
}

/// A durable response paired to the approval request for one tool call.
///
/// Backends receive this only on the resume path.  The pipeline has already
/// checked the call/turn/scope binding and a denial never reaches a backend.
#[derive(Clone, Debug, PartialEq)]
pub struct DurableApprovalResponse {
    pub request_seq: u64,
    pub response_seq: u64,
    pub scope: String,
    pub answer: Option<IJsonValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PipelineDecision {
    Completed {
        result_seq: u64,
    },
    Parked {
        request_seq: u64,
    },
    Deferred,
    /// A durable `state{subkind: tool_continuation}` records the remote
    /// continuation; no `tool_result` exists yet.
    Pending {
        continuation_id: String,
        state_seq: u64,
    },
}

/// The `state` subkind that records one durable step of a remote tool
/// continuation for a call. Runtime-visible only: it is host bookkeeping, not
/// model context.
pub const TOOL_CONTINUATION_SUBKIND: &str = "tool_continuation";

#[derive(Clone, Debug, PartialEq)]
pub enum SecretScan {
    Clean(IJsonValue),
    Redacted(IJsonValue),
    Withheld(&'static str),
}

#[derive(Clone, Debug, Default)]
pub struct SecretScanner {
    force_failure: bool,
}

impl SecretScanner {
    #[must_use]
    pub const fn failing() -> Self {
        Self {
            force_failure: true,
        }
    }

    pub fn scan(&self, value: &IJsonValue) -> SecretScan {
        if self.force_failure {
            return SecretScan::Withheld("scan_failed");
        }
        let mut value = serde_json::to_value(value).expect("IJsonValue is serializable");
        let redacted = redact_value(&mut value);
        let encoded = serde_json::to_vec(&value).expect("JSON value is serializable");
        let value = IJsonValue::parse(&encoded).expect("redacted JSON remains I-JSON");
        if redacted {
            SecretScan::Redacted(value)
        } else {
            SecretScan::Clean(value)
        }
    }
}

pub struct ToolPipeline<'a> {
    ledger: &'a mut LockedLedger,
    assets: AssetStore,
    scanner: SecretScanner,
}

impl<'a> ToolPipeline<'a> {
    #[must_use]
    pub fn new(ledger: &'a mut LockedLedger, assets: AssetStore, scanner: SecretScanner) -> Self {
        Self {
            ledger,
            assets,
            scanner,
        }
    }

    pub fn execute<F>(
        &mut self,
        execution: &ToolExecution,
        hooks: &[HookBinding],
        mut backend: F,
    ) -> Result<PipelineDecision, ToolPipelineError>
    where
        F: FnMut(&str, &IJsonValue) -> Result<IJsonValue, String>,
    {
        self.execute_terminal(execution, hooks, |execution, invocation| {
            match backend(&execution.call, invocation) {
                Ok(value) => BackendTerminal::Completed(value),
                Err(message) => BackendTerminal::Unavailable {
                    code: "backend_error".to_owned(),
                    message,
                    retryable: false,
                },
            }
        })
    }

    pub fn verify_durable_execution(
        &self,
        execution: &ToolExecution,
    ) -> Result<(), ToolPipelineError> {
        let projection = self
            .ledger
            .projection()
            .ok_or(ToolPipelineError::DurableBinding("ledger is empty"))?;
        let thread = projection
            .events
            .first()
            .and_then(|event| event.string_field("thread"))
            .ok_or(ToolPipelineError::DurableBinding(
                "genesis thread binding is missing",
            ))?;
        if thread != execution.thread {
            return Err(ToolPipelineError::DurableBinding(
                "execution thread does not match genesis",
            ));
        }
        let call = projection
            .events
            .iter()
            .find(|event| {
                *event.kind() == schema::EventKind::ToolCall
                    && event.string_field("call") == Some(execution.call.as_str())
            })
            .ok_or(ToolPipelineError::DurableBinding(
                "tool_call is not durable",
            ))?;
        if call.turn() != Some(execution.turn)
            || call.string_field("attempt") != Some(execution.attempt.as_str())
            || call.string_field("name") != Some(execution.name.as_str())
        {
            return Err(ToolPipelineError::DurableBinding(
                "execution tuple does not match tool_call",
            ));
        }
        let raw = serde_json::to_value(call.raw())
            .map_err(|error| ToolPipelineError::Json(error.to_string()))?;
        let args = raw
            .get("args")
            .ok_or(ToolPipelineError::DurableBinding("tool_call args missing"))?;
        let durable = if let Some(spill) = args.get("$spill") {
            let asset = spill.get("asset").and_then(Value::as_str).ok_or(
                ToolPipelineError::DurableBinding("tool_call spill asset missing"),
            )?;
            let bytes = self.assets.read_verified(asset)?;
            IJsonValue::parse(&bytes)?
        } else {
            value_to_ijson(args.clone())?
        };
        if durable != execution.invocation {
            return Err(ToolPipelineError::DurableBinding(
                "execution arguments do not match tool_call",
            ));
        }
        Ok(())
    }

    /// Proves a deferred item was named by a successful discovery result in
    /// the same turn before the consuming call. The ledger result is the
    /// authority; no process-local cache or side log participates.
    pub fn verify_causal_offer(
        &self,
        execution: &ToolExecution,
        source_tool: &str,
        offered_name: &str,
    ) -> Result<(), ToolPipelineError> {
        let projection = self
            .ledger
            .projection()
            .ok_or(ToolPipelineError::DurableBinding("ledger is empty"))?;
        let consuming_seq = projection
            .events
            .iter()
            .find(|event| {
                *event.kind() == schema::EventKind::ToolCall
                    && event.string_field("call") == Some(execution.call.as_str())
            })
            .map(schema::Event::seq)
            .ok_or(ToolPipelineError::DurableBinding(
                "consuming tool_call is not durable",
            ))?;

        if self.has_causal_offer(consuming_seq, source_tool, offered_name)? {
            return Ok(());
        }
        Err(ToolPipelineError::DurableBinding(
            "requested item was not causally offered by a model-visible search result",
        ))
    }

    /// Read-only projection shared by provider schema exposure and dispatch.
    /// `before_seq` prevents later facts from authorizing an earlier call.
    /// Whether a successful `source_tool` result before `before_seq` offered
    /// `offered_name` and is still model-visible: not superseded and not
    /// covered by a `compact`. An offer lives as long as the model can see
    /// the result that made it — across turns within the same provider
    /// conversation — so the declared tool catalog only grows between
    /// compactions instead of changing on every turn boundary.
    pub fn has_causal_offer(
        &self,
        before_seq: u64,
        source_tool: &str,
        offered_name: &str,
    ) -> Result<bool, ToolPipelineError> {
        let projection = self
            .ledger
            .projection()
            .ok_or(ToolPipelineError::DurableBinding("ledger is empty"))?;
        // Only facts before `before_seq` can hide an offer, just as only
        // facts before it can make one.
        let mut hidden = std::collections::BTreeSet::new();
        for event in projection
            .events
            .iter()
            .filter(|event| event.seq() < before_seq)
        {
            let raw = serde_json::to_value(event.raw())
                .map_err(|error| ToolPipelineError::Json(error.to_string()))?;
            for field in ["supersedes", "covers"] {
                if field == "covers" && *event.kind() != schema::EventKind::Compact {
                    continue;
                }
                for range in raw
                    .get(field)
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                {
                    if let (Some(from), Some(to)) = (
                        range.get("from").and_then(Value::as_u64),
                        range.get("to").and_then(Value::as_u64),
                    ) {
                        hidden.extend(from..=to);
                    }
                }
            }
        }
        let source_calls = projection
            .events
            .iter()
            .filter(|event| {
                event.seq() < before_seq
                    && *event.kind() == schema::EventKind::ToolCall
                    && event.string_field("name") == Some(source_tool)
            })
            .filter_map(|event| event.string_field("call"))
            .collect::<std::collections::BTreeSet<_>>();

        for result in projection.events.iter().filter(|event| {
            event.seq() < before_seq
                && !hidden.contains(&event.seq())
                && *event.kind() == schema::EventKind::ToolResult
                && event
                    .string_field("call")
                    .is_some_and(|call| source_calls.contains(call))
        }) {
            let raw = serde_json::to_value(result.raw())
                .map_err(|error| ToolPipelineError::Json(error.to_string()))?;
            if raw.get("outcome").and_then(Value::as_str) != Some("ok") {
                continue;
            }
            let Some(content) = raw.get("content") else {
                continue;
            };
            let content = self.materialize_json_or_spill(content)?;
            if offer_content_names(&content, offered_name) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn materialize_json_or_spill(&self, value: &Value) -> Result<Value, ToolPipelineError> {
        if let Some(spill) = value.get("$spill") {
            let asset = spill.get("asset").and_then(Value::as_str).ok_or(
                ToolPipelineError::DurableBinding("causal offer spill asset is missing"),
            )?;
            let bytes = self.assets.read_verified(asset)?;
            return serde_json::from_slice(&bytes)
                .map_err(|error| ToolPipelineError::Json(error.to_string()));
        }
        Ok(value.clone())
    }

    pub fn execute_terminal<F>(
        &mut self,
        execution: &ToolExecution,
        hooks: &[HookBinding],
        backend: F,
    ) -> Result<PipelineDecision, ToolPipelineError>
    where
        F: FnMut(&ToolExecution, &IJsonValue) -> BackendTerminal,
    {
        self.execute_terminal_with_gate(execution, hooks, |_, _| ApprovalGate::Allow, backend)
    }

    pub fn execute_terminal_with_gate<F, G>(
        &mut self,
        execution: &ToolExecution,
        hooks: &[HookBinding],
        gate: G,
        mut backend: F,
    ) -> Result<PipelineDecision, ToolPipelineError>
    where
        F: FnMut(&ToolExecution, &IJsonValue) -> BackendTerminal,
        G: FnMut(&ToolExecution, &IJsonValue) -> ApprovalGate,
    {
        self.execute_terminal_with_gate_and_resume(
            execution,
            hooks,
            gate,
            |execution, invocation, _approval| backend(execution, invocation),
        )
    }

    /// Executes a new invocation or resumes one after its durable approval.
    ///
    /// Resumption never re-runs pre-hooks or the policy gate.  It uses the
    /// durable `effective_execution` when present, otherwise the original
    /// durable `tool_call` arguments.  A matching denial is terminalized
    /// without calling the backend, while an already terminal call is simply
    /// re-acknowledged by returning its existing result sequence.
    pub fn execute_terminal_with_gate_and_resume<F, G>(
        &mut self,
        execution: &ToolExecution,
        hooks: &[HookBinding],
        mut gate: G,
        mut backend: F,
    ) -> Result<PipelineDecision, ToolPipelineError>
    where
        F: FnMut(&ToolExecution, &IJsonValue, Option<&DurableApprovalResponse>) -> BackendTerminal,
        G: FnMut(&ToolExecution, &IJsonValue) -> ApprovalGate,
    {
        match self.durable_approval_state(execution)? {
            DurableApprovalState::None => {}
            DurableApprovalState::Completed { result_seq } => {
                return Ok(PipelineDecision::Completed { result_seq });
            }
            DurableApprovalState::Pending { request_seq } => {
                return Ok(PipelineDecision::Parked { request_seq });
            }
            DurableApprovalState::Denied { reason } => {
                let seq =
                    self.append_result(execution, json!({"denied": reason}), None, Map::new())?;
                return Ok(PipelineDecision::Completed { result_seq: seq });
            }
            DurableApprovalState::Granted {
                invocation,
                response,
            } => {
                self.append_execution_started(execution)?;
                let terminal = backend(execution, &invocation, Some(&response));
                return self.complete_terminal(execution, hooks, &invocation, Map::new(), terminal);
            }
        }

        let mut invocation = execution.invocation.clone();
        let mut metadata = Map::new();
        let mut mutated = false;
        for binding in hooks
            .iter()
            .filter(|binding| binding.phase == HookPhase::Pre)
        {
            let request = HookRequest {
                format: 1,
                hook_id: binding.id.clone(),
                phase: HookPhase::Pre,
                call: execution.call.clone(),
                name: execution.name.clone(),
                invocation: invocation.clone(),
                result: None,
            };
            match ProcessHook::run(binding, &request) {
                Ok(HookResponse::Pre { verdict, .. }) => match verdict {
                    PreVerdict::Allow {} => {}
                    PreVerdict::Deny { reason } => {
                        let seq = self.append_result(
                            execution,
                            json!({"denied": reason}),
                            None,
                            metadata,
                        )?;
                        return Ok(PipelineDecision::Completed { result_seq: seq });
                    }
                    PreVerdict::Mutate {
                        invocation: effective,
                        reason,
                    } => {
                        invocation = effective;
                        mutated = true;
                        record_pre_mutation(&mut metadata, &binding.id, reason.as_deref());
                    }
                    PreVerdict::Ask { scope, question } => {
                        let seq = self.append_approval(execution, &scope, &question)?;
                        return Ok(PipelineDecision::Parked { request_seq: seq });
                    }
                },
                Ok(HookResponse::Post { .. }) => return Err(ToolPipelineError::HookPhase),
                Err(error) => match binding.failure {
                    HookFailureMode::Closed => {
                        let seq = self.append_result(
                            execution,
                            json!({"denied": format!("hook_failure:{}", binding.id)}),
                            None,
                            hook_failure_meta(&binding.id, &error.to_string()),
                        )?;
                        return Ok(PipelineDecision::Completed { result_seq: seq });
                    }
                    HookFailureMode::Open => {
                        record_hook_failure(&mut metadata, &binding.id, &error.to_string())
                    }
                },
            }
        }

        if mutated {
            self.append_effective_execution(execution, &invocation)?;
        }

        match gate(execution, &invocation) {
            ApprovalGate::Allow => {}
            ApprovalGate::Deny(reason) => {
                let seq =
                    self.append_result(execution, json!({"denied": reason}), None, metadata)?;
                return Ok(PipelineDecision::Completed { result_seq: seq });
            }
            ApprovalGate::Hold { scope, question } => {
                let seq = self.append_approval(execution, &scope, &question)?;
                return Ok(PipelineDecision::Parked { request_seq: seq });
            }
        }

        self.append_execution_started(execution)?;
        let terminal = backend(execution, &invocation, None);
        self.complete_terminal(execution, hooks, &invocation, metadata, terminal)
    }

    fn complete_terminal(
        &mut self,
        execution: &ToolExecution,
        hooks: &[HookBinding],
        invocation: &IJsonValue,
        mut metadata: Map<String, Value>,
        terminal: BackendTerminal,
    ) -> Result<PipelineDecision, ToolPipelineError> {
        let (backend_outcome, backend_value) = match terminal {
            BackendTerminal::Completed(value) => (Value::String("ok".to_owned()), value),
            BackendTerminal::Deferred => return Ok(PipelineDecision::Deferred),
            BackendTerminal::Unavailable {
                code,
                message,
                retryable,
            } => (
                Value::String("error".to_owned()),
                value_to_ijson(json!({
                    "code": code,
                    "message": message,
                    "retryable": retryable,
                }))?,
            ),
            BackendTerminal::Hold { scope, question } => {
                let seq = self.append_approval(execution, &scope, &question)?;
                return Ok(PipelineDecision::Parked { request_seq: seq });
            }
            BackendTerminal::Pending {
                continuation_id,
                state,
            } => {
                let seq = self.append_continuation_step(
                    execution,
                    &continuation_id,
                    0,
                    "bind",
                    &state,
                    None,
                )?;
                return Ok(PipelineDecision::Pending {
                    continuation_id,
                    state_seq: seq,
                });
            }
        };

        let transformed = backend_value;
        let mut forced_withheld = None;

        for binding in hooks
            .iter()
            .filter(|binding| binding.phase == HookPhase::Post)
        {
            let request = HookRequest {
                format: 1,
                hook_id: binding.id.clone(),
                phase: HookPhase::Post,
                call: execution.call.clone(),
                name: execution.name.clone(),
                invocation: invocation.clone(),
                result: Some(HookResultView {
                    outcome: value_to_ijson(backend_outcome.clone())?,
                    content: Some(transformed.clone()),
                    meta: (!metadata.is_empty())
                        .then(|| value_to_ijson(Value::Object(metadata.clone())))
                        .transpose()?,
                }),
            };
            match ProcessHook::run(binding, &request) {
                Ok(HookResponse::Post { verdict, .. }) => match verdict {
                    PostVerdict::Allow { annotations } => {
                        record_post_hook(&mut metadata, &binding.id, None, annotations.as_ref());
                    }
                    PostVerdict::Deny {
                        reason,
                        annotations,
                    } => {
                        record_post_hook(
                            &mut metadata,
                            &binding.id,
                            Some(&reason),
                            annotations.as_ref(),
                        );
                        forced_withheld = Some("quarantined");
                    }
                },
                Ok(HookResponse::Pre { .. }) => return Err(ToolPipelineError::HookPhase),
                Err(error) => match binding.failure {
                    HookFailureMode::Closed => {
                        record_hook_failure(&mut metadata, &binding.id, &error.to_string());
                        forced_withheld = Some("quarantined");
                    }
                    HookFailureMode::Open => {
                        record_hook_failure(&mut metadata, &binding.id, &error.to_string())
                    }
                },
            }
        }

        let combined = value_to_ijson(json!({
            "content": transformed,
            "meta": metadata,
        }))?;
        let scanned = self.scanner.scan(&combined);
        let (sanitized, sanitized_metadata) = match scanned {
            SecretScan::Clean(value) => split_scanned_result(value)?,
            SecretScan::Redacted(value) => {
                let (value, mut scanned_metadata) = split_scanned_result(value)?;
                scanned_metadata.insert(
                    "secret_scan".to_owned(),
                    Value::String("redacted".to_owned()),
                );
                (value, scanned_metadata)
            }
            SecretScan::Withheld(reason) => {
                let seq =
                    self.append_result(execution, json!({"withheld": reason}), None, Map::new())?;
                return Ok(PipelineDecision::Completed { result_seq: seq });
            }
        };

        if let Some(reason) = forced_withheld {
            let seq = self.append_result(
                execution,
                json!({"withheld": reason}),
                None,
                sanitized_metadata,
            )?;
            return Ok(PipelineDecision::Completed { result_seq: seq });
        }

        let content = output_blocks(&sanitized);
        let seq = self.append_result(
            execution,
            backend_outcome,
            Some(content),
            sanitized_metadata,
        )?;
        Ok(PipelineDecision::Completed { result_seq: seq })
    }

    fn durable_approval_state(
        &self,
        execution: &ToolExecution,
    ) -> Result<DurableApprovalState, ToolPipelineError> {
        let Some(projection) = self.ledger.projection() else {
            return Ok(DurableApprovalState::None);
        };
        let mut request = None;
        let mut response = None;
        let mut effective = None;
        let mut result_seq = None;

        for event in &projection.events {
            if event.string_field("call") != Some(execution.call.as_str()) {
                continue;
            }
            let raw = serde_json::to_value(event.raw())
                .map_err(|error| ToolPipelineError::Json(error.to_string()))?;
            match event.kind() {
                schema::EventKind::EffectiveExecution => {
                    if event.turn() != Some(execution.turn) {
                        return Err(ToolPipelineError::DurableBinding(
                            "effective_execution turn does not match tool_call",
                        ));
                    }
                    if effective.is_some() {
                        return Err(ToolPipelineError::DurableBinding(
                            "tool call has multiple effective executions",
                        ));
                    }
                    effective = Some(self.materialize_ijson(raw.get("invocation").ok_or(
                        ToolPipelineError::DurableBinding(
                            "effective_execution invocation is missing",
                        ),
                    )?)?);
                }
                schema::EventKind::ApprovalRequest => {
                    if event.turn() != Some(execution.turn) {
                        return Err(ToolPipelineError::DurableBinding(
                            "approval_request turn does not match tool_call",
                        ));
                    }
                    request = Some((
                        event.seq(),
                        event
                            .string_field("scope")
                            .ok_or(ToolPipelineError::DurableBinding(
                                "approval_request scope is missing",
                            ))?
                            .to_owned(),
                    ));
                }
                schema::EventKind::ApprovalResponse => {
                    if event.turn() != Some(execution.turn) {
                        return Err(ToolPipelineError::DurableBinding(
                            "approval_response turn does not match tool_call",
                        ));
                    }
                    response = Some((event.seq(), raw));
                }
                schema::EventKind::ToolResult => result_seq = Some(event.seq()),
                _ => {}
            }
        }

        if let Some(result_seq) = result_seq {
            return Ok(DurableApprovalState::Completed { result_seq });
        }
        let Some((request_seq, scope)) = request else {
            return Ok(DurableApprovalState::None);
        };
        let Some((response_seq, response)) = response else {
            return Ok(DurableApprovalState::Pending { request_seq });
        };
        if response_seq <= request_seq {
            return Err(ToolPipelineError::DurableBinding(
                "approval_response does not follow approval_request",
            ));
        }
        if response
            .get("scope")
            .and_then(Value::as_str)
            .is_some_and(|response_scope| response_scope != scope)
        {
            return Err(ToolPipelineError::DurableBinding(
                "approval_response scope does not match approval_request",
            ));
        }
        if !response
            .get("grant")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            return Ok(DurableApprovalState::Denied {
                reason: "approval denied".to_owned(),
            });
        }
        let answer = response
            .get("answer")
            .cloned()
            .map(value_to_ijson)
            .transpose()?;
        Ok(DurableApprovalState::Granted {
            invocation: effective.unwrap_or_else(|| execution.invocation.clone()),
            response: DurableApprovalResponse {
                request_seq,
                response_seq,
                scope,
                answer,
            },
        })
    }

    fn materialize_ijson(&self, value: &Value) -> Result<IJsonValue, ToolPipelineError> {
        if let Some(spill) = value.get("$spill") {
            let asset = spill.get("asset").and_then(Value::as_str).ok_or(
                ToolPipelineError::DurableBinding("effective execution spill asset is missing"),
            )?;
            return Ok(IJsonValue::parse(&self.assets.read_verified(asset)?)?);
        }
        value_to_ijson(value.clone())
    }

    /// Terminalizes a call whose backend work finished outside the ordinary
    /// dispatch (a remote continuation reaching its terminal). Post hooks,
    /// secret scanning and the single `tool_result` follow the same path as a
    /// synchronous backend terminal.
    pub fn complete_continuation(
        &mut self,
        execution: &ToolExecution,
        hooks: &[HookBinding],
        invocation: &IJsonValue,
        terminal: BackendTerminal,
    ) -> Result<PipelineDecision, ToolPipelineError> {
        self.complete_terminal(execution, hooks, invocation, Map::new(), terminal)
    }

    /// Records one durable continuation step for a call: `step` 0 is the
    /// initial pending binding from the tool-control result; step `n ≥ 1` is
    /// the receipt of continuation request `n`. The event is runtime-visible.
    pub fn append_continuation_step(
        &mut self,
        execution: &ToolExecution,
        continuation_id: &str,
        step: u64,
        action: &str,
        state: &IJsonValue,
        park: Option<(&str, u64)>,
    ) -> Result<u64, ToolPipelineError> {
        let state = serde_json::to_value(state).expect("IJsonValue is serializable");
        let seq = self.ledger.next_seq();
        let mut payload = json!({
            "call": execution.call,
            "continuation_id": continuation_id,
            "step": step,
            "action": action,
            "state": self.inline_or_spill(state)?,
        });
        if let Some((poll_after, interval_ms)) = park {
            payload["poll_after"] = Value::String(poll_after.to_owned());
            payload["interval_ms"] = Value::from(interval_ms);
        }
        let event = event(json!({
            "v": 1,
            "seq": seq,
            "kind": "state",
            "ts": execution.timestamp,
            "turn": execution.turn,
            "visibility": "runtime",
            "subkind": TOOL_CONTINUATION_SUBKIND,
            "payload": payload,
        }))?;
        self.ledger
            .append_contract(event, BarrierContext::default())?;
        Ok(seq)
    }

    fn append_execution_started(
        &mut self,
        execution: &ToolExecution,
    ) -> Result<(), ToolPipelineError> {
        let event = event(json!({
            "v":1,"seq":self.ledger.next_seq(),"turn":execution.turn,
            "kind":"tool_execution_started","ts":execution.timestamp,"call":execution.call
        }))?;
        // This fsync is the execution boundary, after approval and before
        // entering any backend. A failed append must never reach the effect.
        self.ledger
            .append_contract(event, BarrierContext::default())?;
        Ok(())
    }

    fn append_effective_execution(
        &mut self,
        execution: &ToolExecution,
        invocation: &IJsonValue,
    ) -> Result<u64, ToolPipelineError> {
        let value = serde_json::to_value(invocation).expect("IJsonValue is serializable");
        let value = self.inline_or_spill(value)?;
        let seq = self.ledger.next_seq();
        let event = event(json!({
            "v": 1,
            "seq": seq,
            "kind": "effective_execution",
            "ts": execution.timestamp,
            "turn": execution.turn,
            "call": execution.call,
            "invocation": value,
        }))?;
        self.ledger
            .append_contract(event, BarrierContext::default())?;
        Ok(seq)
    }

    fn append_approval(
        &mut self,
        execution: &ToolExecution,
        scope: &str,
        question: &IJsonValue,
    ) -> Result<u64, ToolPipelineError> {
        let seq = self.ledger.next_seq();
        let event = event(json!({
            "v": 1,
            "seq": seq,
            "kind": "approval_request",
            "ts": execution.timestamp,
            "turn": execution.turn,
            "call": execution.call,
            "scope": scope,
            "question": question,
        }))?;
        self.ledger
            .append_contract(event, BarrierContext::default())?;
        Ok(seq)
    }

    fn append_result(
        &mut self,
        execution: &ToolExecution,
        outcome: Value,
        content: Option<Value>,
        metadata: Map<String, Value>,
    ) -> Result<u64, ToolPipelineError> {
        let seq = self.ledger.next_seq();
        let mut value = json!({
            "v": 1,
            "seq": seq,
            "kind": "tool_result",
            "ts": execution.timestamp,
            "turn": execution.turn,
            "call": execution.call,
            "outcome": outcome,
        });
        let object = value.as_object_mut().expect("event object");
        if let Some(content) = content {
            object.insert("content".to_owned(), self.inline_or_spill(content)?);
        }
        if !metadata.is_empty() {
            object.insert("meta".to_owned(), Value::Object(metadata));
        }
        let event = event(value)?;
        self.ledger
            .append_contract(event, BarrierContext::default())?;
        Ok(seq)
    }

    fn inline_or_spill(&self, value: Value) -> Result<Value, ToolPipelineError> {
        let bytes = serde_json_canonicalizer::to_vec(&value)
            .map_err(|error| ToolPipelineError::Json(error.to_string()))?;
        if bytes.len() <= SPILL_THRESHOLD {
            return Ok(value);
        }
        let reference = self.assets.publish(&bytes)?;
        Ok(json!({
            "$spill": {
                "asset": reference.asset,
                "bytes": reference.bytes,
            }
        }))
    }
}

enum DurableApprovalState {
    None,
    Completed {
        result_seq: u64,
    },
    Pending {
        request_seq: u64,
    },
    Denied {
        reason: String,
    },
    Granted {
        invocation: IJsonValue,
        response: DurableApprovalResponse,
    },
}

#[derive(Debug, Error)]
pub enum ToolPipelineError {
    #[error("store error: {0}")]
    Store(#[from] StoreError),
    #[error("schema error: {0}")]
    Schema(#[from] schema::SchemaError),
    #[error("hook returned the wrong phase")]
    HookPhase,
    #[error("tool JSON conversion failed: {0}")]
    Json(String),
    #[error("durable tool binding failed: {0}")]
    DurableBinding(&'static str),
}

fn event(value: Value) -> Result<Event, ToolPipelineError> {
    let bytes = serde_json_canonicalizer::to_vec(&value)
        .map_err(|error| ToolPipelineError::Json(error.to_string()))?;
    Ok(Event::decode_canonical(&bytes)?)
}

fn value_to_ijson(value: Value) -> Result<IJsonValue, ToolPipelineError> {
    let bytes =
        serde_json::to_vec(&value).map_err(|error| ToolPipelineError::Json(error.to_string()))?;
    Ok(IJsonValue::parse(&bytes)?)
}

fn split_scanned_result(
    value: IJsonValue,
) -> Result<(IJsonValue, Map<String, Value>), ToolPipelineError> {
    let mut object = serde_json::to_value(value)
        .map_err(|error| ToolPipelineError::Json(error.to_string()))?
        .as_object_mut()
        .ok_or_else(|| ToolPipelineError::Json("secret scan changed result shape".to_owned()))?
        .clone();
    let content = object
        .remove("content")
        .ok_or_else(|| ToolPipelineError::Json("secret scan removed content".to_owned()))?;
    let metadata = object
        .remove("meta")
        .and_then(|value| value.as_object().cloned())
        .ok_or_else(|| ToolPipelineError::Json("secret scan changed metadata shape".to_owned()))?;
    Ok((value_to_ijson(content)?, metadata))
}

fn output_blocks(value: &IJsonValue) -> Value {
    let value = serde_json::to_value(value).expect("IJsonValue is serializable");
    if value.as_array().is_some_and(|values| {
        values.iter().all(|item| {
            item.as_object()
                .and_then(|object| object.get("type"))
                .and_then(Value::as_str)
                .is_some()
        })
    }) {
        value
    } else {
        let text = serde_json_canonicalizer::to_string(&value)
            .expect("IJsonValue has canonical representation");
        json!([{"type": "text", "text": text}])
    }
}

fn offer_content_names(content: &Value, offered_name: &str) -> bool {
    content.as_array().is_some_and(|blocks| {
        blocks.iter().any(|block| {
            let Some(text) = block
                .as_object()
                .filter(|object| object.get("type").and_then(Value::as_str) == Some("text"))
                .and_then(|object| object.get("text"))
                .and_then(Value::as_str)
            else {
                return false;
            };
            serde_json::from_str::<Value>(text)
                .ok()
                .and_then(|value| value.get("items").and_then(Value::as_array).cloned())
                .is_some_and(|items| {
                    items
                        .iter()
                        .any(|item| item.get("name").and_then(Value::as_str) == Some(offered_name))
                })
        })
    })
}

fn redact_value(value: &mut Value) -> bool {
    match value {
        Value::String(text) => {
            let mut changed = false;
            for marker in ["sk-", "AKIA", "-----BEGIN PRIVATE KEY-----"] {
                let mut cursor = 0;
                while let Some(offset) = text[cursor..].find(marker) {
                    let index = cursor + offset;
                    // Key prefixes embedded in identifiers (for example
                    // "task-process" in an artifact path) are ordinary text.
                    let embedded = marker != "-----BEGIN PRIVATE KEY-----"
                        && text[..index]
                            .chars()
                            .next_back()
                            .is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '-');
                    if embedded {
                        cursor = index + marker.len();
                        continue;
                    }
                    let end = text[index..]
                        .find(char::is_whitespace)
                        .map_or(text.len(), |offset| index + offset);
                    text.replace_range(index..end, "[REDACTED]");
                    cursor = index + "[REDACTED]".len();
                    changed = true;
                }
            }
            changed
        }
        Value::Array(values) => values
            .iter_mut()
            .fold(false, |changed, value| redact_value(value) || changed),
        Value::Object(values) => values
            .values_mut()
            .fold(false, |changed, value| redact_value(value) || changed),
        Value::Null | Value::Bool(_) | Value::Number(_) => false,
    }
}

fn hook_failure_meta(id: &str, error: &str) -> Map<String, Value> {
    let mut metadata = Map::new();
    record_hook_failure(&mut metadata, id, error);
    metadata
}

fn record_hook_failure(metadata: &mut Map<String, Value>, id: &str, error: &str) {
    let entry = metadata
        .entry("hook_failures".to_owned())
        .or_insert_with(|| Value::Array(Vec::new()));
    entry
        .as_array_mut()
        .expect("hook_failures is always an array")
        .push(json!({"hook_id": id, "error": error}));
}

fn record_pre_mutation(metadata: &mut Map<String, Value>, id: &str, reason: Option<&str>) {
    let entry = metadata
        .entry("pre_hook_mutations".to_owned())
        .or_insert_with(|| Value::Array(Vec::new()));
    entry
        .as_array_mut()
        .expect("pre_hook_mutations is always an array")
        .push(json!({"hook_id": id, "reason": reason}));
}

fn record_post_hook(
    metadata: &mut Map<String, Value>,
    id: &str,
    denied: Option<&str>,
    annotations: Option<&IJsonValue>,
) {
    let entry = metadata
        .entry("post_hooks".to_owned())
        .or_insert_with(|| Value::Array(Vec::new()));
    entry
        .as_array_mut()
        .expect("post_hooks is always an array")
        .push(json!({
            "hook_id": id,
            "denied": denied,
            "annotations": annotations,
        }));
}

#[cfg(test)]
mod secret_token_boundary_tests {
    use super::*;

    #[test]
    fn artifact_task_paths_survive_scanning_without_weakening_key_redaction() {
        let mut value = serde_json::json!({
            "covered_set":[{"id":"/tmp/cf-task-process-9/workspace/docs/0.md","dedup_key":"sha256-fixture"}],
            "text":"task-process masks no key; Bearer sk-fixture-one and sk-fixture-two end"
        });
        assert!(redact_value(&mut value));
        assert_eq!(
            value["covered_set"][0]["id"],
            "/tmp/cf-task-process-9/workspace/docs/0.md"
        );
        assert_eq!(
            value["text"],
            "task-process masks no key; Bearer [REDACTED] and [REDACTED] end"
        );
        let mut clean = serde_json::json!({"path":"/tmp/task-result.md"});
        assert!(!redact_value(&mut clean));
    }
}
