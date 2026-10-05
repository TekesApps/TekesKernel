//! Production supervisor-owned tool-control operations.
//!
//! This module never appends semantic events. Cross-thread delivery, stop,
//! child launch, and report delivery are injected process-host operations whose
//! success values must prove the target worker's durable acknowledgement.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use profile::{DynamicTool, DynamicToolCatalog, DynamicToolEffect};
use schema::{Event, EventKind, IJsonValue, OriginTuple};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use store::{ThreadStore, scan_valid_prefix};
use thiserror::Error;
use tools::{
    BackendFailure, BackendGate, BackendOutcome, CancellationToken, JobBroker, JobLaunchPolicy,
    JobSpec, SandboxedJobLauncher, SecretScan, SecretScanner, validate_fixed_arguments,
};
use worker_control::continuation::{
    ToolContinuationOutcome, ToolContinuationRequest, ToolContinuationResponse,
};
use worker_control::{ToolControl, ToolControlError, ToolControlErrorCode, ToolControlResult};

use crate::tool_control::{
    ExternalEffectResolution, ToolControlHandler, ToolControlPolicy, ToolControlRecovery,
    resolve_durable_binding,
};

const DEFAULT_PAGE_BYTES: usize = 16_384;
const DEFAULT_SEARCH_LIMIT: usize = 20;
const MAX_SEARCH_LIMIT: usize = 100;

#[derive(Clone, Debug, PartialEq)]
pub struct DeliveryRequest {
    pub source_session: String,
    pub source_line: String,
    pub target_session: String,
    pub request_id: String,
    pub message: String,
    pub goal: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterruptRequest {
    pub source_session: String,
    pub source_line: String,
    pub target_session: String,
    pub request_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChildLaunchProof {
    pub session: String,
    pub parent_line: String,
    pub parent_file: String,
    pub parent_spawn_seq: u64,
    pub child_line: String,
    pub child_file: String,
    pub spawn_id: String,
    pub call_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParentReportProof {
    pub session: String,
    pub child_line: String,
    pub child_file: String,
    pub parent_line: String,
    pub parent_file: String,
    pub parent_spawn_seq: u64,
    pub spawn_id: String,
    pub report_call_id: String,
}

/// Process-host operations that cannot be implemented by writing a sibling
/// ledger. Implementations return only after the worker-authored durable fact
/// required by the operation is observable.
pub trait SupervisorRuntimeAuthority {
    fn ensure_running(
        &mut self,
        session: &str,
        request_id: &str,
    ) -> Result<IJsonValue, SupervisorOperationError>;

    fn deliver_input(
        &mut self,
        request: &DeliveryRequest,
    ) -> Result<IJsonValue, SupervisorOperationError>;

    fn interrupt(
        &mut self,
        request: &InterruptRequest,
    ) -> Result<IJsonValue, SupervisorOperationError>;

    fn ensure_child(
        &mut self,
        proof: &ChildLaunchProof,
    ) -> Result<IJsonValue, SupervisorOperationError>;

    fn deliver_report(
        &mut self,
        proof: &ParentReportProof,
        result: &IJsonValue,
    ) -> Result<IJsonValue, SupervisorOperationError>;
}

/// The durable ownerless job service. The concrete implementation wraps
/// `tools::JobBroker`; keeping this trait at the handler boundary lets the
/// process host inject its immutable sandbox profile and helper path.
pub trait SupervisorJobAuthority {
    fn execute(&mut self, request: &ToolControl) -> Result<IJsonValue, SupervisorOperationError>;
}

/// Host-owned route for a catalog entry whose immutable launch binding names
/// `supervisor_control`. Production advertises such an entry only when this
/// authority reports the exact source as executable.
/// Outcome of one dynamic supervisor execution: a terminal value, or a bound
/// remote continuation (an MCP task) the worker must drive to its terminal
/// through `tool_continuation` steps.
#[derive(Clone, Debug, PartialEq)]
pub enum DynamicExecutionOutcome {
    Value(IJsonValue),
    Pending {
        continuation_id: String,
        state: IJsonValue,
    },
}

pub trait DynamicSupervisorAuthority: Send + Sync {
    fn supports(&self, tool: &DynamicTool) -> bool;

    fn execute(
        &self,
        tool: &DynamicTool,
        request: &ToolControl,
    ) -> Result<IJsonValue, SupervisorOperationError>;

    /// Execution that may bind a continuation. `continuation_root` is the
    /// session's durable continuation journal directory. Authorities without
    /// asynchronous results keep the terminal `execute` path.
    fn execute_outcome(
        &self,
        tool: &DynamicTool,
        request: &ToolControl,
        _continuation_root: &Path,
    ) -> Result<DynamicExecutionOutcome, SupervisorOperationError> {
        self.execute(tool, request)
            .map(DynamicExecutionOutcome::Value)
    }

    /// One continuation step (query, input update, cancel) for a bound
    /// continuation. Replays of a committed step return the immutable receipt.
    fn continue_task(
        &self,
        tool: &DynamicTool,
        _request: &ToolContinuationRequest,
        _continuation_root: &Path,
    ) -> Result<ToolContinuationResponse, SupervisorOperationError> {
        Err(SupervisorOperationError::Unsupported(format!(
            "dynamic authority for {} has no continuation operations",
            tool.name
        )))
    }

    fn reconcile(&self, tool: &DynamicTool, _request: &ToolControl) -> ExternalEffectResolution {
        ExternalEffectResolution::Unknown {
            message: format!(
                "dynamic authority for {} has no authoritative reconciliation operation",
                tool.name
            ),
        }
    }

    fn cancel_inflight(&self) {}
}

pub struct JobBrokerSupervisorAuthority<L> {
    broker: JobBroker,
    launch_policy: JobLaunchPolicy,
    launcher: Arc<L>,
    cancellation: CancellationToken,
}

impl<L: SandboxedJobLauncher + 'static> JobBrokerSupervisorAuthority<L> {
    pub fn new(
        job_root: impl Into<PathBuf>,
        tail_bytes: usize,
        launch_policy: JobLaunchPolicy,
        launcher: Arc<L>,
        cancellation: CancellationToken,
    ) -> Result<Self, BackendFailure> {
        Ok(Self {
            broker: JobBroker::new(job_root, tail_bytes)?,
            launch_policy,
            launcher,
            cancellation,
        })
    }
}

impl<L: SandboxedJobLauncher + 'static> SupervisorJobAuthority for JobBrokerSupervisorAuthority<L> {
    fn execute(&mut self, request: &ToolControl) -> Result<IJsonValue, SupervisorOperationError> {
        let arguments = serde_json::to_value(&request.arguments)
            .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))?;
        let action = required_string(&arguments, "action")?;
        let outcome = match action {
            "start" => {
                let id = arguments
                    .get("job_id")
                    .and_then(Value::as_str)
                    .filter(|value| !value.is_empty())
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| format!("job-{}", request.request_id));
                let spec = JobSpec {
                    program: required_string(&arguments, "program")?.to_owned(),
                    args: string_array(&arguments, "args")?,
                    working_directory: arguments
                        .get("working_directory")
                        .and_then(Value::as_str)
                        .filter(|value| !value.is_empty())
                        .map(ToOwned::to_owned),
                    writable_paths: string_array(&arguments, "writable_paths")?,
                    environment: Default::default(),
                };
                job_record_outcome(self.broker.start(
                    &id,
                    spec,
                    &self.launch_policy,
                    Arc::clone(&self.launcher),
                    BackendGate::Ready,
                    &self.cancellation,
                ))
            }
            "list" => match self.broker.list() {
                BackendOutcome::Completed(Ok(records)) => to_ijson(&json!({"jobs":records})),
                BackendOutcome::Completed(Err(error)) => Err(map_backend(error)),
                BackendOutcome::Hold(_) => Err(SupervisorOperationError::Denied(
                    "job list unexpectedly requested approval".into(),
                )),
                BackendOutcome::Unavailable(value) => {
                    Err(SupervisorOperationError::Unavailable(value.reason))
                }
            },
            "status" => {
                let id = required_string(&arguments, "job_id")?;
                job_record_outcome(self.broker.status(id))
            }
            "stop" => {
                let id = required_string(&arguments, "job_id")?;
                job_record_outcome(self.broker.stop(id, BackendGate::Ready, &self.cancellation))
            }
            other => Err(SupervisorOperationError::Unsupported(format!(
                "unknown job action {other}"
            ))),
        }?;
        Ok(outcome)
    }
}

