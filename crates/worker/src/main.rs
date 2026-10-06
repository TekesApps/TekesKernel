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

fn execute_provider_tool_calls<I, O>(
    ledger: &mut LockedLedger,
    batch: ToolCallBatch<'_>,
    credential: Option<&Arc<Mutex<CredentialClient>>>,
    lines: &mut I,
    stdout: &mut O,
) -> Result<bool, Box<dyn std::error::Error>>
where
    I: Iterator<Item = io::Result<String>>,
    O: Write,
{
    let ToolCallBatch {
        options,
        profile,
        selected,
        attempt,
        turn,
        calls,
        cancellation,
    } = batch;
    let thread_folder = ledger
        .path()
        .parent()
        .ok_or("worker ledger has no thread folder")?
        .to_path_buf();
    let thread = ledger
        .projection()
        .and_then(|projection| projection.events.first())
        .and_then(|genesis| genesis.string_field("thread"))
        .ok_or("genesis thread binding is missing")?
        .to_owned();
    let manifest = BuiltinManifest::compiled();
    let assets = store::AssetStore::new(thread_folder.join("assets"))?;
    let hooks = frozen_hook_bindings(&profile.instruction)?;
    // Re-read per batch: a mode selected through the endpoint applies to the
    // next tool call of a running session without a restart.
    let mut policy = session_permission_policy(&thread_folder);
    for call in calls {
        if cancellation.supervisor_lost() {
            return Err(Box::new(ProtocolFailure(
                "supervisor lost before tool execution".to_owned(),
            )));
        }
        if cancellation.stop_requested() {
            return Ok(true);
        }
        if matches!(call.name.as_str(), "task" | "subagent") {
            if execute_child_spawn_tool(
                ledger,
                ChildSpawnExecution {
                    options,
                    profile,
                    selected,
                    attempt,
                    turn,
                    call,
                    hooks: &hooks,
                    manifest: &manifest,
                    cancellation,
                },
                lines,
                stdout,
            )? {
                return Ok(true);
            }
            continue;
        }
        if let Some(detail) = provider::invalid_arguments_detail(&call.arguments) {
            // The model's arguments were not JSON; the call is refused with a
            // durable error result so the model can resend it, never executed.
            append_tool_validation_error(ledger, options, turn, call, &detail)?;
            continue;
        }
        let write_roots = match invocation_write_roots(profile, call) {
            Ok(roots) => roots,
            Err(error) => {
                // Invalid model-supplied authority is a tool rejection, not
                // a worker crash. Otherwise eager dispatch exits with an
                // unpaired call and every recovery repeats the same failure.
                append_tool_validation_error(ledger, options, turn, call, &error.to_string())?;
                continue;
            }
        };
        let (mut backend, mut dynamic_backend) = assemble_tool_backends(
            ToolBackendPlan {
                profile,
                selected,
                thread_folder: &thread_folder,
                invocation_write_roots: &write_roots,
                credential,
                web_search_ready: options.web_search_ready,
            },
            ToolBackendRuntime {
                lines,
                stdout,
                cancellation,
            },
        )?;
        let arguments = IJsonValue::parse(&serde_json::to_vec(&call.arguments)?)?;
        let invocation = ToolInvocation {
            thread: thread.clone(),
            turn,
            attempt: attempt.to_owned(),
            call: call.call_id.clone(),
            name: call.name.clone(),
            arguments,
            timestamp: options.event_timestamp().clone(),
        };
        let has_deferred = !DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)
            .deferred_search_entries(&dynamic_backend)?
            .is_empty();
        let context = tool_catalog_context(profile, has_deferred, options.web_search_ready)?;
        let mut pipeline = ToolPipeline::new(ledger, assets.clone(), SecretScanner::default());
        let decision = if manifest
            .tools
            .iter()
            .any(|tool| tool.name == invocation.name)
        {
            ToolDispatcher::new(&manifest, &context)
                .dispatch(
                    &mut pipeline,
                    &invocation,
                    &hooks,
                    &mut policy,
                    &mut backend,
                )
                .map_err(|error| error.to_string())
        } else {
            DynamicToolDispatcher::new(&profile.bindings.dynamic_catalog)
                .dispatch(
                    &mut pipeline,
                    &invocation,
                    &hooks,
                    &mut policy,
                    &mut dynamic_backend,
                )
                .map_err(|error| error.to_string())
        };
        let decision = match decision {
            Ok(decision) => decision,
            Err(error) => {
                drop(pipeline);
                append_tool_validation_error(ledger, options, turn, call, &error)?;
                continue;
            }
        };
        if cancellation.protocol_failed() {
            return Err(Box::new(ProtocolFailure(
                "tool-control protocol failure".to_owned(),
            )));
        }
        if cancellation.stop_requested() {
            return Ok(true);
        }
        if cancellation.supervisor_lost() {
            return Err(Box::new(ProtocolFailure(
                "supervisor lost during tool execution".to_owned(),
            )));
        }
        if matches!(decision, PipelineDecision::Parked { .. }) {
            return Ok(true);
        }
        if let PipelineDecision::Pending {
            continuation_id, ..
        } = decision
        {
            // The supervisor bound a remote continuation (an MCP task). The
            // durable step-0 state is written; drive it to its single terminal.
            // The backends hold the control channel; release them first.
            drop(backend);
            drop(dynamic_backend);
            if wait_tool_continuation(
                ledger,
                ContinuationWait {
                    options,
                    profile,
                    selected,
                    call: &call.call_id,
                    continuation_id: &continuation_id,
                    cancellation,
                },
                lines,
                stdout,
            )? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// The durable facts of one call's remote continuation, folded from its
/// `state{subkind: tool_continuation}` records and approval pair.
struct ContinuationFacts {
    continuation_id: String,
    /// Highest durable step (0 = the initial pending binding).
    step: u64,
    /// Remote task status at that step.
    status: String,
    state: Value,
    /// Seq of the latest `update` step record, if any.
    updated_at: Option<u64>,
    /// Latest park record: when the next poll is due and the interval used.
    poll_after: Option<String>,
    interval_ms: Option<u64>,
    approval_request: Option<u64>,
    approval_response: Option<(u64, bool, Option<Value>)>,
    result: bool,
}

fn continuation_facts(
    ledger: &LockedLedger,
    call: &str,
) -> Result<Option<ContinuationFacts>, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let mut facts: Option<ContinuationFacts> = None;
    for event in &projection.events {
        match event.kind() {
            EventKind::State
                if event.string_field("subkind") == Some(tools::TOOL_CONTINUATION_SUBKIND) =>
            {
                let raw = serde_json::to_value(event.raw())?;
                let payload = raw
                    .get("payload")
                    .ok_or("tool_continuation lacks payload")?;
                if payload.get("call").and_then(Value::as_str) != Some(call) {
                    continue;
                }
                let continuation_id = payload
                    .get("continuation_id")
                    .and_then(Value::as_str)
                    .ok_or("tool_continuation lacks continuation_id")?
                    .to_owned();
                let step = payload
                    .get("step")
                    .and_then(Value::as_u64)
                    .ok_or("tool_continuation lacks step")?;
                let action = payload
                    .get("action")
                    .and_then(Value::as_str)
                    .unwrap_or("query");
                let state = materialize_json(
                    ledger,
                    payload
                        .get("state")
                        .ok_or("tool_continuation lacks state")?,
                )?;
                let status = state
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned();
                let updated_at = (action == "update").then_some(event.seq());
                let poll_after = payload
                    .get("poll_after")
                    .and_then(Value::as_str)
                    .map(str::to_owned);
                let interval_ms = payload.get("interval_ms").and_then(Value::as_u64);
                match facts.as_mut() {
                    Some(existing) => {
                        if existing.continuation_id != continuation_id {
                            return Err("call has records for two continuations".into());
                        }
                        if step < existing.step {
                            return Err("tool_continuation steps regress".into());
                        }
                        existing.step = step;
                        existing.status = status;
                        existing.state = state;
                        existing.updated_at = updated_at.or(existing.updated_at);
                        // Only a park record carries a due instant; any later
                        // receipt clears it.
                        existing.poll_after = poll_after;
                        existing.interval_ms = interval_ms.or(existing.interval_ms);
                    }
                    None => {
                        facts = Some(ContinuationFacts {
                            continuation_id,
                            step,
                            status,
                            state,
                            updated_at,
                            poll_after,
                            interval_ms,
                            approval_request: None,
                            approval_response: None,
                            result: false,
                        });
                    }
                }
            }
            EventKind::ApprovalRequest if event.string_field("call") == Some(call) => {
                if let Some(existing) = facts.as_mut() {
                    existing.approval_request = Some(event.seq());
                }
            }
            EventKind::ApprovalResponse if event.string_field("call") == Some(call) => {
                if let Some(existing) = facts.as_mut() {
                    let raw = serde_json::to_value(event.raw())?;
                    existing.approval_response = Some((
                        event.seq(),
                        raw.get("grant").and_then(Value::as_bool).unwrap_or(false),
                        raw.get("answer").cloned(),
                    ));
                }
            }
            EventKind::ToolResult if event.string_field("call") == Some(call) => {
                if let Some(existing) = facts.as_mut() {
                    existing.result = true;
                }
            }
            _ => {}
        }
    }
    Ok(facts)
}

/// Scope of the public question a remote task asks through `input_required`.
const CONTINUATION_INPUT_SCOPE: &str = "mcp_task_input";
const CONTINUATION_POLL_FLOOR: Duration = Duration::from_millis(250);
/// Polls shorter than this stay resident; at this interval the worker parks
/// (durable `poll_after`, line lock released) and the supervisor's sweep
/// resumes the line when the poll is due.
const CONTINUATION_PARK_THRESHOLD: Duration = Duration::from_secs(1);
const CONTINUATION_POLL_CEILING: Duration = Duration::from_secs(30);
/// Provider retries per turn under the rate-limit and transport budget.
const PROVIDER_RETRIES: u8 = 3;
const PROVIDER_ADMISSION_FLOOR: Duration = Duration::from_secs(1);
const PROVIDER_ADMISSION_CEILING: Duration = Duration::from_secs(30);
const PROVIDER_ADMISSION_SUBKIND: &str = "provider_admission";

fn rfc3339_after(delay: Duration) -> String {
    (chrono::Utc::now() + chrono::Duration::from_std(delay).unwrap_or_default())
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

struct ContinuationWait<'a> {
    options: &'a Options,
    profile: &'a RuntimeProfile,
    selected: Selected,
    call: &'a str,
    continuation_id: &'a str,
    cancellation: &'a RuntimeCancellation,
}

/// Drives one bound remote continuation to exactly one terminal `tool_result`.
/// Each request step is a distinct receipt-bound identity; every pending
/// response is a durable step record before the next request; `input_required`
/// parks the turn on an ordinary public hold whose answer becomes exactly one
/// input update; a stop cancels the remote task and terminalizes the call as
/// an error. Returns `true` when the turn parked or stopped.
fn wait_tool_continuation<I, O>(
    ledger: &mut LockedLedger,
    wait: ContinuationWait<'_>,
    lines: &mut I,
    stdout: &mut O,
) -> Result<bool, Box<dyn std::error::Error>>
where
    I: Iterator<Item = io::Result<String>>,
    O: Write,
{
    let ContinuationWait {
        options,
        profile,
        selected,
        call,
        continuation_id,
        cancellation,
    } = wait;
    let thread_folder = ledger
        .path()
        .parent()
        .ok_or("worker ledger has no thread folder")?
        .to_path_buf();
    let session = thread_folder
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("thread folder has no UTF-8 session UUID")?
        .to_owned();
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let thread = projection
        .events
        .first()
        .and_then(|genesis| genesis.string_field("thread"))
        .ok_or("genesis thread binding is missing")?
        .to_owned();
    let tool_call = projection
        .events
        .iter()
        .find(|event| {
            *event.kind() == EventKind::ToolCall && event.string_field("call") == Some(call)
        })
        .ok_or("continuation call is not durable")?;
    let turn = tool_call.turn().ok_or("tool_call lacks turn")?;
    let attempt = tool_call
        .string_field("attempt")
        .ok_or("tool_call lacks attempt")?
        .to_owned();
    let name = tool_call
        .string_field("name")
        .ok_or("tool_call lacks name")?
        .to_owned();
    let raw_call = serde_json::to_value(tool_call.raw())?;
    let mut arguments =
        materialize_json(ledger, raw_call.get("args").ok_or("tool_call lacks args")?)?;
    if let Some(effective) = projection.events.iter().rev().find(|event| {
        *event.kind() == EventKind::EffectiveExecution && event.string_field("call") == Some(call)
    }) {
        let raw = serde_json::to_value(effective.raw())?;
        arguments = materialize_json(
            ledger,
            raw.get("invocation")
                .ok_or("effective_execution lacks invocation")?,
        )?;
    }
    let arguments = IJsonValue::parse(&serde_json::to_vec(&arguments)?)?;
    let original = ToolControl::new(
        session,
        thread.clone(),
        turn,
        call,
        name.clone(),
        arguments.clone(),
    )?;
    let side_effectful = profile
        .bindings
        .dynamic_catalog
        .tools
        .iter()
        .find(|tool| tool.name == name)
        .is_none_or(|tool| engine::dynamic_side_effectful(tool.effect));
    let execution = ToolExecution {
        thread,
        call: call.to_owned(),
        name,
        attempt,
        invocation: arguments,
        side_effectful,
        turn,
        timestamp: options.event_timestamp().clone(),
    };
    let hooks = frozen_hook_bindings(&profile.instruction)?;
    let assets = store::AssetStore::new(thread_folder.join("assets"))?;
    let mut interval = CONTINUATION_POLL_FLOOR;
    let mut resumed_from_park = false;
    // After a park the due instant has been honored: the next query goes out
    // immediately, and only later polls may park again.
    let mut poll_immediately = false;
    loop {
        let facts =
            continuation_facts(ledger, call)?.ok_or("continuation has no durable step record")?;
        if facts.continuation_id != continuation_id {
            return Err("continuation identity changed under the call".into());
        }
        if facts.result {
            return Ok(false);
        }
        if !resumed_from_park {
            resumed_from_park = true;
            if let Some(parked_interval) = facts.interval_ms {
                // Resume after a park: honor the durable due instant, then keep
                // growing the interval from where the previous run left it.
                if let Some(due) = facts
                    .poll_after
                    .as_deref()
                    .and_then(|value| chrono::DateTime::parse_from_rfc3339(value).ok())
                {
                    let remaining = (due.with_timezone(&chrono::Utc) - chrono::Utc::now())
                        .to_std()
                        .unwrap_or_default();
                    std::thread::sleep(remaining.min(CONTINUATION_POLL_CEILING));
                }
                interval = Duration::from_millis(parked_interval.saturating_mul(2))
                    .min(CONTINUATION_POLL_CEILING);
                poll_immediately = true;
            }
        }
        let terminalize = |ledger: &mut LockedLedger, terminal: BackendTerminal| {
            let mut pipeline = ToolPipeline::new(ledger, assets.clone(), SecretScanner::default());
            pipeline
                .complete_continuation(&execution, &hooks, &execution.invocation, terminal)
                .map_err(|error| error.to_string())
        };
        if cancellation.stop_requested() {
            // Best effort remote cancellation is its own receipt; the local
            // outcome never claims the remote acknowledged it.
            let cancel = ToolContinuationRequest::new(
                original.clone(),
                continuation_id.to_owned(),
                facts.step + 1,
                ContinuationOperation::Cancel,
            )?;
            let remote = exchange_tool_continuation_runtime(
                &selected,
                &cancel,
                lines,
                stdout,
                cancellation,
                true,
            )
            .map(|response| match response.result {
                ToolContinuationOutcome::Failed { error } => format!(
                    "remote {}: {}",
                    control_error_code_name(&error.code),
                    error.message
                ),
                ToolContinuationOutcome::Pending { .. } => "remote still pending".to_owned(),
                ToolContinuationOutcome::Completed { .. } => {
                    "remote completed before cancellation".to_owned()
                }
            })
            .unwrap_or_else(|error| format!("cancellation not confirmed: {error}"));
            terminalize(
                ledger,
                BackendTerminal::Unavailable {
                    code: "cancelled".to_owned(),
                    message: format!("stopped while the remote task was pending; {remote}"),
                    retryable: false,
                },
            )?;
            return Ok(true);
        }
        let action = if facts.status == "input_required" {
            match (facts.approval_request, facts.approval_response) {
                (None, _) => {
                    // The task contract carries a non-empty `inputRequests`
                    // object; it is the public question verbatim.
                    let question = facts
                        .state
                        .get("inputRequests")
                        .cloned()
                        .unwrap_or_else(|| facts.state.clone());
                    let event = make_event(json!({
                        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"approval_request",
                        "ts":options.event_timestamp(),"call":call,
                        "scope":CONTINUATION_INPUT_SCOPE,"question":question
                    }))?;
                    ledger.append_contract(event, BarrierContext::default())?;
                    announce_appended(ledger, stdout)?;
                    return Ok(true);
                }
                (Some(_), None) => return Ok(true),
                (Some(_), Some((response_seq, grant, answer))) => {
                    if !grant {
                        terminalize(
                            ledger,
                            BackendTerminal::Unavailable {
                                code: "denied".to_owned(),
                                message: "remote task input was declined".to_owned(),
                                retryable: false,
                            },
                        )?;
                        return Ok(false);
                    }
                    if facts.updated_at.is_some_and(|seq| seq > response_seq) {
                        ContinuationOperation::Query
                    } else {
                        ContinuationOperation::Update {
                            input_responses: IJsonValue::parse(&serde_json::to_vec(
                                &answer.unwrap_or_else(|| json!({})),
                            )?)?,
                        }
                    }
                }
            }
        } else {
            ContinuationOperation::Query
        };
        if matches!(action, ContinuationOperation::Query) && std::mem::take(&mut poll_immediately) {
            // The parked interval already elapsed before this run resumed.
        } else if matches!(action, ContinuationOperation::Query) {
            let remote_hint = facts
                .state
                .get("pollIntervalMs")
                .and_then(Value::as_u64)
                .map(Duration::from_millis);
            let wait = remote_hint
                .map_or(interval, |hint| hint.max(interval))
                .min(CONTINUATION_POLL_CEILING);
            if wait >= CONTINUATION_PARK_THRESHOLD && !cancellation.stop_requested() {
                // Park: the due instant is durable, the turn stays open with
                // its unpaired call, and this run releases the line lock. The
                // supervisor spawns the resume when `poll_after` has passed.
                let poll_after = rfc3339_after(wait);
                let state = IJsonValue::parse(&serde_json::to_vec(&facts.state)?)?;
                let mut pipeline =
                    ToolPipeline::new(ledger, assets.clone(), SecretScanner::default());
                pipeline.append_continuation_step(
                    &execution,
                    continuation_id,
                    facts.step,
                    "park",
                    &state,
                    Some((&poll_after, wait.as_millis() as u64)),
                )?;
                announce_appended(ledger, stdout)?;
                return Ok(true);
            }
            std::thread::sleep(wait);
            interval = (interval * 2).min(CONTINUATION_POLL_CEILING);
        }
        let request = ToolContinuationRequest::new(
            original.clone(),
            continuation_id.to_owned(),
            facts.step + 1,
            action.clone(),
        )?;
        let response = match exchange_tool_continuation_runtime(
            &selected,
            &request,
            lines,
            stdout,
            cancellation,
            false,
        ) {
            Ok(response) => response,
            Err(error) if cancellation.stop_requested() && !cancellation.protocol_failed() => {
                let _ = error;
                continue;
            }
            Err(error) => return Err(error),
        };
        match response.result {
            ToolContinuationOutcome::Pending { state, .. } => {
                let action_name = match action {
                    ContinuationOperation::Query => "query",
                    ContinuationOperation::Update { .. } => "update",
                    ContinuationOperation::Cancel => "cancel",
                };
                let mut pipeline =
                    ToolPipeline::new(ledger, assets.clone(), SecretScanner::default());
                pipeline.append_continuation_step(
                    &execution,
                    continuation_id,
                    facts.step + 1,
                    action_name,
                    &state,
                    None,
                )?;
                if action_name == "update" {
                    interval = CONTINUATION_POLL_FLOOR;
                }
            }
            ToolContinuationOutcome::Completed { value } => {
                terminalize(ledger, BackendTerminal::Completed(value))?;
                announce_appended(ledger, stdout)?;
                return Ok(false);
            }
            ToolContinuationOutcome::Failed { error } => {
                terminalize(
                    ledger,
                    BackendTerminal::Unavailable {
                        code: control_error_code_name(&error.code).to_owned(),
                        message: error.message,
                        retryable: error.retryable,
                    },
                )?;
                announce_appended(ledger, stdout)?;
                return Ok(false);
            }
        }
    }
}

fn control_error_code_name(code: &ToolControlErrorCode) -> String {
    serde_json::to_value(code)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_else(|| "internal".to_owned())
}

/// One continuation step over the control channel. `allow_stop` lets a
/// cancellation step through after a stop was requested.
fn exchange_tool_continuation_runtime(
    selected: &Selected,
    request: &ToolContinuationRequest,
    lines: &mut impl Iterator<Item = io::Result<String>>,
    stdout: &mut impl Write,
    cancellation: &RuntimeCancellation,
    allow_stop: bool,
) -> Result<ToolContinuationResponse, Box<dyn std::error::Error>> {
    require_version(selected)?;
    if cancellation.stop_requested() && !allow_stop {
        return Err("tool continuation cancelled by stop".into());
    }
    if cancellation.supervisor_lost() {
        cancellation.mark_protocol_failed();
        return Err(Box::new(ProtocolFailure(
            "supervisor EOF before tool continuation".to_owned(),
        )));
    }
    stdout.write_all(&encode_tool_continuation(request)?)?;
    stdout.flush()?;
    loop {
        let Some(line) = lines.next() else {
            cancellation.cancel(CANCEL_SUPERVISOR_LOSS);
            cancellation.mark_protocol_failed();
            return Err(Box::new(ProtocolFailure(
                "supervisor EOF while awaiting tool continuation result".to_owned(),
            )));
        };
        let line = line.map_err(|error| {
            cancellation.cancel(CANCEL_SUPERVISOR_LOSS);
            cancellation.mark_protocol_failed();
            ProtocolFailure(format!("supervisor read failed: {error}"))
        })?;
        match decode_tool_continuation_result(line.as_bytes()) {
            Ok(response) => {
                if let Err(error) = response.validate_for(request) {
                    cancellation.mark_protocol_failed();
                    return Err(Box::new(ProtocolFailure(error)));
                }
                return Ok(response);
            }
            Err(_) => match decode_supervisor(line.as_bytes()) {
                Ok(SupervisorMessage::Ping(ping)) => {
                    stdout
                        .write_all(&encode_line("pong", &worker_control::Pong { id: ping.id })?)?;
                    stdout.flush()?;
                }
                Ok(SupervisorMessage::Stop(_)) => {
                    cancellation.cancel(CANCEL_STOP);
                    cancellation.defer(line);
                    if !allow_stop {
                        return Err("tool continuation cancelled by stop".into());
                    }
                }
                Ok(SupervisorMessage::QueueTransaction(_)) => cancellation.park_delivery(line),
                Ok(_) => {
                    cancellation.mark_protocol_failed();
                    return Err(Box::new(ProtocolFailure(
                        "unexpected supervisor message while awaiting tool continuation result"
                            .to_owned(),
                    )));
                }
                Err(error) => {
                    cancellation.mark_protocol_failed();
                    return Err(Box::new(ProtocolFailure(format!(
                        "invalid supervisor line while awaiting tool continuation result: {error}"
                    ))));
                }
            },
        }
    }
}

fn append_tool_validation_error(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    call: &provider::ToolCall,
    detail: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"tool_result",
        "ts":options.event_timestamp(),"call":call.call_id,"outcome":"error",
        "content":[{"type":"text","text":bounded_ledger_detail(detail)}],
        "meta":{"classification":"validation","tool":call.name}
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(())
}

fn frozen_hook_bindings(
    instruction: &InstructionSnapshot,
) -> Result<Vec<HookBinding>, Box<dyn std::error::Error>> {
    instruction
        .effective
        .hooks
        .iter()
        .filter(|(_, index)| {
            instruction
                .sources
                .get(**index as usize)
                .and_then(|source| serde_json::from_str::<Value>(&source.content).ok())
                .is_some_and(|value| value["format"] == 1)
        })
        .map(|(logical_name, index)| {
            let source = instruction
                .sources
                .get(*index as usize)
                .ok_or_else(|| format!("effective hook {logical_name:?} has an invalid index"))?;
            decode_hook_binding(source.content.as_bytes(), logical_name)
                .map_err(|error| Box::new(error) as Box<dyn std::error::Error>)
        })
        .collect()
}

fn frozen_skill_catalog(
    instruction: &InstructionSnapshot,
) -> Result<Vec<CatalogEntry>, Box<dyn std::error::Error>> {
    let resources = ResourceCatalog::from_snapshot(instruction)?;
    let mut entries = resources
        .skill_summaries()
        .into_iter()
        .map(|summary| {
            let package = resources.skill(&summary.name)?;
            let content = json!({
                "format": 1,
                "name": summary.name,
                "description": summary.description,
                "body": package.body,
                "resources": package.resources,
            });
            Ok(CatalogEntry {
                name: summary.name,
                summary: summary.description,
                aliases: Vec::new(),
                schema_digest: summary.content_digest,
                content: IJsonValue::parse(&serde_json::to_vec(&content)?)?,
            })
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    let legacy = instruction
        .effective
        .skills
        .iter()
        .filter(|(logical_name, _)| !logical_name.contains('/'))
        .map(|(logical_name, index)| {
            let source = instruction
                .sources
                .get(*index as usize)
                .ok_or_else(|| format!("effective skill {logical_name:?} has an invalid index"))?;
            let summary = source
                .content
                .lines()
                .find(|line| !line.trim().is_empty())
                .map_or_else(|| logical_name.clone(), |line| line.trim().to_owned());
            Ok(CatalogEntry {
                name: logical_name.clone(),
                summary,
                aliases: Vec::new(),
                schema_digest: source.content_sha256.clone(),
                content: IJsonValue::from(source.content.clone()),
            })
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    entries.extend(legacy);
    entries.sort_by(|left, right| left.name.as_bytes().cmp(right.name.as_bytes()));
    Ok(entries)
}

struct ToolCallBatch<'a> {
    options: &'a Options,
    profile: &'a RuntimeProfile,
    selected: Selected,
    attempt: &'a str,
    turn: u64,
    calls: &'a [provider::ToolCall],
    cancellation: &'a RuntimeCancellation,
}

struct ChildSpawnExecution<'a> {
    options: &'a Options,
    profile: &'a RuntimeProfile,
    selected: Selected,
    attempt: &'a str,
    turn: u64,
    call: &'a provider::ToolCall,
    hooks: &'a [HookBinding],
    manifest: &'a BuiltinManifest,
    cancellation: &'a RuntimeCancellation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DurableChildSpawn {
    child: String,
    child_file: String,
    spawn_id: String,
    resume: ResumePolicy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ChildTerminal {
    outcome: String,
    summary: Option<String>,
}

fn execute_child_spawn_tool<I, O>(
    ledger: &mut LockedLedger,
    execution: ChildSpawnExecution<'_>,
    lines: &mut I,
    stdout: &mut O,
) -> Result<bool, Box<dyn std::error::Error>>
where
    I: Iterator<Item = io::Result<String>>,
    O: Write,
{
    let ChildSpawnExecution {
        options,
        profile,
        selected,
        attempt,
        turn,
        call,
        hooks,
        manifest,
        cancellation,
    } = execution;
    if !effective_allowed_tools(profile).contains(&call.name) {
        return Err(format!("child-spawn tool {} is not allowed", call.name).into());
    }
    let thread = ledger
        .projection()
        .and_then(|projection| projection.events.first())
        .and_then(|genesis| genesis.string_field("thread"))
        .ok_or("genesis thread binding is missing")?
        .to_owned();
    let invocation = ToolInvocation {
        thread: thread.clone(),
        turn,
        attempt: attempt.to_owned(),
        call: call.call_id.clone(),
        name: call.name.clone(),
        arguments: IJsonValue::parse(&serde_json::to_vec(&call.arguments)?)?,
        timestamp: options.event_timestamp().clone(),
    };
    let context = tool_catalog_context(profile, false, false)?;
    let dispatcher = ToolDispatcher::new(manifest, &context);
    let assets = AssetStore::new(
        ledger
            .path()
            .parent()
            .ok_or("worker ledger has no thread folder")?
            .join("assets"),
    )?;
    let mut policy = session_permission_policy(
        ledger
            .path()
            .parent()
            .ok_or("worker ledger has no thread folder")?,
    );
    let mut pipeline = ToolPipeline::new(ledger, assets, SecretScanner::default());
    let mut deferred = DeferredChildBackend;
    match dispatcher.dispatch(
        &mut pipeline,
        &invocation,
        hooks,
        &mut policy,
        &mut deferred,
    )? {
        PipelineDecision::Parked { .. } => return Ok(true),
        PipelineDecision::Pending { .. } => {
            return Err("child spawn backend cannot report a remote continuation".into());
        }
        PipelineDecision::Deferred => {}
        PipelineDecision::Completed { .. } => return Ok(false),
    }
    drop(pipeline);

    let effective = durable_invocation(ledger, &call.call_id, &invocation.arguments)?;
    let spawn = match ensure_durable_child(ledger, options, profile, call, &effective) {
        Ok(spawn) => spawn,
        Err(error) if error.downcast_ref::<TaskInputError>().is_some() => {
            append_tool_validation_error(ledger, options, turn, call, &error.to_string())?;
            return Ok(false);
        }
        Err(error) => return Err(error),
    };
    let session = ledger
        .path()
        .parent()
        .and_then(|folder| folder.file_name())
        .and_then(|name| name.to_str())
        .ok_or("thread folder has no UTF-8 session UUID")?
        .to_owned();
    let request = ToolControl::new(
        session,
        thread,
        turn,
        call.call_id.clone(),
        call.name.clone(),
        effective,
    )?;
    let control = exchange_tool_control_runtime(&selected, &request, lines, stdout, cancellation)?;

    if control.error.is_some() {
        append_child_result_once(
            ledger,
            options,
            turn,
            call,
            &spawn,
            &ChildTerminal {
                outcome: "failed".to_owned(),
                summary: control.error.as_ref().map(|error| error.message.clone()),
            },
        )?;
    } else {
        let launch = exchange_launch_child_runtime(&spawn, lines, stdout, cancellation)?;
        if !launch.ok {
            append_child_result_once(
                ledger,
                options,
                turn,
                call,
                &spawn,
                &ChildTerminal {
                    outcome: "failed".to_owned(),
                    summary: launch.error,
                },
            )?;
        } else {
            let terminal =
                wait_for_child_terminal(ledger, &spawn, cancellation, options, &selected, stdout)?;
            append_child_result_once(ledger, options, turn, call, &spawn, &terminal)?;
        }
    }

    let assets = AssetStore::new(
        ledger
            .path()
            .parent()
            .ok_or("worker ledger has no thread folder")?
            .join("assets"),
    )?;
    let mut policy = session_permission_policy(
        ledger
            .path()
            .parent()
            .ok_or("worker ledger has no thread folder")?,
    );
    let mut pipeline = ToolPipeline::new(ledger, assets, SecretScanner::default());
    let mut replayed = ReplayedChildBackend { result: control };
    match dispatcher.dispatch(
        &mut pipeline,
        &invocation,
        hooks,
        &mut policy,
        &mut replayed,
    )? {
        PipelineDecision::Completed { .. } => Ok(false),
        PipelineDecision::Parked { .. } => Ok(true),
        PipelineDecision::Pending { .. } => {
            Err("child result backend cannot report a remote continuation".into())
        }
        PipelineDecision::Deferred => Err("child result backend deferred unexpectedly".into()),
    }
}

fn durable_invocation(
    ledger: &LockedLedger,
    call: &str,
    original: &IJsonValue,
) -> Result<IJsonValue, Box<dyn std::error::Error>> {
    let Some(event) = ledger.projection().and_then(|projection| {
        projection.events.iter().find(|event| {
            event.kind() == &EventKind::EffectiveExecution
                && event.string_field("call") == Some(call)
        })
    }) else {
        return Ok(original.clone());
    };
    let raw = serde_json::to_value(event.raw())?;
    let value = raw
        .get("invocation")
        .ok_or("effective_execution invocation is missing")?;
    Ok(IJsonValue::parse(&serde_json::to_vec(&materialize_json(
        ledger, value,
    )?)?)?)
}

fn ensure_durable_child(
    ledger: &mut LockedLedger,
    options: &Options,
    profile: &RuntimeProfile,
    call: &provider::ToolCall,
    effective: &IJsonValue,
) -> Result<DurableChildSpawn, Box<dyn std::error::Error>> {
    if let Some(spawn) = ledger.projection().and_then(|projection| {
        projection.events.iter().find(|event| {
            event.kind() == &EventKind::Spawn
                && event.string_field("call") == Some(call.call_id.as_str())
        })
    }) {
        let raw = serde_json::to_value(spawn.raw())?;
        let child_file = spawn
            .string_field("child")
            .ok_or("spawn child is missing")?
            .to_owned();
        let child_path = ledger
            .path()
            .parent()
            .ok_or("parent ledger has no folder")?
            .join(&child_file);
        let child_line = child_path
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or("spawn child file has no UTF-8 stem")?
            .to_owned();
        if child_path.exists() {
            let child = read_child_projection(&child_path)?;
            let genesis = child.events.first().ok_or("child genesis is missing")?;
            if genesis.string_field("thread") != Some(child_line.as_str()) {
                return Err("spawn child file does not match child genesis".into());
            }
        }
        return Ok(DurableChildSpawn {
            child: child_line,
            child_file,
            spawn_id: spawn
                .string_field("spawn_id")
                .ok_or("spawn id is missing")?
                .to_owned(),
            resume: serde_json::from_value(
                raw.get("resume")
                    .cloned()
                    .ok_or("spawn resume is missing")?,
            )?,
        });
    }

    let delegation = ensure_delegation_state(ledger, options, call, effective)?;
    let spawn_seq = ledger.next_seq();
    let parent_thread = ledger
        .projection()
        .and_then(|projection| projection.events.first())
        .and_then(|genesis| genesis.string_field("thread"))
        .ok_or("parent genesis thread is missing")?;
    let (child, spawn_id) = child_identity(parent_thread, &call.call_id, spawn_seq);
    let child_file = format!("{child}.jsonl");
    let resume = ResumePolicy::Bounded(3);
    let mut seed_bytes = delegation.canonical_bytes()?;
    seed_bytes.push(b'\n');
    let folder = ledger
        .path()
        .parent()
        .ok_or("parent ledger has no folder")?;
    let assets = AssetStore::new(folder.join("assets"))?;
    let seed = assets.publish(&seed_bytes)?;
    let digest = seed
        .asset
        .strip_prefix("sha256-")
        .ok_or("seed asset name has no sha256 prefix")?;
    let parent_file = ledger
        .path()
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("parent ledger file name is not UTF-8")?;
    let origin_key = format!("spawn:{spawn_id}");
    let mut genesis = json!({
        "v":1,"seq":1,"kind":"genesis","ts":options.event_timestamp(),
        "format":1,"min_reader":1,"min_writer":1,
        "thread":child,"workspace":profile.config.workspace.id,
        "origin_key":origin_key,
        "origin_tuple":{
            "principal":"kernel-worker","client":"worker",
            "target":child,"op":"spawn","key":origin_key
        },
        "parent":{"file":parent_file,"seq":spawn_seq,"spawn_id":spawn_id},
        "seed":{"source":parent_thread,"kinds":["state"],
            "snapshot":{"asset":seed.asset,"digest":digest}},
        "resume":resume,
        "config":{"digest":options.config_digest}
    });
    if let Some(identity) = identity::selected(ledger)? {
        genesis
            .as_object_mut()
            .expect("genesis object")
            .insert("identity_profile".to_owned(), json!(identity.as_str()));
    }
    if !options.instruction_digest.is_empty() {
        genesis.as_object_mut().expect("genesis object").insert(
            "instruction".to_owned(),
            json!({"digest":options.instruction_digest}),
        );
    }
    publish_child_genesis(folder, &child_file, make_event(genesis)?)?;
    let spawn = make_event(json!({
        "v":1,"seq":spawn_seq,"turn":call_turn(ledger, &call.call_id)?,
        "kind":"spawn","ts":options.event_timestamp(),"child":child_file,
        "call":call.call_id,"spawn_id":spawn_id,"resume":resume,
        "seed":{"kinds":["state"]}
    }))?;
    ledger.append_contract(spawn, BarrierContext::default())?;
    Ok(DurableChildSpawn {
        child,
        child_file,
        spawn_id,
        resume,
    })
}

fn ensure_delegation_state(
    ledger: &mut LockedLedger,
    options: &Options,
    call: &provider::ToolCall,
    effective: &IJsonValue,
) -> Result<Event, Box<dyn std::error::Error>> {
    let invocation = serde_json::to_value(effective)?;
    if let Some(existing) = ledger.projection().and_then(|projection| {
        projection.events.iter().find(|event| {
            if event.kind() != &EventKind::State
                || event.string_field("subkind") != Some("delegation")
            {
                return false;
            }
            serde_json::to_value(event.raw())
                .ok()
                .and_then(|value| {
                    value
                        .pointer("/payload/call")
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                })
                .as_deref()
                == Some(call.call_id.as_str())
        })
    }) {
        let raw = serde_json::to_value(existing.raw())?;
        if raw.pointer("/payload/name").and_then(Value::as_str) != Some(call.name.as_str())
            || raw.pointer("/payload/invocation") != Some(&invocation)
        {
            return Err("durable delegation state does not match effective invocation".into());
        }
        return Ok(existing.clone());
    }

    let resolved_inputs = resolve_completed_task_inputs(ledger, &invocation, &call.call_id)?;
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":call_turn(ledger, &call.call_id)?,
        "kind":"state","ts":options.event_timestamp(),"subkind":"delegation",
        "payload":{"call":call.call_id,"name":call.name,"invocation":invocation,
            "resolved_inputs":resolved_inputs}
    }))?;
    ledger.append_contract(event.clone(), BarrierContext::default())?;
    Ok(event)
}

#[derive(Debug)]
struct TaskInputError(String);
impl std::fmt::Display for TaskInputError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}
impl std::error::Error for TaskInputError {}

