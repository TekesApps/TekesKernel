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
mod provider_turn;
use provider_turn::*;

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

#[cfg(test)]
mod provider_context_tests;