pub struct ProductionToolControlHandler<R, J> {
    root: PathBuf,
    runtime: R,
    jobs: J,
    dynamic_catalog: DynamicToolCatalog,
    dynamic: Option<Arc<dyn DynamicSupervisorAuthority>>,
}

impl<R, J> ProductionToolControlHandler<R, J> {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>, runtime: R, jobs: J) -> Self {
        Self {
            root: root.into(),
            runtime,
            jobs,
            dynamic_catalog: DynamicToolCatalog::default(),
            dynamic: None,
        }
    }

    #[must_use]
    pub fn with_dynamic_routes(
        mut self,
        catalog: DynamicToolCatalog,
        authority: Arc<dyn DynamicSupervisorAuthority>,
    ) -> Self {
        self.dynamic_catalog = catalog;
        self.dynamic = Some(authority);
        self
    }

    #[must_use]
    pub const fn runtime(&self) -> &R {
        &self.runtime
    }
}

impl<R: SupervisorRuntimeAuthority, J: SupervisorJobAuthority> ToolControlHandler
    for ProductionToolControlHandler<R, J>
{
    fn execute(&mut self, request: &ToolControl) -> ToolControlResult {
        match self.execute_inner(request) {
            Ok(DynamicExecutionOutcome::Value(value)) => ToolControlResult::success(
                request.request_id.clone(),
                request.call_id.clone(),
                value,
            ),
            Ok(DynamicExecutionOutcome::Pending {
                continuation_id,
                state,
            }) => ToolControlResult::pending(
                request.request_id.clone(),
                request.call_id.clone(),
                continuation_id,
                state,
            ),
            Err(error) => ToolControlResult::failure(
                request.request_id.clone(),
                request.call_id.clone(),
                error.into_control_error(),
            ),
        }
    }

    fn recovery(&self, request: &ToolControl) -> ToolControlRecovery {
        if is_fixed_supervisor_tool(&request.name) {
            return ToolControlRecovery::ReplaySafe;
        }
        let Some(tool) = self
            .dynamic_catalog
            .tools
            .iter()
            .find(|tool| tool.name == request.name)
        else {
            return ToolControlRecovery::ReplaySafe;
        };
        if matches!(
            tool.effect,
            DynamicToolEffect::ReadOnly | DynamicToolEffect::NetworkRead
        ) {
            ToolControlRecovery::ReplaySafe
        } else if tool.external_effect.is_some() {
            ToolControlRecovery::ExternalEffect
        } else {
            ToolControlRecovery::UnqueryableExternalEffect
        }
    }

    fn reconcile(&mut self, request: &ToolControl) -> ExternalEffectResolution {
        let Some(tool) = self
            .dynamic_catalog
            .tools
            .iter()
            .find(|tool| tool.name == request.name)
        else {
            return ExternalEffectResolution::Conflicted {
                message: format!("dynamic tool {} is no longer catalog-bound", request.name),
            };
        };
        let Some(authority) = self.dynamic.as_ref().filter(|route| route.supports(tool)) else {
            return ExternalEffectResolution::Unknown {
                message: format!("dynamic authority {} is unavailable", request.name),
            };
        };
        authority.reconcile(tool, request)
    }
}

impl<R: SupervisorRuntimeAuthority, J: SupervisorJobAuthority> ProductionToolControlHandler<R, J> {
    /// Resolves one continuation step for a call whose tool-control result was
    /// pending. The step's identity binds it to the original request; the
    /// authority's journal makes replays return the immutable receipt.
    pub fn continue_task(&mut self, request: &ToolContinuationRequest) -> ToolContinuationResponse {
        let failed = |error: SupervisorOperationError| ToolContinuationResponse {
            request_id: request.request_id.clone(),
            call_id: request.original.call_id.clone(),
            result: ToolContinuationOutcome::Failed {
                error: error.into_control_error(),
            },
        };
        let Some(tool) = self
            .dynamic_catalog
            .tools
            .iter()
            .find(|tool| tool.name == request.original.name)
        else {
            return failed(SupervisorOperationError::Unsupported(format!(
                "dynamic supervisor route {} is not bound",
                request.original.name
            )));
        };
        let Some(authority) = self.dynamic.as_ref().filter(|route| route.supports(tool)) else {
            return failed(SupervisorOperationError::Unsupported(format!(
                "dynamic supervisor dependency {} is unavailable",
                request.original.name
            )));
        };
        let root = self
            .root
            .join("threads")
            .join(&request.original.session)
            .join("continuations");
        match authority.continue_task(tool, request, &root) {
            Ok(response) => response,
            Err(error) => failed(error),
        }
    }