/// Resolves task-name inputs only from earlier, completed sibling delegations.
/// Paths supplied directly by the model remain ordinary paths and are not
/// rewritten. The durable mapping travels in the child seed so recovery does
/// not depend on re-reading mutable parent state.
fn resolve_completed_task_inputs(
    ledger: &LockedLedger,
    invocation: &Value,
    current_call: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let Some(sources) = invocation.get("input_sources").and_then(Value::as_array) else {
        return Ok(json!([]));
    };
    let projection = ledger.projection().ok_or("task ledger has no projection")?;
    let current_seq = projection
        .events
        .iter()
        .find(|event| {
            event.kind() == &EventKind::ToolCall && event.string_field("call") == Some(current_call)
        })
        .map(Event::seq)
        .ok_or("task call is absent while resolving input sources")?;
    let mut resolved = Vec::new();
    for source in sources {
        let source = source.as_str().ok_or("task input source is not a string")?;
        let matches = projection
            .events
            .iter()
            .filter(|event| {
                event.seq() < current_seq
                    && event.kind() == &EventKind::State
                    && event.string_field("subkind") == Some("delegation")
            })
            .filter_map(|event| {
                let raw = serde_json::to_value(event.raw()).ok()?;
                (raw.pointer("/payload/name").and_then(Value::as_str) == Some("task")
                    && raw
                        .pointer("/payload/invocation/task_name")
                        .and_then(Value::as_str)
                        == Some(source))
                .then_some((event, raw))
            })
            .collect::<Vec<_>>();
        if matches.is_empty() {
            continue;
        }
        // Re-executing a named task supersedes its older version. Never fall
        // back to an old completion while the latest version is unfinished.
        let (delegation, raw) = matches.last().expect("nonempty task matches");
        let prior_call = raw
            .pointer("/payload/call")
            .and_then(Value::as_str)
            .ok_or("prior task delegation lacks call")?;
        let completion = projection.events.iter().find(|event| {
            event.seq() < current_seq
                && event.kind() == &EventKind::ChildResult
                && event.string_field("call") == Some(prior_call)
                && event.string_field("outcome") == Some("completed")
        });
        let Some(completion) = completion else {
            return Err(Box::new(TaskInputError(format!(
                "task input source {source:?} is not completed"
            ))));
        };
        let output = raw
            .pointer("/payload/invocation/output")
            .cloned()
            .ok_or("prior task delegation lacks output contract")?;
        let path = output
            .get("path")
            .and_then(Value::as_str)
            .ok_or("prior task output path is not a string")?;
        resolved.push(json!({
            "source":source,"path":path,"output":output,"summary":completion.string_field("summary"),
            "delegation_seq":delegation.seq(),"completion_seq":completion.seq()
        }));
    }
    Ok(Value::Array(resolved))
}

