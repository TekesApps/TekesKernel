use engine::compaction_anchor_sequences;
mod identity;
mod workspace_edits;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Read, Write};
use std::net::IpAddr;
use std::os::fd::FromRawFd;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::process::ExitCode;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::time::{Duration, Instant};

use base64::Engine as _;
use engine::{
    AdapterCapabilities, CatalogEntry, Continuation, DurableArtifactVersions, DynamicBackend,
    DynamicSupervisorBackend, DynamicToolDispatcher, LockFacts, PermissionModePolicy,
    QueryCapability, QueryResult, RecoveryDecision, RunDecision, RunMode, SupervisorControlBackend,
    SystemToolBackend, SystemToolConfig, TailState, ToolBackend, ToolBackendRouter, ToolDispatcher,
    ToolInvocation, WorkflowBackend, classify, decide_recovery, run_decision, sent_state,
};
use profile::{
    ConfigSnapshot, DynamicTool, DynamicToolEffect, InstructionKind, InstructionSnapshot,
    LaunchBindings, Model, Provider, ResourceCatalog, WebSearch,
};
use provider::{
    ContentBlock, CredentialClient, CredentialGet, DialectId, FinishReason, HttpRuntime,
    PrepareInput, ProviderCompletion, ProviderFailure, ProviderFrame, ProviderTerminal,
    credential_request_id, endpoint_origin, prepare,
};
use schema::{Event, EventKind, IJsonValue, ResumePolicy, Visibility};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use store::{
    AssetStore, BarrierContext, DirectoryLock, LockedLedger, StoreError, scan_valid_prefix,
};
use tools::{
    Backend, BackendFailure, BackendTerminal, BoundedHttpClient, BuiltinManifest,
    CancellationToken, CatalogContext, CatalogRole, DurableApprovalResponse, HelperClient,
    HelperInvoker, HookBinding, HttpLimits, NetworkPolicy, PipelineDecision, ProbeStatus,
    PublicRoute, SandboxBackend, SandboxPolicy, SearchHit, SearchProvider, SearchRequest,
    SearchTopic, SecretScan, SecretScanner, ToolExecution, ToolPipeline, decode_hook_binding,
    is_public_internet_address, probe_backend, route_public_url,
};
use worker_control::continuation::{
    ContinuationOperation, ToolContinuationOutcome, ToolContinuationRequest,
    ToolContinuationResponse, decode_tool_continuation_result, encode_tool_continuation,
};
use worker_control::{
    Appended, AttemptSettled, EX_PROTOCOL, Frame, FrameChannel, LaunchChild, LaunchResult,
    LeaseRequest, Receipt, Selected, SupervisorMessage, decode_reject, decode_supervisor,
    encode_line,
};
use worker_control::{
    Hello, QueueTransaction, QueueTransactionAction, QueueTransactionOutcome,
    QueueTransactionRejectCode, QueueTransactionResult, ToolControl, ToolControlErrorCode,
    ToolControlResult, WorkerStartup, decode_queue_transaction, decode_selection,
    decode_tool_control_result, encode_queue_transaction_result, encode_tool_control,
    require_version,
};

struct Options {
    ledger: PathBuf,
    timestamp: String,
    clock_started: Instant,
    run_id: String,
    binary: String,
    config_digest: String,
    instruction_digest: String,
    launch_bindings_digest: Option<String>,
    policy: String,
    config_fd: Option<i32>,
    instruction_fd: Option<i32>,
    launch_bindings_fd: Option<i32>,
    credential_fd: Option<i32>,
    web_search_ready: bool,
    provider_test_redirect: Option<String>,
    lifecycle_hooks: Option<lifecycle_hooks::LifecycleHooks>,
}

impl Options {
    fn event_timestamp(&self) -> String {
        if cfg!(test) {
            return self.timestamp.clone();
        }
        let Ok(anchor) = chrono::DateTime::parse_from_rfc3339(&self.timestamp) else {
            return self.timestamp.clone();
        };
        let elapsed = chrono::Duration::from_std(self.clock_started.elapsed()).unwrap_or_default();
        anchor
            .checked_add_signed(elapsed)
            .unwrap_or(anchor)
            .with_timezone(&chrono::Utc)
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
    }
}

#[derive(Clone)]
struct RuntimeProfile {
    config: ConfigSnapshot,
    instruction: InstructionSnapshot,
    bindings: LaunchBindings,
    validator: bool,
    subagent: bool,
}

mod lifecycle_hooks;
mod result_presentation;
mod validation_runtime;
mod web_search;
use web_search::*;
mod control_channel;
use control_channel::*;
mod queue;
use queue::*;
mod compaction;
use compaction::*;
mod ledger_events;
use ledger_events::*;
mod delivery;
use delivery::*;
mod recovery;
use recovery::*;
mod tool_backends;
use tool_backends::*;
mod tool_calls;
use tool_calls::*;
mod continuation;
use continuation::*;
mod child_agents;
use child_agents::*;
mod turn_terminal;
use turn_terminal::*;
mod context_projection;
use context_projection::*;

#[cfg(test)]
thread_local! {
    /// Hermetic transport seam: profile resolution and request digests stay bound to the proved
    /// route, while the unit test sends those already-prepared bytes to a loopback fixture.
    static PROVIDER_TEST_REDIRECT: RefCell<Option<String>> = const { RefCell::new(None) };
}

#[cfg(test)]
fn set_provider_test_redirect(endpoint: Option<String>) {
    PROVIDER_TEST_REDIRECT.with(|slot| *slot.borrow_mut() = endpoint);
}

struct ProviderOutcome {
    seq: u64,
    settle: Option<(&'static str, Option<&'static str>)>,
    retry: bool,
    retry_classification: Option<&'static str>,
    /// Provider-declared admission delay for a rate-limited retry.
    retry_after_seconds: Option<u64>,
    compact_retry: bool,
    tool_calls: Vec<provider::ToolCall>,
}

struct ProviderContext {
    items: Vec<IJsonValue>,
    admits: Vec<u64>,
    continuation_id: Option<String>,
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("tekes-worker: {error}");
            if is_protocol_failure(error.as_ref()) {
                ExitCode::from(EX_PROTOCOL as u8)
            } else {
                ExitCode::FAILURE
            }
        }
    }
}

fn is_protocol_failure(mut error: &(dyn std::error::Error + 'static)) -> bool {
    loop {
        if error.is::<ProtocolFailure>()
            || error.is::<worker_control::ProtocolError>()
            || error.is::<worker_control::DurableControlError>()
            || matches!(
                error.downcast_ref::<StoreError>(),
                Some(StoreError::VersionGate { .. })
            )
        {
            return true;
        }
        let Some(source) = error.source() else {
            return false;
        };
        error = source;
    }
}

fn run() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let mut options = parse_options()?;
    let cancellation = RuntimeCancellation::default();
    let mut lines = spawn_control_reader(cancellation.clone());
    let mut stdout = io::stdout().lock();

    stdout.write_all(&encode_line("hello", &Hello::default())?)?;
    stdout.flush()?;
    let Some(selection) = lines.next() else {
        return Ok(ExitCode::from(EX_PROTOCOL as u8));
    };
    let selection = selection.map_err(|error| {
        ProtocolFailure(format!(
            "supervisor read failed during negotiation: {error}"
        ))
    })?;
    let selection = match decode_selection(selection.as_bytes()) {
        Ok(selected) => selected,
        Err(_) if decode_reject(selection.as_bytes()).is_ok() => {
            return Ok(ExitCode::from(EX_PROTOCOL as u8));
        }
        Err(_) => return Ok(ExitCode::from(EX_PROTOCOL as u8)),
    };
    let startup_transaction = if selection.startup == Some(WorkerStartup::QueueTransaction) {
        let Some(line) = lines.next() else {
            return Ok(ExitCode::from(EX_PROTOCOL as u8));
        };
        let line = line.map_err(|error| {
            ProtocolFailure(format!(
                "supervisor read failed during queue-transaction startup: {error}"
            ))
        })?;
        match decode_queue_transaction(line.as_bytes()) {
            Ok(transaction) => Some(transaction),
            Err(_) => return Ok(ExitCode::from(EX_PROTOCOL as u8)),
        }
    } else {
        None
    };
    let selected = selection.selected();
    let mut runtime_profile = load_profiles(&mut options)?;
    let manifest = BuiltinManifest::compiled();
    manifest.validate()?;
    require_version(&selected)?;

    let mut credential = match options.credential_fd.take() {
        Some(fd) => match provider::CredentialClient::from_inherited_fd(fd) {
            Ok(client) => Some(Arc::new(Mutex::new(client))),
            Err(_) => return Ok(ExitCode::from(EX_PROTOCOL as u8)),
        },
        None => None,
    };
    if options.web_search_ready
        && (credential.is_none()
            || runtime_profile
                .as_ref()
                .and_then(|profile| profile.config.providers.web_search.as_ref())
                .is_none())
    {
        return Ok(ExitCode::from(EX_PROTOCOL as u8));
    }

    let folder = options
        .ledger
        .parent()
        .ok_or("ledger path has no thread folder")?;
    let _lifecycle = DirectoryLock::shared(folder)?;
    let mut ledger = match LockedLedger::open(&options.ledger, 1) {
        Ok(ledger) => ledger,
        Err(StoreError::Busy) => return Ok(ExitCode::SUCCESS),
        Err(error) => return Err(error.into()),
    };
    if let Some(profile) = runtime_profile.as_mut() {
        validation_runtime::activate_validator_profile(&ledger, profile)?;
        activate_subagent_profile(&ledger, profile)?;
    }
    options.lifecycle_hooks = match runtime_profile
        .as_ref()
        .map(|profile| lifecycle_hooks::LifecycleHooks::load(profile, &ledger))
        .transpose()
    {
        Ok(hooks) => hooks.flatten(),
        Err(_) => {
            eprintln!("lifecycle-hooks unavailable: initialization failed");
            None
        }
    };
    if let Some(hooks) = &mut options.lifecycle_hooks {
        hooks.cancelled = Arc::clone(&cancellation.provider);
    }
    if let Some(hooks) = &options.lifecycle_hooks {
        hooks.observe(&mut ledger);
    }
    let result = (|| -> Result<ExitCode, Box<dyn std::error::Error>> {
        // A crash between a completed final and the next goal turn must not
        // strand an active goal on a settled tail. This is idempotent because
        // only a settled tail can admit the next turn.
        if startup_transaction.is_none() {
            if let Some(profile) = &runtime_profile {
                open_goal_continuation(&mut ledger, &options, profile, &cancellation)?;
            }
        }
        let facts = ledger
            .projection()
            .ok_or("worker cannot run an empty ledger")?
            .lifecycle
            .clone();
        if let Some(profile) = &runtime_profile {
            let ledger_workspace = ledger
                .projection()
                .and_then(|projection| projection.events.first())
                .and_then(|genesis| genesis.string_field("workspace"));
            if ledger_workspace != Some(profile.config.workspace.id.as_str()) {
                return Err("config snapshot workspace does not match genesis".into());
            }
        }
        let state = classify(&facts, LockFacts::CALLER);
        let decision = if startup_transaction.is_some() {
            RunDecision::Start(RunMode::Reconcile)
        } else if !facts.stop_active && validation_runtime::has_pending_validation(&ledger)? {
            // Finishing a durable validation obligation is not an unsolicited
            // resume of a completed session. In particular, resume=never must not
            // discard a held candidate or its committed repair route after a crash.
            RunDecision::Start(RunMode::Ordinary)
        } else {
            run_decision(state, &facts)
        };
        let RunDecision::Start(mode) = decision else {
            return Ok(ExitCode::SUCCESS);
        };

        append_run_start(&mut ledger, &options, mode, facts.recovery_ordinal)?;
        if let Some(transaction) = startup_transaction {
            let result =
                execute_queue_transaction(&mut ledger, &options.event_timestamp(), &transaction)?;
            stdout.write_all(&encode_queue_transaction_result(&result)?)?;
            stdout.flush()?;
            announce_appended(&ledger, &mut stdout)?;
            return Ok(ExitCode::SUCCESS);
        }
        open_ready_turn(&mut ledger, &options.event_timestamp(), mode)?;
        if matches!(state, TailState::StoppedActive) && facts.unstarted {
            let event = make_event(json!({
                "v": 1,
                "seq": ledger.next_seq(),
                "turn": 1,
                "kind": "settle",
                "ts": options.event_timestamp(),
                "outcome": "interrupted",
                "reason": "recovered"
            }))?;
            ledger.append_contract(event, BarrierContext::default())?;
            announce_appended(&ledger, &mut stdout)?;
            return Ok(ExitCode::SUCCESS);
        }

        if recover_unpaired_tool_calls(
            &mut ledger,
            &options,
            runtime_profile.as_ref(),
            selected,
            matches!(mode, RunMode::Reconcile),
            &mut lines,
            &mut stdout,
            &cancellation,
        )? {
            announce_appended(&ledger, &mut stdout)?;
            return Ok(ExitCode::SUCCESS);
        }

        if matches!(mode, RunMode::Reconcile) {
            reconcile_provider_attempt(&mut ledger, &options, &mut stdout)?;
            announce_appended(&ledger, &mut stdout)?;
            return Ok(ExitCode::SUCCESS);
        }

        if matches!(mode, RunMode::Ordinary) {
            if let Some(profile) = &runtime_profile {
                loop {
                    let provider_result = run_provider_turn(
                        &mut ledger,
                        &options,
                        profile,
                        &selected,
                        &mut credential,
                        &mut lines,
                        &mut stdout,
                        &cancellation,
                    );
                    if let Err(error) = provider_result {
                        if is_protocol_failure(error.as_ref()) {
                            return Err(error);
                        }
                        settle_internal_worker_failure(&mut ledger, &options, error.as_ref())?;
                    }
                    drain_ready_deliveries(
                        &mut ledger,
                        &options,
                        &selected,
                        &mut lines,
                        &mut stdout,
                    )?;
                    if !open_goal_continuation(&mut ledger, &options, profile, &cancellation)? {
                        break;
                    }
                }
            } else {
                drain_ready_deliveries(&mut ledger, &options, &selected, &mut lines, &mut stdout)?;
            }
            return post_turn_exit(&mut ledger, &options, &cancellation, &mut stdout);
        }
        Ok(ExitCode::SUCCESS)
    })();
    if let Some(hooks) = &options.lifecycle_hooks {
        hooks.observe(&mut ledger);
    }
    result
}

/// Worker-side correlated supervisor-control exchange. The version gate runs
/// before the request is written, so a v1 session cannot start an effect.
pub fn exchange_tool_control(
    selected: &Selected,
    request: &ToolControl,
    lines: &mut impl Iterator<Item = io::Result<String>>,
    stdout: &mut impl Write,
) -> Result<ToolControlResult, Box<dyn std::error::Error>> {
    require_version(selected)?;
    stdout.write_all(&encode_tool_control(request)?)?;
    stdout.flush()?;
    loop {
        let line = lines
            .next()
            .ok_or("supervisor EOF while awaiting tool-control result")??;
        match decode_tool_control_result(line.as_bytes()) {
            Ok(result) => {
                result.validate_for(request)?;
                return Ok(result);
            }
            Err(_) => match decode_supervisor(line.as_bytes())? {
                SupervisorMessage::Ping(ping) => {
                    stdout
                        .write_all(&encode_line("pong", &worker_control::Pong { id: ping.id })?)?;
                    stdout.flush()?;
                }
                _ => {
                    return Err(
                        "unexpected supervisor message while awaiting tool-control result".into(),
                    );
                }
            },
        }
    }
}

fn parse_options() -> Result<Options, Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let ledger = args
        .next()
        .ok_or("usage: tekes-worker <ledger> --timestamp <ISO8601>")?;
    let mut options = Options {
        ledger: PathBuf::from(ledger),
        timestamp: String::new(),
        clock_started: Instant::now(),
        run_id: format!("run-{}", std::process::id()),
        binary: option_env!("TEKES_SELECTED_BUILD")
            .unwrap_or(env!("CARGO_PKG_VERSION"))
            .to_owned(),
        config_digest: "injected-config".to_owned(),
        instruction_digest: "injected-instructions".to_owned(),
        launch_bindings_digest: None,
        policy: "default".to_owned(),
        config_fd: None,
        instruction_fd: None,
        launch_bindings_fd: None,
        credential_fd: None,
        web_search_ready: false,
        provider_test_redirect: None,
        lifecycle_hooks: None,
    };
    while let Some(flag) = args.next() {
        let value = args.next().ok_or("missing option value")?;
        match flag.as_str() {
            "--timestamp" => options.timestamp = value,
            "--run-id" => options.run_id = value,
            "--binary" => options.binary = value,
            "--config-digest" => options.config_digest = value,
            "--instruction-digest" => options.instruction_digest = value,
            "--launch-bindings-digest" => options.launch_bindings_digest = Some(value),
            "--policy" => options.policy = value,
            "--config-fd" => options.config_fd = Some(parse_fd(&value)?),
            "--instruction-fd" => options.instruction_fd = Some(parse_fd(&value)?),
            "--launch-bindings-fd" => options.launch_bindings_fd = Some(parse_fd(&value)?),
            "--credential-fd" => options.credential_fd = Some(parse_fd(&value)?),
            "--web-search-ready" => {
                options.web_search_ready = match value.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => return Err("--web-search-ready must be true or false".into()),
                }
            }
            "--provider-test-redirect" => {
                if !cfg!(debug_assertions) || !value.starts_with("http://127.0.0.1:") {
                    return Err("--provider-test-redirect is a debug-only loopback seam".into());
                }
                options.provider_test_redirect = Some(value);
            }
            _ => return Err(format!("unknown option {flag}").into()),
        }
    }
    if options.event_timestamp().is_empty() {
        return Err("--timestamp is required".into());
    }
    if options.web_search_ready && options.credential_fd.is_none() {
        return Err("--web-search-ready requires --credential-fd".into());
    }
    Ok(options)
}

fn parse_fd(value: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let fd = value.parse::<i32>()?;
    if fd < 0 {
        return Err("snapshot descriptor must be nonnegative".into());
    }
    Ok(fd)
}

#[allow(unsafe_code)]
fn read_inherited_fd(fd: i32) -> io::Result<Vec<u8>> {
    // These descriptors are transferred solely to this worker at launch. Reading
    // them directly also works inside the macOS App Sandbox, where reopening
    // /dev/fd/N can block before the worker has loaded its runtime profile.
    // SAFETY: the supervisor supplies an open, uniquely owned descriptor for
    // each snapshot; this function consumes and closes it exactly once.
    let mut file = unsafe { File::from_raw_fd(fd) };
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn load_profiles(
    options: &mut Options,
) -> Result<Option<RuntimeProfile>, Box<dyn std::error::Error>> {
    match (
        options.config_fd,
        options.instruction_fd,
        options.launch_bindings_fd,
        options.launch_bindings_digest.as_deref(),
    ) {
        (None, None, None, None) => Ok(None),
        (Some(config_fd), Some(instruction_fd), Some(bindings_fd), Some(bindings_digest)) => {
            if config_fd <= 2
                || instruction_fd <= 2
                || bindings_fd <= 2
                || config_fd == instruction_fd
                || config_fd == bindings_fd
                || instruction_fd == bindings_fd
            {
                return Err("profile descriptors must be distinct and above stderr".into());
            }
            let config_bytes = read_inherited_fd(config_fd)?;
            let instruction_bytes = read_inherited_fd(instruction_fd)?;
            let bindings_bytes = read_inherited_fd(bindings_fd)?;
            let config = profile::ConfigSnapshot::decode(&config_bytes)?;
            let instruction = profile::InstructionSnapshot::decode(&instruction_bytes)?;
            instruction.validate_against_config(&config)?;
            let bindings = LaunchBindings::decode_verified(
                &bindings_bytes,
                bindings_digest,
                &config,
            )?;
            let policy = instruction.meet_workspace_policy(&config.workspace.policy);
            options.config_digest = config.digest()?;
            options.instruction_digest = instruction.digest()?;
            options.launch_bindings_digest = Some(bindings.digest()?);
            options.policy = format!("sha256-{}", policy.digest()?);
            Ok(Some(RuntimeProfile {
                config,
                instruction,
                bindings,
                validator: false,
            subagent: false,
            }))
        }
        _ => Err(
            "config-fd, instruction-fd, launch-bindings-fd and launch-bindings-digest must be supplied together"
                .into(),
        ),
    }
}

fn activate_subagent_profile(
    ledger: &LockedLedger,
    profile: &mut RuntimeProfile,
) -> Result<(), Box<dyn std::error::Error>> {
    if profile.validator {
        return Ok(());
    }
    let genesis = ledger
        .projection()
        .and_then(|p| p.events.first())
        .ok_or("missing genesis")?;
    let raw = serde_json::to_value(genesis.raw())?;
    let Some(parent) = raw.get("parent") else {
        return Ok(());
    };
    if genesis.string_field("thread") != ledger.path().file_stem().and_then(|n| n.to_str()) {
        return Err("child genesis identity does not match its line".into());
    }
    let file = parent["file"].as_str().ok_or("child parent file missing")?;
    if std::path::Path::new(file)
        .file_name()
        .and_then(|n| n.to_str())
        != Some(file)
        || file == "."
        || file == ".."
    {
        return Err("invalid child parent file".into());
    }
    let parent_projection = read_child_projection(
        &ledger
            .path()
            .parent()
            .ok_or("missing child folder")?
            .join(file),
    )?;
    let seq = parent["seq"].as_u64().ok_or("child parent seq missing")?;
    let spawn = parent_projection
        .events
        .iter()
        .find(|e| e.seq() == seq)
        .ok_or("child parent spawn missing")?;
    if spawn.kind() != &EventKind::Spawn
        || spawn.string_field("spawn_id") != parent["spawn_id"].as_str()
        || spawn.string_field("child") != ledger.path().file_name().and_then(|n| n.to_str())
    {
        return Err("child role does not match durable parent spawn".into());
    }
    let call = spawn
        .string_field("call")
        .ok_or("delegated spawn lacks call")?;
    if !parent_projection.events.iter().any(|e| {
        e.kind() == &EventKind::ToolCall
            && e.seq() < spawn.seq()
            && e.string_field("call") == Some(call)
            && matches!(e.string_field("name"), Some("task" | "subagent"))
    }) {
        return Err("child role lacks a task or subagent invocation".into());
    }
    profile.subagent = true;
    Ok(())
}

/// `tekes-helper` ships beside the worker executable (`BuiltInKernel/`, or
/// `target/<profile>/` for a cargo build). A unit test binary runs from
/// `target/<profile>/deps/`, one level below the helper Cargo built for the
/// `tools` dependency; that fallback is the only difference, and it applies
/// only when no helper sits beside the executable.
fn sibling_helper_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let executable = std::env::current_exe()?;
    let directory = executable
        .parent()
        .ok_or("worker executable has no parent directory")?;
    let sibling = directory.join("tekes-helper");
    if cfg!(test) && !sibling.is_file() && directory.file_name().is_some_and(|name| name == "deps")
    {
        if let Some(parent) = directory.parent() {
            let built = parent.join("tekes-helper");
            if built.is_file() {
                return Ok(built);
            }
        }
    }
    Ok(sibling)
}

fn effective_allowed_tools(profile: &RuntimeProfile) -> BTreeSet<String> {
    let mut tools: BTreeSet<String> = profile
        .instruction
        .meet_workspace_policy(&profile.config.workspace.policy)
        .allowed_tools
        .into_iter()
        // Historical policies may still select the retired memory builtins.
        .filter(|name| !matches!(name.as_str(), "note" | "recall"))
        .collect();
    if profile.validator {
        tools.insert("verify".to_owned());
    }
    if profile.subagent {
        tools.insert("report".to_owned());
    }
    tools
}

#[allow(clippy::too_many_arguments)]
fn run_provider_turn(
    ledger: &mut LockedLedger,
    options: &Options,
    profile: &RuntimeProfile,
    selected: &Selected,
    credential: &mut Option<Arc<Mutex<CredentialClient>>>,
    lines: &mut impl Iterator<Item = io::Result<String>>,
    stdout: &mut impl Write,
    cancellation: &RuntimeCancellation,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(hooks) = &options.lifecycle_hooks {
        ledger.sync_prefix()?;
        if let Some(turn) = ledger.projection().and_then(|p| p.latest_turn) {
            if !ledger.projection().is_some_and(|p| p.terminal_tail) {
                hooks.before_turn(ledger, turn);
            }
        }
    }
    let mut execution_profile = profile.clone();
    validation_runtime::activate_validator_profile(ledger, &mut execution_profile)?;
    let profile = &execution_profile;
    let wall_deadline = profile
        .config
        .workspace
        .policy
        .max_wall_seconds
        .map(|seconds| Instant::now() + Duration::from_secs(seconds));
    let result = run_provider_turn_inner(
        ledger,
        ProviderRunContext {
            options,
            profile,
            selected,
        },
        credential,
        lines,
        stdout,
        ProviderLoopBudget {
            retries_left: PROVIDER_RETRIES,
            compactions_left: 1,
            trims_left: 1,
            nonfinal_responses_left: 32,
            wall_deadline,
        },
        cancellation,
    );
    match result {
        Ok(()) => Ok(()),
        Err(error) if is_protocol_failure(error.as_ref()) => Err(error),
        // Stop owns its receipt and interrupted settlement in post_turn_exit.
        // Child launch/wait cancellation must not preempt it with internal error.
        Err(_) if cancellation.stop_requested() => Ok(()),
        Err(error) => match settle_internal_worker_failure(ledger, options, error.as_ref()) {
            Ok(()) => Ok(()),
            Err(_) => Err(error),
        },
    }
}

#[derive(Clone, Copy)]
struct ProviderLoopBudget {
    retries_left: u8,
    compactions_left: u8,
    trims_left: u8,
    nonfinal_responses_left: u8,
    wall_deadline: Option<Instant>,
}

#[derive(Clone, Copy)]
struct ProviderRunContext<'a> {
    options: &'a Options,
    profile: &'a RuntimeProfile,
    selected: &'a Selected,
}

fn run_provider_turn_inner(
    ledger: &mut LockedLedger,
    run: ProviderRunContext<'_>,
    credential: &mut Option<Arc<Mutex<CredentialClient>>>,
    lines: &mut impl Iterator<Item = io::Result<String>>,
    stdout: &mut impl Write,
    budget: ProviderLoopBudget,
    cancellation: &RuntimeCancellation,
) -> Result<(), Box<dyn std::error::Error>> {
    let ProviderRunContext {
        options,
        profile,
        selected,
    } = run;
    let turn = ledger
        .projection()
        .and_then(|projection| projection.latest_turn)
        .ok_or("provider turn requires turn_open")?;
    if budget
        .wall_deadline
        .is_some_and(|deadline| Instant::now() >= deadline)
    {
        append_settle(
            ledger,
            &options.event_timestamp(),
            turn,
            "interrupted",
            Some("budget_wall"),
        )?;
        return Ok(());
    }
    if ledger
        .projection()
        .is_some_and(|projection| projection.terminal_tail || projection.lifecycle.open_hold)
    {
        return Ok(());
    }
    let recovery = recover_provider_attempt_for_ordinary(ledger, options, stdout)?;
    let RecoveryProgress::Continue { reset_epoch } = recovery else {
        return Ok(());
    };
    if validation_runtime::advance(ledger, run, lines, stdout, cancellation)? {
        return Ok(());
    }
    let (provider_config, model) = match selected_provider(&profile.config) {
        Ok(selected) => selected,
        Err(reason) => {
            settle_provider_unavailable(ledger, options, &reason.detail())?;
            return Ok(());
        }
    };
    let manifest = BuiltinManifest::compiled();
    manifest.validate()?;
    let thread_folder = ledger
        .path()
        .parent()
        .ok_or("worker ledger has no thread folder")?
        .to_path_buf();
    let resolved_profile = match provider::resolve_profile(provider_config, model) {
        Ok(profile) => profile,
        Err(error) => {
            settle_provider_unavailable(ledger, options, &format!("config: {error}"))?;
            return Ok(());
        }
    };
    let dialect = resolved_profile.dialect;
    // Native deferred-tool routing is a property of the exact route (dialect,
    // model, endpoint owner, gateway translation) declared in the reviewed
    // capability catalog, never of the adapter family.
    let native_mode = resolved_profile.native_deferred_tools();
    let mut native_tools: Option<provider::NativeDeferredTools> = None;
    let provider_catalog = {
        let (backend, dynamic_backend) = assemble_tool_backends(
            ToolBackendPlan {
                profile,
                selected: *selected,
                thread_folder: &thread_folder,
                invocation_write_roots: &[],
                credential: credential.as_ref(),
                web_search_ready: options.web_search_ready,
            },
            ToolBackendRuntime {
                lines,
                stdout,
                cancellation,
            },
        )?;
        let dynamic_dispatcher = DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog);
        let deferred = dynamic_dispatcher.deferred_search_entries(&dynamic_backend)?;
        let has_deferred = !deferred.is_empty();
        let catalog_context =
            tool_catalog_context(profile, has_deferred, options.web_search_ready)?;
        let mut catalog =
            ToolDispatcher::new(&manifest, &catalog_context).provider_catalog(&backend)?;
        let before_seq = ledger.next_seq();
        let pipeline = ToolPipeline::new(
            ledger,
            AssetStore::new(thread_folder.join("assets"))?,
            SecretScanner::default(),
        );
        catalog.extend(dynamic_dispatcher.provider_catalog_visible(
            &dynamic_backend,
            &pipeline,
            before_seq,
        )?);
        if native_mode.is_some() && has_deferred {
            // Native mode declares every deferred schema to the provider (the
            // adapter marks them deferred) and carries this turn's durable
            // search offers as bound references; nothing else changes.
            let declared = catalog
                .iter()
                .filter_map(|schema| {
                    serde_json::to_value(schema)
                        .ok()?
                        .get("name")?
                        .as_str()
                        .map(str::to_owned)
                })
                .collect::<BTreeSet<_>>();
            for entry in &deferred {
                if !declared.contains(&entry.name) {
                    catalog.push(entry.content.clone());
                }
            }
            let catalog_revision = dynamic_catalog_revision(&deferred);
            drop(pipeline);
            native_tools = Some(provider::NativeDeferredTools {
                references: native_search_references(
                    ledger,
                    turn,
                    before_seq,
                    &deferred,
                    &catalog_revision,
                )?,
                catalog_revision,
                search_tool_name: "tool_search".to_owned(),
                definitions: deferred
                    .iter()
                    .map(|entry| provider::DeferredToolDefinition {
                        schema: entry.content.clone(),
                        schema_digest: entry.schema_digest.clone(),
                    })
                    .collect(),
            });
        }
        catalog
    };
    let tool_catalog = ijson(Value::Array(
        provider_catalog
            .into_iter()
            .map(|value| serde_json::to_value(value).expect("tool schema is JSON"))
            .collect(),
    ))?;
    let tools_digest = format!("{:x}", Sha256::digest(tool_catalog.canonical_bytes()?));
    let consumed = current_turn_inputs(ledger, turn)?;
    // The root prompt is the runtime-owned identity, profile text and working
    // directory followed by the AGENTS.md scopes (spec builtin-tools
    // §Model-facing tool guidance). The validator
    // keeps its own judge prompt.
    let system = if profile.validator {
        validation_runtime::JUDGE_SYSTEM.to_owned()
    } else {
        let identity = identity::resolve(ledger, &options.event_timestamp(), turn)?;
        let root = tools::root_system_instructions(
            identity,
            resolved_profile.wire_model(),
            profile.config.execution_cwd(),
            &effective_system(&profile.instruction),
        );
        let root = if let Some(bound_id) = profile.bindings.goal_id.as_deref() {
            if let Some((storage_root, session)) = goal_store_location(ledger)? {
                match session_controls::read_goal(&storage_root, &session)? {
                    Some(goal) if goal.id == bound_id => format!(
                        "{root}\n\nSession goal (authoritative, separate from conversation history):\nObjective: {}\nState: {}\nContinue pursuing this objective while it is active. A final answer ends only this turn. Mark the whole goal complete with set_goal_state only when its objective is achieved.",
                        goal.objective, goal.phase
                    ),
                    _ => root,
                }
            } else {
                root
            }
        } else {
            root
        };
        if profile.subagent {
            format!(
                "{root}\n\nYou are a delegated subagent. When your work is complete, call report with one self-contained result for your parent. Final prose alone does not deliver your result to the parent."
            )
        } else {
            root
        }
    };
    let profile_value = provider::epoch_profile(
        &resolved_profile,
        &system,
        profile.config.session_settings.as_ref(),
    )?;
    let profile_digest = format!("{:x}", Sha256::digest(profile_value.canonical_bytes()?));
    let mut epoch = (!reset_epoch)
        .then(|| {
            latest_compatible_epoch(
                ledger,
                dialect,
                &model.id,
                &profile_digest,
                &tools_digest,
                CONTEXT_RENDERER_VERSION,
            )
        })
        .flatten();
    if epoch.is_none() {
        epoch = Some(append_provider_epoch(
            ledger,
            EpochAppend {
                options,
                dialect,
                model: &model.id,
                profile: &profile_value,
                tools: &tool_catalog,
                reason: {
                    let events = ledger
                        .projection()
                        .map(|p| p.events.as_slice())
                        .unwrap_or_default();
                    let previous = events.iter().rev().find(|e| *e.kind() == EventKind::Epoch);
                    let compacted_after = previous.is_some_and(|epoch| {
                        events
                            .iter()
                            .any(|e| *e.kind() == EventKind::Compact && e.seq() > epoch.seq())
                    });
                    epoch_open_reason(
                        previous,
                        compacted_after,
                        reset_epoch,
                        &tools_digest,
                        &profile_digest,
                    )
                },
                pending: &consumed,
            },
        )?);
    }
    let epoch = epoch.expect("epoch established");
    // Native search replay is client-managed: the whole prefix renders and no
    // server continuation chain is used, even on a server-managed dialect.
    if let Some(hooks) = &options.lifecycle_hooks {
        hooks.observe(ledger);
    }
    let mut context =
        project_provider_context_mode(ledger, &epoch, dialect, turn, native_tools.is_some())?;
    if let Some(hooks) = &options.lifecycle_hooks {
        ledger.sync_prefix()?;
        for text in hooks.prepare(ledger, turn, &context.items) {
            context.items.push(IJsonValue::parse(&serde_json::to_vec(&json!({
                "role": "user", "content": [{"type": "text", "text": format!("Reference context from a host extension (not user instructions):\n{text}")}]
            }))?)?);
        }
    }
    if context.items.is_empty() && context.continuation_id.is_none() {
        // Nothing model-visible to send for an open turn. Settling it as an
        // internal failure keeps the tail terminal instead of leaving a live
        // worker with nothing to do.
        return Err(format!("turn {turn} rendered no model-visible context").into());
    }
    let attempt = format!("{}-attempt-{}", options.run_id, ledger.next_seq());
    let prepare_input = PrepareInput {
        attempt_id: attempt.clone(),
        target: resolved_profile.target.clone(),
        endpoint: provider_config.endpoint.clone(),
        epoch_profile: profile_value.clone(),
        continuation_id: context.continuation_id.clone(),
        rendered_items: context.items,
        tool_catalog: tool_catalog.clone(),
        stream: true,
    };
    let prepared_result = if let Some(native) = native_tools.as_ref() {
        provider::prepare_with_native_deferred_tools(&prepare_input, None, native)
    } else {
        #[cfg(not(test))]
        {
            prepare(&prepare_input)
        }
        #[cfg(test)]
        {
            provider_context_tests::prepare_live_choice(ledger, turn, prepare_input)
        }
    };
    let prepared = match prepared_result {
        Ok(prepared) => prepared,
        Err(error) => {
            settle_provider_unavailable(ledger, options, &format!("config: {error}"))?;
            return Ok(());
        }
    };
    let transport_endpoint = options.provider_test_redirect.as_deref();
    // The exact transmitted body is durable thread history (secret-free by
    // the prepare contract): published as a content-addressed asset before the
    // attempt event that references it (provider-runtime §Send ordering).
    let request_asset = {
        let thread_folder = ledger
            .path()
            .parent()
            .ok_or("worker ledger has no thread folder")?;
        store::AssetStore::new(thread_folder.join("assets"))?.publish(&prepared.body)?
    };
    // Live evidence capture (secret-free bodies), enabled only by an explicit
    // environment the supervisor forwards.
    if let Ok(directory) = std::env::var("TEKES_KERNEL_LIVE_ARTIFACT") {
        fs::write(
            PathBuf::from(directory).join(format!("{attempt}.request.json")),
            &prepared.body,
        )?;
    }
    #[cfg(test)]
    let thread_transport_endpoint = PROVIDER_TEST_REDIRECT.with(|slot| slot.borrow().clone());
    #[cfg(test)]
    let transport_endpoint = thread_transport_endpoint.as_deref().or(transport_endpoint);
    let origin = match endpoint_origin(&provider_config.endpoint) {
        Ok(origin) => origin,
        Err(error) => {
            settle_provider_unavailable(ledger, options, &format!("config: {error}"))?;
            return Ok(());
        }
    };
    let cancelled = Arc::clone(&cancellation.provider);
    // Material is resolved for this attempt and dropped (zeroed) when the
    // attempt settles; rotation and revocation reach the next attempt's get.
    let credential_material = match (&provider_config.credential_key, credential.as_ref()) {
        (Some(key), Some(client)) => match client
            .lock()
            .map_err(|_| "credential client lock poisoned")?
            .get(CredentialGet {
                request_id: credential_request_id(&attempt, key, &origin),
                attempt: attempt.clone(),
                credential_id: key.clone(),
                purpose: "provider".to_owned(),
                adapter: provider_config.adapter.clone(),
                endpoint_origin: origin,
            }) {
            Ok(material) => Some(material),
            Err(provider::CredentialClientError::Rejected(code)) => {
                settle_provider_unavailable(ledger, options, &format!("credential_{code}"))?;
                return Ok(());
            }
            Err(error) => {
                settle_provider_unavailable(
                    ledger,
                    options,
                    &format!("config: credential broker: {error}"),
                )?;
                return Ok(());
            }
        },
        (Some(_), None) => {
            settle_provider_unavailable(
                ledger,
                options,
                "config: configured provider requires a credential broker",
            )?;
            return Ok(());
        }
        (None, _) => None,
    };
    drain_parked_deliveries(ledger, options, run.selected, cancellation, stdout)?;
    if latest_compatible_epoch(
        ledger,
        dialect,
        &model.id,
        &profile_digest,
        &tools_digest,
        CONTEXT_RENDERER_VERSION,
    )
    .as_deref()
        != Some(epoch.as_str())
    {
        drop(credential_material);
        return run_provider_turn_inner(
            ledger,
            run,
            credential,
            lines,
            stdout,
            budget,
            cancellation,
        );
    }
    // D-39 preflight: a compatible provider-reported usage anchors the token
    // estimate; each appended byte counts as a possible token. If no such
    // observation exists, retain the conservative byte bound. A candidate
    // above the trigger compacts before send; provider overflow remains the
    // hard limit. The attempt's credential covers the summary request.
    // Before paying for a summary, shorten the tool output that made the
    // request large. Trimming makes no model call and loses no history (the
    // original stays on disk). If the request still does not fit afterwards,
    // the next pass summarizes completed older attempts, including those in
    // the open turn, while retaining anchors and the current batch.
    let compact_due = preflight_compaction_due(ledger, &prepared, model, &epoch);
    if budget.trims_left > 0 && compact_due {
        let trims = engine::plan_tool_result_trim(
            &ledger
                .projection()
                .ok_or("ledger projection missing")?
                .events,
        )?;
        if !trims.is_empty() {
            drop(credential_material);
            for trim in &trims {
                append_tool_result_trim(ledger, options, turn, trim)?;
            }
            append_provider_epoch(
                ledger,
                EpochAppend {
                    options,
                    dialect,
                    model: &model.id,
                    profile: &profile_value,
                    tools: &tool_catalog,
                    reason: "tool_result_trim",
                    pending: &consumed,
                },
            )?;
            announce_appended(ledger, stdout)?;
            return run_provider_turn_inner(
                ledger,
                run,
                credential,
                lines,
                stdout,
                ProviderLoopBudget {
                    trims_left: budget.trims_left - 1,
                    ..budget
                },
                cancellation,
            );
        }
    }
    if budget.compactions_left > 0
        && compact_due
        && !engine::plan_context_compaction(
            &ledger
                .projection()
                .ok_or("ledger projection missing")?
                .events,
            turn,
        )?
        .covers
        .is_empty()
    {
        let summary = run_compaction_summary(
            ledger,
            profile,
            provider_config,
            model,
            &resolved_profile,
            credential_material
                .as_ref()
                .map_or("", |material| material.material.as_str()),
            transport_endpoint,
            &cancelled,
            turn,
            &attempt,
        )?;
        drop(credential_material);
        if auto_compact_for_turn(ledger, options, turn, summary)? {
            append_provider_epoch(
                ledger,
                EpochAppend {
                    options,
                    dialect,
                    model: &model.id,
                    profile: &profile_value,
                    tools: &tool_catalog,
                    reason: "compaction",
                    pending: &consumed,
                },
            )?;
            announce_appended(ledger, stdout)?;
        }
        return run_provider_turn_inner(
            ledger,
            run,
            credential,
            lines,
            stdout,
            ProviderLoopBudget {
                compactions_left: budget.compactions_left - 1,
                ..budget
            },
            cancellation,
        );
    }
    stdout.write_all(&encode_line(
        "lease_request",
        &LeaseRequest {
            attempt: attempt.clone(),
            class: "provider".to_owned(),
        },
    )?)?;
    stdout.flush()?;
    let granted = loop {
        let line = lines
            .next()
            .ok_or_else(|| {
                ProtocolFailure("supervisor EOF while awaiting provider lease".to_owned())
            })?
            .map_err(|error| {
                ProtocolFailure(format!(
                    "supervisor read failed while awaiting lease: {error}"
                ))
            })?;
        match decode_supervisor(line.as_bytes())? {
            SupervisorMessage::Lease(lease) if lease.attempt == attempt => break lease.granted,
            SupervisorMessage::Ping(ping) => {
                stdout.write_all(&encode_line("pong", &worker_control::Pong { id: ping.id })?)?;
                stdout.flush()?;
            }
            SupervisorMessage::Stop(_) => {
                cancellation.cancel(CANCEL_STOP);
                cancellation.defer(line);
                break false;
            }
            SupervisorMessage::QueueTransaction(_) => cancellation.park_delivery(line),
            _ => {
                return Err(Box::new(ProtocolFailure(
                    "unexpected supervisor message while awaiting provider lease".to_owned(),
                )));
            }
        }
    };
    if !granted {
        // Denial is the supervisor draining (or a stop that arrived while waiting).
        // The turn stays open without an attempt; the next sweep classifies it as
        // recovery_needed and a fresh run continues or reconciles it.
        eprintln!(
            "tekes-worker: provider lease denied for turn {turn}; leaving the tail for recovery"
        );
        drop(credential_material);
        return Ok(());
    }

    let attempt_event = make_event(json!({
        "v": 1,
        "seq": ledger.next_seq(),
        "turn": turn,
        "kind": "attempt",
        "ts": options.event_timestamp(),
        "attempt": attempt,
        "epoch": epoch,
        "wire_digest": prepared.request_digest,
        "request": {"asset": request_asset.asset, "bytes": request_asset.bytes},
        "admits": ranges(&context.admits)
    }))?;
    ledger.append_contract(attempt_event, BarrierContext::default())?;
    if prepared.dispatch_marker_required {
        let dispatched = make_event(json!({
            "v": 1,
            "seq": ledger.next_seq(),
            "turn": turn,
            "kind": "attempt_dispatched",
            "ts": options.event_timestamp(),
            "attempt": attempt
        }))?;
        ledger.append_contract(dispatched, BarrierContext::default())?;
    }

    let mut eager = EagerDispatch::default();
    let completion = if cancelled.load(std::sync::atomic::Ordering::Acquire) {
        ProviderCompletion::Failure(ProviderFailure::Cancelled)
    } else {
        // The request runs on its own thread so this thread keeps the ledger and
        // stdout: frames are forwarded as they arrive, and deliveries the reader
        // thread parked are appended and receipted while the provider is still
        // streaming (R2-8; tail-lifecycle audit gap J). A failure on this side
        // cancels the request and is reported after the thread has joined.
        let credential_material = credential_material
            .as_ref()
            .map_or("", |material| material.material.as_str());
        let wall = budget
            .wall_deadline
            .map(|deadline| deadline.saturating_duration_since(Instant::now()));
        let (frame_sender, frame_receiver) = std::sync::mpsc::channel::<ProviderFrame>();
        let prepared = &prepared;
        let capture_path = std::env::var("TEKES_KERNEL_LIVE_ARTIFACT")
            .ok()
            .map(|directory| {
                PathBuf::from(directory).join(format!("{attempt}.response.partial.raw"))
            });
        let cancelled_flag = &cancelled;
        std::thread::scope(
            |scope| -> Result<ProviderCompletion, Box<dyn std::error::Error>> {
                let request = scope.spawn(move || {
                    let runtime = HttpRuntime::new()?;
                    let capture = Arc::new(Mutex::new(Vec::new()));
                    let runtime = if capture_path.is_some() {
                        runtime.with_response_capture(Arc::clone(&capture))
                    } else {
                        runtime
                    };
                    let result = runtime.send_dialect_with_frames_and_wall_transport(
                        dialect,
                        prepared,
                        transport_endpoint,
                        credential_material,
                        cancelled_flag,
                        wall,
                        move |frame| {
                            frame_sender.send(frame).map_err(|_| {
                                provider::HttpRuntimeError::Request(
                                    "provider frame receiver closed".to_owned(),
                                )
                            })
                        },
                    );
                    if let Some(path) = capture_path {
                        let body = capture.lock().expect("response capture lock");
                        fs::write(path, &*body).map_err(|error| {
                            provider::HttpRuntimeError::Request(error.to_string())
                        })?;
                    }
                    result
                });
                let mut block = 0_u64;
                let mut main_failure: Option<Box<dyn std::error::Error>> = None;
                loop {
                    let frame = match frame_receiver.recv_timeout(Duration::from_millis(20)) {
                        Ok(frame) => Some(frame),
                        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => None,
                        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                    };
                    if main_failure.is_some() {
                        continue;
                    }
                    let step = (|| -> Result<(), Box<dyn std::error::Error>> {
                        if let Some(mut frame) = frame {
                            if let ProviderFrame::ToolCallReady(call) = &mut frame {
                                repair_provider_call_arguments(dialect, call);
                            }
                            localize_provider_frame(ledger, &attempt, &mut eager, &mut frame)?;
                            let ledger_seq =
                                ledger.projection().map(|projection| projection.last_seq);
                            if let Some(call) = forward_provider_frame(
                                ledger_seq, &attempt, &mut block, frame, stdout,
                            )? {
                                // Eager dispatch: the call's arguments are complete
                                // and validated, so it executes now, while the
                                // response keeps streaming on its own thread. The
                                // terminal later has to name it unchanged.
                                eager_dispatch_ready_call(
                                    ledger,
                                    EagerReadyCall {
                                        options,
                                        profile,
                                        selected: *selected,
                                        manifest: &manifest,
                                        attempt: &attempt,
                                        turn,
                                        cancellation,
                                    },
                                    call,
                                    &mut eager,
                                    credential.as_ref(),
                                    lines,
                                    stdout,
                                )?;
                            }
                        }
                        drain_parked_deliveries(ledger, options, run.selected, cancellation, stdout)
                    })();
                    if let Err(error) = step {
                        cancelled_flag.store(true, std::sync::atomic::Ordering::Release);
                        main_failure = Some(error);
                    }
                }
                let completion = request
                    .join()
                    .map_err(|_| "provider request thread panicked")??;
                if let Some(failure) = main_failure {
                    return Err(failure);
                }
                Ok(completion)
            },
        )?
    };

    if let (Ok(directory), ProviderCompletion::Terminal(terminal)) =
        (std::env::var("TEKES_KERNEL_LIVE_ARTIFACT"), &completion)
    {
        fs::write(
            PathBuf::from(directory).join(format!("{attempt}.response.raw")),
            &terminal.raw_response,
        )?;
    }
    if let ProviderCompletion::Terminal(terminal) = &completion {
        if let Some(changes) = &terminal.input_transformations {
            record_input_transformations(
                ledger,
                &options.event_timestamp(),
                turn,
                &attempt,
                changes,
            )?;
        }
    }
    let runtime_cancelled_provider = matches!(
        &completion,
        ProviderCompletion::Failure(ProviderFailure::Cancelled)
    );
    let outcome = match completion {
        ProviderCompletion::Terminal(terminal)
            if matches!(
                terminal.finish_reason,
                FinishReason::ContentFilter | FinishReason::ProviderError
            ) =>
        {
            append_provider_terminal_error(ledger, options, turn, &attempt, &terminal)?
        }
        ProviderCompletion::Terminal(terminal)
            if terminal.finish_reason == FinishReason::ContextOverflow =>
        {
            append_context_overflow(ledger, options, turn, &attempt, terminal.usage.as_ref())?
        }
        ProviderCompletion::Terminal(terminal) => append_terminal(
            ledger,
            TerminalAppend {
                options,
                manifest: &manifest,
                eager: &eager,
                dialect,
                server_managed: dialect.server_managed(),
                turn,
                attempt: &attempt,
            },
            terminal,
        )?,
        ProviderCompletion::Failure(failure) => append_provider_failure(
            ledger,
            options,
            turn,
            &attempt,
            failure,
            cancellation,
            dialect,
            &eager,
        )?,
    };
    stdout.write_all(&encode_line(
        "attempt_settled",
        &AttemptSettled {
            attempt: attempt.clone(),
            outcome_seq: outcome.seq,
        },
    )?)?;
    stdout.flush()?;
    // The summary request reuses this attempt's lease before it is released;
    // the compact it feeds is written below on the same plan.
    let compaction_summary = if outcome.compact_retry
        && budget.compactions_left > 0
        && !cancelled.load(std::sync::atomic::Ordering::Acquire)
    {
        run_compaction_summary(
            ledger,
            profile,
            provider_config,
            model,
            &resolved_profile,
            credential_material
                .as_ref()
                .map_or("", |material| material.material.as_str()),
            transport_endpoint,
            &cancelled,
            turn,
            &attempt,
        )?
    } else {
        None
    };
    drop(credential_material);
    if runtime_cancelled_provider && cancellation.supervisor_lost() {
        return Err(Box::new(ProtocolFailure(
            "supervisor EOF during provider request".to_owned(),
        )));
    }
    if runtime_cancelled_provider && cancellation.stop_requested() {
        // The attempt is terminal (usage + error are durable). The settle belongs to
        // `post_turn_exit`, which first echoes the durable `stop_requested` for the
        // stop still sitting in the control channel and then settles `user_stop`
        // (tail-lifecycle audit I11: a stop is durable before it takes semantic effect).
        return Ok(());
    }
    if outcome.compact_retry {
        if budget.compactions_left > 0
            && auto_compact_for_turn(ledger, options, turn, compaction_summary)?
        {
            append_provider_epoch(
                ledger,
                EpochAppend {
                    options,
                    dialect,
                    model: &model.id,
                    profile: &profile_value,
                    tools: &tool_catalog,
                    reason: "compaction",
                    pending: &consumed,
                },
            )?;
            return run_provider_turn_inner(
                ledger,
                run,
                credential,
                lines,
                stdout,
                ProviderLoopBudget {
                    compactions_left: budget.compactions_left - 1,
                    ..budget
                },
                cancellation,
            );
        }
        append_settle(
            ledger,
            &options.event_timestamp(),
            turn,
            "interrupted",
            Some("budget_tokens"),
        )?;
        return Ok(());
    }
    if outcome.retry {
        if budget.retries_left == 0 {
            append_settle(
                ledger,
                &options.event_timestamp(),
                turn,
                "error",
                Some(outcome.retry_classification.unwrap_or("internal")),
            )?;
            return Ok(());
        }
        if let Some(classification @ ("rate_limit" | "transport")) = outcome.retry_classification {
            wait_provider_admission(
                ledger,
                options,
                turn,
                &attempt,
                ProviderAdmissionRetry {
                    classification,
                    retry_after_seconds: outcome.retry_after_seconds,
                    retries_left: budget.retries_left,
                },
                cancellation,
            )?;
            if cancellation.stop_requested() || cancellation.supervisor_lost() {
                return Ok(());
            }
        }
        if dialect.server_managed() {
            append_provider_epoch(
                ledger,
                EpochAppend {
                    options,
                    dialect,
                    model: &model.id,
                    profile: &profile_value,
                    tools: &tool_catalog,
                    reason: "recovery",
                    pending: &consumed,
                },
            )?;
        }
        return run_provider_turn_inner(
            ledger,
            run,
            credential,
            lines,
            stdout,
            ProviderLoopBudget {
                retries_left: budget.retries_left - 1,
                ..budget
            },
            cancellation,
        );
    }
    if !outcome.tool_calls.is_empty() {
        if eager.suspended {
            // Approval can arrive inline before the provider terminal. The
            // earlier park flag is not current lifecycle state: a durable
            // answer and no execution start means this worker can finish the
            // head and its siblings now, without exiting for crash recovery.
            let answered_inline = pending_tool_calls(ledger)?.iter().any(|call| {
                eager.contains(&call.call)
                    && call.execution_tracked
                    && call.approval_response.is_some()
                    && !call.execution_started
            });
            if !answered_inline
                || cancellation.stop_requested()
                || cancellation.supervisor_lost()
                || ledger
                    .projection()
                    .ok_or("ledger projection missing")?
                    .lifecycle
                    .open_hold
            {
                return Ok(());
            }
        }
        // Fan-in: eagerly dispatched calls already own their single result.
        // Only the remainder of the batch executes here, and the continuation
        // below waits for the whole batch either way.
        let paired = ledger
            .projection()
            .ok_or("ledger projection missing")?
            .events
            .iter()
            .filter(|event| *event.kind() == EventKind::ToolResult)
            .filter_map(|event| event.string_field("call").map(str::to_owned))
            .collect::<BTreeSet<_>>();
        let remaining = outcome
            .tool_calls
            .iter()
            .filter(|call| !paired.contains(&call.call_id))
            .cloned()
            .collect::<Vec<_>>();
        if execute_provider_tool_calls(
            ledger,
            ToolCallBatch {
                options,
                profile,
                selected: *selected,
                attempt: &attempt,
                turn,
                calls: &remaining,
                cancellation,
            },
            credential.as_ref(),
            lines,
            stdout,
        )? {
            return Ok(());
        }
        if cancellation.stop_requested() {
            return Ok(());
        }
        if cancellation.supervisor_lost() || cancellation.protocol_failed() {
            return Err(Box::new(ProtocolFailure(
                "supervisor lost during tool execution".to_owned(),
            )));
        }
        drain_parked_deliveries(ledger, options, run.selected, cancellation, stdout)?;
        return run_provider_turn_inner(
            ledger,
            run,
            credential,
            lines,
            stdout,
            budget,
            cancellation,
        );
    }
    if let Some((settle_outcome, reason)) = outcome.settle {
        if settle_outcome == "completed" {
            if validation_runtime::advance(ledger, run, lines, stdout, cancellation)? {
                return Ok(());
            }
            // Failed validation and an unmet validator mandate resume the same
            // session, without treating the provider's final as turn settlement.
            return run_provider_turn_inner(
                ledger,
                run,
                credential,
                lines,
                stdout,
                ProviderLoopBudget {
                    nonfinal_responses_left: budget.nonfinal_responses_left.saturating_sub(1),
                    ..budget
                },
                cancellation,
            );
        }
        append_settle(
            ledger,
            &options.event_timestamp(),
            turn,
            settle_outcome,
            reason,
        )?;
        return Ok(());
    }
    if cancellation.stop_requested() {
        return Ok(());
    }
    if budget.nonfinal_responses_left == 0 {
        settle_provider_unavailable(
            ledger,
            options,
            "provider produced no final answer within the continuation budget",
        )?;
        return Ok(());
    }
    drain_parked_deliveries(ledger, options, run.selected, cancellation, stdout)?;
    run_provider_turn_inner(
        ledger,
        run,
        credential,
        lines,
        stdout,
        ProviderLoopBudget {
            nonfinal_responses_left: budget.nonfinal_responses_left - 1,
            ..budget
        },
        cancellation,
    )
}

/// Mirrors one accepted provider frame to the supervisor. Runs on the worker's
/// main thread, the only stdout writer, while the request itself streams on its
/// own thread. Returns the validated call when the frame was the provider's
/// per-call argument completion, so the caller can dispatch it before the
/// response terminal arrives; the presentation marker itself has already left
/// the pipe by then, ahead of the durable `tool_call`.
fn forward_provider_frame(
    ledger_seq: Option<u64>,
    attempt: &str,
    block: &mut u64,
    frame: ProviderFrame,
    stdout: &mut impl Write,
) -> Result<Option<provider::ToolCall>, Box<dyn std::error::Error>> {
    let mut arguments_complete = None;
    let mut ready = None;
    let (channel, delta, call_id, name) = match frame {
        ProviderFrame::TextDelta(delta) => (FrameChannel::Text, delta, None, None),
        ProviderFrame::ReasoningDelta(delta) => (FrameChannel::Reasoning, delta, None, None),
        ProviderFrame::ToolDelta {
            call_id,
            name,
            delta,
        } => (FrameChannel::Tool, delta, Some(call_id), name),
        ProviderFrame::ToolCallReady(call) => {
            arguments_complete = Some(true);
            let (call_id, name) = (call.call_id.clone(), call.name.clone());
            ready = Some(call);
            (FrameChannel::Tool, String::new(), Some(call_id), Some(name))
        }
        // Live reports are presentation-only; the terminal remains the accounting authority.
        ProviderFrame::UsageDelta(usage) => (
            FrameChannel::Usage,
            serde_json::to_string(&usage_object(Some(&usage)))?,
            None,
            None,
        ),
        ProviderFrame::Status(_) => return Ok(None),
    };
    stdout.write_all(&encode_line(
        "frame",
        &Frame {
            arguments_complete,
            ledger_seq,
            attempt: attempt.to_owned(),
            channel,
            block: *block,
            delta,
            call_id,
            name,
        },
    )?)?;
    stdout.flush()?;
    *block = block.saturating_add(1);
    Ok(ready)
}

/// Calls this attempt dispatched before its response terminal (eager
/// dispatch). Each entry is already a durable `tool_call`; execution ran, or
/// was suspended, through the same pipeline the post-terminal batch uses.
#[derive(Default)]
struct EagerDispatch {
    wire_ids: std::collections::BTreeMap<String, String>,
    calls: Vec<provider::ToolCall>,
    /// A dispatched call parked the turn (hold, spawn) or a stop arrived. No
    /// further call executes in this run; the terminal still lands durably so
    /// the resumed run finds the whole batch exactly as a late batch would.
    suspended: bool,
}

impl EagerDispatch {
    fn contains(&self, call_id: &str) -> bool {
        self.calls.iter().any(|call| call.call_id == call_id)
    }
}

/// Provider IDs identify calls within a response, not across a durable ledger.
/// A new response may reuse a completed call's wire ID, even in visible history.
/// Keep native carriers untouched and use a separate durable identity locally.
fn local_provider_call_id(
    ledger: &LockedLedger,
    attempt: &str,
    wire_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let events = &ledger
        .projection()
        .ok_or("ledger projection missing")?
        .events;
    let previous = events
        .iter()
        .filter(|event| {
            *event.kind() == EventKind::ToolCall
                && event
                    .string_field("provider_call")
                    .or_else(|| event.string_field("call"))
                    == Some(wire_id)
        })
        .collect::<Vec<_>>();
    if let Some(call) = previous
        .iter()
        .find(|event| event.string_field("attempt") == Some(attempt))
    {
        return Ok(call
            .string_field("call")
            .ok_or("tool_call lacks call")?
            .to_owned());
    }
    if previous.is_empty() {
        return Ok(wire_id.to_owned());
    }
    for call in previous {
        let id = call.string_field("call").ok_or("tool_call lacks call")?;
        let owner = call.string_field("attempt");
        let related = events
            .iter()
            .filter(|event| {
                event.seq() == call.seq()
                    || event.string_field("call") == Some(id)
                        && matches!(event.kind(), EventKind::ToolResult | EventKind::ChildResult)
                    || *event.kind() == EventKind::Output && event.string_field("attempt") == owner
            })
            .collect::<Vec<_>>();
        if !related
            .iter()
            .any(|event| *event.kind() == EventKind::ToolResult)
            || !related
                .iter()
                .any(|event| *event.kind() == EventKind::Output)
        {
            return Err(format!(
                "provider tool call id {wire_id} reuses an unresolved durable call"
            )
            .into());
        }
    }
    let digest = Sha256::digest(serde_json_canonicalizer::to_vec(&json!([
        attempt, wire_id
    ]))?);
    let local_id = format!("provider-call-{digest:x}");
    if events.iter().any(|event| {
        *event.kind() == EventKind::ToolCall && event.string_field("call") == Some(&local_id)
    }) {
        return Err("scoped provider call identity collision".into());
    }
    Ok(local_id)
}

fn repair_provider_call_arguments(dialect: DialectId, call: &mut provider::ToolCall) {
    if !matches!(
        dialect,
        DialectId::GlmChatV1 | DialectId::DeepseekResponsesV1
    ) {
        return;
    }
    if let Some(arguments) = tools::fixed_schema(&call.name)
        .and_then(|schema| schema.repair_quoted_null_exclusive(&call.arguments))
    {
        // Only the normalized invocation changes. The terminal's sealed native
        // carrier retains the provider's original argument bytes for replay.
        call.arguments = arguments;
    }
}

fn localize_provider_frame(
    ledger: &LockedLedger,
    attempt: &str,
    eager: &mut EagerDispatch,
    frame: &mut ProviderFrame,
) -> Result<(), Box<dyn std::error::Error>> {
    let id = match frame {
        ProviderFrame::ToolDelta { call_id, .. } => call_id,
        ProviderFrame::ToolCallReady(call) => &mut call.call_id,
        _ => return Ok(()),
    };
    let wire_id = id.clone();
    if let Some((local_id, _)) = eager.wire_ids.iter().find(|(_, wire)| *wire == &wire_id) {
        *id = local_id.clone();
    } else {
        *id = local_provider_call_id(ledger, attempt, &wire_id)?;
        eager.wire_ids.insert(id.clone(), wire_id);
    }
    Ok(())
}

struct EagerReadyCall<'a> {
    options: &'a Options,
    profile: &'a RuntimeProfile,
    selected: Selected,
    manifest: &'a BuiltinManifest,
    attempt: &'a str,
    turn: u64,
    cancellation: &'a RuntimeCancellation,
}

/// Dispatches one provider call as soon as its arguments are complete, while the
/// response is still streaming. The durable `tool_call` is the write-ahead
/// intent exactly as for a post-terminal batch; the response terminal must later
/// name this call with the same identity, name and arguments, and the
/// post-terminal batch skips it. A parked or stopped dispatch suspends eager
/// execution for the rest of the response without cancelling the request.
fn eager_dispatch_ready_call<I, O>(
    ledger: &mut LockedLedger,
    ready: EagerReadyCall<'_>,
    call: provider::ToolCall,
    eager: &mut EagerDispatch,
    credential: Option<&Arc<Mutex<CredentialClient>>>,
    lines: &mut I,
    stdout: &mut O,
) -> Result<(), Box<dyn std::error::Error>>
where
    I: Iterator<Item = io::Result<String>>,
    O: Write,
{
    if eager.suspended {
        return Ok(());
    }
    let EagerReadyCall {
        options,
        profile,
        selected,
        manifest,
        attempt,
        turn,
        cancellation,
    } = ready;
    if call.call_id.is_empty() || eager.contains(&call.call_id) {
        return Err(format!(
            "provider tool call id {} is empty or duplicated",
            call.call_id
        )
        .into());
    }
    let reused = ledger
        .projection()
        .ok_or("ledger projection missing")?
        .events
        .iter()
        .any(|event| {
            *event.kind() == EventKind::ToolCall
                && event.string_field("call") == Some(call.call_id.as_str())
        });
    if reused {
        return Err(format!(
            "provider tool call id {} reuses a durable call",
            call.call_id
        )
        .into());
    }
    append_provider_tool_call_with_wire_id(
        ledger,
        options,
        manifest,
        turn,
        attempt,
        &call,
        eager.wire_ids.get(&call.call_id).map(String::as_str),
    )?;
    announce_appended(ledger, stdout)?;
    let parked = execute_provider_tool_calls(
        ledger,
        ToolCallBatch {
            options,
            profile,
            selected,
            attempt,
            turn,
            calls: std::slice::from_ref(&call),
            cancellation,
        },
        credential,
        lines,
        stdout,
    )?;
    eager.calls.push(call);
    if parked {
        eager.suspended = true;
    }
    announce_appended(ledger, stdout)?;
    Ok(())
}

#[cfg(test)]
fn append_provider_tool_call(
    ledger: &mut LockedLedger,
    options: &Options,
    manifest: &BuiltinManifest,
    turn: u64,
    attempt: &str,
    call: &provider::ToolCall,
) -> Result<(), Box<dyn std::error::Error>> {
    append_provider_tool_call_with_wire_id(ledger, options, manifest, turn, attempt, call, None)
}

fn append_provider_tool_call_with_wire_id(
    ledger: &mut LockedLedger,
    options: &Options,
    manifest: &BuiltinManifest,
    turn: u64,
    attempt: &str,
    call: &provider::ToolCall,
    wire_id: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let effect = manifest
        .tools
        .iter()
        .find(|tool| tool.name == call.name)
        .map(|tool| engine::side_effectful(tool.effect))
        .unwrap_or(true);
    let arguments = spill_json(ledger, &call.arguments)?;
    let mut raw = json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"tool_call",
        "ts":options.event_timestamp(),"attempt":attempt,"call":call.call_id,
        "name":call.name,"args":arguments,"source":"provider","execution_tracked":true
    });
    if let Some(wire_id) = wire_id.filter(|id| *id != call.call_id) {
        raw["provider_call"] = json!(wire_id);
    }
    let event = make_event(raw)?;
    ledger.append_contract(
        event,
        BarrierContext {
            side_effectful_tool_call: effect,
        },
    )?;
    Ok(())
}

fn settle_provider_unavailable(
    ledger: &mut LockedLedger,
    options: &Options,
    detail: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let turn = ledger
        .projection()
        .and_then(|projection| projection.latest_turn)
        .ok_or("provider failure requires turn_open")?;
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"error",
        "ts":options.event_timestamp(),"recoverable":false,
        "classification":"provider_terminal","detail":bounded_ledger_detail(detail)
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    append_settle(
        ledger,
        &options.event_timestamp(),
        turn,
        "error",
        Some("provider_terminal"),
    )?;
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SelectedProviderFailure {
    ProviderNotSelected,
    ProviderNotConfigured(String),
    ModelNotSelected,
    ModelNotConfigured(String),
    ModelDisabled(String),
}

impl SelectedProviderFailure {
    fn detail(&self) -> String {
        match self {
            Self::ProviderNotSelected => "provider_not_selected".to_owned(),
            Self::ProviderNotConfigured(id) => format!("provider_not_configured:{id}"),
            Self::ModelNotSelected => "model_not_selected".to_owned(),
            Self::ModelNotConfigured(id) => format!("model_not_configured:{id}"),
            Self::ModelDisabled(id) => format!("model_disabled:{id}"),
        }
    }
}

fn selected_provider(
    config: &ConfigSnapshot,
) -> Result<(&Provider, &Model), SelectedProviderFailure> {
    let provider_id = config
        .session_settings
        .as_ref()
        .map(|settings| settings.provider.as_str())
        .or(config.workspace.policy.provider.as_deref())
        .or(config.settings.default_provider.as_deref())
        .ok_or(SelectedProviderFailure::ProviderNotSelected)?;
    let model_id = config
        .session_settings
        .as_ref()
        .map(|settings| settings.model.as_str())
        .or(config.workspace.policy.model.as_deref())
        .or(config.settings.default_model.as_deref())
        .ok_or(SelectedProviderFailure::ModelNotSelected)?;
    let provider = config
        .providers
        .providers
        .iter()
        .find(|provider| provider.id == provider_id)
        .ok_or_else(|| SelectedProviderFailure::ProviderNotConfigured(provider_id.to_owned()))?;
    let model = provider
        .models
        .iter()
        .find(|model| model.id == model_id)
        .ok_or_else(|| SelectedProviderFailure::ModelNotConfigured(model_id.to_owned()))?;
    if !model.enabled {
        return Err(SelectedProviderFailure::ModelDisabled(model_id.to_owned()));
    }
    Ok((provider, model))
}

fn current_turn_inputs(
    ledger: &LockedLedger,
    turn: u64,
) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let open = projection
        .events
        .iter()
        .rev()
        .find(|event| *event.kind() == EventKind::TurnOpen && event.turn() == Some(turn))
        .ok_or("current turn has no turn_open")?;
    let raw = serde_json::to_value(open.raw())?;
    Ok(raw
        .as_object()
        .expect("event object")
        .get("trigger")
        .and_then(Value::as_object)
        .and_then(|trigger| trigger.get("inputs"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_u64)
        .collect())
}

#[cfg(test)]
mod provider_context_tests {
    include!("validation_tests.rs");
    include!("live_tests.rs");
    use super::*;
    use std::fs;
    use std::net::TcpListener;

    use profile::{
        EffectiveInstructions, InstructionResolver, ProvidersConfig, ResolvedWorkspace,
        RevisionVector, SettingsConfig, WorkspacePolicy,
    };
    use test_support::FixtureRoot;

    fn queue_origin(key: &str) -> schema::OriginTuple {
        schema::OriginTuple {
            principal: "uid:501".to_owned(),
            client: "session-endpoint".to_owned(),
            target: "018f0000-0000-7000-8000-000000000003".to_owned(),
            op: "session.updateQueue".to_owned(),
            key: key.to_owned(),
        }
    }

    fn queue_ledger() -> (tempfile::TempDir, LockedLedger) {
        let directory = tempfile::tempdir().expect("queue tempdir");
        let path = directory.path().join("main.jsonl");
        let genesis = json!({
            "v":1,"seq":1,"kind":"genesis","ts":"2026-08-28T09:00:00.000Z",
            "format":1,"min_reader":1,"min_writer":1,
            "thread":"018f0000-0000-7000-8000-000000000003","workspace":"ws",
            "origin_key":"create-1","origin_tuple":{
                "principal":"uid:501","client":"session-endpoint",
                "target":"018f0000-0000-7000-8000-000000000003",
                "op":"session.create","key":"create-1"
            },"resume":"never","config":{"digest":"cfg"}
        });
        let mut bytes = serde_json_canonicalizer::to_vec(&genesis).expect("genesis bytes");
        bytes.push(b'\n');
        fs::write(&path, bytes).expect("genesis ledger");
        let mut ledger = LockedLedger::open(&path, 1).expect("queue ledger");
        let input = make_event(json!({
            "v":1,"seq":2,"kind":"input","ts":"2026-08-28T09:00:01.000Z",
            "origin_key":"input-1","origin_tuple":{
                "principal":"uid:501","client":"session-endpoint",
                "target":"018f0000-0000-7000-8000-000000000003",
                "op":"session.prompt","key":"input-1"
            },"content":[{"type":"text","text":"old"}],"steer":false
        }))
        .expect("input");
        ledger
            .append_contract(input, BarrierContext::default())
            .expect("input append");
        (directory, ledger)
    }

    #[test]
    fn file_block_renders_wire_file_block_with_base64_data_and_bytes() {
        let (_directory, ledger) = queue_ledger();
        let folder = ledger.path().parent().unwrap();
        let content = b"hello, attachment\n";
        let asset = store::AssetStore::new(folder.join("assets"))
            .unwrap()
            .publish(content)
            .unwrap()
            .asset;
        let blocks = render_blocks(
            &ledger,
            &json!([{"type":"file","asset":asset,"mime":"text/plain","name":"notes.txt","bytes":content.len()}]),
        )
        .expect("render file block");
        assert_eq!(
            blocks,
            vec![json!({
                "type":"file","name":"notes.txt","mime":"text/plain","bytes":content.len(),
                "data":base64::engine::general_purpose::STANDARD.encode(content)
            })]
        );
    }

    fn edit_transaction() -> QueueTransaction {
        QueueTransaction {
            delivery: "delivery-queue-1".to_owned(),
            rpc_id: "rpc-queue-1".to_owned(),
            target_seq: 2,
            retract_origin: queue_origin("rpc-queue-1/retract"),
            action: QueueTransactionAction::Edit {
                replacement_origin: queue_origin("rpc-queue-1/replacement"),
                content: vec![schema::Block::Text {
                    text: "replacement".to_owned(),
                }],
                steer: false,
                assets: None,
            },
        }
    }

    #[test]
    fn queue_transaction_commits_pair_and_reacks_without_new_events() {
        let (_directory, mut ledger) = queue_ledger();
        let transaction = edit_transaction();
        let result =
            execute_queue_transaction(&mut ledger, "2026-08-28T09:00:02.000Z", &transaction)
                .expect("queue transaction");
        assert_eq!(
            result.outcome,
            QueueTransactionOutcome::Committed {
                first_seq: 3,
                last_seq: 4,
                deduplicated: false
            }
        );
        let before = ledger.next_seq();
        let retry =
            execute_queue_transaction(&mut ledger, "2026-08-28T09:00:03.000Z", &transaction)
                .expect("queue transaction retry");
        assert_eq!(
            retry.outcome,
            QueueTransactionOutcome::Committed {
                first_seq: 3,
                last_seq: 4,
                deduplicated: true
            }
        );
        assert_eq!(ledger.next_seq(), before, "retry must not append");
    }

    #[test]
    fn queue_transaction_completes_retraction_only_crash_window() {
        let (_directory, mut ledger) = queue_ledger();
        let transaction = edit_transaction();
        let mut object = origin_event(
            &ledger,
            "2026-08-28T09:00:02.000Z",
            "queue_edit",
            &transaction.retract_origin,
        );
        object.insert("supersedes".to_owned(), json!([{"from":2,"to":2}]));
        ledger
            .append_contract(
                make_event(Value::Object(object)).expect("retraction"),
                BarrierContext::default(),
            )
            .expect("retraction barrier");
        let result =
            execute_queue_transaction(&mut ledger, "2026-08-28T09:00:03.000Z", &transaction)
                .expect("recovery completion");
        assert_eq!(
            result.outcome,
            QueueTransactionOutcome::Committed {
                first_seq: 3,
                last_seq: 4,
                deduplicated: true
            }
        );
    }

    #[test]
    fn consumed_queue_target_rejects_without_authoring_events() {
        let (_directory, mut ledger) = queue_ledger();
        append_turn_open(&mut ledger, "2026-08-28T09:00:02.000Z", "inputs", &[2], 1)
            .expect("consume input");
        let before = ledger.next_seq();
        let result =
            execute_queue_transaction(&mut ledger, "2026-08-28T09:00:03.000Z", &edit_transaction())
                .expect("rejection");
        assert_eq!(
            result.outcome,
            QueueTransactionOutcome::Rejected {
                code: QueueTransactionRejectCode::QueueItemNotFound,
                reason: None
            }
        );
        assert_eq!(ledger.next_seq(), before, "rejection authors zero events");
    }

    #[test]
    fn invalid_queue_attachment_rejects_before_retraction() {
        let (_directory, mut ledger) = queue_ledger();
        let mut transaction = edit_transaction();
        let QueueTransactionAction::Edit { assets, .. } = &mut transaction.action else {
            unreachable!()
        };
        *assets = Some(vec![worker_control::AssetRef {
            asset: "sha256-0000000000000000000000000000000000000000000000000000000000000000"
                .to_owned(),
            mime: "image/png".to_owned(),
        }]);
        let before = ledger.next_seq();
        let result =
            execute_queue_transaction(&mut ledger, "2026-08-28T09:00:03.000Z", &transaction)
                .expect("attachment rejection");
        assert_eq!(
            result.outcome,
            QueueTransactionOutcome::Rejected {
                code: QueueTransactionRejectCode::AttachmentError,
                reason: Some("CORRUPT".to_owned())
            }
        );
        assert_eq!(ledger.next_seq(), before, "rejection authors zero events");
    }

    #[test]
    fn frozen_instruction_bytes_supply_hooks_and_skill_catalog() {
        let snapshot = InstructionSnapshot::decode(include_bytes!(
            "../../../fixtures/instructions/snapshot.canonical.json"
        ))
        .expect("instruction fixture");
        let hooks = frozen_hook_bindings(&snapshot).expect("effective hooks");
        assert_eq!(
            hooks
                .iter()
                .map(|binding| binding.id.as_str())
                .collect::<Vec<_>>(),
            vec!["pre.txt"]
        );
        let skills = frozen_skill_catalog(&snapshot).expect("effective skills");
        assert_eq!(
            skills
                .iter()
                .map(|entry| entry.name.as_str())
                .collect::<Vec<_>>(),
            vec!["final.md", "shared.md"]
        );
        assert_eq!(skills[0].schema_digest, snapshot.sources[9].content_sha256);
        assert_eq!(skills[0].content, IJsonValue::from("Project one skill.\n"));
        assert_eq!(skills[1].schema_digest, snapshot.sources[7].content_sha256);
        assert_eq!(
            skills[1].content,
            IJsonValue::from("Project zero shared skill.\n")
        );
    }

    #[test]
    fn package_skill_catalog_uses_same_frozen_resource_projection() {
        let fixtures = FixtureRoot::discover().expect("fixtures");
        let root = fixtures.join("resources/migration");
        let snapshot = InstructionResolver::new(root.join("user"), [root.join("project")])
            .capture()
            .expect("resource instruction snapshot");
        let skills = frozen_skill_catalog(&snapshot).expect("package skill catalog");
        assert_eq!(
            skills
                .iter()
                .map(|entry| entry.name.as_str())
                .collect::<Vec<_>>(),
            ["notes", "review", "user-review"]
        );
        let review = skills
            .iter()
            .find(|entry| entry.name == "review")
            .expect("review skill");
        let content: Value = serde_json::from_slice(
            &review
                .content
                .canonical_bytes()
                .expect("canonical package bytes"),
        )
        .expect("package JSON");
        assert_eq!(content["format"], 1);
        assert_eq!(content["description"], "Review from the project package");
        assert_eq!(
            content["resources"]
                .as_array()
                .expect("resources")
                .iter()
                .map(|resource| resource["path"].as_str().expect("resource path"))
                .collect::<Vec<_>>(),
            ["SKILL.md", "references/rules.md"]
        );
        assert_eq!(review.schema_digest.len(), 64);
    }

    fn install_lifecycle_test_hooks(
        root: &std::path::Path,
        profile: &mut RuntimeProfile,
    ) -> PathBuf {
        let user = root.join("hook-user");
        fs::create_dir_all(user.join("hooks")).unwrap();
        let log = root.join("hook-observations.jsonl");
        let script = r#"import json,sys,os
r=json.load(sys.stdin)
assert 'HOME' not in os.environ
with open(os.environ['LOG_FILE'],'a') as f: f.write(json.dumps(r)+'\n')
if os.environ.get('FAIL_ONCE') and not os.path.exists(os.environ['FAIL_ONCE']):
 open(os.environ['FAIL_ONCE'],'w').close(); sys.exit(3)
reply=dict(format=2,hook_id=r['hook_id'],event_id=r['event_id'])
if r['event']=='context.prepare': reply['context']=['LIFECYCLE_CONTEXT_MARKER']
print(json.dumps(reply,sort_keys=True,separators=(',',':')))
"#;
        for event in [
            "turn.before",
            "context.prepare",
            "tool.completed",
            "context.before_compact",
            "turn.settled",
        ] {
            let id = format!("{event}.json");
            let binding = json!({"format":2,"id":id,"event":event,
                "argv":["/usr/bin/python3","-c",script],"timeout_ms":1000,
                "stdout_bytes":65536,"stderr_bytes":4096,"env":{"LOG_FILE":log}});
            fs::write(
                user.join("hooks").join(id),
                tools::encode_hook_line(&binding).unwrap(),
            )
            .unwrap();
        }
        profile.instruction = InstructionResolver::new(&user, [root]).capture().unwrap();
        assert!(
            frozen_hook_bindings(&profile.instruction)
                .unwrap()
                .is_empty()
        );
        log
    }

    #[test]
    fn lifecycle_hooks_frozen_configuration_replay_and_terminal_outcomes() {
        for (outcome, reason) in [
            ("completed", None),
            ("error", Some("internal")),
            ("interrupted", Some("user_stop")),
        ] {
            let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
            let mut profile = tool_profile(directory.path(), &[]);
            let log = install_lifecycle_test_hooks(directory.path(), &mut profile);
            let hooks = lifecycle_hooks::LifecycleHooks::load(&profile, &ledger)
                .unwrap()
                .unwrap();
            let before = ledger.next_seq();
            hooks.before_turn(&ledger, 2);
            hooks.before_turn(&ledger, 2);
            assert_eq!(
                hooks.prepare(&ledger, 2, &[]),
                vec!["LIFECYCLE_CONTEXT_MARKER"]
            );
            assert_eq!(
                hooks.prepare(&ledger, 2, &[]),
                vec!["LIFECYCLE_CONTEXT_MARKER"]
            );
            hooks.before_compact(&ledger, 2, &[2], false);
            assert_eq!(
                ledger.next_seq(),
                before,
                "hooks do not fabricate model calls or mutate the ledger"
            );
            append_settle(&mut ledger, "2026-08-28T09:00:03.000Z", 2, outcome, reason).unwrap();
            hooks.observe(&mut ledger);
            let records: Vec<Value> = fs::read_to_string(&log)
                .unwrap()
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect();
            assert_eq!(
                records
                    .iter()
                    .map(|r| r["event"].as_str().unwrap())
                    .collect::<Vec<_>>(),
                vec![
                    "turn.before",
                    "context.prepare",
                    "context.before_compact",
                    "turn.settled"
                ]
            );
            assert_eq!(
                records.last().unwrap()["data"]["payload"]["record"]["outcome"],
                outcome
            );
            let hooks = lifecycle_hooks::LifecycleHooks::load(&profile, &ledger)
                .unwrap()
                .unwrap();
            hooks.observe(&mut ledger);
            assert_eq!(
                fs::read_to_string(log).unwrap().lines().count(),
                4,
                "acknowledged events are not delivered twice after reload"
            );
        }
    }

    #[test]
    fn lifecycle_observer_retries_unacknowledged_event_with_stable_identity() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let mut profile = tool_profile(directory.path(), &[]);
        let log = install_lifecycle_test_hooks(directory.path(), &mut profile);
        let user = directory.path().join("hook-user");
        let path = user.join("hooks/turn.settled.json");
        let mut binding: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        binding["env"]["FAIL_ONCE"] = json!(directory.path().join("failed-once"));
        fs::write(&path, tools::encode_hook_line(&binding).unwrap()).unwrap();
        profile.instruction = InstructionResolver::new(&user, [directory.path()])
            .capture()
            .unwrap();
        let hooks = lifecycle_hooks::LifecycleHooks::load(&profile, &ledger)
            .unwrap()
            .unwrap();
        append_settle(
            &mut ledger,
            "2026-08-28T09:00:03.000Z",
            2,
            "completed",
            None,
        )
        .unwrap();
        hooks.observe(&mut ledger);
        assert!(
            ledger.projection().unwrap().terminal_tail,
            "extension failure does not alter settlement"
        );
        // Changing live config cannot change the captured argv/env on retry.
        fs::write(path, "not valid JSON").unwrap();
        let hooks = lifecycle_hooks::LifecycleHooks::load(&profile, &ledger)
            .unwrap()
            .unwrap();
        hooks.observe(&mut ledger);
        let records: Vec<Value> = fs::read_to_string(&log)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["event_id"], records[1]["event_id"]);
        let hooks = lifecycle_hooks::LifecycleHooks::load(&profile, &ledger)
            .unwrap()
            .unwrap();
        hooks.observe(&mut ledger);
        assert_eq!(fs::read_to_string(log).unwrap().lines().count(), 2);
    }

    #[test]
    fn lifecycle_manual_compaction_notifies_before_the_compact_record() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let mut profile = tool_profile(directory.path(), &[]);
        let log = install_lifecycle_test_hooks(directory.path(), &mut profile);
        let mut options = test_options(&ledger);
        options.lifecycle_hooks = lifecycle_hooks::LifecycleHooks::load(&profile, &ledger).unwrap();
        let before = ledger.next_seq();
        let event = build_compaction_event(&mut ledger, &options, 2, true, None)
            .unwrap()
            .unwrap();
        let records: Vec<Value> = fs::read_to_string(log)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0]["event"], "context.before_compact");
        assert_eq!(records[0]["data"]["payload"]["through_seq"], before - 1);
        assert_eq!(event["kind"], "compact");
        assert_ne!(
            ledger.projection().unwrap().events.last().unwrap().kind(),
            &EventKind::Compact
        );
    }

    #[test]
    fn retired_memory_selectors_never_become_executable_tools() {
        let root = tempfile::tempdir().unwrap();
        let profile = tool_profile(root.path(), &["note", "read", "recall"]);
        assert_eq!(
            effective_allowed_tools(&profile),
            BTreeSet::from(["read".to_owned()])
        );
    }

    #[test]
    fn shell_uses_effective_workspace_write_permission_without_model_restatement() {
        let root = tempfile::tempdir().unwrap();
        let mut profile = tool_profile(root.path(), &["shell", "edit", "write"]);
        let workspace = root
            .path()
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        profile.config.workspace.policy.writable_roots = vec![workspace.clone()];
        let call = |name: &str, arguments| provider::ToolCall {
            call_id: "call".to_owned(),
            name: name.to_owned(),
            arguments,
        };
        assert_eq!(
            invocation_write_roots(&profile, &call("shell", json!({"command":"touch a.txt"})))
                .unwrap(),
            vec![workspace.clone()],
        );
        assert_eq!(
            invocation_write_roots(
                &profile,
                &call(
                    "shell",
                    json!({
                        "command":"touch a.txt", "writable_paths":[]
                    })
                )
            )
            .unwrap(),
            vec![workspace.clone()],
        );
        assert!(
            invocation_write_roots(
                &profile,
                &call(
                    "shell",
                    json!({
                        "command":"touch a.txt", "writable_paths":["../outside"]
                    })
                )
            )
            .is_err()
        );
        assert_eq!(
            invocation_write_roots(
                &profile,
                &call(
                    "write",
                    json!({
                        "path":"a.txt", "content":"hello"
                    })
                )
            )
            .unwrap(),
            vec![workspace.clone()],
        );
        assert_eq!(
            invocation_write_roots(
                &profile,
                &call(
                    "edit",
                    json!({
                        "path":"a.txt", "old_text":"hello", "new_text":"world"
                    })
                )
            )
            .unwrap(),
            vec![workspace.clone()],
        );
        profile.config.workspace.policy.writable_roots.clear();
        assert!(
            invocation_write_roots(&profile, &call("shell", json!({"command":"touch a.txt"})))
                .unwrap()
                .is_empty()
        );
        assert!(
            invocation_write_roots(
                &profile,
                &call("write", json!({"path":"a.txt", "content":"hello"}))
            )
            .is_err()
        );
        assert!(
            invocation_write_roots(
                &profile,
                &call(
                    "edit",
                    json!({"path":"a.txt", "old_text":"hello", "new_text":"world"})
                )
            )
            .is_err()
        );
    }

    fn tool_profile(root: &std::path::Path, allowed_tools: &[&str]) -> RuntimeProfile {
        let mut allowed_tools = allowed_tools
            .iter()
            .map(|name| (*name).to_owned())
            .collect::<Vec<_>>();
        allowed_tools.sort();
        allowed_tools.dedup();
        let config = ConfigSnapshot {
            format: 1,
            workspace: ResolvedWorkspace {
                format: 1,
                revision: 1,
                id: "ws".to_owned(),
                name: "ws".to_owned(),
                folder_binding: None,
                selected_cwd: None,
                cwd: vec![
                    root.canonicalize()
                        .expect("canonical workspace")
                        .to_string_lossy()
                        .into_owned(),
                ],
                policy: WorkspacePolicy {
                    allowed_tools,
                    ..WorkspacePolicy::default()
                },
            },
            providers: ProvidersConfig::default(),
            legacy_integrations: (),
            settings: SettingsConfig::default(),
            session_settings: None,
            revisions: RevisionVector {
                workspace: 1,
                providers: 0,
                legacy_integrations: (),
                settings: 0,
                session_settings: None,
            },
        };
        let bindings = LaunchBindings::bind(&config, None, Default::default())
            .expect("default launch bindings");
        RuntimeProfile {
            config,
            validator: false,
            subagent: false,
            instruction: InstructionSnapshot {
                format: 1,
                sources: Vec::new(),
                effective: EffectiveInstructions::default(),
            },
            bindings,
        }
    }

    fn test_options(ledger: &LockedLedger) -> Options {
        Options {
            ledger: ledger.path().to_owned(),
            timestamp: "2026-08-27T09:00:03.000Z".to_owned(),
            clock_started: Instant::now(),
            run_id: "tool-run".to_owned(),
            binary: "worker".to_owned(),
            config_digest: "cfg".to_owned(),
            instruction_digest: "ins".to_owned(),
            policy: "p".to_owned(),
            config_fd: None,
            instruction_fd: None,
            launch_bindings_digest: None,
            launch_bindings_fd: None,
            credential_fd: None,
            web_search_ready: false,
            provider_test_redirect: None,
            lifecycle_hooks: None,
        }
    }

    #[test]
    fn unconfigured_provider_settles_with_the_exact_owned_reason() {
        let (directory, mut ledger) = context_ledger("responses", true);
        let mut profile = tool_profile(directory.path(), &[]);
        profile.config.workspace.policy.network = true;
        let options = test_options(&ledger);
        let mut lines = std::iter::empty::<io::Result<String>>();
        let mut output = Vec::new();
        run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut None,
            &mut lines,
            &mut output,
            &RuntimeCancellation::default(),
        )
        .expect("unavailable provider terminal");

        let events = &ledger.projection().expect("projection").events;
        assert_eq!(events[11].kind(), &EventKind::Error);
        assert_eq!(
            events[11].string_field("classification"),
            Some("provider_terminal")
        );
        assert_eq!(
            events[11].string_field("detail"),
            Some("provider_not_selected")
        );
        assert_eq!(events[12].kind(), &EventKind::Settle);
        assert_eq!(events[12].string_field("outcome"), Some("error"));
        assert!(ledger.projection().expect("projection").terminal_tail);
    }

    #[test]
    fn provider_failures_keep_http_ownership_and_terminal_classification() {
        let cases = [
            (
                ProviderFailure::AuthFailure {
                    status: 401,
                    detail: Some("unauthorized".to_owned()),
                },
                "provider_terminal",
                "http_401: unauthorized",
                Some("provider_terminal"),
            ),
            (
                ProviderFailure::Transport {
                    status: Some(503),
                    detail: "upstream unavailable".to_owned(),
                    partial_fragments: None,
                },
                "transport",
                "http_503: upstream unavailable",
                None,
            ),
            (
                ProviderFailure::Malformed {
                    detail: "invalid event stream".to_owned(),
                },
                "provider_terminal",
                "malformed_response: invalid event stream",
                Some("provider_terminal"),
            ),
        ];

        for (failure, classification, detail, settle_classification) in cases {
            let (_directory, mut ledger) = context_ledger("responses", true);
            append_test_attempt(&mut ledger, "failure-attempt");
            let options = test_options(&ledger);
            let cancellation = RuntimeCancellation::default();
            let outcome = append_provider_failure(
                &mut ledger,
                &options,
                2,
                "failure-attempt",
                failure,
                &cancellation,
                DialectId::OpenaiResponsesV1,
                &EagerDispatch::default(),
            )
            .expect("append provider failure");
            let error = ledger
                .projection()
                .expect("projection")
                .events
                .iter()
                .rev()
                .find(|event| *event.kind() == EventKind::Error)
                .expect("durable error");
            assert_eq!(error.string_field("classification"), Some(classification));
            assert_eq!(error.string_field("detail"), Some(detail));
            assert_eq!(
                outcome
                    .settle
                    .and_then(|(_, classification)| classification),
                settle_classification
            );
        }
    }

    fn configured_test_provider_profile(root: &std::path::Path) -> RuntimeProfile {
        let mut profile = tool_profile(root, &[]);
        profile.config.workspace.policy.network = true;
        profile.config.workspace.policy.provider = Some("fixture".to_owned());
        profile.config.workspace.policy.model = Some("gpt-5".to_owned());
        profile.config.providers = ProvidersConfig {
            format: 1,
            revision: 1,
            providers: vec![Provider {
                id: "fixture".to_owned(),
                name: None,
                adapter: "responses".to_owned(),
                dialect: "openai_responses_v1".to_owned(),
                endpoint_owner: "openai".to_owned(),
                gateway_translation: "direct".to_owned(),
                evidence_revision: "openai-2026-08-01".to_owned(),
                endpoint: "https://api.openai.com/v1".to_owned(),
                credential_key: None,
                models: vec![Model {
                    id: "gpt-5".to_owned(),
                    profile: "openai_responses_v1:gpt-5".to_owned(),
                    enabled: true,
                    context_window_tokens: 100_000,
                    compact_trigger_tokens: 90_000,
                }],
            }],
            web_search: None,
        };
        profile.config.revisions.providers = 1;
        profile
    }

    #[test]
    fn fixed_401_is_attempted_once_and_settles_provider_terminal() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        let endpoint = format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        );
        set_provider_test_redirect(Some(endpoint));
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("provider accept");
            let mut request = [0_u8; 4096];
            let _ = stream.read(&mut request).expect("provider request");
            let body = br#"{"error":{"code":"invalid_api_key","message":"unauthorized"}}"#;
            write!(
                stream,
                "HTTP/1.1 401 Unauthorized\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                body.len()
            )
            .expect("provider response header");
            stream.write_all(body).expect("provider response body");
        });

        let profile = configured_test_provider_profile(directory.path());
        let options = test_options(&ledger);
        let mut replies = [Ok(
            "{\"lease\":{\"attempt\":\"tool-run-attempt-13\",\"granted\":true}}".to_owned(),
        )]
        .into_iter();
        let mut output = Vec::new();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut None,
            &mut replies,
            &mut output,
            &RuntimeCancellation::default(),
        );
        set_provider_test_redirect(None);
        result.expect("401 terminal result");
        server.join().expect("provider server");

        let events = &ledger.projection().expect("projection").events;
        let attempts = events
            .iter()
            .filter(|event| *event.kind() == EventKind::Attempt && event.turn() == Some(2))
            .count();
        assert_eq!(attempts, 1, "authentication failure must not retry");
        let tail = &events[events.len() - 2..];
        assert_eq!(tail[0].kind(), &EventKind::Error);
        assert_eq!(
            tail[0].string_field("classification"),
            Some("provider_terminal")
        );
        assert_eq!(tail[1].kind(), &EventKind::Settle);
        assert_eq!(tail[1].string_field("outcome"), Some("error"));
        assert_eq!(
            tail[1].string_field("classification"),
            Some("provider_terminal")
        );
    }

    #[test]
    fn permanent_400_releases_queued_retry_body_after_error_settlement() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let options = test_options(&ledger);
        let queued = ledger.next_seq();
        ledger
            .append_contract(
                make_event(
                    json!({"v":1,"seq":queued,"kind":"input","ts":options.timestamp,
            "content":[{"type":"text","text":"重试"}],"origin_key":"queued-after-400",
            "origin_tuple":{"principal":"test","client":"test","target":"thread",
                "op":"submit","key":"queued-after-400"}}),
                )
                .unwrap(),
                BarrierContext::default(),
            )
            .unwrap();
        assert_eq!(
            open_ready_turn(&mut ledger, &options.timestamp, RunMode::Ordinary).unwrap(),
            None
        );
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        set_provider_test_redirect(Some(format!("http://{}", listener.local_addr().unwrap())));
        let server = std::thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(5);
            let mut stream = loop {
                match listener.accept() {
                    Ok((stream, _)) => break stream,
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                        assert!(Instant::now() < deadline);
                        std::thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => panic!("accept: {error}"),
                }
            };
            // macOS can inherit O_NONBLOCK from the listening socket. The
            // bounded request reader below requires blocking reads.
            stream.set_nonblocking(false).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut bytes = Vec::new();
            loop {
                let mut chunk = [0; 4096];
                let count = stream.read(&mut chunk).unwrap();
                assert!(count > 0);
                bytes.extend_from_slice(&chunk[..count]);
                if let Some(end) = bytes.windows(4).position(|b| b == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&bytes[..end]).to_lowercase();
                    let length: usize = headers
                        .lines()
                        .find_map(|line| line.strip_prefix("content-length: "))
                        .unwrap()
                        .parse()
                        .unwrap();
                    if bytes.len() >= end + 4 + length {
                        break;
                    }
                }
            }
            let body = br#"{"error":{"message":"model not available"}}"#;
            write!(stream, "HTTP/1.1 400 Bad Request\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", body.len()).unwrap();
            stream.write_all(body).unwrap();
        });
        let profile = configured_test_provider_profile(directory.path());
        let attempt_seq = ledger.next_seq() + 1; // Initial provider epoch precedes the attempt.
        let mut replies = [Ok(format!(
            "{{\"lease\":{{\"attempt\":\"tool-run-attempt-{attempt_seq}\",\"granted\":true}}}}"
        ))]
        .into_iter();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut None,
            &mut replies,
            &mut Vec::new(),
            &RuntimeCancellation::default(),
        );
        set_provider_test_redirect(None);
        result.unwrap();
        server.join().unwrap();
        let events = &ledger.projection().unwrap().events;
        assert_eq!(
            events
                .iter()
                .filter(|e| e.kind() == &EventKind::Attempt && e.turn() == Some(2))
                .count(),
            1
        );
        let settled = events.last().unwrap();
        assert_eq!(settled.kind(), &EventKind::Settle);
        assert_eq!(settled.string_field("outcome"), Some("error"));
        let settled_seq = settled.seq();
        assert!(events.iter().any(|e| {
            e.kind() == &EventKind::Error
                && e.string_field("detail")
                    .is_some_and(|d| d.contains("http_400") && d.contains("model not available"))
        }));
        assert_eq!(
            open_ready_turn(&mut ledger, &options.timestamp, RunMode::Ordinary).unwrap(),
            Some(3)
        );
        assert_eq!(current_turn_inputs(&ledger, 3).unwrap(), vec![queued]);
        assert!(ledger.projection().unwrap().events.last().unwrap().seq() > settled_seq);
        let next = ledger.next_seq();
        assert_eq!(
            open_ready_turn(&mut ledger, &options.timestamp, RunMode::Ordinary).unwrap(),
            None
        );
        assert_eq!(ledger.next_seq(), next);
    }

    fn assert_retry_exhaustion(status: &str, body: &'static [u8], classification: &str) {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        let endpoint = format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        );
        set_provider_test_redirect(Some(endpoint));
        let status = status.to_owned();
        let server = std::thread::spawn(move || {
            for _ in 0..4 {
                let (mut stream, _) = listener.accept().expect("provider accept");
                let mut request = [0_u8; 4096];
                let _ = stream.read(&mut request).expect("provider request");
                write!(
                    stream,
                    "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                    body.len()
                )
                .expect("provider response header");
                stream.write_all(body).expect("provider response body");
            }
        });

        let profile = configured_test_provider_profile(directory.path());
        let options = test_options(&ledger);
        // Every transient retry writes a durable admission wait before the
        // next attempt, so its attempts sit one seq later each time.
        let attempt_seqs = [13, 18, 23, 28];
        let mut replies = attempt_seqs
            .map(|seq| {
                Ok(format!(
                    "{{\"lease\":{{\"attempt\":\"tool-run-attempt-{seq}\",\"granted\":true}}}}"
                ))
            })
            .into_iter();
        let mut output = Vec::new();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut None,
            &mut replies,
            &mut output,
            &RuntimeCancellation::default(),
        );
        set_provider_test_redirect(None);
        result.expect("retry exhaustion");
        server.join().expect("provider server");

        let events = &ledger.projection().expect("projection").events;
        assert_eq!(
            events
                .iter()
                .filter(|event| *event.kind() == EventKind::Attempt && event.turn() == Some(2))
                .count(),
            4
        );
        let admission_waits = events
            .iter()
            .filter(|event| {
                *event.kind() == EventKind::State
                    && event.string_field("subkind") == Some(PROVIDER_ADMISSION_SUBKIND)
            })
            .count();
        assert_eq!(admission_waits, 3);
        let waits = events
            .iter()
            .filter(|event| {
                *event.kind() == EventKind::State
                    && event.string_field("subkind") == Some(PROVIDER_ADMISSION_SUBKIND)
            })
            .map(|event| serde_json::to_value(event.raw()).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            waits
                .iter()
                .map(|w| w["payload"]["wait_ms"].as_u64().unwrap())
                .collect::<Vec<_>>(),
            vec![1000, 2000, 4000]
        );
        assert!(
            waits
                .iter()
                .all(|w| w["payload"]["classification"] == classification)
        );
        let settle = events.last().expect("terminal settle");
        assert_eq!(settle.kind(), &EventKind::Settle);
        assert_eq!(settle.string_field("outcome"), Some("error"));
        assert_eq!(settle.string_field("classification"), Some(classification));
    }

    #[test]
    fn fixed_429_retries_and_exhausts_as_rate_limit() {
        assert_retry_exhaustion(
            "429 Too Many Requests",
            br#"{"error":{"code":"rate_limit","message":"slow down"}}"#,
            "rate_limit",
        );
    }

    #[test]
    fn fixed_500_retries_and_exhausts_as_transport() {
        assert_retry_exhaustion(
            "500 Internal Server Error",
            br#"{"error":{"code":"upstream","message":"failed"}}"#,
            "transport",
        );
    }

    #[test]
    fn provider_cancellation_records_the_runtime_owner() {
        let (_directory, mut ledger) = context_ledger("responses", true);
        append_test_attempt(&mut ledger, "cancel-attempt");
        let options = test_options(&ledger);
        let cancellation = RuntimeCancellation::default();
        cancellation.cancel(CANCEL_STOP);
        let outcome = append_provider_failure(
            &mut ledger,
            &options,
            2,
            "cancel-attempt",
            ProviderFailure::Cancelled,
            &cancellation,
            DialectId::OpenaiResponsesV1,
            &EagerDispatch::default(),
        )
        .expect("append cancellation");
        let error = ledger
            .projection()
            .expect("projection")
            .events
            .iter()
            .rev()
            .find(|event| *event.kind() == EventKind::Error)
            .expect("durable cancellation");
        assert_eq!(error.string_field("detail"), Some("user_stop"));
        assert_eq!(outcome.settle, Some(("interrupted", Some("user_stop"))));
    }

    #[test]
    fn unexpected_post_turn_error_is_durably_owned_by_the_worker() {
        let (_directory, mut ledger) = context_ledger("responses", true);
        let options = test_options(&ledger);
        let error = io::Error::other("injected provider loop failure");
        settle_internal_worker_failure(&mut ledger, &options, &error)
            .expect("worker-owned terminal failure");

        let events = &ledger.projection().expect("projection").events;
        let error = events
            .iter()
            .rev()
            .find(|event| *event.kind() == EventKind::Error)
            .expect("durable internal error");
        assert_eq!(error.string_field("classification"), Some("internal"));
        assert_eq!(
            error.string_field("detail"),
            Some("injected provider loop failure")
        );
        let settle = events.last().expect("terminal settle");
        assert_eq!(settle.kind(), &EventKind::Settle);
        assert_eq!(settle.string_field("outcome"), Some("error"));
        assert_eq!(settle.string_field("classification"), Some("internal"));
    }

    fn append_test_attempt(ledger: &mut LockedLedger, id: &str) {
        let event = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":2,"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},
            "ts":"2026-08-27T09:00:03.000Z","attempt":id,"epoch":"e1",
            "wire_digest":"test-wire","admits":[{"from":9,"to":9}]
        }))
        .expect("attempt event");
        ledger
            .append_contract(event, BarrierContext::default())
            .expect("append attempt");
    }

    fn append_test_call(
        ledger: &mut LockedLedger,
        attempt: &str,
        call: &str,
        name: &str,
        args: Value,
    ) {
        let effect = BuiltinManifest::compiled()
            .tools
            .into_iter()
            .find(|tool| tool.name == name)
            .expect("fixed tool")
            .effect;
        let event = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":2,"kind":"tool_call",
            "ts":"2026-08-27T09:00:03.000Z","attempt":attempt,"call":call,
            "name":name,"args":args,"source":"provider"
        }))
        .expect("tool_call event");
        ledger
            .append_contract(
                event,
                BarrierContext {
                    side_effectful_tool_call: engine::side_effectful(effect),
                },
            )
            .expect("append tool_call");
    }

    #[test]
    fn catalog_role_is_derived_once_and_conflicts_fail_closed() {
        let directory = tempfile::tempdir().expect("workspace");
        let plan = tool_profile(directory.path(), &["plan"]);
        assert_eq!(
            tool_catalog_context(&plan, false, false)
                .expect("plan role")
                .role,
            CatalogRole::Plan
        );
        let conflict = tool_profile(directory.path(), &["plan", "verify"]);
        assert!(tool_catalog_context(&conflict, false, false).is_err());
        let root_report = tool_profile(directory.path(), &["report"]);
        assert!(tool_catalog_context(&root_report, false, false).is_err());
        let mut delegated = tool_profile(directory.path(), &["read"]);
        delegated.subagent = true;
        let context = tool_catalog_context(&delegated, false, false).unwrap();
        assert_eq!(context.role, CatalogRole::Subagent);
        assert!(context.selected_tools.contains("report"));
        assert!(
            BuiltinManifest::compiled()
                .projection(&context)
                .iter()
                .any(|tool| tool.name == "report")
        );

        let root = tool_profile(directory.path(), &["read", "shell", "write"]);
        let manifest = BuiltinManifest::compiled();
        let root_catalog = manifest.projection(&tool_catalog_context(&root, false, false).unwrap());
        assert!(!root_catalog.iter().any(|tool| tool.name == "verify"));
        let mut judge = root.clone();
        judge.validator = true;
        let judge_catalog =
            manifest.projection(&tool_catalog_context(&judge, false, false).unwrap());
        assert!(judge_catalog.iter().any(|tool| tool.name == "verify"));
        assert_eq!(
            root_catalog,
            manifest.projection(&tool_catalog_context(&root, false, false).unwrap())
        );
    }

    #[test]
    fn web_search_catalog_requires_network_config_and_active_launch_readiness() {
        let directory = tempfile::tempdir().expect("workspace");
        let mut profile = tool_profile(directory.path(), &["web_search"]);
        profile.config.workspace.policy.network = true;
        profile.config.providers.web_search = Some(WebSearch {
            adapter: "tavily_v1".to_owned(),
            endpoint: "https://api.tavily.com".to_owned(),
            credential_key: "web-search-main".to_owned(),
        });
        let manifest = BuiltinManifest::compiled();
        let visible = manifest
            .projection(&tool_catalog_context(&profile, false, true).expect("ready catalog"));
        assert!(visible.iter().any(|tool| tool.name == "web_search"));
        let unavailable = manifest
            .projection(&tool_catalog_context(&profile, false, false).expect("not-ready catalog"));
        assert!(!unavailable.iter().any(|tool| tool.name == "web_search"));
        profile.config.workspace.policy.network = false;
        let denied = manifest.projection(
            &tool_catalog_context(&profile, false, true).expect("network denied catalog"),
        );
        assert!(!denied.iter().any(|tool| tool.name == "web_search"));
    }

    #[test]
    fn web_search_scope_is_independent_from_model_provider_scopes() {
        let directory = tempfile::tempdir().expect("workspace");
        let mut profile = tool_profile(directory.path(), &["web_search"]);
        profile.config.providers.providers = vec![Provider {
            id: "model-provider".to_owned(),
            name: None,
            adapter: "responses".to_owned(),
            dialect: "openai_responses_v1".to_owned(),
            endpoint_owner: "openai".to_owned(),
            gateway_translation: "direct".to_owned(),
            evidence_revision: "openai-2026-08-01".to_owned(),
            endpoint: "https://models.example/v1".to_owned(),
            credential_key: Some("shared-key".to_owned()),
            models: vec![Model {
                id: "gpt-5".to_owned(),
                profile: "openai_responses_v1:gpt-5".to_owned(),
                enabled: true,
                context_window_tokens: 100,
                compact_trigger_tokens: 50,
            }],
        }];
        profile.config.providers.web_search = Some(WebSearch {
            adapter: "tavily_v1".to_owned(),
            endpoint: "https://search.example".to_owned(),
            credential_key: "shared-key".to_owned(),
        });
        let store = provider::MemorySecretStore::new();
        store
            .publish(
                "shared-key",
                provider::SecretRecord::Active {
                    generation: 1,
                    material: "fixture-secret-never-log".to_owned(),
                },
            )
            .expect("secret");
        let scopes =
            provider::resolve_config_credentials(&profile.config, &store).expect("resolved scopes");
        assert_eq!(scopes.active.len(), 2);
        assert!(scopes.active.iter().any(|scope| {
            scope.adapter == "responses"
                && scope.endpoint_origin == "https://models.example:443"
                && scope.purpose == "provider"
        }));
        assert!(scopes.active.iter().any(|scope| {
            scope.adapter == "tavily_v1"
                && scope.endpoint_origin == "https://search.example:443"
                && scope.purpose == "web_search"
        }));
        assert!(
            !scopes
                .active
                .iter()
                .any(|scope| { scope.adapter == "responses" && scope.purpose == "web_search" })
        );
    }

    #[test]
    fn adopted_response_must_preserve_durable_call_arguments_and_name() {
        let (_directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        append_test_attempt(&mut ledger, "adopt-attempt");
        let args = json!({"thought":"already dispatched".repeat(2000)});
        let spilled = spill_json(&ledger, &args).unwrap();
        assert!(spilled.get("$spill").is_some());
        append_test_call(&mut ledger, "adopt-attempt", "adopt-call", "think", spilled);
        let original = provider::ToolCall {
            call_id: "adopt-call".into(),
            name: "think".into(),
            arguments: args,
        };
        let before = fs::read(ledger.path()).unwrap();
        validate_adopted_calls(&ledger, "adopt-attempt", &[original.clone()]).unwrap();
        let mut renamed = original.clone();
        renamed.name = "shell".into();
        let mut changed = original.clone();
        changed.arguments = json!({"thought":"replacement"});
        let mut reidentified = original.clone();
        reidentified.call_id = "another-call".into();
        for calls in [vec![], vec![renamed], vec![changed], vec![reidentified]] {
            assert!(validate_adopted_calls(&ledger, "adopt-attempt", &calls).is_err());
            assert_eq!(fs::read(ledger.path()).unwrap(), before);
        }
        // A different attempt's records do not belong to the adopted response.
        validate_adopted_calls(&ledger, "another-attempt", &[]).unwrap();
    }

    #[test]
    fn slice8_gate_60_tool_write_ahead_crash_matrix() {
        let (directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let profile = tool_profile(directory.path(), &["think", "shell"]);
        let options = test_options(&ledger);
        append_test_attempt(&mut ledger, "tool-attempt");
        append_test_call(
            &mut ledger,
            "tool-attempt",
            "call-safe",
            "think",
            json!({"thought":"recover after the provider-output crash window"}),
        );
        let value = json!({
            "v":1,"seq":ledger.next_seq(),"turn":2,"kind":"output",
            "ts":options.timestamp,"attempt":"tool-attempt","content":[],
            "usage":{"availability":"reported","input_tokens":"1","output_tokens":"1"},
            "sealed":{"version":1,"adapter":"anthropic_messages_v1","fragments":"[]"}
        });
        ledger
            .append_contract(
                make_event(value).expect("provider terminal event"),
                BarrierContext::default(),
            )
            .expect("append provider terminal event");
        assert_eq!(
            pending_tool_calls(&ledger)
                .expect("pending crash call")
                .len(),
            1,
            "provider output can be durable while the preceding call is unpaired"
        );
        let mut no_replies = std::iter::empty();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        assert!(
            !recover_unpaired_tool_calls(
                &mut ledger,
                &options,
                Some(&profile),
                Selected { version: 2 },
                false,
                &mut no_replies,
                &mut output,
                &cancellation,
            )
            .expect("safe recovery")
        );
        let safe_results = ledger
            .projection()
            .expect("projection")
            .events
            .iter()
            .filter(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("call-safe")
            })
            .count();
        assert_eq!(safe_results, 1, "replay-safe calls execute exactly once");

        append_test_call(
            &mut ledger,
            "tool-attempt",
            "call-unsafe",
            "shell",
            json!({
                "working_directory":"", "max_duration_ms":1000,
                "command":"touch must-not-run", "program":null, "args":[], "steps":[],
                "failure_policy":"stop_on_error", "writable_paths":[], "artifact_outputs":[]
            }),
        );
        recover_unpaired_tool_calls(
            &mut ledger,
            &options,
            Some(&profile),
            Selected { version: 2 },
            false,
            &mut no_replies,
            &mut output,
            &cancellation,
        )
        .expect("unsafe recovery");
        let unsafe_result = ledger
            .projection()
            .expect("projection")
            .events
            .iter()
            .find(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("call-unsafe")
            })
            .expect("aborted unsafe result");
        let raw = serde_json::to_value(unsafe_result.raw()).expect("result JSON");
        assert_eq!(raw.pointer("/outcome/aborted"), Some(&json!("crash")));
        assert!(!directory.path().join("must-not-run").exists());
    }

    fn exit_code_is(code: ExitCode, expected: ExitCode) -> bool {
        format!("{code:?}") == format!("{expected:?}")
    }

    fn stop_control_lines(stop_line: Option<&str>) -> (ControlLines, RuntimeCancellation) {
        let cancellation = RuntimeCancellation::default();
        cancellation.cancel(CANCEL_STOP);
        if let Some(line) = stop_line {
            cancellation.defer(line.to_owned());
        }
        let (_sender, receiver) = sync_channel(1);
        (
            ControlLines {
                receiver,
                cancellation: cancellation.clone(),
            },
            cancellation,
        )
    }

    #[test]
    fn reader_thread_parks_deliveries_and_channels_stops_and_replies() {
        let cancellation = RuntimeCancellation::default();
        let (sender, receiver) = sync_channel(8);
        let lines = [
            r#"{"input":{"assets":null,"content":[{"text":"queued","type":"text"}],"delivery":"d-in","origin":{"client":"cli","key":"in-1","op":"submit","principal":"p","target":"t"},"steer":null}}"#,
            r#"{"lease":{"attempt":"tool-run-attempt-13","granted":true}}"#,
            r#"{"meta":{"delivery":"d-meta","origin":{"client":"cli","key":"meta-1","op":"session.rename","principal":"p","target":"t"},"title":"Renamed"}}"#,
            r#"{"stop":{"delivery":"d-stop","generation":1,"origin":{"client":"cli","key":"stop-1","op":"session.cancel","principal":"p","target":"t"}}}"#,
        ]
        .into_iter()
        .map(|line| Ok(line.to_owned()));
        forward_control_lines(lines, sender, cancellation.clone());

        let channel = std::iter::from_fn(|| receiver.try_recv().ok())
            .map(|line| control_line_key(&line.expect("line")))
            .collect::<Vec<_>>();
        assert_eq!(
            channel,
            [Some("lease".to_owned()), Some("stop".to_owned())],
            "only protocol replies and stops reach the waits"
        );
        assert!(cancellation.stop_requested());
        let parked = cancellation
            .take_parked_deliveries()
            .iter()
            .map(|line| control_line_key(line))
            .collect::<Vec<_>>();
        assert_eq!(parked, [Some("input".to_owned()), Some("meta".to_owned())]);
    }

    #[test]
    fn compact_at_predispatch_yield_rebuilds_before_requesting_a_lease() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let options = test_options(&ledger);
        let profile = configured_test_provider_profile(directory.path());
        let cancellation = RuntimeCancellation::default();
        cancellation.park_delivery(r#"{"compact":{"delivery":"preflight-compact","origin":{"client":"cli","key":"preflight-compact","op":"compact","principal":"p","target":"t"}}}"#.into());
        let mut replies = [Ok(
            r#"{"lease":{"attempt":"tool-run-attempt-16","granted":false}}"#.to_owned(),
        )]
        .into_iter();
        let mut output = Vec::new();
        run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut None,
            &mut replies,
            &mut output,
            &cancellation,
        )
        .unwrap();
        let events = &ledger.projection().unwrap().events;
        let compact = events
            .iter()
            .find(|e| *e.kind() == EventKind::Compact)
            .unwrap();
        let epoch = events
            .iter()
            .rev()
            .find(|e| *e.kind() == EventKind::Epoch)
            .unwrap();
        assert!(compact.seq() < epoch.seq());
        assert_eq!(epoch.string_field("reason"), Some("compaction"));
        assert_eq!(
            epoch.integer_field("renderer"),
            Some(CONTEXT_RENDERER_VERSION)
        );
        assert!(
            !events
                .iter()
                .any(|e| *e.kind() == EventKind::Attempt && e.seq() > 12)
        );
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("tool-run-attempt-16"));
        assert!(
            !output.contains("tool-run-attempt-13"),
            "stale prepared request must never ask for a lease"
        );
    }

    #[test]
    fn manual_compact_is_durable_receipted_and_idempotent_across_reopen() {
        for with_history in [false, true] {
            let (_directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
            let path = ledger.path().to_owned();
            if !with_history {
                let prefix = fs::read_to_string(&path)
                    .unwrap()
                    .lines()
                    .take(5)
                    .collect::<Vec<_>>()
                    .join("\n")
                    + "\n";
                drop(ledger);
                fs::write(&path, prefix).unwrap();
                ledger = LockedLedger::open(&path, 1).unwrap();
            }
            let options = test_options(&ledger);
            let before = fs::read(&path).unwrap();
            let message = r#"{"compact":{"delivery":"compact-1","origin":{"client":"cli","key":"compact-key","op":"compact","principal":"p","target":"t"}}}"#;
            assert!(is_delivery_control_line(message));
            let mut output = Vec::new();
            handle_delivery(
                &mut ledger,
                &options,
                &Selected { version: 2 },
                &mut output,
                decode_supervisor(message.as_bytes()).unwrap(),
            )
            .unwrap();
            let events = &ledger.projection().unwrap().events;
            let compact = events.last().unwrap();
            assert_eq!(*compact.kind(), EventKind::Compact);
            assert_eq!(compact.string_field("origin_key"), Some("compact-key"));
            assert_eq!(*events[events.len() - 2].kind(), EventKind::Checkpoint);
            let receipt: Value =
                serde_json::from_slice(output.split(|b| *b == b'\n').next().unwrap()).unwrap();
            assert_eq!(receipt["receipt"]["seq"], compact.seq());
            assert_eq!(receipt["receipt"]["deduplicated"], false);
            let after = fs::read(&path).unwrap();
            assert!(after.starts_with(&before));
            assert!(
                latest_compatible_epoch(&ledger, DialectId::AnthropicMessagesV1, "m", "d", "td", 1)
                    .is_none(),
                "manual compaction closes the old epoch"
            );
            drop(ledger);
            let mut ledger = LockedLedger::open(&path, 1).unwrap();
            let mut repeated = Vec::new();
            handle_delivery(
                &mut ledger,
                &options,
                &Selected { version: 2 },
                &mut repeated,
                decode_supervisor(message.replace("compact-1", "compact-retry").as_bytes())
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(
                fs::read(&path).unwrap(),
                after,
                "retry must not append another checkpoint or compact"
            );
            let receipt: Value =
                serde_json::from_slice(repeated.split(|b| *b == b'\n').next().unwrap()).unwrap();
            assert_eq!(receipt["receipt"]["deduplicated"], true);
        }
    }

    #[test]
    fn parked_deliveries_are_appended_and_receipted_at_a_yield_point() {
        let (_directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let options = test_options(&ledger);
        let cancellation = RuntimeCancellation::default();
        cancellation.park_delivery(
            r#"{"input":{"assets":null,"content":[{"text":"while busy","type":"text"}],"delivery":"d-busy","origin":{"client":"cli","key":"busy-1","op":"submit","principal":"p","target":"t"},"steer":null}}"#.to_owned(),
        );
        cancellation.park_delivery(
            r#"{"input":{"assets":null,"content":[{"text":"retry body","type":"text"}],"delivery":"d-busy-retry","origin":{"client":"cli","key":"busy-1","op":"submit","principal":"p","target":"t"},"steer":null}}"#.to_owned(),
        );
        let mut output = Vec::new();
        drain_parked_deliveries(
            &mut ledger,
            &options,
            &Selected { version: 2 },
            &cancellation,
            &mut output,
        )
        .expect("drain parked deliveries");
        let projection = ledger.projection().expect("projection");
        let queued = projection
            .events
            .iter()
            .filter(|event| *event.kind() == EventKind::Input)
            .filter(|event| event.string_field("origin_key") == Some("busy-1"))
            .count();
        assert_eq!(
            queued, 1,
            "the retry with the same origin tuple is deduplicated"
        );
        let output = String::from_utf8(output).expect("UTF-8");
        assert!(output.contains("\"receipt\":{\"deduplicated\":false,\"delivery\":\"d-busy\""));
        assert!(
            output.contains("\"receipt\":{\"deduplicated\":true,\"delivery\":\"d-busy-retry\"")
        );
        assert!(cancellation.take_parked_deliveries().is_empty());
        assert!(
            !projection.terminal_tail,
            "a queued input does not change the open turn"
        );
    }

    #[test]
    fn deliveries_are_receipted_while_the_provider_request_is_in_flight() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        let endpoint = format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        );
        set_provider_test_redirect(Some(endpoint));
        let cancellation = RuntimeCancellation::default();
        let parker = cancellation.clone();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("provider accept");
            let mut request = [0_u8; 4096];
            let _ = stream.read(&mut request).expect("provider request");
            // The user queues a message while the provider is still thinking.
            parker.park_delivery(
                r#"{"input":{"assets":null,"content":[{"text":"queued mid-request","type":"text"}],"delivery":"d-flight","origin":{"client":"cli","key":"in-flight","op":"submit","principal":"p","target":"t"},"submission":{"provider":"openai","model":"gpt-5","reasoningEffort":"high","configDigest":"captured-at-send"},"steer":null}}"#.to_owned(),
            );
            std::thread::sleep(Duration::from_millis(300));
            let body = br#"{"error":{"code":"invalid_api_key","message":"unauthorized"}}"#;
            write!(
                stream,
                "HTTP/1.1 401 Unauthorized\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                body.len()
            )
            .expect("provider response header");
            stream.write_all(body).expect("provider response body");
        });

        let profile = configured_test_provider_profile(directory.path());
        let options = test_options(&ledger);
        let mut replies = [Ok(
            "{\"lease\":{\"attempt\":\"tool-run-attempt-13\",\"granted\":true}}".to_owned(),
        )]
        .into_iter();
        let mut output = Vec::new();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut None,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        result.expect("terminal result");
        server.join().expect("provider server");

        let events = &ledger.projection().expect("projection").events;
        let queued_seq = events
            .iter()
            .find(|event| event.string_field("origin_key") == Some("in-flight"))
            .map(Event::seq)
            .expect("the in-flight delivery was appended");
        let input = serde_json::to_value(
            events
                .iter()
                .find(|event| event.seq() == queued_seq)
                .unwrap()
                .raw(),
        )
        .unwrap();
        assert_eq!(input["submission"]["model"], "gpt-5");
        assert_eq!(input["submission"]["reasoningEffort"], "high");
        let outcome_seq = events
            .iter()
            .find(|event| {
                matches!(event.kind(), EventKind::Output | EventKind::Error)
                    && event.string_field("attempt") == Some("tool-run-attempt-13")
            })
            .map(Event::seq)
            .expect("attempt outcome");
        assert!(
            queued_seq < outcome_seq,
            "the delivery must land while the request is in flight, before its outcome"
        );
        let output = String::from_utf8(output).expect("UTF-8");
        let receipt_at = output
            .find("\"receipt\":{\"deduplicated\":false,\"delivery\":\"d-flight\"")
            .expect("receipt for the in-flight delivery");
        let settled_at = output
            .find("\"attempt_settled\"")
            .expect("attempt_settled after the outcome");
        assert!(
            receipt_at < settled_at,
            "receipt precedes the attempt outcome: {output}"
        );
        assert!(cancellation.take_parked_deliveries().is_empty());
    }

    #[test]
    fn post_turn_open_turn_without_stop_exits_for_tail_recovery() {
        let (_directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let options = test_options(&ledger);
        let mut output = Vec::new();
        let code = post_turn_exit(
            &mut ledger,
            &options,
            &RuntimeCancellation::default(),
            &mut output,
        )
        .expect("post-turn exit");
        assert!(exit_code_is(code, ExitCode::SUCCESS));
        let projection = ledger.projection().expect("projection");
        assert!(
            !projection.terminal_tail,
            "no settle is invented for an open turn"
        );
        assert!(matches!(
            projection.events.last().expect("last event").kind(),
            EventKind::TurnOpen
        ));
        assert_eq!(
            String::from_utf8(output).expect("UTF-8"),
            "{\"appended\":{\"seq\":11}}\n"
        );
    }

    #[test]
    fn post_turn_stop_without_unresolved_work_echoes_the_stop_and_settles_user_stop() {
        let (_directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let options = test_options(&ledger);
        let (mut lines, cancellation) = stop_control_lines(Some(
            r#"{"stop":{"delivery":"stop-1","generation":1,"origin":{"client":"cli","key":"stop-key","op":"session.cancel","principal":"p","target":"t"}}}"#,
        ));
        let mut output = Vec::new();
        drain_ready_deliveries(
            &mut ledger,
            &options,
            &Selected { version: 2 },
            &mut lines,
            &mut output,
        )
        .expect("echo stop");
        let code = post_turn_exit(&mut ledger, &options, &cancellation, &mut output)
            .expect("post-turn exit");
        assert!(exit_code_is(code, ExitCode::SUCCESS));
        let projection = ledger.projection().expect("projection");
        let kinds = projection
            .events
            .iter()
            .rev()
            .take(2)
            .map(|event| event.kind().clone())
            .collect::<Vec<_>>();
        assert!(matches!(
            kinds.as_slice(),
            [EventKind::Settle, EventKind::StopRequested]
        ));
        let settle = projection.events.last().expect("settle");
        assert_eq!(settle.string_field("outcome"), Some("interrupted"));
        assert_eq!(settle.string_field("reason"), Some("user_stop"));
        assert!(projection.terminal_tail);
        assert!(
            !projection.lifecycle.stop_active,
            "the settle closes the generation"
        );
        let output = String::from_utf8(output).expect("UTF-8");
        assert!(output.contains("\"receipt\":{\"deduplicated\":false,\"delivery\":\"stop-1\""));
        assert!(output.ends_with("{\"appended\":{\"seq\":13}}\n"));
    }

    #[test]
    fn channel_stop_is_echoed_before_the_user_stop_settle() {
        let (_directory, mut ledger) = context_ledger("responses", true);
        append_test_attempt(&mut ledger, "cancel-attempt");
        let options = test_options(&ledger);
        // The reader thread flagged the stop while the provider request was in flight; the
        // line itself is still buffered in the channel, never deferred.
        let cancellation = RuntimeCancellation::default();
        cancellation.cancel(CANCEL_STOP);
        let (sender, receiver) = sync_channel(1);
        sender
            .send(Ok(
                r#"{"stop":{"delivery":"stop-2","generation":1,"origin":{"client":"cli","key":"stop-key-2","op":"session.cancel","principal":"p","target":"t"}}}"#.to_owned(),
            ))
            .expect("buffer stop line");
        let mut lines = ControlLines {
            receiver,
            cancellation: cancellation.clone(),
        };
        let outcome = append_provider_failure(
            &mut ledger,
            &options,
            2,
            "cancel-attempt",
            ProviderFailure::Cancelled,
            &cancellation,
            DialectId::OpenaiResponsesV1,
            &EagerDispatch::default(),
        )
        .expect("append cancellation");
        assert_eq!(outcome.settle, Some(("interrupted", Some("user_stop"))));

        let mut output = Vec::new();
        drain_ready_deliveries(
            &mut ledger,
            &options,
            &Selected { version: 2 },
            &mut lines,
            &mut output,
        )
        .expect("echo stop");
        let code =
            post_turn_exit(&mut ledger, &options, &cancellation, &mut output).expect("post-turn");
        assert!(exit_code_is(code, ExitCode::SUCCESS));
        let projection = ledger.projection().expect("projection");
        let kinds = projection
            .events
            .iter()
            .rev()
            .take(3)
            .map(|event| event.kind().clone())
            .collect::<Vec<_>>();
        assert!(
            matches!(
                kinds.as_slice(),
                [
                    EventKind::Settle,
                    EventKind::StopRequested,
                    EventKind::Error
                ]
            ),
            "stop_requested must be durable before the settle: {kinds:?}"
        );
        let settle = projection.events.last().expect("settle");
        assert_eq!(settle.string_field("reason"), Some("user_stop"));
        assert!(!projection.lifecycle.stop_active);
    }

    #[test]
    fn lease_denial_leaves_the_turn_open_for_tail_recovery() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        // A closed port: the denied attempt must never reach the provider.
        set_provider_test_redirect(Some("http://127.0.0.1:9".to_owned()));
        let profile = configured_test_provider_profile(directory.path());
        let options = test_options(&ledger);
        let mut replies = [Ok(
            "{\"lease\":{\"attempt\":\"tool-run-attempt-13\",\"granted\":false}}".to_owned(),
        )]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut None,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        result.expect("denial is not a failure");

        let events = &ledger.projection().expect("projection").events;
        assert!(
            !events
                .iter()
                .any(|event| *event.kind() == EventKind::Attempt && event.turn() == Some(2)),
            "a denied lease authors no attempt"
        );
        let code = post_turn_exit(&mut ledger, &options, &cancellation, &mut output)
            .expect("post-turn exit");
        assert!(exit_code_is(code, ExitCode::SUCCESS));
        let facts = ledger.projection().expect("projection").lifecycle.clone();
        assert!(!facts.terminal_tail);
        assert!(!facts.unresolved_work);
        assert_eq!(
            classify(&facts, LockFacts::FREE),
            TailState::RecoveryNeeded,
            "the sweep reclassifies the open turn once the lock is free"
        );
    }

    #[test]
    fn post_turn_stop_with_unresolved_work_exits_nonzero_without_settle() {
        let (_directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let options = test_options(&ledger);
        append_test_attempt(&mut ledger, "stopped-attempt");
        let (_lines, cancellation) = stop_control_lines(None);
        let mut output = Vec::new();
        let code = post_turn_exit(&mut ledger, &options, &cancellation, &mut output)
            .expect("post-turn exit");
        assert!(exit_code_is(code, ExitCode::FAILURE));
        let projection = ledger.projection().expect("projection");
        assert!(projection.lifecycle.unresolved_work);
        assert!(
            !projection.terminal_tail,
            "the recovery run owns the settle"
        );
        assert!(matches!(
            projection.events.last().expect("last event").kind(),
            EventKind::Attempt
        ));
    }

    /// `cancelQuestion` (SessionQuestionCancellationEndpoint): the client's
    /// keyed `approval_response {grant:false}` without an `answer` closes the
    /// held `ask_user_questions` call with a denied `tool_result`; the turn
    /// continues and nothing stops the session.
    #[test]
    fn question_cancellation_denies_held_call_and_keeps_session_running() {
        let (directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let profile = tool_profile(directory.path(), &["ask_user_questions"]);
        let options = test_options(&ledger);
        append_test_attempt(&mut ledger, "hold-attempt");
        let call = provider::ToolCall {
            call_id: "call-cancel".to_owned(),
            name: "ask_user_questions".to_owned(),
            arguments: json!({"question":"Continue?","options":null}),
        };
        append_test_call(
            &mut ledger,
            "hold-attempt",
            &call.call_id,
            &call.name,
            call.arguments.clone(),
        );
        let mut no_replies = std::iter::empty();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        assert!(
            execute_provider_tool_calls(
                &mut ledger,
                ToolCallBatch {
                    options: &options,
                    profile: &profile,
                    selected: Selected { version: 2 },
                    attempt: "hold-attempt",
                    turn: 2,
                    calls: std::slice::from_ref(&call),
                    cancellation: &cancellation,
                },
                None,
                &mut no_replies,
                &mut output,
            )
            .expect("production hold path")
        );
        let cancel = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":2,"kind":"approval_response",
            "ts":options.timestamp,"call":"call-cancel","grant":false,
            "origin_key":"rpc-q/response","origin_tuple":{
                "principal":"p","client":"endpoint","target":"t","op":"respond","key":"rpc-q/response"
            }
        }))
        .expect("cancel event");
        ledger
            .append_contract(cancel, BarrierContext::default())
            .expect("durable cancellation");
        assert!(
            !recover_unpaired_tool_calls(
                &mut ledger,
                &options,
                Some(&profile),
                Selected { version: 2 },
                false,
                &mut no_replies,
                &mut output,
                &cancellation,
            )
            .expect("cancellation resumes the held call"),
            "a cancelled question leaves no unresolved hold behind"
        );
        let projection = ledger.projection().expect("projection");
        let results: Vec<Value> = projection
            .events
            .iter()
            .filter(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("call-cancel")
            })
            .map(|event| serde_json::to_value(event.raw()).expect("json"))
            .collect();
        assert_eq!(
            results.len(),
            1,
            "cancellation pairs the held call exactly once"
        );
        assert!(
            results[0]["outcome"].get("denied").is_some(),
            "the held call closes as denied, not aborted: {:?}",
            results[0]["outcome"]
        );
        assert!(results[0]["outcome"].get("aborted").is_none());
        assert!(
            !projection.lifecycle.stop_active,
            "cancelling a question never stops the session"
        );
        assert!(!cancellation.stop_requested());
        assert!(
            !projection
                .events
                .iter()
                .any(|event| *event.kind() == EventKind::StopRequested),
            "no stop is authored"
        );
    }

    #[test]
    fn slice8_gate_61_tool_hold_stop_resume() {
        let (directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let profile = tool_profile(directory.path(), &["ask_user_questions"]);
        let options = test_options(&ledger);
        append_test_attempt(&mut ledger, "hold-attempt");
        let call = provider::ToolCall {
            call_id: "call-hold".to_owned(),
            name: "ask_user_questions".to_owned(),
            arguments: json!({"question":"Continue?","options":null}),
        };
        append_test_call(
            &mut ledger,
            "hold-attempt",
            &call.call_id,
            &call.name,
            call.arguments.clone(),
        );
        let mut no_replies = std::iter::empty();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        assert!(
            execute_provider_tool_calls(
                &mut ledger,
                ToolCallBatch {
                    options: &options,
                    profile: &profile,
                    selected: Selected { version: 2 },
                    attempt: "hold-attempt",
                    turn: 2,
                    calls: std::slice::from_ref(&call),
                    cancellation: &cancellation,
                },
                None,
                &mut no_replies,
                &mut output,
            )
            .expect("production hold path")
        );
        assert!(
            recover_unpaired_tool_calls(
                &mut ledger,
                &options,
                Some(&profile),
                Selected { version: 2 },
                false,
                &mut no_replies,
                &mut output,
                &cancellation,
            )
            .expect("park remains durable")
        );

        let response = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":2,"kind":"approval_response",
            "ts":options.timestamp,"call":"call-hold","grant":true,"answer":"yes",
            "origin_key":"answer-1","origin_tuple":{
                "principal":"p","client":"cli","target":"t","op":"answer","key":"answer-1"
            }
        }))
        .expect("answer event");
        ledger
            .append_contract(response, BarrierContext::default())
            .expect("durable answer");
        assert!(
            !recover_unpaired_tool_calls(
                &mut ledger,
                &options,
                Some(&profile),
                Selected { version: 2 },
                false,
                &mut no_replies,
                &mut output,
                &cancellation,
            )
            .expect("resume hold")
        );
        assert_eq!(
            ledger
                .projection()
                .expect("projection")
                .events
                .iter()
                .filter(|event| {
                    *event.kind() == EventKind::ToolResult
                        && event.string_field("call") == Some("call-hold")
                })
                .count(),
            1,
            "answer resumes and pairs the held call once"
        );

        append_test_call(
            &mut ledger,
            "hold-attempt",
            "call-stop",
            "ask_user_questions",
            json!({"question":"Stop?","options":null}),
        );
        let stopped = provider::ToolCall {
            call_id: "call-stop".to_owned(),
            name: "ask_user_questions".to_owned(),
            arguments: json!({"question":"Stop?","options":null}),
        };
        assert!(
            execute_provider_tool_calls(
                &mut ledger,
                ToolCallBatch {
                    options: &options,
                    profile: &profile,
                    selected: Selected { version: 2 },
                    attempt: "hold-attempt",
                    turn: 2,
                    calls: std::slice::from_ref(&stopped),
                    cancellation: &cancellation,
                },
                None,
                &mut no_replies,
                &mut output,
            )
            .expect("second production hold")
        );
        assert!(
            !recover_unpaired_tool_calls(
                &mut ledger,
                &options,
                Some(&profile),
                Selected { version: 2 },
                true,
                &mut no_replies,
                &mut output,
                &cancellation,
            )
            .expect("stop/reconcile abort")
        );
        let stop_result = ledger
            .projection()
            .expect("projection")
            .events
            .iter()
            .find(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("call-stop")
            })
            .expect("stop result");
        let raw = serde_json::to_value(stop_result.raw()).expect("stop JSON");
        assert_eq!(raw.pointer("/outcome/aborted"), Some(&json!("recovered")));
    }

    #[test]
    fn production_task_creates_launches_waits_and_pairs_child_once() {
        let (directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let profile = tool_profile(directory.path(), &["task"]);
        let options = test_options(&ledger);
        append_test_attempt(&mut ledger, "task-attempt");
        let call = provider::ToolCall {
            call_id: "call-task".to_owned(),
            name: "task".to_owned(),
            arguments: json!({
                "task_name":"audit","goal":"find defects",
                "instructions":"inspect inputs","input_sources":[],
                "output":{"mode":"inline","format":"text","name":"result","path":"","contract":"findings"}
            }),
        };
        append_test_call(
            &mut ledger,
            "task-attempt",
            &call.call_id,
            &call.name,
            call.arguments.clone(),
        );
        let approval = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":2,"kind":"approval_request",
            "ts":options.timestamp,"call":call.call_id,"scope":"destructive",
            "question":{"tool":"task"}
        }))
        .expect("approval request");
        ledger
            .append_contract(approval, BarrierContext::default())
            .expect("approval request barrier");
        let answer = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":2,"kind":"approval_response",
            "ts":options.timestamp,"call":call.call_id,"grant":true,
            "origin_key":"approve-task","origin_tuple":{
                "principal":"p","client":"cli","target":"t",
                "op":"answer","key":"approve-task"
            }
        }))
        .expect("approval response");
        ledger
            .append_contract(answer, BarrierContext::default())
            .expect("approval response barrier");

        // The durable delegation-state record precedes the spawn barrier.
        let spawn_seq = ledger.next_seq() + 2;
        let (child, spawn_id) = child_identity(
            "018f0000-0000-7000-8000-000000000003",
            &call.call_id,
            spawn_seq,
        );
        let request = ToolControl::new(
            "018f0000-0000-7000-8000-000000000003",
            "018f0000-0000-7000-8000-000000000003",
            2,
            call.call_id.clone(),
            call.name.clone(),
            IJsonValue::parse(&serde_json::to_vec(&call.arguments).unwrap()).unwrap(),
        )
        .expect("tool control request");
        let control = ToolControlResult::success(
            request.request_id,
            call.call_id.clone(),
            IJsonValue::parse(
                &serde_json::to_vec(&json!({"child":child,"spawn_id":spawn_id})).unwrap(),
            )
            .unwrap(),
        );
        let launch = LaunchResult {
            child: child.clone(),
            spawn_id: spawn_id.clone(),
            ok: true,
            error: None,
        };
        let mut replies = [
            Ok(
                String::from_utf8(worker_control::encode_tool_control_result(&control).unwrap())
                    .unwrap(),
            ),
            Ok(String::from_utf8(encode_line("launch_result", &launch).unwrap()).unwrap()),
        ]
        .into_iter();

        let folder = ledger.path().parent().unwrap().to_path_buf();
        let child_file = folder.join(format!("{child}.jsonl"));
        let child_writer = std::thread::spawn(move || {
            for _ in 0..400 {
                if child_file.exists() {
                    let mut child_ledger = match LockedLedger::open(&child_file, 1) {
                        Ok(ledger) => ledger,
                        Err(StoreError::Busy) => {
                            std::thread::sleep(Duration::from_millis(5));
                            continue;
                        }
                        Err(error) => panic!("child open failed: {error}"),
                    };
                    let system = AssetStore::new(child_file.parent().unwrap().join("assets"))
                        .unwrap()
                        .publish(b"{}")
                        .unwrap();
                    for value in [
                        json!({"v":1,"seq":2,"turn":1,"kind":"turn_open","ts":"2026-08-27T09:00:04.000Z","trigger":"genesis"}),
                        json!({"v":1,"seq":3,"kind":"epoch","ts":"2026-08-27T09:00:04.000Z","id":"child-epoch","reason":"initial","adapter":"anthropic_messages_v1","model":"m","system":{"asset":system.asset,"digest":"d"},"tools":{"asset":"sha256-td","digest":"td"},"renderer":1}),
                        json!({"v":1,"seq":4,"turn":1,"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"ts":"2026-08-27T09:00:04.000Z","attempt":"child-attempt","epoch":"child-epoch","wire_digest":"w","admits":[]}),
                        json!({"v":1,"seq":5,"turn":1,"kind":"tool_call","ts":"2026-08-27T09:00:04.000Z","attempt":"child-attempt","call":"report-call","name":"report","args":{"result":"found one defect"},"source":"provider"}),
                        json!({"v":1,"seq":6,"turn":1,"kind":"tool_call","ts":"2026-08-27T09:00:04.000Z","attempt":"child-attempt","call":"write-call","name":"apply_patch","args":{"operation":"create_file","path":"delegated.txt","diff":"+delivered","expected_artifact_version":0,"summary":"write"},"source":"provider"}),
                        json!({"v":1,"seq":7,"turn":1,"kind":"output","ts":"2026-08-27T09:00:04.000Z","attempt":"child-attempt","content":[],"sealed":{"version":1,"adapter":"anthropic_messages_v1","fragments":"[]"},"usage":{"availability":"unavailable"}}),
                        json!({"v":1,"seq":8,"turn":1,"kind":"tool_result","ts":"2026-08-27T09:00:04.000Z","call":"report-call","outcome":"ok","content":[{"type":"text","text":"delivered"}]}),
                        json!({"v":1,"seq":9,"turn":1,"kind":"tool_result","ts":"2026-08-27T09:00:04.000Z","call":"write-call","outcome":"ok","content":[{"type":"text","text":json!({"path":"delegated.txt","sha256":"child-sha","artifact_version":1}).to_string()}]}),
                        json!({"v":1,"seq":10,"turn":1,"kind":"settle","ts":"2026-08-27T09:00:04.000Z","outcome":"completed"}),
                    ] {
                        child_ledger
                            .append_contract(make_event(value).unwrap(), BarrierContext::default())
                            .unwrap();
                    }
                    return;
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            panic!("child genesis was never published");
        });

        let cancellation = RuntimeCancellation::default();
        let mut output = Vec::new();
        assert!(
            !execute_provider_tool_calls(
                &mut ledger,
                ToolCallBatch {
                    options: &options,
                    profile: &profile,
                    selected: Selected { version: 2 },
                    attempt: "task-attempt",
                    turn: 2,
                    calls: std::slice::from_ref(&call),
                    cancellation: &cancellation,
                },
                None,
                &mut replies,
                &mut output,
            )
            .expect("task spawn transaction")
        );
        child_writer.join().unwrap();
        let snapshot =
            validation_runtime::snapshot(&ledger, 2, &profile).expect("joined child artifacts");
        let artifact_path = PathBuf::from(&profile.config.workspace.cwd[0])
            .join("delegated.txt")
            .to_string_lossy()
            .into_owned();
        assert_eq!(
            snapshot,
            std::collections::BTreeMap::from([(artifact_path, "child-sha".to_owned())])
        );
        // A similarly named ledger with a different parent binding cannot supply artifacts.
        let child_path = ledger
            .path()
            .parent()
            .unwrap()
            .join(format!("{child}.jsonl"));
        let original = fs::read(&child_path).unwrap();
        let mut records: Vec<Value> = original
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| serde_json::from_slice(line).unwrap())
            .collect();
        records[0]["parent"]["spawn_id"] = json!("foreign-spawn");
        let mut forged = Vec::new();
        for record in records {
            forged.extend(
                IJsonValue::parse(&serde_json::to_vec(&record).unwrap())
                    .unwrap()
                    .canonical_bytes()
                    .unwrap(),
            );
            forged.push(b'\n');
        }
        fs::write(&child_path, forged).unwrap();
        assert!(validation_runtime::snapshot(&ledger, 2, &profile).is_err());
        fs::write(&child_path, original).unwrap();

        let projection = ledger.projection().unwrap();
        assert_eq!(
            projection
                .events
                .iter()
                .filter(|event| event.kind() == &EventKind::Spawn)
                .count(),
            1
        );
        let delegation = projection
            .events
            .iter()
            .find(|event| {
                event.kind() == &EventKind::State
                    && event.string_field("subkind") == Some("delegation")
            })
            .expect("durable delegation state");
        let delegation_raw = serde_json::to_value(delegation.raw()).unwrap();
        assert_eq!(
            delegation_raw.pointer("/payload/invocation"),
            Some(&call.arguments)
        );
        let result = projection
            .events
            .iter()
            .find(|event| event.kind() == &EventKind::ChildResult)
            .expect("child result");
        let raw = serde_json::to_value(result.raw()).unwrap();
        assert_eq!(raw["outcome"], "completed");
        assert_eq!(raw["summary"], "found one defect");
        assert_eq!(
            projection
                .events
                .iter()
                .filter(|event| event.kind() == &EventKind::ToolResult)
                .filter(|event| event.string_field("call") == Some("call-task"))
                .count(),
            1
        );
        let spawn_event = projection
            .events
            .iter()
            .find(|event| event.kind() == &EventKind::Spawn)
            .expect("spawn");
        let spawn_raw = serde_json::to_value(spawn_event.raw()).unwrap();
        let durable_spawn = DurableChildSpawn {
            child: child.clone(),
            child_file: spawn_event.string_field("child").unwrap().to_owned(),
            spawn_id: spawn_event.string_field("spawn_id").unwrap().to_owned(),
            resume: serde_json::from_value(spawn_raw["resume"].clone()).unwrap(),
        };
        let terminal = ChildTerminal {
            outcome: "completed".to_owned(),
            summary: Some("found one defect".to_owned()),
        };
        let _ = projection;
        append_child_result_once(&mut ledger, &options, 2, &call, &durable_spawn, &terminal)
            .expect("retry child_result is idempotent");
        assert_eq!(
            ledger
                .projection()
                .unwrap()
                .events
                .iter()
                .filter(|event| event.kind() == &EventKind::ChildResult)
                .count(),
            1
        );
        let context = project_provider_context(&ledger, "e1", DialectId::AnthropicMessagesV1, 2)
            .expect("provider projection after child completion");
        let call_roles = context
            .items
            .iter()
            .filter_map(|item| serde_json::to_value(item).ok())
            .filter(|item| {
                item.get("content")
                    .and_then(Value::as_array)
                    .is_some_and(|content| {
                        content.iter().any(|block| {
                            block.get("call_id").and_then(Value::as_str) == Some("call-task")
                        })
                    })
            })
            .filter_map(|item| item.get("role").and_then(Value::as_str).map(str::to_owned))
            .collect::<Vec<_>>();
        assert_eq!(
            call_roles,
            ["assistant", "tool"],
            "child_result is the sole provider-visible result for a spawned call"
        );
        assert!(String::from_utf8(output).unwrap().contains("launch_child"));

        let consumer = provider::ToolCall {
            call_id: "consume-audit".into(),
            name: "task".into(),
            arguments: json!({"task_name":"consumer","input_sources":["audit","direct/path.md"],
                "output":{"mode":"file","path":"consumer.md"}}),
        };
        append_test_call(
            &mut ledger,
            "task-attempt",
            &consumer.call_id,
            &consumer.name,
            consumer.arguments.clone(),
        );
        let effective = ijson(consumer.arguments.clone()).unwrap();
        let seed = ensure_delegation_state(&mut ledger, &options, &consumer, &effective).unwrap();
        let raw = serde_json::to_value(seed.raw()).unwrap();
        let inputs = raw["payload"]["resolved_inputs"].as_array().unwrap();
        assert_eq!(
            inputs.len(),
            1,
            "direct paths must not be resolved as task names"
        );
        assert_eq!(inputs[0]["source"], "audit");
        assert_eq!(inputs[0]["summary"], "found one defect");
        assert_eq!(inputs[0]["output"]["mode"], "inline");
        assert!(inputs[0]["completion_seq"].as_u64().unwrap() < seed.seq());
        assert_eq!(
            raw["payload"]["invocation"], consumer.arguments,
            "effective arguments must stay intact"
        );
        assert_eq!(
            ensure_delegation_state(&mut ledger, &options, &consumer, &effective)
                .unwrap()
                .seq(),
            seed.seq()
        );
        append_test_call(
            &mut ledger,
            "task-attempt",
            "unfinished-consumer",
            "task",
            json!({}),
        );
        assert!(
            resolve_completed_task_inputs(
                &ledger,
                &json!({"input_sources":["consumer"]}),
                "unfinished-consumer"
            )
            .is_err(),
            "an unfinished upstream must not become a usable input"
        );
        let replacement = provider::ToolCall {
            call_id: "replace-audit".into(),
            name: "task".into(),
            arguments: json!({"task_name":"audit","input_sources":[],"output":{"mode":"file","path":"new-audit.md"}}),
        };
        append_test_call(
            &mut ledger,
            "task-attempt",
            &replacement.call_id,
            &replacement.name,
            replacement.arguments.clone(),
        );
        ensure_delegation_state(
            &mut ledger,
            &options,
            &replacement,
            &ijson(replacement.arguments.clone()).unwrap(),
        )
        .unwrap();
        append_test_call(
            &mut ledger,
            "task-attempt",
            "after-replacement",
            "task",
            json!({}),
        );
        let error = resolve_completed_task_inputs(
            &ledger,
            &json!({"input_sources":["audit"]}),
            "after-replacement",
        )
        .unwrap_err();
        assert!(
            error.downcast_ref::<TaskInputError>().is_some(),
            "invalid task inputs must be recoverable tool errors"
        );
        assert!(
            error.to_string().contains("not completed"),
            "must not fall back to the older completed audit task"
        );
        let replacement_spawn = ensure_durable_child(
            &mut ledger,
            &options,
            &profile,
            &replacement,
            &ijson(replacement.arguments.clone()).unwrap(),
        )
        .unwrap();
        append_child_result_once(
            &mut ledger,
            &options,
            2,
            &replacement,
            &replacement_spawn,
            &ChildTerminal {
                outcome: "completed".into(),
                summary: Some("new findings".into()),
            },
        )
        .unwrap();
        append_test_call(
            &mut ledger,
            "task-attempt",
            "after-new-completion",
            "task",
            json!({}),
        );
        let newest = resolve_completed_task_inputs(
            &ledger,
            &json!({"input_sources":["audit"]}),
            "after-new-completion",
        )
        .unwrap();
        assert_eq!(newest[0]["path"], "new-audit.md");
        assert_eq!(newest[0]["summary"], "new findings");
        assert_eq!(newest[0]["output"]["mode"], "file");
    }

    #[test]
    fn delegation_seed_reuses_the_post_hook_effective_invocation() {
        let (_directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let options = test_options(&ledger);
        append_test_attempt(&mut ledger, "task-attempt");
        let call = provider::ToolCall {
            call_id: "call-task-mutated".to_owned(),
            name: "task".to_owned(),
            arguments: json!({"goal":"provider value"}),
        };
        append_test_call(
            &mut ledger,
            "task-attempt",
            &call.call_id,
            &call.name,
            call.arguments.clone(),
        );
        let effective = ijson(json!({"goal":"approved effective value"})).unwrap();
        let first = ensure_delegation_state(&mut ledger, &options, &call, &effective)
            .expect("first delegation state");
        let second = ensure_delegation_state(&mut ledger, &options, &call, &effective)
            .expect("retry delegation state");
        assert_eq!(first.seq(), second.seq());
        let raw = serde_json::to_value(first.raw()).unwrap();
        assert_eq!(
            raw.pointer("/payload/invocation"),
            Some(&json!({"goal":"approved effective value"}))
        );
        assert_eq!(
            ledger
                .projection()
                .unwrap()
                .events
                .iter()
                .filter(|event| {
                    event.kind() == &EventKind::State
                        && event.string_field("subkind") == Some("delegation")
                })
                .count(),
            1
        );
        assert!(
            ensure_delegation_state(
                &mut ledger,
                &options,
                &call,
                &ijson(json!({"goal":"different retry"})).unwrap(),
            )
            .is_err(),
            "retry with changed effective bytes must fail closed"
        );
    }

    #[test]
    fn supervisor_control_recovery_reuses_the_durable_request_identity() {
        let (directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let profile = tool_profile(directory.path(), &["context"]);
        let options = test_options(&ledger);
        append_test_attempt(&mut ledger, "control-attempt");
        let arguments = json!({"operation":"threads","reason":"recover receipt"});
        append_test_call(
            &mut ledger,
            "control-attempt",
            "call-control",
            "context",
            arguments.clone(),
        );
        let session = "018f0000-0000-7000-8000-000000000003";
        let request = ToolControl::new(
            session,
            session,
            2,
            "call-control",
            "context",
            ijson(arguments).expect("arguments"),
        )
        .expect("stable request");
        let response = ToolControlResult::success(
            request.request_id.clone(),
            request.call_id.clone(),
            ijson(json!({"threads":[]})).expect("response value"),
        );
        let mut replies = [Ok(String::from_utf8(
            worker_control::encode_tool_control_result(&response).expect("response wire"),
        )
        .expect("UTF-8"))]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        assert!(
            !recover_unpaired_tool_calls(
                &mut ledger,
                &options,
                Some(&profile),
                Selected { version: 2 },
                false,
                &mut replies,
                &mut output,
                &cancellation,
            )
            .expect("receipt recovery")
        );
        let replayed = worker_control::decode_tool_control(&output).expect("request wire");
        assert_eq!(
            replayed, request,
            "recovery must query the identical receipt id"
        );
        assert_eq!(
            ledger
                .projection()
                .expect("projection")
                .events
                .iter()
                .filter(|event| {
                    *event.kind() == EventKind::ToolResult
                        && event.string_field("call") == Some("call-control")
                })
                .count(),
            1
        );
    }

    #[test]
    fn deferred_control_frames_remain_fifo_when_stop_cancels_runtime() {
        let cancellation = RuntimeCancellation::default();
        cancellation.defer("first".to_owned());
        cancellation.cancel(CANCEL_STOP);
        cancellation.defer("stop".to_owned());
        let (_sender, receiver) = sync_channel(0);
        let mut lines = ControlLines {
            receiver,
            cancellation: cancellation.clone(),
        };
        assert!(cancellation.tools.is_cancelled());
        assert_eq!(lines.next().unwrap().unwrap(), "first");
        assert_eq!(lines.next().unwrap().unwrap(), "stop");
    }

    #[test]
    fn ping_then_eof_cancels_a_blocked_backend_without_consuming_ping() {
        let cancellation = RuntimeCancellation::default();
        let backend_cancellation = cancellation.tools.clone();
        let blocked_backend = std::thread::spawn(move || {
            while !backend_cancellation.is_cancelled() {
                std::thread::yield_now();
            }
        });
        let (sender, receiver) = sync_channel(1);
        forward_control_lines(
            [Ok("{\"ping\":{\"id\":\"p1\"}}".to_owned())].into_iter(),
            sender,
            cancellation.clone(),
        );
        assert!(
            cancellation.tools.is_cancelled(),
            "EOF must reach the shared backend cancellation token"
        );
        assert!(cancellation.supervisor_lost());
        blocked_backend.join().expect("blocked backend cancelled");
        assert_eq!(
            receiver.recv().expect("queued ping").expect("valid ping"),
            "{\"ping\":{\"id\":\"p1\"}}",
            "the cancellation signal must not consume or reorder the prior frame"
        );
    }

    #[test]
    fn runtime_control_eof_and_mismatched_result_are_protocol_failures() {
        let request = ToolControl::new(
            "018f0000-0000-7000-8000-000000000003",
            "018f0000-0000-7000-8000-000000000003",
            2,
            "call-control",
            "context",
            IJsonValue::from("arguments"),
        )
        .expect("request");

        let cancellation = RuntimeCancellation::default();
        let mut eof = std::iter::empty();
        let error = exchange_tool_control_runtime(
            &Selected { version: 2 },
            &request,
            &mut eof,
            &mut Vec::new(),
            &cancellation,
        )
        .expect_err("EOF must fail");
        assert!(is_protocol_failure(error.as_ref()));
        assert!(cancellation.protocol_failed());

        let cancellation = RuntimeCancellation::default();
        let mismatched = ToolControlResult::success(
            request.request_id.clone(),
            "different-call",
            IJsonValue::from(true),
        );
        let mut replies = [Ok(String::from_utf8(
            worker_control::encode_tool_control_result(&mismatched).expect("mismatch wire"),
        )
        .expect("UTF-8"))]
        .into_iter();
        let error = exchange_tool_control_runtime(
            &Selected { version: 2 },
            &request,
            &mut replies,
            &mut Vec::new(),
            &cancellation,
        )
        .expect_err("mismatch must fail");
        assert!(is_protocol_failure(error.as_ref()));
        assert!(cancellation.protocol_failed());
    }

    #[test]
    fn production_provider_ignores_tool_network_policy_and_continues_after_tool_result() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        let endpoint = format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        );
        set_provider_test_redirect(Some(endpoint));
        let server = std::thread::spawn(move || {
            for response in [
                br#"{"id":"response-tool","output":[{"arguments":"{\"thought\":\"inspect\"}","call_id":"call-think","name":"think","type":"function_call"}],"status":"completed","usage":{"input_tokens":2,"output_tokens":1}}"#.as_slice(),
                br#"{"id":"response-done","output":[{"content":[{"text":"done","type":"output_text"}],"type":"message"}],"status":"completed","usage":{"input_tokens":2,"output_tokens":1}}"#.as_slice(),
            ] {
                let (mut stream, _) = listener.accept().expect("provider accept");
                let mut request = Vec::new();
                let mut chunk = [0_u8; 4096];
                let mut expected = None;
                loop {
                    let count = stream.read(&mut chunk).expect("provider request");
                    request.extend_from_slice(&chunk[..count]);
                    if expected.is_none() {
                        if let Some(end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                            let headers = String::from_utf8_lossy(&request[..end + 4]);
                            let length = headers.lines().find_map(|line| {
                                line.strip_prefix("content-length: ")
                                    .or_else(|| line.strip_prefix("Content-Length: "))
                            }).and_then(|value| value.parse::<usize>().ok()).unwrap_or(0);
                            expected = Some(end + 4 + length);
                        }
                    }
                    if count == 0 || expected.is_some_and(|length| request.len() >= length) { break; }
                }
                write!(stream, "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", response.len()).expect("provider header");
                stream.write_all(response).expect("provider body");
            }
        });
        let mut profile = tool_profile(directory.path(), &["think"]);
        profile.config.workspace.policy.network = false;
        profile.config.workspace.policy.provider = Some("fixture".to_owned());
        profile.config.workspace.policy.model = Some("gpt-5".to_owned());
        profile.config.providers = ProvidersConfig {
            format: 1,
            revision: 1,
            providers: vec![Provider {
                id: "fixture".to_owned(),
                name: None,
                adapter: "responses".to_owned(),
                dialect: "openai_responses_v1".to_owned(),
                endpoint_owner: "openai".to_owned(),
                gateway_translation: "direct".to_owned(),
                evidence_revision: "openai-2026-08-01".to_owned(),
                endpoint: "https://api.openai.com/v1".to_owned(),
                credential_key: Some("provider-main".to_owned()),
                models: vec![Model {
                    id: "gpt-5".to_owned(),
                    profile: "openai_responses_v1:gpt-5".to_owned(),
                    enabled: true,
                    context_window_tokens: 100_000,
                    compact_trigger_tokens: 100_000,
                }],
            }],
            web_search: None,
        };
        profile.config.revisions.providers = 1;
        let (supervisor, worker) = std::os::unix::net::UnixStream::pair().expect("socket pair");
        let (control, service) = provider::start_credential_channel(
            supervisor,
            provider::CredentialBroker::new([provider::CredentialScope {
                credential_id: "provider-main".to_owned(),
                adapter: "responses".to_owned(),
                endpoint_origin: "https://api.openai.com:443".to_owned(),
                purpose: "provider".to_owned(),
                generation: "1".to_owned(),
                material: "fixture-secret".to_owned(),
            }]),
        );
        let mut credential = Some(Arc::new(Mutex::new(CredentialClient::new(worker))));
        let options = test_options(&ledger);
        let mut replies = [
            Ok("{\"lease\":{\"attempt\":\"tool-run-attempt-13\",\"granted\":true}}".to_owned()),
            Ok("{\"lease\":{\"attempt\":\"tool-run-attempt-19\",\"granted\":true}}".to_owned()),
        ]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut credential,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        result.expect("provider tool continuation");
        server.join().expect("provider server");
        drop(credential);
        drop(control);
        service.join().expect("broker thread").expect("broker EOF");
        let events = &ledger.projection().expect("projection").events;
        let call_seq = events
            .iter()
            .find(|event| {
                *event.kind() == EventKind::ToolCall
                    && event.string_field("call") == Some("call-think")
            })
            .expect("tool call")
            .seq();
        let result_seq = events
            .iter()
            .find(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("call-think")
            })
            .expect("tool result")
            .seq();
        let next_attempt = events
            .iter()
            .find(|event| *event.kind() == EventKind::Attempt && event.seq() > result_seq)
            .expect("next attempt")
            .seq();
        assert!(call_seq < result_seq && result_seq < next_attempt);
    }

    struct RealMemoryFixture {
        child: std::process::Child,
        output: std::io::BufReader<std::process::ChildStdout>,
        hook_root: PathBuf,
    }

    impl RealMemoryFixture {
        fn start(root: &std::path::Path) -> Self {
            let fixture = env::var("TEKES_MEMORY_FIXTURE_BIN")
                .expect("absolute TEKES_MEMORY_FIXTURE_BIN required");
            let binary = env::var("TEKES_MEMORY_BIN").expect("absolute TEKES_MEMORY_BIN required");
            assert!(PathBuf::from(&fixture).is_absolute() && PathBuf::from(&binary).is_absolute());
            let mut child = std::process::Command::new(fixture)
                .arg(root)
                .arg(binary)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("start real Memory fixture");
            let mut output = std::io::BufReader::new(child.stdout.take().unwrap());
            let mut line = String::new();
            output.read_line(&mut line).expect("fixture ready line");
            let ready: Value = serde_json::from_str(&line).expect("fixture initialized");
            Self {
                child,
                output,
                hook_root: PathBuf::from(ready["hook_root"].as_str().unwrap()),
            }
        }

        fn verify(&mut self) {
            writeln!(self.child.stdin.as_mut().unwrap(), "verify").unwrap();
            let mut line = String::new();
            self.output.read_line(&mut line).unwrap();
            let result: Value = serde_json::from_str(&line).expect("Memory durable observations");
            assert_eq!(result["verified"], true);
            assert!(self.child.wait().unwrap().success());
        }
    }

    impl Drop for RealMemoryFixture {
        fn drop(&mut self) {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }

    #[test]
    #[ignore = "requires Rust TekesMemory binaries; TEKES_MEMORY_BIN / TEKES_MEMORY_FIXTURE_BIN"]
    fn production_turn_with_real_tekesmemory_service() {
        assert_production_turn_continuation(true);
    }

    #[test]
    fn production_turn_continues_through_tools_commentary_and_reasoning_until_final() {
        assert_production_turn_continuation(false);
    }

    fn assert_production_turn_continuation(real_memory_enabled: bool) {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        let endpoint = format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        );
        set_provider_test_redirect(Some(endpoint));
        let server = std::thread::spawn(move || {
            for response in [
                br#"{"id":"response-tool","output":[{"arguments":"{\"thought\":\"inspect\"}","call_id":"call-think","name":"think","type":"function_call"}],"status":"completed","usage":{"input_tokens":2,"output_tokens":1}}"#.as_slice(),
                br#"{"id":"response-commentary","output":[{"content":[{"text":"Continuing the work","type":"output_text"}],"phase":"commentary","status":"completed","type":"message"}],"status":"completed"}"#.as_slice(),
                br#"{"id":"response-reasoning","output":[{"summary":[{"text":"Still thinking","type":"summary_text"}],"type":"reasoning"}],"status":"completed"}"#.as_slice(),
                br#"{"id":"response-done","output":[{"content":[{"text":"done","type":"output_text"}],"phase":"final_answer","status":"completed","type":"message"}],"status":"completed","usage":{"input_tokens":2,"output_tokens":1}}"#.as_slice(),
            ] {
                let (mut stream, _) = listener.accept().expect("provider accept");
                let mut request = Vec::new();
                let mut chunk = [0_u8; 4096];
                let mut expected = None;
                loop {
                    let count = stream.read(&mut chunk).expect("provider request");
                    request.extend_from_slice(&chunk[..count]);
                    if expected.is_none() {
                        if let Some(end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                            let headers = String::from_utf8_lossy(&request[..end + 4]);
                            let length = headers.lines().find_map(|line| {
                                line.strip_prefix("content-length: ")
                                    .or_else(|| line.strip_prefix("Content-Length: "))
                            }).and_then(|value| value.parse::<usize>().ok()).unwrap_or(0);
                            expected = Some(end + 4 + length);
                        }
                    }
                    if count == 0 || expected.is_some_and(|length| request.len() >= length) { break; }
                }
                assert!(String::from_utf8_lossy(&request).contains("LIFECYCLE_CONTEXT_MARKER"));
                write!(stream, "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", response.len()).expect("provider header");
                stream.write_all(response).expect("provider body");
            }
        });
        let mut profile = tool_profile(directory.path(), &["think"]);
        profile.config.workspace.policy.network = false;
        profile.config.workspace.policy.provider = Some("fixture".to_owned());
        profile.config.workspace.policy.model = Some("gpt-5".to_owned());
        profile.config.providers = ProvidersConfig {
            format: 1,
            revision: 1,
            providers: vec![Provider {
                id: "fixture".to_owned(),
                name: None,
                adapter: "responses".to_owned(),
                dialect: "openai_responses_v1".to_owned(),
                endpoint_owner: "openai".to_owned(),
                gateway_translation: "direct".to_owned(),
                evidence_revision: "openai-2026-08-01".to_owned(),
                endpoint: "https://api.openai.com/v1".to_owned(),
                credential_key: Some("provider-main".to_owned()),
                models: vec![Model {
                    id: "gpt-5".to_owned(),
                    profile: "openai_responses_v1:gpt-5".to_owned(),
                    enabled: true,
                    context_window_tokens: 100_000,
                    compact_trigger_tokens: 100_000,
                }],
            }],
            web_search: None,
        };
        profile.config.revisions.providers = 1;
        let (supervisor, worker) = std::os::unix::net::UnixStream::pair().expect("socket pair");
        let (control, service) = provider::start_credential_channel(
            supervisor,
            provider::CredentialBroker::new([provider::CredentialScope {
                credential_id: "provider-main".to_owned(),
                adapter: "responses".to_owned(),
                endpoint_origin: "https://api.openai.com:443".to_owned(),
                purpose: "provider".to_owned(),
                generation: "1".to_owned(),
                material: "fixture-secret".to_owned(),
            }]),
        );
        let mut credential = Some(Arc::new(Mutex::new(CredentialClient::new(worker))));
        let mut real_memory =
            real_memory_enabled.then(|| RealMemoryFixture::start(directory.path()));
        let log = if let Some(memory) = &real_memory {
            profile.instruction = InstructionResolver::new(&memory.hook_root, [directory.path()])
                .capture()
                .unwrap();
            None
        } else {
            Some(install_lifecycle_test_hooks(directory.path(), &mut profile))
        };
        let mut options = test_options(&ledger);
        options.lifecycle_hooks = lifecycle_hooks::LifecycleHooks::load(&profile, &ledger).unwrap();
        let mut replies = [13, 19, 22, 26]
            .map(|seq| {
                Ok(format!(
                    "{{\"lease\":{{\"attempt\":\"tool-run-attempt-{seq}\",\"granted\":true}}}}"
                ))
            })
            .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut credential,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        result.expect("provider tool continuation");
        server.join().expect("provider server");
        options
            .lifecycle_hooks
            .as_ref()
            .unwrap()
            .observe(&mut ledger);
        if let Some(log) = log {
            let observed: Vec<Value> = fs::read_to_string(log)
                .unwrap()
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect();
            assert_eq!(
                observed
                    .iter()
                    .filter(|r| r["event"] == "context.prepare")
                    .count(),
                4
            );
            assert_eq!(
                observed
                    .iter()
                    .filter(|r| r["event"] == "turn.before")
                    .count(),
                1
            );
            assert_eq!(
                observed
                    .iter()
                    .filter(|r| r["event"] == "tool.completed")
                    .count(),
                1
            );
            assert_eq!(observed.last().unwrap()["event"], "turn.settled");
        }
        if let Some(memory) = &mut real_memory {
            memory.verify();
        }
        drop(credential);
        drop(control);
        service.join().expect("broker thread").expect("broker EOF");
        let events = &ledger.projection().expect("projection").events;
        let call_seq = events
            .iter()
            .find(|event| {
                *event.kind() == EventKind::ToolCall
                    && event.string_field("call") == Some("call-think")
            })
            .expect("tool call")
            .seq();
        let result_seq = events
            .iter()
            .find(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("call-think")
            })
            .expect("tool result")
            .seq();
        let next_attempt = events
            .iter()
            .find(|event| *event.kind() == EventKind::Attempt && event.seq() > result_seq)
            .expect("next attempt")
            .seq();
        assert!(call_seq < result_seq && result_seq < next_attempt);
        let turn_events = events
            .iter()
            .filter(|event| event.turn() == Some(2))
            .collect::<Vec<_>>();
        assert_eq!(
            turn_events
                .iter()
                .filter(|event| *event.kind() == EventKind::Attempt)
                .count(),
            4
        );
        let settles = turn_events
            .iter()
            .filter(|event| *event.kind() == EventKind::Settle)
            .collect::<Vec<_>>();
        assert_eq!(settles.len(), 1);
        assert_eq!(settles[0].string_field("outcome"), Some("completed"));
        let outputs = turn_events
            .iter()
            .filter(|event| *event.kind() == EventKind::Output)
            .map(|event| serde_json::to_value(event.raw()).unwrap()["final_answer"].clone())
            .collect::<Vec<_>>();
        assert_eq!(
            outputs,
            vec![json!(false), json!(false), json!(false), json!(true)]
        );
    }

    #[test]
    fn tool_control_exchange_is_correlated() {
        let request = ToolControl::new(
            "018f0000-0000-7000-8000-000000000001",
            "018f0000-0000-7000-8000-000000000001",
            1,
            "call-1",
            "context",
            IJsonValue::from("arguments"),
        )
        .expect("request");
        let response = ToolControlResult::success(
            request.request_id.clone(),
            request.call_id.clone(),
            IJsonValue::from(true),
        );
        let mut replies = [Ok(String::from_utf8(
            worker_control::encode_tool_control_result(&response).expect("response wire"),
        )
        .expect("UTF-8"))]
        .into_iter();
        let mut output = Vec::new();
        let actual = exchange_tool_control(
            &Selected { version: 2 },
            &request,
            &mut replies,
            &mut output,
        )
        .expect("exchange");
        assert_eq!(actual, response);
        assert_eq!(
            output,
            worker_control::encode_tool_control(&request).expect("request wire")
        );
    }

    #[test]
    fn epoch_compatibility_cannot_resurrect_an_older_continuation_chain() {
        let (_directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let dialect = DialectId::OpenaiResponsesV1;
        assert_eq!(
            latest_compatible_epoch(&ledger, dialect, "m", "d", "td", 1).as_deref(),
            Some("e1")
        );
        let mut next = serde_json::to_value(
            ledger
                .projection()
                .unwrap()
                .events
                .iter()
                .find(|e| e.kind() == &EventKind::Epoch)
                .unwrap()
                .raw(),
        )
        .unwrap();
        next["seq"] = json!(ledger.next_seq());
        next["id"] = json!("e2");
        next["tools"] = json!({"asset":"sha256-activated-tools","digest":"activated-tools"});
        ledger
            .append_contract(make_event(next).unwrap(), BarrierContext::default())
            .unwrap();
        assert!(latest_compatible_epoch(&ledger, dialect, "m", "d", "td", 1).is_none());
        assert_eq!(
            latest_compatible_epoch(&ledger, dialect, "m", "d", "activated-tools", 1).as_deref(),
            Some("e2")
        );
    }

    fn context_ledger(adapter: &str, continuation: bool) -> (tempfile::TempDir, LockedLedger) {
        let directory = tempfile::tempdir().expect("thread root");
        let folder = directory
            .path()
            .join("018f0000-0000-7000-8000-000000000003");
        fs::create_dir(&folder).expect("thread folder");
        let profile_asset = store::AssetStore::new(folder.join("assets"))
            .expect("asset store")
            .publish(b"{}")
            .expect("profile asset");
        let continuation = continuation
            .then_some(",\"continuation\":{\"id\":\"response-1\"}")
            .unwrap_or_default();
        let fragments = if adapter == "deepseek_responses_v1" {
            r#"[{\"content\":[{\"text\":\"answer\",\"type\":\"output_text\"}],\"role\":\"assistant\",\"type\":\"message\"}]"#
        } else {
            "[]"
        };
        let bytes = concat!(
                "{\"config\":{\"digest\":\"cfg\"},\"format\":1,\"kind\":\"genesis\",\"min_reader\":1,\"min_writer\":1,\"origin_key\":\"create\",\"origin_tuple\":{\"client\":\"cli\",\"key\":\"create\",\"op\":\"create\",\"principal\":\"p\",\"target\":\"018f0000-0000-7000-8000-000000000003\"},\"resume\":\"never\",\"seq\":1,\"thread\":\"018f0000-0000-7000-8000-000000000003\",\"ts\":\"2026-08-27T09:00:00.000Z\",\"v\":1,\"workspace\":\"ws\"}\n",
                "{\"content\":[{\"text\":\"first\",\"type\":\"text\"}],\"kind\":\"input\",\"origin_key\":\"i1\",\"origin_tuple\":{\"client\":\"cli\",\"key\":\"i1\",\"op\":\"submit\",\"principal\":\"p\",\"target\":\"t\"},\"seq\":2,\"ts\":\"2026-08-27T09:00:00.000Z\",\"v\":1}\n",
                "{\"binary\":\"worker\",\"config_digest\":\"cfg\",\"instruction_digest\":\"ins\",\"kind\":\"run_start\",\"mode\":\"ordinary\",\"policy\":\"p\",\"recovery_ordinal\":0,\"run\":\"r1\",\"seq\":3,\"ts\":\"2026-08-27T09:00:00.000Z\",\"v\":1}\n",
                "{\"kind\":\"turn_open\",\"seq\":4,\"trigger\":{\"inputs\":[2]},\"ts\":\"2026-08-27T09:00:00.000Z\",\"turn\":1,\"v\":1}\n",
                "{\"adapter\":\"__ADAPTER__\",\"id\":\"e1\",\"kind\":\"epoch\",\"model\":\"m\",\"reason\":\"initial\",\"renderer\":1,\"seq\":5,\"system\":{\"asset\":\"__PROFILE_ASSET__\",\"digest\":\"d\"},\"tools\":{\"asset\":\"sha256-td\",\"digest\":\"td\"},\"ts\":\"2026-08-27T09:00:00.000Z\",\"v\":1}\n",
                "{\"admits\":[{\"from\":2,\"to\":2}],\"attempt\":\"a1\",\"epoch\":\"e1\",\"kind\":\"attempt\",\"request\":{\"asset\":\"sha256-abababababababababababababababababababababababababababababababab\",\"bytes\":4096},\"seq\":6,\"ts\":\"2026-08-27T09:00:00.000Z\",\"turn\":1,\"v\":1,\"wire_digest\":\"w\"}\n",
                "{\"attempt\":\"a1\",\"content\":[{\"text\":\"answer\",\"type\":\"text\"}]__CONTINUATION__,\"kind\":\"output\",\"sealed\":{\"adapter\":\"__ADAPTER__\",\"fragments\":\"__FRAGMENTS__\",\"version\":1},\"seq\":7,\"ts\":\"2026-08-27T09:00:00.000Z\",\"turn\":1,\"usage\":{\"availability\":\"reported\",\"input_tokens\":\"1\",\"output_tokens\":\"1\"},\"v\":1}\n",
                "{\"kind\":\"settle\",\"outcome\":\"completed\",\"seq\":8,\"ts\":\"2026-08-27T09:00:00.000Z\",\"turn\":1,\"v\":1}\n",
                "{\"content\":[{\"text\":\"second\",\"type\":\"text\"}],\"kind\":\"input\",\"origin_key\":\"i2\",\"origin_tuple\":{\"client\":\"cli\",\"key\":\"i2\",\"op\":\"submit\",\"principal\":\"p\",\"target\":\"t\"},\"seq\":9,\"ts\":\"2026-08-27T09:00:01.000Z\",\"v\":1}\n",
                "{\"binary\":\"worker\",\"config_digest\":\"cfg\",\"instruction_digest\":\"ins\",\"kind\":\"run_start\",\"mode\":\"ordinary\",\"policy\":\"p\",\"recovery_ordinal\":0,\"run\":\"r2\",\"seq\":10,\"ts\":\"2026-08-27T09:00:01.000Z\",\"v\":1}\n",
                "{\"kind\":\"turn_open\",\"seq\":11,\"trigger\":{\"inputs\":[9]},\"ts\":\"2026-08-27T09:00:01.000Z\",\"turn\":2,\"v\":1}\n"
            )
            .replace("__ADAPTER__", adapter)
            .replace("__PROFILE_ASSET__", &profile_asset.asset)
            .replace("__CONTINUATION__", continuation)
            .replace("__FRAGMENTS__", fragments);
        let path = folder.join("main.jsonl");
        schema::validate_ledger(bytes.as_bytes(), 1).expect("schema-valid context ledger");
        fs::write(&path, bytes).expect("ledger bytes");
        let ledger = LockedLedger::open(&path, 1).expect("locked ledger");
        (directory, ledger)
    }

    #[test]
    fn goal_final_opens_next_turn_only_while_active_and_under_budget() {
        let fixture = context_ledger("openai_responses_v1", false);
        let original = fs::read_to_string(fixture.1.path()).unwrap();
        let settled = original.lines().take(8).collect::<Vec<_>>().join("\n") + "\n";
        for (phase, budget, expected) in [
            ("active", 16, true),
            ("complete", 16, false),
            ("blocked", 16, false),
            ("paused", 16, false),
            ("active", 0, false),
        ] {
            let root = tempfile::tempdir().unwrap();
            let session = "018f0000-0000-7000-8000-000000000003";
            let folder = root.path().join("threads").join(session);
            fs::create_dir_all(&folder).unwrap();
            let path = folder.join("main.jsonl");
            fs::write(&path, &settled).unwrap();
            let goal_folder = root.path().join("goals/sessions");
            fs::create_dir_all(&goal_folder).unwrap();
            fs::write(
                goal_folder.join(format!("{session}.json")),
                serde_json::to_vec(&json!({
                    "format":1,"id":"goal-test","revision":1,"objective":"finish the work",
                    "phase":phase,"maxGoalRounds":budget,"roundsStarted":0,
                    "createdAt":"2026-08-27T08:00:00.000Z","updatedAt":"2026-08-27T08:00:00.000Z"
                }))
                .unwrap(),
            )
            .unwrap();
            let mut ledger = LockedLedger::open(&path, 1).unwrap();
            let options = test_options(&ledger);
            let mut profile = tool_profile(root.path(), &["set_goal_state"]);
            profile.bindings = LaunchBindings::bind(
                &profile.config,
                Some("goal-test".to_owned()),
                Default::default(),
            )
            .unwrap();
            assert_eq!(
                open_goal_continuation(
                    &mut ledger,
                    &options,
                    &profile,
                    &RuntimeCancellation::default()
                )
                .unwrap(),
                expected,
                "phase={phase}"
            );
            assert_eq!(
                ledger.projection().unwrap().lifecycle.latest_turn,
                Some(if expected { 2 } else { 1 })
            );
            if expected {
                let event = ledger.projection().unwrap().events.last().unwrap();
                assert_eq!(
                    serde_json::to_value(event.raw()).unwrap()["trigger"],
                    json!({"goal":"goal-test"})
                );
            }
        }
    }

    /// The reason names what moved in the frozen head, so a ledger explains
    /// every cache reset: a catalog change is `tool_profile_change`, never a
    /// second `initial`.
    #[test]
    fn epoch_open_reason_names_the_part_of_the_head_that_moved() {
        let epoch = |tools: &str, system: &str, renderer: u64| {
            make_event(json!({"v":1,"seq":5,"kind":"epoch","ts":"2026-08-27T09:00:00.000Z","id":"e1","reason":"initial",
                "adapter":"anthropic_messages_v1","model":"m","system":{"asset":"sha256-aa","digest":system},
                "tools":{"asset":format!("sha256-{tools}"),"digest":tools},"renderer":renderer})).unwrap()
        };
        assert_eq!(epoch_open_reason(None, false, false, "td", "d"), "initial");
        assert_eq!(epoch_open_reason(None, false, true, "td", "d"), "recovery");
        let same = epoch("td", "d", CONTEXT_RENDERER_VERSION);
        assert_eq!(
            epoch_open_reason(Some(&same), true, false, "td", "d"),
            "compaction"
        );
        assert_eq!(
            epoch_open_reason(
                Some(&epoch("td", "d", CONTEXT_RENDERER_VERSION - 1)),
                false,
                false,
                "td",
                "d"
            ),
            "renderer_change"
        );
        assert_eq!(
            epoch_open_reason(Some(&same), false, false, "td2", "d"),
            "tool_profile_change"
        );
        assert_eq!(
            epoch_open_reason(Some(&same), false, false, "td", "d2"),
            "system_change"
        );
        assert_eq!(
            epoch_open_reason(Some(&same), false, false, "td", "d"),
            "model_change"
        );
    }

    #[test]
    fn semantic_anchors_select_latest_results_and_keep_whole_tool_batches() {
        let (_dir, ledger) = context_ledger("anthropic_messages_v1", false);
        let mut events = ledger.projection().unwrap().events.clone();
        let rows = [
            json!({"kind":"tool_call","attempt":"old","call":"ask-old","name":"ask_user_questions","args":{},"source":"provider"}),
            json!({"kind":"tool_result","call":"ask-old","outcome":"ok","content":[]}),
            json!({"kind":"tool_call","attempt":"new","call":"ask-new","name":"ask_user_questions","args":{},"source":"provider"}),
            json!({"kind":"tool_call","attempt":"new","call":"sibling","name":"read","args":{},"source":"provider"}),
            json!({"kind":"output","attempt":"new","usage":{"availability":"unavailable"},"content":[],"sealed":{"adapter":"anthropic_messages_v1","version":1,"fragments":"[]"}}),
            json!({"kind":"tool_result","call":"ask-new","outcome":"ok","content":[]}),
            json!({"kind":"tool_result","call":"sibling","outcome":"ok","content":[]}),
            json!({"kind":"tool_call","attempt":"task-old","call":"task-old","name":"task","args":{"task_name":"A"},"source":"provider"}),
            json!({"kind":"child_result","call":"task-old","child":"old.jsonl","spawn_id":"old","outcome":"completed","summary":"old task result"}),
            json!({"kind":"tool_call","attempt":"task-new","call":"task-new","name":"task","args":{"task_name":"A"},"source":"provider"}),
            json!({"kind":"child_result","call":"task-new","child":"new.jsonl","spawn_id":"new","outcome":"completed","summary":"new task result"}),
            json!({"kind":"tool_call","attempt":"task-b","call":"task-b","name":"task","args":{"task_name":"B"},"source":"provider"}),
            json!({"kind":"child_result","call":"task-b","child":"b.jsonl","spawn_id":"b","outcome":"completed","summary":"task B result"}),
        ];
        for (i, mut row) in rows.into_iter().enumerate() {
            let object = row.as_object_mut().unwrap();
            object.extend([
                ("v".into(), json!(1)),
                ("seq".into(), json!(13 + i)),
                ("turn".into(), json!(2)),
                ("ts".into(), json!("2026-08-27T09:00:02.000Z")),
            ]);
            events.push(make_event(row).unwrap());
        }
        let anchors = compaction_anchor_sequences(&events).unwrap();
        assert!(anchors.contains(&2), "initial admitted input survives");
        for seq in [15, 16, 17, 18, 19, 22, 23, 24, 25] {
            assert!(anchors.contains(&seq), "missing anchor {seq}");
        }
        for seq in [10, 13, 14, 20, 21] {
            assert!(!anchors.contains(&seq), "stale fact retained {seq}");
        }
        assert!(
            latest_compatible_epoch(
                &ledger,
                DialectId::AnthropicMessagesV1,
                "m",
                "d",
                "td",
                CONTEXT_RENDERER_VERSION
            )
            .is_none(),
            "old renderer cannot reuse continuation"
        );
    }

    #[test]
    fn input_transformation_diagnostics_do_not_change_the_cached_context() {
        let (_directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let before =
            project_provider_context(&ledger, "e1", DialectId::AnthropicMessagesV1, 2).unwrap();
        let before = before
            .items
            .iter()
            .map(|v| v.canonical_bytes().unwrap())
            .collect::<Vec<_>>();
        let changes = json!([{"type":"thinking_dropped","path":"messages.1.content.0","reason":"prefix_binding_mismatch"}]);
        record_input_transformations(&mut ledger, "2026-08-27T09:00:02.000Z", 2, "a2", &changes)
            .unwrap();
        let event = ledger.projection().unwrap().events.last().unwrap();
        assert_eq!(event.effective_visibility(), Visibility::Runtime);
        let raw = serde_json::to_value(event.raw()).unwrap();
        let text = materialize_string(&ledger, &raw["payload"]["input_transformations"]).unwrap();
        assert_eq!(serde_json::from_str::<Value>(&text).unwrap(), changes);
        let after =
            project_provider_context(&ledger, "e1", DialectId::AnthropicMessagesV1, 2).unwrap();
        assert_eq!(
            after
                .items
                .iter()
                .map(|v| v.canonical_bytes().unwrap())
                .collect::<Vec<_>>(),
            before
        );
    }

    #[test]
    fn appending_visible_state_preserves_context_prefix_and_recovery_bytes() {
        for dialect in [
            DialectId::AnthropicMessagesV1,
            DialectId::GoogleGenerationV1,
            DialectId::OpenaiChatV1,
            DialectId::OllamaChatV1,
            DialectId::DeepseekResponsesV1,
        ] {
            let (_directory, mut ledger) = context_ledger(dialect.as_str(), false);
            let path = ledger.path().to_owned();
            let original = fs::read(&path).unwrap();
            let before = project_provider_context(&ledger, "e1", dialect, 2).unwrap();
            let before_bytes = before
                .items
                .iter()
                .map(|item| item.canonical_bytes().unwrap())
                .collect::<Vec<_>>();
            let event = make_event(json!({"v":1,"seq":ledger.next_seq(),"turn":2,
                "kind":"state","ts":"2026-08-27T09:00:02.000Z",
                "subkind":"audit.append_only","payload":{"text":"new durable fact"}}))
            .unwrap();
            ledger
                .append_contract(event, BarrierContext::default())
                .unwrap();
            let after = project_provider_context(&ledger, "e1", dialect, 2).unwrap();
            let after_bytes = after
                .items
                .iter()
                .map(|item| item.canonical_bytes().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(after_bytes.len(), before_bytes.len() + 1, "{dialect:?}");
            assert_eq!(&after_bytes[..before_bytes.len()], &before_bytes);
            assert!(fs::read(&path).unwrap().starts_with(&original));
            drop(ledger);
            let recovered = LockedLedger::open(&path, 1).unwrap();
            let replay = project_provider_context(&recovered, "e1", dialect, 2).unwrap();
            assert_eq!(
                replay
                    .items
                    .iter()
                    .map(|item| item.canonical_bytes().unwrap())
                    .collect::<Vec<_>>(),
                after_bytes
            );
            assert_eq!(replay.admits, after.admits);
        }
    }

    #[test]
    fn stateless_projection_replays_sealed_baseline_and_only_new_admits() {
        let (_directory, ledger) = context_ledger("anthropic_messages_v1", false);
        let context = project_provider_context(&ledger, "e1", DialectId::AnthropicMessagesV1, 2)
            .expect("stateless context");
        assert_eq!(context.admits, vec![9]);
        assert_eq!(context.items.len(), 3);
        let roles = context
            .items
            .iter()
            .map(|item| {
                serde_json::to_value(item)
                    .expect("item value")
                    .get("role")
                    .and_then(Value::as_str)
                    .expect("role")
                    .to_owned()
            })
            .collect::<Vec<_>>();
        assert_eq!(roles, ["user", "sealed", "user"]);
        assert!(context.continuation_id.is_none());
    }

    #[test]
    fn server_managed_projection_uses_continuation_and_only_pending_host_facts() {
        let (_directory, ledger) = context_ledger("openai_responses_v1", true);
        let context = project_provider_context(&ledger, "e1", DialectId::OpenaiResponsesV1, 2)
            .expect("incremental context");
        assert_eq!(context.admits, vec![9]);
        assert_eq!(context.items.len(), 1);
        assert_eq!(context.continuation_id.as_deref(), Some("response-1"));
        let value = serde_json::to_value(&context.items[0]).expect("item value");
        assert_eq!(value["content"][0]["text"], "second");
    }

    #[test]
    fn deepseek_responses_ignores_family_continuation_and_replays_full_history() {
        let (_directory, ledger) = context_ledger("deepseek_responses_v1", true);
        let context = project_provider_context(&ledger, "e1", DialectId::DeepseekResponsesV1, 2)
            .expect("DeepSeek stateless context");
        assert_eq!(context.admits, vec![9]);
        assert_eq!(context.items.len(), 3);
        assert!(context.continuation_id.is_none());
        let roles = context
            .items
            .iter()
            .map(|item| {
                serde_json::to_value(item)
                    .expect("item value")
                    .get("role")
                    .and_then(Value::as_str)
                    .expect("role")
                    .to_owned()
            })
            .collect::<Vec<_>>();
        assert_eq!(roles, ["user", "sealed", "user"]);

        let provider_config = Provider {
            id: "deepseek".to_owned(),
            name: Some("DeepSeek".to_owned()),
            adapter: "responses".to_owned(),
            dialect: "deepseek_responses_v1".to_owned(),
            endpoint_owner: "deepseek".to_owned(),
            gateway_translation: "direct".to_owned(),
            evidence_revision:
                "deepseek-direct-responses-v4-flash-2026-07-31+function-json-schema-strict-v1"
                    .to_owned(),
            endpoint: "https://api.deepseek.com".to_owned(),
            credential_key: None,
            models: Vec::new(),
        };
        let model = Model {
            id: "deepseek-v4-flash".to_owned(),
            profile: "deepseek_responses_v1:deepseek-v4-flash".to_owned(),
            enabled: true,
            context_window_tokens: 100_000,
            compact_trigger_tokens: 90_000,
        };
        let resolved =
            provider::resolve_profile(&provider_config, &model).expect("proved DeepSeek profile");
        let epoch =
            provider::epoch_profile(&resolved, "system", None).expect("DeepSeek epoch profile");
        let prepared = prepare(&PrepareInput {
            attempt_id: "deepseek-multiturn".to_owned(),
            target: resolved.target,
            endpoint: provider_config.endpoint,
            epoch_profile: epoch,
            continuation_id: context.continuation_id,
            rendered_items: context.items,
            tool_catalog: ijson(json!([])).expect("empty tool catalog"),
            stream: true,
        })
        .expect("DeepSeek full-history request");
        let body: Value = serde_json::from_slice(&prepared.body).expect("request JSON");
        assert_eq!(body["input"].as_array().map(Vec::len), Some(3));
        assert!(body.get("previous_response_id").is_none());
        assert!(body.get("store").is_none());
    }

    #[test]
    fn overflow_compaction_is_checkpointed_and_changes_the_retry_projection() {
        let (_directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let options = Options {
            ledger: ledger.path().to_owned(),
            timestamp: "2026-08-27T09:00:02.000Z".to_owned(),
            clock_started: Instant::now(),
            run_id: "compact-run".to_owned(),
            binary: "worker".to_owned(),
            config_digest: "cfg".to_owned(),
            instruction_digest: "ins".to_owned(),
            policy: "p".to_owned(),
            config_fd: None,
            instruction_fd: None,
            launch_bindings_digest: None,
            launch_bindings_fd: None,
            credential_fd: None,
            web_search_ready: false,
            provider_test_redirect: None,
            lifecycle_hooks: None,
        };
        let history_before_compaction = fs::read(ledger.path()).expect("original history");
        assert!(auto_compact_for_turn(&mut ledger, &options, 2, None).expect("compact"));
        assert!(
            fs::read(ledger.path())
                .expect("compacted ledger")
                .starts_with(&history_before_compaction),
            "compaction must not rewrite history"
        );
        let projection = ledger.projection().expect("projection");
        assert!(matches!(
            projection.events[11].kind(),
            EventKind::Checkpoint
        ));
        assert!(matches!(projection.events[12].kind(), EventKind::Compact));
        let context = project_provider_context(&ledger, "e1", DialectId::AnthropicMessagesV1, 2)
            .expect("compacted context");
        assert_eq!(context.admits, vec![9]);
        assert_eq!(context.items.len(), 3);
        let summary = serde_json::to_value(&context.items[0]).expect("summary item");
        assert!(
            summary["content"][0]["text"]
                .as_str()
                .is_some_and(|text| text.starts_with("[compacted history]"))
        );
        let initial = serde_json::to_value(&context.items[1]).expect("initial input anchor");
        assert_eq!(initial["content"][0]["text"], "first");
        let current = serde_json::to_value(&context.items[2]).expect("current item");
        assert_eq!(current["content"][0]["text"], "second");
    }

    #[test]
    fn child_seed_snapshot_projects_normalized_payload_without_parent_baseline() {
        let directory = tempfile::tempdir().expect("thread root");
        let folder = directory.path().join("thread");
        fs::create_dir(&folder).expect("thread folder");
        let assets = store::AssetStore::new(folder.join("assets")).expect("assets");
        let mut seed = Vec::new();
        for value in [
            json!({
                "v":1,"seq":2,"kind":"input","ts":"2026-08-27T09:00:00.000Z",
                "content":[{"type":"text","text":"seed question"}],
                "origin_key":"seed-input","origin_tuple":{
                    "principal":"p","client":"cli","target":"parent","op":"submit","key":"seed-input"
                }
            }),
            json!({
                "v":1,"seq":8,"turn":1,"kind":"output","ts":"2026-08-27T09:00:00.000Z",
                "attempt":"source-attempt","content":[{"type":"text","text":"seed answer"}],
                "usage":{"availability":"unavailable"},
                "sealed":{"version":1,"adapter":"openai_responses_v1","fragments":"[]"}
            }),
            json!({
                "v":1,"seq":9,"turn":1,"kind":"state","ts":"2026-08-27T09:00:00.000Z",
                "subkind":"delegation","payload":{"call":"task-1","name":"task",
                    "invocation":{"goal":"write capital","output":{"path":"markdown/1.md"}}}
            }),
        ] {
            seed.extend(serde_json_canonicalizer::to_vec(&value).expect("seed line"));
            seed.push(b'\n');
        }
        let seed_asset = assets.publish(&seed).expect("seed asset");
        let digest = seed_asset
            .asset
            .strip_prefix("sha256-")
            .expect("digest prefix")
            .to_owned();
        let genesis = json!({
            "v":1,"seq":1,"kind":"genesis","ts":"2026-08-27T09:00:01.000Z",
            "thread":"018f0000-0000-7000-8000-000000000004","workspace":"ws",
            "format":1,"min_reader":1,"min_writer":1,"resume":"never",
            "parent":{"file":"parent.jsonl","seq":1,"spawn_id":"spawn-1"},
            "seed":{"source":"018f0000-0000-7000-8000-000000000003","kinds":["input","output","state"],"snapshot":{"asset":seed_asset.asset,"digest":digest}},
            "config":{"digest":"cfg"},"origin_key":"create-child","origin_tuple":{
                "principal":"p","client":"kernel","target":"018f0000-0000-7000-8000-000000000004","op":"create","key":"create-child"
            }
        });
        let mut bytes = serde_json_canonicalizer::to_vec(&genesis).expect("genesis");
        bytes.push(b'\n');
        let path = folder.join("main.jsonl");
        fs::write(&path, bytes).expect("ledger");
        let ledger = LockedLedger::open(&path, 1).expect("seeded ledger");
        let items = seed_provider_items(&ledger).expect("seed projection");
        assert_eq!(items.len(), 3);
        let first = serde_json::to_value(&items[0]).expect("first item");
        let second = serde_json::to_value(&items[1]).expect("second item");
        assert_eq!(first["role"], "user");
        assert_eq!(first["content"][0]["text"], "seed question");
        assert_eq!(second["role"], "assistant");
        assert_eq!(second["content"][0]["text"], "seed answer");
        assert!(second.get("sealed").is_none());
        let assignment = serde_json::to_value(&items[2]).unwrap();
        assert!(
            assignment["content"][0]["text"]
                .as_str()
                .unwrap()
                .contains("You are the worker assigned")
        );
        let args: Value =
            serde_json::from_str(assignment["content"][1]["text"].as_str().unwrap()).unwrap();
        assert_eq!(
            args,
            json!({"delegation":"task","arguments":{"goal":"write capital","output":{"path":"markdown/1.md"}},"resolved_inputs":[]})
        );
    }

    /// One scripted provider connection per response, in order; the request is
    /// drained by content-length before the response is written.
    fn scripted_provider_server(
        listener: TcpListener,
        responses: Vec<Vec<u8>>,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            for response in responses {
                let response = response.as_slice();
                let (mut stream, _) = listener.accept().expect("provider accept");
                let mut request = Vec::new();
                let mut chunk = [0_u8; 4096];
                let mut expected = None;
                loop {
                    let count = stream.read(&mut chunk).expect("provider request");
                    request.extend_from_slice(&chunk[..count]);
                    if expected.is_none() {
                        if let Some(end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n")
                        {
                            let headers = String::from_utf8_lossy(&request[..end + 4]);
                            let length = headers
                                .lines()
                                .find_map(|line| {
                                    line.strip_prefix("content-length: ")
                                        .or_else(|| line.strip_prefix("Content-Length: "))
                                })
                                .and_then(|value| value.parse::<usize>().ok())
                                .unwrap_or(0);
                            expected = Some(end + 4 + length);
                        }
                    }
                    if count == 0 || expected.is_some_and(|length| request.len() >= length) {
                        break;
                    }
                }
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                    response.len()
                )
                .expect("provider response header");
                stream.write_all(response).expect("provider response body");
            }
        })
    }

    fn overflow_profile(directory: &std::path::Path) -> RuntimeProfile {
        overflow_profile_with_trigger(directory, 90_000)
    }

    fn overflow_profile_with_trigger(
        directory: &std::path::Path,
        compact_trigger_tokens: u64,
    ) -> RuntimeProfile {
        let config = ConfigSnapshot {
            format: 1,
            workspace: ResolvedWorkspace {
                format: 1,
                revision: 1,
                id: "ws".to_owned(),
                name: "ws".to_owned(),
                folder_binding: None,
                selected_cwd: None,
                cwd: vec![
                    directory
                        .canonicalize()
                        .expect("canonical workspace")
                        .to_string_lossy()
                        .into_owned(),
                ],
                policy: WorkspacePolicy {
                    network: true,
                    provider: Some("fixture".to_owned()),
                    model: Some("gpt-5".to_owned()),
                    ..WorkspacePolicy::default()
                },
            },
            providers: ProvidersConfig {
                format: 1,
                revision: 1,
                providers: vec![Provider {
                    id: "fixture".to_owned(),
                    name: None,
                    adapter: "responses".to_owned(),
                    dialect: "openai_responses_v1".to_owned(),
                    endpoint_owner: "openai".to_owned(),
                    gateway_translation: "direct".to_owned(),
                    evidence_revision: "openai-2026-08-01".to_owned(),
                    endpoint: "https://api.openai.com/v1".to_owned(),
                    credential_key: None,
                    models: vec![Model {
                        id: "gpt-5".to_owned(),
                        profile: "openai_responses_v1:gpt-5".to_owned(),
                        enabled: true,
                        context_window_tokens: 100_000,
                        compact_trigger_tokens,
                    }],
                }],
                web_search: None,
            },
            legacy_integrations: (),
            settings: SettingsConfig::default(),
            session_settings: None,
            revisions: RevisionVector {
                workspace: 1,
                providers: 1,
                legacy_integrations: (),
                settings: 0,
                session_settings: None,
            },
        };
        let bindings = LaunchBindings::bind(&config, None, Default::default())
            .expect("default launch bindings");
        RuntimeProfile {
            config,
            validator: false,
            subagent: false,
            instruction: InstructionSnapshot {
                format: 1,
                sources: Vec::new(),
                effective: EffectiveInstructions::default(),
            },
            bindings,
        }
    }

    fn overflow_options(ledger_path: &std::path::Path) -> Options {
        Options {
            ledger: ledger_path.to_owned(),
            timestamp: "2026-08-27T09:00:03.000Z".to_owned(),
            clock_started: Instant::now(),
            run_id: "overflow-run".to_owned(),
            binary: "worker".to_owned(),
            config_digest: "cfg".to_owned(),
            instruction_digest: "ins".to_owned(),
            policy: "p".to_owned(),
            config_fd: None,
            instruction_fd: None,
            launch_bindings_digest: None,
            launch_bindings_fd: None,
            credential_fd: None,
            web_search_ready: false,
            provider_test_redirect: None,
            lifecycle_hooks: None,
        }
    }

    #[test]
    fn provider_attempt_persists_the_exact_request_body_as_a_thread_asset() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        let endpoint = format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        );
        set_provider_test_redirect(Some(endpoint));
        let server = scripted_provider_server(listener, vec![
            br#"{"id":"response-2","output":[{"content":[{"text":"answer two","type":"output_text"}],"type":"message"}],"status":"completed","usage":{"input_tokens":2,"output_tokens":2}}"#.to_vec(),
        ]);
        let profile = overflow_profile(directory.path());
        let options = overflow_options(ledger.path());
        let mut replies = [Ok(
            "{\"lease\":{\"attempt\":\"overflow-run-attempt-13\",\"granted\":true}}".to_owned(),
        )]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut None,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        result.expect("provider turn");
        server.join().expect("provider server");
        let projection = ledger.projection().expect("projection");
        let attempt = projection
            .events
            .iter()
            .find(|event| event.seq() == 13)
            .expect("attempt event");
        assert_eq!(attempt.kind().as_str(), "attempt");
        let attempt_value = serde_json::to_value(attempt.raw()).expect("attempt value");
        let request = attempt_value["request"]
            .as_object()
            .expect("attempt.request");
        let asset = request["asset"].as_str().expect("asset name");
        let bytes = request["bytes"].as_u64().expect("byte length");
        let thread_folder = ledger.path().parent().expect("thread folder").to_path_buf();
        let body = store::AssetStore::new(thread_folder.join("assets"))
            .expect("asset store")
            .read_verified(asset)
            .expect("the request asset is durable before the attempt");
        assert_eq!(body.len() as u64, bytes);
        let rendered: serde_json::Value =
            serde_json::from_slice(&body).expect("request body is JSON");
        assert_eq!(rendered["model"], json!("gpt-5"));
        assert!(
            serde_json::to_string(&rendered).unwrap().contains("second"),
            "the persisted body is the rendered request for this turn"
        );
        assert_eq!(
            projection.events.last().map(|event| event.kind().as_str()),
            Some("settle")
        );
    }

    #[test]
    fn context_overflow_closes_compacts_and_retries_without_resetting_the_turn() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        let endpoint = format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        );
        set_provider_test_redirect(Some(endpoint));
        // Order on the wire: the overflowing attempt, the compactor request
        // (answered without an artifact: the summary falls back to the
        // deterministic quoted history), then the post-compact retry.
        let server = scripted_provider_server(listener, vec![
            br#"{"error":{"code":"context_length_exceeded"}}"#.to_vec(),
            br#"{"id":"response-compactor","output":[{"content":[{"text":"no artifact","type":"output_text"}],"type":"message"}],"status":"completed","usage":{"input_tokens":1,"output_tokens":1}}"#.to_vec(),
            br#"{"id":"response-2","output":[{"content":[{"text":"after compact","type":"output_text"}],"type":"message"}],"status":"completed","usage":{"input_tokens":2,"output_tokens":2}}"#.to_vec(),
        ]);
        let profile = overflow_profile(directory.path());
        let options = overflow_options(ledger.path());
        let mut replies = [
            Ok("{\"lease\":{\"attempt\":\"overflow-run-attempt-13\",\"granted\":true}}".to_owned()),
            Ok("{\"lease\":{\"attempt\":\"overflow-run-attempt-19\",\"granted\":true}}".to_owned()),
        ]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut None,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        result.expect("overflow retry");
        server.join().expect("provider server");
        let projection = ledger.projection().expect("projection");
        assert_eq!(
            projection
                .events
                .iter()
                .skip(11)
                .map(|event| event.kind().as_str())
                .collect::<Vec<_>>(),
            [
                "epoch",
                "attempt",
                "attempt_dispatched",
                "error",
                "checkpoint",
                "compact",
                "epoch",
                "attempt",
                "attempt_dispatched",
                "output",
                "settle",
            ]
        );
        assert_eq!(projection.latest_turn, Some(2));
        assert!(projection.terminal_tail);
        assert_eq!(serde_json::to_value(projection.events.last().unwrap().raw()).unwrap()["promoted_output_seq"].as_u64(),
            projection.events.iter().rev().find(|event| event.kind() == &EventKind::Output).map(Event::seq));
        let compact = projection
            .events
            .iter()
            .find(|event| event.kind().as_str() == "compact")
            .map(|event| serde_json::to_value(event.raw()).unwrap())
            .expect("compact");
        assert_eq!(
            compact["summary_request"]["accepted"], false,
            "{}",
            compact["summary_request"]
        );
        assert!(
            compact["summary_request"]["reason"]
                .as_str()
                .unwrap()
                .contains("carries no summary_artifact"),
            "{}",
            compact["summary_request"]
        );
        let deterministic = compact["summary"].as_str().unwrap();
        assert!(
            deterministic.starts_with("[compacted history]\nseq "),
            "the deterministic quoted history is the fallback: {deterministic}"
        );
        assert!(
            !deterministic.contains("\"kind\":"),
            "the fallback reads as history, not as a JSON dump: {deterministic}"
        );
    }

    fn overflow_with_summary(evidence: impl FnOnce(&[u64]) -> u64) -> (Vec<Value>, u64) {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let turn = ledger.projection().unwrap().latest_turn.unwrap();
        let plan =
            engine::plan_context_compaction(&ledger.projection().unwrap().events, turn).unwrap();
        assert!(
            !plan.covers.is_empty(),
            "the fixture has compactable history"
        );
        let evidence = evidence(&plan.covers);
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        let endpoint = format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        );
        set_provider_test_redirect(Some(endpoint));
        // Order on the wire: the overflowing attempt, the compactor request
        // (inside the same lease), then the post-compact retry.
        let compactor = format!(
            r#"{{"id":"response-summary","output":[{{"type":"function_call","call_id":"call-summary","name":"summary_artifact","arguments":"{{\"continuation\":\"Earlier the user asked for the fixture task.\",\"evidence_refs\":[{evidence}]}}"}}],"status":"completed","usage":{{"input_tokens":9,"output_tokens":3}}}}"#
        );
        let server = scripted_provider_server(listener, vec![
            br#"{"error":{"code":"context_length_exceeded"}}"#.to_vec(),
            compactor.into_bytes(),
            br#"{"id":"response-2","output":[{"content":[{"text":"after compact","type":"output_text"}],"type":"message"}],"status":"completed","usage":{"input_tokens":2,"output_tokens":2}}"#.to_vec(),
        ]);
        let profile = overflow_profile(directory.path());
        let options = overflow_options(ledger.path());
        let mut replies = [
            Ok("{\"lease\":{\"attempt\":\"overflow-run-attempt-13\",\"granted\":true}}".to_owned()),
            Ok("{\"lease\":{\"attempt\":\"overflow-run-attempt-19\",\"granted\":true}}".to_owned()),
        ]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut None,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        result.expect("overflow retry with summary");
        server.join().expect("provider server");
        let projection = ledger.projection().expect("projection");
        assert!(projection.terminal_tail);
        let events = projection
            .events
            .iter()
            .map(|event| serde_json::to_value(event.raw()).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            events
                .iter()
                .filter(|event| event["kind"] == "compact")
                .count(),
            1
        );
        (events, evidence)
    }

    /// event §compact `summary_request`: the admitted artifact is the
    /// compact summary and the record carries the frozen bundle, the evidence
    /// and the compactor usage; the turn otherwise proceeds exactly as the
    /// plain overflow retry.
    #[test]
    fn context_overflow_writes_the_admitted_model_summary() {
        let (events, evidence) = overflow_with_summary(|covers| covers[0]);
        let compact = events
            .iter()
            .find(|event| event["kind"] == "compact")
            .unwrap();
        let record = &compact["summary_request"];
        assert_eq!(record["accepted"], true, "{record}");
        assert_eq!(record["evidence_refs"], json!([evidence]));
        assert_eq!(record["bundle"]["covers"], compact["covers"]);
        assert!(
            record["bundle"]["sha256"]
                .as_str()
                .unwrap()
                .starts_with("sha256-")
        );
        assert_eq!(record["usage"]["input_tokens"], "9");
        assert_eq!(record["model"], "gpt-5");
        assert!(
            record.get("continuation").is_none(),
            "the continuation is the summary itself"
        );
        assert_eq!(
            compact["summary"],
            "[compacted history]\nEarlier the user asked for the fixture task."
        );
        assert_eq!(
            events
                .iter()
                .filter(|event| event["kind"] == "settle")
                .last()
                .unwrap()["outcome"],
            "completed"
        );
    }

    /// A reference outside the frozen bundle is rejected: the record says why
    /// and the deterministic quoted history is the summary.
    #[test]
    fn context_overflow_rejects_foreign_evidence_and_keeps_the_deterministic_summary() {
        let (events, _) = overflow_with_summary(|covers| covers.iter().max().unwrap() + 1000);
        let compact = events
            .iter()
            .find(|event| event["kind"] == "compact")
            .unwrap();
        let record = &compact["summary_request"];
        assert_eq!(record["accepted"], false, "{record}");
        assert!(
            record["reason"]
                .as_str()
                .unwrap()
                .contains("outside the frozen bundle"),
            "{record}"
        );
        assert!(
            compact["summary"]
                .as_str()
                .unwrap()
                .starts_with("[compacted history]\nseq ")
        );
    }

    #[test]
    fn quoted_null_ready_terminal_and_reopen_keep_one_normalized_call() {
        for dialect in [DialectId::GlmChatV1, DialectId::DeepseekResponsesV1] {
            let (_directory, mut ledger) = context_ledger(dialect.as_str(), false);
            let options = test_options(&ledger);
            let manifest = BuiltinManifest::compiled();
            append_test_attempt(&mut ledger, "glm-null");
            let arguments = json!({"working_directory":".","max_duration_ms":1000,"command":"pwd","program":"null",
            "args":null,"steps":[],"failure_policy":"stop_on_error","writable_paths":[],"artifact_outputs":[]});
            let response = if dialect == DialectId::GlmChatV1 {
                json!({"id":"glm-response","choices":[{"index":0,"finish_reason":"tool_calls","message":{
            "role":"assistant","content":null,"tool_calls":[{"id":"glm-call","type":"function","function":{
                "name":"shell","arguments":serde_json::to_string(&arguments).unwrap()}}]}}]})
            } else {
                json!({"id":"ds-response","status":"completed","output":[{"type":"function_call","id":"item-1",
                "call_id":"glm-call","name":"shell","arguments":serde_json::to_string(&arguments).unwrap()}]})
            };
            let terminal = provider::normalize_dialect_response(
                dialect,
                &serde_json::to_vec(&response).unwrap(),
            )
            .unwrap();
            let native = terminal.sealed_fragments.clone();
            let mut ready = terminal.tool_calls[0].clone();
            repair_provider_call_arguments(DialectId::OpenaiResponsesV1, &mut ready);
            assert_eq!(
                ready.arguments, arguments,
                "other dialects do not opt into this repair"
            );
            repair_provider_call_arguments(dialect, &mut ready);
            assert_eq!(ready.arguments["program"], Value::Null);
            tools::fixed_schema("shell")
                .unwrap()
                .validate(&ready.arguments)
                .unwrap();
            append_provider_tool_call(&mut ledger, &options, &manifest, 2, "glm-null", &ready)
                .unwrap();
            let eager = EagerDispatch {
                calls: vec![ready.clone()],
                ..Default::default()
            };
            append_terminal(
                &mut ledger,
                TerminalAppend {
                    options: &options,
                    manifest: &manifest,
                    eager: &eager,
                    dialect,
                    server_managed: false,
                    turn: 2,
                    attempt: "glm-null",
                },
                terminal,
            )
            .unwrap();
            let events = &ledger.projection().unwrap().events;
            assert_eq!(
                events
                    .iter()
                    .filter(|e| *e.kind() == EventKind::ToolCall
                        && e.string_field("call") == Some("glm-call"))
                    .count(),
                1
            );
            let output = events
                .iter()
                .find(|e| {
                    *e.kind() == EventKind::Output && e.string_field("attempt") == Some("glm-null")
                })
                .unwrap();
            let raw = serde_json::to_value(output.raw()).unwrap();
            assert_eq!(
                serde_json::from_str::<Value>(
                    &materialize_string(&ledger, &raw["sealed"]["fragments"]).unwrap()
                )
                .unwrap(),
                native
            );
            let path = ledger.path().to_owned();
            drop(ledger);
            let reopened = LockedLedger::open(&path, 1).unwrap();
            validate_adopted_calls(&reopened, "glm-null", &[ready]).unwrap();
        }
    }

    /// A provider batch whose first call needs approval parks the worker
    /// before its siblings run. Recovery used to bury every unpaired call as
    /// `{aborted: "crash"}`, which both lied and threw the work away — on the
    /// Anthropic routes it escalated to `unresolved_dispatch` and ended the
    /// session. A batch executes in order, so a later unpaired call in the
    /// same attempt provably never started and is re-dispatched.
    #[test]
    fn completed_wire_call_id_gets_a_local_identity_and_keeps_native_result_pairing() {
        for compact_old in [false, true] {
            let (directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
            let options = test_options(&ledger);
            let profile = tool_profile(directory.path(), &["think"]);
            let manifest = BuiltinManifest::compiled();
            let cancellation = RuntimeCancellation::default();
            let call = provider::ToolCall {
                call_id: "shell_1".into(),
                name: "think".into(),
                arguments: json!({"thought":"work"}),
            };
            let native =
                json!([{"type":"tool_use","id":"shell_1","name":"think","input":call.arguments}]);
            let terminal = || {
                provider::normalize_dialect_response(DialectId::AnthropicMessagesV1,
            &serde_json::to_vec(&json!({"id":"m1","type":"message","role":"assistant","content":native,"stop_reason":"tool_use"})).unwrap()).unwrap()
            };
            append_test_attempt(&mut ledger, "old-id");
            let mut old_eager = EagerDispatch::default();
            eager_dispatch_ready_call(
                &mut ledger,
                EagerReadyCall {
                    options: &options,
                    profile: &profile,
                    selected: Selected { version: 2 },
                    manifest: &manifest,
                    attempt: "old-id",
                    turn: 2,
                    cancellation: &cancellation,
                },
                call.clone(),
                &mut old_eager,
                None,
                &mut std::iter::empty::<io::Result<String>>(),
                &mut Vec::new(),
            )
            .unwrap();
            assert!(
                local_provider_call_id(&ledger, "next-id", "shell_1").is_err(),
                "incomplete response cannot be reused"
            );
            append_terminal(
                &mut ledger,
                TerminalAppend {
                    options: &options,
                    manifest: &manifest,
                    eager: &old_eager,
                    dialect: DialectId::AnthropicMessagesV1,
                    server_managed: false,
                    turn: 2,
                    attempt: "old-id",
                },
                terminal(),
            )
            .unwrap();
            assert_ne!(
                local_provider_call_id(&ledger, "next-id", "shell_1").unwrap(),
                "shell_1"
            );
            if compact_old {
                let covers = ledger
                    .projection()
                    .unwrap()
                    .events
                    .iter()
                    .filter(|event| {
                        event.string_field("attempt") == Some("old-id")
                            && matches!(event.kind(), EventKind::ToolCall | EventKind::Output)
                            || event.string_field("call") == Some("shell_1")
                                && *event.kind() == EventKind::ToolResult
                    })
                    .map(Event::seq)
                    .collect::<Vec<_>>();
                let boundary = ledger.next_seq() - 1;
                ledger.append_contract(make_event(json!({"v":1,"seq":ledger.next_seq(),"kind":"checkpoint","ts":options.event_timestamp(),"covers":boundary,"summary":"test boundary"})).unwrap(),BarrierContext::default()).unwrap();
                ledger.append_contract(make_event(json!({"v":1,"seq":ledger.next_seq(),"kind":"compact","ts":options.event_timestamp(),"covers":ranges(&covers),"summary":"old step done"})).unwrap(),BarrierContext::default()).unwrap();
            }
            append_test_attempt(&mut ledger, "next-id");
            let mut eager = EagerDispatch::default();
            let mut delta = ProviderFrame::ToolDelta {
                call_id: "shell_1".into(),
                name: Some("think".into()),
                delta: "{}".into(),
            };
            localize_provider_frame(&ledger, "next-id", &mut eager, &mut delta).unwrap();
            let ProviderFrame::ToolDelta {
                call_id: local_id, ..
            } = delta
            else {
                panic!()
            };
            assert_ne!(local_id, "shell_1");
            let mut ready = ProviderFrame::ToolCallReady(call.clone());
            localize_provider_frame(&ledger, "next-id", &mut eager, &mut ready).unwrap();
            let ProviderFrame::ToolCallReady(local_call) = ready else {
                panic!()
            };
            assert_eq!(local_call.call_id, local_id);
            eager_dispatch_ready_call(
                &mut ledger,
                EagerReadyCall {
                    options: &options,
                    profile: &profile,
                    selected: Selected { version: 2 },
                    manifest: &manifest,
                    attempt: "next-id",
                    turn: 2,
                    cancellation: &cancellation,
                },
                local_call.clone(),
                &mut eager,
                None,
                &mut std::iter::empty::<io::Result<String>>(),
                &mut Vec::new(),
            )
            .unwrap();
            let outcome = append_terminal(
                &mut ledger,
                TerminalAppend {
                    options: &options,
                    manifest: &manifest,
                    eager: &eager,
                    dialect: DialectId::AnthropicMessagesV1,
                    server_managed: false,
                    turn: 2,
                    attempt: "next-id",
                },
                terminal(),
            )
            .unwrap();
            assert_eq!(outcome.tool_calls[0].call_id, local_id);
            assert_eq!(
                local_provider_call_id(&ledger, "next-id", "shell_1").unwrap(),
                local_id
            );
            assert_ne!(
                local_provider_call_id(&ledger, "third-id", "shell_1").unwrap(),
                local_id,
                "each response owns a distinct identity"
            );
            let events = &ledger.projection().unwrap().events;
            let new_call = events
                .iter()
                .find(|e| {
                    *e.kind() == EventKind::ToolCall && e.string_field("call") == Some(&local_id)
                })
                .unwrap();
            assert_eq!(new_call.string_field("provider_call"), Some("shell_1"));
            assert_eq!(
                events
                    .iter()
                    .filter(|e| *e.kind() == EventKind::ToolResult
                        && e.string_field("call") == Some(&local_id))
                    .count(),
                1
            );
            let projected =
                project_provider_context(&ledger, "e1", DialectId::AnthropicMessagesV1, 2).unwrap();
            let serialized = serde_json::to_string(
                &projected
                    .items
                    .iter()
                    .map(|i| serde_json::to_value(i).unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            assert!(
                !serialized.contains(&local_id),
                "local durable identity must not leak onto provider wire"
            );
            let result = events
                .iter()
                .find(|e| {
                    *e.kind() == EventKind::ToolResult && e.string_field("call") == Some(&local_id)
                })
                .unwrap();
            let raw = serde_json::to_value(result.raw()).unwrap();
            let rendered = render_event_item(
                &ledger,
                result,
                raw.as_object().unwrap(),
                &Default::default(),
                DialectId::AnthropicMessagesV1,
            )
            .unwrap()
            .unwrap();
            assert_eq!(rendered["content"][0]["call_id"], "shell_1");
            let output = events
                .iter()
                .find(|e| {
                    *e.kind() == EventKind::Output && e.string_field("attempt") == Some("next-id")
                })
                .unwrap();
            let raw = serde_json::to_value(output.raw()).unwrap();
            assert_eq!(
                serde_json::from_str::<Value>(
                    &materialize_string(&ledger, &raw["sealed"]["fragments"]).unwrap()
                )
                .unwrap(),
                native
            );
            validate_adopted_calls(&ledger, "next-id", std::slice::from_ref(&local_call)).unwrap();
            let path = ledger.path().to_owned();
            drop(ledger);
            let reopened = LockedLedger::open(&path, 1).unwrap();
            assert_eq!(
                local_provider_call_id(&reopened, "next-id", "shell_1").unwrap(),
                local_id
            );
            assert!(pending_tool_calls(&reopened).unwrap().is_empty());
        }
    }

    #[test]
    fn invalid_tool_write_authority_pairs_an_error_without_crashing_or_starting_execution() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", false);
        let options = overflow_options(ledger.path());
        let profile = overflow_profile(directory.path());
        append_test_attempt(&mut ledger, "invalid-authority");
        let call = provider::ToolCall {
            call_id: "bad-root".into(),
            name: "shell".into(),
            arguments: json!({"command":"echo must-not-run","writable_paths":["/opt/homebrew"]}),
        };
        append_provider_tool_call(
            &mut ledger,
            &options,
            &BuiltinManifest::compiled(),
            2,
            "invalid-authority",
            &call,
        )
        .unwrap();
        let cancelled = RuntimeCancellation::default();
        let parked = execute_provider_tool_calls(
            &mut ledger,
            ToolCallBatch {
                options: &options,
                profile: &profile,
                selected: Selected { version: 2 },
                attempt: "invalid-authority",
                turn: 2,
                calls: std::slice::from_ref(&call),
                cancellation: &cancelled,
            },
            None,
            &mut std::iter::empty::<io::Result<String>>(),
            &mut Vec::new(),
        )
        .unwrap();
        assert!(!parked);
        let events = &ledger.projection().unwrap().events;
        assert!(
            !events
                .iter()
                .any(|event| *event.kind() == EventKind::ToolExecutionStarted)
        );
        let result = serde_json::to_value(events.last().unwrap().raw()).unwrap();
        assert_eq!(result["kind"], "tool_result");
        assert_eq!(result["outcome"], "error");
        assert!(
            result["content"][0]["text"]
                .as_str()
                .unwrap()
                .contains("outside the effective writable roots")
        );
        assert!(pending_tool_calls(&ledger).unwrap().is_empty());
    }

    /// A permission mode change in the session folder takes effect on the
    /// next tool batch of the same (running) ledger: no restart, no reopen.
    #[cfg(target_os = "macos")]
    #[test]
    fn permission_mode_file_change_applies_to_the_next_tool_batch() {
        let (directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let options = test_options(&ledger);
        let profile = tool_profile(directory.path(), &["shell"]);
        let folder = ledger.path().parent().unwrap().to_path_buf();
        append_test_attempt(&mut ledger, "mode-attempt");
        let cancelled = RuntimeCancellation::default();
        let run = |ledger: &mut LockedLedger, id: &str| {
            let call = provider::ToolCall {
                call_id: id.into(),
                name: "shell".into(),
                arguments: json!({
                    "working_directory":"", "max_duration_ms":1000,
                    "command":"echo mode", "program":null, "args":[], "steps":[],
                    "failure_policy":"stop_on_error", "writable_paths":[], "artifact_outputs":[]
                }),
            };
            append_provider_tool_call(
                ledger,
                &options,
                &BuiltinManifest::compiled(),
                2,
                "mode-attempt",
                &call,
            )
            .unwrap();
            let parked = execute_provider_tool_calls(
                ledger,
                ToolCallBatch {
                    options: &options,
                    profile: &profile,
                    selected: Selected { version: 2 },
                    attempt: "mode-attempt",
                    turn: 2,
                    calls: std::slice::from_ref(&call),
                    cancellation: &cancelled,
                },
                None,
                &mut std::iter::empty::<io::Result<String>>(),
                &mut Vec::new(),
            )
            .unwrap();
            assert!(
                !parked,
                "{id}: an execute call never parks under these modes"
            );
            let events = &ledger.projection().unwrap().events;
            let started = events.iter().any(|event| {
                *event.kind() == EventKind::ToolExecutionStarted
                    && event.string_field("call") == Some(id)
            });
            let result = events
                .iter()
                .rev()
                .find(|event| {
                    *event.kind() == EventKind::ToolResult && event.string_field("call") == Some(id)
                })
                .expect("paired result");
            (started, serde_json::to_value(result.raw()).unwrap())
        };

        engine::write_permission_mode(&folder, engine::PermissionMode::ReadOnly).unwrap();
        let (started, result) = run(&mut ledger, "denied-call");
        assert!(!started, "read-only never starts an execute call");
        assert_eq!(
            result["outcome"]["denied"], "read-only permission mode",
            "{result}"
        );

        engine::write_permission_mode(&folder, engine::PermissionMode::WorkspaceWrite).unwrap();
        let (started, result) = run(&mut ledger, "allowed-call");
        assert!(
            started,
            "workspace-write runs execute calls on the very next batch"
        );
        assert!(result["outcome"].get("denied").is_none(), "{result}");

        // A corrupt record is the default mode plus a diagnostic, never a crash.
        fs::write(engine::permission_mode_path(&folder), b"{").unwrap();
        let (started, result) = run(&mut ledger, "corrupt-call");
        assert!(started);
        assert!(result["outcome"].get("denied").is_none(), "{result}");
        assert!(pending_tool_calls(&ledger).unwrap().is_empty());
    }

    #[test]
    fn tracked_approved_head_is_unstarted_until_the_execution_barrier() {
        for started in [false, true] {
            let (_directory, mut ledger) = context_ledger("openai_responses_v1", false);
            let options = overflow_options(ledger.path());
            append_test_attempt(&mut ledger, "head-attempt");
            append_provider_tool_call(
                &mut ledger,
                &options,
                &BuiltinManifest::compiled(),
                2,
                "head-attempt",
                &provider::ToolCall {
                    call_id: "head".into(),
                    name: "shell".into(),
                    arguments: json!({"command":"echo once"}),
                },
            )
            .unwrap();
            for mut row in [
                json!({"kind":"approval_request","call":"head","scope":"execute"}),
                json!({"kind":"approval_response","call":"head","grant":true,"origin_key":"head-approval","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"approve","key":"head-approval"}}),
            ] {
                row["v"] = json!(1);
                row["seq"] = json!(ledger.next_seq());
                row["turn"] = json!(2);
                row["ts"] = json!(options.event_timestamp());
                ledger
                    .append_contract(make_event(row).unwrap(), BarrierContext::default())
                    .unwrap();
            }
            if started {
                ledger.append_contract(make_event(json!({"v":1,"seq":ledger.next_seq(),"turn":2,"ts":options.event_timestamp(),"kind":"tool_execution_started","call":"head"})).unwrap(), BarrierContext::default()).unwrap();
            }
            let path = ledger.path().to_owned();
            drop(ledger);
            let mut ledger = LockedLedger::open(path, 1).unwrap();
            let pending = pending_tool_calls(&ledger).unwrap();
            assert!(pending[0].execution_tracked);
            assert_eq!(pending[0].execution_started, started);
            recover_unpaired_tool_calls(
                &mut ledger,
                &options,
                None,
                Selected { version: 2 },
                true,
                &mut std::iter::empty::<io::Result<String>>(),
                &mut Vec::new(),
                &RuntimeCancellation::default(),
            )
            .unwrap();
            let result =
                serde_json::to_value(ledger.projection().unwrap().events.last().unwrap().raw())
                    .unwrap();
            assert_eq!(
                result["outcome"]["aborted"],
                if started { "crash" } else { "recovered" }
            );
        }
    }

    #[test]
    fn a_batch_sibling_behind_an_approval_hold_is_recovered_not_buried() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", false);
        let ts = "2026-08-27T09:00:01.000Z";
        for row in [
            json!({"v":1,"seq":12,"turn":2,"kind":"attempt","ts":ts,"attempt":"a2","epoch":"e1","wire_digest":"w2","request":{"asset":format!("sha256-{}", "cd".repeat(32)),"bytes":16},"admits":[{"from":9,"to":9}]}),
            // Three shell calls in one batch; the first parks on an approval.
            json!({"v":1,"seq":13,"turn":2,"kind":"tool_call","ts":ts,"attempt":"a2","call":"c1","name":"shell","args":{"command":"echo one"},"source":"provider"}),
            json!({"v":1,"seq":14,"turn":2,"kind":"approval_request","ts":ts,"call":"c1","scope":"execute"}),
            json!({"v":1,"seq":15,"turn":2,"kind":"tool_call","ts":ts,"attempt":"a2","call":"c2","name":"shell","args":{"command":"echo two"},"source":"provider"}),
            json!({"v":1,"seq":16,"turn":2,"kind":"tool_call","ts":ts,"attempt":"a2","call":"c3","name":"shell","args":{"command":"echo three"},"source":"provider"}),
            json!({"v":1,"seq":17,"turn":2,"kind":"output","ts":ts,"attempt":"a2","content":[],"sealed":{"version":1,"adapter":"openai_responses_v1","fragments":"[]"},"usage":{"availability":"unavailable"}}),
            json!({"v":1,"seq":18,"turn":2,"kind":"approval_response","ts":ts,"call":"c1","grant":true,"origin_key":"ap1","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"approve","key":"ap1"}}),
        ] {
            ledger
                .append_contract(make_event(row).unwrap(), BarrierContext::default())
                .expect("seed");
        }
        let pending = pending_tool_calls(&ledger).expect("pending");
        assert_eq!(
            pending.iter().map(|c| c.call.as_str()).collect::<Vec<_>>(),
            ["c1", "c2", "c3"]
        );

        // Reconcile-only: no profile to execute with, but the verdict must
        // still tell the truth about which calls ran.
        let options = overflow_options(ledger.path());
        let mut lines = std::iter::empty::<io::Result<String>>();
        let mut stdout = Vec::new();
        let cancellation = RuntimeCancellation::default();
        recover_unpaired_tool_calls(
            &mut ledger,
            &options,
            None,
            Selected { version: 2 },
            true,
            &mut lines,
            &mut stdout,
            &cancellation,
        )
        .expect("reconcile");
        let outcomes = ledger
            .projection()
            .unwrap()
            .events
            .iter()
            .filter(|event| *event.kind() == EventKind::ToolResult)
            .map(|event| {
                let raw = serde_json::to_value(event.raw()).unwrap();
                (
                    raw["call"].as_str().unwrap().to_owned(),
                    raw["outcome"]["aborted"].as_str().unwrap().to_owned(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            outcomes,
            // c1 stays conservative: it was approved, and recovery cannot tell
            // a worker that parked at the hold from one that took the answer
            // inline and died running it. c2 and c3 are behind it in the same
            // ordered batch, so they provably never started.
            [
                ("c1".to_owned(), "crash".to_owned()),
                ("c2".to_owned(), "recovered".to_owned()),
                ("c3".to_owned(), "recovered".to_owned())
            ],
            "only the ambiguous head of the batch may be called a crash"
        );
        drop(directory);
    }

    /// A trimmed result answers the same assistant call, so it must render
    /// where the result it retracts rendered. Ordering it by its own seq
    /// pushes it past later calls and the provider rejects the request with
    /// "no tool output found for tool call" — a live corpus run did exactly
    /// that. The rendered order must keep every call adjacent to its result.
    #[test]
    fn a_trimmed_result_renders_in_the_place_of_the_result_it_retracts() {
        let (_directory, mut ledger) = context_ledger("deepseek_responses_v1", false);
        for row in [
            json!({"v":1,"seq":12,"turn":2,"kind":"attempt","ts":"2026-08-27T09:00:01.000Z","attempt":"a2","epoch":"e1","wire_digest":"w2","request":{"asset":format!("sha256-{}", "cd".repeat(32)),"bytes":16},"admits":[{"from":9,"to":9}]}),
            json!({"v":1,"seq":13,"turn":2,"kind":"tool_call","ts":"2026-08-27T09:00:01.000Z","attempt":"a2","call":"c1","name":"shell","args":{},"source":"provider"}),
            json!({"v":1,"seq":14,"turn":2,"kind":"tool_result","ts":"2026-08-27T09:00:01.000Z","call":"c1","outcome":"ok","content":[{"type":"text","text":"first"}]}),
            json!({"v":1,"seq":15,"turn":2,"kind":"output","ts":"2026-08-27T09:00:01.000Z","attempt":"a2","content":[{"type":"text","text":"one"}],"sealed":{"version":1,"adapter":"deepseek_responses_v1","fragments":"[]"},"usage":{"availability":"unavailable"}}),
            json!({"v":1,"seq":16,"turn":2,"kind":"attempt","ts":"2026-08-27T09:00:01.000Z","attempt":"a3","epoch":"e1","wire_digest":"w3","request":{"asset":format!("sha256-{}", "ef".repeat(32)),"bytes":16},"admits":[]}),
            json!({"v":1,"seq":17,"turn":2,"kind":"tool_call","ts":"2026-08-27T09:00:01.000Z","attempt":"a3","call":"c2","name":"shell","args":{},"source":"provider"}),
            json!({"v":1,"seq":18,"turn":2,"kind":"tool_result","ts":"2026-08-27T09:00:01.000Z","call":"c2","outcome":"ok","content":[{"type":"text","text":"second"}]}),
            json!({"v":1,"seq":19,"turn":2,"kind":"output","ts":"2026-08-27T09:00:01.000Z","attempt":"a3","content":[{"type":"text","text":"two"}],"sealed":{"version":1,"adapter":"deepseek_responses_v1","fragments":"[]"},"usage":{"availability":"unavailable"}}),
            // The trim of the FIRST call lands after the second call's result.
            json!({"v":1,"seq":20,"turn":2,"kind":"tool_result","ts":"2026-08-27T09:00:02.000Z","call":"c1","outcome":"ok","content":[{"type":"text","text":"first [...] trimmed"}],"supersedes":[{"from":14,"to":14}]}),
        ] {
            ledger
                .append_contract(make_event(row).unwrap(), BarrierContext::default())
                .expect("seed");
        }
        let context = project_provider_context(&ledger, "e1", DialectId::DeepseekResponsesV1, 2)
            .expect("render");
        let rendered = context
            .items
            .iter()
            .map(|item| serde_json::to_value(item).unwrap())
            .collect::<Vec<_>>();
        // Sealed fragments carry the assistant calls, so compare the item
        // order rather than the wire pairing: the replacement must occupy the
        // slot of the result it retracts, between its own attempt's output and
        // the next one. Before this fix it sorted by its own seq and landed
        // after c2's result, stranding c1's call.
        let shape = rendered
            .iter()
            .map(|item| {
                let blocks = item["content"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default()
                    .iter()
                    .map(|block| {
                        format!(
                            "{}:{}",
                            block["type"].as_str().unwrap_or("?"),
                            block["call_id"]
                                .as_str()
                                .or(block["text"].as_str())
                                .unwrap_or("")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}[{}]", item["role"].as_str().unwrap_or("?"), blocks)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            shape,
            [
                "user[text:first]",
                "sealed[]",
                "user[text:second]",
                "sealed[]",
                "tool[tool_result:c1]",
                "sealed[]",
                "tool[tool_result:c2]"
            ],
            "the trimmed result keeps its original slot"
        );
        let text = serde_json::to_string(&rendered).unwrap();
        assert!(
            text.contains("first [...] trimmed"),
            "the replacement renders: {text}"
        );
    }

    /// A trim written after a turn boundary retracts a result from the turn
    /// that has already settled. Every turn-bound event carries the latest
    /// opened turn (event constraint 8), so the replacement is stamped with
    /// the turn that writes it, not the one it retracts — the `call` is what
    /// pairs it to the original. A live corpus run hit exactly this at its
    /// first turn boundary.
    #[test]
    fn a_trim_written_in_a_later_turn_is_stamped_with_the_open_turn() {
        let (_directory, mut ledger) = context_ledger("openai_responses_v1", false);
        let big = "y".repeat(12_000);
        for row in [
            json!({"v":1,"seq":12,"turn":2,"kind":"attempt","ts":"2026-08-27T09:00:01.000Z","attempt":"a2","epoch":"e1","wire_digest":"w2","request":{"asset":format!("sha256-{}", "cd".repeat(32)),"bytes":16},"admits":[{"from":9,"to":9}]}),
            json!({"v":1,"seq":13,"turn":2,"kind":"tool_call","ts":"2026-08-27T09:00:01.000Z","attempt":"a2","call":"c9","name":"shell","args":{"command":"cat big"},"source":"provider"}),
            json!({"v":1,"seq":14,"turn":2,"kind":"tool_result","ts":"2026-08-27T09:00:01.000Z","call":"c9","outcome":"ok","content":[{"type":"text","text":format!("HEAD{big}TAIL")}]}),
            json!({"v":1,"seq":15,"turn":2,"kind":"output","ts":"2026-08-27T09:00:01.000Z","attempt":"a2","content":[{"type":"text","text":"read it"}],"sealed":{"version":1,"adapter":"openai_responses_v1","fragments":"[]"},"usage":{"availability":"unavailable"}}),
            // Turn 2 settles and turn 3 opens: the oversized result is now history.
            json!({"v":1,"seq":16,"turn":2,"kind":"settle","ts":"2026-08-27T09:00:02.000Z","outcome":"completed"}),
            json!({"v":1,"seq":17,"kind":"input","ts":"2026-08-27T09:00:02.000Z","content":[{"type":"text","text":"again"}],"origin_key":"i3","origin_tuple":{"principal":"p","client":"cli","target":"t","op":"submit","key":"i3"}}),
            json!({"v":1,"seq":18,"turn":3,"kind":"turn_open","ts":"2026-08-27T09:00:02.000Z","trigger":{"inputs":[17]}}),
        ] {
            ledger
                .append_contract(make_event(row).unwrap(), BarrierContext::default())
                .expect("seed");
        }
        let targets =
            engine::plan_tool_result_trim(&ledger.projection().unwrap().events).expect("plan");
        assert_eq!(
            targets.iter().map(|target| target.seq).collect::<Vec<_>>(),
            vec![14],
            "the settled turn's result is eligible"
        );

        let options = overflow_options(ledger.path());
        append_tool_result_trim(&mut ledger, &options, 3, &targets[0])
            .expect("the replacement is a valid append");

        let projection = ledger.projection().expect("the ledger still folds");
        let replacement = serde_json::to_value(projection.events.last().unwrap().raw()).unwrap();
        assert_eq!(replacement["kind"], "tool_result");
        assert_eq!(
            replacement["turn"], 3,
            "stamped with the turn that wrote it, not turn 2"
        );
        assert_eq!(
            replacement["call"], "c9",
            "the call is what pairs it to the original"
        );
        assert_eq!(replacement["supersedes"], json!([{"from":14,"to":14}]));
        let text = replacement["content"][0]["text"].as_str().expect("text");
        assert!(
            text.starts_with("HEAD")
                && text.ends_with("TAIL")
                && text.contains("durable at seq 14")
        );
        // The original is still on disk and the render still skips it.
        assert_eq!(
            projection
                .events
                .iter()
                .filter(|event| event.seq() == 14)
                .count(),
            1
        );
    }

    #[test]
    fn observed_tokens_prevent_byte_only_compaction_for_a_preserved_prefix() {
        let old = json!({"model":"deepseek-v4-flash","input":[{"role":"user","content":"x".repeat(899_900)}],"tools":[]});
        let mut next = old.clone();
        next["input"]
            .as_array_mut()
            .unwrap()
            .push(json!({"role":"tool","content":"small result"}));
        let old = serde_json::to_vec(&old).unwrap();
        let next = serde_json::to_vec(&next).unwrap();
        assert!(
            next.len() > 900_000,
            "the old byte-only preflight would compact"
        );
        let bound = prefix_preserving_token_bound(&old, 228_000, &next).unwrap();
        assert!(
            bound < 900_000,
            "reported tokens plus a byte-bound delta stay below the trigger"
        );
        assert!(prefix_preserving_token_bound(&old, 895_000, &next).unwrap() > 900_000);

        let mut changed = serde_json::from_slice::<Value>(&next).unwrap();
        changed["input"][0]["content"] = json!("different earlier content");
        assert!(
            prefix_preserving_token_bound(&old, 228_000, &serde_json::to_vec(&changed).unwrap())
                .is_none()
        );
        changed = serde_json::from_slice::<Value>(&next).unwrap();
        changed["tools"] = json!([{"name":"new-tool"}]);
        assert!(
            prefix_preserving_token_bound(&old, 228_000, &serde_json::to_vec(&changed).unwrap())
                .is_none()
        );
    }

    #[test]
    fn observed_token_bound_reads_the_reported_usage_and_saved_request() {
        let (_directory, mut ledger) = context_ledger("deepseek_responses_v1", false);
        let old = json!({"model":"deepseek-v4-flash","input":[{"role":"user","content":"x".repeat(899_900)}],"tools":[]});
        let mut next = old.clone();
        next["input"]
            .as_array_mut()
            .unwrap()
            .push(json!({"role":"tool","content":"small result"}));
        let old = serde_json::to_vec(&old).unwrap();
        let next = serde_json::to_vec(&next).unwrap();
        let folder = ledger.path().parent().unwrap();
        let asset = store::AssetStore::new(folder.join("assets"))
            .unwrap()
            .publish(&old)
            .unwrap();
        for row in [
            json!({"v":1,"seq":12,"turn":2,"kind":"attempt","ts":"2026-08-27T09:00:03.000Z","attempt":"a2","epoch":"e1","wire_digest":"w2","request":{"asset":asset.asset,"bytes":old.len()},"admits":[{"from":9,"to":9}]}),
            json!({"v":1,"seq":13,"turn":2,"kind":"output","ts":"2026-08-27T09:00:03.000Z","attempt":"a2","content":[],"sealed":{"version":1,"adapter":"deepseek_responses_v1","fragments":"[]"},"usage":{"availability":"reported","input_tokens":"228000","output_tokens":"1"}}),
        ] {
            ledger
                .append_contract(make_event(row).unwrap(), BarrierContext::default())
                .unwrap();
        }
        assert!(observed_prefix_token_bound(&ledger, &next, "e1").unwrap() < 900_000);
        assert!(observed_prefix_token_bound(&ledger, &next, "other-epoch").is_none());
    }

    /// D-39: a prepared candidate above `compact_trigger_tokens` compacts and
    /// opens the compaction epoch before the send; the provider then sees one
    /// request (no overflow round trip) and the turn completes.
    #[test]
    fn preflight_trim_epoch_reentry_compaction_preserves_the_open_turn_and_saved_wire() {
        #[derive(Clone, Default)]
        struct Wire(Rc<RefCell<Vec<u8>>>);
        impl Write for Wire {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                self.0.borrow_mut().extend_from_slice(bytes);
                Ok(bytes.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let (directory, mut ledger) = context_ledger("openai_responses_v1", false);
        for row in [
            json!({"v":1,"seq":12,"turn":2,"kind":"attempt","ts":"2026-08-27T09:00:03.000Z","attempt":"a2","epoch":"e1","wire_digest":"w2","request":{"asset":format!("sha256-{}", "cd".repeat(32)),"bytes":16},"admits":[{"from":9,"to":9}]}),
            json!({"v":1,"seq":13,"turn":2,"kind":"tool_call","ts":"2026-08-27T09:00:03.000Z","attempt":"a2","call":"old-call","name":"shell","args":{"command":"cat old"},"source":"provider"}),
            json!({"v":1,"seq":14,"turn":2,"kind":"tool_result","ts":"2026-08-27T09:00:03.000Z","call":"old-call","outcome":"ok","content":[{"type":"text","text":json!({"stdout":"history".repeat(1800),"artifacts":[]}).to_string()}]}),
            json!({"v":1,"seq":15,"turn":2,"kind":"output","ts":"2026-08-27T09:00:03.000Z","attempt":"a2","content":[],"sealed":{"version":1,"adapter":"openai_responses_v1","fragments":"[{\"type\":\"function_call\",\"call_id\":\"old-call\",\"name\":\"shell\",\"arguments\":\"{\\\"command\\\":\\\"cat old\\\"}\"}]"},"usage":{"availability":"unavailable"}}),
            json!({"v":1,"seq":16,"turn":2,"kind":"attempt","ts":"2026-08-27T09:00:03.000Z","attempt":"a3","epoch":"e1","wire_digest":"w3","request":{"asset":format!("sha256-{}", "ef".repeat(32)),"bytes":16},"admits":[{"from":9,"to":9}]}),
            json!({"v":1,"seq":17,"turn":2,"kind":"output","ts":"2026-08-27T09:00:03.000Z","attempt":"a3","content":[{"type":"text","text":"retain current step"}],"sealed":{"version":1,"adapter":"openai_responses_v1","fragments":"[{\"type\":\"message\",\"role\":\"assistant\",\"content\":[{\"type\":\"output_text\",\"text\":\"retain current step\"}]}]"},"usage":{"availability":"unavailable"}}),
        ] {
            ledger
                .append_contract(make_event(row).unwrap(), BarrierContext::default())
                .unwrap();
        }
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        set_provider_test_redirect(Some(format!("http://{}", listener.local_addr().unwrap())));
        let server = scripted_provider_server(listener, vec![
            // Invalid summary artifact exercises the durable deterministic fallback.
            br#"{"id":"summary","output":[],"status":"completed"}"#.to_vec(),
            br#"{"id":"final","output":[{"type":"message","content":[{"type":"output_text","text":"finished"}]}],"status":"completed"}"#.to_vec(),
        ]);
        let profile = overflow_profile_with_trigger(directory.path(), 1);
        let options = overflow_options(ledger.path());
        let mut output = Wire::default();
        let observed = output.clone();
        // Respond to the emitted lease identity. Epochs/trim/compact append
        // records before leasing; a fixture guessing the old seq replies to
        // the wrong attempt and fails before it can exercise the send.
        let mut replies = std::iter::from_fn(move || {
            let bytes = observed.0.borrow();
            let request = bytes
                .split(|byte| *byte == b'\n')
                .rev()
                .filter_map(|line| serde_json::from_slice::<Value>(line).ok())
                .find_map(|row| row.get("lease_request").cloned())
                .expect("worker requested lease");
            Some(Ok(
                json!({"lease":{"attempt":request["attempt"],"granted":true}}).to_string(),
            ))
        });
        let outcome = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut None,
            &mut replies,
            &mut output,
            &RuntimeCancellation::default(),
        );
        set_provider_test_redirect(None);
        outcome.unwrap();
        server.join().unwrap();
        let path = ledger.path().to_owned();
        drop(ledger);
        let ledger = LockedLedger::open(path, 1).unwrap();
        let events = &ledger.projection().unwrap().events;
        let trim = events
            .iter()
            .find(|event| *event.kind() == EventKind::ToolResult && event.has_field("supersedes"))
            .unwrap();
        let trim_epoch = events
            .iter()
            .find(|event| {
                *event.kind() == EventKind::Epoch
                    && event.string_field("reason") == Some("tool_result_trim")
            })
            .unwrap();
        let compact = events
            .iter()
            .find(|event| *event.kind() == EventKind::Compact)
            .unwrap();
        let compact_epoch = events
            .iter()
            .find(|event| {
                *event.kind() == EventKind::Epoch
                    && event.string_field("reason") == Some("compaction")
            })
            .unwrap();
        assert!(
            trim.seq() < trim_epoch.seq()
                && trim_epoch.seq() < compact.seq()
                && compact.seq() < compact_epoch.seq()
        );
        let raw = serde_json::to_value(compact.raw()).unwrap();
        let covered = raw["covers"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|range| range["from"].as_u64().unwrap()..=range["to"].as_u64().unwrap())
            .collect::<BTreeSet<_>>();
        assert!(covered.contains(&13) && covered.contains(&15) && covered.contains(&trim.seq()));
        assert!(!covered.contains(&17) && !covered.contains(&9) && !covered.contains(&2));
        let attempt = events
            .iter()
            .rev()
            .find(|event| *event.kind() == EventKind::Attempt)
            .unwrap();
        let raw = serde_json::to_value(attempt.raw()).unwrap();
        let assets =
            store::AssetStore::new(ledger.path().parent().unwrap().join("assets")).unwrap();
        let bytes = assets
            .read_verified(raw["request"]["asset"].as_str().unwrap())
            .unwrap();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert!(
            body["input"]
                .as_array()
                .unwrap()
                .iter()
                .all(|item| !matches!(
                    item["type"].as_str(),
                    Some("function_call" | "function_call_output")
                ))
        );
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.contains("retain current step") && text.contains("compacted history"));
        assert_eq!(
            events.last().unwrap().string_field("outcome"),
            Some("completed"),
            "{:?}",
            events
                .iter()
                .rev()
                .take(3)
                .map(|event| event.raw())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn preflight_compaction_precedes_the_send_when_the_candidate_exceeds_the_trigger() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let turn = ledger.projection().unwrap().latest_turn.unwrap();
        let plan =
            engine::plan_context_compaction(&ledger.projection().unwrap().events, turn).unwrap();
        let evidence = plan.covers[0];
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        let endpoint = format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        );
        set_provider_test_redirect(Some(endpoint));
        // Order on the wire: the compactor request (before the send, inside the
        // lease), then the one send on the compaction generation.
        let compactor = format!(
            r#"{{"id":"response-summary","output":[{{"type":"function_call","call_id":"call-summary","name":"summary_artifact","arguments":"{{\"continuation\":\"Earlier the user asked for the fixture task.\",\"evidence_refs\":[{evidence}]}}"}}],"status":"completed","usage":{{"input_tokens":9,"output_tokens":3}}}}"#
        );
        let server = scripted_provider_server(listener, vec![
            compactor.into_bytes(),
            br#"{"id":"response-1","output":[{"content":[{"text":"after preflight","type":"output_text"}],"type":"message"}],"status":"completed","usage":{"input_tokens":2,"output_tokens":2}}"#.to_vec(),
        ]);
        // Any real candidate exceeds one token.
        let profile = overflow_profile_with_trigger(directory.path(), 1);
        let options = overflow_options(ledger.path());
        // checkpoint 14, compact 15, epoch 16: the one send is attempt 17.
        let mut replies = [Ok(
            "{\"lease\":{\"attempt\":\"overflow-run-attempt-16\",\"granted\":true}}".to_owned(),
        )]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut None,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        result.expect("preflight compaction turn");
        server.join().expect("provider server");
        let projection = ledger.projection().expect("projection");
        let kinds = projection
            .events
            .iter()
            .skip(12)
            .map(|event| event.kind().as_str().to_owned())
            .collect::<Vec<_>>();
        let compact = kinds
            .iter()
            .position(|kind| kind == "compact")
            .expect("compact before the send");
        let attempt = kinds
            .iter()
            .position(|kind| kind == "attempt")
            .expect("one attempt");
        assert!(
            compact < attempt,
            "compaction precedes the first attempt: {kinds:?}"
        );
        assert_eq!(
            kinds.iter().filter(|kind| *kind == "attempt").count(),
            1,
            "no overflow round trip: {kinds:?}"
        );
        assert!(!kinds.iter().any(|kind| kind == "error"), "{kinds:?}");
        let epoch = projection
            .events
            .iter()
            .rev()
            .find(|event| event.kind().as_str() == "epoch")
            .unwrap();
        assert_eq!(
            serde_json::to_value(epoch.raw()).unwrap()["reason"],
            "compaction"
        );
        let compact = projection
            .events
            .iter()
            .find(|event| event.kind().as_str() == "compact")
            .map(|event| serde_json::to_value(event.raw()).unwrap())
            .unwrap();
        assert_eq!(
            compact["summary_request"]["accepted"], true,
            "{}",
            compact["summary_request"]
        );
        assert_eq!(
            compact["summary"],
            "[compacted history]\nEarlier the user asked for the fixture task."
        );
        assert!(projection.terminal_tail);
        assert_eq!(
            serde_json::to_value(projection.events.last().unwrap().raw()).unwrap()["outcome"],
            "completed"
        );
    }

    #[derive(Default)]
    struct FixtureTavilyTransport {
        materials: Mutex<Vec<String>>,
    }

    impl TavilyTransport for FixtureTavilyTransport {
        fn send(
            &self,
            _endpoint: &str,
            credential: &str,
            request: &SearchRequest,
            _limits: &HttpLimits,
            _cancellation: &CancellationToken,
        ) -> Result<Vec<SearchHit>, BackendFailure> {
            self.materials
                .lock()
                .expect("materials lock")
                .push(credential.to_owned());
            parse_tavily_response(
                include_bytes!("../../../fixtures/web-search-provider/response.canonical.json"),
                request.max_results,
            )
        }
    }

    fn search_scope(material: &str, generation: &str) -> provider::CredentialScope {
        provider::CredentialScope {
            credential_id: "web-search-main".to_owned(),
            adapter: "tavily_v1".to_owned(),
            endpoint_origin: "https://api.tavily.com:443".to_owned(),
            purpose: "web_search".to_owned(),
            generation: generation.to_owned(),
            material: material.to_owned(),
        }
    }

    fn search_request(call_id: &str) -> SearchRequest {
        SearchRequest {
            call_id: call_id.to_owned(),
            query: "durable local agents".to_owned(),
            max_results: 2,
            topic: SearchTopic::General,
        }
    }

    #[test]
    fn tavily_request_and_response_match_the_oracle() {
        let request = search_request("fixture-call");
        let expected =
            include_bytes!("../../../fixtures/web-search-provider/request.canonical.json");
        assert_eq!(
            tavily_request_bytes(&request).expect("request bytes"),
            expected.strip_suffix(b"\n").expect("fixture LF")
        );
        let hits = parse_tavily_response(
            include_bytes!("../../../fixtures/web-search-provider/response.canonical.json"),
            2,
        )
        .expect("response mapping");
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].snippet, "First snippet");
        assert!(parse_tavily_response(br#"{"results":"not-an-array"}"#, 2).is_err());
        assert!(
            parse_tavily_response(
                br#"{"results":[{"content":"x","title":"x","url":"http://127.0.0.1/private"}]}"#,
                2
            )
            .is_err()
        );
        assert_eq!(classify_tavily_status(200), Ok(()));
        assert!(matches!(
            classify_tavily_status(401),
            Err(BackendFailure::Denied(_))
        ));
        assert!(matches!(
            classify_tavily_status(429),
            Err(BackendFailure::Unavailable(_))
        ));
        assert!(matches!(
            classify_tavily_status(301),
            Err(BackendFailure::TerminalUnavailable(_))
        ));
        assert!(matches!(
            resolve_public_search_endpoint("https://[::ffff:127.0.0.1]"),
            Err(BackendFailure::Denied(_))
        ));
        assert!(validate_public_result_url("https://[::ffff:127.0.0.1]/private").is_err());
    }

    #[test]
    fn web_search_resolves_material_per_call_and_observes_rotation() {
        let (supervisor, worker) = std::os::unix::net::UnixStream::pair().expect("socket pair");
        let (control, service) = provider::start_credential_channel(
            supervisor,
            provider::CredentialBroker::new([search_scope("generation-one", "1")]),
        );
        let credential = Arc::new(Mutex::new(CredentialClient::new(worker)));
        let transport = Arc::new(FixtureTavilyTransport::default());
        let search = TavilySearchProvider {
            config: WebSearch {
                adapter: "tavily_v1".to_owned(),
                endpoint: "https://api.tavily.com".to_owned(),
                credential_key: "web-search-main".to_owned(),
            },
            credential: Arc::clone(&credential),
            transport: Arc::clone(&transport) as Arc<dyn TavilyTransport>,
        };
        assert_eq!(
            search
                .search(
                    &search_request("call-one"),
                    &HttpLimits::default(),
                    &CancellationToken::default()
                )
                .expect("first search")
                .len(),
            2
        );
        control
            .rotate("web-search-main", "2", "generation-two")
            .expect("rotate");
        search
            .search(
                &search_request("call-two"),
                &HttpLimits::default(),
                &CancellationToken::default(),
            )
            .expect("rotated search");
        assert_eq!(
            *transport.materials.lock().expect("materials"),
            ["generation-one", "generation-two"]
        );
        drop(search);
        drop(credential);
        drop(control);
        service.join().expect("broker thread").expect("broker EOF");
    }

    #[test]
    fn web_search_revocation_reaches_the_next_call() {
        let (supervisor, worker) = std::os::unix::net::UnixStream::pair().expect("socket pair");
        let (control, service) = provider::start_credential_channel(
            supervisor,
            provider::CredentialBroker::new([search_scope("generation-one", "1")]),
        );
        let credential = Arc::new(Mutex::new(CredentialClient::new(worker)));
        let transport = Arc::new(FixtureTavilyTransport::default());
        let search = TavilySearchProvider {
            config: WebSearch {
                adapter: "tavily_v1".to_owned(),
                endpoint: "https://api.tavily.com".to_owned(),
                credential_key: "web-search-main".to_owned(),
            },
            credential: Arc::clone(&credential),
            transport: Arc::clone(&transport) as Arc<dyn TavilyTransport>,
        };
        search
            .search(
                &search_request("call-one"),
                &HttpLimits::default(),
                &CancellationToken::default(),
            )
            .expect("first search");
        control.revoke("web-search-main", "2").expect("revoke");
        assert!(matches!(
            search.search(
                &search_request("call-revoked"),
                &HttpLimits::default(),
                &CancellationToken::default(),
            ),
            Err(BackendFailure::Unavailable(detail)) if detail == "web-search credential revoked"
        ));
        assert_eq!(
            *transport.materials.lock().expect("materials"),
            ["generation-one"],
            "no send happens with revoked material"
        );
        drop(search);
        drop(credential);
        drop(control);
        service.join().expect("broker thread").expect("broker EOF");
    }

    #[test]
    fn slice14e_gate_111_web_search_exact_wire_and_negative_matrix() {
        tavily_request_and_response_match_the_oracle();
        let invalid: serde_json::Value = serde_json::from_slice(include_bytes!(
            "../../../fixtures/web-search-provider/invalid-responses.canonical.json"
        ))
        .expect("invalid-response fixture");
        for case in invalid["cases"].as_array().expect("cases") {
            let bytes =
                serde_json_canonicalizer::to_vec(&case["body"]).expect("canonical invalid body");
            assert!(
                parse_tavily_response(&bytes, 2).is_err(),
                "accepted invalid case {}",
                case["id"]
            );
        }
        for status in 200..=299 {
            assert_eq!(classify_tavily_status(status), Ok(()));
        }
        for status in [401, 403] {
            assert!(matches!(
                classify_tavily_status(status),
                Err(BackendFailure::Denied(_))
            ));
        }
        for status in [429, 500, 503, 599] {
            assert!(matches!(
                classify_tavily_status(status),
                Err(BackendFailure::Unavailable(_))
            ));
        }
    }

    #[test]
    fn slice14e_gate_112_web_parity_retirement_and_secret_lifecycle() {
        let parity: serde_json::Value = serde_json::from_slice(include_bytes!(
            "../../../fixtures/web-tools/parity.canonical.json"
        ))
        .expect("parity fixture");
        assert_eq!(parity["rows"].as_array().expect("rows").len(), 6);
        assert_eq!(
            parity["rows"]
                .as_array()
                .expect("rows")
                .iter()
                .filter(|row| row["disposition"] == "permanently_retired")
                .count(),
            4
        );
        web_search_resolves_material_per_call_and_observes_rotation();
        web_search_revocation_reaches_the_next_call();
    }

    /// (profile, credential client, broker control, broker thread)
    type EagerResponsesProfile = (
        RuntimeProfile,
        Option<Arc<Mutex<CredentialClient>>>,
        provider::CredentialBrokerControl,
        std::thread::JoinHandle<Result<(), provider::CredentialClientError>>,
    );

    /// Responses profile with a credential broker, for scripted-provider turns.
    fn eager_responses_profile(
        root: &std::path::Path,
        allowed_tools: &[&str],
    ) -> EagerResponsesProfile {
        eager_responses_profile_for(root, allowed_tools, "openai_responses_v1")
    }

    /// A Responses-family fixture provider for the named dialect, with a
    /// credential scope bound to that provider's endpoint origin.
    fn eager_responses_profile_for(
        root: &std::path::Path,
        allowed_tools: &[&str],
        dialect: &str,
    ) -> EagerResponsesProfile {
        let (endpoint_owner, endpoint, origin, model, evidence_revision) = match dialect {
            "deepseek_responses_v1" => (
                "deepseek",
                "https://api.deepseek.com",
                "https://api.deepseek.com:443",
                "deepseek-v4-flash",
                "deepseek-direct-responses-v4-flash-2026-07-31+function-json-schema-strict-v1",
            ),
            "openai_responses_v1" => (
                "openai",
                "https://api.openai.com/v1",
                "https://api.openai.com:443",
                "gpt-5",
                "openai-2026-08-01",
            ),
            other => panic!("no eager Responses fixture for dialect {other}"),
        };
        let mut profile = tool_profile(root, allowed_tools);
        profile.config.workspace.policy.provider = Some("fixture".to_owned());
        profile.config.workspace.policy.model = Some(model.to_owned());
        profile.config.providers = ProvidersConfig {
            format: 1,
            revision: 1,
            providers: vec![Provider {
                id: "fixture".to_owned(),
                name: None,
                adapter: "responses".to_owned(),
                dialect: dialect.to_owned(),
                endpoint_owner: endpoint_owner.to_owned(),
                gateway_translation: "direct".to_owned(),
                evidence_revision: evidence_revision.to_owned(),
                endpoint: endpoint.to_owned(),
                credential_key: Some("provider-main".to_owned()),
                models: vec![Model {
                    id: model.to_owned(),
                    profile: format!("{dialect}:{model}"),
                    enabled: true,
                    context_window_tokens: 100_000,
                    compact_trigger_tokens: 100_000,
                }],
            }],
            web_search: None,
        };
        profile.config.revisions.providers = 1;
        let (supervisor, worker) = std::os::unix::net::UnixStream::pair().expect("socket pair");
        let (control, service) = provider::start_credential_channel(
            supervisor,
            provider::CredentialBroker::new([provider::CredentialScope {
                credential_id: "provider-main".to_owned(),
                adapter: "responses".to_owned(),
                endpoint_origin: origin.to_owned(),
                purpose: "provider".to_owned(),
                generation: "1".to_owned(),
                material: "fixture-secret".to_owned(),
            }]),
        );
        let credential = Some(Arc::new(Mutex::new(CredentialClient::new(worker))));
        (profile, credential, control, service)
    }

    fn read_http_request(stream: &mut std::net::TcpStream) -> Vec<u8> {
        let mut request = Vec::new();
        let mut chunk = [0_u8; 4096];
        let mut expected = None;
        loop {
            let count = stream.read(&mut chunk).expect("provider request");
            request.extend_from_slice(&chunk[..count]);
            if expected.is_none() {
                if let Some(end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request[..end + 4]);
                    let length = headers
                        .lines()
                        .find_map(|line| {
                            let (name, value) = line.split_once(':')?;
                            name.eq_ignore_ascii_case("content-length")
                                .then(|| value.trim().parse::<usize>().ok())
                                .flatten()
                        })
                        .unwrap_or(0);
                    expected = Some(end + 4 + length);
                }
            }
            if count == 0 || expected.is_some_and(|length| request.len() >= length) {
                break;
            }
        }
        request
    }

    fn ledger_file_contains(path: &std::path::Path, needle: &str, timeout: Duration) -> bool {
        let started = Instant::now();
        while started.elapsed() < timeout {
            if fs::read_to_string(path).is_ok_and(|bytes| bytes.contains(needle)) {
                return true;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        false
    }

    const EAGER_TOOL_SSE_PREFIX: &str = concat!(
        "data: {\"type\":\"response.output_item.added\",\"output_index\":0,\"item\":{\"type\":\"function_call\",\"call_id\":\"call-think\",\"name\":\"think\",\"arguments\":\"\"}}\n\n",
        "data: {\"type\":\"response.function_call_arguments.delta\",\"output_index\":0,\"delta\":\"{\\\"thought\\\":\\\"inspect\\\"}\"}\n\n",
        "data: {\"type\":\"response.function_call_arguments.done\",\"output_index\":0,\"arguments\":\"{\\\"thought\\\":\\\"inspect\\\"}\"}\n\n",
    );
    const EAGER_TOOL_SSE_TERMINAL: &str = "data: {\"type\":\"response.completed\",\"response\":{\"id\":\"response-tool\",\"status\":\"completed\",\"output\":[{\"type\":\"function_call\",\"call_id\":\"call-think\",\"name\":\"think\",\"arguments\":\"{\\\"thought\\\":\\\"inspect\\\"}\"}],\"usage\":{\"input_tokens\":2,\"output_tokens\":1}}}\n\n";
    const FINAL_JSON_RESPONSE: &[u8] = br#"{"id":"response-done","output":[{"content":[{"text":"done","type":"output_text"}],"type":"message"}],"status":"completed","usage":{"input_tokens":2,"output_tokens":1}}"#;

    /// The response terminal bytes are withheld until the eagerly dispatched
    /// call's result is durable: execution demonstrably precedes the terminal,
    /// the terminal then reuses the call without a second record or result,
    /// and the continuation request follows the whole batch.
    #[test]
    fn eager_dispatch_executes_ready_call_before_response_terminal_bytes() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let ledger_path = ledger.path().to_owned();
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        set_provider_test_redirect(Some(format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        )));
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("provider accept");
            let _ = read_http_request(&mut stream);
            let body_len = EAGER_TOOL_SSE_PREFIX.len() + EAGER_TOOL_SSE_TERMINAL.len();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\ncontent-length: {body_len}\r\nconnection: close\r\n\r\n"
            )
            .expect("provider header");
            stream
                .write_all(EAGER_TOOL_SSE_PREFIX.as_bytes())
                .expect("provider prefix");
            stream.flush().expect("provider flush");
            // Canonical key order: a tool_result line reads `"call":…,"content":…`
            // while the tool_call line reads `"call":…,"kind":"tool_call"`.
            let executed = ledger_file_contains(
                &ledger_path,
                "\"call\":\"call-think\",\"content\"",
                Duration::from_secs(10),
            );
            stream
                .write_all(EAGER_TOOL_SSE_TERMINAL.as_bytes())
                .expect("provider terminal");
            let (mut stream, _) = listener.accept().expect("continuation accept");
            let _ = read_http_request(&mut stream);
            write!(
                stream,
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                FINAL_JSON_RESPONSE.len()
            )
            .expect("continuation header");
            stream
                .write_all(FINAL_JSON_RESPONSE)
                .expect("continuation body");
            executed
        });
        let (profile, mut credential, control, service) =
            eager_responses_profile(directory.path(), &["think"]);
        let options = test_options(&ledger);
        let mut replies = [
            Ok("{\"lease\":{\"attempt\":\"tool-run-attempt-13\",\"granted\":true}}".to_owned()),
            Ok("{\"lease\":{\"attempt\":\"tool-run-attempt-19\",\"granted\":true}}".to_owned()),
        ]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut credential,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        result.expect("eager tool continuation");
        let executed_before_terminal = server.join().expect("provider server");
        drop(credential);
        drop(control);
        service.join().expect("broker thread").expect("broker EOF");

        assert!(
            executed_before_terminal,
            "the tool result must be durable before the response terminal bytes are released"
        );
        let events = &ledger.projection().expect("projection").events;
        let seq_of = |kind: EventKind, field: &str, value: &str| {
            events
                .iter()
                .filter(|event| *event.kind() == kind && event.string_field(field) == Some(value))
                .map(Event::seq)
                .collect::<Vec<_>>()
        };
        let calls = seq_of(EventKind::ToolCall, "call", "call-think");
        let results = seq_of(EventKind::ToolResult, "call", "call-think");
        let outputs = seq_of(EventKind::Output, "attempt", "tool-run-attempt-13");
        assert_eq!(
            calls.len(),
            1,
            "the terminal must not write the eager call again"
        );
        assert_eq!(results.len(), 1, "exactly one result per call");
        assert_eq!(outputs.len(), 1);
        assert!(calls[0] < results[0], "write-ahead precedes execution");
        assert!(
            results[0] < outputs[0],
            "eager result precedes the response terminal record"
        );
        let next_attempt = events
            .iter()
            .find(|event| *event.kind() == EventKind::Attempt && event.seq() > outputs[0])
            .expect("continuation attempt")
            .seq();
        assert!(
            next_attempt > outputs[0],
            "one continuation, after the whole batch and the terminal joined"
        );
        let settle = events.last().expect("settle");
        assert_eq!(settle.kind(), &EventKind::Settle);
        assert_eq!(settle.string_field("outcome"), Some("completed"));

        // Pipe order: the ready marker (stamped with the pre-append ledger
        // high-water) leaves before the durable call, and the doorbell follows.
        let lines = String::from_utf8(output).expect("worker stdout");
        let mut ready_index = None;
        let mut doorbell_after_ready = false;
        for (index, line) in lines.lines().enumerate() {
            let value: Value = serde_json::from_str(line).expect("worker message");
            if let Some(frame) = value.get("frame") {
                if frame["arguments_complete"] == true {
                    assert_eq!(frame["call_id"], "call-think");
                    let stamped = frame["ledger_seq"].as_u64().expect("ledger_seq stamp");
                    assert!(
                        stamped < calls[0],
                        "ready frame precedes the write-ahead call"
                    );
                    ready_index = Some(index);
                }
            } else if value.get("appended").is_some()
                && ready_index.is_some_and(|ready| index > ready)
            {
                doorbell_after_ready = true;
            }
        }
        assert!(
            ready_index.is_some(),
            "argument completion marker was forwarded"
        );
        assert!(
            doorbell_after_ready,
            "eager records ring the doorbell after the marker"
        );
    }

    /// A DeepSeek thinking-mode stream: a completed `reasoning` item, then a
    /// `function_call` whose arguments complete (eager dispatch), with the
    /// connection lost before the response terminal.
    const PARTIAL_REASONING_TOOL_SSE: &str = concat!(
        "data: {\"type\":\"response.output_item.added\",\"output_index\":0,\"item\":{\"type\":\"reasoning\",\"id\":\"rs-1\",\"summary\":[],\"content\":[],\"status\":\"in_progress\"}}\n\n",
        "data: {\"type\":\"response.reasoning_text.delta\",\"output_index\":0,\"content_index\":0,\"delta\":\"Need to inspect first.\"}\n\n",
        "data: {\"type\":\"response.output_item.done\",\"output_index\":0,\"item\":{\"type\":\"reasoning\",\"id\":\"rs-1\",\"summary\":[],\"content\":[{\"type\":\"reasoning_text\",\"text\":\"Need to inspect first.\"}],\"encrypted_content\":\"enc-rs-1\",\"status\":\"completed\"}}\n\n",
        "data: {\"type\":\"response.output_item.added\",\"output_index\":1,\"item\":{\"type\":\"function_call\",\"id\":\"fc-1\",\"call_id\":\"call-think\",\"name\":\"think\",\"arguments\":\"\"}}\n\n",
        "data: {\"type\":\"response.function_call_arguments.delta\",\"output_index\":1,\"delta\":\"{\\\"thought\\\":\\\"inspect\\\"}\"}\n\n",
        "data: {\"type\":\"response.function_call_arguments.done\",\"output_index\":1,\"arguments\":\"{\\\"thought\\\":\\\"inspect\\\"}\"}\n\n",
        "data: {\"type\":\"response.output_item.done\",\"output_index\":1,\"item\":{\"type\":\"function_call\",\"id\":\"fc-1\",\"call_id\":\"call-think\",\"name\":\"think\",\"arguments\":\"{\\\"thought\\\":\\\"inspect\\\"}\",\"status\":\"completed\"}}\n\n",
    );

    /// Evidence: DeepSeek streamed a `reasoning` item and a `shell` call, the
    /// worker executed the call eagerly, then the connection was reset before
    /// the response terminal. The retry replayed the call's result behind the
    /// normalized `tool_call` only, and DeepSeek rejected it (`http_400: The
    /// reasoning_text in the thinking mode must be passed back to the API`),
    /// settling the turn as a provider error. The recoverable transport error
    /// now seals the completed native items, and the retry replays
    /// `reasoning` then `function_call` verbatim, then the result.
    #[test]
    fn transport_loss_after_eager_dispatch_seals_completed_items_for_the_retry() {
        let (directory, mut ledger) = context_ledger("deepseek_responses_v1", false);
        let ledger_path = ledger.path().to_owned();
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        set_provider_test_redirect(Some(format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        )));
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("provider accept");
            let _ = read_http_request(&mut stream);
            // Declare more body than is ever delivered: the close below is a
            // transport loss, not a clean end of the response.
            let body_len = PARTIAL_REASONING_TOOL_SSE.len() + 4096;
            write!(
                stream,
                "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\ncontent-length: {body_len}\r\nconnection: close\r\n\r\n"
            )
            .expect("provider header");
            stream
                .write_all(PARTIAL_REASONING_TOOL_SSE.as_bytes())
                .expect("provider partial body");
            stream.flush().expect("provider flush");
            let executed = ledger_file_contains(
                &ledger_path,
                "\"call\":\"call-think\",\"content\"",
                Duration::from_secs(10),
            );
            stream
                .shutdown(std::net::Shutdown::Both)
                .expect("drop the connection before the terminal");
            drop(stream);
            let (mut stream, _) = listener.accept().expect("retry accept");
            let retry_request = read_http_request(&mut stream);
            write!(
                stream,
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                FINAL_JSON_RESPONSE.len()
            )
            .expect("retry header");
            stream.write_all(FINAL_JSON_RESPONSE).expect("retry body");
            (executed, retry_request)
        });
        let (profile, mut credential, control, service) =
            eager_responses_profile_for(directory.path(), &["think"], "deepseek_responses_v1");
        let options = test_options(&ledger);
        // An attempt is named after the ledger seq it will occupy, and the
        // lease is requested before it is appended: grant whatever attempt
        // the ledger's current length implies.
        let lease_ledger = ledger.path().to_owned();
        let mut leased = Vec::new();
        let mut replies = std::iter::from_fn(|| {
            let lines = fs::read_to_string(&lease_ledger)
                .expect("ledger bytes")
                .lines()
                .count();
            let attempt = format!("tool-run-attempt-{}", lines + 1);
            leased.push(attempt.clone());
            Some(Ok(format!(
                "{{\"lease\":{{\"attempt\":\"{attempt}\",\"granted\":true}}}}"
            )))
        });
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut credential,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        result.expect("retry after transport loss");
        let (executed_before_loss, retry_request) = server.join().expect("provider server");
        drop(credential);
        drop(control);
        service.join().expect("broker thread").expect("broker EOF");
        assert!(
            executed_before_loss,
            "the eager call executed before the transport loss"
        );
        let [lost_attempt, _retry_attempt] = leased.as_slice() else {
            panic!("one lost attempt and one retry, got {leased:?}");
        };
        let lost_attempt = lost_attempt.as_str();

        let events = &ledger.projection().expect("projection").events;
        let error = events
            .iter()
            .find(|event| {
                *event.kind() == EventKind::Error
                    && event.string_field("attempt") == Some(lost_attempt)
            })
            .expect("recoverable transport error for the lost attempt");
        assert_eq!(error.string_field("classification"), Some("transport"));
        let raw = serde_json::to_value(error.raw()).expect("error value");
        assert_eq!(raw["recoverable"], true);
        assert_eq!(raw["sealed"]["version"], 1);
        assert_eq!(raw["sealed"]["adapter"], "deepseek_responses_v1");
        let fragments: Value = serde_json::from_str(
            &materialize_string(&ledger, &raw["sealed"]["fragments"]).expect("fragments"),
        )
        .expect("fragments array");
        let types = fragments
            .as_array()
            .expect("fragments array")
            .iter()
            .map(|item| item["type"].as_str().unwrap_or_default().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(types, ["reasoning", "function_call"]);
        assert!(
            !events.iter().any(|event| {
                *event.kind() == EventKind::Output
                    && event.string_field("attempt") == Some(lost_attempt)
            }),
            "a lost response has no output record"
        );
        assert_eq!(
            events
                .iter()
                .filter(|event| *event.kind() == EventKind::ToolCall)
                .count(),
            1,
            "the retry never re-records the eager call"
        );
        let settle = events.last().expect("settle");
        assert_eq!(settle.kind(), &EventKind::Settle);
        assert_eq!(settle.string_field("outcome"), Some("completed"));

        // The retry's wire body: reasoning (with its encrypted content and
        // text) precedes the function_call, which precedes its output, and
        // the reasoning never degrades into assistant output_text.
        let body_start = retry_request
            .windows(4)
            .position(|bytes| bytes == b"\r\n\r\n")
            .expect("retry request body")
            + 4;
        let body: Value =
            serde_json::from_slice(&retry_request[body_start..]).expect("retry body JSON");
        let input = body["input"].as_array().expect("retry input array");
        let position = |predicate: &dyn Fn(&Value) -> bool| input.iter().position(predicate);
        let reasoning = position(&|item| item["type"] == "reasoning" && item["id"] == "rs-1")
            .expect("retry replays the reasoning item");
        assert_eq!(input[reasoning]["encrypted_content"], "enc-rs-1");
        assert_eq!(input[reasoning]["content"][0]["type"], "reasoning_text");
        assert_eq!(
            input[reasoning]["content"][0]["text"],
            "Need to inspect first."
        );
        let calls = input
            .iter()
            .filter(|item| item["type"] == "function_call" && item["call_id"] == "call-think")
            .count();
        assert_eq!(
            calls, 1,
            "the sealed call is not doubled by the eager tool_call record: {input:?}"
        );
        let call =
            position(&|item| item["type"] == "function_call" && item["call_id"] == "call-think")
                .expect("retry replays the function_call");
        assert_eq!(input[call]["arguments"], "{\"thought\":\"inspect\"}");
        let output = position(&|item| {
            item["type"] == "function_call_output" && item["call_id"] == "call-think"
        })
        .expect("retry replays the function_call_output");
        assert!(reasoning < call && call < output, "order: {input:?}");
        assert!(
            !input.iter().any(|item| {
                item["type"] == "message"
                    && item["content"].as_array().is_some_and(|parts| {
                        parts.iter().any(|part| {
                            part["type"] == "output_text"
                                && part["text"]
                                    .as_str()
                                    .is_some_and(|text| text.contains("Need to inspect first."))
                        })
                    })
            }),
            "reasoning must not be replayed as assistant output_text: {input:?}"
        );
    }

    /// A parked eager call suspends further execution for the response without
    /// cancelling it: the terminal still lands with the whole batch, the second
    /// call stays a durable unpaired intent, and the resumed run pairs both once.
    #[test]
    fn eager_dispatch_park_suspends_execution_and_resume_pairs_the_whole_batch() {
        eager_dispatch_hold_case(false);
    }

    #[test]
    fn inline_approval_before_provider_terminal_continues_the_batch_without_worker_exit() {
        eager_dispatch_hold_case(true);
    }

    fn eager_dispatch_hold_case(inline: bool) {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", true);
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        set_provider_test_redirect(Some(format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        )));
        let body = concat!(
            "data: {\"type\":\"response.output_item.added\",\"output_index\":0,\"item\":{\"type\":\"function_call\",\"call_id\":\"call-hold\",\"name\":\"ask_user_questions\",\"arguments\":\"\"}}\n\n",
            "data: {\"type\":\"response.function_call_arguments.done\",\"output_index\":0,\"arguments\":\"{\\\"question\\\":\\\"Continue?\\\",\\\"options\\\":null}\"}\n\n",
            "data: {\"type\":\"response.output_item.added\",\"output_index\":1,\"item\":{\"type\":\"function_call\",\"call_id\":\"call-think\",\"name\":\"think\",\"arguments\":\"\"}}\n\n",
            "data: {\"type\":\"response.function_call_arguments.done\",\"output_index\":1,\"arguments\":\"{\\\"thought\\\":\\\"after hold\\\"}\"}\n\n",
            "data: {\"type\":\"response.completed\",\"response\":{\"id\":\"response-hold\",\"status\":\"completed\",\"output\":[{\"type\":\"function_call\",\"call_id\":\"call-hold\",\"name\":\"ask_user_questions\",\"arguments\":\"{\\\"question\\\":\\\"Continue?\\\",\\\"options\\\":null}\"},{\"type\":\"function_call\",\"call_id\":\"call-think\",\"name\":\"think\",\"arguments\":\"{\\\"thought\\\":\\\"after hold\\\"}\"}],\"usage\":{\"input_tokens\":2,\"output_tokens\":1}}}\n\n",
        );
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("provider accept");
            let _ = read_http_request(&mut stream);
            write!(
                stream,
                "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                body.len()
            )
            .expect("provider header");
            stream.write_all(body.as_bytes()).expect("provider body");
            drop(stream);
            if inline {
                listener.set_nonblocking(true).unwrap();
                let deadline = Instant::now() + Duration::from_secs(5);
                let (mut stream, _) = loop {
                    match listener.accept() {
                        Ok(connection) => break connection,
                        Err(error)
                            if error.kind() == io::ErrorKind::WouldBlock
                                && Instant::now() < deadline =>
                        {
                            std::thread::sleep(Duration::from_millis(10));
                        }
                        Err(error) => panic!("inline approval did not continue: {error}"),
                    }
                };
                stream.set_nonblocking(false).unwrap();
                let _ = read_http_request(&mut stream);
                let body = br#"{"id":"after-inline","output":[{"type":"message","content":[{"type":"output_text","text":"done"}]}],"status":"completed","usage":{"input_tokens":2,"output_tokens":1}}"#;
                write!(stream,"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",body.len()).unwrap();
                stream.write_all(body).unwrap();
            }
        });
        let (profile, mut credential, control, service) =
            eager_responses_profile(directory.path(), &["ask_user_questions", "think"]);
        let options = test_options(&ledger);
        #[derive(Clone)]
        struct InlineReplyWriter {
            bytes: Rc<RefCell<Vec<u8>>>,
            path: PathBuf,
            cancellation: RuntimeCancellation,
            inline: bool,
            delivered: bool,
        }
        impl Write for InlineReplyWriter {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                self.bytes.borrow_mut().extend_from_slice(bytes);
                if self.inline && !self.delivered {
                    let tail = fs::read_to_string(&self.path)?;
                    let event: Value = serde_json::from_str(tail.lines().last().unwrap()).unwrap();
                    if event["kind"] == "approval_request" {
                        self.delivered = true;
                        self.cancellation.park_delivery(json!({"approval_response":{
                            "delivery":"inline-answer","call":"call-hold","grant":true,"answer":"yes",
                            "origin":{"principal":"p","client":"cli","target":"t","op":"answer","key":"inline-answer"}
                        }}).to_string());
                    }
                }
                Ok(bytes.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let cancellation = RuntimeCancellation::default();
        let mut output = InlineReplyWriter {
            bytes: Rc::new(RefCell::new(Vec::new())),
            path: ledger.path().to_owned(),
            cancellation: cancellation.clone(),
            inline,
            delivered: false,
        };
        let wire = Rc::clone(&output.bytes);
        let mut replies = std::iter::from_fn(move || {
            let bytes = wire.borrow();
            let request = bytes
                .split(|byte| *byte == b'\n')
                .rev()
                .filter_map(|line| serde_json::from_slice::<Value>(line).ok())
                .find_map(|row| row.get("lease_request").cloned())
                .expect("lease request");
            Some(Ok(
                json!({"lease":{"attempt":request["attempt"],"granted":true}}).to_string(),
            ))
        });
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut credential,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        result.expect("parked eager turn");
        server.join().expect("provider server");

        if inline {
            drop(credential);
            drop(control);
            service.join().unwrap().unwrap();
            let events = &ledger.projection().unwrap().events;
            assert_eq!(
                events.last().unwrap().string_field("outcome"),
                Some("completed")
            );
            let answer = events
                .iter()
                .find(|event| *event.kind() == EventKind::ApprovalResponse)
                .unwrap()
                .seq();
            let terminal = events
                .iter()
                .find(|event| {
                    *event.kind() == EventKind::Output
                        && event.string_field("attempt") == Some("tool-run-attempt-13")
                })
                .unwrap()
                .seq();
            assert!(
                answer < terminal,
                "answer must arrive before provider terminal"
            );
            for call in ["call-hold", "call-think"] {
                assert_eq!(
                    events
                        .iter()
                        .filter(|event| *event.kind() == EventKind::ToolResult
                            && event.string_field("call") == Some(call))
                        .count(),
                    1
                );
            }
            assert!(pending_tool_calls(&ledger).unwrap().is_empty());
            return;
        }
        let projection = ledger.projection().expect("projection");
        let events = &projection.events;
        assert!(
            projection.lifecycle.open_hold,
            "the eager hold parked the turn"
        );
        assert!(
            events
                .iter()
                .any(|event| *event.kind() == EventKind::ApprovalRequest
                    && event.string_field("call") == Some("call-hold")),
            "the hold marker is durable"
        );
        assert!(
            events.iter().any(|event| *event.kind() == EventKind::Output
                && event.string_field("attempt") == Some("tool-run-attempt-13")),
            "the response terminal still lands after the park"
        );
        let think_calls = events
            .iter()
            .filter(|event| {
                *event.kind() == EventKind::ToolCall
                    && event.string_field("call") == Some("call-think")
            })
            .count();
        let think_results = events
            .iter()
            .filter(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("call-think")
            })
            .count();
        assert_eq!(
            think_calls, 1,
            "the terminal appends the not-yet-durable call once"
        );
        assert_eq!(
            think_results, 0,
            "no call executes after the park in this run"
        );
        assert!(
            events
                .iter()
                .all(|event| *event.kind() != EventKind::Settle || event.turn() != Some(2))
        );

        // Resume: the answer pairs the hold; the remaining call executes once.
        let response = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":2,"kind":"approval_response",
            "ts":options.timestamp,"call":"call-hold","grant":true,"answer":"yes",
            "origin_key":"answer-eager","origin_tuple":{
                "principal":"p","client":"cli","target":"t","op":"answer","key":"answer-eager"
            }
        }))
        .expect("answer event");
        ledger
            .append_contract(response, BarrierContext::default())
            .expect("append answer");
        let mut no_replies = std::iter::empty();
        assert!(
            !recover_unpaired_tool_calls(
                &mut ledger,
                &options,
                Some(&profile),
                Selected { version: 2 },
                false,
                &mut no_replies,
                &mut output,
                &cancellation,
            )
            .expect("resume pairs the batch")
        );
        drop(credential);
        drop(control);
        service.join().expect("broker thread").expect("broker EOF");
        let events = &ledger.projection().expect("projection").events;
        for call in ["call-hold", "call-think"] {
            let results = events
                .iter()
                .filter(|event| {
                    *event.kind() == EventKind::ToolResult
                        && event.string_field("call") == Some(call)
                })
                .count();
            assert_eq!(
                results, 1,
                "{call} pairs exactly once across park and resume"
            );
        }
        assert!(pending_tool_calls(&ledger).expect("pending").is_empty());
    }

    /// The terminal may name an eagerly dispatched call again only with the
    /// same identity, name and materialized arguments, and must name all of
    /// them; any other durable id is a reuse.
    #[test]
    fn response_terminal_reuse_of_eager_call_requires_identical_facts() {
        let (_directory, mut ledger) = context_ledger("openai_responses_v1", true);
        append_test_attempt(&mut ledger, "older-attempt");
        append_test_call(
            &mut ledger,
            "older-attempt",
            "call-old",
            "think",
            json!({"thought":"old"}),
        );
        append_test_attempt(&mut ledger, "eager-attempt");
        let arguments = json!({"thought":"x".repeat(20_000)});
        let options = test_options(&ledger);
        append_provider_tool_call(
            &mut ledger,
            &options,
            &BuiltinManifest::compiled(),
            2,
            "eager-attempt",
            &provider::ToolCall {
                call_id: "call-a".to_owned(),
                name: "think".to_owned(),
                arguments: arguments.clone(),
            },
        )
        .expect("eager write-ahead spills large arguments");
        let eager = EagerDispatch {
            wire_ids: Default::default(),
            calls: vec![provider::ToolCall {
                call_id: "call-a".to_owned(),
                name: "think".to_owned(),
                arguments: arguments.clone(),
            }],
            suspended: false,
        };
        let call = |id: &str, name: &str, arguments: Value| provider::ToolCall {
            call_id: id.to_owned(),
            name: name.to_owned(),
            arguments,
        };
        let validate = |calls: &[provider::ToolCall]| {
            validate_terminal_tool_calls(&ledger, "eager-attempt", &eager, calls)
                .map_err(|error| error.to_string())
        };
        assert!(
            ledger
                .projection()
                .unwrap()
                .events
                .iter()
                .any(|event| *event.kind() == EventKind::ToolCall
                    && event.string_field("call") == Some("call-a")
                    && serde_json::to_value(event.raw()).unwrap()["args"]
                        .get("$spill")
                        .is_some()),
            "the eager call's arguments were spilled, so the comparison materializes them"
        );
        validate(&[
            call("call-a", "think", arguments.clone()),
            call("call-b", "think", json!({"thought":"new"})),
        ])
        .expect("exact same-attempt reuse plus a new call");
        let changed =
            validate(&[call("call-a", "think", json!({"thought":"changed"}))]).unwrap_err();
        assert!(
            changed.contains("changed eagerly dispatched call call-a"),
            "{changed}"
        );
        let renamed = validate(&[call("call-a", "plan", arguments.clone())]).unwrap_err();
        assert!(
            renamed.contains("changed eagerly dispatched call call-a"),
            "{renamed}"
        );
        let omitted = validate(&[call("call-b", "think", json!({"thought":"new"}))]).unwrap_err();
        assert!(
            omitted.contains("omitted eagerly dispatched call call-a"),
            "{omitted}"
        );
        let empty = validate(&[]).unwrap_err();
        assert!(
            empty.contains("omitted eagerly dispatched call call-a"),
            "{empty}"
        );
        let cross = validate(&[
            call("call-a", "think", arguments.clone()),
            call("call-old", "think", json!({"thought":"old"})),
        ])
        .unwrap_err();
        assert!(cross.contains("call-old reuses a durable call"), "{cross}");
        let duplicated = validate(&[
            call("call-a", "think", arguments.clone()),
            call("call-a", "think", arguments.clone()),
        ])
        .unwrap_err();
        assert!(
            duplicated.contains("call-a is empty or duplicated"),
            "{duplicated}"
        );
        let blank = validate(&[
            call("call-a", "think", arguments.clone()),
            call("", "think", json!({})),
        ])
        .unwrap_err();
        assert!(blank.contains("is empty or duplicated"), "{blank}");
        // A durable id of this attempt that eager dispatch did not produce is
        // still a reuse: only the eager set may be named again.
        let stranger = EagerDispatch::default();
        let reused = validate_terminal_tool_calls(
            &ledger,
            "eager-attempt",
            &stranger,
            &[call("call-a", "think", arguments)],
        )
        .map_err(|error| error.to_string())
        .unwrap_err();
        assert!(reused.contains("call-a reuses a durable call"), "{reused}");
    }

    /// Interruption after the eager write-ahead and before its result: the
    /// resumed run pairs the call exactly once through the ordinary unpaired
    /// recovery, then closes the attempt that never reached its terminal.
    #[test]
    fn eager_call_interrupted_before_result_is_recovered_once_and_attempt_closes() {
        let (directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let profile = tool_profile(directory.path(), &["think"]);
        let options = test_options(&ledger);
        append_test_attempt(&mut ledger, "eager-attempt");
        append_test_call(
            &mut ledger,
            "eager-attempt",
            "call-eager",
            "think",
            json!({"thought":"crashed"}),
        );
        let unresolved = unresolved_attempt(&ledger)
            .expect("scan")
            .expect("attempt is live");
        assert_eq!(unresolved.id, "eager-attempt");
        assert!(unresolved.calls.contains("call-eager") && unresolved.results.is_empty());

        let mut no_replies = std::iter::empty();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        assert!(
            !recover_unpaired_tool_calls(
                &mut ledger,
                &options,
                Some(&profile),
                Selected { version: 2 },
                false,
                &mut no_replies,
                &mut output,
                &cancellation,
            )
            .expect("replay-safe recovery")
        );
        let results = ledger
            .projection()
            .expect("projection")
            .events
            .iter()
            .filter(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("call-eager")
            })
            .count();
        assert_eq!(results, 1, "the interrupted eager call pairs exactly once");
        assert!(pending_tool_calls(&ledger).expect("pending").is_empty());

        reconcile_provider_attempt(&mut ledger, &options, &mut output).expect("close the attempt");
        let events = &ledger.projection().expect("projection").events;
        let tail = &events[events.len() - 3..];
        assert_eq!(tail[0].kind(), &EventKind::AttemptRecovery);
        assert_eq!(tail[1].kind(), &EventKind::Error);
        assert_eq!(tail[1].string_field("attempt"), Some("eager-attempt"));
        assert_eq!(
            tail[1]
                .usage()
                .and_then(|usage| usage.get("availability"))
                .and_then(Value::as_str),
            Some("unavailable")
        );
        assert_eq!(tail[2].kind(), &EventKind::Settle);
        assert_eq!(tail[2].string_field("outcome"), Some("interrupted"));
        assert!(unresolved_attempt(&ledger).expect("scan").is_none());
        let results = events
            .iter()
            .filter(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("call-eager")
            })
            .count();
        assert_eq!(results, 1, "closing the attempt never adds a second result");
    }

    fn async_dynamic_tool(name: &str) -> DynamicTool {
        DynamicTool::declared(
            name,
            profile::DynamicToolSource {
                kind: profile::DynamicToolSourceKind::Mcp,
                id: "fixture".to_owned(),
            },
            DynamicToolEffect::ReadOnly,
            true,
            Vec::new(),
            IJsonValue::parse(
                &serde_json::to_vec(&json!({
                    "name":name,"description":"Start one remote task",
                    "parameters":{"type":"object","properties":{"value":{"type":"string"}},
                        "required":["value"],"additionalProperties":false}
                }))
                .unwrap(),
            )
            .unwrap(),
        )
        .expect("dynamic tool")
    }

    fn task_state(status: &str) -> IJsonValue {
        let mut state = json!({
            "taskId":"task-async","status":status,
            "createdAt":"2026-09-05T00:00:00Z","lastUpdatedAt":"2026-09-05T00:00:01Z"
        });
        if status == "input_required" {
            state["inputRequests"] = json!({"confirm":{"question":"Proceed?"}});
        }
        IJsonValue::parse(&serde_json::to_vec(&state).unwrap()).unwrap()
    }

    fn line(bytes: Vec<u8>) -> io::Result<String> {
        Ok(String::from_utf8(bytes)
            .expect("UTF-8 wire line")
            .trim_end()
            .to_owned())
    }

    /// A pending tool-control result becomes a durable step-0 record and a
    /// bounded poll loop; each pending poll is a durable step; `input_required`
    /// parks the turn on a public hold with no `tool_result`; the answered hold
    /// resumes through ordinary recovery as exactly one input update; the
    /// terminal receipt becomes the call's single `tool_result`.
    #[test]
    fn remote_task_continuation_parks_on_input_and_resumes_to_one_result() {
        let (directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let name = "mcp__fixture__async";
        let mut profile = tool_profile(directory.path(), &[]);
        profile.config.workspace.policy.allowed_tools = vec![name.to_owned()];
        profile.bindings.dynamic_catalog =
            profile::DynamicToolCatalog::resolve(vec![async_dynamic_tool(name)]).expect("catalog");
        let options = test_options(&ledger);
        append_test_attempt(&mut ledger, "async-attempt");
        let arguments = json!({"value":"A"});
        let call = provider::ToolCall {
            call_id: "call-async".to_owned(),
            name: name.to_owned(),
            arguments: arguments.clone(),
        };
        append_provider_tool_call(
            &mut ledger,
            &options,
            &BuiltinManifest::compiled(),
            2,
            "async-attempt",
            &call,
        )
        .expect("durable call");
        let session = "018f0000-0000-7000-8000-000000000003";
        let original = ToolControl::new(
            session,
            session,
            2,
            "call-async",
            name,
            IJsonValue::parse(&serde_json::to_vec(&arguments).unwrap()).unwrap(),
        )
        .expect("original control");
        let continuation_id = "c".repeat(64);
        let step = |n: u64, action: ContinuationOperation| {
            ToolContinuationRequest::new(original.clone(), continuation_id.clone(), n, action)
                .expect("step")
        };
        let pending_reply = |n: u64, status: &str| ToolContinuationResponse {
            request_id: step(n, ContinuationOperation::Query).request_id,
            call_id: "call-async".to_owned(),
            result: ToolContinuationOutcome::Pending {
                continuation_id: continuation_id.clone(),
                next_step: n + 1,
                state: task_state(status),
            },
        };
        let initial = ToolControlResult::pending(
            original.request_id.clone(),
            "call-async",
            continuation_id.clone(),
            task_state("working"),
        );
        let mut replies = [
            line(worker_control::encode_tool_control_result(&initial).unwrap()),
            line(
                worker_control::continuation::encode_tool_continuation_result(&pending_reply(
                    1, "working",
                ))
                .unwrap(),
            ),
            line(
                worker_control::continuation::encode_tool_continuation_result(&pending_reply(
                    2,
                    "input_required",
                ))
                .unwrap(),
            ),
        ]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        assert!(
            execute_provider_tool_calls(
                &mut ledger,
                ToolCallBatch {
                    options: &options,
                    profile: &profile,
                    selected: Selected { version: 2 },
                    attempt: "async-attempt",
                    turn: 2,
                    calls: std::slice::from_ref(&call),
                    cancellation: &cancellation,
                },
                None,
                &mut replies,
                &mut output,
            )
            .expect("initial working task"),
            "input_required parks the turn"
        );
        assert!(
            replies.next().is_none(),
            "every scripted reply was consumed in order"
        );

        let events = ledger.projection().expect("projection").events.clone();
        let steps = events
            .iter()
            .filter(|event| {
                *event.kind() == EventKind::State
                    && event.string_field("subkind") == Some(tools::TOOL_CONTINUATION_SUBKIND)
            })
            .map(|event| {
                let raw = serde_json::to_value(event.raw()).unwrap();
                assert_eq!(
                    event.effective_visibility(),
                    Visibility::Runtime,
                    "continuation bookkeeping is not model context"
                );
                (
                    raw["payload"]["step"].as_u64().unwrap(),
                    raw["payload"]["action"].as_str().unwrap().to_owned(),
                    raw["payload"]["state"]["status"]
                        .as_str()
                        .unwrap()
                        .to_owned(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            steps,
            vec![
                (0, "bind".to_owned(), "working".to_owned()),
                (1, "query".to_owned(), "working".to_owned()),
                (2, "query".to_owned(), "input_required".to_owned()),
            ]
        );
        let hold = events
            .iter()
            .find(|event| {
                *event.kind() == EventKind::ApprovalRequest
                    && event.string_field("call") == Some("call-async")
            })
            .expect("input_required publishes a hold");
        assert_eq!(hold.string_field("scope"), Some(CONTINUATION_INPUT_SCOPE));
        assert_eq!(
            serde_json::to_value(hold.raw()).unwrap()["question"],
            json!({"confirm":{"question":"Proceed?"}})
        );
        assert!(
            events
                .iter()
                .all(|event| *event.kind() != EventKind::ToolResult),
            "no result while the remote task is pending"
        );
        assert!(ledger.projection().unwrap().lifecycle.open_hold);
        let requests = String::from_utf8(output.clone()).unwrap();
        let continuation_requests = requests
            .lines()
            .filter_map(|line| {
                worker_control::continuation::decode_tool_continuation(line.as_bytes()).ok()
            })
            .collect::<Vec<_>>();
        assert_eq!(continuation_requests.len(), 2);
        assert!(
            continuation_requests
                .iter()
                .enumerate()
                .all(|(index, request)| request.step == index as u64 + 1
                    && matches!(request.action, ContinuationOperation::Query)
                    && request.original == original
                    && request.continuation_id == continuation_id)
        );

        // Resume: the public answer becomes exactly one input update; the
        // completed receipt becomes the single tool_result.
        let response = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":2,"kind":"approval_response",
            "ts":options.timestamp,"call":"call-async","grant":true,"answer":{"answer":"yes"},
            "origin_key":"answer-async","origin_tuple":{
                "principal":"p","client":"cli","target":"t","op":"answer","key":"answer-async"
            }
        }))
        .expect("answer event");
        ledger
            .append_contract(response, BarrierContext::default())
            .expect("append answer");
        let update = step(
            3,
            ContinuationOperation::Update {
                input_responses: IJsonValue::parse(br#"{"answer":"yes"}"#).unwrap(),
            },
        );
        let completed = ToolContinuationResponse {
            request_id: update.request_id.clone(),
            call_id: "call-async".to_owned(),
            result: ToolContinuationOutcome::Completed {
                value: IJsonValue::parse(
                    br#"{"content":[{"type":"text","text":"async done"}],"isError":false}"#,
                )
                .unwrap(),
            },
        };
        let mut replies = [line(
            worker_control::continuation::encode_tool_continuation_result(&completed).unwrap(),
        )]
        .into_iter();
        output.clear();
        assert!(
            !recover_unpaired_tool_calls(
                &mut ledger,
                &options,
                Some(&profile),
                Selected { version: 2 },
                false,
                &mut replies,
                &mut output,
                &cancellation,
            )
            .expect("resume after the answer")
        );
        let sent = String::from_utf8(output).unwrap();
        let resumed = sent
            .lines()
            .filter_map(|line| {
                worker_control::continuation::decode_tool_continuation(line.as_bytes()).ok()
            })
            .collect::<Vec<_>>();
        assert_eq!(resumed.len(), 1, "the answer is sent exactly once");
        assert_eq!(
            resumed[0], update,
            "the resumed step is the update bound to the durable step ledger"
        );
        let events = &ledger.projection().expect("projection").events;
        let results = events
            .iter()
            .filter(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("call-async")
            })
            .collect::<Vec<_>>();
        assert_eq!(results.len(), 1, "exactly one final tool_result");
        let result = serde_json::to_value(results[0].raw()).unwrap();
        assert_eq!(result["outcome"], "ok");
        let text = result["content"][0]["text"]
            .as_str()
            .expect("dynamic results render as one text block");
        assert!(
            text.contains("async done") && text.contains("\"isError\":false"),
            "{text}"
        );
        assert!(pending_tool_calls(&ledger).unwrap().is_empty());
    }

    /// A stop while the remote task is pending dispatches a cancellation step
    /// and terminalizes the call as an error carrying the remote reply; the
    /// remote task never becomes a successful result.
    #[test]
    fn remote_task_continuation_stop_cancels_remotely_and_terminalizes_as_error() {
        let (directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let name = "mcp__fixture__async";
        let mut profile = tool_profile(directory.path(), &[]);
        profile.config.workspace.policy.allowed_tools = vec![name.to_owned()];
        profile.bindings.dynamic_catalog =
            profile::DynamicToolCatalog::resolve(vec![async_dynamic_tool(name)]).expect("catalog");
        let options = test_options(&ledger);
        append_test_attempt(&mut ledger, "async-attempt");
        let arguments = json!({"value":"B"});
        let call = provider::ToolCall {
            call_id: "call-stop".to_owned(),
            name: name.to_owned(),
            arguments: arguments.clone(),
        };
        append_provider_tool_call(
            &mut ledger,
            &options,
            &BuiltinManifest::compiled(),
            2,
            "async-attempt",
            &call,
        )
        .unwrap();
        let session = "018f0000-0000-7000-8000-000000000003";
        let invocation = IJsonValue::parse(&serde_json::to_vec(&arguments).unwrap()).unwrap();
        let original =
            ToolControl::new(session, session, 2, "call-stop", name, invocation.clone()).unwrap();
        let continuation_id = "d".repeat(64);
        {
            let assets =
                store::AssetStore::new(ledger.path().parent().unwrap().join("assets")).unwrap();
            let mut pipeline = ToolPipeline::new(&mut ledger, assets, SecretScanner::default());
            pipeline
                .append_continuation_step(
                    &ToolExecution {
                        thread: session.to_owned(),
                        call: "call-stop".to_owned(),
                        name: name.to_owned(),
                        attempt: "async-attempt".to_owned(),
                        invocation,
                        side_effectful: false,
                        turn: 2,
                        timestamp: options.timestamp.clone(),
                    },
                    &continuation_id,
                    0,
                    "bind",
                    &task_state("working"),
                    None,
                )
                .expect("durable bind step");
        }
        let cancel = ToolContinuationRequest::new(
            original.clone(),
            continuation_id.clone(),
            1,
            ContinuationOperation::Cancel,
        )
        .unwrap();
        let reply = ToolContinuationResponse {
            request_id: cancel.request_id.clone(),
            call_id: "call-stop".to_owned(),
            result: ToolContinuationOutcome::Failed {
                error: worker_control::ToolControlError {
                    code: ToolControlErrorCode::Unavailable,
                    message: "MCP task cancelled".to_owned(),
                    retryable: false,
                },
            },
        };
        let mut replies = [line(
            worker_control::continuation::encode_tool_continuation_result(&reply).unwrap(),
        )]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        cancellation.cancel(CANCEL_STOP);
        assert!(
            recover_unpaired_tool_calls(
                &mut ledger,
                &options,
                Some(&profile),
                Selected { version: 2 },
                false,
                &mut replies,
                &mut output,
                &cancellation,
            )
            .expect("stop path")
        );
        let sent = String::from_utf8(output).unwrap();
        let requests = sent
            .lines()
            .filter_map(|line| {
                worker_control::continuation::decode_tool_continuation(line.as_bytes()).ok()
            })
            .collect::<Vec<_>>();
        assert_eq!(requests, vec![cancel]);
        let events = &ledger.projection().unwrap().events;
        let results = events
            .iter()
            .filter(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("call-stop")
            })
            .collect::<Vec<_>>();
        assert_eq!(results.len(), 1);
        let result = serde_json::to_value(results[0].raw()).unwrap();
        assert_eq!(result["outcome"], "error");
        let text = result["content"][0]["text"].as_str().unwrap();
        assert!(
            text.contains("cancelled") && text.contains("MCP task cancelled"),
            "{text}"
        );
    }

    /// Native OpenAI client tool search selected automatically from the exact
    /// route: the first request declares `tool_search` natively, omits the
    /// deferred function and uses client-managed replay; the native search
    /// call executes the ordinary durable `tool_search`; the second request
    /// replays the sealed native call with a bound `tool_search_output`
    /// carrying the loaded deferred schema and no server continuation.
    #[test]
    fn native_route_declares_client_tool_search_and_replays_bound_output() {
        let (directory, mut ledger) = context_ledger("openai_responses_v1", false);
        let name = "mcp__fixture__marker";
        let mut profile = tool_profile(directory.path(), &[]);
        profile.config.workspace.policy.allowed_tools =
            vec!["tool_search".to_owned(), name.to_owned()];
        let deferred = DynamicTool::declared(
            name,
            profile::DynamicToolSource {
                kind: profile::DynamicToolSourceKind::Mcp,
                id: "fixture".to_owned(),
            },
            DynamicToolEffect::ReadOnly,
            false,
            vec!["uat marker".to_owned()],
            IJsonValue::parse(
                &serde_json::to_vec(&json!({
                    "name":name,"description":"Emit the UAT marker",
                    "parameters":{"type":"object","properties":{"marker":{"type":"string"}},
                        "required":["marker"],"additionalProperties":false}
                }))
                .unwrap(),
            )
            .unwrap(),
        )
        .expect("deferred tool");
        let deferred_schema = serde_json::to_value(&deferred.schema).unwrap();
        profile.bindings.dynamic_catalog =
            profile::DynamicToolCatalog::resolve(vec![deferred]).expect("catalog");
        profile.config.workspace.policy.provider = Some("cf".to_owned());
        profile.config.workspace.policy.model = Some("openai/gpt-5.6-luna".to_owned());
        profile.config.providers = ProvidersConfig {
            format: 1,
            revision: 1,
            providers: vec![Provider {
                id: "cf".to_owned(),
                name: None,
                adapter: "responses".to_owned(),
                dialect: "openai_responses_v1".to_owned(),
                endpoint_owner: "cloudflare".to_owned(),
                gateway_translation: "router".to_owned(),
                evidence_revision: "legacy-example-v2".to_owned(),
                endpoint: "https://api.cloudflare.com/client/v4/accounts/test/ai/v1".to_owned(),
                credential_key: Some("provider-main".to_owned()),
                models: vec![Model {
                    id: "openai/gpt-5.6-luna".to_owned(),
                    profile: "openai_responses_v1:openai/gpt-5.6-luna".to_owned(),
                    enabled: true,
                    context_window_tokens: 200_000,
                    compact_trigger_tokens: 180_000,
                }],
            }],
            web_search: None,
        };
        profile.config.revisions.providers = 1;
        let (supervisor, worker) = std::os::unix::net::UnixStream::pair().expect("socket pair");
        let (control, service) = provider::start_credential_channel(
            supervisor,
            provider::CredentialBroker::new([provider::CredentialScope {
                credential_id: "provider-main".to_owned(),
                adapter: "responses".to_owned(),
                endpoint_origin: "https://api.cloudflare.com:443".to_owned(),
                purpose: "provider".to_owned(),
                generation: "1".to_owned(),
                material: "fixture-secret".to_owned(),
            }]),
        );
        let mut credential = Some(Arc::new(Mutex::new(CredentialClient::new(worker))));

        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        set_provider_test_redirect(Some(format!("http://{}", listener.local_addr().unwrap())));
        let bodies = Arc::new(Mutex::new(Vec::<Value>::new()));
        let server_bodies = Arc::clone(&bodies);
        listener.set_nonblocking(true).unwrap();
        let server = std::thread::spawn(move || {
            for response in [
                br#"{"id":"resp-search","output":[{"arguments":{"limit":5,"query":"uat marker"},"call_id":"search-1","execution":"client","name":"tool_search","status":"completed","type":"tool_search_call"}],"status":"completed","usage":{"input_tokens":3,"output_tokens":1}}"#.as_slice(),
                br#"{"id":"resp-final","output":[{"content":[{"text":"done","type":"output_text"}],"type":"message"}],"status":"completed","usage":{"input_tokens":3,"output_tokens":1}}"#.as_slice(),
            ] {
                let deadline = Instant::now() + Duration::from_secs(20);
                let mut stream = loop {
                    match listener.accept() {
                        Ok((stream, _)) => break stream,
                        Err(error) if error.kind() == io::ErrorKind::WouldBlock && Instant::now() < deadline => {
                            std::thread::sleep(Duration::from_millis(10));
                        }
                        Err(_) => return,
                    }
                };
                stream.set_nonblocking(false).unwrap();
                let request = read_http_request(&mut stream);
                let split = request.windows(4).position(|bytes| bytes == b"\r\n\r\n").unwrap() + 4;
                server_bodies.lock().unwrap().push(serde_json::from_slice(&request[split..]).expect("request body JSON"));
                write!(stream, "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", response.len()).unwrap();
                stream.write_all(response).unwrap();
            }
        });
        let options = test_options(&ledger);
        let mut replies = [
            Ok("{\"lease\":{\"attempt\":\"tool-run-attempt-13\",\"granted\":true}}".to_owned()),
            Ok("{\"lease\":{\"attempt\":\"tool-run-attempt-19\",\"granted\":true}}".to_owned()),
        ]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut credential,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        result.expect("native two-round turn");
        server.join().expect("provider server");
        drop(credential);
        drop(control);
        service.join().expect("broker thread").expect("broker EOF");

        let bodies = bodies.lock().unwrap();
        let summary = ledger
            .projection()
            .unwrap()
            .events
            .iter()
            .map(|event| {
                let raw = serde_json::to_value(event.raw()).unwrap();
                format!(
                    "{} {} {} {} {}",
                    event.seq(),
                    event.kind().as_str(),
                    raw["name"],
                    raw["outcome"],
                    raw["detail"]
                        .as_str()
                        .or(raw["content"][0]["text"].as_str())
                        .unwrap_or("")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(
            bodies.len(),
            2,
            "one search round and one final round\n{summary}"
        );
        let first = &bodies[0];
        assert_eq!(
            first["store"], false,
            "native client search is client-managed replay"
        );
        assert!(first.get("previous_response_id").is_none());
        let tools = first["tools"].as_array().unwrap();
        let search = tools
            .iter()
            .find(|tool| tool["type"] == "tool_search")
            .expect("native tool_search declaration");
        assert_eq!(search["execution"], "client");
        assert!(
            tools.iter().all(|tool| tool["name"] != name),
            "deferred function is not declared in the initial native tool list"
        );
        let second = &bodies[1];
        assert_eq!(second["store"], false);
        assert!(
            second.get("previous_response_id").is_none(),
            "replay stays client-managed after a server-managed response"
        );
        let input = second["input"].as_array().unwrap();
        assert!(
            input
                .iter()
                .any(|item| item["type"] == "message" && item["role"] == "user"),
            "original user input is retained on replay"
        );
        let sealed = input
            .iter()
            .find(|item| item["type"] == "tool_search_call")
            .expect("sealed native search call replays");
        assert_eq!(sealed["call_id"], "search-1");
        let output = input
            .iter()
            .find(|item| item["type"] == "tool_search_output")
            .expect("bound native search output");
        assert_eq!(output["call_id"], "search-1");
        assert_eq!(output["tools"][0]["name"], name);
        assert_eq!(output["tools"][0]["defer_loading"], true);
        assert_eq!(
            output["tools"][0]["parameters"],
            deferred_schema["parameters"]
        );
        assert!(
            input
                .iter()
                .all(|item| item["type"] != "function_call_output"),
            "the search result is not also replayed as an ordinary function output"
        );
        assert!(
            second["tools"]
                .as_array()
                .unwrap()
                .iter()
                .all(|tool| tool["name"] != name),
            "loaded schema travels in tool_search_output, not the tool list"
        );

        let events = &ledger.projection().unwrap().events;
        let search_call = events
            .iter()
            .find(|event| {
                *event.kind() == EventKind::ToolCall
                    && event.string_field("name") == Some("tool_search")
            })
            .expect("durable search call");
        assert_eq!(search_call.string_field("call"), Some("search-1"));
        let search_result = events
            .iter()
            .find(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("search-1")
            })
            .expect("durable offer");
        assert_eq!(search_result.string_field("outcome"), Some("ok"));
        let settle = events.last().unwrap();
        assert_eq!(settle.kind(), &EventKind::Settle);
        assert_eq!(settle.string_field("outcome"), Some("completed"));
        assert!(
            events
                .iter()
                .filter(|event| *event.kind() == EventKind::Output && event.has_field("usage"))
                .count()
                >= 2,
            "usage is present for both native rounds"
        );
    }

    /// Eager dispatch is not a Responses-only behavior: an Anthropic Messages
    /// stream announces the call at `content_block_stop`, the worker executes it
    /// before `message_stop` bytes exist, and the terminal reuses it once.
    #[test]
    fn eager_dispatch_executes_anthropic_call_before_message_stop_bytes() {
        let (directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let ledger_path = ledger.path().to_owned();
        let listener = TcpListener::bind("127.0.0.1:0").expect("provider listener");
        set_provider_test_redirect(Some(format!(
            "http://{}",
            listener.local_addr().expect("provider address")
        )));
        let prefix = concat!(
            "data: {\"type\":\"message_start\",\"message\":{\"usage\":{\"input_tokens\":2}}}\n\n",
            "data: {\"type\":\"content_block_start\",\"index\":0,\"content_block\":{\"type\":\"tool_use\",\"id\":\"call-think\",\"name\":\"think\",\"input\":{}}}\n\n",
            "data: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"input_json_delta\",\"partial_json\":\"{\\\"thought\\\":\\\"inspect\\\"}\"}}\n\n",
            "data: {\"type\":\"content_block_stop\",\"index\":0}\n\n",
        );
        let terminal = concat!(
            "data: {\"type\":\"message_delta\",\"delta\":{\"stop_reason\":\"tool_use\"},\"usage\":{\"output_tokens\":1}}\n\n",
            "data: {\"type\":\"message_stop\"}\n\n",
        );
        let final_json: &[u8] = br#"{"content":[{"text":"done","type":"text"}],"id":"msg-final","role":"assistant","stop_reason":"end_turn","type":"message","usage":{"input_tokens":2,"output_tokens":1}}"#;
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("provider accept");
            let _ = read_http_request(&mut stream);
            write!(stream, "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", prefix.len() + terminal.len()).unwrap();
            stream.write_all(prefix.as_bytes()).unwrap();
            stream.flush().unwrap();
            let executed = ledger_file_contains(
                &ledger_path,
                "\"call\":\"call-think\",\"content\"",
                Duration::from_secs(10),
            );
            stream.write_all(terminal.as_bytes()).unwrap();
            let (mut stream, _) = listener.accept().expect("continuation accept");
            let _ = read_http_request(&mut stream);
            write!(stream, "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n", final_json.len()).unwrap();
            stream.write_all(final_json).unwrap();
            executed
        });
        let mut profile = tool_profile(directory.path(), &["think"]);
        profile.config.workspace.policy.provider = Some("anthropic".to_owned());
        profile.config.workspace.policy.model = Some("claude-sonnet-4-20250514".to_owned());
        profile.config.providers = ProvidersConfig {
            format: 1,
            revision: 1,
            providers: vec![Provider {
                id: "anthropic".to_owned(),
                name: None,
                adapter: "anthropic_messages".to_owned(),
                dialect: "anthropic_messages_v1".to_owned(),
                endpoint_owner: "anthropic".to_owned(),
                gateway_translation: "direct".to_owned(),
                evidence_revision: "anthropic-2026-08-01".to_owned(),
                endpoint: "https://api.anthropic.com".to_owned(),
                credential_key: Some("provider-main".to_owned()),
                models: vec![Model {
                    id: "claude-sonnet-4-20250514".to_owned(),
                    profile: "anthropic_messages_v1:claude-sonnet-4-20250514".to_owned(),
                    enabled: true,
                    context_window_tokens: 200_000,
                    compact_trigger_tokens: 180_000,
                }],
            }],
            web_search: None,
        };
        profile.config.revisions.providers = 1;
        let (supervisor, worker) = std::os::unix::net::UnixStream::pair().expect("socket pair");
        let (control, service) = provider::start_credential_channel(
            supervisor,
            provider::CredentialBroker::new([provider::CredentialScope {
                credential_id: "provider-main".to_owned(),
                adapter: "anthropic_messages".to_owned(),
                endpoint_origin: "https://api.anthropic.com:443".to_owned(),
                purpose: "provider".to_owned(),
                generation: "1".to_owned(),
                material: "fixture-secret".to_owned(),
            }]),
        );
        let mut credential = Some(Arc::new(Mutex::new(CredentialClient::new(worker))));
        let options = test_options(&ledger);
        let mut replies = [
            Ok("{\"lease\":{\"attempt\":\"tool-run-attempt-13\",\"granted\":true}}".to_owned()),
            Ok("{\"lease\":{\"attempt\":\"tool-run-attempt-19\",\"granted\":true}}".to_owned()),
        ]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        let result = run_provider_turn(
            &mut ledger,
            &options,
            &profile,
            &Selected { version: 2 },
            &mut credential,
            &mut replies,
            &mut output,
            &cancellation,
        );
        set_provider_test_redirect(None);
        if let Err(error) = &result {
            panic!(
                "anthropic eager continuation: {error}\nworker stdout:\n{}",
                String::from_utf8_lossy(&output)
            );
        }
        let executed_before_terminal = server.join().expect("provider server");
        drop(credential);
        drop(control);
        service.join().expect("broker thread").expect("broker EOF");
        assert!(
            executed_before_terminal,
            "the result must be durable before message_stop bytes are released"
        );
        let events = &ledger.projection().expect("projection").events;
        let seq_of = |kind: EventKind, field: &str, value: &str| {
            events
                .iter()
                .filter(|event| *event.kind() == kind && event.string_field(field) == Some(value))
                .map(Event::seq)
                .collect::<Vec<_>>()
        };
        let calls = seq_of(EventKind::ToolCall, "call", "call-think");
        let results = seq_of(EventKind::ToolResult, "call", "call-think");
        let outputs = seq_of(EventKind::Output, "attempt", "tool-run-attempt-13");
        assert_eq!((calls.len(), results.len(), outputs.len()), (1, 1, 1));
        assert!(calls[0] < results[0] && results[0] < outputs[0]);
        let settle = events.last().unwrap();
        assert_eq!(settle.kind(), &EventKind::Settle);
        assert_eq!(settle.string_field("outcome"), Some("completed"));
    }

    /// An eagerly dispatched result is durable before its response output.
    /// Stateless replay must still present the assistant's sealed call before
    /// that result, or native and ordinary tool protocols both break.
    #[test]
    fn eager_results_render_after_their_attempt_output_in_replay() {
        let (_directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let options = test_options(&ledger);
        append_test_attempt(&mut ledger, "eager");
        append_test_call(&mut ledger, "eager", "c1", "think", json!({"thought":"x"}));
        let result = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":2,"kind":"tool_result","ts":options.timestamp,
            "call":"c1","outcome":"ok","content":[{"type":"text","text":"recorded"}]
        }))
        .unwrap();
        ledger
            .append_contract(result, BarrierContext::default())
            .unwrap();
        let fragments = sealed_fragments(
            &ledger,
            br#"[{"type":"tool_use","id":"c1","name":"think","input":{"thought":"x"}}]"#,
        )
        .unwrap();
        let output = make_event(json!({
            "v":1,"seq":ledger.next_seq(),"turn":2,"kind":"output","ts":options.timestamp,
            "attempt":"eager","content":[],"final_answer":false,"usage":usage_object(None),
            "sealed":{"version":1,"adapter":"anthropic_messages_v1","fragments":fragments}
        }))
        .unwrap();
        ledger
            .append_contract(output, BarrierContext::default())
            .unwrap();
        let context = project_provider_context(&ledger, "e1", DialectId::AnthropicMessagesV1, 2)
            .expect("context");
        let roles = context
            .items
            .iter()
            .map(|item| {
                serde_json::to_value(item).unwrap()["role"]
                    .as_str()
                    .unwrap()
                    .to_owned()
            })
            .collect::<Vec<_>>();
        let sealed = roles
            .iter()
            .rposition(|role| role == "sealed")
            .expect("sealed output item");
        let tool = roles
            .iter()
            .rposition(|role| role == "tool")
            .expect("tool result item");
        assert!(
            sealed < tool,
            "result must follow the sealed call it answers: {roles:?}"
        );
        assert_eq!(roles.iter().filter(|role| *role == "tool").count(), 1);
    }

    /// Slow remote polls park the worker instead of keeping it resident: the
    /// due instant and interval are durable, the turn stays open, the exit is
    /// clean, the lifecycle facts expose the wait, and the resumed run polls
    /// once the instant has passed and pairs the call exactly once.
    #[test]
    fn remote_task_continuation_parks_when_polls_slow_and_resumes_when_due() {
        let (directory, mut ledger) = context_ledger("anthropic_messages_v1", false);
        let name = "mcp__fixture__async";
        let mut profile = tool_profile(directory.path(), &[]);
        profile.config.workspace.policy.allowed_tools = vec![name.to_owned()];
        profile.bindings.dynamic_catalog =
            profile::DynamicToolCatalog::resolve(vec![async_dynamic_tool(name)]).expect("catalog");
        let options = test_options(&ledger);
        append_test_attempt(&mut ledger, "async-attempt");
        let arguments = json!({"value":"P"});
        let call = provider::ToolCall {
            call_id: "call-park".to_owned(),
            name: name.to_owned(),
            arguments: arguments.clone(),
        };
        append_provider_tool_call(
            &mut ledger,
            &options,
            &BuiltinManifest::compiled(),
            2,
            "async-attempt",
            &call,
        )
        .unwrap();
        let session = "018f0000-0000-7000-8000-000000000003";
        let original = ToolControl::new(
            session,
            session,
            2,
            "call-park",
            name,
            IJsonValue::parse(&serde_json::to_vec(&arguments).unwrap()).unwrap(),
        )
        .unwrap();
        let continuation_id = "e".repeat(64);
        let step = |n: u64| {
            ToolContinuationRequest::new(
                original.clone(),
                continuation_id.clone(),
                n,
                ContinuationOperation::Query,
            )
            .unwrap()
        };
        let pending = |n: u64| ToolContinuationResponse {
            request_id: step(n).request_id,
            call_id: "call-park".to_owned(),
            result: ToolContinuationOutcome::Pending {
                continuation_id: continuation_id.clone(),
                next_step: n + 1,
                state: task_state("working"),
            },
        };
        let initial = ToolControlResult::pending(
            original.request_id.clone(),
            "call-park",
            continuation_id.clone(),
            task_state("working"),
        );
        let mut replies = [
            line(worker_control::encode_tool_control_result(&initial).unwrap()),
            line(
                worker_control::continuation::encode_tool_continuation_result(&pending(1)).unwrap(),
            ),
            line(
                worker_control::continuation::encode_tool_continuation_result(&pending(2)).unwrap(),
            ),
        ]
        .into_iter();
        let mut output = Vec::new();
        let cancellation = RuntimeCancellation::default();
        let started = Instant::now();
        assert!(
            execute_provider_tool_calls(
                &mut ledger,
                ToolCallBatch {
                    options: &options,
                    profile: &profile,
                    selected: Selected { version: 2 },
                    attempt: "async-attempt",
                    turn: 2,
                    calls: std::slice::from_ref(&call),
                    cancellation: &cancellation,
                },
                None,
                &mut replies,
                &mut output
            )
            .expect("slow polls park"),
            "the worker parks instead of polling resident"
        );
        assert!(replies.next().is_none());
        let projection = ledger.projection().unwrap();
        let park = projection
            .events
            .iter()
            .rev()
            .find(|event| *event.kind() == EventKind::State)
            .unwrap();
        let park = serde_json::to_value(park.raw()).unwrap();
        assert_eq!(park["payload"]["action"], "park");
        assert_eq!(park["payload"]["step"], 2);
        assert_eq!(park["payload"]["interval_ms"], 1000);
        let poll_after = park["payload"]["poll_after"].as_str().unwrap().to_owned();
        let due = chrono::DateTime::parse_from_rfc3339(&poll_after)
            .unwrap()
            .with_timezone(&chrono::Utc);
        assert!(
            due > chrono::Utc::now() - chrono::Duration::seconds(5)
                && due <= chrono::Utc::now() + chrono::Duration::seconds(2)
        );
        assert_eq!(
            projection.lifecycle.continuation_wait_until.as_deref(),
            Some(poll_after.as_str())
        );
        assert!(projection.lifecycle.unresolved_work && !projection.lifecycle.open_hold);
        assert!(
            started.elapsed() < Duration::from_secs(3),
            "the resident phase is bounded"
        );
        // The parked run exits clean without settling the open turn.
        let mut exit_output = Vec::new();
        let code = post_turn_exit(&mut ledger, &options, &cancellation, &mut exit_output).unwrap();
        assert!(exit_code_is(code, ExitCode::SUCCESS));
        assert!(
            ledger
                .projection()
                .unwrap()
                .events
                .iter()
                .all(|event| *event.kind() != EventKind::Settle || event.turn() != Some(2))
        );
        let facts = ledger.projection().unwrap().lifecycle.clone();
        let state = classify(&facts, LockFacts::CALLER);
        assert_eq!(state, TailState::RecoveryNeeded);
        assert_eq!(
            engine::ensure_action_at(state, &facts, Some("2020-01-01T00:00:00.000Z")),
            engine::EnsureAction::None,
            "not due yet"
        );
        assert_eq!(
            engine::ensure_action_at(state, &facts, Some("2999-01-01T00:00:00.000Z")),
            engine::EnsureAction::SpawnCandidate,
            "due"
        );
        assert_eq!(
            run_decision(state, &facts),
            RunDecision::Start(RunMode::Ordinary),
            "a parked continuation resumes ordinarily under resume=never"
        );

        // Resume: waits for the durable instant, then step 3 completes.
        let completed = ToolContinuationResponse {
            request_id: step(3).request_id,
            call_id: "call-park".to_owned(),
            result: ToolContinuationOutcome::Completed {
                value: IJsonValue::parse(
                    br#"{"content":[{"type":"text","text":"parked done"}],"isError":false}"#,
                )
                .unwrap(),
            },
        };
        let mut replies = [line(
            worker_control::continuation::encode_tool_continuation_result(&completed).unwrap(),
        )]
        .into_iter();
        let resumed = Instant::now();
        assert!(
            !recover_unpaired_tool_calls(
                &mut ledger,
                &options,
                Some(&profile),
                Selected { version: 2 },
                false,
                &mut replies,
                &mut output,
                &cancellation
            )
            .unwrap()
        );
        assert!(
            chrono::Utc::now() >= due,
            "the resumed run did not poll before the durable instant"
        );
        assert!(resumed.elapsed() < Duration::from_secs(3));
        let events = &ledger.projection().unwrap().events;
        let results = events
            .iter()
            .filter(|event| {
                *event.kind() == EventKind::ToolResult
                    && event.string_field("call") == Some("call-park")
            })
            .count();
        assert_eq!(results, 1);
        assert!(
            ledger
                .projection()
                .unwrap()
                .lifecycle
                .continuation_wait_until
                .is_none()
        );
    }
}