    fn execute_inner(
        &mut self,
        request: &ToolControl,
    ) -> Result<DynamicExecutionOutcome, SupervisorOperationError> {
        let arguments = serde_json::to_value(&request.arguments)
            .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))?;
        let session_folder = self.root.join("threads").join(&request.session);
        let resolved = resolve_durable_binding(&session_folder, request)
            .map_err(|error| SupervisorOperationError::Conflict(error.to_string()))?;
        if !is_fixed_supervisor_tool(&request.name) {
            return self.execute_dynamic(request, &session_folder.join("continuations"));
        }
        validate_fixed_arguments(&request.name, &arguments)
            .map_err(|error| SupervisorOperationError::Invalid(error.to_string()))?;
        let value = match request.name.as_str() {
            "context" => self.execute_context(request, &resolved, &arguments),
            "context_get" => self.execute_context_get(request, &resolved, &arguments),
            "job" => self.jobs.execute(request),
            "task" | "subagent" => {
                let proof = child_launch_proof(request, &resolved)?;
                self.runtime.ensure_child(&proof)
            }
            "report" => {
                let proof = parent_report_proof(request, &resolved)?;
                let result = arguments.get("result").ok_or_else(|| {
                    SupervisorOperationError::Invalid("report.result missing".into())
                })?;
                let result = to_ijson(result)?;
                self.runtime.deliver_report(&proof, &result)
            }
            other => Err(SupervisorOperationError::Unsupported(format!(
                "supervisor does not own tool {other}"
            ))),
        }?;
        Ok(DynamicExecutionOutcome::Value(value))
    }

    fn execute_dynamic(
        &self,
        request: &ToolControl,
        continuation_root: &Path,
    ) -> Result<DynamicExecutionOutcome, SupervisorOperationError> {
        let tool = self
            .dynamic_catalog
            .tools
            .iter()
            .find(|tool| tool.name == request.name)
            .ok_or_else(|| {
                SupervisorOperationError::Unsupported(format!(
                    "dynamic supervisor route {} is not bound",
                    request.name
                ))
            })?;
        engine::validate_dynamic_invocation(tool, &request.arguments)
            .map_err(|error| SupervisorOperationError::Invalid(error.to_string()))?;
        let authority = self
            .dynamic
            .as_ref()
            .filter(|route| route.supports(tool))
            .ok_or_else(|| {
                SupervisorOperationError::Unsupported(format!(
                    "dynamic supervisor dependency {} is unavailable",
                    request.name
                ))
            })?;
        authority.execute_outcome(tool, request, continuation_root)
    }

    fn execute_context(
        &mut self,
        request: &ToolControl,
        resolved: &crate::tool_control::ResolvedToolControlLine,
        arguments: &Value,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        let operation = arguments
            .get("operation")
            .and_then(Value::as_str)
            .ok_or_else(|| SupervisorOperationError::Invalid("context.operation missing".into()))?;
        let workspace = genesis_workspace(&resolved.projection)?;
        match operation {
            "threads" => self.context_threads(workspace),
            "read" => {
                let target = required_string(arguments, "thread_id")?;
                self.context_read(workspace, target)
            }
            "records" => self.context_records(resolved, arguments),
            "search" => self.context_search(request, resolved, workspace, arguments),
            "start" => self.context_start(request, resolved),
            "fork" => self.context_fork(request, resolved),
            "send" => {
                let target = required_string(arguments, "thread_id")?;
                require_session_in_workspace(&self.root, target, workspace)?;
                let message = required_string(arguments, "message")?;
                self.runtime.deliver_input(&DeliveryRequest {
                    source_session: request.session.clone(),
                    source_line: request.thread.clone(),
                    target_session: target.to_owned(),
                    request_id: request.request_id.clone(),
                    message: message.to_owned(),
                    goal: arguments
                        .get("goal")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                })
            }
            "resume" => {
                let target = required_string(arguments, "thread_id")?;
                require_session_in_workspace(&self.root, target, workspace)?;
                self.runtime.ensure_running(target, &request.request_id)
            }
            "archive" => {
                let target = required_string(arguments, "thread_id")?;
                if target == request.session {
                    return Err(SupervisorOperationError::Denied(
                        "a running worker cannot archive its containing session".into(),
                    ));
                }
                require_active_session_in_workspace(&self.root, target, workspace)?;
                ThreadStore::open(&self.root)
                    .map_err(map_store)?
                    .archive(target)
                    .map_err(map_store)?;
                to_ijson(&json!({"operation":"archive","session_id":target,"archived":true}))
            }
            "interrupt" => {
                let target = required_string(arguments, "thread_id")?;
                require_session_in_workspace(&self.root, target, workspace)?;
                self.runtime.interrupt(&InterruptRequest {
                    source_session: request.session.clone(),
                    source_line: request.thread.clone(),
                    target_session: target.to_owned(),
                    request_id: request.request_id.clone(),
                })
            }
            other => Err(SupervisorOperationError::Unsupported(format!(
                "unknown context operation {other}"
            ))),
        }
    }

    fn context_threads(&self, workspace: &str) -> Result<IJsonValue, SupervisorOperationError> {
        let mut items = Vec::new();
        for (area, archived) in [("threads", false), ("archive", true)] {
            for entry in fs::read_dir(self.root.join(area)).map_err(map_io)? {
                let entry = entry.map_err(map_io)?;
                if !entry.file_type().map_err(map_io)?.is_dir() {
                    continue;
                }
                let session = entry.file_name().to_string_lossy().into_owned();
                let projection = read_projection(&entry.path().join("main.jsonl"))?;
                if genesis_workspace(&projection)? != workspace {
                    continue;
                }
                let genesis = projection.events.first().expect("projection has genesis");
                let title = projection
                    .events
                    .iter()
                    .rev()
                    .find_map(|event| event.string_field("title"));
                items.push(json!({
                    "session_id": session,
                    "thread_id": genesis.string_field("thread"),
                    "title": title,
                    "archived": archived,
                    "last_seq": projection.last_seq,
                    "settled": projection.lifecycle.terminal_tail,
                    "parked": projection.lifecycle.open_hold && !projection.lifecycle.unresolved_work,
                }));
            }
        }
        items.sort_by(|left, right| {
            left.get("session_id")
                .and_then(Value::as_str)
                .cmp(&right.get("session_id").and_then(Value::as_str))
        });
        to_ijson(&json!({"threads":items}))
    }

    fn context_read(
        &self,
        workspace: &str,
        target: &str,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        let (folder, archived) = session_folder(&self.root, target)?;
        let projection = read_projection(&folder.join("main.jsonl"))?;
        if genesis_workspace(&projection)? != workspace {
            return Err(SupervisorOperationError::NotFound(
                "thread not found in this workspace".into(),
            ));
        }
        let genesis = projection.events.first().expect("projection has genesis");
        to_ijson(&json!({
            "session_id":target,
            "thread_id":genesis.string_field("thread"),
            "workspace":workspace,
            "archived":archived,
            "last_seq":projection.last_seq,
            "latest_turn":projection.latest_turn,
            "settled":projection.lifecycle.terminal_tail,
            "parked":projection.lifecycle.open_hold && !projection.lifecycle.unresolved_work,
        }))
    }

    fn context_records(
        &self,
        resolved: &crate::tool_control::ResolvedToolControlLine,
        arguments: &Value,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        let turn = required_string(arguments, "turn_id")?
            .parse::<u64>()
            .map_err(|_| {
                SupervisorOperationError::Invalid("turn_id is not a positive integer".into())
            })?;
        let offset = decode_cursor(arguments.get("cursor"), &format!("records:{turn}"))?;
        let limit = argument_limit(arguments, DEFAULT_SEARCH_LIMIT)?;
        let matching = resolved
            .projection
            .events
            .iter()
            .filter(|event| event.turn() == Some(turn))
            .collect::<Vec<_>>();
        let records = matching
            .iter()
            .skip(offset)
            .take(limit)
            .map(event_record)
            .collect::<Result<Vec<_>, _>>()?;
        let next = (offset + records.len() < matching.len())
            .then(|| encode_cursor(&format!("records:{turn}"), offset + records.len()));
        to_ijson(&json!({"records":records,"cursor":next}))
    }

    fn context_search(
        &self,
        request: &ToolControl,
        resolved: &crate::tool_control::ResolvedToolControlLine,
        workspace: &str,
        arguments: &Value,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        let query = required_string(arguments, "query")?;
        let folded_query = query.to_lowercase();
        let scope = arguments
            .get("scope")
            .and_then(Value::as_str)
            .unwrap_or("current_thread");
        let sessions = match scope {
            "current_thread" => vec![request.session.clone()],
            "thread" => {
                let target = required_string(arguments, "thread_id")?;
                require_session_in_workspace(&self.root, target, workspace)?;
                vec![target.to_owned()]
            }
            "project" => workspace_sessions(&self.root, workspace)?,
            _ => {
                return Err(SupervisorOperationError::Invalid(
                    "invalid context.search scope".into(),
                ));
            }
        };
        let identity = format!("search:{workspace}:{scope}:{query}");
        let offset = decode_cursor(arguments.get("cursor"), &identity)?;
        let limit = argument_limit(arguments, DEFAULT_SEARCH_LIMIT)?;
        let mut hits = Vec::new();
        for session in sessions {
            let (folder, _) = session_folder(&self.root, &session)?;
            for (line_path, projection) in session_lines(&folder)? {
                let line_id = genesis_line(&projection)?;
                for event in &projection.events {
                    let canonical = String::from_utf8(event.canonical_bytes().map_err(map_schema)?)
                        .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))?;
                    if canonical.to_lowercase().contains(&folded_query) {
                        hits.push(json!({
                            "session_id":session,
                            "thread_id":line_id,
                            "file":line_path.file_name().and_then(|value| value.to_str()),
                            "record_id":event.seq(),
                            "kind":event.kind().as_str(),
                            "turn":event.turn(),
                            "snippet":bounded_snippet(&canonical, 512),
                        }));
                    }
                }
            }
        }
        hits.sort_by(|left, right| {
            (
                left.get("session_id").and_then(Value::as_str),
                left.get("thread_id").and_then(Value::as_str),
                left.get("record_id").and_then(Value::as_u64),
            )
                .cmp(&(
                    right.get("session_id").and_then(Value::as_str),
                    right.get("thread_id").and_then(Value::as_str),
                    right.get("record_id").and_then(Value::as_u64),
                ))
        });
        let page = hits
            .iter()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect::<Vec<_>>();
        let next = (offset + page.len() < hits.len())
            .then(|| encode_cursor(&identity, offset + page.len()));
        let _ = resolved;
        to_ijson(&json!({"hits":page,"cursor":next}))
    }

    fn context_start(
        &mut self,
        request: &ToolControl,
        resolved: &crate::tool_control::ResolvedToolControlLine,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        let genesis =
            resolved.projection.events.first().ok_or_else(|| {
                SupervisorOperationError::Corruption("caller has no genesis".into())
            })?;
        let source = serde_json::to_value(genesis.raw())
            .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))?;
        let session = deterministic_uuid(&request.request_id);
        let timestamp = paired_tool_timestamp(&resolved.projection, request)?;
        let origin = OriginTuple {
            principal: "kernel-worker".into(),
            client: "context".into(),
            target: session.clone(),
            op: "create".into(),
            key: request.request_id.clone(),
        };
        let mut event = json!({
            "v":1,"seq":1,"ts":timestamp,"kind":"genesis",
            "thread":session,"workspace":genesis_workspace(&resolved.projection)?,
            "format":1,"min_reader":1,"min_writer":1,
            "resume":source.get("resume").cloned().unwrap_or_else(|| json!("never")),
            "config":source.get("config").cloned().ok_or_else(|| SupervisorOperationError::Corruption("caller genesis config missing".into()))?,
            "origin_key":origin.key,"origin_tuple":origin,
        });
        if let Some(instruction) = source.get("instruction") {
            event["instruction"] = instruction.clone();
        }
        let event = Event::from_value(to_ijson(&event)?).map_err(map_schema)?;
        ThreadStore::open(&self.root)
            .map_err(map_store)?
            .create_thread(&session, event)
            .map_err(map_store)?;
        self.runtime.ensure_running(&session, &request.request_id)
    }

    fn context_fork(
        &mut self,
        request: &ToolControl,
        resolved: &crate::tool_control::ResolvedToolControlLine,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        let destination = deterministic_uuid(&request.request_id);
        let timestamp = paired_tool_timestamp(&resolved.projection, request)?;
        let store = ThreadStore::open(&self.root).map_err(map_store)?;
        store
            .begin_fork(
                &request.request_id,
                &request.session,
                &destination,
                timestamp,
            )
            .map_err(map_store)?;
        store.recover_rewrites().map_err(map_store)?;
        to_ijson(&json!({"operation":"fork","session_id":destination}))
    }

    fn execute_context_get(
        &self,
        request: &ToolControl,
        resolved: &crate::tool_control::ResolvedToolControlLine,
        arguments: &Value,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        let workspace = genesis_workspace(&resolved.projection)?;
        let projection = match arguments.get("thread_id").and_then(Value::as_str) {
            Some(line) if line != request.thread => {
                resolve_workspace_line(&self.root, workspace, line)?.1
            }
            _ => resolved.projection.clone(),
        };
        let ids = arguments
            .get("record_ids")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                SupervisorOperationError::Invalid("context_get.record_ids missing".into())
            })?;
        let mut ordered = Vec::new();
        let mut seen = BTreeSet::new();
        for id in ids {
            let id = id.as_u64().ok_or_else(|| {
                SupervisorOperationError::Invalid("record id is not an integer".into())
            })?;
            if seen.insert(id) {
                ordered.push(id);
            }
        }
        let identity = format!(
            "context_get:{}:{}",
            genesis_line(&projection)?,
            ordered
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(",")
        );
        let offset = decode_cursor(arguments.get("cursor"), &identity)?;
        let page_bytes = arguments
            .get("page_bytes")
            .and_then(Value::as_u64)
            .map_or(DEFAULT_PAGE_BYTES, |value| value as usize);
        let mut records = Vec::new();
        let mut used = 0usize;
        let mut consumed = 0usize;
        for id in ordered.iter().skip(offset) {
            let event = projection
                .events
                .get(usize::try_from(*id - 1).map_err(|_| {
                    SupervisorOperationError::Invalid("record id does not fit this platform".into())
                })?)
                .filter(|event| event.seq() == *id)
                .ok_or_else(|| SupervisorOperationError::NotFound(format!("record {id}")))?;
            let canonical = event.canonical_bytes().map_err(map_schema)?;
            if canonical.len() > page_bytes {
                return Err(SupervisorOperationError::Limit(format!(
                    "record {id} exceeds page_bytes"
                )));
            }
            if used + canonical.len() > page_bytes {
                break;
            }
            used += canonical.len();
            consumed += 1;
            records.push(json!({
                "record_id":id,
                "event":serde_json::from_slice::<Value>(&canonical)
                    .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))?,
            }));
        }
        let next = (offset + consumed < ordered.len())
            .then(|| encode_cursor(&identity, offset + consumed));
        to_ijson(&json!({
            "thread_id":genesis_line(&projection)?,
            "records":records,
            "cursor":next,
        }))
    }
}