fn child_identity(parent: &str, call: &str, spawn_seq: u64) -> (String, String) {
    let mut hasher = Sha256::new();
    hasher.update(b"tekes-child-v1\0");
    hasher.update(parent.as_bytes());
    hasher.update([0]);
    hasher.update(call.as_bytes());
    hasher.update([0]);
    hasher.update(spawn_seq.to_string().as_bytes());
    let digest: [u8; 32] = hasher.finalize().into();
    let mut bytes = [0_u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x50;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let mut hex = String::with_capacity(32);
    for byte in bytes {
        use std::fmt::Write as _;
        write!(&mut hex, "{byte:02x}").expect("writing to String cannot fail");
    }
    let child = format!(
        "{}-{}-{}-{}-{}",
        &hex[0..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..32]
    );
    (child, format!("spawn-{}", &hex[..24]))
}

fn call_turn(ledger: &LockedLedger, call: &str) -> Result<u64, Box<dyn std::error::Error>> {
    ledger
        .projection()
        .and_then(|projection| {
            projection.events.iter().find(|event| {
                event.kind() == &EventKind::ToolCall && event.string_field("call") == Some(call)
            })
        })
        .and_then(Event::turn)
        .ok_or_else(|| "tool_call turn is missing".into())
}

fn publish_child_genesis(
    folder: &std::path::Path,
    child_file: &str,
    genesis: Event,
) -> Result<(), Box<dyn std::error::Error>> {
    let destination = folder.join(child_file);
    let mut expected = genesis.canonical_bytes()?;
    expected.push(b'\n');
    if destination.exists() {
        if fs::read(&destination)? == expected {
            return Ok(());
        }
        return Err(format!("child file {child_file} exists with different genesis").into());
    }
    let temp = folder.join(format!(".{child_file}.{}.tmp", std::process::id()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&temp)?;
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        file.write_all(&expected)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temp, &destination)?;
        File::open(folder)?.sync_all()?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

fn exchange_launch_child_runtime<I, O>(
    spawn: &DurableChildSpawn,
    lines: &mut I,
    stdout: &mut O,
    cancellation: &RuntimeCancellation,
) -> Result<LaunchResult, Box<dyn std::error::Error>>
where
    I: Iterator<Item = io::Result<String>>,
    O: Write,
{
    stdout.write_all(&encode_line(
        "launch_child",
        &LaunchChild {
            child: spawn.child.clone(),
            spawn_id: spawn.spawn_id.clone(),
            resume: spawn.resume,
        },
    )?)?;
    stdout.flush()?;
    loop {
        let line = lines.next().ok_or_else(|| {
            cancellation.cancel(CANCEL_SUPERVISOR_LOSS);
            ProtocolFailure("supervisor EOF while awaiting launch_result".to_owned())
        })??;
        match decode_supervisor(line.as_bytes())? {
            SupervisorMessage::LaunchResult(result)
                if result.child == spawn.child && result.spawn_id == spawn.spawn_id =>
            {
                return Ok(result);
            }
            SupervisorMessage::Ping(ping) => {
                stdout.write_all(&encode_line("pong", &worker_control::Pong { id: ping.id })?)?;
                stdout.flush()?;
            }
            SupervisorMessage::Stop(_) => {
                cancellation.cancel(CANCEL_STOP);
                cancellation.defer(line);
                return Err("child launch cancelled by stop".into());
            }
            SupervisorMessage::QueueTransaction(_) => cancellation.park_delivery(line),
            _ => {
                cancellation.mark_protocol_failed();
                return Err(Box::new(ProtocolFailure(
                    "unexpected supervisor message while awaiting launch_result".to_owned(),
                )));
            }
        }
    }
}

fn wait_for_child_terminal(
    parent: &mut LockedLedger,
    spawn: &DurableChildSpawn,
    cancellation: &RuntimeCancellation,
    options: &Options,
    selected: &Selected,
    stdout: &mut impl Write,
) -> Result<ChildTerminal, Box<dyn std::error::Error>> {
    let path = parent
        .path()
        .parent()
        .ok_or("parent ledger has no folder")?
        .join(&spawn.child_file);
    loop {
        if cancellation.stop_requested() {
            return Err("child wait cancelled by stop".into());
        }
        if cancellation.supervisor_lost() {
            return Err(Box::new(ProtocolFailure(
                "supervisor EOF while waiting for child".to_owned(),
            )));
        }
        // A child wait remains a yield point for durable user input. Receipting
        // the input queues it; turn admission still waits for root settlement.
        drain_parked_deliveries(parent, options, selected, cancellation, stdout)?;
        let projection = read_child_projection(&path)?;
        if projection.terminal_tail {
            let settle = projection
                .events
                .iter()
                .rev()
                .find(|event| event.kind() == &EventKind::Settle)
                .ok_or("terminal child has no settle")?;
            let raw = serde_json::to_value(settle.raw())?;
            let outcome = match raw.get("outcome").and_then(Value::as_str) {
                Some("completed") => "completed",
                Some("interrupted") => "interrupted",
                Some("error") => "error",
                _ => return Err("child settle has unknown outcome".into()),
            };
            return Ok(ChildTerminal {
                outcome: outcome.to_owned(),
                summary: child_report_summary(&path, &projection)?,
            });
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}

fn read_child_projection(
    path: &std::path::Path,
) -> Result<schema::LedgerProjection, Box<dyn std::error::Error>> {
    let bytes = fs::read(path)?;
    scan_valid_prefix(&bytes, 1)
        .projection
        .ok_or_else(|| "child ledger has no projection".into())
}

fn child_report_summary(
    path: &std::path::Path,
    projection: &schema::LedgerProjection,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let assets = AssetStore::new(
        path.parent()
            .ok_or("child ledger has no folder")?
            .join("assets"),
    )?;
    for event in projection.events.iter().rev() {
        if event.kind() != &EventKind::ToolCall || event.string_field("name") != Some("report") {
            continue;
        }
        if event.turn() != projection.lifecycle.latest_turn
            || !projection.events.iter().any(|result| {
                result.kind() == &EventKind::ToolResult
                    && result.seq() > event.seq()
                    && result.turn() == event.turn()
                    && result.string_field("call") == event.string_field("call")
                    && result.string_field("outcome") == Some("ok")
            })
        {
            continue;
        }
        let raw = serde_json::to_value(event.raw())?;
        let mut args = raw.get("args").cloned().ok_or("report args are missing")?;
        if let Some(asset) = args
            .get("$spill")
            .and_then(|spill| spill.get("asset"))
            .and_then(Value::as_str)
        {
            args = serde_json::from_slice(&assets.read_verified(asset)?)?;
        }
        if let Some(result) = args.get("result").and_then(Value::as_str) {
            return Ok(Some(bounded_text(result, 4096)));
        }
    }
    Ok(None)
}

fn bounded_text(value: &str, max: usize) -> String {
    if value.len() <= max {
        return value.to_owned();
    }
    let mut end = max;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}[truncated]", &value[..end])
}

fn append_child_result_once(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    call: &provider::ToolCall,
    spawn: &DurableChildSpawn,
    terminal: &ChildTerminal,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(existing) = ledger.projection().and_then(|projection| {
        projection.events.iter().find(|event| {
            event.kind() == &EventKind::ChildResult
                && event.string_field("call") == Some(call.call_id.as_str())
        })
    }) {
        if existing.string_field("child") == Some(spawn.child_file.as_str())
            && existing.string_field("spawn_id") == Some(spawn.spawn_id.as_str())
        {
            return Ok(());
        }
        return Err("existing child_result does not match spawn".into());
    }
    let mut value = json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"child_result",
        "ts":options.event_timestamp(),"child":spawn.child_file,"call":call.call_id,
        "spawn_id":spawn.spawn_id,"outcome":terminal.outcome
    });
    if let Some(summary) = &terminal.summary {
        value
            .as_object_mut()
            .expect("child_result object")
            .insert("summary".to_owned(), Value::String(summary.clone()));
    }
    ledger.append_contract(make_event(value)?, BarrierContext::default())?;
    Ok(())
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

/// Immutable identity of the deferred catalog this turn declares natively.
fn dynamic_catalog_revision(deferred: &[CatalogEntry]) -> String {
    let mut digest = Sha256::new();
    digest.update(b"tekes-native-deferred-catalog-v1\0");
    for entry in deferred {
        digest.update(entry.name.as_bytes());
        digest.update([0]);
        digest.update(entry.schema_digest.as_bytes());
        digest.update([0]);
    }
    format!("sha256-{:x}", digest.finalize())
}

/// This turn's durable successful `tool_search` offers, as native references.
/// Only names in the declared deferred catalog with the exact offered schema
/// digest are carried; unbound names and stale digests are dropped, never
/// promoted.
fn native_search_references(
    ledger: &LockedLedger,
    turn: u64,
    before_seq: u64,
    deferred: &[CatalogEntry],
    catalog_revision: &str,
) -> Result<Vec<provider::DeferredToolReference>, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let searches = projection
        .events
        .iter()
        .filter(|event| {
            event.seq() < before_seq
                && event.turn() == Some(turn)
                && *event.kind() == EventKind::ToolCall
                && event.string_field("name") == Some("tool_search")
        })
        .filter_map(|event| event.string_field("call").map(str::to_owned))
        .collect::<Vec<_>>();
    let mut references = Vec::new();
    let mut seen = BTreeSet::new();
    for call in &searches {
        let Some(result) = projection.events.iter().find(|event| {
            event.seq() < before_seq
                && *event.kind() == EventKind::ToolResult
                && event.string_field("call") == Some(call.as_str())
        }) else {
            continue;
        };
        let raw = serde_json::to_value(result.raw())?;
        if raw.get("outcome").and_then(Value::as_str) != Some("ok") {
            continue;
        }
        let Some(content) = raw.get("content") else {
            continue;
        };
        let content = materialize_json(ledger, content)?;
        for block in content.as_array().into_iter().flatten() {
            let Some(text) = block.get("text").and_then(Value::as_str) else {
                continue;
            };
            let Ok(offer) = serde_json::from_str::<Value>(text) else {
                continue;
            };
            for item in offer
                .get("items")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                let (Some(name), Some(digest)) = (
                    item.get("name").and_then(Value::as_str),
                    item.get("schema_digest").and_then(Value::as_str),
                ) else {
                    continue;
                };
                let bound = deferred
                    .iter()
                    .any(|entry| entry.name == name && entry.schema_digest == digest);
                if bound && seen.insert((call.clone(), name.to_owned())) {
                    references.push(provider::DeferredToolReference {
                        tool_name: name.to_owned(),
                        schema_digest: digest.to_owned(),
                        catalog_revision: catalog_revision.to_owned(),
                        source_search_call_id: call.clone(),
                    });
                }
            }
        }
    }
    Ok(references)
}