fn is_fixed_supervisor_tool(name: &str) -> bool {
    matches!(
        name,
        "context" | "context_get" | "job" | "report" | "subagent" | "task"
    )
}

#[derive(Clone, Debug, Default)]
pub struct ProductionToolControlPolicy {
    scanner: SecretScanner,
}

impl ProductionToolControlPolicy {
    #[must_use]
    pub const fn new(scanner: SecretScanner) -> Self {
        Self { scanner }
    }
}

impl ProductionToolControlPolicy {
    /// The same secret policy for continuation step responses: the pending
    /// remote state, the terminal value and the failure message are scanned
    /// exactly like a tool-control result; a withheld payload becomes a
    /// non-retryable failure, never a partially leaked value.
    pub fn apply_continuation(
        &mut self,
        mut response: ToolContinuationResponse,
    ) -> ToolContinuationResponse {
        let withheld = |response: &ToolContinuationResponse| ToolContinuationResponse {
            request_id: response.request_id.clone(),
            call_id: response.call_id.clone(),
            result: ToolContinuationOutcome::Failed {
                error: ToolControlError {
                    code: ToolControlErrorCode::Denied,
                    message: "tool continuation result withheld by secret policy".into(),
                    retryable: false,
                },
            },
        };
        match &mut response.result {
            ToolContinuationOutcome::Pending { state, .. } => match self.scanner.scan(state) {
                SecretScan::Clean(clean) | SecretScan::Redacted(clean) => *state = clean,
                SecretScan::Withheld(_) => return withheld(&response),
            },
            ToolContinuationOutcome::Completed { value } => match self.scanner.scan(value) {
                SecretScan::Clean(clean) | SecretScan::Redacted(clean) => *value = clean,
                SecretScan::Withheld(_) => return withheld(&response),
            },
            ToolContinuationOutcome::Failed { error } => {
                let message = IJsonValue::from(error.message.clone());
                match self.scanner.scan(&message) {
                    SecretScan::Clean(message) | SecretScan::Redacted(message) => {
                        error.message = serde_json::from_value(
                            serde_json::to_value(message).unwrap_or(Value::Null),
                        )
                        .unwrap_or_else(|_| "continuation error was redacted".to_owned());
                    }
                    SecretScan::Withheld(_) => return withheld(&response),
                }
            }
        }
        response
    }
}

impl ToolControlPolicy for ProductionToolControlPolicy {
    fn apply(&mut self, mut result: ToolControlResult) -> ToolControlResult {
        if let Some(pending) = result.pending.as_mut() {
            // A pending continuation carries remote task state, not a value;
            // it is scanned the same way and withheld the same way.
            return match self.scanner.scan(&pending.state) {
                SecretScan::Clean(state) | SecretScan::Redacted(state) => {
                    pending.state = state;
                    result
                }
                SecretScan::Withheld(_) => policy_withheld(result),
            };
        }
        if let Some(value) = result.value.take() {
            return match self.scanner.scan(&value) {
                SecretScan::Clean(value) | SecretScan::Redacted(value) => {
                    result.value = Some(value);
                    result
                }
                SecretScan::Withheld(_) => policy_withheld(result),
            };
        }
        if let Some(error) = result.error.as_mut() {
            let message = IJsonValue::from(error.message.clone());
            match self.scanner.scan(&message) {
                SecretScan::Clean(message) | SecretScan::Redacted(message) => {
                    error.message = serde_json::from_value(
                        serde_json::to_value(message).unwrap_or(Value::Null),
                    )
                    .unwrap_or_else(|_| "backend error was redacted".to_owned());
                    result
                }
                SecretScan::Withheld(_) => policy_withheld(result),
            }
        } else {
            policy_withheld(result)
        }
    }
}

fn policy_withheld(result: ToolControlResult) -> ToolControlResult {
    ToolControlResult::failure(
        result.request_id,
        result.call_id,
        ToolControlError {
            code: ToolControlErrorCode::Denied,
            message: "tool-control result withheld by secret policy".into(),
            retryable: false,
        },
    )
}

fn child_launch_proof(
    request: &ToolControl,
    resolved: &crate::tool_control::ResolvedToolControlLine,
) -> Result<ChildLaunchProof, SupervisorOperationError> {
    let spawn = resolved
        .projection
        .events
        .iter()
        .find(|event| {
            matches!(event.kind(), EventKind::Spawn)
                && event.turn() == Some(request.turn)
                && event.string_field("call") == Some(request.call_id.as_str())
        })
        .ok_or_else(|| {
            SupervisorOperationError::Conflict(
                "task/subagent has no durable parent spawn proof".into(),
            )
        })?;
    if resolved.projection.events.iter().any(|event| {
        matches!(event.kind(), EventKind::ChildResult)
            && event.string_field("call") == Some(request.call_id.as_str())
    }) {
        return Err(SupervisorOperationError::Conflict(
            "child_result already terminalized this spawn".into(),
        ));
    }
    let child_file = spawn
        .string_field("child")
        .ok_or_else(|| SupervisorOperationError::Corruption("spawn.child missing".into()))?;
    validate_line_file_name(child_file)?;
    let child_path = resolved.thread_folder.join(child_file);
    let child = read_projection(&child_path)?;
    let genesis = child
        .events
        .first()
        .ok_or_else(|| SupervisorOperationError::Corruption("child genesis missing".into()))?;
    let value = serde_json::to_value(genesis.raw())
        .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))?;
    let parent = value
        .get("parent")
        .and_then(Value::as_object)
        .ok_or_else(|| SupervisorOperationError::Conflict("child parent binding missing".into()))?;
    let parent_file = resolved
        .line_path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            SupervisorOperationError::Corruption("parent file name is not UTF-8".into())
        })?;
    let spawn_id = spawn
        .string_field("spawn_id")
        .ok_or_else(|| SupervisorOperationError::Corruption("spawn_id missing".into()))?;
    if parent.get("file").and_then(Value::as_str) != Some(parent_file)
        || parent.get("seq").and_then(Value::as_u64) != Some(spawn.seq())
        || parent.get("spawn_id").and_then(Value::as_str) != Some(spawn_id)
    {
        return Err(SupervisorOperationError::Conflict(
            "child genesis does not match the durable parent spawn".into(),
        ));
    }
    Ok(ChildLaunchProof {
        session: request.session.clone(),
        parent_line: request.thread.clone(),
        parent_file: parent_file.to_owned(),
        parent_spawn_seq: spawn.seq(),
        child_line: genesis_line(&child)?.to_owned(),
        child_file: child_file.to_owned(),
        spawn_id: spawn_id.to_owned(),
        call_id: request.call_id.clone(),
    })
}