#[cfg(test)]
fn project_provider_context(
    ledger: &LockedLedger,
    epoch_id: &str,
    dialect: DialectId,
    turn: u64,
) -> Result<ProviderContext, Box<dyn std::error::Error>> {
    project_provider_context_mode(ledger, epoch_id, dialect, turn, false)
}

/// `stateless` forces full-prefix replay without a server continuation id.
fn project_provider_context_mode(
    ledger: &LockedLedger,
    epoch_id: &str,
    dialect: DialectId,
    turn: u64,
    stateless: bool,
) -> Result<ProviderContext, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let mut attempt_epoch = std::collections::BTreeMap::<String, String>::new();
    let mut attempt_admits = std::collections::BTreeMap::<String, Vec<u64>>::new();
    let mut settled_outputs = std::collections::BTreeSet::<String>::new();
    // Attempts whose response was lost in transport after eager dispatch and
    // whose recoverable `error` seals the completed native items: the carrier
    // stands in for the output as the model-visible assistant turn.
    let mut partial_carriers = std::collections::BTreeSet::<String>::new();
    // Where an attempt's assistant carrier (output or partial) renders; a
    // result of a call dispatched before that carrier renders right after it.
    let mut output_positions = std::collections::BTreeMap::<String, u64>::new();
    let mut call_attempts = std::collections::BTreeMap::<String, String>::new();
    let mut superseded = std::collections::BTreeSet::<u64>::new();
    let mut compacted = std::collections::BTreeSet::<u64>::new();
    let mut input_turn = std::collections::BTreeMap::<u64, u64>::new();
    let mut input_position = std::collections::BTreeMap::<u64, u64>::new();
    let mut call_turn = std::collections::BTreeMap::<String, u64>::new();
    let mut call_name = std::collections::BTreeMap::<String, String>::new();
    let mut spawned_calls = std::collections::BTreeSet::<String>::new();
    let mut open_turn = None;
    let mut continuation_id = None;
    let mut epoch_seq = None;
    let mut epoch_pending = std::collections::BTreeSet::<u64>::new();

    for event in &projection.events {
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("event is not an object")?;
        for seq in seqs_from_ranges(object.get("supersedes"))? {
            superseded.insert(seq);
        }
        if *event.kind() == EventKind::Compact {
            for seq in seqs_from_ranges(object.get("covers"))? {
                compacted.insert(seq);
            }
        }
        match event.kind() {
            EventKind::Epoch if event.string_field("id") == Some(epoch_id) => {
                epoch_seq = Some(event.seq());
                for seq in object
                    .get("pending")
                    .and_then(Value::as_object)
                    .into_iter()
                    .flat_map(|pending| [pending.get("eligible"), pending.get("withheld")])
                    .flatten()
                    .flat_map(|value| value.as_array().into_iter().flatten())
                    .filter_map(Value::as_u64)
                {
                    epoch_pending.insert(seq);
                }
            }
            EventKind::TurnOpen => {
                let event_turn = event.turn().ok_or("turn_open lacks turn")?;
                open_turn = Some(event_turn);
                for seq in object
                    .get("trigger")
                    .and_then(|trigger| trigger.get("inputs"))
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_u64)
                {
                    input_turn.insert(seq, event_turn);
                    input_position.insert(seq, event.seq());
                }
            }
            EventKind::Settle => {
                if open_turn == event.turn() {
                    open_turn = None;
                }
            }
            EventKind::Input if object.get("steer").and_then(Value::as_bool) == Some(true) => {
                if let Some(target) = open_turn {
                    input_turn.insert(event.seq(), target);
                }
            }
            EventKind::Attempt => {
                let id = event
                    .string_field("attempt")
                    .ok_or("attempt lacks id")?
                    .to_owned();
                attempt_epoch.insert(
                    id.clone(),
                    event
                        .string_field("epoch")
                        .ok_or("attempt lacks epoch")?
                        .to_owned(),
                );
                attempt_admits.insert(id, seqs_from_ranges(object.get("admits"))?);
            }
            EventKind::Output => {
                let attempt = event
                    .string_field("attempt")
                    .ok_or("output lacks attempt")?;
                settled_outputs.insert(attempt.to_owned());
                output_positions
                    .entry(attempt.to_owned())
                    .or_insert(event.seq());
                if attempt_epoch.get(attempt).is_some_and(|id| id == epoch_id) {
                    continuation_id = object
                        .get("continuation")
                        .and_then(|continuation| continuation.get("id"))
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                        .or(continuation_id);
                }
            }
            EventKind::Error if object.contains_key("sealed") => {
                let attempt = event
                    .string_field("attempt")
                    .ok_or("sealed error lacks attempt")?;
                partial_carriers.insert(attempt.to_owned());
                output_positions
                    .entry(attempt.to_owned())
                    .or_insert(event.seq());
            }
            EventKind::ToolCall => {
                let call = event.string_field("call").ok_or("tool_call lacks call")?;
                if let Some(attempt) = event.string_field("attempt") {
                    call_attempts.insert(call.to_owned(), attempt.to_owned());
                }
                call_turn.insert(call.to_owned(), event.turn().ok_or("tool_call lacks turn")?);
                call_name.insert(
                    call.to_owned(),
                    event
                        .string_field("name")
                        .ok_or("tool_call lacks name")?
                        .to_owned(),
                );
            }
            EventKind::Spawn => {
                spawned_calls.insert(
                    event
                        .string_field("call")
                        .ok_or("spawn lacks call")?
                        .to_owned(),
                );
            }
            _ => {}
        }
    }

    let admitted_all = attempt_admits
        .iter()
        .filter(|(attempt, _)| settled_outputs.contains(*attempt))
        .flat_map(|(_, seqs)| seqs.iter().copied())
        .collect::<std::collections::BTreeSet<_>>();
    let admitted = attempt_admits
        .iter()
        .filter(|(attempt, _)| {
            settled_outputs.contains(*attempt)
                && attempt_epoch.get(*attempt).is_some_and(|id| id == epoch_id)
        })
        .flat_map(|(_, seqs)| seqs.iter().copied())
        .collect::<std::collections::BTreeSet<_>>();
    let server_managed = dialect.server_managed();
    let incremental_continuation = server_managed && continuation_id.is_some() && !stateless;
    let epoch_seq = epoch_seq.ok_or("active epoch is missing")?;
    let mut items = Vec::new();
    if !incremental_continuation {
        items.extend(seed_provider_items(ledger)?);
    }
    for event in &projection.events {
        if incremental_continuation
            || *event.kind() != EventKind::Compact
            || superseded.contains(&event.seq())
            || compacted.contains(&event.seq())
        {
            continue;
        }
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("compact is not an object")?;
        let summary = materialize_string(
            ledger,
            object.get("summary").ok_or("compact lacks summary")?,
        )?;
        items.push(ijson(json!({
            "role":"user",
            "content":[{"type":"text","text":summary}]
        }))?);
    }
    // Freeze resurrected semantic anchors at the epoch boundary. Re-selecting
    // "latest" after each response could retract an older, already-admitted
    // result that a legacy compact marker covered.
    let boundary = projection
        .events
        .partition_point(|event| event.seq() <= epoch_seq);
    let semantic_anchors = compaction_anchor_sequences(&projection.events[..boundary])?;
    let mut admits = Vec::new();
    // Queued inputs are persisted while the preceding turn is still running.
    // Replay them at admission into their own turn, never between an earlier
    // assistant tool call and the corresponding result. Steers retain chronology.
    // An eagerly dispatched call's result is durable before its response's
    // output. The model must still see the assistant call (sealed in that
    // output) before the result: such results render right after the output.
    // A trimmed result renders where the result it retracts rendered, not at
    // its own seq: it answers the same assistant call, and a provider rejects
    // a call whose output has drifted away from it. `supersedes` points
    // strictly backward (event constraint 7), so the walk terminates.
    let mut replaced_result = std::collections::BTreeMap::<u64, u64>::new();
    for event in &projection.events {
        if *event.kind() != EventKind::ToolResult {
            continue;
        }
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("event is not an object")?;
        if let Some(previous) = seqs_from_ranges(object.get("supersedes"))?
            .into_iter()
            .max()
        {
            replaced_result.insert(event.seq(), previous);
        }
    }
    let render_anchor = |seq: u64| -> u64 {
        let mut anchor = seq;
        while let Some(previous) = replaced_result.get(&anchor) {
            anchor = *previous;
        }
        anchor
    };
    let deferred_result_position = |event: &Event| -> Option<u64> {
        if !matches!(event.kind(), EventKind::ToolResult | EventKind::ChildResult) {
            return None;
        }
        let anchor = render_anchor(event.seq());
        let output = output_positions.get(call_attempts.get(event.string_field("call")?)?)?;
        (*output > anchor).then_some(*output)
    };
    let mut ordered_events = projection.events.iter().collect::<Vec<_>>();
    ordered_events.sort_by_key(|event| {
        let anchor = render_anchor(event.seq());
        if let Some(position) = input_position.get(&anchor) {
            (*position, 0_u8, anchor)
        } else if let Some(position) = deferred_result_position(event) {
            (position, 1, anchor)
        } else {
            (anchor, 0, anchor)
        }
    });
    for event in ordered_events {
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("event is not an object")?;
        let anchored = semantic_anchors.contains(&event.seq());
        if superseded.contains(&event.seq()) || (compacted.contains(&event.seq()) && !anchored) {
            continue;
        }
        // The error stays runtime-visible; only its sealed partial carrier
        // is projected, as the assistant items the lost response produced.
        let partial_carrier = *event.kind() == EventKind::Error
            && object.contains_key("sealed")
            && event
                .string_field("attempt")
                .is_some_and(|attempt| partial_carriers.contains(attempt));
        if !partial_carrier
            && !matches!(
                event.effective_visibility(),
                Visibility::Model | Visibility::DialectDependent
            )
        {
            continue;
        }
        if event.kind() == &EventKind::ToolResult
            && event
                .string_field("call")
                .is_some_and(|call| spawned_calls.contains(call))
        {
            // Spawned calls retain their generic tool_result for the
            // tool_call/tool_result lifecycle fold, but child_result is the
            // sole provider-visible result for that call.
            continue;
        }
        let host_born = matches!(
            event.kind(),
            EventKind::Input | EventKind::ToolResult | EventKind::ChildResult | EventKind::State
        );
        let eligible = match event.kind() {
            EventKind::Input => input_turn.get(&event.seq()).is_some_and(|target| {
                if object.get("steer").and_then(Value::as_bool) == Some(true) {
                    *target == turn || admitted_all.contains(&event.seq())
                } else {
                    *target <= turn
                }
            }),
            EventKind::ToolResult | EventKind::ChildResult => object
                .get("call")
                .and_then(Value::as_str)
                .and_then(|call| call_turn.get(call))
                .is_some_and(|target| *target <= turn),
            EventKind::State => event.turn().is_none_or(|target| target <= turn),
            _ => true,
        };
        let admit_eligible = match event.kind() {
            EventKind::Input => input_turn
                .get(&event.seq())
                .is_some_and(|target| *target == turn),
            EventKind::ToolResult | EventKind::ChildResult => object
                .get("call")
                .and_then(Value::as_str)
                .and_then(|call| call_turn.get(call))
                .is_some_and(|target| *target == turn),
            EventKind::State => event.turn() == Some(turn),
            _ => false,
        };
        let eligible_at_epoch = event.seq() >= epoch_seq || epoch_pending.contains(&event.seq());
        if !eligible
            || (incremental_continuation
                && (!host_born || admitted.contains(&event.seq()) || !eligible_at_epoch))
        {
            continue;
        }
        if !incremental_continuation
            && matches!(event.kind(), EventKind::Reasoning | EventKind::ToolCall)
            && event.string_field("attempt").is_some_and(|attempt| {
                settled_outputs.contains(attempt) || partial_carriers.contains(attempt)
            })
        {
            continue;
        }
        let item =
            render_event_item(ledger, event, object, &call_name, dialect)?.ok_or_else(|| {
                format!(
                    "model-visible {} has no provider projection",
                    event.kind().as_str()
                )
            })?;
        if host_born && admit_eligible && !admitted.contains(&event.seq()) {
            admits.push(event.seq());
        }
        items.push(ijson(item)?);
    }
    admits.sort_unstable();
    admits.dedup();
    Ok(ProviderContext {
        items,
        admits,
        continuation_id: incremental_continuation
            .then_some(continuation_id)
            .flatten(),
    })
}