fn parent_report_proof(
    request: &ToolControl,
    resolved: &crate::tool_control::ResolvedToolControlLine,
) -> Result<ParentReportProof, SupervisorOperationError> {
    let child_genesis = resolved.projection.events.first().ok_or_else(|| {
        SupervisorOperationError::Corruption("reporting line has no genesis".into())
    })?;
    let value = serde_json::to_value(child_genesis.raw())
        .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))?;
    let parent = value
        .get("parent")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            SupervisorOperationError::Denied("report is valid only in a child line".into())
        })?;
    let parent_file = parent
        .get("file")
        .and_then(Value::as_str)
        .ok_or_else(|| SupervisorOperationError::Corruption("parent.file missing".into()))?;
    validate_line_file_name(parent_file)?;
    let parent_seq = parent
        .get("seq")
        .and_then(Value::as_u64)
        .ok_or_else(|| SupervisorOperationError::Corruption("parent.seq missing".into()))?;
    let spawn_id = parent
        .get("spawn_id")
        .and_then(Value::as_str)
        .ok_or_else(|| SupervisorOperationError::Corruption("parent.spawn_id missing".into()))?;
    let parent_projection = read_projection(&resolved.thread_folder.join(parent_file))?;
    let spawn = parent_projection
        .events
        .get(usize::try_from(parent_seq - 1).map_err(|_| {
            SupervisorOperationError::Corruption("parent seq does not fit this platform".into())
        })?)
        .filter(|event| event.seq() == parent_seq && matches!(event.kind(), EventKind::Spawn))
        .ok_or_else(|| SupervisorOperationError::Conflict("parent spawn proof missing".into()))?;
    let child_file = resolved
        .line_path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            SupervisorOperationError::Corruption("child file name is not UTF-8".into())
        })?;
    if spawn.string_field("child") != Some(child_file)
        || spawn.string_field("spawn_id") != Some(spawn_id)
    {
        return Err(SupervisorOperationError::Conflict(
            "reporting child does not match parent spawn".into(),
        ));
    }
    Ok(ParentReportProof {
        session: request.session.clone(),
        child_line: request.thread.clone(),
        child_file: child_file.to_owned(),
        parent_line: genesis_line(&parent_projection)?.to_owned(),
        parent_file: parent_file.to_owned(),
        parent_spawn_seq: parent_seq,
        spawn_id: spawn_id.to_owned(),
        report_call_id: request.call_id.clone(),
    })
}

fn session_lines(
    folder: &Path,
) -> Result<Vec<(PathBuf, schema::LedgerProjection)>, SupervisorOperationError> {
    let mut lines = Vec::new();
    for entry in fs::read_dir(folder).map_err(map_io)? {
        let entry = entry.map_err(map_io)?;
        if !entry.file_type().map_err(map_io)?.is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) == Some("jsonl")
            && path.file_name().and_then(|value| value.to_str()) != Some(endpoint::JOURNAL_FILE)
        {
            if let Ok(projection) = read_projection(&path) {
                lines.push((path, projection));
            }
        }
    }
    lines.sort_by(|left, right| left.0.as_os_str().cmp(right.0.as_os_str()));
    Ok(lines)
}

fn resolve_workspace_line(
    root: &Path,
    workspace: &str,
    line: &str,
) -> Result<(PathBuf, schema::LedgerProjection), SupervisorOperationError> {
    let mut found = None;
    for session in workspace_sessions(root, workspace)? {
        let (folder, _) = session_folder(root, &session)?;
        for candidate in session_lines(&folder)? {
            if genesis_line(&candidate.1)? == line {
                if found.is_some() {
                    return Err(SupervisorOperationError::Corruption(
                        "line identity is duplicated across the workspace".into(),
                    ));
                }
                found = Some(candidate);
            }
        }
    }
    found.ok_or_else(|| SupervisorOperationError::NotFound("line not found in workspace".into()))
}

fn workspace_sessions(
    root: &Path,
    workspace: &str,
) -> Result<Vec<String>, SupervisorOperationError> {
    let mut sessions = Vec::new();
    for area in ["threads", "archive"] {
        for entry in fs::read_dir(root.join(area)).map_err(map_io)? {
            let entry = entry.map_err(map_io)?;
            if entry.file_type().map_err(map_io)?.is_dir()
                && read_projection(&entry.path().join("main.jsonl"))
                    .and_then(|projection| Ok(genesis_workspace(&projection)?.to_owned()))
                    .is_ok_and(|candidate| candidate == workspace)
            {
                sessions.push(entry.file_name().to_string_lossy().into_owned());
            }
        }
    }
    sessions.sort();
    Ok(sessions)
}

fn require_session_in_workspace(
    root: &Path,
    session: &str,
    workspace: &str,
) -> Result<(), SupervisorOperationError> {
    let (folder, _) = session_folder(root, session)?;
    let projection = read_projection(&folder.join("main.jsonl"))?;
    if genesis_workspace(&projection)? != workspace {
        return Err(SupervisorOperationError::NotFound(
            "thread not found in this workspace".into(),
        ));
    }
    Ok(())
}

fn require_active_session_in_workspace(
    root: &Path,
    session: &str,
    workspace: &str,
) -> Result<(), SupervisorOperationError> {
    let folder = root.join("threads").join(session);
    if !folder.is_dir() {
        return Err(SupervisorOperationError::NotFound(
            "active thread not found in this workspace".into(),
        ));
    }
    let projection = read_projection(&folder.join("main.jsonl"))?;
    if genesis_workspace(&projection)? != workspace {
        return Err(SupervisorOperationError::NotFound(
            "active thread not found in this workspace".into(),
        ));
    }
    Ok(())
}

fn session_folder(root: &Path, session: &str) -> Result<(PathBuf, bool), SupervisorOperationError> {
    let active = root.join("threads").join(session);
    if active.is_dir() {
        return Ok((active, false));
    }
    let archived = root.join("archive").join(session);
    if archived.is_dir() {
        return Ok((archived, true));
    }
    Err(SupervisorOperationError::NotFound(
        "thread not found".into(),
    ))
}

fn read_projection(path: &Path) -> Result<schema::LedgerProjection, SupervisorOperationError> {
    let bytes = fs::read(path).map_err(map_io)?;
    let scan = scan_valid_prefix(&bytes, 1);
    scan.projection.ok_or_else(|| {
        SupervisorOperationError::Corruption(format!(
            "{} has no valid genesis prefix",
            path.display()
        ))
    })
}

fn genesis_workspace(
    projection: &schema::LedgerProjection,
) -> Result<&str, SupervisorOperationError> {
    projection
        .events
        .first()
        .and_then(|event| event.string_field("workspace"))
        .ok_or_else(|| SupervisorOperationError::Corruption("genesis workspace missing".into()))
}

fn genesis_line(projection: &schema::LedgerProjection) -> Result<&str, SupervisorOperationError> {
    projection
        .events
        .first()
        .and_then(|event| event.string_field("thread"))
        .ok_or_else(|| SupervisorOperationError::Corruption("genesis thread missing".into()))
}

fn paired_tool_timestamp<'a>(
    projection: &'a schema::LedgerProjection,
    request: &ToolControl,
) -> Result<&'a str, SupervisorOperationError> {
    projection
        .events
        .iter()
        .find(|event| {
            matches!(event.kind(), EventKind::ToolCall)
                && event.turn() == Some(request.turn)
                && event.string_field("call") == Some(request.call_id.as_str())
        })
        .and_then(|event| event.string_field("ts"))
        .ok_or_else(|| SupervisorOperationError::Corruption("tool_call timestamp missing".into()))
}

fn event_record(event: &&Event) -> Result<Value, SupervisorOperationError> {
    let canonical = event.canonical_bytes().map_err(map_schema)?;
    serde_json::from_slice(&canonical)
        .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))
}

fn required_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, SupervisorOperationError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| SupervisorOperationError::Invalid(format!("{field} is required")))
}

fn string_array(value: &Value, field: &str) -> Result<Vec<String>, SupervisorOperationError> {
    value.get(field).map_or(Ok(Vec::new()), |value| {
        value
            .as_array()
            .ok_or_else(|| SupervisorOperationError::Invalid(format!("{field} must be an array")))?
            .iter()
            .map(|value| {
                value.as_str().map(ToOwned::to_owned).ok_or_else(|| {
                    SupervisorOperationError::Invalid(format!("{field} contains a non-string"))
                })
            })
            .collect()
    })
}

fn job_record_outcome(
    outcome: BackendOutcome<tools::JobRecord>,
) -> Result<IJsonValue, SupervisorOperationError> {
    match outcome {
        BackendOutcome::Completed(Ok(record)) => to_ijson(&json!({"job":record})),
        BackendOutcome::Completed(Err(error)) => Err(map_backend(error)),
        BackendOutcome::Hold(hold) => Err(SupervisorOperationError::Denied(hold.reason)),
        BackendOutcome::Unavailable(value) => {
            Err(SupervisorOperationError::Unavailable(value.reason))
        }
    }
}

fn map_backend(error: BackendFailure) -> SupervisorOperationError {
    match error {
        BackendFailure::Invalid(message) => SupervisorOperationError::Invalid(message),
        BackendFailure::Denied(message) => SupervisorOperationError::Denied(message),
        BackendFailure::Cancelled => SupervisorOperationError::Unavailable("job cancelled".into()),
        BackendFailure::Timeout => SupervisorOperationError::Timeout("job timed out".into()),
        BackendFailure::Limit(message) => SupervisorOperationError::Limit(message),
        BackendFailure::NotFound(message) => SupervisorOperationError::NotFound(message),
        BackendFailure::Conflict(message) => SupervisorOperationError::Conflict(message),
        BackendFailure::Unavailable(message) => SupervisorOperationError::Unavailable(message),
        BackendFailure::TerminalUnavailable(message) => {
            SupervisorOperationError::Unavailable(message)
        }
        BackendFailure::Io(message) => SupervisorOperationError::Unavailable(message),
        BackendFailure::Protocol(message) => SupervisorOperationError::Protocol(message),
        BackendFailure::Unknown(message) => SupervisorOperationError::Unavailable(message),
    }
}

fn argument_limit(value: &Value, default: usize) -> Result<usize, SupervisorOperationError> {
    let value = value
        .get("limit")
        .and_then(Value::as_u64)
        .map_or(default, |value| value as usize);
    if value == 0 || value > MAX_SEARCH_LIMIT {
        return Err(SupervisorOperationError::Limit(format!(
            "limit must be in 1..={MAX_SEARCH_LIMIT}"
        )));
    }
    Ok(value)
}

fn encode_cursor(identity: &str, offset: usize) -> String {
    let digest = Sha256::digest(identity.as_bytes());
    format!("tc1-{}-{offset}", &format!("{digest:x}")[..16])
}

fn decode_cursor(value: Option<&Value>, identity: &str) -> Result<usize, SupervisorOperationError> {
    let Some(cursor) = value
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
    else {
        return Ok(0);
    };
    let cursor_zero = encode_cursor(identity, 0);
    let prefix = cursor_zero
        .rsplit_once('-')
        .map(|(prefix, _)| prefix)
        .expect("cursor contains offset");
    let (actual, offset) = cursor
        .rsplit_once('-')
        .ok_or_else(|| SupervisorOperationError::Invalid("invalid cursor".into()))?;
    if actual != prefix {
        return Err(SupervisorOperationError::Conflict(
            "cursor does not belong to this query".into(),
        ));
    }
    offset
        .parse()
        .map_err(|_| SupervisorOperationError::Invalid("invalid cursor offset".into()))
}

fn deterministic_uuid(request_id: &str) -> String {
    let mut bytes = request_id.as_bytes()[..32].to_vec();
    bytes[12] = b'4';
    bytes[16] = match bytes[16] {
        b'0'..=b'3' => b'8',
        b'4'..=b'7' => b'9',
        b'8'..=b'b' => b'a',
        _ => b'b',
    };
    let text = String::from_utf8(bytes).expect("request id is ASCII hex");
    format!(
        "{}-{}-{}-{}-{}",
        &text[..8],
        &text[8..12],
        &text[12..16],
        &text[16..20],
        &text[20..32]
    )
}

fn validate_line_file_name(value: &str) -> Result<(), SupervisorOperationError> {
    let path = Path::new(value);
    if path.components().count() != 1
        || !matches!(path.components().next(), Some(Component::Normal(_)))
        || path.extension().and_then(|value| value.to_str()) != Some("jsonl")
    {
        return Err(SupervisorOperationError::Denied(
            "line file escapes the session folder".into(),
        ));
    }
    Ok(())
}