fn seed_provider_items(
    ledger: &LockedLedger,
) -> Result<Vec<IJsonValue>, Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let Some(genesis) = projection.events.first() else {
        return Ok(Vec::new());
    };
    let raw = serde_json::to_value(genesis.raw())?;
    let Some(asset) = raw.pointer("/seed/snapshot/asset").and_then(Value::as_str) else {
        return Ok(Vec::new());
    };
    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
    let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(asset)?;
    let mut items = Vec::new();
    for line in bytes.split_inclusive(|byte| *byte == b'\n') {
        let canonical = line.strip_suffix(b"\n").ok_or("seed line lacks LF")?;
        if canonical.is_empty() {
            return Err("seed snapshot has an empty line".into());
        }
        let event = Event::decode_canonical(canonical)?;
        if !matches!(
            event.effective_visibility(),
            Visibility::Model | Visibility::DialectDependent
        ) {
            continue;
        }
        let raw = serde_json::to_value(event.raw())?;
        let object = raw.as_object().ok_or("seed event is not an object")?;
        let item = match event.kind() {
            EventKind::Input => json!({
                "role":"user",
                "content":render_blocks(ledger, object.get("content").ok_or("seed input lacks content")?)?
            }),
            EventKind::Output => json!({
                "role":"assistant",
                "content":render_blocks(ledger, object.get("content").ok_or("seed output lacks content")?)?
            }),
            EventKind::Reasoning => json!({
                "role":"assistant",
                "content":[{"type":"reasoning","text":materialize_string(ledger, object.get("content").ok_or("seed reasoning lacks content")?)?}]
            }),
            EventKind::ToolResult => {
                let call = event
                    .string_field("call")
                    .ok_or("seed tool_result lacks call")?;
                let outcome = object
                    .get("outcome")
                    .ok_or("seed tool_result lacks outcome")?;
                json!({
                    "role":"tool",
                    "content":[{
                        "type":"tool_result","call_id":provider_wire_call_id(ledger, call),"name":"tool",
                        "result":object.get("content").map(|value| materialize_json(ledger, value)).transpose()?.unwrap_or_else(|| json!({"outcome":outcome})),
                        "error":outcome != "ok"
                    }]
                })
            }
            EventKind::ChildResult => json!({
                "role":"tool",
                "content":[{
                    "type":"tool_result",
                    "call_id":event.string_field("call").ok_or("seed child_result lacks call")?,
                    "name":"subagent","result":object,
                    "error":object.get("outcome").and_then(Value::as_str) != Some("completed")
                }]
            }),
            EventKind::State if event.string_field("subkind") == Some("delegation") => {
                let payload = object
                    .get("payload")
                    .and_then(Value::as_object)
                    .ok_or("seed delegation state lacks payload")?;
                json!({
                    "role":"user",
                    "content":[{
                        "type":"text",
                        "text":"You are the worker assigned the following delegation. Execute its goal and instructions and produce the specified output. This is your assignment, not a request to call the delegation tool with the same arguments. Use the available tools to do the work; delegate only distinct subtasks when needed. When resolved_inputs binds an input source name, read its path (not the task name); for inline outputs use its summary. These bindings identify completed upstream tasks, not filesystem paths inferred from task names."
                    }, {
                        "type":"text",
                        "text":serde_json_canonicalizer::to_string(&json!({
                            "delegation":payload.get("name").and_then(Value::as_str).ok_or("seed delegation state lacks name")?,
                            "arguments":payload.get("invocation").ok_or("seed delegation state lacks invocation")?,
                            "resolved_inputs":payload.get("resolved_inputs").cloned().unwrap_or_else(|| json!([]))
                        }))?
                    }]
                })
            }
            EventKind::State => json!({
                "role":"user",
                "content":[{"type":"text","text":serde_json_canonicalizer::to_string(object)?}]
            }),
            other => {
                return Err(format!(
                    "model-visible seed kind {} has no provider projection",
                    other.as_str()
                )
                .into());
            }
        };
        items.push(ijson(item)?);
    }
    Ok(items)
}

fn provider_wire_call_id<'a>(ledger: &'a LockedLedger, call: &'a str) -> &'a str {
    ledger
        .projection()
        .and_then(|projection| {
            projection.events.iter().find(|event| {
                *event.kind() == EventKind::ToolCall && event.string_field("call") == Some(call)
            })
        })
        .and_then(|event| event.string_field("provider_call"))
        .unwrap_or(call)
}

/// The provider-native assistant items an `output` (or a transport-lost
/// attempt's `error`) sealed, replayed verbatim through the active adapter.
fn render_sealed_carrier(
    ledger: &LockedLedger,
    object: &Map<String, Value>,
    dialect: DialectId,
    kind: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let sealed = object
        .get("sealed")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("{kind} lacks sealed carrier"))?;
    if sealed.get("version").and_then(Value::as_u64) != Some(1)
        || sealed.get("adapter").and_then(Value::as_str) != Some(dialect.as_str())
    {
        return Err(
            format!("{kind} sealed carrier is incompatible with the active adapter").into(),
        );
    }
    let fragments = materialize_string(
        ledger,
        sealed
            .get("fragments")
            .ok_or("sealed carrier lacks fragments")?,
    )?;
    let fragments = serde_json::to_value(IJsonValue::parse(fragments.as_bytes())?)?;
    Ok(json!({"role":"sealed","adapter":dialect.as_str(),"fragments":fragments}))
}

fn render_event_item(
    ledger: &LockedLedger,
    event: &Event,
    object: &Map<String, Value>,
    call_names: &std::collections::BTreeMap<String, String>,
    dialect: DialectId,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    let item = match event.kind() {
        EventKind::Input => json!({
            "role":"user",
            "content":render_blocks(ledger, object.get("content").ok_or("input lacks content")?)?
        }),
        EventKind::Output => render_sealed_carrier(ledger, object, dialect, "output")?,
        // An error projects only through its sealed partial carrier.
        EventKind::Error if object.contains_key("sealed") => {
            render_sealed_carrier(ledger, object, dialect, "error")?
        }
        EventKind::Reasoning => json!({
            "role":"assistant",
            "content":[{"type":"reasoning","text":materialize_string(ledger, object.get("content").ok_or("reasoning lacks content")?)?}]
        }),
        EventKind::ToolCall => json!({
            "role":"assistant",
            "content":[{
                "type":"tool_call",
                "call_id":event.string_field("provider_call").or_else(|| event.string_field("call")).ok_or("tool_call lacks call")?,
                "name":event.string_field("name").ok_or("tool_call lacks name")?,
                "arguments":materialize_json(ledger, object.get("args").ok_or("tool_call lacks args")?)?
            }]
        }),
        EventKind::ToolResult => {
            let call = event.string_field("call").ok_or("tool_result lacks call")?;
            let outcome = object.get("outcome").ok_or("tool_result lacks outcome")?;
            let name = call_names.get(call).map(String::as_str).unwrap_or("tool");
            let result = if let Some(content) = object.get("content") {
                let mut blocks = render_blocks(ledger, content)?;
                // The ledger body stays canonical JSON; the model reads the
                // fixed tool's plain-text projection of it.
                if let [block] = blocks.as_mut_slice() {
                    if let Some(text) = block
                        .get("text")
                        .and_then(Value::as_str)
                        .filter(|_| block.get("type").and_then(Value::as_str) == Some("text"))
                        .and_then(|text| result_presentation::present(name, text, outcome == "ok"))
                    {
                        *block = json!({"type":"text","text":text});
                    }
                }
                Value::Array(blocks)
            } else {
                json!({"outcome":outcome})
            };
            json!({
                "role":"tool",
                "content":[{
                    "type":"tool_result","call_id":provider_wire_call_id(ledger, call),
                    "name":name,
                    "result":result,
                    "error":outcome != "ok"
                }]
            })
        }
        EventKind::ChildResult => {
            let call = event
                .string_field("call")
                .ok_or("child_result lacks call")?;
            json!({
                "role":"tool",
                "content":[{
                    "type":"tool_result","call_id":provider_wire_call_id(ledger, call),
                    "name":call_names.get(call).map(String::as_str).unwrap_or("subagent"),
                    "result":object,
                    "error":object.get("outcome").and_then(Value::as_str) != Some("completed")
                }]
            })
        }
        EventKind::State => json!({
            "role":"user",
            "content":[{"type":"text","text":serde_json_canonicalizer::to_string(object)?}]
        }),
        _ => return Ok(None),
    };
    Ok(Some(item))
}

fn render_blocks(
    ledger: &LockedLedger,
    value: &Value,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let value = materialize_json(ledger, value)?;
    let blocks = value.as_array().ok_or("content is not a block array")?;
    blocks
        .iter()
        .map(|block| {
            let object = block.as_object().ok_or("block is not an object")?;
            Ok(match object.get("type").and_then(Value::as_str) {
                Some("text") => json!({"type":"text","text":object.get("text").and_then(Value::as_str).ok_or("text block lacks text")?}),
                Some("reasoning") => json!({"type":"reasoning","text":object.get("text").and_then(Value::as_str).ok_or("reasoning block lacks text")?}),
                Some("image") => {
                    let asset = object.get("asset").and_then(Value::as_str).ok_or("image block lacks asset")?;
                    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
                    let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(asset)?;
                    json!({
                        "type":"file","name":asset,
                        "data":base64::engine::general_purpose::STANDARD.encode(bytes),
                        "mime":object.get("mime").and_then(Value::as_str).ok_or("image block lacks mime")?
                    })
                },
                Some("file") => {
                    let asset = object.get("asset").and_then(Value::as_str).ok_or("file block lacks asset")?;
                    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
                    let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(asset)?;
                    json!({
                        "type":"file",
                        "name":object.get("name").and_then(Value::as_str).ok_or("file block lacks name")?,
                        "mime":object.get("mime").and_then(Value::as_str).ok_or("file block lacks mime")?,
                        "bytes":bytes.len(),
                        "data":base64::engine::general_purpose::STANDARD.encode(bytes)
                    })
                },
                Some("tool-call") => json!({
                    "type":"tool_call","call_id":object.get("call").and_then(Value::as_str).ok_or("nested call lacks id")?,
                    "name":object.get("name").and_then(Value::as_str).ok_or("nested call lacks name")?,
                    "arguments":object.get("args").cloned().ok_or("nested call lacks args")?
                }),
                Some("tool-result") => json!({
                    "type":"tool_result","call_id":object.get("call").and_then(Value::as_str).ok_or("nested result lacks call")?,
                    "name":"tool","result":object.get("content").cloned().ok_or("nested result lacks content")?,
                    "error":object.get("error").and_then(Value::as_bool).unwrap_or(false)
                }),
                _ => return Err("unknown block kind".into()),
            })
        })
        .collect()
}

fn materialize_json(
    ledger: &LockedLedger,
    value: &Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    materialize_json_at(ledger.path(), value)
}

fn materialize_json_at(
    ledger_path: &std::path::Path,
    value: &Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let Some(spill) = value
        .as_object()
        .filter(|object| object.len() == 1)
        .and_then(|object| object.get("$spill"))
        .and_then(Value::as_object)
    else {
        return Ok(value.clone());
    };
    let asset = spill
        .get("asset")
        .and_then(Value::as_str)
        .ok_or("spill lacks asset")?;
    let folder = ledger_path.parent().ok_or("ledger has no folder")?;
    let bytes = store::AssetStore::new(folder.join("assets"))?.read_verified(asset)?;
    Ok(serde_json::to_value(IJsonValue::parse(&bytes)?)?)
}

fn materialize_string(
    ledger: &LockedLedger,
    value: &Value,
) -> Result<String, Box<dyn std::error::Error>> {
    let value = materialize_json(ledger, value)?;
    value
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| "spilled value is not a string".into())
}

fn seqs_from_ranges(value: Option<&Value>) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let mut seqs = Vec::new();
    for range in value.and_then(Value::as_array).into_iter().flatten() {
        let from = range
            .get("from")
            .and_then(Value::as_u64)
            .ok_or("range lacks from")?;
        let to = range
            .get("to")
            .and_then(Value::as_u64)
            .ok_or("range lacks to")?;
        seqs.extend(from..=to);
    }
    Ok(seqs)
}

fn effective_system(instruction: &InstructionSnapshot) -> String {
    instruction
        .effective
        .agents
        .iter()
        .filter_map(|index| instruction.sources.get(*index as usize))
        .filter(|source| source.kind == InstructionKind::Agents)
        .map(|source| source.content.as_str())
        .collect::<Vec<_>>()
        .join("\n\n")
}

struct EpochAppend<'a> {
    options: &'a Options,
    dialect: DialectId,
    model: &'a str,
    profile: &'a IJsonValue,
    tools: &'a IJsonValue,
    reason: &'a str,
    pending: &'a [u64],
}