fn bounded_snippet(value: &str, max_bytes: usize) -> String {
    if value.len() <= max_bytes {
        return value.to_owned();
    }
    let mut boundary = max_bytes;
    while !value.is_char_boundary(boundary) {
        boundary -= 1;
    }
    format!("{}[truncated]", &value[..boundary])
}

fn to_ijson(value: &Value) -> Result<IJsonValue, SupervisorOperationError> {
    IJsonValue::parse(
        &serde_json::to_vec(value)
            .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))?,
    )
    .map_err(map_schema)
}

fn map_io(error: std::io::Error) -> SupervisorOperationError {
    if error.kind() == std::io::ErrorKind::NotFound {
        SupervisorOperationError::NotFound("resource not found".into())
    } else {
        SupervisorOperationError::Unavailable(error.to_string())
    }
}

fn map_store(error: store::StoreError) -> SupervisorOperationError {
    match error {
        store::StoreError::NotFound => {
            SupervisorOperationError::NotFound("thread not found".into())
        }
        store::StoreError::Archived => {
            SupervisorOperationError::Conflict("thread is archived".into())
        }
        other => SupervisorOperationError::Unavailable(other.to_string()),
    }
}

fn map_schema(error: schema::SchemaError) -> SupervisorOperationError {
    SupervisorOperationError::Corruption(error.to_string())
}

#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum SupervisorOperationError {
    #[error("invalid request: {0}")]
    Invalid(String),
    #[error("operation denied: {0}")]
    Denied(String),
    #[error("resource not found: {0}")]
    NotFound(String),
    #[error("operation conflict: {0}")]
    Conflict(String),
    #[error("external effect authority conflict: {0}")]
    EffectConflicted(String),
    #[error("operation unavailable: {0}")]
    Unavailable(String),
    #[error("remote tool failed: {0}")]
    ToolFailed(String),
    #[error("operation timed out: {0}")]
    Timeout(String),
    #[error("external effect outcome is unknown: {0}")]
    AmbiguousEffect(String),
    #[error("limit exceeded: {0}")]
    Limit(String),
    #[error("protocol failure: {0}")]
    Protocol(String),
    #[error("durable state is corrupt: {0}")]
    Corruption(String),
    #[error("unsupported operation: {0}")]
    Unsupported(String),
}

impl SupervisorOperationError {
    fn into_control_error(self) -> ToolControlError {
        let (code, retryable) = match self {
            Self::Invalid(_) | Self::Protocol(_) | Self::Limit(_) => {
                (ToolControlErrorCode::Denied, false)
            }
            Self::Denied(_) => (ToolControlErrorCode::Denied, false),
            Self::NotFound(_) => (ToolControlErrorCode::NotFound, false),
            Self::Conflict(_) | Self::Corruption(_) => (ToolControlErrorCode::Conflict, false),
            Self::EffectConflicted(_) => (ToolControlErrorCode::EffectConflicted, false),
            Self::Unavailable(_) => (ToolControlErrorCode::Unavailable, true),
            Self::ToolFailed(_) => (ToolControlErrorCode::Unavailable, false),
            Self::Timeout(_) => (ToolControlErrorCode::Timeout, true),
            Self::AmbiguousEffect(_) => (ToolControlErrorCode::EffectUnknown, false),
            Self::Unsupported(_) => (ToolControlErrorCode::Unsupported, false),
        };
        ToolControlError {
            code,
            message: self.to_string(),
            retryable,
        }
    }
}

#[cfg(test)]
mod tool_failure_tests {
    use super::*;
    #[test]
    fn definitive_remote_tool_failure_is_not_retryable_transport_loss() {
        let failure =
            SupervisorOperationError::ToolFailed("remote details".into()).into_control_error();
        assert_eq!(failure.code, ToolControlErrorCode::Unavailable);
        assert!(!failure.retryable);
        assert!(failure.message.contains("remote details"));
        assert!(
            SupervisorOperationError::Unavailable("transport".into())
                .into_control_error()
                .retryable
        );
    }
}

#[cfg(test)]
mod continuation_policy_tests {
    use super::*;
    use worker_control::continuation::{ContinuationOperation, ToolContinuationRequest};

    fn value(raw: &str) -> IJsonValue {
        IJsonValue::parse(raw.as_bytes()).unwrap()
    }

    /// A pending tool-control result is remote task state, not a value: the
    /// secret policy scans it and passes it through instead of withholding it.
    #[test]
    fn pending_results_and_continuation_responses_pass_the_secret_policy() {
        let mut policy = ProductionToolControlPolicy::new(tools::SecretScanner::default());
        let pending = ToolControlResult::pending(
            "a".repeat(64),
            "call-1",
            "b".repeat(64),
            value(r#"{"taskId":"t","status":"working"}"#),
        );
        let applied = policy.apply(pending.clone());
        assert_eq!(applied, pending, "clean pending state passes unchanged");
        let original = ToolControl::new(
            "018f0000-0000-7000-8000-000000000003",
            "018f0000-0000-7000-8000-000000000003",
            1,
            "call-1",
            "mcp__t__x",
            value("{}"),
        )
        .unwrap();
        let request =
            ToolContinuationRequest::new(original, "b".repeat(64), 1, ContinuationOperation::Query)
                .unwrap();
        let completed = ToolContinuationResponse {
            request_id: request.request_id.clone(),
            call_id: "call-1".into(),
            result: ToolContinuationOutcome::Completed {
                value: value(r#"{"content":[{"type":"text","text":"done"}]}"#),
            },
        };
        assert_eq!(policy.apply_continuation(completed.clone()), completed);
        let leaking = ToolContinuationResponse {
            request_id: request.request_id.clone(),
            call_id: "call-1".into(),
            result: ToolContinuationOutcome::Completed {
                value: value(
                    r#"{"content":[{"type":"text","text":"key sk-1234567890abcdef leaked"}]}"#,
                ),
            },
        };
        match policy.apply_continuation(leaking).result {
            ToolContinuationOutcome::Completed { value } => {
                let text = serde_json::to_value(value).unwrap()["content"][0]["text"]
                    .as_str()
                    .unwrap()
                    .to_owned();
                assert!(
                    !text.contains("sk-1234567890abcdef"),
                    "secret markers are redacted: {text}"
                );
            }
            other => panic!("redaction keeps a completed outcome: {other:?}"),
        }
        let mut failing = ProductionToolControlPolicy::new(tools::SecretScanner::failing());
        assert!(
            matches!(failing.apply_continuation(completed).result, ToolContinuationOutcome::Failed { ref error } if error.code == ToolControlErrorCode::Denied)
        );
        assert!(
            failing
                .apply(pending)
                .error
                .is_some_and(|error| error.code == ToolControlErrorCode::Denied)
        );
    }
}