/// Why a new epoch opens when no compatible one exists (context §Epochs):
/// recovery and compaction first, then a renderer upgrade, then — with a
/// previous epoch to compare against — the part of the frozen head that
/// moved: the declared tool catalog, the system profile, or the target
/// model. Only the very first epoch of a file is `initial`.
fn epoch_open_reason(
    previous: Option<&Event>,
    compacted_after: bool,
    reset_epoch: bool,
    tools_digest: &str,
    profile_digest: &str,
) -> &'static str {
    if reset_epoch {
        return "recovery";
    }
    if compacted_after {
        return "compaction";
    }
    let Some(previous) = previous else {
        return "initial";
    };
    if previous.integer_field("renderer") != Some(CONTEXT_RENDERER_VERSION) {
        return "renderer_change";
    }
    let head = serde_json::to_value(previous.raw()).ok();
    let field = |pointer: &str| {
        head.as_ref().and_then(|raw| {
            raw.pointer(pointer)
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
    };
    if field("/tools/digest").as_deref() != Some(tools_digest) {
        return "tool_profile_change";
    }
    if field("/system/digest").as_deref() != Some(profile_digest) {
        return "system_change";
    }
    "model_change"
}

/// Replace one oversized `tool_result` with a bounded head and tail. The
/// replacement retracts the original by `supersedes`, so the call still pairs
/// with exactly one live result, the render skips the original, and the whole
/// output stays on disk for audit, fork and re-compaction.
fn append_tool_result_trim(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    trim: &engine::ToolResultTrim,
) -> Result<(), Box<dyn std::error::Error>> {
    let projection = ledger.projection().ok_or("ledger projection missing")?;
    let original = projection
        .events
        .iter()
        .find(|event| event.seq() == trim.seq)
        .ok_or("trim target is not in the ledger")?;
    let raw = serde_json::to_value(original.raw())?;
    let object = raw.as_object().ok_or("tool_result is not an object")?;
    // The replacement is stamped with the turn that writes it, not the turn
    // of the record it retracts: every turn-bound event carries the latest
    // opened turn (event constraint 8). The pairing to the original is the
    // `call`, and eligibility still follows that call's own turn.
    let content = materialize_json(
        ledger,
        object.get("content").ok_or("tool_result lacks content")?,
    )?;
    let trimmed = engine::trimmed_tool_result_content(&content, trim.seq);
    let mut replacement = json!({
        "v":1, "seq": ledger.next_seq(), "turn": turn, "kind": "tool_result",
        "ts": options.event_timestamp(), "call": trim.call,
        "outcome": object.get("outcome").cloned().unwrap_or_else(|| json!("ok")),
        "content": trimmed,
        "supersedes": [{"from": trim.seq, "to": trim.seq}]
    });
    if let Some(meta) = object.get("meta") {
        replacement
            .as_object_mut()
            .expect("replacement object")
            .insert("meta".to_owned(), meta.clone());
    }
    ledger.append_contract(make_event(replacement)?, BarrierContext::default())?;
    Ok(())
}

fn append_provider_epoch(
    ledger: &mut LockedLedger,
    append: EpochAppend<'_>,
) -> Result<String, Box<dyn std::error::Error>> {
    let EpochAppend {
        options,
        dialect,
        model,
        profile,
        tools,
        reason,
        pending,
    } = append;
    let id = format!("{}-epoch-{}", options.run_id, ledger.next_seq());
    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
    let assets = store::AssetStore::new(folder.join("assets"))?;
    let system_bytes = profile.canonical_bytes()?;
    let system_asset = assets.publish(&system_bytes)?;
    // Content-addressed, so an unchanged catalog costs one blob across every
    // epoch of the thread.
    let tools_bytes = tools.canonical_bytes()?;
    let tools_asset = assets.publish(&tools_bytes)?;
    let event = make_event(json!({
        "v": 1,
        "seq": ledger.next_seq(),
        "kind": "epoch",
        "ts": options.event_timestamp(),
        "id": id,
        "reason": reason,
        "adapter": dialect.as_str(),
        "model": model,
        "system": {
            "asset": system_asset.asset,
            "digest": format!("{:x}", Sha256::digest(&system_bytes))
        },
        "tools": {
            "asset": tools_asset.asset,
            "digest": format!("{:x}", Sha256::digest(&tools_bytes))
        },
        "renderer": CONTEXT_RENDERER_VERSION,
        "pending": {"eligible": pending, "withheld": []}
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(id)
}

fn latest_compatible_epoch(
    ledger: &LockedLedger,
    dialect: DialectId,
    model: &str,
    system_digest: &str,
    tools_digest: &str,
    renderer: u64,
) -> Option<String> {
    // A compatible historical epoch is not a continuation of the current
    // chain. Reusing it after a catalog/profile change can pair later tool
    // results with an older response that never contained those calls.
    let event = ledger
        .projection()?
        .events
        .iter()
        .rev()
        .find(|event| *event.kind() == EventKind::Epoch)?;
    if ledger
        .projection()?
        .events
        .iter()
        .any(|later| later.seq() > event.seq() && *later.kind() == EventKind::Compact)
    {
        return None;
    }
    let raw = serde_json::to_value(event.raw()).ok()?;
    (event.string_field("adapter") == Some(dialect.as_str())
        && event.string_field("model") == Some(model)
        && raw.pointer("/system/digest").and_then(Value::as_str) == Some(system_digest)
        && raw.pointer("/tools/digest").and_then(Value::as_str) == Some(tools_digest)
        && event.integer_field("renderer") == Some(renderer))
    .then(|| event.string_field("id").map(str::to_owned))
    .flatten()
}

fn ranges(seqs: &[u64]) -> Vec<Value> {
    let mut ranges = Vec::new();
    for seq in seqs {
        match ranges.last_mut().and_then(Value::as_object_mut) {
            Some(range)
                if range.get("to").and_then(Value::as_u64) == Some(seq.saturating_sub(1)) =>
            {
                range.insert("to".to_owned(), Value::from(*seq));
            }
            _ => ranges.push(json!({"from": seq, "to": seq})),
        }
    }
    ranges
}

/// The response terminal may name a call again only when this attempt already
/// dispatched it eagerly, with the same name and materialized arguments. Any
/// other durable id — another attempt's call, a replayed id, a duplicate within
/// the manifest — and any eager call the manifest omits fail the terminal.
fn validate_terminal_tool_calls(
    ledger: &LockedLedger,
    attempt: &str,
    eager: &EagerDispatch,
    calls: &[provider::ToolCall],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut durable = std::collections::BTreeMap::<&str, (&str, &str, &Value)>::new();
    let events = &ledger
        .projection()
        .ok_or("ledger projection missing")?
        .events;
    let mut raws = Vec::with_capacity(events.len());
    for event in events {
        if *event.kind() != EventKind::ToolCall {
            continue;
        }
        raws.push((event, serde_json::to_value(event.raw())?));
    }
    for (event, raw) in &raws {
        durable.insert(
            event
                .string_field("call")
                .ok_or("durable call identity missing")?,
            (
                event
                    .string_field("attempt")
                    .ok_or("durable call lacks attempt")?,
                event
                    .string_field("name")
                    .ok_or("durable call lacks name")?,
                raw.get("args").ok_or("durable call args missing")?,
            ),
        );
    }
    let mut incoming = BTreeSet::new();
    for call in calls {
        if call.call_id.is_empty() || !incoming.insert(call.call_id.as_str()) {
            return Err(format!(
                "provider tool call id {} is empty or duplicated",
                call.call_id
            )
            .into());
        }
        let Some((owner, name, args)) = durable.get(call.call_id.as_str()) else {
            continue;
        };
        if *owner != attempt || !eager.contains(&call.call_id) {
            return Err(format!(
                "provider tool call id {} reuses a durable call",
                call.call_id
            )
            .into());
        }
        if *name != call.name || materialize_json(ledger, args)? != call.arguments {
            return Err(format!(
                "response terminal changed eagerly dispatched call {}",
                call.call_id
            )
            .into());
        }
    }
    for call in &eager.calls {
        if !incoming.contains(call.call_id.as_str()) {
            return Err(format!(
                "response terminal omitted eagerly dispatched call {}",
                call.call_id
            )
            .into());
        }
    }
    Ok(())
}

struct TerminalAppend<'a> {
    options: &'a Options,
    manifest: &'a BuiltinManifest,
    eager: &'a EagerDispatch,
    dialect: DialectId,
    server_managed: bool,
    turn: u64,
    attempt: &'a str,
}

fn append_terminal(
    ledger: &mut LockedLedger,
    append: TerminalAppend<'_>,
    mut terminal: ProviderTerminal,
) -> Result<ProviderOutcome, Box<dyn std::error::Error>> {
    let TerminalAppend {
        options,
        manifest,
        eager,
        dialect,
        server_managed,
        turn,
        attempt,
    } = append;
    let mut wire_ids = std::collections::BTreeMap::new();
    for call in &mut terminal.tool_calls {
        repair_provider_call_arguments(dialect, call);
        let wire_id = call.call_id.clone();
        call.call_id = local_provider_call_id(ledger, attempt, &wire_id)?;
        if call.call_id != wire_id {
            wire_ids.insert(call.call_id.clone(), wire_id);
        }
    }
    validate_terminal_tool_calls(ledger, attempt, eager, &terminal.tool_calls)?;
    if server_managed && terminal.response_identity.is_none() {
        return Err("server-managed provider terminal lacks continuation identity".into());
    }
    let mut output_content = Vec::new();
    for content in &terminal.content {
        match content {
            ContentBlock::Text(text) => output_content.push(json!({"type":"text","text":text})),
            ContentBlock::Reasoning(text) => {
                let content = sealed_fragments(ledger, text.as_bytes())?;
                let event = make_event(json!({
                    "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"reasoning",
                    "ts":options.event_timestamp(),"attempt":attempt,"content":content
                }))?;
                ledger.append_contract(event, BarrierContext::default())?;
            }
        }
    }
    for call in &terminal.tool_calls {
        // An eagerly dispatched call is already the durable write-ahead intent
        // (validated above to match); writing it again would reuse its id.
        if eager.contains(&call.call_id) {
            continue;
        }
        append_provider_tool_call_with_wire_id(
            ledger,
            options,
            manifest,
            turn,
            attempt,
            call,
            wire_ids.get(&call.call_id).map(String::as_str),
        )?;
    }
    let native = serde_json_canonicalizer::to_vec(&terminal.sealed_fragments)?;
    let fragments = sealed_fragments(ledger, &native)?;
    let mut output = json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"output",
        "ts":options.event_timestamp(),"attempt":attempt,"content":output_content,
        "final_answer":terminal.is_final_answer(),"usage":usage_object(terminal.usage.as_ref()),
        "sealed":{"version":1,"adapter":dialect.as_str(),"fragments":fragments}
    });
    if server_managed {
        if let Some(identity) = terminal.response_identity.as_deref() {
            output
                .as_object_mut()
                .expect("output object")
                .insert("continuation".to_owned(), json!({"id":identity}));
        }
    }
    let event = make_event(output)?;
    let outcome_seq = event.seq();
    ledger.append_contract(event, BarrierContext::default())?;
    let settle = match terminal.finish_reason {
        FinishReason::Completed if terminal.is_final_answer() => Some(("completed", None)),
        FinishReason::Completed => None,
        FinishReason::Length => Some(("interrupted", Some("budget_tokens"))),
        // A paused turn is resent with its output in history, like a
        // non-final completion; the continuation budget bounds the loop.
        FinishReason::ToolCalls | FinishReason::Paused => None,
        FinishReason::ContextOverflow
        | FinishReason::ContentFilter
        | FinishReason::ProviderError => unreachable!("handled above"),
    };
    Ok(ProviderOutcome {
        seq: outcome_seq,
        settle,
        retry: false,
        retry_classification: None,
        retry_after_seconds: None,
        compact_retry: false,
        tool_calls: terminal.tool_calls,
    })
}

fn append_provider_terminal_error(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    attempt: &str,
    terminal: &ProviderTerminal,
) -> Result<ProviderOutcome, Box<dyn std::error::Error>> {
    let mut value = json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"error",
        "ts":options.event_timestamp(),"attempt":attempt,"recoverable":false,
        "classification":"provider_terminal","usage":usage_object(terminal.usage.as_ref())
    });
    let detail = if terminal.finish_reason == FinishReason::ContentFilter {
        // Anthropic refusals name a category (`cyber`, `bio`, ...); keep it
        // beside the classification so the fallback decision has its input.
        Some(
            match terminal
                .stop_details
                .as_ref()
                .and_then(|details| details.get("category"))
                .and_then(Value::as_str)
            {
                Some(category) => bounded_ledger_detail(&format!("content_filter: {category}")),
                None => "content_filter".to_owned(),
            },
        )
    } else {
        terminal
            .provider_error
            .as_ref()
            .and_then(provider_error_detail)
            .or_else(|| terminal.incomplete_reason.clone())
            .map(|value| bounded_ledger_detail(&value))
    };
    let detail = match (terminal.http_status, detail) {
        (Some(status), Some(detail)) => {
            Some(bounded_ledger_detail(&format!("http_{status}: {detail}")))
        }
        (Some(status), None) => Some(format!("http_{status}")),
        (None, detail) => detail,
    };
    if let Some(detail) = detail {
        value["detail"] = Value::String(detail);
    }
    let event = make_event(value)?;
    let outcome_seq = event.seq();
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(ProviderOutcome {
        seq: outcome_seq,
        settle: Some(("error", Some("provider_terminal"))),
        retry: false,
        retry_classification: None,
        retry_after_seconds: None,
        compact_retry: false,
        tool_calls: Vec::new(),
    })
}

fn provider_error_detail(value: &Value) -> Option<String> {
    let value = value.get("error").unwrap_or(value);
    let fields = ["code", "type", "message"]
        .into_iter()
        .filter_map(|field| {
            value
                .get(field)
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .map(|value| format!("{field}={value}"))
        })
        .collect::<Vec<_>>();
    (!fields.is_empty()).then(|| fields.join(" "))
}

fn append_context_overflow(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    attempt: &str,
    usage: Option<&provider::Usage>,
) -> Result<ProviderOutcome, Box<dyn std::error::Error>> {
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"error",
        "ts":options.event_timestamp(),"attempt":attempt,"recoverable":true,
        "classification":"transport","detail":"context_overflow","usage":usage_object(usage)
    }))?;
    let outcome_seq = event.seq();
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(ProviderOutcome {
        seq: outcome_seq,
        settle: None,
        retry: false,
        retry_classification: None,
        retry_after_seconds: None,
        compact_retry: true,
        tool_calls: Vec::new(),
    })
}

#[allow(clippy::too_many_arguments)]
fn append_provider_failure(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    attempt: &str,
    failure: ProviderFailure,
    cancellation: &RuntimeCancellation,
    dialect: DialectId,
    eager: &EagerDispatch,
) -> Result<ProviderOutcome, Box<dyn std::error::Error>> {
    let (classification, detail, recoverable, retry, settle) = match &failure {
        ProviderFailure::AuthFailure { status, detail } => (
            "provider_terminal",
            detail.as_ref().map_or_else(
                || format!("http_{status}"),
                |detail| format!("http_{status}: {detail}"),
            ),
            false,
            false,
            Some(("error", Some("provider_terminal"))),
        ),
        ProviderFailure::RateLimited {
            status,
            detail,
            retry_after_seconds,
        } => (
            "rate_limit",
            format!(
                "http_{status}{}; retry_after_seconds={}",
                detail
                    .as_ref()
                    .map(|detail| format!(": {detail}"))
                    .unwrap_or_default(),
                retry_after_seconds.map_or_else(|| "unknown".to_owned(), |value| value.to_string())
            ),
            true,
            true,
            None,
        ),
        ProviderFailure::Transport { status, detail, .. } => (
            "transport",
            status.map_or_else(
                || detail.clone(),
                |status| format!("http_{status}: {detail}"),
            ),
            true,
            true,
            None,
        ),
        ProviderFailure::Malformed { detail } => (
            "provider_terminal",
            format!("malformed_response: {detail}"),
            false,
            false,
            Some(("error", Some("provider_terminal"))),
        ),
        ProviderFailure::Cancelled if cancellation.supervisor_lost() => {
            ("transport", "supervisor_lost".to_owned(), true, false, None)
        }
        ProviderFailure::Cancelled if cancellation.stop_requested() => (
            "transport",
            "user_stop".to_owned(),
            false,
            false,
            Some(("interrupted", Some("user_stop"))),
        ),
        ProviderFailure::Cancelled => ("transport", "cancelled".to_owned(), true, false, None),
    };
    let detail = bounded_ledger_detail(&detail);
    let mut error = json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"error",
        "ts":options.event_timestamp(),"attempt":attempt,"recoverable":recoverable,
        "classification":classification,"detail":detail,"usage":usage_object(None)
    });
    if let ProviderFailure::Transport {
        partial_fragments: Some(partial),
        ..
    } = &failure
    {
        if let Some(sealed) = partial_carrier_for_eager_calls(ledger, dialect, eager, partial)? {
            error
                .as_object_mut()
                .expect("error object")
                .insert("sealed".to_owned(), sealed);
        }
    }
    let event = make_event(error)?;
    let outcome_seq = event.seq();
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(ProviderOutcome {
        seq: outcome_seq,
        settle,
        retry,
        retry_classification: retry.then_some(classification),
        retry_after_seconds: match &failure {
            ProviderFailure::RateLimited {
                retry_after_seconds,
                ..
            } => *retry_after_seconds,
            _ => None,
        },
        compact_retry: false,
        tool_calls: Vec::new(),
    })
}

/// The sealed partial carrier for a response that lost its transport after
/// eagerly dispatching calls. Their `tool_call`/`tool_result` records are
/// durable, so the retry replays each result; a provider that binds a call
/// to the native items before it (DeepSeek thinking mode rejects a
/// `function_call` without its `reasoning` item) needs those items verbatim,
/// not the normalized `tool_call` projection. The carrier has the shape of
/// `output.sealed`; it is written only when every eager call is in it, so the
/// renderer can skip the eager `tool_call` records without losing a call.
fn partial_carrier_for_eager_calls(
    ledger: &LockedLedger,
    dialect: DialectId,
    eager: &EagerDispatch,
    partial: &Value,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    if eager.calls.is_empty() {
        return Ok(None);
    }
    let Some(items) = partial.as_array() else {
        return Ok(None);
    };
    let sealed_calls = items
        .iter()
        .filter_map(|item| {
            match item.get("type").and_then(Value::as_str)? {
                "tool_use" => item.get("id"),
                _ => item.get("call_id"),
            }
            .and_then(Value::as_str)
        })
        .collect::<BTreeSet<_>>();
    let complete = eager.calls.iter().all(|call| {
        let wire_id = eager.wire_ids.get(&call.call_id).unwrap_or(&call.call_id);
        sealed_calls.contains(wire_id.as_str())
    });
    if !complete {
        return Ok(None);
    }
    let native = serde_json_canonicalizer::to_vec(partial)?;
    let fragments = sealed_fragments(ledger, &native)?;
    Ok(Some(
        json!({"version":1,"adapter":dialect.as_str(),"fragments":fragments}),
    ))
}

/// Why a provider attempt is waiting and how much retry budget is left.
struct ProviderAdmissionRetry<'a> {
    classification: &'a str,
    retry_after_seconds: Option<u64>,
    retries_left: u8,
}

/// Durable admission wait before a rate-limited or transport retry. The delay is the
/// provider's `Retry-After` when declared, otherwise a doubling backoff from
/// one second; both are clamped to `PROVIDER_ADMISSION_CEILING`. The wait is
/// written as a runtime-visible `state{subkind: "provider_admission"}` record
/// carrying `next_attempt_at`, so a worker killed mid-wait leaves the retry as
/// a durable obligation of the open turn (tail-lifecycle `admission_wait_until`)
/// rather than an unsolicited resume, and a stop or supervisor loss cuts the
/// wait short.
fn wait_provider_admission(
    ledger: &mut LockedLedger,
    options: &Options,
    turn: u64,
    attempt: &str,
    retry: ProviderAdmissionRetry<'_>,
    cancellation: &RuntimeCancellation,
) -> Result<(), Box<dyn std::error::Error>> {
    let ProviderAdmissionRetry {
        classification,
        retry_after_seconds,
        retries_left,
    } = retry;
    let exhausted = PROVIDER_RETRIES.saturating_sub(retries_left);
    let backoff = Duration::from_secs(1u64 << u32::from(exhausted.min(8)));
    let wait = retry_after_seconds
        .map_or(backoff, Duration::from_secs)
        .clamp(PROVIDER_ADMISSION_FLOOR, PROVIDER_ADMISSION_CEILING);
    let next_attempt_at = rfc3339_after(wait);
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"state",
        "ts":options.event_timestamp(),"visibility":"runtime",
        "subkind":PROVIDER_ADMISSION_SUBKIND,
        "payload":{
            "attempt":attempt,"classification":classification,
            "next_attempt_at":next_attempt_at,"wait_ms":wait.as_millis() as u64,
            "retries_left":retries_left,"declared_retry_after":retry_after_seconds.is_some()
        }
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    let deadline = Instant::now() + wait;
    while Instant::now() < deadline {
        if cancellation.stop_requested() || cancellation.supervisor_lost() {
            break;
        }
        std::thread::sleep((deadline - Instant::now()).min(Duration::from_millis(100)));
    }
    Ok(())
}

fn settle_internal_worker_failure(
    ledger: &mut LockedLedger,
    options: &Options,
    error: &(dyn std::error::Error + 'static),
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(projection) = ledger.projection() else {
        return Err("worker ledger projection missing while reporting failure".into());
    };
    if projection.terminal_tail
        || projection.lifecycle.open_hold
        || unresolved_attempt(ledger)?.is_some()
    {
        return Err(error.to_string().into());
    }
    let Some(turn) = projection.latest_turn else {
        return Err(error.to_string().into());
    };
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"error",
        "ts":options.event_timestamp(),"recoverable":false,
        "classification":"internal","detail":bounded_ledger_detail(&error.to_string())
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    append_settle(
        ledger,
        &options.event_timestamp(),
        turn,
        "error",
        Some("internal"),
    )
}

fn bounded_ledger_detail(value: &str) -> String {
    let sanitized = value.replace(['\r', '\n', '\t'], " ");
    let scanned = IJsonValue::parse(
        &serde_json::to_vec(&sanitized).expect("diagnostic string is JSON serializable"),
    )
    .map(|value| SecretScanner::default().scan(&value));
    let sanitized = match scanned {
        Ok(SecretScan::Clean(value) | SecretScan::Redacted(value)) => serde_json::to_value(value)
            .ok()
            .and_then(|value| value.as_str().map(str::to_owned))
            .unwrap_or_else(|| "diagnostic unavailable".to_owned()),
        Ok(SecretScan::Withheld(_)) | Err(_) => "diagnostic withheld".to_owned(),
    };
    let mut end = sanitized.len().min(512);
    while !sanitized.is_char_boundary(end) {
        end -= 1;
    }
    sanitized[..end].to_owned()
}

/// The `usage` object carried by the `output`/`error` that settles an
/// attempt: `unavailable` when the provider reported no figures.
fn usage_object(usage: Option<&provider::Usage>) -> Value {
    let mut value = json!({
        "availability": if usage.is_some() { "reported" } else { "unavailable" }
    });
    if let Some(usage) = usage {
        let object = value.as_object_mut().expect("usage object");
        for (field, figure) in [
            ("input_tokens", usage.input_tokens.as_ref()),
            ("output_tokens", usage.output_tokens.as_ref()),
            ("cache_read", usage.cache_read.as_ref()),
            ("cache_miss", usage.cache_miss.as_ref()),
            ("cache_write", usage.cache_write.as_ref()),
            ("reasoning_tokens", usage.reasoning_tokens.as_ref()),
        ] {
            if let Some(figure) = figure {
                object.insert(field.to_owned(), Value::String(figure.clone()));
            }
        }
    }
    value
}

fn record_input_transformations(
    ledger: &mut LockedLedger,
    timestamp: &str,
    turn: u64,
    attempt: &str,
    changes: &Value,
) -> Result<(), Box<dyn std::error::Error>> {
    // Runtime visibility keeps diagnostics out of the cached prompt. Large
    // reports use the existing spill carrier instead of exceeding ledger limits.
    let changes = sealed_fragments(ledger, &serde_json_canonicalizer::to_vec(changes)?)?;
    let event = make_event(json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"state",
        "ts":timestamp,"visibility":"runtime",
        "subkind":"provider_input_transformations",
        "payload":{"attempt":attempt,"input_transformations":changes}
    }))?;
    ledger.append_contract(event, BarrierContext::default())?;
    Ok(())
}

fn sealed_fragments(
    ledger: &LockedLedger,
    bytes: &[u8],
) -> Result<Value, Box<dyn std::error::Error>> {
    let text = std::str::from_utf8(bytes)?;
    let encoded = serde_json_canonicalizer::to_vec(&Value::String(text.to_owned()))?;
    if encoded.len() <= 16 * 1024 {
        return Ok(Value::String(text.to_owned()));
    }
    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
    let asset = store::AssetStore::new(folder.join("assets"))?.publish(&encoded)?;
    Ok(json!({"$spill":{"asset":asset.asset,"bytes":asset.bytes}}))
}

fn spill_json(ledger: &LockedLedger, value: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    let bytes = serde_json_canonicalizer::to_vec(value)?;
    if bytes.len() <= 16 * 1024 {
        return Ok(value.clone());
    }
    let folder = ledger.path().parent().ok_or("ledger has no folder")?;
    let asset = AssetStore::new(folder.join("assets"))?.publish(&bytes)?;
    Ok(json!({"$spill":{"asset":asset.asset,"bytes":asset.bytes}}))
}

fn append_settle(
    ledger: &mut LockedLedger,
    timestamp: &str,
    turn: u64,
    outcome: &str,
    reason: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut value = json!({
        "v":1,"seq":ledger.next_seq(),"turn":turn,"kind":"settle",
        "ts":timestamp,"outcome":outcome
    });
    if let Some(reason) = reason {
        value.as_object_mut().expect("settle object").insert(
            if outcome == "error" {
                "classification"
            } else {
                "reason"
            }
            .to_owned(),
            Value::String(reason.to_owned()),
        );
    }
    ledger.append_contract(make_event(value)?, BarrierContext::default())?;
    Ok(())
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
