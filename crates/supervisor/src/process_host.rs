//! Resident worker process table and the concrete Slice-9 endpoint authorities.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::Write as _;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, TryLockError, Weak};
use std::time::{Duration, Instant, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use endpoint::{
    AUTOMATIC_TITLE_REFINE_OPERATION, AUTOMATIC_TITLE_SEED_OPERATION, AcceptedStreamFrame,
    EndpointJournal, ManagementStore, MaterializedPrompt, MutationReceipt, NativeEndpoint,
    Projector, SESSION_NOTICE_OPERATION, SessionCreateOperation, SessionInventoryItem,
    SessionNotice, SessionNoticeSeverity,
};
use engine::{
    AdmissionLease, AdmissionPool, EnsureAction, LockFacts, TailState, classify, ensure_action_at,
};
use plugins::{MacOsNativeHelperVerifier, PluginStore};
use profile::{ConfigRepository, ConfigSnapshot, DynamicToolCatalog};
use schema::{Block, Event, EventKind, IJsonValue, OriginTuple};
use sha2::{Digest, Sha256};
use store::{AssetStore, LockedLedger, ThreadStore, scan_valid_prefix};
use worker_control::{
    ApprovalResponse, Frame, FrameChannel, Input, LaunchChild, LaunchResult, Lease, Meta, Receipt,
    Reject, Stop, WorkerMessage, decode_hello, decode_worker, encode_line,
};
use worker_control::{
    QueueTransaction, QueueTransactionResult, Selection, WorkerStartup, decode_tool_control,
    encode_queue_transaction,
};

use crate::dynamic_bindings::resolve_worker_launch_bindings;
use crate::endpoint_carrier::{
    HoldCatalog, HoldRef, LiveRespondAuthority, ProductionCarrierError, ProductionCarrierStreams,
    SessionGateError, SupervisorSessionAuthority, VerifiedHolds,
};
use crate::endpoint_host::ProductionRouteFailure;
use crate::endpoint_host::{
    ProviderReadinessAuthority, QueueTransactionAuthority, RuntimeModelReadiness,
    RuntimeProviderFailure, RuntimeProviderReadiness, RuntimeProviderStatus,
    SessionDeliveryAuthority,
};
use crate::host_runtime::DaemonError;
use crate::mcp_runtime::McpRuntime;
use crate::production_tool_control::{
    ChildLaunchProof, DeliveryRequest, DynamicSupervisorAuthority, InterruptRequest,
    JobBrokerSupervisorAuthority, ParentReportProof, ProductionToolControlHandler,
    ProductionToolControlPolicy, SupervisorOperationError, SupervisorRuntimeAuthority,
};
use crate::tool_control::ToolControlSession;
use crate::{
    ProfiledWorkerLaunchSpec, launch_profiled_worker_with_secret_store_and_binding_resolver,
};

const DELIVERY_TIMEOUT: Duration = Duration::from_secs(30);
const MCP_ONLY_APP_SANDBOX_MARKER: &str = ".mcp-only-app-sandbox";
const WORKER_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
const WORKER_HELLO_MAX_BYTES: usize = 4_096;
/// Seconds between two retries of pending management operations.
const MANAGEMENT_RECOVERY_SECONDS: u64 = 30;
const PERIODIC_SWEEP_INTERVAL: Duration = Duration::from_secs(1);

/// What the sweep can tell about a ledger from its metadata alone: the bytes are the same while
/// all three hold, because a ledger only ever grows or is truncated by repair.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SweepLedgerIdentity {
    inode: u64,
    length: u64,
    modified_nanos: i128,
}

impl SweepLedgerIdentity {
    fn of(path: &Path) -> Result<Self, DaemonError> {
        let metadata = fs::metadata(path).map_err(DaemonError::io)?;
        Ok(Self {
            inode: metadata.ino(),
            length: metadata.len(),
            modified_nanos: i128::from(metadata.mtime()) * 1_000_000_000
                + i128::from(metadata.mtime_nsec()),
        })
    }
}

/// The sweep's view of one ledger, valid for exactly one file identity.
#[derive(Debug)]
struct SweepLedgerScan {
    identity: SweepLedgerIdentity,
    last_seq: u64,
    lifecycle: schema::LifecycleFacts,
    /// The line's thread id from its genesis.
    line: String,
    /// A descendant line's parent ledger file name, from its genesis.
    parent_file: Option<String>,
}
/// The cheap model a thread title is written with, matched as a substring of the
/// model id or its dialect profile. A model id is chosen by whoever writes
/// `providers.json` — Tekes ships `deepseek-flash` — so pinning one exact
/// spelling silently disabled titling for every real installation and left the
/// seeded prompt as the thread's name (2026-09-20).
const AUTOMATIC_THREAD_TITLE_MODEL_HINT: &str = "flash";
const AUTOMATIC_THREAD_TITLE_MAX_CHARS: usize = 40;
const AUTOMATIC_THREAD_TITLE_TIMEOUT: Duration = Duration::from_secs(30);
/// Output ceiling of the title request. A 40-character title needs far less;
/// a model that answers the prompt instead stops at the cap with a length
/// terminal, and the deterministic seed title stays.
const AUTOMATIC_THREAD_TITLE_MAX_OUTPUT_TOKENS: u64 = 64;
const COMPACTION_SUMMARY_TIMEOUT: Duration = Duration::from_secs(120);
const AUTOMATIC_THREAD_TITLE_SYSTEM: &str = "Generate a concise conversation title from the user's first message. Return only the title, on one line, with no quotes, markdown, explanation, or trailing punctuation. Preserve the user's language. Maximum 40 characters.";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LineLockState {
    Free,
    Busy,
}

fn production_sandbox_backend() -> tools::SandboxBackend {
    if cfg!(target_os = "macos") {
        tools::SandboxBackend::DarwinSeatbeltV1
    } else {
        tools::SandboxBackend::LinuxAutoV1
    }
}

/// Wall clock in the ledger's RFC 3339 UTC millisecond form, comparable
/// lexically with event timestamps and continuation `poll_after` instants.
/// Consecutive seqs collapse into one `{from,to}` range (event seqrange).
fn seq_ranges(seqs: &[u64]) -> Vec<serde_json::Value> {
    let mut ranges: Vec<serde_json::Value> = Vec::new();
    for seq in seqs {
        match ranges.last_mut().and_then(serde_json::Value::as_object_mut) {
            Some(range)
                if range.get("to").and_then(serde_json::Value::as_u64)
                    == Some(seq.saturating_sub(1)) =>
            {
                range.insert("to".to_owned(), serde_json::Value::from(*seq));
            }
            _ => ranges.push(serde_json::json!({"from": seq, "to": seq})),
        }
    }
    ranges
}

/// A compaction summary is inline up to 16 KiB of canonical JSON and spilled
/// to the line's asset store beyond that — the worker's `sealed_fragments`
/// rule, so both authors produce the same durable shape.
fn spill_compaction_summary(
    ledger: &store::LockedLedger,
    summary: &str,
) -> Result<serde_json::Value, store::StoreError> {
    let encoded = serde_json_canonicalizer::to_vec(&serde_json::Value::String(summary.to_owned()))
        .map_err(|error| store::StoreError::Corruption(error.to_string()))?;
    if encoded.len() <= 16 * 1024 {
        return Ok(serde_json::Value::String(summary.to_owned()));
    }
    let folder = ledger
        .path()
        .parent()
        .ok_or_else(|| store::StoreError::Corruption("ledger has no folder".to_owned()))?;
    let asset = AssetStore::new(folder.join("assets"))?.publish(&encoded)?;
    Ok(serde_json::json!({"$spill": {"asset": asset.asset, "bytes": asset.bytes}}))
}

fn now_rfc3339() -> String {
    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

fn session_event_time_now() -> Result<f64, DaemonError> {
    session_event_time(std::time::SystemTime::now())
}

fn session_event_time(time: std::time::SystemTime) -> Result<f64, DaemonError> {
    let millis = time
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| DaemonError::protocol(error.to_string()))?
        .as_millis();
    let millis = u64::try_from(millis)
        .map_err(|_| DaemonError::protocol("session event time exceeds u64"))?;
    Ok(millis as f64)
}

fn compact_title_source(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn bounded_title(value: &str) -> String {
    value
        .chars()
        .take(AUTOMATIC_THREAD_TITLE_MAX_CHARS)
        .collect()
}

fn deterministic_automatic_thread_title(input: &str) -> String {
    bounded_title(&compact_title_source(input))
}

fn normalized_automatic_thread_title(raw: &str) -> String {
    let mut value = compact_title_source(raw);
    loop {
        value = value.trim().to_owned();
        let Some(first) = value.chars().next() else {
            break;
        };
        if !"#`\"'“‘".contains(first) {
            break;
        }
        value.drain(..first.len_utf8());
    }
    loop {
        value = value.trim().to_owned();
        let Some(last) = value.chars().next_back() else {
            break;
        };
        if !"`\"'”’。.!！?？:：;；".contains(last) {
            break;
        }
        value.truncate(value.len() - last.len_utf8());
    }
    bounded_title(&value)
}

fn prompt_text(blocks: &[Block]) -> String {
    blocks
        .iter()
        .filter_map(|block| match block {
            Block::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn automatic_title_origin(session_id: &str, operation: &str) -> OriginTuple {
    OriginTuple {
        principal: "host".to_owned(),
        client: "tekes-supervisor".to_owned(),
        target: session_id.to_owned(),
        op: operation.to_owned(),
        key: format!("{operation}:{session_id}"),
    }
}

fn automatic_title_route(
    providers: &profile::ProvidersConfig,
) -> Result<
    (
        profile::Provider,
        profile::Model,
        provider::ResolvedDialectProfile,
    ),
    String,
> {
    // Prefer a flash model; fall back to any enabled DeepSeek model rather than
    // leave the thread named after its raw prompt.
    let mut fallback = None;
    for configured in &providers.providers {
        for model in configured.models.iter().filter(|model| model.enabled) {
            let Ok(resolved) = provider::resolve_profile(configured, model) else {
                continue;
            };
            if !matches!(
                resolved.dialect,
                provider::DialectId::DeepseekResponsesV1 | provider::DialectId::DeepseekChatV1
            ) {
                continue;
            }
            let route = (configured.clone(), model.clone(), resolved);
            if model.id.contains(AUTOMATIC_THREAD_TITLE_MODEL_HINT)
                || model.profile.contains(AUTOMATIC_THREAD_TITLE_MODEL_HINT)
            {
                return Ok(route);
            }
            fallback.get_or_insert(route);
        }
    }
    fallback.ok_or_else(|| "no enabled DeepSeek model is configured".to_owned())
}

/// The session's selected model, else the configured defaults, else the first
/// enabled model: the route of supervisor-side requests made on the session's
/// behalf (the compaction summary request).
fn session_model_route(
    config: &profile::ConfigSnapshot,
) -> Result<
    (
        profile::Provider,
        profile::Model,
        provider::ResolvedDialectProfile,
    ),
    String,
> {
    let (provider_id, model_id) = match (
        &config.session_settings,
        &config.settings.default_provider,
        &config.settings.default_model,
    ) {
        (Some(session), _, _) => (session.provider.clone(), session.model.clone()),
        (None, Some(provider_id), Some(model_id)) => (provider_id.clone(), model_id.clone()),
        (None, _, _) => {
            let configured = config
                .providers
                .providers
                .iter()
                .find(|configured| configured.models.iter().any(|model| model.enabled))
                .ok_or("no enabled provider model is configured")?;
            let model = configured
                .models
                .iter()
                .find(|model| model.enabled)
                .expect("provider with an enabled model");
            (configured.id.clone(), model.id.clone())
        }
    };
    let configured = config
        .providers
        .providers
        .iter()
        .find(|configured| configured.id == provider_id)
        .ok_or_else(|| format!("provider {provider_id} is not configured"))?;
    let model = configured
        .models
        .iter()
        .find(|model| model.enabled && model.id == model_id)
        .ok_or_else(|| format!("model {model_id} is not enabled on {provider_id}"))?;
    let resolved = provider::resolve_profile(configured, model)
        .map_err(|error| format!("session model profile is invalid: {error}"))?;
    Ok((configured.clone(), model.clone(), resolved))
}

fn prepare_automatic_title_request(
    configured: &profile::Provider,
    resolved: &provider::ResolvedDialectProfile,
    source: &str,
    attempt_id: String,
) -> Result<provider::PreparedRequest, String> {
    prepare_automatic_text_request(
        configured,
        resolved,
        AUTOMATIC_THREAD_TITLE_SYSTEM,
        source,
        attempt_id,
    )
}

/// One no-reasoning, no-tools, non-streaming text request for a supervisor
/// automatic helper (title naming).
fn prepare_automatic_text_request(
    configured: &profile::Provider,
    resolved: &provider::ResolvedDialectProfile,
    system: &str,
    source: &str,
    attempt_id: String,
) -> Result<provider::PreparedRequest, String> {
    // Deliberately do not call `epoch_profile` here: that helper applies the
    // configured model's default reasoning effort. Presentation naming is a
    // no-reasoning, no-tools request regardless of the session's settings.
    // Omitting the effort is not enough where the model thinks by default:
    // deepseek-flash given a long coding request reasoned for 25k tokens and
    // began implementing it, past the title timeout. Reasoning is always
    // disabled explicitly; a dialect without an off switch fails to prepare,
    // and the deterministic seed title stays.
    let epoch_profile = IJsonValue::parse(
        &serde_json_canonicalizer::to_vec(&serde_json::json!({
            "controls": {
                "title_max_output_tokens": AUTOMATIC_THREAD_TITLE_MAX_OUTPUT_TOKENS,
                "reasoning_disabled": true,
            },
            "serializer_revision": resolved.serializer_revision,
            "system": system,
            "target": resolved.target,
        }))
        .map_err(|error| format!("request profile: {error}"))?,
    )
    .map_err(|error| format!("request profile: {error}"))?;
    let rendered = IJsonValue::parse(
        &serde_json_canonicalizer::to_vec(&serde_json::json!({
            "content": [{"text": source, "type": "text"}],
            "role": "user",
        }))
        .map_err(|error| format!("request input: {error}"))?,
    )
    .map_err(|error| format!("request input: {error}"))?;
    let tools = IJsonValue::parse(b"[]").map_err(|error| format!("request tools: {error}"))?;
    provider::prepare(&provider::PrepareInput {
        attempt_id,
        target: resolved.target.clone(),
        endpoint: configured.endpoint.clone(),
        epoch_profile,
        continuation_id: None,
        rendered_items: vec![rendered],
        tool_catalog: tools,
        stream: false,
    })
    .map_err(|error| format!("prepare: {error}"))
}

#[cfg(test)]
fn accepted_stream_event(
    result: Result<endpoint::SessionEvent, endpoint::ProjectionError>,
) -> Result<Option<endpoint::SessionEvent>, DaemonError> {
    match result {
        Ok(event) => Ok(Some(event)),
        // The worker writes frames to its control pipe before it appends the terminal output, but
        // the ledger write can become visible while the supervisor is still draining those pipe
        // bytes. Reconciliation has already published the durable assistant message, so a queued
        // frame for that terminal attempt is redundant rather than a worker failure.
        Err(endpoint::ProjectionError::TerminalAttempt(_)) => Ok(None),
        Err(error) => Err(DaemonError::protocol(error.to_string())),
    }
}

#[derive(Clone)]
struct ProcessToolRuntime {
    host: Weak<ProductionProcessHost>,
}

pub struct ProductionProcessHost {
    root: PathBuf,
    /// Writes `thread-card.json` off the doorbell path (G-BOOT-1 S2).
    card_writer: crate::thread_card::CardWriter,
    repository: ConfigRepository,
    endpoint: NativeEndpoint,
    worker_binary: PathBuf,
    build: String,
    user_agent_dir: PathBuf,
    secret_store: Arc<dyn provider::SecretStore>,
    mcp_runtime: Arc<McpRuntime>,
    plugin_store: Option<Arc<Mutex<PluginStore<MacOsNativeHelperVerifier>>>>,
    schedule: schedule::ScheduleAuthority,
    workers: Mutex<HashMap<String, Arc<WorkerHandle>>>,
    /// Pre-existing sessions the client has released with `sessions.recover` (DESIGN §6). A
    /// session is deferred iff it is neither admitted (created/forked/unarchived by this
    /// process, `store::folder_events`) nor in this set.
    released: Mutex<HashSet<String>>,
    catalog: Arc<crate::catalog::Catalog>,
    /// Opened once: `ThreadStore::open` probes the filesystem (a full sync), which a per-row
    /// `running` check must not repeat.
    threads: ThreadStore,
    /// Per-thread single-flight gates of the hydrator.
    hydrating: Mutex<HashMap<String, Arc<Mutex<()>>>>,
    hydrate_failures: Mutex<HashMap<String, Instant>>,
    /// Stale ephemeral threads the last cleanup found busy; the 30 s tick retries while non-zero.
    stale_ephemeral_busy: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    hydrate_runs: AtomicU64,
    #[cfg(test)]
    card_builds: AtomicU64,
    #[cfg(test)]
    verify_order: Mutex<Vec<String>>,
    #[cfg(test)]
    list_wait_override: Mutex<Option<Duration>>,
    file_observations: crate::file_observation::SessionFileObservations,
    pending_workers: Mutex<BTreeMap<String, PendingWorker>>,
    workers_changed: Condvar,
    admission: Arc<Mutex<AdmissionPool>>,
    admission_waiters: Mutex<VecDeque<String>>,
    admission_changed: Condvar,
    next_run: AtomicU64,
    max_workers: AtomicU64,
    max_provider_leases: AtomicU64,
    self_weak: Weak<ProductionProcessHost>,
    streams: Mutex<Option<ProductionCarrierStreams>>,
    sweep_scans: Mutex<HashMap<PathBuf, Arc<SweepLedgerScan>>>,
    projection_repair_failures: Mutex<HashSet<String>>,
    context_projection_retries: Mutex<HashSet<String>>,
    protocol_quarantine: Mutex<BTreeSet<WorkerProtocolQuarantine>>,
    projection_lock: Mutex<()>,
    projection_cache: Mutex<HashMap<String, SessionProjectionCache>>,
    sweep_lock: Mutex<()>,
    periodic_sweep_started: AtomicBool,
    schedule_timer_started: AtomicBool,
    schedule_failure: Mutex<Option<String>>,
    restart_failures: Mutex<HashMap<String, RestartBackoff>>,
    draining: AtomicBool,
    #[cfg(test)]
    automatic_title_test_redirect: Mutex<Option<String>>,
}

struct SessionProjectionCache {
    kernel_file_len: u64,
    projector: Projector,
    next_frames: HashMap<String, u64>,
}

struct WorkerHandle {
    /// Held for the complete process lifetime so a workspace relocation can
    /// acquire the matching exclusive lock without a check-then-spawn race.
    _workspace_quiescence: store::NamedLock,
    stdin: Mutex<ChildStdin>,
    child: Mutex<Child>,
    state: Mutex<WorkerState>,
    stderr_tail: Mutex<Vec<u8>>,
    changed: Condvar,
    alive: AtomicBool,
    tool_launch_policy: tools::JobLaunchPolicy,
    tool_launcher: Arc<tools::HelperJobLauncher>,
    credential_control: Mutex<Option<provider::CredentialBrokerControl>>,
    credential_broker: Mutex<Option<std::thread::JoinHandle<Result<(), String>>>>,
    credential_bindings: Mutex<Option<provider::ResolvedCredentialBindings>>,
    config_snapshot: ConfigSnapshot,
    dynamic_catalog: DynamicToolCatalog,
    dynamic_authority: Option<Arc<dyn DynamicSupervisorAuthority>>,
    tool_cancellation: tools::CancellationToken,
}

struct WorkerHandshakeGuard {
    child: Option<Child>,
    credential_control: Option<provider::CredentialBrokerControl>,
    credential_broker: Option<std::thread::JoinHandle<Result<(), String>>>,
}

impl WorkerHandshakeGuard {
    fn child_mut(&mut self) -> &mut Child {
        self.child.as_mut().expect("handshake guard owns child")
    }

    fn take_child(&mut self) -> Child {
        self.child.take().expect("handshake guard owns child")
    }

    fn take_credential_control(&mut self) -> Option<provider::CredentialBrokerControl> {
        self.credential_control.take()
    }

    fn take_credential_broker(&mut self) -> Option<std::thread::JoinHandle<Result<(), String>>> {
        self.credential_broker.take()
    }

    fn wait_for_exit(&mut self, timeout: Duration) {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            match self.child_mut().try_wait() {
                Ok(Some(_)) => return,
                Ok(None) => std::thread::sleep(Duration::from_millis(10)),
                Err(_) => return,
            }
        }
    }
}

impl Drop for WorkerHandshakeGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        drop(self.credential_control.take());
        if let Some(join) = self.credential_broker.take() {
            let _ = join.join();
        }
    }
}

#[derive(Clone)]
struct PendingWorker {
    session_id: String,
    ledger: PathBuf,
    startup: Option<QueueTransaction>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WorkerProtocolQuarantine {
    binary_digest: [u8; 32],
    min: u64,
    max: u64,
}

#[derive(Clone, Debug)]
struct RestartBackoff {
    reason_digest: [u8; 32],
    failures: u8,
    next_ensure_at: Instant,
    exhausted: bool,
}

struct ScheduledWorker {
    worker: Option<Arc<WorkerHandle>>,
    startup_preloaded: bool,
}

#[derive(Default)]
struct WorkerState {
    receipts: HashMap<String, Receipt>,
    queue_results: HashMap<String, QueueTransactionResult>,
    failure: Option<String>,
    held_attempts: HashSet<String>,
}

fn reconcile_credential_bindings(
    worker: &WorkerHandle,
    previous: &provider::ResolvedCredentialBindings,
    current: &provider::ResolvedCredentialBindings,
) -> Result<(), DaemonError> {
    let ids = previous
        .availability
        .keys()
        .chain(current.availability.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    for credential_id in ids {
        let before = previous.availability.get(&credential_id);
        let after = current.availability.get(&credential_id);
        use provider::CredentialAvailability::{Active, NotFound, Revoked, Unavailable};
        match (before, after) {
            (
                Some(Active {
                    generation: before_generation,
                }),
                Some(Active {
                    generation: after_generation,
                }),
            ) if after_generation == before_generation => {
                if active_material(previous, &credential_id)
                    != active_material(current, &credential_id)
                {
                    return Err(DaemonError::required_broker(
                        "credential-generation-conflict",
                    ));
                }
            }
            (
                Some(Active {
                    generation: before_generation,
                }),
                Some(Active {
                    generation: after_generation,
                }),
            ) if after_generation > before_generation => {
                let material = active_material(current, &credential_id).ok_or_else(|| {
                    DaemonError::required_broker("credential-active-material-missing")
                })?;
                credential_control(worker)?
                    .rotate(
                        credential_id.clone(),
                        after_generation.to_string(),
                        material.to_owned(),
                    )
                    .map_err(|_| DaemonError::required_broker("credential-rotation-failed"))?;
            }
            (
                Some(Revoked {
                    generation: before_generation,
                }),
                Some(Revoked {
                    generation: after_generation,
                }),
            ) if after_generation == before_generation => {}
            (
                Some(Active {
                    generation: before_generation,
                })
                | Some(Revoked {
                    generation: before_generation,
                }),
                Some(Revoked {
                    generation: after_generation,
                }),
            ) if after_generation > before_generation => {
                credential_control(worker)?
                    .revoke(credential_id.clone(), after_generation.to_string())
                    .map_err(|_| DaemonError::required_broker("credential-revocation-failed"))?;
            }
            (
                Some(Revoked {
                    generation: before_generation,
                }),
                Some(Active {
                    generation: after_generation,
                }),
            ) if after_generation > before_generation => {
                let material = active_material(current, &credential_id).ok_or_else(|| {
                    DaemonError::required_broker("credential-active-material-missing")
                })?;
                credential_control(worker)?
                    .rotate(
                        credential_id.clone(),
                        after_generation.to_string(),
                        material.to_owned(),
                    )
                    .map_err(|_| DaemonError::required_broker("credential-rotation-failed"))?;
            }
            (Some(NotFound | Unavailable) | None, Some(NotFound | Unavailable) | None) => {}
            _ => {
                return Err(DaemonError::required_broker(
                    "credential-authority-requires-respawn",
                ));
            }
        }
    }
    Ok(())
}

fn credential_control(
    worker: &WorkerHandle,
) -> Result<provider::CredentialBrokerControl, DaemonError> {
    worker
        .credential_control
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone()
        .ok_or_else(|| DaemonError::required_broker("credential-channel-unavailable"))
}

fn active_material<'a>(
    bindings: &'a provider::ResolvedCredentialBindings,
    credential_id: &str,
) -> Option<&'a str> {
    let mut scopes = bindings
        .active
        .iter()
        .filter(|scope| scope.credential_id == credential_id);
    let material = scopes.next()?.material.as_str();
    scopes
        .all(|scope| scope.material == material)
        .then_some(material)
}

fn retain_prior_for_unknown_credentials(
    previous: &provider::ResolvedCredentialBindings,
    current: &mut provider::ResolvedCredentialBindings,
) {
    let unknown = current
        .availability
        .iter()
        .filter_map(|(credential_id, availability)| {
            matches!(availability, provider::CredentialAvailability::Unavailable)
                .then_some(credential_id.clone())
        })
        .collect::<BTreeSet<_>>();
    for credential_id in unknown {
        let Some(prior_availability) = previous.availability.get(&credential_id) else {
            continue;
        };
        current
            .availability
            .insert(credential_id.clone(), prior_availability.clone());
        current
            .active
            .retain(|scope| scope.credential_id != credential_id);
        current
            .revoked
            .retain(|scope| scope.credential_id != credential_id);
        current.active.extend(
            previous
                .active
                .iter()
                .filter(|scope| scope.credential_id == credential_id)
                .cloned(),
        );
        current.revoked.extend(
            previous
                .revoked
                .iter()
                .filter(|scope| scope.credential_id == credential_id)
                .cloned(),
        );
    }
}

impl ProductionProcessHost {
    pub fn open(
        root: impl AsRef<Path>,
        worker_binary: impl Into<PathBuf>,
        build: impl Into<String>,
        user_agent_dir: impl Into<PathBuf>,
    ) -> Result<Arc<Self>, DaemonError> {
        Self::open_with_secret_store(
            root,
            worker_binary,
            build,
            user_agent_dir,
            Arc::new(provider::MemorySecretStore::new()),
        )
    }

    pub fn open_with_secret_store(
        root: impl AsRef<Path>,
        worker_binary: impl Into<PathBuf>,
        build: impl Into<String>,
        user_agent_dir: impl Into<PathBuf>,
        secret_store: Arc<dyn provider::SecretStore>,
    ) -> Result<Arc<Self>, DaemonError> {
        Self::open_with_secret_authorities(
            root,
            worker_binary,
            build,
            user_agent_dir,
            secret_store,
            None,
        )
    }

    /// `secret_mutation` is the write path for OAuth grants the Kernel mints
    /// (secret-store §OAuth secret mutation); without it `oauth` MCP
    /// bindings carry platform-installed tokens only.
    pub fn open_with_secret_authorities(
        root: impl AsRef<Path>,
        worker_binary: impl Into<PathBuf>,
        build: impl Into<String>,
        user_agent_dir: impl Into<PathBuf>,
        secret_store: Arc<dyn provider::SecretStore>,
        secret_mutation: Option<Arc<dyn provider::SecretMutationAuthority>>,
    ) -> Result<Arc<Self>, DaemonError> {
        let root = root.as_ref().to_path_buf();
        let worker_binary = worker_binary.into();
        validate_worker_binary(&worker_binary)?;
        let repository = ConfigRepository::open(&root)
            .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
        let endpoint =
            NativeEndpoint::open(&root).map_err(|error| DaemonError::corrupt(error.to_string()))?;
        let schedule = schedule::ScheduleAuthority::open(&root)
            .map_err(|error| DaemonError::corrupt(error.to_string()))?;
        let build = build.into();
        let (mcp_runtime, plugin_store) = McpRuntime::open_production_authorities_with_mutation(
            root.join("config"),
            root.join("plugins"),
            &build,
            Arc::clone(&secret_store),
            secret_mutation,
        )
        .map_err(|error| DaemonError::required_broker(error.to_string()))?;
        let mcp_runtime = Arc::new(mcp_runtime);
        let root_for_catalog = root.clone();
        let threads_store = ThreadStore::open(&root_for_catalog).map_err(DaemonError::store)?;
        let host = Arc::new_cyclic(|weak| Self {
            card_writer: crate::thread_card::CardWriter::spawn_from_env(),
            repository,
            endpoint,
            root,
            worker_binary,
            build,
            user_agent_dir: user_agent_dir.into(),
            secret_store,
            mcp_runtime,
            plugin_store,
            schedule,
            workers: Mutex::new(HashMap::new()),
            released: Mutex::new(HashSet::new()),
            catalog: Arc::new(crate::catalog::Catalog::new(&root_for_catalog)),
            threads: threads_store,
            hydrating: Mutex::new(HashMap::new()),
            hydrate_failures: Mutex::new(HashMap::new()),
            stale_ephemeral_busy: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            hydrate_runs: AtomicU64::new(0),
            #[cfg(test)]
            card_builds: AtomicU64::new(0),
            #[cfg(test)]
            verify_order: Mutex::new(Vec::new()),
            #[cfg(test)]
            list_wait_override: Mutex::new(None),
            file_observations: crate::file_observation::SessionFileObservations::default(),
            pending_workers: Mutex::new(BTreeMap::new()),
            workers_changed: Condvar::new(),
            admission: Arc::new(Mutex::new(AdmissionPool::new(usize::MAX))),
            admission_waiters: Mutex::new(VecDeque::new()),
            admission_changed: Condvar::new(),
            next_run: AtomicU64::new(1),
            max_workers: AtomicU64::new(1),
            max_provider_leases: AtomicU64::new(1),
            self_weak: weak.clone(),
            streams: Mutex::new(None),
            sweep_scans: Mutex::new(HashMap::new()),
            projection_repair_failures: Mutex::new(HashSet::new()),
            context_projection_retries: Mutex::new(HashSet::new()),
            protocol_quarantine: Mutex::new(BTreeSet::new()),
            projection_lock: Mutex::new(()),
            projection_cache: Mutex::new(HashMap::new()),
            sweep_lock: Mutex::new(()),
            periodic_sweep_started: AtomicBool::new(false),
            schedule_timer_started: AtomicBool::new(false),
            schedule_failure: Mutex::new(None),
            restart_failures: Mutex::new(HashMap::new()),
            draining: AtomicBool::new(false),
            #[cfg(test)]
            automatic_title_test_redirect: Mutex::new(None),
        });
        // Folder membership changes (store::folder_events) invalidate what this host remembers
        // about a session: its catalog entry, projection cache, release and published actionables.
        let events_host = Arc::downgrade(&host);
        let events_root = store::folder_events::canonical_root(&host.root);
        store::folder_events::subscribe(Arc::new(move |event| {
            let Some(host) = events_host.upgrade() else {
                return false;
            };
            if event.root() == events_root {
                host.on_folder_event(event);
            }
            true
        }));
        let refresh_host = Arc::downgrade(&host);
        std::thread::spawn(move || {
            while let Some(host) = refresh_host.upgrade() {
                if host.draining.load(Ordering::Acquire) {
                    return;
                }
                std::thread::sleep(Duration::from_secs(1));
                if host.draining.load(Ordering::Acquire) {
                    return;
                }
                let _ = host.refresh_live_credentials();
            }
        });
        Ok(host)
    }

    #[must_use]
    pub fn mcp_runtime(&self) -> Arc<McpRuntime> {
        Arc::clone(&self.mcp_runtime)
    }

    /// Secret-free readiness used by provider administration. This checks the
    /// same durable secret authority used for worker credential scopes without
    /// returning material to the endpoint layer.
    pub fn credential_is_ready(&self, credential_id: Option<&str>) -> Result<bool, DaemonError> {
        let Some(credential_id) = credential_id else {
            return Ok(true);
        };
        Ok(matches!(
            self.secret_store
                .resolve(credential_id)
                .map_err(|error| DaemonError::required_broker(error.to_string()))?,
            provider::SecretResolution::Active(_)
        ))
    }

    /// Validates a replacement workspace-local policy against the current
    /// immutable instruction ceiling and every currently installed tool
    /// catalog. No worker is started and no config bytes are written.
    ///
    /// Returns the policy-named tools that are unavailable because their
    /// healthy MCP server refused to project them (per tool; see
    /// mcp-runtime). Such a name is neither an unknown tool nor an unavailable
    /// server: the policy is accepted, the tool is not callable and not in the
    /// catalog, and the caller reports it with its reason.
    pub fn validate_workspace_policy_candidate(
        &self,
        workspace_id: &str,
        policy: &profile::WorkspacePolicy,
    ) -> Result<Vec<crate::mcp_runtime::McpRefusedTool>, PolicyCandidateError> {
        use PolicyCandidateCause as Cause;
        let mut config = self
            .repository
            .resolve(workspace_id)
            .map_err(|error| PolicyCandidateError::new(Cause::CatalogUnavailable, error))?;
        config.workspace.policy = policy.clone();
        let instruction = profile::InstructionResolver::new_scoped(
            &self.user_agent_dir,
            self.root.join("workspaces").join(workspace_id),
            config.workspace.cwd.iter().map(PathBuf::from),
        )
        .capture()
        .map_err(|error| PolicyCandidateError::new(Cause::CatalogUnavailable, error))?;
        let effective = instruction.meet_workspace_policy(policy);
        if effective.network != policy.network
            || effective.allowed_tools.iter().collect::<BTreeSet<_>>()
                != policy.allowed_tools.iter().collect::<BTreeSet<_>>()
            || effective.writable_roots.iter().collect::<BTreeSet<_>>()
                != policy.writable_roots.iter().collect::<BTreeSet<_>>()
            || effective.max_wall_seconds != policy.max_wall_seconds
        {
            return Err(PolicyCandidateError::new(
                Cause::AuthorityCeiling,
                "workspace policy exceeds the immutable instruction ceiling",
            ));
        }

        let requested = policy
            .allowed_tools
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut available = tools::BuiltinManifest::compiled()
            .tools
            .into_iter()
            .map(|tool| tool.name)
            .collect::<BTreeSet<_>>();
        let mut discovery_config = config.clone();
        discovery_config.workspace.policy.allowed_tools.clear();
        let bindings = resolve_worker_launch_bindings(
            &discovery_config,
            &instruction,
            None,
            Vec::new(),
            Vec::new(),
        )
        .map_err(|error| PolicyCandidateError::new(Cause::AuthorityCeiling, error))?;
        available.extend(
            bindings
                .dynamic_catalog
                .tools
                .into_iter()
                .map(|tool| tool.name),
        );
        let mcp = self
            .mcp_runtime
            .prepare_workspace(workspace_id)
            .map_err(|error| PolicyCandidateError::new(Cause::CatalogUnavailable, error))?;
        if mcp_failure_is_required(&mcp.failures, &policy.allowed_tools) {
            return Err(PolicyCandidateError::new(
                Cause::McpServerUnavailable,
                required_mcp_failure_reason(&mcp.failures, &policy.allowed_tools),
            ));
        }
        available.extend(mcp.catalog.tools.iter().map(|tool| tool.name.clone()));
        // A refused tool is excused only by its exact projected name on a
        // server that otherwise prepared; every other absent name is still
        // outside the installed catalog.
        let unavailable = mcp.refused_allowed_tools(&policy.allowed_tools);
        let excused = unavailable
            .iter()
            .filter_map(|refused| refused.refusal.name.clone())
            .collect::<BTreeSet<_>>();
        let unknown = requested
            .iter()
            .filter(|name| !available.contains(*name) && !excused.contains(*name))
            .collect::<Vec<_>>();
        if !unknown.is_empty() {
            return Err(PolicyCandidateError::new(
                Cause::ToolNotInstalled,
                format!(
                    "workspace policy names a tool outside the installed catalog: {}",
                    bounded_name_list(unknown.iter().map(|name| name.as_str()))
                ),
            ));
        }
        Ok(unavailable)
    }

    /// Applies the privilege-reduction half of config publication. A relaxed
    /// policy affects only later runs; a tightened effective snapshot stops
    /// every live process in that workspace so no old authority survives the
    /// successful endpoint response.
    pub fn workspace_policy_published(
        &self,
        workspace_id: &str,
        previous: &ConfigSnapshot,
    ) -> Result<(), DaemonError> {
        let current = self
            .repository
            .resolve(workspace_id)
            .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
        if current.requires_respawn_from(previous) {
            let workers = self
                .workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .values()
                .filter(|worker| worker.config_snapshot.workspace.id == workspace_id)
                .cloned()
                .collect::<Vec<_>>();
            for worker in workers {
                terminate_worker(&worker);
            }
        }
        Ok(())
    }

    /// Conservative recovery hook for a policy intent whose durable replace
    /// is visible but whose post-publication receipt was interrupted. The old
    /// snapshot is no longer reconstructible, so all live workers in the
    /// workspace are stopped; later ensure-running uses the new snapshot.
    pub fn workspace_policy_recovered(&self, workspace_id: &str) {
        let workers = self
            .workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .values()
            .filter(|worker| worker.config_snapshot.workspace.id == workspace_id)
            .cloned()
            .collect::<Vec<_>>();
        for worker in workers {
            terminate_worker(&worker);
        }
    }

    #[must_use]
    pub(crate) fn plugin_store(
        &self,
    ) -> Option<Arc<Mutex<PluginStore<MacOsNativeHelperVerifier>>>> {
        self.plugin_store.clone()
    }

    pub(crate) fn client_file_changes(
        &self,
        session_id: &str,
    ) -> Result<crate::file_observation::FileSubscription, DaemonError> {
        endpoint::validate_session_id(session_id)
            .map_err(|_| DaemonError::invalid_config("invalid session identity"))?;
        let folder = ["threads", "archive"]
            .into_iter()
            .map(|area| self.root.join(area).join(session_id))
            .find(|path| path.is_dir())
            .ok_or_else(|| DaemonError::invalid_config("session not found"))?;
        let (workspace, binding) = session_workspace_binding(&folder.join("main.jsonl"))?;
        let repository = self.repository.clone();
        let resolve = move || {
            match &binding {
                Some(binding) => {
                    repository.resolve_for_session_binding(&workspace, &folder, binding)
                }
                None => repository.resolve_for_session(&workspace, &folder),
            }
            .map(|config| config.workspace)
            .map_err(|error| error.to_string())
        };
        let initial = resolve().map_err(DaemonError::invalid_config)?;
        let subscription = self
            .file_observations
            .open(session_id)
            .map_err(DaemonError::invalid_config)?;
        Ok(subscription.with_authority_check(move || {
            if resolve()? != initial {
                return Err("Session workspace authority changed".into());
            }
            Ok(())
        }))
    }

    /// Resolve file authority from the durable session binding, never a client
    /// supplied workspace root. Absolute paths must stay in an authored root.
    pub(crate) fn client_file_page(
        &self,
        session_id: &str,
        path: &str,
        text: bool,
        offset: u64,
        limit: usize,
    ) -> Result<serde_json::Value, DaemonError> {
        endpoint::validate_session_id(session_id)
            .map_err(|_| DaemonError::invalid_config("invalid session identity"))?;
        let folder = ["threads", "archive"]
            .into_iter()
            .map(|area| self.root.join(area).join(session_id))
            .find(|path| path.is_dir())
            .ok_or_else(|| DaemonError::invalid_config("session not found"))?;
        let (workspace, binding) = session_workspace_binding(&folder.join("main.jsonl"))?;
        let config = match binding {
            Some(binding) => self
                .repository
                .resolve_for_session_binding(&workspace, &folder, &binding),
            None => self.repository.resolve_for_session(&workspace, &folder),
        }
        .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
        let (root, relative) = scoped_client_file_path(&config.workspace, path)?;
        self.file_observations
            .register(session_id, &root, &relative)
            .map_err(DaemonError::invalid_config)?;
        let result = if text {
            let offset = usize::try_from(offset)
                .map_err(|_| DaemonError::invalid_config("invalid line offset"))?;
            workspace_service::file_text_page(&root, &relative, offset, limit)
        } else {
            workspace_service::file_byte_page(&root, &relative, offset, limit)
        };
        result.map_err(|error| {
            DaemonError::invalid_config(format!("{}: {}", error.code, error.message))
        })
    }

    /// The user-level `.agents` directory (instruction sources).
    pub fn user_agent_dir(&self) -> &Path {
        &self.user_agent_dir
    }

    /// Canonical workspace roots for a session, primary (selected) root first.
    /// Resolved from the durable session binding exactly like `client_file_page`.
    pub fn client_session_roots(&self, session_id: &str) -> Result<Vec<PathBuf>, DaemonError> {
        endpoint::validate_session_id(session_id)
            .map_err(|_| DaemonError::invalid_config("invalid session identity"))?;
        let folder = ["threads", "archive"]
            .into_iter()
            .map(|area| self.root.join(area).join(session_id))
            .find(|path| path.is_dir())
            .ok_or_else(|| DaemonError::invalid_config("session not found"))?;
        let (workspace, binding) = session_workspace_binding(&folder.join("main.jsonl"))?;
        let config = match binding {
            Some(binding) => self
                .repository
                .resolve_for_session_binding(&workspace, &folder, &binding),
            None => self.repository.resolve_for_session(&workspace, &folder),
        }
        .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
        let workspace = &config.workspace;
        let selected = workspace.selected_cwd.as_deref();
        selected
            .into_iter()
            .chain(
                workspace
                    .cwd
                    .iter()
                    .map(String::as_str)
                    .filter(|root| Some(*root) != selected),
            )
            .map(|root| Path::new(root).canonicalize().map_err(DaemonError::io))
            .collect()
    }

    /// Resolve resources using the session's durable workspace and folder binding.
    pub(crate) fn client_resource_catalog(
        &self,
        session_id: &str,
    ) -> Result<profile::ResourceCatalog, DaemonError> {
        if session_id.is_empty()
            || session_id.contains(['/', '\\'])
            || session_id == "."
            || session_id == ".."
        {
            return Err(DaemonError::invalid_config("invalid session identity"));
        }
        let folder = ["threads", "archive"]
            .into_iter()
            .map(|area| self.root.join(area).join(session_id))
            .find(|path| path.is_dir())
            .ok_or_else(|| DaemonError::invalid_config("session not found"))?;
        let (workspace, binding) = session_workspace_binding(&folder.join("main.jsonl"))?;
        let config = match binding {
            Some(binding) => self
                .repository
                .resolve_for_session_binding(&workspace, &folder, &binding),
            None => self.repository.resolve_for_session(&workspace, &folder),
        }
        .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
        let snapshot = profile::InstructionResolver::new_scoped(
            &self.user_agent_dir,
            self.root.join("workspaces").join(&workspace),
            &config.workspace.cwd,
        )
        .capture()
        .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
        profile::ResourceCatalog::from_snapshot(&snapshot)
            .map_err(|error| DaemonError::invalid_config(error.to_string()))
    }

    /// Resolves the same immutable dynamic catalog used by a newly launched
    /// worker. Client catalog reads therefore cannot advertise a row that the
    /// launch path would omit, nor retain a revoked MCP generation.
    pub(crate) fn client_tool_catalog(
        &self,
        workspace_id: &str,
        session_id: Option<&str>,
    ) -> Result<DynamicToolCatalog, DaemonError> {
        let config = session_id
            .map_or_else(
                || self.repository.resolve(workspace_id),
                |session_id| {
                    let folder = ["threads", "archive"]
                        .into_iter()
                        .map(|area| self.root.join(area).join(session_id))
                        .find(|path| path.is_dir())
                        .ok_or_else(|| profile::ProfileError::InvalidPath {
                            path: self.root.join("threads").join(session_id),
                            reason: "session folder does not exist".to_owned(),
                        })?;
                    let (_, binding) = session_workspace_binding(&folder.join("main.jsonl"))
                        .map_err(|error| profile::ProfileError::InvalidPath {
                            path: folder.join("main.jsonl"),
                            reason: error.to_string(),
                        })?;
                    match binding {
                        Some(binding) => self.repository.resolve_for_session_binding(
                            workspace_id,
                            folder,
                            &binding,
                        ),
                        None => self.repository.resolve_for_session(workspace_id, folder),
                    }
                },
            )
            .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
        let instruction = profile::InstructionResolver::new_scoped(
            &self.user_agent_dir,
            self.root.join("workspaces").join(workspace_id),
            config.workspace.cwd.iter().map(PathBuf::from),
        )
        .capture()
        .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
        let mcp = self
            .mcp_runtime
            .prepare_workspace(workspace_id)
            .map_err(|error| DaemonError::required_broker(error.to_string()))?;
        if mcp_failure_is_required(&mcp.failures, &config.workspace.policy.allowed_tools) {
            return Err(DaemonError::required_broker(
                "workspace policy requires an unavailable MCP server",
            ));
        }
        let unavailable = mcp.refused_allowed_tool_names(&config.workspace.policy.allowed_tools);
        let binding = resolve_worker_launch_bindings(
            &config,
            &instruction,
            None,
            mcp.catalog.tools,
            unavailable,
        )
        .map_err(|error| DaemonError::required_broker(error.to_string()))?;
        Ok(binding.dynamic_catalog)
    }

    /// Internal Slice-14C authority. Public schedule DTOs and capability
    /// negotiation remain owned by Slice 14F.
    #[must_use]
    pub fn schedule_authority(&self) -> &schedule::ScheduleAuthority {
        &self.schedule
    }

    /// Starts the sole host-local schedule timer after boot recovery. The
    /// method is idempotent; Slice 14F calls it after saving the first enabled
    /// definition if startup found no work.
    pub fn start_schedule_timer(&self) -> Result<(), DaemonError> {
        if self.schedule_timer_started.swap(true, Ordering::AcqRel) {
            return Ok(());
        }
        *self
            .schedule_failure
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
        if let Err(error) = self.drive_schedule(true, Utc::now()) {
            self.schedule_timer_started.store(false, Ordering::Release);
            return Err(error);
        }
        if !self.schedule_has_work()? {
            self.schedule_timer_started.store(false, Ordering::Release);
            return Ok(());
        }
        let host = self.self_weak.clone();
        std::thread::spawn(move || {
            while let Some(host) = host.upgrade() {
                if host.draining.load(Ordering::Acquire) {
                    host.schedule_timer_started.store(false, Ordering::Release);
                    return;
                }
                std::thread::sleep(Duration::from_secs(1));
                if host.draining.load(Ordering::Acquire) {
                    host.schedule_timer_started.store(false, Ordering::Release);
                    return;
                }
                match host.drive_schedule(false, Utc::now()) {
                    Ok(()) => {
                        *host
                            .schedule_failure
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
                    }
                    Err(error) => {
                        // A durable unbound claim remains runnable work. Keep
                        // the sole bounded timer alive so a transient launch
                        // or projection failure cannot strand it forever;
                        // readiness still exposes the latest failure.
                        *host
                            .schedule_failure
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner) =
                            Some(error.to_string());
                        continue;
                    }
                }
                match host.schedule_has_work() {
                    Ok(true) => {}
                    Ok(false) => {
                        host.schedule_timer_started.store(false, Ordering::Release);
                        // Close the save-vs-idle-exit race. A save that landed
                        // before the false store is now visible and this
                        // thread retakes ownership; a save after it observes
                        // false and starts its own timer.
                        match host.schedule_has_work() {
                            Ok(true)
                                if host
                                    .schedule_timer_started
                                    .compare_exchange(
                                        false,
                                        true,
                                        Ordering::AcqRel,
                                        Ordering::Acquire,
                                    )
                                    .is_ok() =>
                            {
                                continue;
                            }
                            Ok(_) => {}
                            Err(error) => {
                                *host
                                    .schedule_failure
                                    .lock()
                                    .unwrap_or_else(std::sync::PoisonError::into_inner) =
                                    Some(error.to_string());
                            }
                        }
                        return;
                    }
                    Err(error) => {
                        *host
                            .schedule_failure
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner) =
                            Some(error.to_string());
                        host.schedule_timer_started.store(false, Ordering::Release);
                        return;
                    }
                }
            }
        });
        Ok(())
    }

    #[must_use]
    pub fn schedule_failure(&self) -> Option<String> {
        self.schedule_failure
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }

    fn schedule_has_work(&self) -> Result<bool, DaemonError> {
        self.schedule
            .list(None)
            .map(|tasks| {
                tasks
                    .into_iter()
                    .any(|task| task.definition.enabled || task.last_status.is_active())
            })
            .map_err(|error| DaemonError::corrupt(error.to_string()))
    }

    fn drive_schedule(&self, startup: bool, now: DateTime<Utc>) -> Result<(), DaemonError> {
        self.reconcile_schedule_statuses(now)?;
        let claims = if startup {
            self.schedule.recover(now)
        } else {
            self.schedule.poll_due(now)
        }
        .map_err(|error| DaemonError::corrupt(error.to_string()))?;
        for claim in claims {
            self.execute_schedule_claim(&claim, now)?;
        }
        Ok(())
    }

    fn execute_schedule_claim(
        &self,
        claim: &schedule::LaunchClaim,
        now: DateTime<Utc>,
    ) -> Result<(), DaemonError> {
        // A claim may be driven once immediately and again by startup/timer
        // recovery. Its session-create intent must remain byte-identical
        // across those runs; wall-clock recovery time is not durable identity.
        let timestamp = DateTime::parse_from_rfc3339(&claim.scheduled_for)
            .map_err(|error| DaemonError::corrupt(error.to_string()))?
            .with_timezone(&Utc)
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let request_bytes = serde_json_canonicalizer::to_vec(claim)
            .map_err(|error| DaemonError::protocol(error.to_string()))?;
        let request_sha256 = Sha256::digest(&request_bytes).iter().fold(
            String::with_capacity(64),
            |mut output, byte| {
                write!(output, "{byte:02x}").expect("writing to String cannot fail");
                output
            },
        );
        let management = ManagementStore::open_light_at(&self.root, &timestamp)
            .map_err(|error| DaemonError::corrupt(error.to_string()))?;
        let session_id = management
            .create_session(SessionCreateOperation {
                rpc_id: &claim.claim_id,
                request_sha256: &request_sha256,
                requested_session_id: None,
                workspace_id: Some(&claim.definition.workspace_id),
                cwd: None,
                identity_profile: None,
                preset: None,
                preset_mcp: None,
                user_agent_dir: &self.user_agent_dir,
                started_at: &timestamp,
                principal: "schedule",
            })
            .map_err(|error| DaemonError::corrupt(error.to_string()))?;
        let origin = OriginTuple {
            principal: "schedule".to_owned(),
            client: "schedule".to_owned(),
            target: session_id.clone(),
            op: "session.prompt".to_owned(),
            key: claim.claim_id.clone(),
        };
        let receipt = SessionDeliveryAuthority::prompt(
            self,
            &session_id,
            &timestamp,
            &origin,
            &MaterializedPrompt {
                blocks: vec![Block::Text {
                    text: claim.definition.prompt.clone(),
                }],
                attachments: Vec::new(),
                files: Vec::new(),
            },
            false,
        )
        .map_err(|error| DaemonError::protocol(format!("{}: {}", error.code, error.message)))?;
        self.schedule
            .bind_launch(
                &claim.task_id,
                &claim.claim_id,
                &session_id,
                receipt.seq,
                now,
            )
            .map_err(|error| DaemonError::corrupt(error.to_string()))?;
        Ok(())
    }

    fn reconcile_schedule_statuses(&self, now: DateTime<Utc>) -> Result<(), DaemonError> {
        let tasks = self
            .schedule
            .list(None)
            .map_err(|error| DaemonError::corrupt(error.to_string()))?;
        for task in tasks.into_iter().filter(|task| {
            matches!(
                task.last_status,
                schedule::RunStatus::Running | schedule::RunStatus::Parked
            )
        }) {
            let (Some(claim_id), Some(session_id)) = (
                task.active_claim_id.as_deref(),
                task.last_session_id.as_deref(),
            ) else {
                return Err(DaemonError::corrupt(
                    "active bound schedule is missing claim/session identity",
                ));
            };
            let active = self.root.join("threads").join(session_id);
            let archived = self.root.join("archive").join(session_id);
            let folder = if active.is_dir() { active } else { archived };
            let ledger = folder.join("main.jsonl");
            let bytes = fs::read(&ledger).map_err(DaemonError::io)?;
            let scan = scan_valid_prefix(&bytes, schema::READER_VERSION);
            let projection = scan
                .projection
                .ok_or_else(|| DaemonError::corrupt("scheduled session has no valid projection"))?;
            let lock_facts = if self.live_worker(session_id).is_some()
                || probe_line_lock(&ledger)? == LineLockState::Busy
            {
                LockFacts::OTHER
            } else {
                LockFacts::FREE
            };
            let tail = classify(&projection.lifecycle, lock_facts);
            let (status, error) = match tail {
                TailState::ParkedHold => (schedule::RunStatus::Parked, None),
                TailState::Settled => {
                    let settle = projection
                        .events
                        .iter()
                        .rev()
                        .find(|event| matches!(event.kind(), EventKind::Settle))
                        .ok_or_else(|| {
                            DaemonError::corrupt("scheduled session settled without settle")
                        })?;
                    match settle.string_field("outcome") {
                        Some("completed") => (schedule::RunStatus::Completed, None),
                        Some("interrupted") => (schedule::RunStatus::Interrupted, None),
                        Some("error") => (
                            schedule::RunStatus::Failed,
                            settle.string_field("reason").map(str::to_owned),
                        ),
                        _ => {
                            return Err(DaemonError::corrupt(
                                "scheduled session has unknown settle outcome",
                            ));
                        }
                    }
                }
                _ => (schedule::RunStatus::Running, None),
            };
            if task.last_status != status || task.last_error != error {
                self.schedule
                    .record_status(&task.definition.id, claim_id, status, error, now)
                    .map_err(|error| DaemonError::corrupt(error.to_string()))?;
            }
        }
        Ok(())
    }

    /// Starts the work that touches threads, after the ready line (ADDENDUM-01 §1.4). One
    /// background thread, in order: load the catalog, clean stale ephemeral threads and
    /// tombstones, start the schedule timer (its startup pass reads the sessions it launched),
    /// then build the missing cards. A request that needs the List or one thread does not wait for
    /// this thread: it loads/verifies what it needs itself (single flight).
    pub fn run_post_ready(self: &Arc<Self>) {
        let host = Arc::downgrade(self);
        let spawned = std::thread::Builder::new()
            .name("post-ready".to_owned())
            .spawn(move || {
                if let Some(host) = host.upgrade() {
                    host.post_ready_tasks();
                }
            });
        if let Err(error) = spawned {
            eprintln!("process-host-post-ready-failed: {error}");
        }
    }

    fn post_ready_tasks(self: &Arc<Self>) {
        self.catalog.ensure_loaded();
        crate::boot_trace::phase("catalog_loaded");
        if self.draining.load(Ordering::Acquire) {
            return;
        }
        self.cleanup_stale_ephemeral();
        if self.draining.load(Ordering::Acquire) {
            return;
        }
        if let Err(error) = self.start_schedule_timer() {
            eprintln!("process-host-schedule-start-failed: {error}");
        }
        crate::boot_trace::phase("post_ready_timers");
        self.run_upgrade_pass();
    }

    /// Removes ephemeral scratch threads and `discard-*` tombstones left by a previous process
    /// (ADDENDUM-02 S4, -03 C7). Threads this process owns (admitted, or with a live worker) are
    /// never touched; the store re-checks under the catalog lock. Until removed, such a thread is
    /// hidden from the List (`is_stale_ephemeral`).
    fn cleanup_stale_ephemeral(&self) {
        let Ok(threads) = store::ThreadStore::open(&self.root) else {
            return;
        };
        let live = self.live_sessions_for_cleanup();
        let report = threads.cleanup_stale_ephemeral(&|id| {
            store::folder_events::is_admitted(&self.root, id) || live.contains(id)
        });
        match report {
            Ok(report) => {
                self.stale_ephemeral_busy
                    .store(report.busy.len(), Ordering::Release);
                if !report.removed.is_empty() || !report.busy.is_empty() || report.tombstones > 0 {
                    eprintln!(
                        "process-host-ephemeral-cleanup: removed={} busy={} tombstones={}",
                        report.removed.len(),
                        report.busy.len(),
                        report.tombstones
                    );
                }
            }
            Err(error) => eprintln!("process-host-ephemeral-cleanup-failed: {error}"),
        }
    }

    fn live_sessions_for_cleanup(&self) -> HashSet<String> {
        self.workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .iter()
            .filter(|(_, worker)| worker.alive.load(Ordering::Acquire))
            .map(|(key, _)| key.split(':').next().unwrap_or(key).to_owned())
            .collect()
    }

    /// The lazy upgrade of a root without (fresh) cards (DESIGN §9): fold each unverified thread
    /// once, newest first, at a low duty cycle. It never delays readiness or a List request,
    /// which verify what they need themselves.
    fn run_upgrade_pass(&self) {
        let work = self.catalog.unverified();
        if work.is_empty() {
            return;
        }
        let started = Instant::now();
        let total = work.len();
        let mut built = 0_usize;
        let mut last_report = Instant::now();
        for session in work {
            if self.draining.load(Ordering::Acquire) {
                return;
            }
            if self.recently_failed(&session) {
                continue;
            }
            let began = Instant::now();
            match self.hydrate(&session) {
                Ok(()) => built += 1,
                Err(error) => {
                    eprintln!("process-host-upgrade-skipped: session={session} error={error}");
                }
            }
            if last_report.elapsed() >= Duration::from_millis(250) {
                eprintln!(
                    "catalog-upgrade: built={built} total={total} elapsed_ms={}",
                    started.elapsed().as_millis()
                );
                last_report = Instant::now();
            }
            // Duty cycle: rest as long as the fold took (bounded), so a workload that started
            // meanwhile is not starved.
            std::thread::sleep(began.elapsed().min(Duration::from_millis(200)));
        }
        eprintln!(
            "catalog-upgrade: done built={built} total={total} elapsed_ms={}",
            started.elapsed().as_millis()
        );
    }

    /// Test helper: what `sessions.recover` does to the deferral set, without the verification.
    #[cfg(test)]
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    pub(crate) fn release_for_test(&self, session_id: &str) {
        self.released
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(session_id.to_owned());
    }

    /// A previous process's ephemeral scratch thread that cleanup has not removed yet.
    pub(crate) fn is_stale_ephemeral(&self, session_id: &str, ephemeral: bool) -> bool {
        ephemeral && !store::folder_events::is_admitted(&self.root, session_id)
    }

    fn on_folder_event(&self, event: &store::folder_events::FolderEvent) {
        use store::folder_events::FolderEvent;
        let id = event.id();
        self.catalog.forget(id);
        // A folder that arrives in `threads/` (create, fork, redact destination, unarchive)
        // enters the catalog now: the List and the sweep must see it without anybody having to
        // ask for it first (E832 M1). Nothing is read here -- this runs under the store's
        // locks (E833 F6): the first `get` reads its card, and a thread without one is Cold and
        // verified by the next List or open.
        if matches!(
            event,
            FolderEvent::Published { .. }
                | FolderEvent::Moved {
                    to: store::folder_events::Area::Threads,
                    ..
                }
        ) {
            self.catalog.note_present(id);
        }
        self.projection_cache
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(id);
        if let Some(streams) = self
            .streams
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
        {
            streams.forget_session(id);
        }
        if matches!(event, FolderEvent::Removed { .. }) {
            self.released
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .remove(id);
        }
    }

    pub fn attach_streams(&self, streams: ProductionCarrierStreams) {
        streams.attach_session_authority(self.self_weak.clone());
        *self
            .streams
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(streams);
    }

    /// Derives the requests of one child line and announces what changed (a child's append, or
    /// its exit). Failure only logs: the next recompute of the baseline reads the line again.
    fn reconcile_child_line(&self, session_id: &str, ledger: &Path) {
        let streams = self
            .streams
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        let (Some(streams), Some(line)) =
            (streams, ledger.file_name().and_then(|name| name.to_str()))
        else {
            return;
        };
        if let Err(error) = streams.reconcile_actionables(session_id, Some(line)) {
            eprintln!("process-host-publish-child-actionables-failed: {error}");
        }
    }

    /// Announces a worker start or exit so inventory `running` follows the
    /// process table instead of freezing at the stream baseline (audit I5).
    /// Failure is logged only: the next inventory baseline recomputes the flag.
    fn publish_session_status(&self, session_id: &str) {
        let streams = self
            .streams
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        let Some(streams) = streams else {
            return;
        };
        let running = self.live_sessions().contains(session_id);
        if let Err(error) = streams.publish_session_status(session_id, running) {
            eprintln!("process-host-session-status-failed: session={session_id} error={error}");
        }
    }

    /// Starts the periodic recovery safety net after boot recovery and the
    /// endpoint stream assembly are complete. Repeated starts are idempotent.
    pub fn start_periodic_sweep(self: &Arc<Self>) {
        if self
            .periodic_sweep_started
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return;
        }
        let sweep_host = Arc::downgrade(self);
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(PERIODIC_SWEEP_INTERVAL);
                let Some(host) = sweep_host.upgrade() else {
                    return;
                };
                if host.draining.load(Ordering::Acquire) {
                    return;
                }
                if let Err(error) = host.periodic_sweep_once() {
                    eprintln!("process-host-periodic-sweep-failed: {error}");
                }
            }
        });
        // The management recovery retry has its own thread: a pending queue transaction can
        // start a worker (a handshake of up to 10 s) and must never delay the sweep.
        let recovery_host = Arc::downgrade(self);
        std::thread::spawn(move || {
            let mut validated = endpoint::CompleteOperations::default();
            loop {
                for _ in 0..MANAGEMENT_RECOVERY_SECONDS {
                    std::thread::sleep(Duration::from_secs(1));
                    match recovery_host.upgrade() {
                        Some(host) if !host.draining.load(Ordering::Acquire) => {}
                        _ => return,
                    }
                }
                let Some(host) = recovery_host.upgrade() else {
                    return;
                };
                host.recover_pending_management(&mut validated);
                // A stale scratch thread that was busy at post-ready cleanup (a lock holder, a
                // live rewrite) is tried again here (O-final m4).
                if host.stale_ephemeral_busy.load(Ordering::Acquire) > 0 {
                    host.cleanup_stale_ephemeral();
                }
            }
        });
    }

    /// The 30 s retry of pending management operations (create/fork/archive/queue/workspace
    /// records a failure left unfinished). Failure only logs: the next tick retries.
    fn recover_pending_management(&self, validated: &mut endpoint::CompleteOperations) {
        if let Err(error) =
            crate::endpoint_host::recover_pending_management(&self.root, self, validated)
        {
            eprintln!("process-host-management-recovery-failed: {error}");
        }
    }

    pub fn refresh_live_credentials(&self) -> Result<(), DaemonError> {
        let workers = self
            .workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .values()
            .filter(|worker| worker.alive.load(Ordering::Acquire))
            .cloned()
            .collect::<Vec<_>>();
        for worker in workers {
            self.refresh_worker_credentials(&worker)?;
        }
        Ok(())
    }

    /// Synchronous hook for any successful config-authority mutation. Callers
    /// invoke this before reporting mutation completion so credential changes
    /// cannot wait for the periodic safety-net refresh.
    pub fn config_mutation_succeeded(&self) -> Result<(), DaemonError> {
        self.refresh_live_credentials()?;
        if let Some(streams) = self
            .streams
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
        {
            streams
                .refresh_all_context_projections()
                .map_err(DaemonError::from)?;
        }
        Ok(())
    }

    fn refresh_worker_credentials(&self, worker: &Arc<WorkerHandle>) -> Result<(), DaemonError> {
        let mut current = provider::resolve_config_credentials(
            &worker.config_snapshot,
            self.secret_store.as_ref(),
        )
        .map_err(|_| DaemonError::required_broker("credential-scope-invalid"))?;
        let mut prior = worker
            .credential_bindings
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(previous) = prior.as_ref() else {
            if !current.availability.is_empty() {
                terminate_worker(worker);
                return Err(DaemonError::required_broker(
                    "credential-authority-requires-respawn",
                ));
            }
            *prior = Some(current);
            return Ok(());
        };
        retain_prior_for_unknown_credentials(previous, &mut current);
        if let Err(error) = reconcile_credential_bindings(worker, previous, &current) {
            terminate_worker(worker);
            return Err(error);
        }
        if web_search_scope_ready(&worker.config_snapshot, previous)
            != web_search_scope_ready(&worker.config_snapshot, &current)
        {
            *prior = Some(current);
            terminate_worker(worker);
            return Ok(());
        }
        *prior = Some(current);
        Ok(())
    }

    /// One synchronous sweep tick (single-flight with the periodic thread). The Kernel no longer
    /// calls this at boot -- readiness does not wait for any thread -- but tests and tools use
    /// it to run the sweep deterministically.
    pub fn boot_sweep(&self) -> Result<(), DaemonError> {
        let _sweep = self
            .sweep_lock
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        self.sweep_once()
    }

    /// Starts recording which sessions this process creates, forks or unarchives. Called once,
    /// immediately before readiness: folders published earlier (recovery of pending
    /// operations) are pre-existing sessions and stay deferred.
    pub fn arm_admission(&self) {
        store::folder_events::arm_admission(&self.root);
    }

    /// The client releases pre-existing sessions after it installed its control and actionable
    /// baselines (spec/session-endpoint.md, `sessions.recover`). For each id: bring the thread's
    /// view to a verified state (repair a torn tail, project the journal), un-defer it, and give
    /// it its share of a sweep -- which is where an interrupted turn gets its reconcile-only
    /// worker. Other sessions are not touched.
    pub fn recover_client_sessions(&self, sessions: &[String]) -> Result<(), DaemonError> {
        for session in sessions {
            if session.is_empty()
                || session.contains(['/', '\\'])
                || session == "."
                || session == ".."
                || !self
                    .root
                    .join("threads")
                    .join(session)
                    .join("main.jsonl")
                    .is_file()
            {
                return Err(DaemonError::invalid_config("recovery session not found"));
            }
        }
        for session in sessions {
            if let Err(error) = self.ensure_verified(session) {
                // A quarantined or unreadable thread is not recovered; the others still are.
                eprintln!("process-host-recover-skipped: session={session} error={error}");
                continue;
            }
            self.released
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .insert(session.clone());
            let _sweep = self
                .sweep_lock
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Err(error) = self.sweep_session(session) {
                eprintln!("process-host-sweep-session-failed: session={session} error={error}");
            }
        }
        Ok(())
    }

    /// Brings one thread to a verified state: `Fresh` in the catalog. A view that a fold has
    /// produced is trusted; otherwise the thread's tails are repaired when no one holds them
    /// and the thread is folded and projected once (`publish_appended`, which also refreshes
    /// the catalog and hands the card to its writer). Returns an error for a quarantined
    /// thread. Cost: one decode of this thread, never of any other.
    pub(crate) fn ensure_verified(&self, session_id: &str) -> Result<(), DaemonError> {
        let Some(entry) = self.catalog.get(session_id) else {
            return Err(DaemonError::invalid_config("session not found"));
        };
        match entry.trust {
            crate::catalog::Trust::Fresh => return Ok(()),
            crate::catalog::Trust::Corrupt(reason) => {
                return Err(DaemonError::corrupt(format!("quarantined: {reason}")));
            }
            crate::catalog::Trust::Ahead | crate::catalog::Trust::Cold => {}
        }
        self.hydrate(session_id)
    }

    /// The hydrator's unit of work (single flight per thread).
    pub(crate) fn hydrate(&self, session_id: &str) -> Result<(), DaemonError> {
        self.hydrate_with(session_id, None)
    }

    /// `after_fold` is a test seam called after the thread's ledger is folded and before the
    /// journal work, i.e. outside every global lock.
    pub(crate) fn hydrate_with(
        &self,
        session_id: &str,
        after_fold: Option<&dyn Fn()>,
    ) -> Result<(), DaemonError> {
        let outcome = self.hydrate_unrecorded(session_id, after_fold);
        let mut failures = self
            .hydrate_failures
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if outcome.is_ok() {
            failures.remove(session_id);
        } else {
            failures.insert(session_id.to_owned(), Instant::now());
        }
        outcome
    }

    /// A thread whose verification failed a moment ago is not retried by every List: it keeps
    /// its placeholder row and is tried again after `HYDRATE_RETRY`.
    fn recently_failed(&self, session_id: &str) -> bool {
        self.hydrate_failures
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(session_id)
            .is_some_and(|at| at.elapsed() < HYDRATE_RETRY)
    }

    fn hydrate_unrecorded(
        &self,
        session_id: &str,
        after_fold: Option<&dyn Fn()>,
    ) -> Result<(), DaemonError> {
        let gate = {
            let mut gates = self
                .hydrating
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            Arc::clone(gates.entry(session_id.to_owned()).or_default())
        };
        let _single_flight = gate
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        // Another caller may have finished while we waited.
        if let Some(entry) = self.catalog.get(session_id) {
            match entry.trust {
                crate::catalog::Trust::Fresh => return Ok(()),
                crate::catalog::Trust::Corrupt(reason) => {
                    return Err(DaemonError::corrupt(format!("quarantined: {reason}")));
                }
                _ => {}
            }
        }
        let folder = self.root.join("threads").join(session_id);
        let report = store::repair_thread_tails(&folder).map_err(DaemonError::store)?;
        // Damage is decided on a read taken under the line's lock (`damage_under_lock`): the
        // unlocked scan above can straddle another process's repair and an append and show a
        // record that never existed (O-final n3). A line a live holder owns defers the decision.
        let damaged = report
            .corrupt
            .iter()
            .find(|(file, _)| match damage_under_lock(&folder.join(file)) {
                DamageVerdict::Damaged(_) => true,
                DamageVerdict::Sound => false,
                DamageVerdict::Deferred => {
                    eprintln!(
                        "process-host-damage-deferred: session={session_id} {file}: line lock is held"
                    );
                    false
                }
            });
        if let Some((file, reason)) = damaged {
            let reason = format!("{file}: {reason}");
            eprintln!("process-host-hydrate-quarantined: session={session_id} {reason}");
            self.catalog.mark_corrupt(session_id, &reason);
            return Err(DaemonError::corrupt(format!("quarantined: {reason}")));
        }
        if report.truncated_bytes > 0 {
            eprintln!(
                "process-host-hydrate-repair: session={session_id} discarded_partial_bytes={}",
                report.truncated_bytes
            );
            // The repair fsynced the ledger: it counts against the same limiter the card writer
            // uses, and never waits (hydration may be serving a List or a thread open).
            self.card_writer.governor().charge(1);
        }
        // A crashed process can leave its temporary card file behind (never read, never named
        // like a thread file); a dead pid's file is removed here, once per hydration.
        let stale = store::card::remove_stale_card_temps(&folder);
        if stale > 0 {
            eprintln!("process-host-hydrate-card-temps: session={session_id} removed={stale}");
        }
        self.card_writer.reset_window(session_id);
        #[cfg(test)]
        self.hydrate_runs.fetch_add(1, Ordering::Relaxed);
        self.publish_appended_with(session_id, after_fold)?;
        // The main ledger is sound; every child ledger must be too (E833 F5).
        if let Some(reason) = child_ledger_damage(&folder) {
            eprintln!("process-host-quarantined: session={session_id} {reason}");
            self.catalog.mark_corrupt(session_id, &reason);
            return Err(DaemonError::corrupt(format!("quarantined: {reason}")));
        }
        // The entry's child lines are exact as of this verification (O-final B1): the main
        // fold above carried none forward.
        self.catalog.set_children(
            session_id,
            crate::thread_card::fold_child_views(session_id, &folder),
        );
        Ok(())
    }

    /// Audit invariant I3: the endpoint journal catches up with the semantic
    /// ledger within one sweep tick. The worker doorbell, the exit path, and
    /// boot are the fast paths; this is the deterministic retry when one of
    /// them failed or the ledger grew without a live worker (supervisor locked
    /// appends). It never races a live main-line worker: that worker's
    /// doorbell orders projection against the frames still in its pipe.
    fn repair_main_projection(&self, session_id: &str, ledger_last_seq: u64) {
        let cached_through = self
            .projection_cache
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(session_id)
            .map(|cache| cache.projector.processed_through().unwrap_or(0));
        if cached_through.is_some_and(|through| through >= ledger_last_seq)
            && !self
                .context_projection_retries
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .contains(session_id)
        {
            return;
        }
        match self.publish_appended(session_id) {
            Ok(()) => {
                self.projection_repair_failures
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .remove(session_id);
            }
            Err(error) => {
                // The retry itself stays per-tick; only the report is de-duplicated
                // until the session projects again.
                let first_failure = self
                    .projection_repair_failures
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .insert(session_id.to_owned());
                if first_failure {
                    eprintln!(
                        "process-host-sweep-projection-failed: session={session_id} error={error}"
                    );
                }
            }
        }
    }

    /// Runs one nonblocking periodic reconciliation pass.
    ///
    /// Boot and periodic callers share the same single-flight lock. A tick
    /// that overlaps another pass is coalesced rather than queued, and drain
    /// prevents a new pass from launching work.
    pub fn periodic_sweep_once(&self) -> Result<bool, DaemonError> {
        if self.draining.load(Ordering::Acquire) {
            return Ok(false);
        }
        let _sweep = match self.sweep_lock.try_lock() {
            Ok(sweep) => sweep,
            Err(TryLockError::WouldBlock) => return Ok(false),
            Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
        };
        if self.draining.load(Ordering::Acquire) {
            return Ok(false);
        }
        self.sweep_once()?;
        Ok(true)
    }

    #[must_use]
    pub fn is_draining(&self) -> bool {
        self.draining.load(Ordering::Acquire)
    }

    /// The sweep facts of one ledger, keyed by the file identity the scan was taken from.
    /// The periodic sweep runs every second over every active line; decoding and re-validating
    /// each ledger twice per tick (durable-stop propagation, then classification) cost a Debug
    /// supervisor 15–20 % of a core while nothing ran (2026-09-20). A ledger is append-only and
    /// its repairs truncate, so an unchanged (inode, length, mtime) is the same bytes and the
    /// same facts; the scan is retaken the moment any of the three moves.
    fn sweep_ledger_scan(&self, path: &Path) -> Result<Arc<SweepLedgerScan>, DaemonError> {
        let identity = SweepLedgerIdentity::of(path)?;
        if let Some(cached) = self
            .sweep_scans
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(path)
            .filter(|scan| scan.identity == identity)
        {
            return Ok(Arc::clone(cached));
        }
        let bytes = fs::read(path).map_err(DaemonError::io)?;
        // The identity is taken before the bytes: a write landing between the two changes the
        // identity, so the next tick rescans rather than trusting facts from a shorter file.
        let scan = scan_valid_prefix(&bytes, schema::READER_VERSION);
        let Some(projection) = scan.projection else {
            self.sweep_scans
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .remove(path);
            return Err(DaemonError::corrupt("active thread has no projection"));
        };
        let genesis = projection
            .events
            .first()
            .ok_or_else(|| DaemonError::corrupt("line has no genesis"))?;
        let line = genesis
            .string_field("thread")
            .ok_or_else(|| DaemonError::corrupt("line genesis has no thread"))?
            .to_owned();
        let parent_file = serde_json::to_value(genesis.raw())
            .map_err(|error| DaemonError::protocol(error.to_string()))?
            .get("parent")
            .and_then(|parent| parent.get("file"))
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned);
        let scan = Arc::new(SweepLedgerScan {
            identity,
            last_seq: projection.last_seq,
            lifecycle: projection.lifecycle,
            line,
            parent_file,
        });
        self.sweep_scans
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(path.to_path_buf(), Arc::clone(&scan));
        Ok(scan)
    }

    fn sweep_once(&self) -> Result<(), DaemonError> {
        for session_id in self.sweep_ids() {
            let started = Instant::now();
            if let Err(error) = self.sweep_session(&session_id) {
                eprintln!("process-host-sweep-session-failed: session={session_id} error={error}");
            }
            // Opt-in (`TEKES_KERNEL_BOOT_TRACE=1`): the sweep runs every second, so only a
            // session that took noticeable time is reported.
            if crate::boot_trace::per_item_enabled() && started.elapsed().as_millis() >= 100 {
                crate::boot_trace::detail(
                    "sweep_session",
                    &format!("session={session_id} ms={}", started.elapsed().as_millis()),
                );
            }
        }
        Ok(())
    }

    /// The sessions a tick considers. Before the catalog has loaded (the first moments after
    /// `ready`) that is only what this process created or released -- no directory is read.
    fn sweep_ids(&self) -> Vec<String> {
        let mut ids = if self.catalog.is_loaded() {
            self.catalog
                .entries_now()
                .into_iter()
                .map(|(id, _)| id)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        // Threads this process created or unarchived are always swept, whatever the catalog
        // has seen so far.
        ids.extend(store::folder_events::admitted_ids(&self.root));
        ids.extend(
            self.released
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .iter()
                .cloned(),
        );
        ids.sort();
        ids.dedup();
        ids
    }

    /// True for a session that existed before this process and the client has not released with
    /// `sessions.recover`: its workers may not be started by a sweep (DESIGN §6). No thread is
    /// enumerated to decide this: a session is deferred unless this process created, forked or
    /// unarchived it (`store::folder_events` admission) or the client released it.
    fn is_deferred(&self, session_id: &str) -> bool {
        !store::folder_events::is_admitted(&self.root, session_id)
            && !self
                .released
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .contains(session_id)
    }

    /// One session's share of a sweep tick. A deferred session only gets durable-stop
    /// propagation, and only when its verified view says a stop is active; everything else about
    /// it waits for `sessions.recover` (projection repair is the hydrator's job).
    fn sweep_session(&self, session_id: &str) -> Result<(), DaemonError> {
        let Some(entry) = self.catalog.get(session_id) else {
            return Ok(());
        };
        if matches!(entry.trust, crate::catalog::Trust::Corrupt(_)) {
            return Ok(());
        }
        let deferred = self.is_deferred(session_id);
        if deferred
            && !entry
                .view
                .as_ref()
                .is_some_and(|view| view.stop_active() || !view.children_exact)
        {
            return Ok(());
        }
        let session_dir = self.root.join("threads").join(session_id);
        let mut ledgers = fs::read_dir(&session_dir)
            .map_err(DaemonError::io)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(DaemonError::io)?;
        ledgers.retain(|entry| {
            entry.file_type().is_ok_and(|kind| kind.is_file())
                && entry.path().extension().and_then(|value| value.to_str()) == Some("jsonl")
                && entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name != endpoint::JOURNAL_FILE)
        });
        ledgers.sort_by_key(|entry| {
            (
                entry.file_name() != std::ffi::OsStr::new("main.jsonl"),
                entry.file_name(),
            )
        });
        self.propagate_durable_stops_before_sweep(session_id, &ledgers)?;
        if deferred {
            return Ok(());
        }
        for ledger in ledgers {
            let path = ledger.path();
            let scan = self.sweep_ledger_scan(&path)?;
            let ledger_last_seq = scan.last_seq;
            let facts = &scan.lifecycle;
            let key = if ledger.file_name() == std::ffi::OsStr::new("main.jsonl") {
                session_id.to_owned()
            } else {
                format!("{session_id}:{}", scan.line)
            };
            let owned_live = self
                .workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .get(&key)
                .is_some_and(|worker| worker.alive.load(Ordering::Acquire));
            if key == session_id && !owned_live {
                self.repair_main_projection(session_id, ledger_last_seq);
            }
            let lock_facts = if owned_live || probe_line_lock(&path)? == LineLockState::Busy {
                LockFacts::OTHER
            } else {
                LockFacts::FREE
            };
            let state = classify(facts, lock_facts);
            if ensure_action_at(state, facts, Some(&now_rfc3339())) == EnsureAction::None
                && !(key == session_id && goal_continuation_due(&path)?)
            {
                continue;
            }
            if !self.restart_is_due(&key) {
                continue;
            }
            self.refresh_limits(&path)?;
            if let Err(error) = self.schedule_worker_at(&key, session_id, path) {
                self.note_restart_failure(&key, session_id, error.bootstrap_code().as_bytes());
            }
        }
        Ok(())
    }

    /// Completes durable stop propagation before any line in the session can
    /// be classified for ordinary work. A partially propagated descendant is
    /// itself a root when its parent generation has already closed.
    fn propagate_durable_stops_before_sweep(
        &self,
        session_id: &str,
        ledgers: &[fs::DirEntry],
    ) -> Result<(), DaemonError> {
        let folder = self.root.join("threads").join(session_id);
        let mut active = Vec::new();
        let mut active_paths = HashSet::new();
        for ledger in ledgers {
            let path = ledger.path();
            let scan = self.sweep_ledger_scan(&path)?;
            if !scan.lifecycle.stop_active {
                continue;
            }
            let parent = scan.parent_file.as_deref().map(|file| folder.join(file));
            active_paths.insert(path.clone());
            active.push((path, scan.line.clone(), parent));
        }
        for (path, line, parent) in active {
            if parent
                .as_ref()
                .is_some_and(|parent| active_paths.contains(parent))
            {
                continue;
            }
            let process_key = if path.file_name() == Some(std::ffi::OsStr::new("main.jsonl")) {
                session_id.to_owned()
            } else {
                format!("{session_id}:{line}")
            };
            self.cascade_stop_from(session_id, &process_key, &path, &mut HashSet::new())?;
        }
        Ok(())
    }

    pub fn shutdown(&self) {
        self.shutdown_with(None, &HashSet::new());
    }

    /// `after_workers_stopped` runs between stopping the workers and flushing the cards (a test
    /// seam: it stands for the last append a killed worker had already made durable);
    /// `also_live` names sessions to treat as live at quit (a test has no real workers).
    fn shutdown_with(&self, after_workers_stopped: Option<&dyn Fn()>, also_live: &HashSet<String>) {
        self.draining.store(true, Ordering::Release);
        self.pending_workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear();
        self.admission_changed.notify_all();
        self.workers_changed.notify_all();
        let mut live_before = self.live_sessions_for_cleanup();
        live_before.extend(also_live.iter().cloned());
        let workers = self
            .workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .values()
            .cloned()
            .collect::<Vec<_>>();
        for worker in workers {
            let mut child = worker
                .child
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let _ = child.kill();
            let _ = child.wait();
            worker.alive.store(false, Ordering::Release);
        }
        // Workers first, then the cards (E831 ruling): a thread that was live at quit gets a
        // final fold of its now-stable ledger, so its card is not left Ahead. The flush is bounded
        // by the writer's shutdown budget, which is the most quit can be delayed.
        if let Some(after_workers_stopped) = after_workers_stopped {
            after_workers_stopped();
        }
        for session in live_before {
            let folder = self.root.join("threads").join(&session);
            self.card_writer.mark_dirty(&session, folder, None);
        }
        self.card_writer.shutdown();
    }

    fn ensure_running(&self, session_id: &str) -> Result<Arc<WorkerHandle>, DaemonError> {
        self.reset_restart_backoff(session_id);
        let ledger = self
            .root
            .join("threads")
            .join(session_id)
            .join("main.jsonl");
        self.refresh_limits(&ledger)?;
        if let Some(worker) = self.schedule_worker_at(session_id, session_id, ledger)? {
            return Ok(worker);
        }
        self.wait_for_worker(session_id)
    }

    fn live_worker(&self, session_id: &str) -> Option<Arc<WorkerHandle>> {
        let worker = self
            .workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(session_id)
            .cloned()?;
        worker.alive.load(Ordering::Acquire).then_some(worker)
    }

    fn spawn_worker_at(
        &self,
        process_key: &str,
        session_id: &str,
        ledger: PathBuf,
        startup: Option<&QueueTransaction>,
    ) -> Result<Arc<WorkerHandle>, DaemonError> {
        self.spawn_worker_at_with_handshake_timeout(
            process_key,
            session_id,
            ledger,
            startup,
            WORKER_HANDSHAKE_TIMEOUT,
        )
    }

    fn spawn_worker_at_with_handshake_timeout(
        &self,
        process_key: &str,
        session_id: &str,
        ledger: PathBuf,
        startup: Option<&QueueTransaction>,
        handshake_timeout: Duration,
    ) -> Result<Arc<WorkerHandle>, DaemonError> {
        let binary_digest = self.worker_binary_digest()?;
        if self
            .protocol_quarantine
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .iter()
            .any(|entry| entry.binary_digest == binary_digest)
        {
            return Err(DaemonError::protocol(
                "worker binary protocol tuple is quarantined",
            ));
        }
        let (workspace_id, folder_binding) = session_workspace_binding(&ledger)?;
        let workspace_quiescence =
            store::NamedLock::shared(workspace_quiescence_lock_path(&self.root, &workspace_id))
                .map_err(DaemonError::store)?;
        let mcp_routes = Arc::new(Mutex::new(
            None::<(DynamicToolCatalog, Arc<dyn DynamicSupervisorAuthority>)>,
        ));
        let mcp_routes_capture = Arc::clone(&mcp_routes);
        let mcp_runtime = &self.mcp_runtime;
        // goals.v1 is the host goal authority: an active or blocked session
        // record binds its id so the model's goal tools enter the catalog.
        let goal_id = session_controls::bound_goal_id(&self.root, session_id)
            .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
        let ordinal = self.next_run.fetch_add(1, Ordering::Relaxed);
        let timestamp = crate::host_runtime::system_timestamp().map_err(DaemonError::protocol)?;
        let run_id = format!("daemon-{}-{ordinal}", std::process::id());
        let launch_notices = Arc::new(Mutex::new(Vec::<SessionNotice>::new()));
        let launch_notices_capture = Arc::clone(&launch_notices);
        let mut launch = launch_profiled_worker_with_secret_store_and_binding_resolver(
            &self.repository,
            &ProfiledWorkerLaunchSpec {
                binary: self.worker_binary.clone(),
                ledger: ledger.clone(),
                timestamp,
                run_id: run_id.clone(),
                binary_attribution: self.build.clone(),
                workspace_id,
                folder_binding,
                user_agent_dir: self.user_agent_dir.clone(),
            },
            self.secret_store.as_ref(),
            move |config, instruction| {
                // MCP tools join the effective catalog before the policy is
                // validated against it: a policy may allow-list an MCP tool.
                let mcp = mcp_runtime
                    .prepare_workspace(&config.workspace.id)
                    .map_err(|error| error.to_string())?;
                if mcp_failure_is_required(&mcp.failures, &config.workspace.policy.allowed_tools) {
                    return Err("workspace policy requires an unavailable MCP server".to_owned());
                }
                // A degraded MCP catalog still launches; the session learns
                // what it is missing through a notice rather than a dead turn.
                *launch_notices_capture
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner) = mcp_launch_notices(
                    mcp.registry_failure.as_deref(),
                    &mcp.failures,
                    &mcp.tool_refusals,
                    &config.workspace.policy.allowed_tools,
                );
                let unavailable =
                    mcp.refused_allowed_tool_names(&config.workspace.policy.allowed_tools);
                // The bound goal is the session's durable goals.v1 record id
                // (never a session, ledger, or process id).
                let bindings = resolve_worker_launch_bindings(
                    config,
                    instruction,
                    goal_id.clone(),
                    mcp.catalog.tools,
                    unavailable,
                )
                .map_err(|error| error.to_string())?;
                *mcp_routes_capture
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner) = Some((
                    bindings.dynamic_catalog.clone(),
                    mcp.authority as Arc<dyn DynamicSupervisorAuthority>,
                ));
                Ok(bindings)
            },
        )
        .map_err(|error| DaemonError::required_broker(error.to_string()))?;
        let launch_notices = std::mem::take(
            &mut *launch_notices
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        );
        for notice in launch_notices {
            self.record_session_notice(session_id, &run_id, notice);
        }
        let handshake_deadline = Instant::now() + handshake_timeout;
        let mut guard = WorkerHandshakeGuard {
            child: Some(launch.child),
            credential_control: launch.credential_control.take(),
            credential_broker: launch.credential_broker.take(),
        };
        let (mut tool_launch_policy, tool_launcher) =
            self.freeze_tool_authority(&launch.config_snapshot, &launch.instruction_snapshot)?;
        if let Some(private_policy) = validator_tool_launch_policy(
            &ledger,
            &launch.config_snapshot,
            &launch.instruction_snapshot,
        )? {
            tool_launch_policy = private_policy;
        }
        let mut stdin = guard
            .child_mut()
            .stdin
            .take()
            .ok_or_else(|| DaemonError::protocol("worker stdin was not piped"))?;
        let stdout = guard
            .child_mut()
            .stdout
            .take()
            .ok_or_else(|| DaemonError::protocol("worker stdout was not piped"))?;
        let stderr = guard
            .child_mut()
            .stderr
            .take()
            .ok_or_else(|| DaemonError::protocol("worker stderr was not piped"))?;
        let mut reader = BufReader::new(stdout);
        let hello = read_worker_hello(&mut reader, handshake_deadline)?;
        let selected = match worker_control::negotiate(
            &hello,
            worker_control::PROTOCOL_VERSION,
            worker_control::PROTOCOL_VERSION,
        ) {
            Ok(selected) => selected.version,
            Err(worker_control::ProtocolError::NoMutualVersion) => {
                self.protocol_quarantine
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .insert(WorkerProtocolQuarantine {
                        binary_digest,
                        min: hello.min,
                        max: hello.max,
                    });
                let reject = encode_line(
                    "reject",
                    &Reject {
                        reason: "no_mutual_version".to_owned(),
                    },
                )?;
                stdin.write_all(&reject).map_err(DaemonError::io)?;
                stdin.flush().map_err(DaemonError::io)?;
                guard.wait_for_exit(Duration::from_secs(1));
                return Err(DaemonError::protocol("worker does not support protocol v2"));
            }
            Err(error) => return Err(DaemonError::protocol(error.to_string())),
        };
        stdin
            .write_all(&encode_line(
                "selected",
                &Selection {
                    version: selected,
                    startup: startup.map(|_| WorkerStartup::QueueTransaction),
                },
            )?)
            .map_err(DaemonError::io)?;
        if let Some(transaction) = startup {
            stdin
                .write_all(
                    &encode_queue_transaction(transaction)
                        .map_err(|error| DaemonError::protocol(error.to_string()))?,
                )
                .map_err(DaemonError::io)?;
        }
        stdin.flush().map_err(DaemonError::io)?;
        let child = guard.take_child();
        let credential_control = guard.take_credential_control();
        let credential_broker = guard.take_credential_broker();
        let (dynamic_catalog, dynamic_authority) = mcp_routes
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take()
            .map_or_else(
                || (DynamicToolCatalog::default(), None),
                |(catalog, authority)| (catalog, Some(authority)),
            );
        let handle = Arc::new(WorkerHandle {
            _workspace_quiescence: workspace_quiescence,
            stdin: Mutex::new(stdin),
            child: Mutex::new(child),
            state: Mutex::new(WorkerState::default()),
            stderr_tail: Mutex::new(Vec::new()),
            changed: Condvar::new(),
            alive: AtomicBool::new(true),
            tool_launch_policy,
            tool_launcher,
            credential_control: Mutex::new(credential_control),
            credential_broker: Mutex::new(credential_broker),
            credential_bindings: Mutex::new(launch.credential_bindings.take()),
            config_snapshot: launch.config_snapshot,
            dynamic_catalog,
            dynamic_authority,
            tool_cancellation: tools::CancellationToken::default(),
        });
        let reader_handle = Arc::clone(&handle);
        let owner = self.self_weak.clone();
        let reader_session = session_id.to_owned();
        let reader_process_key = process_key.to_owned();
        let reader_ledger = ledger.clone();
        std::thread::spawn(move || {
            worker_reader(
                reader,
                reader_handle,
                owner,
                reader_session,
                reader_process_key,
                reader_ledger,
            )
        });
        let stderr_handle = Arc::clone(&handle);
        std::thread::spawn(move || worker_stderr_reader(BufReader::new(stderr), stderr_handle));
        Ok(handle)
    }

    pub fn workspace_service_binary(&self) -> PathBuf {
        self.worker_binary.with_file_name("tekes-workspace-service")
    }

    fn worker_binary_digest(&self) -> Result<[u8; 32], DaemonError> {
        let bytes = fs::read(&self.worker_binary).map_err(DaemonError::io)?;
        Ok(Sha256::digest(bytes).into())
    }

    fn refresh_limits(&self, ledger: &Path) -> Result<(), DaemonError> {
        let workspace = workspace_id(ledger)?;
        let snapshot = self
            .repository
            .resolve(&workspace)
            .map_err(|error| DaemonError::invalid_config(error.to_string()))?;
        let limits = snapshot.settings.limits.as_ref();
        self.max_workers.store(
            limits.and_then(|limits| limits.max_workers).unwrap_or(1),
            Ordering::Release,
        );
        self.max_provider_leases.store(
            limits
                .and_then(|limits| limits.max_provider_leases)
                .unwrap_or(1),
            Ordering::Release,
        );
        Ok(())
    }

    pub fn preflight_mandatory_authorities(&self) -> Result<(), DaemonError> {
        if !self.root.join("jobs").is_dir() {
            return Err(DaemonError::required_broker(
                "durable job authority is unavailable",
            ));
        }
        if self.is_mcp_only_app_sandbox_host()? {
            tools::HelperJobLauncher::disabled(self.worker_binary.with_file_name("tekes-helper"))
                .map_err(|error| DaemonError::required_broker(error.to_string()))?;
        } else {
            tools::HelperJobLauncher::new(
                self.worker_binary.with_file_name("tekes-helper"),
                tools::probe_backend(production_sandbox_backend()),
            )
            .map_err(|error| DaemonError::required_broker(error.to_string()))?;
        }
        Ok(())
    }

    fn is_mcp_only_app_sandbox_host(&self) -> Result<bool, DaemonError> {
        let marker = self.root.join(MCP_ONLY_APP_SANDBOX_MARKER);
        // SAFETY: geteuid has no preconditions and does not access user data.
        let expected_uid = unsafe { libc::geteuid() };
        match fs::symlink_metadata(&marker) {
            Ok(metadata)
                if metadata.file_type().is_file()
                    && metadata.uid() == expected_uid
                    && metadata.mode() & 0o777 == 0o600 =>
            {
                Ok(true)
            }
            Ok(_) => Err(DaemonError::required_broker(
                "MCP-only App Sandbox marker has invalid ownership, mode or type",
            )),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(DaemonError::io(error)),
        }
    }

    fn freeze_tool_authority(
        &self,
        config: &ConfigSnapshot,
        instruction: &profile::InstructionSnapshot,
    ) -> Result<(tools::JobLaunchPolicy, Arc<tools::HelperJobLauncher>), DaemonError> {
        let launch_policy = frozen_tool_launch_policy(config, instruction)?;
        let executable = self.worker_binary.with_file_name("tekes-helper");
        let launcher = if self.is_mcp_only_app_sandbox_host()? {
            tools::HelperJobLauncher::disabled(executable)
        } else {
            tools::HelperJobLauncher::new(
                executable,
                tools::probe_backend(production_sandbox_backend()),
            )
        }
        .map_err(|error| DaemonError::required_broker(error.to_string()))?;
        let launcher = Arc::new(launcher);
        Ok((launch_policy, launcher))
    }

    fn schedule_main(&self, session_id: &str) -> Result<Option<Arc<WorkerHandle>>, DaemonError> {
        self.schedule_worker_at(
            session_id,
            session_id,
            self.root
                .join("threads")
                .join(session_id)
                .join("main.jsonl"),
        )
    }

    fn schedule_line(
        &self,
        session_id: &str,
        line_id: &str,
        line_file: &str,
    ) -> Result<Option<Arc<WorkerHandle>>, DaemonError> {
        let (process_key, ledger) =
            line_schedule_target(&self.root, session_id, line_id, line_file)?;
        if let Some(parent_key) =
            unresolved_parent_dependency(&self.root, session_id, line_id, &ledger)?
        {
            return self.schedule_child_from_parent(&parent_key, &process_key, session_id, ledger);
        }
        self.schedule_worker_at(&process_key, session_id, ledger)
    }

    fn schedule_worker_at(
        &self,
        process_key: &str,
        session_id: &str,
        ledger: PathBuf,
    ) -> Result<Option<Arc<WorkerHandle>>, DaemonError> {
        Ok(self
            .schedule_worker_at_with_startup(process_key, session_id, ledger, None)?
            .worker)
    }

    fn schedule_worker_at_with_startup(
        &self,
        process_key: &str,
        session_id: &str,
        ledger: PathBuf,
        startup: Option<&QueueTransaction>,
    ) -> Result<ScheduledWorker, DaemonError> {
        // A thread is verified (a torn tail repaired, its view folded) before its first worker
        // of this process starts; the lazy boot no longer does that for every thread.
        if self.live_worker(process_key).is_none() {
            ProductionProcessHost::ensure_verified(self, session_id)?;
        }
        self.refresh_limits(&ledger)?;
        let mut workers = self
            .workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(worker) = workers
            .get(process_key)
            .filter(|worker| worker.alive.load(Ordering::Acquire))
            .cloned()
        {
            drop(workers);
            self.refresh_worker_credentials(&worker)?;
            return Ok(ScheduledWorker {
                worker: Some(worker),
                startup_preloaded: false,
            });
        }
        if !self.has_worker_capacity(&workers) {
            let mut pending = self
                .pending_workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let pending = pending
                .entry(process_key.to_owned())
                .or_insert_with(|| PendingWorker {
                    session_id: session_id.to_owned(),
                    ledger,
                    startup: None,
                });
            if let Some(startup) = startup {
                match pending.startup.as_ref() {
                    Some(existing) if existing != startup => {
                        return Err(DaemonError::protocol(
                            "conflicting queue transaction is already pending",
                        ));
                    }
                    Some(_) => {}
                    None => pending.startup = Some(startup.clone()),
                }
            }
            return Ok(ScheduledWorker {
                worker: None,
                startup_preloaded: startup.is_some(),
            });
        }
        let pending_startup = self
            .pending_workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(process_key)
            .and_then(|pending| pending.startup.clone());
        let effective_startup = match (startup, pending_startup.as_ref()) {
            (Some(requested), Some(pending)) if requested != pending => {
                return Err(DaemonError::protocol(
                    "conflicting queue transaction is already pending",
                ));
            }
            (Some(requested), _) => Some(requested),
            (None, pending) => pending,
        };
        let worker = self.spawn_worker_at(process_key, session_id, ledger, effective_startup)?;
        self.pending_workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(process_key);
        workers.insert(process_key.to_owned(), Arc::clone(&worker));
        drop(workers);
        self.workers_changed.notify_all();
        self.publish_session_status(session_id);
        Ok(ScheduledWorker {
            worker: Some(worker),
            startup_preloaded: effective_startup.is_some(),
        })
    }

    fn schedule_child_from_parent(
        &self,
        parent_process_key: &str,
        child_process_key: &str,
        session_id: &str,
        ledger: PathBuf,
    ) -> Result<Option<Arc<WorkerHandle>>, DaemonError> {
        self.refresh_limits(&ledger)?;
        let mut workers = self
            .workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(worker) = workers
            .get(child_process_key)
            .filter(|worker| worker.alive.load(Ordering::Acquire))
            .cloned()
        {
            drop(workers);
            self.refresh_worker_credentials(&worker)?;
            return Ok(Some(worker));
        }
        let parent_is_live_waiter = workers
            .get(parent_process_key)
            .is_some_and(|worker| worker.alive.load(Ordering::Acquire));
        if !child_dependency_admitted(self.has_worker_capacity(&workers), parent_is_live_waiter) {
            self.pending_workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .entry(child_process_key.to_owned())
                .or_insert_with(|| PendingWorker {
                    session_id: session_id.to_owned(),
                    ledger,
                    startup: None,
                });
            return Ok(None);
        }
        let worker = self.spawn_worker_at(child_process_key, session_id, ledger, None)?;
        self.pending_workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(child_process_key);
        workers.insert(child_process_key.to_owned(), Arc::clone(&worker));
        drop(workers);
        self.workers_changed.notify_all();
        self.publish_session_status(session_id);
        Ok(Some(worker))
    }

    fn wait_for_worker(&self, process_key: &str) -> Result<Arc<WorkerHandle>, DaemonError> {
        let deadline = Instant::now() + DELIVERY_TIMEOUT;
        let mut workers = self
            .workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        loop {
            if let Some(worker) = workers
                .get(process_key)
                .filter(|worker| worker.alive.load(Ordering::Acquire))
            {
                return Ok(Arc::clone(worker));
            }
            if self.draining.load(Ordering::Acquire) {
                return Err(DaemonError::required_broker("server-draining"));
            }
            let now = Instant::now();
            if now >= deadline {
                return Err(DaemonError::required_broker("worker-admission-timed-out"));
            }
            let (next, _) = self
                .workers_changed
                .wait_timeout(workers, deadline - now)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            workers = next;
        }
    }

    fn start_pending_workers(&self) {
        loop {
            if self.draining.load(Ordering::Acquire) {
                return;
            }
            let mut workers = self
                .workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if !self.has_worker_capacity(&workers) {
                return;
            }
            let next = {
                let pending = self
                    .pending_workers
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                let Some(key) = pending.keys().next().cloned() else {
                    return;
                };
                pending.get(&key).cloned().map(|pending| (key, pending))
            };
            let Some((process_key, pending)) = next else {
                continue;
            };
            let needs_worker = if pending.startup.is_some() {
                true
            } else {
                match ledger_needs_worker(&pending.ledger) {
                    Ok(needs_worker) => needs_worker,
                    Err(error) => {
                        eprintln!("process-host-pending-probe-failed: {error}");
                        return;
                    }
                }
            };
            if !needs_worker {
                self.pending_workers
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .remove(&process_key);
                continue;
            }
            match self.spawn_worker_at(
                &process_key,
                &pending.session_id,
                pending.ledger.clone(),
                pending.startup.as_ref(),
            ) {
                Ok(worker) => {
                    self.pending_workers
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .remove(&process_key);
                    workers.insert(process_key, worker);
                    drop(workers);
                    self.workers_changed.notify_all();
                    self.publish_session_status(&pending.session_id);
                }
                Err(error) => {
                    eprintln!("process-host-pending-start-failed: {error}");
                    return;
                }
            }
        }
    }

    fn has_worker_capacity(&self, workers: &HashMap<String, Arc<WorkerHandle>>) -> bool {
        let live = workers
            .values()
            .filter(|worker| worker.alive.load(Ordering::Acquire))
            .count() as u64;
        live < self.max_workers.load(Ordering::Acquire)
    }

    fn request_provider_lease(&self, lease: AdmissionLease) -> bool {
        let attempt = lease.attempt.clone();
        let mut waiters = self
            .admission_waiters
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if !waiters.contains(&attempt) {
            waiters.push_back(attempt.clone());
        }
        loop {
            if self.draining.load(Ordering::Acquire) {
                waiters.retain(|waiting| waiting != &attempt);
                self.admission_changed.notify_all();
                return false;
            }
            let is_front = waiters.front() == Some(&attempt);
            if is_front {
                let mut admission = self
                    .admission
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                if admission.is_held(&attempt)
                    || ((admission.held_count() as u64)
                        < self.max_provider_leases.load(Ordering::Acquire)
                        && admission.request(lease.clone()))
                {
                    waiters.pop_front();
                    drop(admission);
                    self.admission_changed.notify_all();
                    return true;
                }
            }
            waiters = self
                .admission_changed
                .wait(waiters)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }

    fn locked_prompt(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        prompt: &MaterializedPrompt,
        steer: bool,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        let content = IJsonValue::parse(&serde_json::to_vec(&prompt.blocks).map_err(internal)?)
            .map_err(internal)?;
        let receipt = self
            .endpoint
            .prompt_for_endpoint(session_id, timestamp, origin, content, steer)
            .map_err(internal)?;
        let source = self
            .first_root_input(session_id)
            .unwrap_or_else(|| prompt_text(&prompt.blocks));
        let should_refine = self.seed_automatic_title(session_id, timestamp, &source);
        self.reset_restart_backoff(session_id);
        if let Err(error) = self.schedule_main(session_id) {
            self.record_spawn_failure("prompt", session_id, receipt.seq, &error);
        }
        if should_refine {
            self.spawn_automatic_title_refinement(session_id.to_owned(), source);
        }
        Ok(receipt)
    }

    /// Manual compaction on a line with no live worker: the supervisor is the
    /// locked author (D-61 whitelist: manual `compact`). It mirrors the
    /// worker's manual path exactly — checkpoint at the current key floor, the
    /// pure `engine::plan_context_compaction` plan for the turn after the
    /// terminal tail (or the open turn), summary inline or spilled — and keys
    /// the event by the request origin so a retry is the original receipt.
    fn locked_compact(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        // The summary request runs before the line lock; the lock then
        // re-plans and the bundle must still be the plan (engine::CompactionSummary::apply).
        // The request blocks on its own HTTP runtime, so it runs on a plain
        // thread: the caller is usually an endpoint task on the async runtime
        // (the same rule as the post-turn memorization).
        let summary = std::thread::scope(|scope| {
            scope
                .spawn(|| self.summary_for_manual_compaction(session_id))
                .join()
                .unwrap_or_else(|_| Err("summary request thread panicked".to_owned()))
        });
        let summary = match summary {
            Ok(summary) => summary,
            Err(error) => {
                eprintln!(
                    "process-host-compaction-summary-skipped: session={session_id} error={error}"
                );
                None
            }
        };
        let appended = self
            .endpoint
            .author_keyed_with_ledger(session_id, origin, |ledger| {
                let (turn, plan) = {
                    let projection = ledger.projection().ok_or_else(|| {
                        store::StoreError::Corruption("empty thread ledger".to_owned())
                    })?;
                    let turn = projection
                        .latest_turn
                        .unwrap_or(0)
                        .saturating_add(u64::from(
                            projection.terminal_tail || projection.latest_turn.is_none(),
                        ));
                    let plan = engine::plan_context_compaction(&projection.events, turn).map_err(
                        |error| store::StoreError::Corruption(format!("compaction plan: {error}")),
                    )?;
                    (turn, plan)
                };
                let _ = turn;
                let (summary_record, summary_text) = match summary.as_ref() {
                    Some(model_summary) => {
                        let (record, text) =
                            model_summary.apply(&plan.covers, plan.summary.clone());
                        (Some(record), text)
                    }
                    None => (None, plan.summary.clone()),
                };
                ledger.create_checkpoint(timestamp, "manual compaction boundary")?;
                let summary = spill_compaction_summary(ledger, &summary_text)?;
                let mut value = serde_json::json!({
                    "v": 1, "seq": ledger.next_seq(), "kind": "compact", "ts": timestamp,
                    "covers": seq_ranges(&plan.covers), "summary": summary,
                    "origin_key": origin.key, "origin_tuple": origin,
                });
                if let Some(record) = summary_record {
                    value["summary_request"] = record;
                }
                let bytes = serde_json::to_vec(&value)
                    .map_err(|error| store::StoreError::Corruption(error.to_string()))?;
                let parsed = IJsonValue::parse(&bytes)
                    .map_err(|error| store::StoreError::Corruption(error.to_string()))?;
                Ok(Some(schema::Event::from_value(parsed)?))
            })
            .map_err(internal)?;
        let receipt = appended.ok_or_else(|| internal("manual compaction produced no event"))?;
        if !receipt.deduplicated {
            self.publish_appended(session_id).map_err(internal)?;
        }
        Ok(receipt)
    }

    fn first_root_input(&self, session_id: &str) -> Option<String> {
        let folder = self.root.join("threads").join(session_id);
        let bytes = fs::read(folder.join("main.jsonl")).ok()?;
        let projection = scan_valid_prefix(&bytes, schema::READER_VERSION).projection?;
        projection
            .events
            .iter()
            .find(|event| *event.kind() == EventKind::Input)
            .and_then(|event| serde_json::to_value(event.raw()).ok())
            .and_then(|event| {
                event
                    .get("content")
                    .and_then(serde_json::Value::as_array)
                    .cloned()
            })
            .map(|blocks| {
                blocks
                    .iter()
                    .filter_map(|block| {
                        (block.get("type").and_then(serde_json::Value::as_str) == Some("text"))
                            .then(|| block.get("text").and_then(serde_json::Value::as_str))
                            .flatten()
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .filter(|value| !value.trim().is_empty())
    }

    /// Returns true only for the process that durably won the one-shot naming
    /// claim. A failed provider request therefore keeps the useful fallback
    /// and is not retried by later prompts or another supervisor instance.
    fn seed_automatic_title(&self, session_id: &str, timestamp: &str, source: &str) -> bool {
        match self.try_seed_automatic_title(session_id, timestamp, source) {
            Ok(value) => value,
            Err(error) => {
                eprintln!(
                    "process-host-automatic-title-fallback-failed: session={session_id} error={error}"
                );
                false
            }
        }
    }

    fn try_seed_automatic_title(
        &self,
        session_id: &str,
        timestamp: &str,
        source: &str,
    ) -> Result<bool, endpoint::NativeEndpointError> {
        let fallback = deterministic_automatic_thread_title(source);
        if fallback.is_empty() {
            return Ok(false);
        }
        let origin = automatic_title_origin(session_id, AUTOMATIC_TITLE_SEED_OPERATION);
        match self
            .endpoint
            .seed_automatic_title_if_missing(session_id, timestamp, &origin, &fallback)
        {
            Ok(Some(receipt)) if !receipt.deduplicated => {
                if let Err(error) = self.publish_appended(session_id) {
                    eprintln!(
                        "process-host-automatic-title-fallback-publish-failed: session={session_id} error={error}"
                    );
                }
                Ok(true)
            }
            Ok(Some(_) | None) => Ok(false),
            Err(error) => Err(error),
        }
    }

    fn wait_and_seed_automatic_title(
        &self,
        session_id: &str,
        timestamp: &str,
        source: &str,
    ) -> bool {
        loop {
            if self.draining.load(Ordering::Acquire) {
                return false;
            }
            match self.try_seed_automatic_title(session_id, timestamp, source) {
                Ok(value) => return value,
                Err(endpoint::NativeEndpointError::Store(store::StoreError::Busy)) => {
                    std::thread::sleep(Duration::from_millis(25));
                }
                Err(error) => {
                    eprintln!(
                        "process-host-automatic-title-fallback-failed: session={session_id} error={error}"
                    );
                    return false;
                }
            }
        }
    }

    fn spawn_automatic_title_refinement(&self, session_id: String, source: String) {
        let owner = self.self_weak.clone();
        std::thread::spawn(move || {
            let Some(owner) = owner.upgrade() else {
                return;
            };
            if let Err(error) = owner.refine_automatic_title(&session_id, &source) {
                eprintln!(
                    "process-host-automatic-title-refinement-failed: session={session_id} error={error}"
                );
            }
        });
    }

    fn spawn_automatic_title_seed_and_refinement(
        &self,
        session_id: String,
        timestamp: String,
        submitted_source: String,
    ) {
        let owner = self.self_weak.clone();
        std::thread::spawn(move || {
            let Some(owner) = owner.upgrade() else {
                return;
            };
            let source = owner
                .first_root_input(&session_id)
                .unwrap_or(submitted_source);
            if owner.wait_and_seed_automatic_title(&session_id, &timestamp, &source)
                && let Err(error) = owner.refine_automatic_title(&session_id, &source)
            {
                eprintln!(
                    "process-host-automatic-title-refinement-failed: session={session_id} error={error}"
                );
            }
        });
    }

    fn refine_automatic_title(&self, session_id: &str, source: &str) -> Result<(), String> {
        if self.draining.load(Ordering::Acquire) {
            return Ok(());
        }
        let config = self
            .endpoint
            .session_config_snapshot(session_id)
            .map_err(|error| format!("config snapshot: {error}"))?;
        let generated = self.generate_automatic_title(&config.providers, source)?;
        let origin = automatic_title_origin(session_id, AUTOMATIC_TITLE_REFINE_OPERATION);
        let timestamp = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let receipt = loop {
            if self.draining.load(Ordering::Acquire) {
                return Ok(());
            }
            match self
                .endpoint
                .refine_automatic_title(session_id, &timestamp, &origin, &generated)
            {
                Ok(receipt) => break receipt,
                Err(endpoint::NativeEndpointError::Store(store::StoreError::Busy)) => {
                    // The main worker owns the ledger for its complete run.
                    // Wait to persist this already-generated result; never send
                    // a second provider request.
                    std::thread::sleep(Duration::from_millis(25));
                }
                Err(error) => return Err(format!("persist: {error}")),
            }
        };
        if receipt.is_some_and(|receipt| !receipt.deduplicated) {
            self.publish_appended(session_id)
                .map_err(|error| format!("publish: {error}"))?;
        }
        Ok(())
    }

    /// Send one prepared automatic-helper request and return its text.
    fn complete_automatic_text(
        &self,
        configured: &profile::Provider,
        resolved: &provider::ResolvedDialectProfile,
        prepared: &provider::PreparedRequest,
        timeout: Duration,
    ) -> Result<String, String> {
        let terminal = self.complete_automatic_terminal(configured, resolved, prepared, timeout)?;
        if terminal.finish_reason != provider::FinishReason::Completed
            || !terminal.tool_calls.is_empty()
        {
            return Err(format!("unexpected terminal: {:?}", terminal.finish_reason));
        }
        let raw = terminal
            .content
            .iter()
            .filter_map(|block| match block {
                provider::ContentBlock::Text(text) => Some(text.as_str()),
                provider::ContentBlock::Reasoning(_) => None,
            })
            .collect::<Vec<_>>()
            .join("");
        Ok(raw)
    }

    /// One non-streaming automatic request; the caller judges the terminal.
    fn complete_automatic_terminal(
        &self,
        configured: &profile::Provider,
        resolved: &provider::ResolvedDialectProfile,
        prepared: &provider::PreparedRequest,
        timeout: Duration,
    ) -> Result<provider::ProviderTerminal, String> {
        let credential = configured
            .credential_key
            .as_deref()
            .map(|key| self.secret_store.resolve(key))
            .transpose()
            .map_err(|error| format!("credential: {error}"))?;
        let material = match credential.as_ref() {
            Some(provider::SecretResolution::Active(provider::SecretRecord::Active {
                material,
                ..
            })) => material.as_str(),
            Some(provider::SecretResolution::Active(provider::SecretRecord::Revoked {
                ..
            }))
            | Some(provider::SecretResolution::Revoked { .. }) => {
                return Err("credential is revoked".to_owned());
            }
            Some(provider::SecretResolution::NotFound) => {
                return Err("credential is missing".to_owned());
            }
            None => "",
        };
        let runtime = provider::HttpRuntime::new().map_err(|error| error.to_string())?;
        let cancelled = Arc::new(AtomicBool::new(false));
        #[cfg(test)]
        let transport_endpoint = self
            .automatic_title_test_redirect
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        #[cfg(not(test))]
        let transport_endpoint: Option<String> = None;
        let completion = runtime
            .send_dialect_with_frames_and_wall_transport(
                resolved.dialect,
                prepared,
                transport_endpoint.as_deref(),
                material,
                &cancelled,
                Some(timeout),
                |_| Ok(()),
            )
            .map_err(|error| error.to_string())?;
        match completion {
            provider::ProviderCompletion::Terminal(terminal) => Ok(terminal),
            provider::ProviderCompletion::Failure(failure) => {
                Err(format!("provider failure: {failure:?}"))
            }
        }
    }

    /// The model summary for a supervisor-authored manual compact: planned
    /// outside the line lock over the current ledger, the request on the
    /// session's model (else the configured defaults); `None` when nothing
    /// would be covered or no route can be prepared (reported on stderr — a
    /// missing summary request is never a failed compaction: the deterministic
    /// quoted history is written instead).
    fn summary_for_manual_compaction(
        &self,
        session_id: &str,
    ) -> Result<Option<engine::CompactionSummary>, String> {
        let config = self
            .endpoint
            .session_config_snapshot(session_id)
            .map_err(|error| format!("config snapshot: {error}"))?;
        let folder = self.root.join("threads").join(session_id);
        let bytes = fs::read(folder.join("main.jsonl")).map_err(|error| error.to_string())?;
        let projection = scan_valid_prefix(&bytes, schema::READER_VERSION)
            .projection
            .ok_or("session has no projection")?;
        let turn = projection
            .latest_turn
            .unwrap_or(0)
            .saturating_add(u64::from(
                projection.terminal_tail || projection.latest_turn.is_none(),
            ));
        let plan = engine::plan_context_compaction(&projection.events, turn)
            .map_err(|error| error.to_string())?;
        if plan.covers.is_empty() {
            return Ok(None);
        }
        let (configured, model, resolved) = session_model_route(&config)?;
        let bundle = engine::freeze_source_bundle(
            &projection.events,
            &plan.covers,
            engine::summary_request_bytes(model.context_window_tokens),
        )?;
        let admission = (|| -> Result<_, String> {
            let schema =
                tools::fixed_schema("summary_artifact").ok_or("summary_artifact schema missing")?;
            let catalog = IJsonValue::parse(
                &serde_json::to_vec(&serde_json::json!([schema.model_schema()]))
                    .map_err(|error| error.to_string())?,
            )
            .map_err(|error| error.to_string())?;
            let prepared = provider::prepare_summary_request(
                &configured.endpoint,
                &resolved,
                engine::SUMMARY_SYSTEM,
                &bundle.rendered,
                catalog,
                format!("compaction-summary-{}", uuid::Uuid::new_v4()),
            )?;
            self.complete_automatic_terminal(
                &configured,
                &resolved,
                &prepared,
                COMPACTION_SUMMARY_TIMEOUT,
            )
        })();
        let (outcome, usage) = match admission {
            Ok(terminal) => {
                let (artifact, usage) = provider::summary_completion_artifact(Ok(
                    provider::ProviderCompletion::Terminal(terminal),
                ));
                (
                    artifact
                        .and_then(|arguments| engine::admit_summary_artifact(&bundle, &arguments)),
                    usage,
                )
            }
            Err(reason) => (Err(reason), None),
        };
        Ok(Some(engine::CompactionSummary::from_outcome(
            &model.id, bundle, outcome, usage,
        )))
    }

    fn generate_automatic_title(
        &self,
        providers: &profile::ProvidersConfig,
        source: &str,
    ) -> Result<String, String> {
        let (configured, _model, resolved) = automatic_title_route(providers)?;
        let prepared = prepare_automatic_title_request(
            &configured,
            &resolved,
            source,
            format!("automatic-title-{}", uuid::Uuid::new_v4()),
        )?;
        let raw = self.complete_automatic_text(
            &configured,
            &resolved,
            &prepared,
            AUTOMATIC_THREAD_TITLE_TIMEOUT,
        )?;
        let title = normalized_automatic_thread_title(&raw);
        if title.is_empty() {
            return Err("provider returned an empty title".to_owned());
        }
        Ok(title)
    }

    fn handle_tool_control(
        &self,
        handle: &WorkerHandle,
        line: &[u8],
    ) -> Result<Vec<u8>, DaemonError> {
        let jobs = JobBrokerSupervisorAuthority::new(
            self.root.join("jobs"),
            64 * 1024,
            handle.tool_launch_policy.clone(),
            Arc::clone(&handle.tool_launcher),
            handle.tool_cancellation.clone(),
        )
        .map_err(|error| DaemonError::required_broker(error.to_string()))?;
        let runtime = ProcessToolRuntime {
            host: self.self_weak.clone(),
        };
        let mut handler = ProductionToolControlHandler::new(self.root.clone(), runtime, jobs);
        if let Some(authority) = handle.dynamic_authority.as_ref() {
            handler =
                handler.with_dynamic_routes(handle.dynamic_catalog.clone(), Arc::clone(authority));
        }
        let mut session = ToolControlSession::new(
            self.root.clone(),
            worker_control::Selected { version: 2 },
            handler,
            ProductionToolControlPolicy::new(tools::SecretScanner::default()),
        );
        session
            .handle_line(line)
            .map_err(|error| DaemonError::protocol(error.to_string()))
    }

    /// One `tool_continuation` step from a worker whose tool-control result was
    /// pending. Binding, step order and receipt replay are the continuation
    /// journal's; this only routes to the bound dynamic authority.
    fn handle_tool_continuation(
        &self,
        handle: &WorkerHandle,
        line: &[u8],
    ) -> Result<Vec<u8>, DaemonError> {
        let request = worker_control::continuation::decode_tool_continuation(line)
            .map_err(|error| DaemonError::protocol(error.to_string()))?;
        let thread_folder = self.root.join("threads").join(&request.original.session);
        let metadata = fs::symlink_metadata(&thread_folder)
            .map_err(|_| DaemonError::protocol("continuation names an unknown thread"))?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(DaemonError::protocol(
                "continuation names an unknown thread",
            ));
        }
        let jobs = JobBrokerSupervisorAuthority::new(
            self.root.join("jobs"),
            64 * 1024,
            handle.tool_launch_policy.clone(),
            Arc::clone(&handle.tool_launcher),
            handle.tool_cancellation.clone(),
        )
        .map_err(|error| DaemonError::required_broker(error.to_string()))?;
        let runtime = ProcessToolRuntime {
            host: self.self_weak.clone(),
        };
        let mut handler = ProductionToolControlHandler::new(self.root.clone(), runtime, jobs);
        if let Some(authority) = handle.dynamic_authority.as_ref() {
            handler =
                handler.with_dynamic_routes(handle.dynamic_catalog.clone(), Arc::clone(authority));
        }
        let response = handler.continue_task(&request);
        let response = ProductionToolControlPolicy::new(tools::SecretScanner::default())
            .apply_continuation(response);
        worker_control::continuation::encode_tool_continuation_result(&response)
            .map_err(|error| DaemonError::protocol(error.to_string()))
    }

    fn validate_child_proof(&self, proof: &ChildLaunchProof) -> Result<(), DaemonError> {
        let ledger = self
            .root
            .join("threads")
            .join(&proof.session)
            .join(&proof.child_file);
        let genesis = store::read_genesis(&ledger)
            .map_err(DaemonError::store)?
            .ok_or_else(|| DaemonError::corrupt("child ledger has no projection"))?;
        if genesis.string_field("thread") != Some(proof.child_line.as_str()) {
            return Err(DaemonError::corrupt(
                "child proof line does not match child genesis",
            ));
        }
        Ok(())
    }

    fn publish_appended(&self, session_id: &str) -> Result<(), DaemonError> {
        self.publish_appended_with(session_id, None)
    }

    /// `after_decode` is a test seam called between the ledger decode and the journal
    /// reconcile; it lets a test grow the ledger exactly where a concurrent respond would.
    fn publish_appended_with(
        &self,
        session_id: &str,
        after_decode: Option<&dyn Fn()>,
    ) -> Result<(), DaemonError> {
        let streams = self
            .streams
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        let folder = self.root.join("threads").join(session_id);
        let ledger_path = folder.join("main.jsonl");
        // Phase 1, no lock: read and fold the ledger and build the card. This is the part that
        // grows with the thread, so concurrent folds of different threads (a cold List, the
        // upgrade pass) run in parallel and never hold up a live session's doorbell (E832 M2).
        let mut bytes = fs::read(&ledger_path).map_err(DaemonError::io)?;
        let mut scan = scan_valid_prefix(&bytes, schema::READER_VERSION);
        // A complete line that does not validate (or a ledger with no valid genesis) is damage a
        // repair cannot fix: quarantine the thread (E832 M3). A torn tail -- no complete line
        // after the valid prefix -- is a partial append and only needs repair. The decision is
        // taken on a read under the line's lock: one unlocked read can straddle another process's
        // repair or append and show a record that never existed (O-final m3); while a live holder
        // owns the line the decision is deferred.
        if ledger_damage(&bytes).is_some() {
            match damage_under_lock(&ledger_path) {
                DamageVerdict::Damaged(offset) => {
                    let reason = format!("main.jsonl: no valid record at byte {offset}");
                    eprintln!("process-host-quarantined: session={session_id} {reason}");
                    self.catalog.mark_corrupt(session_id, &reason);
                    return Err(DaemonError::corrupt(format!("quarantined: {reason}")));
                }
                DamageVerdict::Sound | DamageVerdict::Deferred => {
                    // Not damage (now): the new read is the truth.
                    bytes = fs::read(&ledger_path).map_err(DaemonError::io)?;
                    scan = scan_valid_prefix(&bytes, schema::READER_VERSION);
                }
            }
        }
        let kernel_file_len = bytes.len() as u64;
        let valid_bytes = usize::try_from(scan.valid_bytes).unwrap_or(usize::MAX);
        // The shared-decode fast paths below are only taken for a clean read (no torn tail,
        // which the separate-read paths reject with "requires tail recovery") and only while
        // the ledger has not grown since this decode; otherwise the previous behaviour applies.
        let clean_read = !scan.needs_repair();
        let projection = scan
            .projection
            .ok_or_else(|| DaemonError::corrupt("active thread has no projection"))?;
        let due = self.card_writer.is_due(session_id);
        // Between card writes the catalog entry is advanced from the fold without rebuilding
        // the card view (K6); the writer re-folds the thread when its window expires.
        let advanced_light =
            clean_read && !due && self.catalog.refresh_light(session_id, &projection);
        #[cfg(test)]
        if clean_read && !advanced_light {
            self.card_builds.fetch_add(1, Ordering::Relaxed);
        }
        let prepared = if clean_read && !advanced_light {
            crate::thread_card::prepare(
                session_id,
                folder.clone(),
                bytes,
                valid_bytes,
                &projection,
                None,
            )
        } else {
            None
        };
        if let Some(after_decode) = after_decode {
            after_decode();
        }
        // Phase 2, under the projection lock: the journal, the published frames, the
        // actionables and the card hand-off. A fold that finished after a newer one (two
        // callers raced) only re-describes older bytes: the journal reconcile appends nothing,
        // the actionables guard (K9) refuses it, and the catalog refuses an older view.
        let _projection_guard = self
            .projection_lock
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let journal = EndpointJournal::open(&folder)
            .map_err(|error| DaemonError::corrupt(error.to_string()))?;
        let mut projector = Projector::default();
        let appended = projector
            .reconcile(&projection.events, &journal)
            .map_err(|error| DaemonError::corrupt(error.to_string()))?;
        // The ledger may grow while the journal is reconciled (a respond appends its
        // `approval_response` and reconciles the actionables itself, outside `projection_lock`).
        // The shared projection is only trusted while the file still has the length it was
        // decoded at; otherwise the decode-per-call path runs, exactly as before this change.
        let ledger_unchanged = |len: u64| {
            clean_read && fs::metadata(&ledger_path).is_ok_and(|metadata| metadata.len() == len)
        };
        if let Some(streams) = &streams {
            let reconciled = if ledger_unchanged(kernel_file_len) {
                streams.reconcile_actionables_main(session_id, &projection)
            } else {
                streams.reconcile_actionables(session_id, None)
            };
            match reconciled {
                Ok(()) => {}
                // 256 pinned sessions hold the live registrations: the request frame cannot be
                // announced, but the durable journal events below must still be (a client that
                // misses them rebaselines; O-final m10).
                Err(error @ ProductionCarrierError::ActiveSessionLimit(_)) => {
                    eprintln!("process-host-actionables-limit: session={session_id} {error}");
                }
                Err(error) => return Err(DaemonError::from(error)),
            }
        }
        for event in appended {
            if let Some(streams) = &streams {
                streams
                    .publish_durable_session_event_from_journal(session_id, &journal, event, None)
                    .map_err(DaemonError::from)?;
            }
        }
        if let Some(streams) = &streams {
            let refreshed = if ledger_unchanged(kernel_file_len) {
                streams.refresh_context_projection_shared(session_id, &projection, &journal)
            } else {
                streams.refresh_context_projection(session_id)
            };
            match refreshed {
                Ok(()) => {
                    self.context_projection_retries
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .remove(session_id);
                }
                Err(error) => {
                    self.context_projection_retries
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .insert(session_id.to_owned());
                    eprintln!(
                        "process-host-context-projection-failed: session={session_id} error={error}"
                    );
                }
            }
        }
        // The fold above is the verification a card and the catalog view need. The view is
        // refreshed on every clean fold (cheap: no ledger bytes); the card is only handed to the
        // writer when it is due for this thread (K6), otherwise the thread is marked dirty and
        // the writer re-folds it itself when its window expires. A torn tail is repaired first
        // and described by the next fold. The journal block is taken after the reconcile.
        if advanced_light {
            self.card_writer.mark_dirty(
                session_id,
                folder.clone(),
                crate::thread_card::journal_input(&journal),
            );
        } else if clean_read {
            let journal_snapshot = crate::thread_card::journal_input(&journal);
            match prepared {
                Some(card) => {
                    let card = card.with_journal(journal_snapshot);
                    if self.catalog.set_fresh_main(session_id, card.view()) {
                        if due {
                            self.card_writer.submit(card);
                        } else {
                            self.card_writer.mark_dirty(
                                session_id,
                                folder.clone(),
                                journal_snapshot,
                            );
                        }
                    }
                }
                None if !due => {
                    self.card_writer
                        .mark_dirty(session_id, folder.clone(), journal_snapshot);
                }
                None => {}
            }
        }
        let mut projection_cache = self
            .projection_cache
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match projection_cache.remove(session_id) {
            // A caller that folded a newer ledger finished first: keep its cache.
            Some(previous) if previous.kernel_file_len > kernel_file_len => {
                projection_cache.insert(session_id.to_owned(), previous);
            }
            previous => {
                projection_cache.insert(
                    session_id.to_owned(),
                    SessionProjectionCache {
                        kernel_file_len,
                        projector,
                        next_frames: previous.map(|cache| cache.next_frames).unwrap_or_default(),
                    },
                );
            }
        }
        drop(projection_cache);
        Ok(())
    }

    fn publish_frame(&self, session_id: &str, frame: Frame) -> Result<(), DaemonError> {
        let _projection_guard = self
            .projection_lock
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let streams = self
            .streams
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        let folder = self.root.join("threads").join(session_id);
        let ledger_path = folder.join("main.jsonl");
        let kernel_file_len = fs::metadata(&ledger_path).map_err(DaemonError::io)?.len();
        let mut projection_cache = self
            .projection_cache
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let cache_is_current = projection_cache.get(session_id).is_some_and(|cache| {
            cache.kernel_file_len == kernel_file_len
                && cache.next_frames.contains_key(&frame.attempt)
        });
        if !cache_is_current {
            let bytes = fs::read(&ledger_path).map_err(DaemonError::io)?;
            let projection = scan_valid_prefix(&bytes, schema::READER_VERSION)
                .projection
                .ok_or_else(|| DaemonError::corrupt("active thread has no projection"))?;
            let journal = EndpointJournal::open(&folder)
                .map_err(|error| DaemonError::corrupt(error.to_string()))?;
            // A control frame precedes its response's semantic output on the
            // worker pipe, even when the worker has already appended that output
            // to disk. Do not let this frame's cache refresh publish future facts.
            let journaled_through = journal
                .records()
                .map_err(|error| DaemonError::corrupt(error.to_string()))?
                .iter()
                .flat_map(|record| record.kernel_seqs.iter().copied())
                .max()
                .unwrap_or(0);
            let prefix = if let Some(ledger_seq) = frame.ledger_seq {
                // Exact causal cut: the worker stamped the ledger high-water it
                // had when it wrote this frame. Events past it (an eagerly
                // dispatched tool_call/tool_result, the terminal usage/output)
                // were appended after the frame left the pipe, so they wait for
                // their own doorbell. Never fall behind what the journal already
                // holds: a projector that does not know an attempt's published
                // terminal output would reopen it for a late frame.
                let cut = ledger_seq.max(journaled_through);
                &projection.events[..projection
                    .events
                    .partition_point(|event| event.seq() <= cut)]
            } else {
                let boundary = projection
                    .events
                    .iter()
                    .position(|event| {
                        event.string_field("attempt") == Some(frame.attempt.as_str())
                            && matches!(
                                event.kind(),
                                EventKind::ToolCall | EventKind::Output | EventKind::Error
                            )
                    })
                    .unwrap_or(projection.events.len());
                let prefix = &projection.events[..boundary];
                let through = prefix.last().map_or(0, Event::seq);
                if journaled_through > through {
                    // A separately published semantic event already won. Never
                    // reopen its sealed output or rewrite historical provenance.
                    return Ok(());
                }
                prefix
            };
            let mut projector = Projector::default();
            let projected = projector
                .reconcile(prefix, &journal)
                .map_err(|error| DaemonError::corrupt(error.to_string()))?;
            for event in projected {
                if let Some(streams) = &streams {
                    streams
                        .publish_durable_session_event_from_journal(
                            session_id, &journal, event, None,
                        )
                        .map_err(DaemonError::from)?;
                }
            }
            let next_frames = projection_cache
                .remove(session_id)
                .map(|previous| previous.next_frames)
                .unwrap_or_default();
            projection_cache.insert(
                session_id.to_owned(),
                SessionProjectionCache {
                    kernel_file_len,
                    projector,
                    next_frames,
                },
            );
        }
        let cache = projection_cache
            .get_mut(session_id)
            .expect("current projection cache was installed");
        // Chunks are transient: the frame ordinal is a per-attempt
        // presentation version kept in memory for the life of the cache.
        let next_frame = cache.next_frames.get(&frame.attempt).copied().unwrap_or(0);
        cache
            .next_frames
            .insert(frame.attempt.clone(), next_frame.saturating_add(1));
        let time = session_event_time_now()?;
        let accepted = AcceptedStreamFrame {
            arguments_complete: frame.arguments_complete,
            attempt: frame.attempt,
            frame: next_frame,
            channel: match frame.channel {
                FrameChannel::Text => "text",
                FrameChannel::Reasoning => "reasoning",
                FrameChannel::Tool => "tool",
                FrameChannel::Usage => "usage",
            }
            .to_owned(),
            block: frame.block,
            delta: frame.delta,
            call_id: frame.call_id,
            name: frame.name,
            time,
        };
        let transient = match cache.projector.stream_event(&accepted) {
            Ok(event) => event,
            // The durable full output may overtake control-pipe presentation
            // frames. It is authoritative; late deltas cannot reopen it.
            Err(endpoint::ProjectionError::TerminalAttempt(_)) => return Ok(()),
            Err(error) => return Err(DaemonError::corrupt(error.to_string())),
        };
        if let Some(streams) = &streams {
            streams
                .publish_session_frame(
                    session_id,
                    endpoint::MuxFrame::Transient {
                        session_id: session_id.to_owned(),
                        event: transient.clone(),
                    },
                )
                .map_err(DaemonError::from)?;
        }
        Ok(())
    }

    fn launch_child(
        &self,
        session_id: &str,
        parent_process_key: &str,
        parent_ledger: &Path,
        request: &LaunchChild,
    ) -> Result<LaunchResult, DaemonError> {
        let folder = self.root.join("threads").join(session_id);
        let mut ledgers = fs::read_dir(&folder)
            .map_err(DaemonError::io)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(DaemonError::io)?;
        ledgers.sort_by_key(fs::DirEntry::file_name);
        for ledger in ledgers {
            let path = ledger.path();
            if !ledger.file_type().map_err(DaemonError::io)?.is_file()
                || path.extension().and_then(|value| value.to_str()) != Some("jsonl")
                || path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .is_some_and(|name| name == endpoint::JOURNAL_FILE)
            {
                continue;
            }
            let bytes = fs::read(&path).map_err(DaemonError::io)?;
            let projection = scan_valid_prefix(&bytes, schema::READER_VERSION)
                .projection
                .ok_or_else(|| DaemonError::corrupt("child line has no projection"))?;
            let Some(genesis) = projection.events.first() else {
                continue;
            };
            if genesis.string_field("thread") != Some(request.child.as_str()) {
                continue;
            }
            let genesis_value = serde_json::to_value(genesis.raw())
                .map_err(|error| DaemonError::protocol(error.to_string()))?;
            if genesis_value
                .get("parent")
                .and_then(|parent| parent.get("spawn_id"))
                .and_then(serde_json::Value::as_str)
                != Some(request.spawn_id.as_str())
            {
                return Ok(LaunchResult {
                    child: request.child.clone(),
                    spawn_id: request.spawn_id.clone(),
                    ok: false,
                    error: Some("spawn_mismatch".to_owned()),
                });
            }
            let key = format!("{session_id}:{}", request.child);
            self.propagate_parent_stop(parent_ledger, &path, &key, &request.child)?;
            let bytes = fs::read(&path).map_err(DaemonError::io)?;
            let projection = scan_valid_prefix(&bytes, schema::READER_VERSION)
                .projection
                .ok_or_else(|| DaemonError::corrupt("child line has no projection"))?;
            let facts = projection.lifecycle;
            let owned_live = self
                .workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .get(&key)
                .is_some_and(|worker| worker.alive.load(Ordering::Acquire));
            let lock_facts = if owned_live || probe_line_lock(&path)? == LineLockState::Busy {
                LockFacts::OTHER
            } else {
                LockFacts::FREE
            };
            if ensure_action_at(classify(&facts, lock_facts), &facts, Some(&now_rfc3339()))
                != EnsureAction::None
            {
                self.schedule_child_from_parent(parent_process_key, &key, session_id, path)?;
            }
            return Ok(LaunchResult {
                child: request.child.clone(),
                spawn_id: request.spawn_id.clone(),
                ok: true,
                error: None,
            });
        }
        Ok(LaunchResult {
            child: request.child.clone(),
            spawn_id: request.spawn_id.clone(),
            ok: false,
            error: Some("missing".to_owned()),
        })
    }

    fn propagate_parent_stop(
        &self,
        parent_ledger: &Path,
        child_ledger: &Path,
        child_key: &str,
        child_line: &str,
    ) -> Result<(), DaemonError> {
        let parent_bytes = fs::read(parent_ledger).map_err(DaemonError::io)?;
        let parent = scan_valid_prefix(&parent_bytes, schema::READER_VERSION)
            .projection
            .ok_or_else(|| DaemonError::corrupt("parent line has no projection"))?;
        if !parent.lifecycle.stop_active {
            return Ok(());
        }
        let stop = parent
            .events
            .iter()
            .rev()
            .find(|event| event.kind() == &schema::EventKind::StopRequested)
            .ok_or_else(|| DaemonError::corrupt("active parent stop has no stop_requested"))?;
        let generation = stop
            .integer_field("generation")
            .ok_or_else(|| DaemonError::corrupt("parent stop has no generation"))?;
        let value = serde_json::to_value(stop.raw())
            .map_err(|error| DaemonError::protocol(error.to_string()))?;
        let origin: OriginTuple = serde_json::from_value(
            value
                .get("origin_tuple")
                .cloned()
                .ok_or_else(|| DaemonError::corrupt("parent stop has no origin tuple"))?,
        )
        .map_err(|error| DaemonError::protocol(error.to_string()))?;

        let child_bytes = fs::read(child_ledger).map_err(DaemonError::io)?;
        let child = scan_valid_prefix(&child_bytes, schema::READER_VERSION)
            .projection
            .ok_or_else(|| DaemonError::corrupt("child line has no projection"))?;
        let child_generation = child
            .events
            .iter()
            .filter_map(|event| {
                (event.kind() == &schema::EventKind::StopRequested)
                    .then(|| event.integer_field("generation"))
                    .flatten()
            })
            .max()
            .unwrap_or(0);
        if child_generation >= generation {
            return Ok(());
        }
        if let Some(worker) = self
            .workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(child_key)
            .filter(|worker| worker.alive.load(Ordering::Acquire))
            .cloned()
        {
            let delivery = format!("stop-propagation-{generation}-{child_line}");
            worker.write(&encode_line(
                "stop",
                &Stop {
                    delivery: delivery.clone(),
                    origin,
                    generation,
                },
            )?)?;
            worker
                .receipt(&delivery)
                .map_err(|error| DaemonError::protocol(error.message))?;
            return Ok(());
        }
        let mut ledger =
            LockedLedger::open(child_ledger, schema::READER_VERSION).map_err(DaemonError::store)?;
        let event = schema::Event::from_value(
            IJsonValue::parse(
                &serde_json::to_vec(&serde_json::json!({
                    "v":1,
                    "seq":ledger.next_seq(),
                    "ts":crate::host_runtime::system_timestamp().map_err(DaemonError::protocol)?,
                    "kind":"stop_requested",
                    "generation":generation,
                    "origin_key":origin.key,
                    "origin_tuple":origin,
                }))
                .map_err(|error| DaemonError::protocol(error.to_string()))?,
            )
            .map_err(|error| DaemonError::protocol(error.to_string()))?,
        )
        .map_err(|error| DaemonError::protocol(error.to_string()))?;
        ledger.append(event, true).map_err(DaemonError::store)
    }

    fn cascade_stop_from(
        &self,
        session_id: &str,
        parent_process_key: &str,
        parent_ledger: &Path,
        visited: &mut HashSet<PathBuf>,
    ) -> Result<(), DaemonError> {
        let canonical_parent = parent_ledger.canonicalize().map_err(DaemonError::io)?;
        if !visited.insert(canonical_parent) {
            return Err(DaemonError::corrupt("spawn graph contains a cycle"));
        }
        let folder = parent_ledger
            .parent()
            .ok_or_else(|| DaemonError::corrupt("parent ledger has no folder"))?;
        let bytes = fs::read(parent_ledger).map_err(DaemonError::io)?;
        let projection = scan_valid_prefix(&bytes, schema::READER_VERSION)
            .projection
            .ok_or_else(|| DaemonError::corrupt("parent line has no projection"))?;
        let completed_calls = projection
            .events
            .iter()
            .filter(|event| event.kind() == &EventKind::ChildResult)
            .filter_map(|event| event.string_field("call"))
            .collect::<HashSet<_>>();
        let mut children = projection
            .events
            .iter()
            .filter(|event| event.kind() == &EventKind::Spawn)
            .filter_map(|event| {
                let call = event.string_field("call")?;
                (!completed_calls.contains(call))
                    .then(|| event.string_field("child").map(str::to_owned))
                    .flatten()
            })
            .collect::<Vec<_>>();
        children.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        children.dedup();
        for child_file in children {
            let child_ledger = folder.join(&child_file);
            if child_ledger.parent() != Some(folder)
                || child_ledger.extension().and_then(|v| v.to_str()) != Some("jsonl")
            {
                return Err(DaemonError::corrupt("spawn child path is invalid"));
            }
            let child_bytes = fs::read(&child_ledger).map_err(DaemonError::io)?;
            let child_projection = scan_valid_prefix(&child_bytes, schema::READER_VERSION)
                .projection
                .ok_or_else(|| DaemonError::corrupt("child line has no projection"))?;
            let child_line = child_projection
                .events
                .first()
                .and_then(|event| event.string_field("thread"))
                .ok_or_else(|| DaemonError::corrupt("child genesis has no thread"))?;
            let (child_key, checked_ledger) =
                line_schedule_target(&self.root, session_id, child_line, &child_file)?;
            if checked_ledger != child_ledger {
                return Err(DaemonError::corrupt(
                    "spawn child path does not match its genesis",
                ));
            }
            self.propagate_parent_stop(parent_ledger, &child_ledger, &child_key, child_line)?;
            self.cascade_stop_from(session_id, &child_key, &child_ledger, visited)?;
            if let Some(worker) = self
                .workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .get(&child_key)
                .cloned()
            {
                terminate_worker(&worker);
            } else {
                self.schedule_child_from_parent(
                    parent_process_key,
                    &child_key,
                    session_id,
                    child_ledger,
                )?;
            }
        }
        Ok(())
    }

    /// `failure` is the worker's recorded failure, `None` for a clean exit. A
    /// clean exit whose tail still asks for a run (an answered hold, a queued
    /// input, a due wait) is ordinary work and never counts toward the
    /// crash-loop backoff; only recorded failures do.
    fn reconcile_after_exit(
        &self,
        process_key: &str,
        session_id: &str,
        ledger: &Path,
        failure: Option<&[u8]>,
    ) {
        if self.draining.load(Ordering::Acquire) {
            return;
        }
        let result = (|| {
            if process_key == session_id {
                self.publish_appended(session_id)?;
            }
            self.refresh_limits(ledger)?;
            let bytes = fs::read(ledger).map_err(DaemonError::io)?;
            let facts = scan_valid_prefix(&bytes, schema::READER_VERSION)
                .projection
                .ok_or_else(|| DaemonError::corrupt("worker ledger has no projection"))?
                .lifecycle;
            let lock_facts = if probe_line_lock(ledger)? == LineLockState::Busy {
                LockFacts::OTHER
            } else {
                LockFacts::FREE
            };
            let state = classify(&facts, lock_facts);
            if ensure_action_at(state, &facts, Some(&now_rfc3339())) == EnsureAction::None
                && !(process_key == session_id && goal_continuation_due(ledger)?)
            {
                self.reset_restart_backoff(process_key);
                return Ok(());
            }
            match failure {
                Some(failure) => {
                    let Some(delay) = self.note_restart_failure(process_key, session_id, failure)
                    else {
                        return Ok(());
                    };
                    std::thread::sleep(delay);
                    if !self.restart_is_due(process_key) {
                        return Ok(());
                    }
                }
                None => self.reset_restart_backoff(process_key),
            }
            match self.schedule_worker_at(process_key, session_id, ledger.to_owned()) {
                Ok(Some(_)) => {}
                Ok(None) => {}
                Err(error) => {
                    self.note_restart_failure(
                        process_key,
                        session_id,
                        error.bootstrap_code().as_bytes(),
                    );
                }
            }
            Ok::<_, DaemonError>(())
        })();
        if let Err(error) = result {
            eprintln!("process-host-reconcile-failed: {error}");
        }
    }

    fn reset_restart_backoff(&self, process_key: &str) {
        self.restart_failures
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .remove(process_key);
    }

    fn restart_is_due(&self, process_key: &str) -> bool {
        self.restart_failures
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(process_key)
            .is_none_or(|backoff| !backoff.exhausted && Instant::now() >= backoff.next_ensure_at)
    }

    fn note_restart_failure(
        &self,
        process_key: &str,
        session_id: &str,
        reason: &[u8],
    ) -> Option<Duration> {
        let reason_digest: [u8; 32] = Sha256::digest(reason).into();
        let mut failures = self
            .restart_failures
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let now = Instant::now();
        let backoff = failures
            .entry(process_key.to_owned())
            .or_insert(RestartBackoff {
                reason_digest,
                failures: 0,
                next_ensure_at: now,
                exhausted: false,
            });
        if backoff.reason_digest != reason_digest {
            backoff.reason_digest = reason_digest;
            backoff.failures = 0;
            backoff.exhausted = false;
        }
        backoff.failures = backoff.failures.saturating_add(1);
        if backoff.failures >= 5 {
            backoff.exhausted = true;
            eprintln!("process-host-restart-limit: session={session_id}");
            return None;
        }
        let seconds = (1_u64 << backoff.failures.saturating_sub(1).min(4)).min(30);
        let delay = Duration::from_secs(seconds);
        backoff.next_ensure_at = now + delay;
        Some(delay)
    }

    /// A durable input whose worker could not start: the ledger already holds
    /// the input, so the RPC stays accepted, but the session must show that
    /// nothing will run it. The notice is keyed by the input's seq so every
    /// stranded input gets its own line and a retry of the same RPC does not.
    fn record_spawn_failure(
        &self,
        operation: &str,
        session_id: &str,
        input_seq: u64,
        error: &DaemonError,
    ) {
        eprintln!(
            "process-host-spawn-failed: operation={operation} session={session_id} error={error}"
        );
        self.record_session_notice(
            session_id,
            &format!("input-{input_seq}"),
            SessionNotice {
                severity: SessionNoticeSeverity::Error,
                classification: "worker_launch".to_owned(),
                operation: operation.to_owned(),
                message: format!("The worker could not be started: {error}"),
            },
        );
    }

    /// Appends one session notice under the supervisor's host origin and
    /// publishes it to the live streams. `key_suffix` scopes idempotency: the
    /// same suffix and session fold repeated appends into the original seq.
    fn record_session_notice(&self, session_id: &str, key_suffix: &str, notice: SessionNotice) {
        let timestamp = match crate::host_runtime::system_timestamp() {
            Ok(timestamp) => timestamp,
            Err(error) => {
                eprintln!("process-host-session-notice-failed: session={session_id} error={error}");
                return;
            }
        };
        let origin = OriginTuple {
            principal: "host".to_owned(),
            client: "tekes-supervisor".to_owned(),
            target: session_id.to_owned(),
            op: SESSION_NOTICE_OPERATION.to_owned(),
            key: format!("{SESSION_NOTICE_OPERATION}:{session_id}:{key_suffix}"),
        };
        match self
            .endpoint
            .record_session_notice(session_id, &timestamp, &origin, &notice)
        {
            Ok(Some(receipt)) if !receipt.deduplicated => {
                if let Err(error) = self.publish_appended(session_id) {
                    eprintln!(
                        "process-host-session-notice-publish-failed: session={session_id} error={error}"
                    );
                }
            }
            Ok(_) => {}
            Err(error) => {
                eprintln!("process-host-session-notice-failed: session={session_id} error={error}");
            }
        }
    }
}

/// The notices a degraded MCP launch owes the session: one for an unreadable
/// registry (no registry-configured server joined the catalog) and one per
/// server that could not be prepared. Nothing when the catalog is complete.
/// Most refused tools one `mcp_tool` notice names.
const MAX_NOTICE_TOOLS: usize = 8;

fn mcp_launch_notices(
    registry_failure: Option<&str>,
    failures: &[crate::mcp_runtime::McpServerFailure],
    tool_refusals: &[crate::mcp_runtime::McpRefusedTool],
    allowed_tools: &[String],
) -> Vec<SessionNotice> {
    let mut notices = Vec::new();
    if let Some(message) = registry_failure {
        notices.push(SessionNotice {
            severity: SessionNoticeSeverity::Warning,
            classification: "mcp_registry".to_owned(),
            operation: "worker-launch".to_owned(),
            message: format!("MCP servers were skipped for this run: {message}"),
        });
    }
    for failure in failures {
        let reason = failure.detail.as_deref().unwrap_or(failure.code);
        notices.push(SessionNotice {
            severity: SessionNoticeSeverity::Warning,
            classification: "mcp_server".to_owned(),
            operation: "worker-launch".to_owned(),
            message: format!(
                "MCP server {} was skipped for this run: {reason}",
                failure.server.name
            ),
        });
    }
    // One notice per healthy server that skipped tools: the server stays
    // available, the notice names each skipped tool, where in its schema and
    // why. At most `MAX_NOTICE_TOOLS` are listed, those the workspace policy
    // names first; the rest is counted.
    let mut servers = BTreeMap::<&mcp::McpServerReference, Vec<&mcp::McpToolRefusal>>::new();
    for refused in tool_refusals {
        servers
            .entry(&refused.server)
            .or_default()
            .push(&refused.refusal);
    }
    for (server, mut refusals) in servers {
        refusals.sort_by_key(|refusal| {
            !refusal
                .name
                .as_ref()
                .is_some_and(|name| allowed_tools.contains(name))
        });
        let listed = refusals
            .iter()
            .take(MAX_NOTICE_TOOLS)
            .map(|refusal| {
                let location = refusal
                    .pointer
                    .as_deref()
                    .map(|pointer| format!(" at {pointer}"))
                    .unwrap_or_default();
                format!("{}{location}: {}", refusal.tool, refusal.reason)
            })
            .collect::<Vec<_>>()
            .join("; ");
        let omitted = refusals.len().saturating_sub(MAX_NOTICE_TOOLS);
        let tail = if omitted == 0 {
            String::new()
        } else {
            format!("; and {omitted} more")
        };
        notices.push(SessionNotice {
            severity: SessionNoticeSeverity::Warning,
            classification: "mcp_tool".to_owned(),
            operation: "worker-launch".to_owned(),
            message: format!(
                "MCP server {} is available but {} of its tools were skipped for this run: {listed}{tail}",
                server.name,
                refusals.len()
            ),
        });
    }
    notices
}

fn frozen_tool_launch_policy(
    config: &ConfigSnapshot,
    instruction: &profile::InstructionSnapshot,
) -> Result<tools::JobLaunchPolicy, DaemonError> {
    let effective = instruction.meet_workspace_policy(&config.workspace.policy);
    let execution_cwd = config
        .execution_cwd()
        .map(str::to_owned)
        .ok_or_else(|| DaemonError::invalid_config("workspace has no execution cwd"))?;
    let sandbox = tools::SandboxPolicy {
        format: 1,
        read_roots: config.workspace.cwd.clone(),
        write_roots: Vec::new(),
        network: if effective.network {
            tools::NetworkPolicy::All
        } else {
            tools::NetworkPolicy::Deny
        },
        allow_process: true,
        scratch: None,
    };
    tools::JobLaunchPolicy::new(execution_cwd, sandbox, effective.writable_roots)
        .map_err(|error| DaemonError::required_broker(error.to_string()))
}

fn validator_tool_launch_policy(
    ledger: &Path,
    config: &ConfigSnapshot,
    instruction: &profile::InstructionSnapshot,
) -> Result<Option<tools::JobLaunchPolicy>, DaemonError> {
    let bytes = fs::read(ledger).map_err(DaemonError::io)?;
    let projection = scan_valid_prefix(&bytes, schema::READER_VERSION)
        .projection
        .ok_or_else(|| DaemonError::corrupt("validator ledger has no projection"))?;
    if !projection
        .events
        .first()
        .is_some_and(|genesis| genesis.has_field("validator_for"))
    {
        return Ok(None);
    }
    let folder = ledger
        .parent()
        .ok_or_else(|| DaemonError::protocol("validator ledger has no folder"))?
        .canonicalize()
        .map_err(DaemonError::io)?;
    let child = ledger
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| DaemonError::protocol("validator ledger has no identity"))?;
    let private = folder.join("validator-workspaces").join(child);
    let scratch = private
        .join("scratch")
        .canonicalize()
        .map_err(DaemonError::io)?;
    let snapshot = private
        .join("snapshot")
        .canonicalize()
        .map_err(DaemonError::io)?;
    if scratch != private.join("scratch") || snapshot != private.join("snapshot") {
        return Err(DaemonError::required_broker(
            "validator private workspace contains a symlink",
        ));
    }
    private_validator_launch_policy(&scratch, &snapshot, config, instruction).map(Some)
}

fn private_validator_launch_policy(
    scratch: &Path,
    snapshot: &Path,
    config: &ConfigSnapshot,
    instruction: &profile::InstructionSnapshot,
) -> Result<tools::JobLaunchPolicy, DaemonError> {
    let scratch = scratch.to_string_lossy().into_owned();
    let mut read_roots = vec![scratch.clone(), snapshot.to_string_lossy().into_owned()];
    read_roots.extend(config.workspace.cwd.iter().cloned());
    read_roots.sort();
    read_roots.dedup();
    let effective = instruction.meet_workspace_policy(&config.workspace.policy);
    let sandbox = tools::SandboxPolicy {
        format: 1,
        read_roots,
        write_roots: Vec::new(),
        network: if effective.network {
            tools::NetworkPolicy::All
        } else {
            tools::NetworkPolicy::Deny
        },
        allow_process: true,
        scratch: None,
    };
    tools::JobLaunchPolicy::new(scratch.clone(), sandbox, vec![scratch])
        .map_err(|error| DaemonError::required_broker(error.to_string()))
}

fn web_search_scope_ready(
    config: &ConfigSnapshot,
    bindings: &provider::ResolvedCredentialBindings,
) -> bool {
    let Some(search) = &config.providers.web_search else {
        return false;
    };
    let Ok(origin) = provider::endpoint_origin(&search.endpoint) else {
        return false;
    };
    bindings.active.iter().any(|scope| {
        scope.credential_id == search.credential_key
            && scope.adapter == search.adapter
            && scope.endpoint_origin == origin
            && scope.purpose == "web_search"
    })
}

fn mcp_failure_is_required(
    failures: &[crate::mcp_runtime::McpServerFailure],
    allowed_tools: &[String],
) -> bool {
    failures.iter().any(|failure| {
        let marker = mcp::project_name(&failure.server.name, "required_marker")
            .expect("validated MCP server names are nonempty");
        let prefix = marker
            .strip_suffix("required_marker")
            .expect("static marker remains unescaped");
        allowed_tools.iter().any(|tool| tool.starts_with(prefix))
    })
}

/// The reason text for a policy that names an unavailable MCP server: which
/// servers, and each one's stable failure code.
fn required_mcp_failure_reason(
    failures: &[crate::mcp_runtime::McpServerFailure],
    allowed_tools: &[String],
) -> String {
    let required = failures
        .iter()
        .filter(|failure| mcp_failure_is_required(std::slice::from_ref(failure), allowed_tools))
        .map(|failure| format!("{} ({})", failure.server.name, failure.code))
        .collect::<Vec<_>>();
    format!(
        "workspace policy requires an unavailable MCP server: {}",
        bounded_name_list(required.iter().map(String::as_str))
    )
}

/// At most eight comma-separated names, each cut to 128 bytes, then a count.
fn bounded_name_list<'a>(names: impl Iterator<Item = &'a str>) -> String {
    const LISTED: usize = 8;
    let names = names.collect::<Vec<_>>();
    let mut text = names
        .iter()
        .take(LISTED)
        .map(|name| {
            let mut end = name.len().min(128);
            while !name.is_char_boundary(end) {
                end -= 1;
            }
            name[..end].to_owned()
        })
        .collect::<Vec<_>>()
        .join(", ");
    if names.len() > LISTED {
        text.push_str(&format!(" and {} more", names.len() - LISTED));
    }
    text
}

/// Why `validate_workspace_policy_candidate` refused a policy. The cause is
/// stable and machine-readable; the reason is the precise human sentence.
#[derive(Debug)]
pub struct PolicyCandidateError {
    pub cause: PolicyCandidateCause,
    reason: String,
}

impl PolicyCandidateError {
    fn new(cause: PolicyCandidateCause, reason: impl ToString) -> Self {
        Self {
            cause,
            reason: reason.to_string(),
        }
    }
}

impl std::fmt::Display for PolicyCandidateError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.reason)
    }
}

impl std::error::Error for PolicyCandidateError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyCandidateCause {
    /// The policy exceeds a ceiling: the immutable instruction ceiling, or a
    /// limit the resolved configuration enforces (a writable root outside
    /// every workspace folder).
    AuthorityCeiling,
    /// The policy names a tool that is in no installed catalog.
    ToolNotInstalled,
    /// The policy names a tool of an MCP server that is not available.
    McpServerUnavailable,
    /// The catalogs needed to judge the policy could not be assembled.
    CatalogUnavailable,
}

impl PolicyCandidateCause {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::AuthorityCeiling => "authority-ceiling",
            Self::ToolNotInstalled => "tool-not-installed",
            Self::McpServerUnavailable => "mcp-server-unavailable",
            Self::CatalogUnavailable => "catalog-unavailable",
        }
    }
}

const fn child_dependency_admitted(ordinary_capacity: bool, parent_is_live_waiter: bool) -> bool {
    ordinary_capacity || parent_is_live_waiter
}

impl WorkerHandle {
    fn write(&self, bytes: &[u8]) -> Result<(), DaemonError> {
        let mut stdin = self
            .stdin
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        stdin.write_all(bytes).map_err(DaemonError::io)?;
        stdin.flush().map_err(DaemonError::io)
    }

    fn receipt(&self, delivery: &str) -> Result<Receipt, ProductionRouteFailure> {
        let deadline = Instant::now() + DELIVERY_TIMEOUT;
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        loop {
            if let Some(receipt) = state.receipts.remove(delivery) {
                return Ok(receipt);
            }
            if let Some(failure) = state.failure.as_ref() {
                return Err(internal(failure));
            }
            let now = Instant::now();
            if now >= deadline {
                return Err(internal("worker receipt timed out"));
            }
            let (next, _) = self
                .changed
                .wait_timeout(state, deadline - now)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            state = next;
        }
    }

    fn queue_result(
        &self,
        delivery: &str,
    ) -> Result<QueueTransactionResult, ProductionRouteFailure> {
        let deadline = Instant::now() + DELIVERY_TIMEOUT;
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        loop {
            if let Some(result) = state.queue_results.remove(delivery) {
                return Ok(result);
            }
            if let Some(failure) = state.failure.as_ref() {
                return Err(internal(failure));
            }
            let now = Instant::now();
            if now >= deadline {
                return Err(internal("worker queue transaction timed out"));
            }
            let (next, _) = self
                .changed
                .wait_timeout(state, deadline - now)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            state = next;
        }
    }
}

fn worker_reader(
    mut reader: BufReader<impl std::io::Read>,
    handle: Arc<WorkerHandle>,
    owner: Weak<ProductionProcessHost>,
    session_id: String,
    process_key: String,
    ledger: PathBuf,
) {
    let mut line = String::new();
    let mut reader_failed = false;
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(error) => {
                fail_worker(&handle, error.to_string());
                reader_failed = true;
                break;
            }
        }
        match decode_worker(line.as_bytes()) {
            Ok(WorkerMessage::Receipt(receipt)) => {
                handle
                    .state
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .receipts
                    .insert(receipt.delivery.clone(), receipt);
                handle.changed.notify_all();
            }
            Ok(WorkerMessage::QueueTransactionResult(result)) => {
                handle
                    .state
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .queue_results
                    .insert(result.delivery.clone(), result);
                handle.changed.notify_all();
            }
            Ok(WorkerMessage::LeaseRequest(request)) => {
                let granted = owner.upgrade().is_some_and(|owner| {
                    owner.request_provider_lease(AdmissionLease {
                        attempt: request.attempt.clone(),
                        class: request.class,
                    })
                });
                if granted {
                    handle
                        .state
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .held_attempts
                        .insert(request.attempt.clone());
                }
                let _ = handle.write(
                    &encode_line(
                        "lease",
                        &Lease {
                            attempt: request.attempt,
                            granted,
                        },
                    )
                    .unwrap_or_default(),
                );
            }
            Ok(WorkerMessage::AttemptSettled(settled)) => {
                if let Some(owner) = owner.upgrade() {
                    handle
                        .state
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .held_attempts
                        .remove(&settled.attempt);
                    owner
                        .admission
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .settle(&settled.attempt);
                    owner.admission_changed.notify_all();
                }
            }
            Ok(WorkerMessage::Appended(_)) => {
                if process_key != session_id
                    && let Some(owner) = owner.upgrade()
                {
                    owner.reconcile_child_line(&session_id, &ledger);
                }
                if process_key == session_id
                    && let Some(owner) = owner.upgrade()
                    && let Err(error) = owner.publish_appended(&session_id)
                {
                    eprintln!("process-host-publish-appended-failed: {error}");
                }
            }
            Ok(WorkerMessage::Frame(frame)) => {
                if process_key == session_id
                    && let Some(owner) = owner.upgrade()
                    && let Err(error) = owner.publish_frame(&session_id, frame)
                {
                    eprintln!("process-host-publish-frame-failed: {error}");
                }
            }
            Ok(WorkerMessage::LaunchChild(child)) => {
                let result = owner
                    .upgrade()
                    .ok_or_else(|| DaemonError::protocol("process host was dropped"))
                    .and_then(|owner| {
                        owner.launch_child(&session_id, &process_key, &ledger, &child)
                    });
                let result = match result {
                    Ok(result) => result,
                    Err(error) => LaunchResult {
                        child: child.child,
                        spawn_id: child.spawn_id,
                        ok: false,
                        error: Some(error.bootstrap_code().to_owned()),
                    },
                };
                match encode_line("launch_result", &result).and_then(|bytes| {
                    handle.write(&bytes).map_err(|error| {
                        worker_control::ProtocolError::InvalidDurableControl(error.to_string())
                    })
                }) {
                    Ok(()) => {}
                    Err(error) => {
                        fail_worker(&handle, error.to_string());
                        reader_failed = true;
                        break;
                    }
                }
            }
            Ok(WorkerMessage::Unknown { .. }) => {
                let is_control = decode_tool_control(line.as_bytes()).is_ok();
                let is_continuation = !is_control
                    && worker_control::continuation::decode_tool_continuation(line.as_bytes())
                        .is_ok();
                if is_control || is_continuation {
                    let result = owner
                        .upgrade()
                        .ok_or_else(|| DaemonError::protocol("process host was dropped"))
                        .and_then(|owner| {
                            if is_control {
                                owner.handle_tool_control(&handle, line.as_bytes())
                            } else {
                                owner.handle_tool_continuation(&handle, line.as_bytes())
                            }
                        });
                    match result.and_then(|bytes| handle.write(&bytes)) {
                        Ok(()) => {}
                        Err(error) => {
                            fail_worker(&handle, error.to_string());
                            reader_failed = true;
                            break;
                        }
                    }
                }
            }
            Ok(WorkerMessage::State(_) | WorkerMessage::Pong(_)) => {}
            Err(error) => {
                fail_worker(&handle, error.to_string());
                reader_failed = true;
                break;
            }
        }
    }
    handle.alive.store(false, Ordering::Release);
    fail_worker(&handle, "worker exited".to_owned());
    if reader_failed {
        terminate_worker(&handle);
    }
    let exit_status = handle
        .child
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .wait();
    // A clean exit: the pipe closed without a protocol failure and the
    // process reported success. Anything else is a recorded failure for the
    // restart backoff.
    let exited_cleanly = !reader_failed
        && exit_status
            .as_ref()
            .is_ok_and(std::process::ExitStatus::success);
    drop(
        handle
            .credential_control
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take(),
    );
    if let Some(join) = handle
        .credential_broker
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .take()
    {
        let _ = join.join();
    }
    if let Some(owner) = owner.upgrade() {
        let held_attempts = std::mem::take(
            &mut handle
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .held_attempts,
        );
        owner
            .admission
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .reap(held_attempts);
        owner.admission_changed.notify_all();
        let mut workers = owner
            .workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if workers
            .get(&process_key)
            .is_some_and(|registered| Arc::ptr_eq(registered, &handle))
        {
            workers.remove(&process_key);
        }
        drop(workers);
        owner.workers_changed.notify_all();
        owner.publish_session_status(&session_id);
        // A child worker that died between a durable append and its `Appended` message left a
        // request nobody announced: look at its line once more (O-final n1).
        if process_key != session_id {
            owner.reconcile_child_line(&session_id, &ledger);
        }
        owner.start_pending_workers();
        if !owner.draining.load(Ordering::Acquire) {
            let failure = if exited_cleanly {
                None
            } else {
                Some(
                    handle
                        .state
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .failure
                        .clone()
                        .unwrap_or_else(|| "worker-exit".to_owned()),
                )
            };
            owner.reconcile_after_exit(
                &process_key,
                &session_id,
                &ledger,
                failure.as_deref().map(str::as_bytes),
            );
        }
    }
}

fn terminate_worker(handle: &WorkerHandle) {
    handle.tool_cancellation.cancel();
    if let Some(authority) = handle.dynamic_authority.as_ref() {
        authority.cancel_inflight();
    }
    let mut child = handle
        .child
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let pid = child.id() as libc::pid_t;
    // SAFETY: pid came from this live child; SIGTERM carries no pointers.
    let _ = unsafe { libc::kill(pid, libc::SIGTERM) };
    let deadline = Instant::now() + Duration::from_secs(1);
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return,
            Ok(None) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(10));
            }
            _ => {
                let _ = child.kill();
                return;
            }
        }
    }
}

fn fail_worker(handle: &WorkerHandle, message: String) {
    handle.tool_cancellation.cancel();
    if let Some(authority) = handle.dynamic_authority.as_ref() {
        authority.cancel_inflight();
    }
    handle.alive.store(false, Ordering::Release);
    let mut state = handle
        .state
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let stderr = handle
        .stderr_tail
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let message = if stderr.is_empty() {
        message
    } else {
        format!("{message}; stderr: {}", String::from_utf8_lossy(&stderr))
    };
    preserve_first_failure(&mut state.failure, message);
    drop(state);
    handle.changed.notify_all();
}

fn worker_stderr_reader(mut reader: BufReader<impl std::io::Read>, handle: Arc<WorkerHandle>) {
    const MAX_STDERR_TAIL: usize = 4 * 1024;
    let mut chunk = [0_u8; 1024];
    loop {
        let count = match reader.read(&mut chunk) {
            Ok(0) | Err(_) => return,
            Ok(count) => count,
        };
        let mut tail = handle
            .stderr_tail
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        tail.extend_from_slice(&chunk[..count]);
        if tail.len() > MAX_STDERR_TAIL {
            let remove = tail.len() - MAX_STDERR_TAIL;
            tail.drain(..remove);
        }
    }
}

fn preserve_first_failure(failure: &mut Option<String>, message: String) {
    if failure.is_none() {
        *failure = Some(message);
    }
}

impl SessionDeliveryAuthority for ProductionProcessHost {
    fn session_metadata_changed(&self, session_id: &str) {
        self.publish_session_status(session_id);
    }

    fn prompt(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        prompt: &MaterializedPrompt,
        steer: bool,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        self.reset_restart_backoff(session_id);
        if let Some(worker) = self.live_worker(session_id) {
            let delivery = origin.key.clone();
            let delivered = worker
                .write(
                    &encode_line(
                        "input",
                        &Input {
                            delivery: delivery.clone(),
                            origin: origin.clone(),
                            content: prompt.blocks.clone(),
                            submission: Some(
                                self.endpoint
                                    .submission_snapshot(session_id)
                                    .map_err(internal)?,
                            ),
                            steer: Some(steer),
                            assets: (!prompt.attachments.is_empty()).then(|| {
                                prompt
                                    .attachments
                                    .iter()
                                    .map(|attachment| worker_control::AssetRef {
                                        asset: attachment.attachment_id.clone(),
                                        mime: attachment.media_type.as_str().to_owned(),
                                    })
                                    .collect()
                            }),
                        },
                    )
                    .map_err(internal)?,
                )
                .map_err(internal)
                .and_then(|()| worker.receipt(&delivery));
            if let Ok(receipt) = delivered {
                self.spawn_automatic_title_seed_and_refinement(
                    session_id.to_owned(),
                    timestamp.to_owned(),
                    prompt_text(&prompt.blocks),
                );
                return Ok(MutationReceipt {
                    seq: receipt.seq,
                    deduplicated: receipt.deduplicated,
                });
            }
            // A closed pipe alone does not relinquish ledger ownership. After
            // confirmed process exit, the locked author reuses the same origin
            // and re-acks any input durably written before its receipt was lost.
            if !child_exited_within(&worker.child, Duration::from_secs(1)) {
                return Err(delivered.unwrap_err());
            }
            worker.alive.store(false, Ordering::Release);
        }
        self.locked_prompt(session_id, timestamp, origin, prompt, steer)
    }

    fn compact(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        self.reset_restart_backoff(session_id);
        if let Some(worker) = self.live_worker(session_id) {
            let delivery = origin.key.clone();
            let delivered = worker
                .write(
                    &encode_line(
                        "compact",
                        &worker_control::Compact {
                            delivery: delivery.clone(),
                            origin: origin.clone(),
                        },
                    )
                    .map_err(internal)?,
                )
                .map_err(internal)
                .and_then(|()| worker.receipt(&delivery));
            if let Ok(receipt) = delivered {
                return Ok(MutationReceipt {
                    seq: receipt.seq,
                    deduplicated: receipt.deduplicated,
                });
            }
            // Same rule as prompt delivery: a closed pipe alone does not
            // relinquish ledger ownership; only a confirmed exit lets the
            // locked author take the same origin.
            if !child_exited_within(&worker.child, Duration::from_secs(1)) {
                return Err(delivered.unwrap_err());
            }
            worker.alive.store(false, Ordering::Release);
        }
        self.locked_compact(session_id, timestamp, origin)
    }

    fn cancel(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        self.reset_restart_backoff(session_id);
        let started = Instant::now();
        if let Some(worker) = self.live_worker(session_id) {
            worker.tool_cancellation.cancel();
            if let Some(authority) = worker.dynamic_authority.as_ref() {
                authority.cancel_inflight();
            }
            let delivery = origin.key.clone();
            let generation = next_stop_generation(&self.root, session_id)?;
            let stop = encode_line(
                "stop",
                &Stop {
                    delivery: delivery.clone(),
                    origin: origin.clone(),
                    generation,
                },
            )
            .map_err(internal)?;
            let delivered = worker
                .write(&stop)
                .map_err(|error| ("stop-write", internal(error)))
                .and_then(|()| {
                    worker
                        .receipt(&delivery)
                        .map_err(|error| ("receipt", error))
                });
            match delivered {
                Ok(receipt) => {
                    let ledger = self
                        .root
                        .join("threads")
                        .join(session_id)
                        .join("main.jsonl");
                    self.cascade_stop_from(session_id, session_id, &ledger, &mut HashSet::new())
                        .map_err(|error| {
                            cancel_stage_failure("cascade", started, internal(error))
                        })?;
                    terminate_worker(&worker);
                    return Ok(MutationReceipt {
                        seq: receipt.seq,
                        deduplicated: receipt.deduplicated,
                    });
                }
                Err((stage, error)) => {
                    // A worker exits when its turn ends, so a stop can find a
                    // closed pipe after the turn already settled. As for
                    // prompt, compact and respond, a closed pipe alone does
                    // not relinquish ledger ownership; once the process has
                    // exited, the locked author keys the stop by the same
                    // origin and returns the durable receipt.
                    if !child_exited_within(&worker.child, Duration::from_secs(1)) {
                        return Err(cancel_stage_failure(stage, started, error));
                    }
                    worker.alive.store(false, Ordering::Release);
                    eprintln!(
                        "session-cancel-worker-exited: stage={stage} elapsed_ms={} fallback=locked-author",
                        started.elapsed().as_millis()
                    );
                }
            }
        }
        let receipt = self
            .endpoint
            .cancel_for_endpoint(session_id, timestamp, origin)
            .map_err(internal)?;
        let ledger = self
            .root
            .join("threads")
            .join(session_id)
            .join("main.jsonl");
        self.cascade_stop_from(session_id, session_id, &ledger, &mut HashSet::new())
            .map_err(internal)?;
        if let Err(error) = self.schedule_main(session_id) {
            self.record_spawn_failure("cancel", session_id, receipt.seq, &error);
        }
        Ok(receipt)
    }

    fn rename(
        &self,
        session_id: &str,
        timestamp: &str,
        origin: &OriginTuple,
        title: &str,
    ) -> Result<MutationReceipt, ProductionRouteFailure> {
        self.reset_restart_backoff(session_id);
        if let Some(worker) = self.live_worker(session_id) {
            let delivery = origin.key.clone();
            worker
                .write(
                    &encode_line(
                        "meta",
                        &Meta {
                            delivery: delivery.clone(),
                            origin: origin.clone(),
                            title: Some(title.to_owned()),
                            labels: None,
                        },
                    )
                    .map_err(internal)?,
                )
                .map_err(internal)?;
            let receipt = worker.receipt(&delivery)?;
            return Ok(MutationReceipt {
                seq: receipt.seq,
                deduplicated: receipt.deduplicated,
            });
        }
        self.endpoint
            .rename_session_for_endpoint(session_id, timestamp, origin, title)
            .map_err(internal)
    }
}

impl LiveRespondAuthority for ProductionProcessHost {
    fn deliver_if_live(
        &self,
        session_id: &str,
        response: &ApprovalResponse,
    ) -> Result<Option<Receipt>, ProductionRouteFailure> {
        let Some(worker) = self.live_worker(session_id) else {
            return Ok(None);
        };
        let bytes = encode_line("approval_response", response).map_err(internal)?;
        let result = worker
            .write(&bytes)
            .map_err(internal)
            .and_then(|()| worker.receipt(&response.delivery))
            .map(Some);
        if result.is_err() && child_exited_within(&worker.child, Duration::from_secs(1)) {
            // The durable receipt may have been lost at exit. The locked author
            // reuses this exact origin, so an already-written answer deduplicates.
            // Only process exit (and thus released ledger ownership) permits
            // fallback; a transport error or stale alive flag alone does not.
            return Ok(None);
        }
        result
    }

    fn ensure_after_locked_append(&self, session_id: &str) -> Result<(), ProductionRouteFailure> {
        if let Some((session, child)) = session_id.split_once(':') {
            self.schedule_line(session, child, &format!("{child}.jsonl"))
                .map(|_| ())
                .map_err(internal)
        } else {
            self.schedule_main(session_id).map(|_| ()).map_err(internal)
        }
    }
}

/// Names the cancel stage and the time spent in it on a failure that the
/// endpoint folds into `internal`, so `endpoint-route-error-folded` shows where
/// a stop failed. Typed (exact) failures keep their payload.
fn cancel_stage_failure(
    stage: &str,
    started: Instant,
    error: ProductionRouteFailure,
) -> ProductionRouteFailure {
    if error.code != "internal" {
        return error;
    }
    ProductionRouteFailure::new(
        error.code,
        format!(
            "session.cancel stage={stage} elapsed_ms={}: {}",
            started.elapsed().as_millis(),
            error.message
        ),
        error.details,
    )
}

fn child_exited_within(child: &Mutex<Child>, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_lock() {
            Ok(mut child) => match child.try_wait() {
                Ok(Some(_)) => return true,
                Ok(None) => {}
                Err(_) => return false,
            },
            Err(std::sync::TryLockError::WouldBlock) => {}
            Err(std::sync::TryLockError::Poisoned(_)) => return false,
        }
        if Instant::now() >= deadline {
            return false;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// Facts that classify as `settled`: used for the placeholder row of a thread whose verification
/// has not finished inside the List's wait budget. It is never `interrupted` or
/// `recovery_needed`: a wrong-but-neutral tail is corrected by the upsert that follows.
fn neutral_lifecycle() -> schema::LifecycleFacts {
    schema::LifecycleFacts {
        latest_turn: None,
        terminal_tail: true,
        stop_active: false,
        stop_closure_point: None,
        open_hold: false,
        answered_hold: false,
        runnable_inputs: Vec::new(),
        turn_open_inputs: Vec::new(),
        live_inputs: Vec::new(),
        unstarted: false,
        unresolved_work: false,
        resume_policy: schema::ResumePolicy::Never,
        recovery_ordinal: 0,
        continuation_wait_until: None,
        admission_wait_until: None,
    }
}

/// Whether the bytes of a ledger file show damage no repair can fix: no valid genesis, or a
/// complete (LF-terminated) record after the valid prefix that does not validate. Returns the
/// offset of the first invalid byte.
fn ledger_damage(bytes: &[u8]) -> Option<u64> {
    let scan = scan_valid_prefix(bytes, schema::READER_VERSION);
    let valid = usize::try_from(scan.valid_bytes).unwrap_or(usize::MAX);
    (scan.projection.is_none()
        || (scan.needs_repair() && bytes.get(valid..).is_some_and(|tail| tail.contains(&b'\n'))))
    .then(|| scan.first_invalid_offset.unwrap_or(scan.valid_bytes))
}

/// The decision about a ledger an unlocked read showed damaged.
#[derive(Debug, PartialEq, Eq)]
enum DamageVerdict {
    /// Damage at this byte offset, read while holding the line's exclusive lock.
    Damaged(u64),
    /// The read under the lock shows no damage: the first picture was transient.
    Sound,
    /// A live holder owns the line (or the file cannot be examined): nothing is decided now.
    Deferred,
}

/// Decides whether a ledger is damaged on a read taken under its exclusive lock, never blocking.
/// Repairs and appends happen only under that lock, so a read under it cannot straddle them; a
/// quarantine is sticky for the life of the process, so it must not rest on an unlocked read
/// (O-final m3, n3). When the lock is busy a live holder owns the line and the decision is
/// deferred to a later doorbell or sweep. Holding the lock briefly can make a starting worker's
/// non-blocking acquire fail with `Busy`, exactly as `probe_line_lock` and `read_ledger_repaired`
/// already accept.
fn damage_under_lock(path: &Path) -> DamageVerdict {
    let Ok(file) = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
    else {
        return DamageVerdict::Deferred;
    };
    loop {
        // SAFETY: file owns a live descriptor and flock retains no pointers.
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
            break;
        }
        if std::io::Error::last_os_error().kind() != std::io::ErrorKind::Interrupted {
            return DamageVerdict::Deferred;
        }
    }
    // The lock is held until `file` drops, after the read. It must be the lock of the file the
    // path names now (the ledger is never replaced under a holder).
    let (Ok(descriptor), Ok(pathname)) = (file.metadata(), fs::metadata(path)) else {
        return DamageVerdict::Deferred;
    };
    if descriptor.dev() != pathname.dev() || descriptor.ino() != pathname.ino() {
        return DamageVerdict::Deferred;
    }
    match fs::read(path) {
        Ok(bytes) => ledger_damage(&bytes).map_or(DamageVerdict::Sound, DamageVerdict::Damaged),
        Err(_) => DamageVerdict::Deferred,
    }
}

/// A complete but invalid record, or no valid genesis, in a child line ledger (`<uuid>.jsonl`):
/// damage no repair can fix. A torn tail is only a partial append and is not damage. An empty
/// child file is not judged here: `repair_thread_tails` already reports it as damage before this
/// runs, and no live path produces one (a child's genesis is published by rename).
fn child_ledger_damage(folder: &Path) -> Option<String> {
    let entries = fs::read_dir(folder).ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "main.jsonl"
            || !store::card::valid_line_name(&name)
            || !entry.file_type().is_ok_and(|kind| kind.is_file())
        {
            continue;
        }
        let Ok(bytes) = fs::read(entry.path()) else {
            continue;
        };
        if bytes.is_empty() {
            continue;
        }
        if ledger_damage(&bytes).is_some()
            && let DamageVerdict::Damaged(offset) = damage_under_lock(&entry.path())
        {
            return Some(format!("{name}: no valid record at byte {offset}"));
        }
    }
    None
}

/// A ledger that holds its genesis and nothing else: a thread nobody has spoken in yet.
fn is_single_line_ledger(path: &Path) -> bool {
    let Ok(file) = fs::File::open(path) else {
        return false;
    };
    let mut reader = BufReader::new(file);
    let mut first = Vec::new();
    if reader.read_until(b'\n', &mut first).is_err() || first.last() != Some(&b'\n') {
        return false;
    }
    reader.fill_buf().is_ok_and(<[u8]>::is_empty)
}

/// How long a thread that failed verification is left alone before the next attempt.
const HYDRATE_RETRY: Duration = Duration::from_secs(30);

const DEFAULT_LIST_WAIT: Duration = Duration::from_millis(2_000);

impl ProductionProcessHost {
    fn list_wait(&self) -> Duration {
        #[cfg(test)]
        if let Some(wait) = *self
            .list_wait_override
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
        {
            return wait;
        }
        std::env::var("TEKES_KERNEL_LIST_COLD_WAIT_MS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .map_or(DEFAULT_LIST_WAIT, Duration::from_millis)
    }

    fn is_running(&self, session_id: &str, live: &HashSet<String>) -> bool {
        live.contains(session_id)
            || self
                .threads
                .session_has_live_line_holder(session_id)
                .unwrap_or(false)
    }

    /// The List row of one catalog entry. `None` hides it (a previous process's ephemeral
    /// scratch thread) or means the folder is unreadable.
    fn catalog_item(
        &self,
        session_id: &str,
        entry: &crate::catalog::Entry,
        live: &HashSet<String>,
    ) -> Option<SessionInventoryItem> {
        let folder = self.root.join("threads").join(session_id);
        let running = self.is_running(session_id, live);
        if let Some(view) = entry
            .view
            .as_ref()
            .filter(|_| !matches!(entry.trust, crate::catalog::Trust::Cold))
        {
            let main = view.main()?;
            if self.is_stale_ephemeral(session_id, view.summary.ephemeral) {
                return None;
            }
            return Some(SessionInventoryItem {
                session_id: session_id.to_owned(),
                workspace_id: view.summary.workspace.clone(),
                title: view.summary.title.clone(),
                updated_at: view.summary.updated_ms,
                archived: false,
                running,
                blank: view.summary.blank,
                as_of_seq: main.last_seq,
                lifecycle: main.facts.clone(),
                permission_mode: endpoint::permission_mode_of(&folder),
                identity_profile: view.summary.identity_profile.clone(),
                preset: view.summary.preset.clone(),
                ephemeral: view.summary.ephemeral,
            });
        }
        // Placeholder: a Cold or quarantined thread. Identity, workspace and recency come from
        // the genesis line and the file time; no title, neutral tail.
        let genesis = store::read_genesis(folder.join("main.jsonl")).ok()??;
        let ephemeral = genesis.is_ephemeral_genesis();
        if self.is_stale_ephemeral(session_id, ephemeral) {
            return None;
        }
        // The last record's timestamp (what a verified row shows), else the file time.
        let updated_at = endpoint::last_record_updated_ms(&folder.join("main.jsonl"))
            .unwrap_or_else(|| {
                fs::metadata(folder.join("main.jsonl"))
                    .and_then(|metadata| metadata.modified())
                    .ok()
                    .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
                    .map_or(0, |duration| {
                        u64::try_from(duration.as_millis()).unwrap_or(0)
                    })
            });
        Some(SessionInventoryItem {
            session_id: session_id.to_owned(),
            workspace_id: genesis.string_field("workspace")?.to_owned(),
            title: None,
            updated_at,
            archived: false,
            running,
            blank: is_single_line_ledger(&folder.join("main.jsonl")),
            as_of_seq: 0,
            lifecycle: neutral_lifecycle(),
            permission_mode: endpoint::permission_mode_of(&folder),
            preset: endpoint::genesis_preset_summary(&genesis),
            identity_profile: match genesis.string_field("identity_profile") {
                None if endpoint::genesis_preset_summary(&genesis).is_some() => None,
                None => Some("coding".to_owned()),
                Some(profile @ ("coding" | "general")) => Some(profile.to_owned()),
                _ => None,
            },
            ephemeral,
        })
    }

    /// Verifies the not-yet-verified threads in parallel and waits for them at most `budget`
    /// (R2): the List ships whatever finished; the rest keep verifying in the background and
    /// announce themselves with a status frame, which turns into an inventory upsert.
    fn verify_within(&self, ids: Vec<String>, budget: Duration) {
        if ids.is_empty() {
            return;
        }
        let Some(host) = self.self_weak.upgrade() else {
            return;
        };
        let total = ids.len();
        let queue = Arc::new(Mutex::new(ids.into_iter().collect::<VecDeque<_>>()));
        let finished = Arc::new((Mutex::new(0_usize), Condvar::new()));
        let workers = std::thread::available_parallelism()
            .map_or(2, std::num::NonZeroUsize::get)
            .min(8)
            .min(total);
        for _ in 0..workers {
            let (host, queue, finished) =
                (Arc::clone(&host), Arc::clone(&queue), Arc::clone(&finished));
            std::thread::spawn(move || {
                loop {
                    let next = {
                        let mut queue = queue
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner);
                        let next = queue.pop_front();
                        // Recorded under the queue lock, so the record is the pop order.
                        #[cfg(test)]
                        if let Some(id) = &next {
                            host.verify_order
                                .lock()
                                .unwrap_or_else(std::sync::PoisonError::into_inner)
                                .push(id.clone());
                        }
                        next
                    };
                    let Some(id) = next else {
                        return;
                    };
                    if let Err(error) = host.hydrate(&id) {
                        eprintln!("process-host-list-verify-skipped: session={id} error={error}");
                    }
                    host.publish_session_status(&id);
                    let (count, changed) = &*finished;
                    *count
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner) += 1;
                    changed.notify_all();
                }
            });
        }
        let (count, changed) = &*finished;
        let deadline = Instant::now() + budget;
        let mut done = count
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        while *done < total {
            let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
                break;
            };
            done = changed
                .wait_timeout(done, remaining)
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .0;
        }
    }

    fn build_inventory(
        &self,
        include_archived: bool,
        wait: Duration,
    ) -> Result<Vec<SessionInventoryItem>, DaemonError> {
        let live = self.live_sessions();
        self.catalog.ensure_loaded();
        for id in store::folder_events::admitted_ids(&self.root) {
            let _ = self.catalog.get(&id);
        }
        // Newest ledger first, so the rows at the top of a sidebar resolve first.
        let unverified = self
            .catalog
            .unverified()
            .into_iter()
            .filter(|id| !self.recently_failed(id))
            .collect::<Vec<_>>();
        self.verify_within(unverified, wait);
        let mut items = self
            .catalog
            .entries_now()
            .into_iter()
            .filter_map(|(id, entry)| self.catalog_item(&id, &entry, &live))
            .collect::<Vec<_>>();
        if include_archived {
            for entry in fs::read_dir(self.root.join("archive")).map_err(DaemonError::io)? {
                let entry = entry.map_err(DaemonError::io)?;
                let id = entry.file_name().to_string_lossy().into_owned();
                if entry.file_type().map_err(DaemonError::io)?.is_dir()
                    && endpoint::validate_session_id(&id).is_ok()
                {
                    items.push(SessionInventoryItem {
                        session_id: id,
                        workspace_id: String::new(),
                        title: None,
                        updated_at: 0,
                        archived: true,
                        running: false,
                        blank: false,
                        as_of_seq: 0,
                        lifecycle: neutral_lifecycle(),
                        permission_mode: String::new(),
                        identity_profile: None,
                        preset: None,
                        ephemeral: false,
                    });
                }
            }
        }
        items.sort_by(|left, right| left.session_id.as_bytes().cmp(right.session_id.as_bytes()));
        Ok(items)
    }
}

impl SupervisorSessionAuthority for ProductionProcessHost {
    fn inventory_items(
        &self,
        include_archived: bool,
    ) -> Option<Result<Vec<SessionInventoryItem>, String>> {
        Some(
            self.build_inventory(include_archived, self.list_wait())
                .map_err(|error| error.to_string()),
        )
    }

    fn ensure_verified(&self, session_id: &str) -> Result<(), SessionGateError> {
        // A thread with a live worker is already verified by that worker's lock holder.
        if self.live_worker(session_id).is_some() {
            return Ok(());
        }
        ProductionProcessHost::ensure_verified(self, session_id).map_err(|error| {
            let message = error.to_string();
            if message.contains("quarantined") {
                SessionGateError::Quarantined(message)
            } else {
                SessionGateError::Failed(message)
            }
        })
    }

    fn has_catalog(&self) -> bool {
        true
    }

    fn hold_catalog(&self) -> Option<HoldCatalog> {
        use crate::catalog::Trust;
        self.catalog.ensure_loaded();
        let mut result = HoldCatalog::default();
        for (session, entry) in self.catalog.entries_now() {
            let holds = entry
                .view
                .as_ref()
                .map(|view| {
                    view.lines
                        .iter()
                        .flat_map(|line| {
                            line.holds.iter().map(|hold| HoldRef {
                                line: (line.file != "main.jsonl").then(|| line.file.clone()),
                                hold_seq: hold.hold_seq,
                                kind: if hold.kind == "question" {
                                    endpoint::RequestFrameType::Question
                                } else {
                                    endpoint::RequestFrameType::Approval
                                },
                                resolved: hold.resolved.is_some(),
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            match entry.trust {
                Trust::Fresh => {
                    if entry.view.as_ref().is_some_and(|view| !view.children_exact) {
                        result.children_unverified.push(session.clone());
                    }
                    result.verified.push(VerifiedHolds {
                        session,
                        holds,
                        quarantined: false,
                    });
                }
                Trust::Corrupt(_) => result.verified.push(VerifiedHolds {
                    session,
                    holds,
                    quarantined: true,
                }),
                Trust::Ahead => {
                    result.verified.push(VerifiedHolds {
                        session: session.clone(),
                        holds,
                        quarantined: false,
                    });
                    result.unverified.push(session);
                }
                Trust::Cold => result.unverified.push(session),
            }
        }
        Some(result)
    }

    fn inventory_item(
        &self,
        session_id: &str,
    ) -> Option<Result<Option<SessionInventoryItem>, String>> {
        let live = self.live_sessions();
        let mut entry = self.catalog.get(session_id);
        // The single-row path serves one thread, so it verifies that thread instead of listing
        // a placeholder for it (a brand-new session would otherwise show `blank:false` for its
        // whole first turn, E832 M4). A thread that cannot be verified keeps its placeholder or
        // quarantined row.
        if entry.as_ref().is_some_and(|entry| {
            matches!(
                entry.trust,
                crate::catalog::Trust::Cold | crate::catalog::Trust::Ahead
            )
        }) && self.live_worker(session_id).is_none()
        {
            if let Err(error) = self.hydrate(session_id) {
                eprintln!(
                    "process-host-inventory-verify-skipped: session={session_id} error={error}"
                );
            }
            entry = self.catalog.get(session_id);
        }
        Some(Ok(entry.and_then(|entry| {
            self.catalog_item(session_id, &entry, &live)
        })))
    }

    fn live_sessions(&self) -> HashSet<String> {
        self.workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .iter()
            .filter(|(_, worker)| worker.alive.load(Ordering::Acquire))
            .map(|(key, _)| key.split(':').next().unwrap_or(key).to_owned())
            .collect()
    }

    fn reconcile_projection(&self, session_id: &str) -> Result<(), String> {
        if self.live_worker(session_id).is_some() {
            return Ok(());
        }
        self.publish_appended(session_id)
            .map_err(|error| error.to_string())
    }
}

impl QueueTransactionAuthority for ProductionProcessHost {
    fn execute(
        &self,
        session_id: &str,
        transaction: &QueueTransaction,
    ) -> Result<QueueTransactionResult, ProductionRouteFailure> {
        self.reset_restart_backoff(session_id);
        let ledger = self
            .root
            .join("threads")
            .join(session_id)
            .join("main.jsonl");
        let scheduled = self
            .schedule_worker_at_with_startup(session_id, session_id, ledger, Some(transaction))
            .map_err(internal)?;
        let worker = match scheduled.worker {
            Some(worker) => worker,
            None => self.wait_for_worker(session_id).map_err(internal)?,
        };
        if !scheduled.startup_preloaded {
            worker
                .write(&encode_queue_transaction(transaction).map_err(internal)?)
                .map_err(internal)?;
        }
        worker.queue_result(&transaction.delivery)
    }
}

impl ProviderReadinessAuthority for ProductionProcessHost {
    fn readiness(
        &self,
        _session_id: &str,
        config: &ConfigSnapshot,
    ) -> Result<Vec<RuntimeProviderReadiness>, ProductionRouteFailure> {
        let credentials = provider::resolve_config_credentials(config, self.secret_store.as_ref())
            .map_err(internal)?;
        Ok(config
            .providers
            .providers
            .iter()
            .map(|configured| {
                let route_misconfigured = provider::validate_provider(configured).is_err();
                let resolved_models = configured
                    .models
                    .iter()
                    .filter(|model| model.enabled)
                    .filter_map(|model| {
                        provider::resolve_profile(configured, model)
                            .ok()
                            .map(|profile| (model, profile))
                    })
                    .collect::<Vec<_>>();
                let status = if route_misconfigured
                    || provider::endpoint_origin(&configured.endpoint).is_err()
                    || resolved_models.is_empty()
                {
                    RuntimeProviderStatus::Failed {
                        failure: RuntimeProviderFailure::Misconfigured,
                    }
                } else if let Some(credential_id) = &configured.credential_key {
                    match credentials.availability.get(credential_id) {
                        Some(provider::CredentialAvailability::Active { .. }) => {
                            RuntimeProviderStatus::Ready
                        }
                        Some(provider::CredentialAvailability::Unavailable) => {
                            RuntimeProviderStatus::Failed {
                                failure: RuntimeProviderFailure::Unavailable,
                            }
                        }
                        Some(
                            provider::CredentialAvailability::Revoked { .. }
                            | provider::CredentialAvailability::NotFound,
                        )
                        | None => RuntimeProviderStatus::Failed {
                            failure: RuntimeProviderFailure::InvalidCredential,
                        },
                    }
                } else {
                    RuntimeProviderStatus::Ready
                };
                RuntimeProviderReadiness {
                    provider: configured.id.clone(),
                    models: if status == RuntimeProviderStatus::Ready {
                        resolved_models
                            .into_iter()
                            .map(|(model, profile)| RuntimeModelReadiness {
                                id: model.id.clone(),
                                efforts: profile
                                    .reasoning_efforts()
                                    .iter()
                                    .map(|value| (*value).to_owned())
                                    .collect(),
                                default_effort: profile
                                    .default_reasoning_effort()
                                    .map(str::to_owned),
                            })
                            .collect()
                    } else {
                        Vec::new()
                    },
                    status,
                }
            })
            .collect())
    }

    fn config_mutation_succeeded(&self, session_id: &str) -> Result<(), ProductionRouteFailure> {
        self.refresh_live_credentials().map_err(internal)?;
        if let Some(streams) = self
            .streams
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
        {
            let result = if session_id.is_empty() {
                streams.refresh_all_context_projections()
            } else {
                streams.refresh_context_projection(session_id)
            };
            if let Err(error) = result {
                eprintln!(
                    "process-host-context-projection-failed: session={session_id} error={error}"
                );
            }
        }
        Ok(())
    }
}

impl ProcessToolRuntime {
    fn host(&self) -> Result<Arc<ProductionProcessHost>, SupervisorOperationError> {
        self.host.upgrade().ok_or_else(|| {
            SupervisorOperationError::Unavailable("production process host was dropped".into())
        })
    }
}

impl SupervisorRuntimeAuthority for ProcessToolRuntime {
    fn ensure_running(
        &mut self,
        session: &str,
        _request_id: &str,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        self.host()?
            .ensure_running(session)
            .map_err(operation_unavailable)?;
        operation_value(serde_json::json!({"running":true,"session_id":session}))
    }

    fn deliver_input(
        &mut self,
        request: &DeliveryRequest,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        let host = self.host()?;
        let timestamp =
            crate::host_runtime::system_timestamp().map_err(SupervisorOperationError::Protocol)?;
        let origin = OriginTuple {
            principal: "kernel-worker".into(),
            client: "context".into(),
            target: request.target_session.clone(),
            op: "prompt".into(),
            key: request.request_id.clone(),
        };
        let receipt = host
            .locked_prompt(
                &request.target_session,
                &timestamp,
                &origin,
                &MaterializedPrompt {
                    blocks: vec![Block::Text {
                        text: request.message.clone(),
                    }],
                    attachments: Vec::new(),
                    files: Vec::new(),
                },
                false,
            )
            .map_err(|error| SupervisorOperationError::Unavailable(error.message))?;
        operation_value(serde_json::json!({
            "deduplicated":receipt.deduplicated,
            "seq":receipt.seq,
            "session_id":request.target_session,
        }))
    }

    fn interrupt(
        &mut self,
        request: &InterruptRequest,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        let host = self.host()?;
        let timestamp =
            crate::host_runtime::system_timestamp().map_err(SupervisorOperationError::Protocol)?;
        let origin = OriginTuple {
            principal: "kernel-worker".into(),
            client: "context".into(),
            target: request.target_session.clone(),
            op: "cancel".into(),
            key: request.request_id.clone(),
        };
        let receipt = host
            .cancel(&request.target_session, &timestamp, &origin)
            .map_err(|error| SupervisorOperationError::Unavailable(error.message))?;
        operation_value(serde_json::json!({
            "deduplicated":receipt.deduplicated,
            "seq":receipt.seq,
            "session_id":request.target_session,
        }))
    }

    fn ensure_child(
        &mut self,
        proof: &ChildLaunchProof,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        self.host()?
            .validate_child_proof(proof)
            .map_err(operation_unavailable)?;
        operation_value(serde_json::json!({
            "child":proof.child_line,
            "session_id":proof.session,
            "spawn_id":proof.spawn_id,
        }))
    }

    fn deliver_report(
        &mut self,
        proof: &ParentReportProof,
        _result: &IJsonValue,
    ) -> Result<IJsonValue, SupervisorOperationError> {
        // The parent worker owns the child_result semantic append and derives
        // it by tailing the durably bound child ledger. Waking the exact parent
        // is the supervisor's only authority at this boundary.
        self.host()?
            .schedule_line(&proof.session, &proof.parent_line, &proof.parent_file)
            .map_err(operation_unavailable)?;
        operation_value(serde_json::json!({
            "delivered":true,
            "parent":proof.parent_line,
            "session_id":proof.session,
            "spawn_id":proof.spawn_id,
        }))
    }
}

// A resumed child retains the same dependency admission as its initial launch.
// Derive that privilege from both ledgers, never from the client target alone.
fn unresolved_parent_dependency(
    root: &Path,
    session: &str,
    child_line: &str,
    child_path: &Path,
) -> Result<Option<String>, DaemonError> {
    let read = |path: &Path| -> Result<schema::LedgerProjection, DaemonError> {
        scan_valid_prefix(
            &fs::read(path).map_err(DaemonError::io)?,
            schema::READER_VERSION,
        )
        .projection
        .ok_or_else(|| DaemonError::corrupt("dependency ledger has no genesis"))
    };
    let child = read(child_path)?;
    let genesis = child
        .events
        .first()
        .ok_or_else(|| DaemonError::corrupt("child genesis missing"))?;
    if genesis.string_field("thread") != Some(child_line) {
        return Err(DaemonError::corrupt("dependency child identity mismatch"));
    }
    let raw =
        serde_json::to_value(genesis.raw()).map_err(|e| DaemonError::corrupt(e.to_string()))?;
    let Some(parent) = raw.get("parent") else {
        return Ok(None);
    };
    let file = parent["file"]
        .as_str()
        .ok_or_else(|| DaemonError::corrupt("dependency parent file missing"))?;
    let parent_line = if file == "main.jsonl" {
        session
    } else {
        file.strip_suffix(".jsonl")
            .ok_or_else(|| DaemonError::corrupt("dependency parent file invalid"))?
    };
    let (parent_key, parent_path) = line_schedule_target(root, session, parent_line, file)?;
    if parent_path == child_path {
        return Err(DaemonError::corrupt("self dependency"));
    }
    let projection = read(&parent_path)?;
    if projection
        .events
        .first()
        .and_then(|e| e.string_field("thread"))
        != Some(parent_line)
    {
        return Err(DaemonError::corrupt("dependency parent identity mismatch"));
    }
    let seq = parent["seq"]
        .as_u64()
        .ok_or_else(|| DaemonError::corrupt("dependency spawn seq missing"))?;
    let spawn_id = parent["spawn_id"]
        .as_str()
        .ok_or_else(|| DaemonError::corrupt("dependency spawn id missing"))?;
    let spawn = projection
        .events
        .iter()
        .find(|e| e.seq() == seq && e.kind() == &EventKind::Spawn)
        .ok_or_else(|| DaemonError::corrupt("dependency spawn missing"))?;
    if spawn.string_field("child") != child_path.file_name().and_then(|n| n.to_str())
        || spawn.string_field("spawn_id") != Some(spawn_id)
    {
        return Err(DaemonError::corrupt("dependency spawn mismatch"));
    }
    if spawn.turn() != projection.lifecycle.latest_turn
        || projection.events.iter().any(|e| {
            e.seq() > seq
                && ((e.kind() == &EventKind::ChildResult
                    && e.string_field("spawn_id") == Some(spawn_id))
                    || (e.kind() == &EventKind::Settle && e.turn() == spawn.turn()))
        })
    {
        return Ok(None);
    }
    Ok(Some(parent_key))
}

fn line_schedule_target(
    root: &Path,
    session_id: &str,
    line_id: &str,
    line_file: &str,
) -> Result<(String, PathBuf), DaemonError> {
    let file_path = Path::new(line_file);
    if file_path.file_name().and_then(|name| name.to_str()) != Some(line_file)
        || file_path
            .extension()
            .and_then(|extension| extension.to_str())
            != Some("jsonl")
    {
        return Err(DaemonError::corrupt("parent line file name is invalid"));
    }
    let process_key = if line_file == "main.jsonl" {
        if line_id != session_id {
            return Err(DaemonError::corrupt(
                "main parent line does not match the session id",
            ));
        }
        session_id.to_owned()
    } else {
        if line_file != format!("{line_id}.jsonl") {
            return Err(DaemonError::corrupt(
                "child parent file does not match the parent line id",
            ));
        }
        format!("{session_id}:{line_id}")
    };
    Ok((
        process_key,
        root.join("threads").join(session_id).join(line_file),
    ))
}

fn operation_unavailable(error: DaemonError) -> SupervisorOperationError {
    SupervisorOperationError::Unavailable(error.to_string())
}

fn operation_value(value: serde_json::Value) -> Result<IJsonValue, SupervisorOperationError> {
    IJsonValue::parse(
        &serde_json::to_vec(&value)
            .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))?,
    )
    .map_err(|error| SupervisorOperationError::Protocol(error.to_string()))
}

pub(crate) fn workspace_quiescence_lock_path(root: &Path, workspace_id: &str) -> PathBuf {
    let digest = Sha256::digest(workspace_id.as_bytes());
    root.join(format!(".workspace-quiescence-{digest:x}.lock"))
}

fn workspace_id(ledger: &Path) -> Result<String, DaemonError> {
    session_workspace_binding(ledger).map(|(workspace_id, _)| workspace_id)
}

fn scoped_client_file_path(
    workspace: &profile::ResolvedWorkspace,
    path: &str,
) -> Result<(PathBuf, String), DaemonError> {
    let requested = Path::new(path);
    if path.is_empty()
        || path.contains('\0')
        || requested
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(DaemonError::invalid_config("invalid file path"));
    }
    let roots = workspace
        .cwd
        .iter()
        .map(|root| Path::new(root).canonicalize().map_err(DaemonError::io))
        .collect::<Result<Vec<_>, _>>()?;
    let target = if requested.is_absolute() {
        requested.to_owned()
    } else {
        let selected = workspace
            .selected_cwd
            .as_deref()
            .or_else(|| workspace.cwd.first().map(String::as_str))
            .ok_or_else(|| DaemonError::invalid_config("session has no directory"))?;
        Path::new(selected).join(requested)
    }
    .canonicalize()
    .map_err(DaemonError::io)?;
    let root = roots
        .into_iter()
        .find(|root| target.starts_with(root))
        .ok_or_else(|| DaemonError::invalid_config("path outside session workspace"))?;
    let relative = target
        .strip_prefix(&root)
        .unwrap()
        .to_str()
        .ok_or_else(|| DaemonError::invalid_config("file path is not UTF-8"))?
        .to_owned();
    Ok((root, relative))
}

fn session_workspace_binding(ledger: &Path) -> Result<(String, Option<String>), DaemonError> {
    // The genesis line is the only input: do not decode the whole ledger per lookup.
    let genesis = store::read_genesis(ledger)
        .map_err(DaemonError::store)?
        .ok_or_else(|| DaemonError::corrupt("worker ledger has no projection"))?;
    let genesis = &genesis;
    let workspace_id = genesis
        .string_field("workspace")
        .map(str::to_owned)
        .ok_or_else(|| DaemonError::corrupt("worker genesis has no workspace"))?;
    let folder_binding = genesis.string_field("folder_binding").map(str::to_owned);
    Ok((workspace_id, folder_binding))
}

fn ledger_needs_worker(ledger: &Path) -> Result<bool, DaemonError> {
    let bytes = fs::read(ledger).map_err(DaemonError::io)?;
    let facts = scan_valid_prefix(&bytes, schema::READER_VERSION)
        .projection
        .ok_or_else(|| DaemonError::corrupt("worker ledger has no projection"))?
        .lifecycle;
    let lock_facts = if probe_line_lock(ledger)? == LineLockState::Busy {
        LockFacts::OTHER
    } else {
        LockFacts::FREE
    };
    Ok(
        ensure_action_at(classify(&facts, lock_facts), &facts, Some(&now_rfc3339()))
            != EnsureAction::None
            || (lock_facts == LockFacts::FREE && goal_continuation_due(ledger)?),
    )
}

fn goal_continuation_due(ledger: &Path) -> Result<bool, DaemonError> {
    if ledger.file_name() != Some(std::ffi::OsStr::new("main.jsonl")) {
        return Ok(false);
    }
    let folder = ledger
        .parent()
        .ok_or_else(|| DaemonError::corrupt("goal ledger has no session folder"))?;
    let session = folder
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| DaemonError::corrupt("goal session id is invalid"))?;
    let root = folder
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| DaemonError::corrupt("goal ledger has no storage root"))?;
    let Some(goal) = session_controls::read_goal(root, session)
        .map_err(|error| DaemonError::invalid_config(error.to_string()))?
    else {
        return Ok(false);
    };
    if goal.phase != session_controls::PHASE_ACTIVE || goal.rounds_started >= goal.max_goal_rounds {
        return Ok(false);
    }
    let bytes = fs::read(ledger).map_err(DaemonError::io)?;
    let projection = scan_valid_prefix(&bytes, schema::READER_VERSION)
        .projection
        .ok_or_else(|| DaemonError::corrupt("goal ledger has no projection"))?;
    let facts = &projection.lifecycle;
    if !facts.terminal_tail || facts.stop_active || !facts.turn_open_inputs.is_empty() {
        return Ok(false);
    }
    let Some(turn) = facts.latest_turn else {
        return Ok(false);
    };
    Ok(projection
        .events
        .iter()
        .rev()
        .find(|event| event.kind() == &schema::EventKind::Settle)
        .is_some_and(|event| {
            event.turn() == Some(turn) && event.string_field("outcome") == Some("completed")
        }))
}

fn probe_line_lock(path: &Path) -> Result<LineLockState, DaemonError> {
    let file = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
        .map_err(DaemonError::io)?;
    loop {
        // SAFETY: file owns a live descriptor and flock retains no pointers.
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
            let descriptor = file.metadata().map_err(DaemonError::io)?;
            let pathname = fs::metadata(path).map_err(DaemonError::io)?;
            if descriptor.dev() != pathname.dev() || descriptor.ino() != pathname.ino() {
                return Err(DaemonError::corrupt(
                    "ledger replaced while probing its lock",
                ));
            }
            // SAFETY: this descriptor acquired the lock above.
            if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) } != 0 {
                return Err(DaemonError::io(std::io::Error::last_os_error()));
            }
            return Ok(LineLockState::Free);
        }
        let error = std::io::Error::last_os_error();
        match error.raw_os_error() {
            Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN => {
                return Ok(LineLockState::Busy);
            }
            _ if error.kind() == std::io::ErrorKind::Interrupted => continue,
            _ => return Err(DaemonError::io(error)),
        }
    }
}

fn read_worker_hello(
    reader: &mut BufReader<std::process::ChildStdout>,
    deadline: Instant,
) -> Result<worker_control::Hello, DaemonError> {
    let mut line = Vec::new();
    loop {
        let now = Instant::now();
        // The child may already have written or closed its pipe while the
        // supervisor was freezing the rest of the launch authority.  At the
        // deadline, probe the descriptor once without blocking so a ready
        // EOF remains an early-exit result instead of being misclassified as
        // a timeout under scheduler contention.
        let timeout_ms = if now >= deadline {
            0
        } else {
            let remaining = deadline - now;
            remaining
                .as_millis()
                .saturating_add(u128::from(
                    !remaining.subsec_nanos().is_multiple_of(1_000_000),
                ))
                .clamp(1, i32::MAX as u128) as i32
        };
        let mut descriptor = libc::pollfd {
            fd: reader.get_ref().as_raw_fd(),
            events: libc::POLLIN | libc::POLLHUP,
            revents: 0,
        };
        // SAFETY: descriptor points to one initialized pollfd for this call.
        let status = unsafe { libc::poll(&mut descriptor, 1, timeout_ms) };
        if status == 0 {
            return Err(DaemonError::protocol("worker hello timed out"));
        }
        if status < 0 {
            let error = std::io::Error::last_os_error();
            if error.kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            return Err(DaemonError::io(error));
        }
        if descriptor.revents & libc::POLLNVAL != 0 {
            return Err(DaemonError::protocol(
                "worker stdout became invalid during hello",
            ));
        }
        let (consumed, complete) = {
            let available = reader.fill_buf().map_err(DaemonError::io)?;
            if available.is_empty() {
                return Err(DaemonError::protocol("worker exited before hello"));
            }
            if let Some(index) = available.iter().position(|byte| *byte == b'\n') {
                let consumed = index + 1;
                if line.len().saturating_add(consumed) > WORKER_HELLO_MAX_BYTES {
                    return Err(DaemonError::protocol("worker hello is too large"));
                }
                line.extend_from_slice(&available[..consumed]);
                (consumed, true)
            } else {
                if line.len().saturating_add(available.len()) >= WORKER_HELLO_MAX_BYTES {
                    return Err(DaemonError::protocol("worker hello is too large"));
                }
                line.extend_from_slice(available);
                (available.len(), false)
            }
        };
        reader.consume(consumed);
        if complete {
            return decode_hello(&line).map_err(|error| DaemonError::protocol(error.to_string()));
        }
    }
}

fn next_stop_generation(root: &Path, session_id: &str) -> Result<u64, ProductionRouteFailure> {
    let ledger = root.join("threads").join(session_id).join("main.jsonl");
    let bytes = fs::read(&ledger).map_err(internal)?;
    let projection = scan_valid_prefix(&bytes, schema::READER_VERSION)
        .projection
        .ok_or_else(|| internal("worker ledger has no projection"))?;
    Ok(projection
        .events
        .iter()
        .filter_map(|event| {
            (event.kind() == &schema::EventKind::StopRequested)
                .then(|| event.integer_field("generation"))
                .flatten()
        })
        .max()
        .unwrap_or(0)
        .saturating_add(1))
}

fn validate_worker_binary(path: &Path) -> Result<(), DaemonError> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| DaemonError::invalid_install(path.to_path_buf(), error))?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
        return Err(DaemonError::invalid_install_reason(
            path,
            "worker is not a regular file",
        ));
    }
    Ok(())
}

fn failure(code: &str, message: &str) -> ProductionRouteFailure {
    ProductionRouteFailure::new(
        code,
        message,
        IJsonValue::parse_str("{}").expect("empty object is I-JSON"),
    )
}

fn internal(error: impl std::fmt::Display) -> ProductionRouteFailure {
    failure("internal", &error.to_string())
}

impl From<worker_control::ProtocolError> for DaemonError {
    fn from(error: worker_control::ProtocolError) -> Self {
        DaemonError::protocol(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn client_file_scope_uses_selected_directory_and_rejects_escape() {
        let first = tempfile::tempdir().unwrap();
        let selected = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        for directory in [first.path(), selected.path(), outside.path()] {
            std::fs::write(directory.join("file"), "fixture").unwrap();
        }
        let workspace: profile::ResolvedWorkspace = serde_json::from_value(serde_json::json!({
            "format":1,"revision":1,"id":"w","name":"w","selected_cwd":selected.path(),
            "cwd":[first.path(),selected.path()],"policy":{}
        }))
        .unwrap();
        let (root, relative) = super::scoped_client_file_path(&workspace, "file").unwrap();
        assert_eq!(root, selected.path().canonicalize().unwrap());
        assert_eq!(relative, "file");
        assert!(
            super::scoped_client_file_path(&workspace, first.path().join("file").to_str().unwrap())
                .is_ok()
        );
        assert!(
            super::scoped_client_file_path(
                &workspace,
                outside.path().join("file").to_str().unwrap()
            )
            .is_err()
        );
        assert!(super::scoped_client_file_path(&workspace, "../file").is_err());
        std::os::unix::fs::symlink(outside.path().join("file"), selected.path().join("escape"))
            .unwrap();
        assert!(super::scoped_client_file_path(&workspace, "escape").is_err());
    }
    include!("process_live_tests.rs");
    use std::future::Future;
    use std::io::{Read as _, Write as _};
    use std::net::TcpListener;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::mpsc;
    use std::task::{Context, Poll, Waker};

    use serde_json::{Value, json};

    use super::*;

    fn deepseek_title_provider() -> profile::Provider {
        profile::Provider {
            id: "deepseek-responses".to_owned(),
            name: Some("DeepSeek".to_owned()),
            adapter: "responses".to_owned(),
            dialect: "deepseek_responses_v1".to_owned(),
            endpoint_owner: "deepseek".to_owned(),
            gateway_translation: "direct".to_owned(),
            evidence_revision:
                "deepseek-direct-responses-v4-2026-07-31+function-json-schema-strict-v1".to_owned(),
            endpoint: "https://api.deepseek.com".to_owned(),
            credential_key: Some("deepseek-key".to_owned()),
            // The spelling Tekes actually writes into `providers.json`.
            models: vec![profile::Model {
                id: "deepseek-flash".to_owned(),
                profile: "deepseek_responses_v1:deepseek-flash".to_owned(),
                enabled: true,
                context_window_tokens: 200_000,
                compact_trigger_tokens: 180_000,
            }],
        }
    }

    fn serve_title_once(title: &str) -> (String, std::thread::JoinHandle<Vec<u8>>) {
        serve_title_response(
            "200 OK",
            &json!({
                "id": "title-response",
                "output": [{
                    "content": [{"text": title, "type": "output_text"}],
                    "role": "assistant",
                    "type": "message"
                }],
                "status": "completed"
            }),
        )
    }

    fn serve_title_response(
        status: &'static str,
        response: &Value,
    ) -> (String, std::thread::JoinHandle<Vec<u8>>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind title server");
        let address = listener.local_addr().expect("title server address");
        let body = serde_json::to_vec(response).expect("title response");
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept title request");
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .expect("title read timeout");
            let mut request = Vec::new();
            let mut chunk = [0_u8; 4096];
            let mut expected = None;
            loop {
                let count = stream.read(&mut chunk).expect("read title request");
                if count == 0 {
                    break;
                }
                request.extend_from_slice(&chunk[..count]);
                if expected.is_none()
                    && let Some(end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n")
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
                if expected.is_some_and(|length| request.len() >= length) {
                    break;
                }
            }
            let header = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            stream
                .write_all(header.as_bytes())
                .expect("write title headers");
            stream.write_all(&body).expect("write title response");
            stream.flush().expect("flush title response");
            request
        });
        (format!("http://{address}"), handle)
    }

    #[test]
    fn automatic_title_text_normalization_matches_the_legacy_contract() {
        assert_eq!(
            deterministic_automatic_thread_title("  第一行\n  第二行  "),
            "第一行 第二行"
        );
        assert_eq!(
            normalized_automatic_thread_title("  ## 修复\n登录失败。 "),
            "修复 登录失败"
        );
        assert_eq!(normalized_automatic_thread_title(" # # 标题！！ "), "标题");
        assert_eq!(
            normalized_automatic_thread_title(&"字".repeat(80))
                .chars()
                .count(),
            AUTOMATIC_THREAD_TITLE_MAX_CHARS
        );
    }

    /// A model id belongs to whoever writes `providers.json`. The route picks the
    /// flash model whatever it is called, and still titles the thread when only a
    /// heavier model is enabled — leaving the raw prompt as the name is worse
    /// than spending one short call on it (2026-09-20).
    #[test]
    fn automatic_title_route_finds_flash_by_hint_and_falls_back_to_any_deepseek() {
        fn providers_with(models: Vec<(&str, &str)>) -> profile::ProvidersConfig {
            let mut provider = deepseek_title_provider();
            provider.models = models
                .into_iter()
                .map(|(id, profile_id)| profile::Model {
                    id: id.to_owned(),
                    profile: profile_id.to_owned(),
                    enabled: true,
                    context_window_tokens: 200_000,
                    compact_trigger_tokens: 180_000,
                })
                .collect();
            profile::ProvidersConfig {
                format: 1,
                revision: 1,
                providers: vec![provider],
                web_search: None,
            }
        }

        // Production order: the pro model is listed first, flash second.
        let production = providers_with(vec![
            ("deepseek-v4-pro", "deepseek_responses_v1:deepseek-v4-pro"),
            ("deepseek-flash", "deepseek_responses_v1:deepseek-flash"),
        ]);
        let (_, model, _) = automatic_title_route(&production).expect("flash route");
        assert_eq!(model.id, "deepseek-flash");

        // Named anything, as long as the profile says flash.
        let renamed = providers_with(vec![("fast", "deepseek_responses_v1:deepseek-flash")]);
        let (_, model, _) = automatic_title_route(&renamed).expect("profile-matched route");
        assert_eq!(model.id, "fast");

        // No flash at all: still titled rather than left as the raw prompt.
        let pro_only = providers_with(vec![(
            "deepseek-v4-pro",
            "deepseek_responses_v1:deepseek-v4-pro",
        )]);
        let (_, model, _) = automatic_title_route(&pro_only).expect("fallback route");
        assert_eq!(model.id, "deepseek-v4-pro");
    }

    #[test]
    fn automatic_title_request_uses_configured_deepseek_adapter_and_pins_flash() {
        let providers = profile::ProvidersConfig {
            format: 1,
            revision: 1,
            providers: vec![deepseek_title_provider()],
            web_search: None,
        };
        let (configured, model, resolved) =
            automatic_title_route(&providers).expect("DeepSeek Flash route");
        assert_eq!(model.id, "deepseek-flash");
        assert_eq!(resolved.dialect, provider::DialectId::DeepseekResponsesV1);
        let prepared = prepare_automatic_title_request(
            &configured,
            &resolved,
            "请修复登录问题",
            "automatic-title-test".to_owned(),
        )
        .expect("title request");
        assert_eq!(prepared.url, "https://api.deepseek.com/responses");
        let body: Value = serde_json::from_slice(&prepared.body).expect("request JSON");
        assert_eq!(body["model"], "deepseek-flash");
        assert_eq!(body["stream"], false);
        assert_eq!(body["tools"], json!([]));
        assert_eq!(body["reasoning"], json!({"effort": "none"}));
        assert_eq!(
            body["max_output_tokens"],
            json!(AUTOMATIC_THREAD_TITLE_MAX_OUTPUT_TOKENS)
        );
        assert_eq!(
            body.pointer("/input/0/content/0/text"),
            Some(&json!("请修复登录问题"))
        );
        assert_eq!(body["instructions"], json!(AUTOMATIC_THREAD_TITLE_SYSTEM));
    }

    #[test]
    fn automatic_title_uses_the_provider_runtime_and_normalizes_its_terminal() {
        let root = tempfile::tempdir().expect("process host root");
        let secrets = Arc::new(provider::MemorySecretStore::new());
        secrets
            .publish(
                "deepseek-key",
                provider::SecretRecord::Active {
                    generation: 1,
                    material: "fixture-title-secret".to_owned(),
                },
            )
            .expect("title secret");
        let host = ProductionProcessHost::open_with_secret_store(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
            secrets,
        )
        .expect("process host");
        let (transport, request) = serve_title_once("  ## 修复\n登录失败。 ");
        *host
            .automatic_title_test_redirect
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(transport);
        let providers = profile::ProvidersConfig {
            format: 1,
            revision: 1,
            providers: vec![deepseek_title_provider()],
            web_search: None,
        };
        assert_eq!(
            host.generate_automatic_title(&providers, "请修复登录问题")
                .expect("generated title"),
            "修复 登录失败"
        );
        let request = String::from_utf8(request.join().expect("title server")).expect("HTTP text");
        assert!(request.starts_with("POST /responses HTTP/1.1\r\n"));
        assert!(request.contains("authorization: Bearer fixture-title-secret\r\n"));
        host.shutdown();
    }

    /// The title output cap and provider failures never become a title: the
    /// helper returns an error and the deterministic seed title stays.
    #[test]
    fn automatic_title_rejects_cut_and_failed_responses() {
        let root = tempfile::tempdir().expect("process host root");
        let secrets = Arc::new(provider::MemorySecretStore::new());
        secrets
            .publish(
                "deepseek-key",
                provider::SecretRecord::Active {
                    generation: 1,
                    material: "fixture-title-secret".to_owned(),
                },
            )
            .expect("title secret");
        let host = ProductionProcessHost::open_with_secret_store(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
            secrets,
        )
        .expect("process host");
        let providers = profile::ProvidersConfig {
            format: 1,
            revision: 1,
            providers: vec![deepseek_title_provider()],
            web_search: None,
        };
        let cut = json!({
            "id": "title-response",
            "incomplete_details": {"reason": "max_output_tokens"},
            "output": [{
                "content": [{"text": "```python\nclass WeightedTTLCache:", "type": "output_text"}],
                "role": "assistant",
                "type": "message"
            }],
            "status": "incomplete"
        });
        let failed = json!({"error": {"message": "upstream failure", "type": "server_error"}});
        for (status, response) in [("200 OK", cut), ("500 Internal Server Error", failed)] {
            let (transport, request) = serve_title_response(status, &response);
            *host
                .automatic_title_test_redirect
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(transport);
            assert!(
                host.generate_automatic_title(&providers, "Implement a weighted TTL cache")
                    .is_err(),
                "{status} must not produce a title"
            );
            request.join().expect("title server");
        }
        host.shutdown();
    }

    #[test]
    fn automatic_title_request_fails_closed_without_a_reasoning_off_switch() {
        let configured = profile::Provider {
            id: "openai".to_owned(),
            name: None,
            adapter: "responses".to_owned(),
            dialect: "openai_responses_v1".to_owned(),
            endpoint_owner: "openai".to_owned(),
            gateway_translation: "direct".to_owned(),
            evidence_revision: "openai-2026-08-01".to_owned(),
            endpoint: "https://api.openai.com/v1".to_owned(),
            credential_key: None,
            models: vec![profile::Model {
                id: "gpt-5".to_owned(),
                profile: "openai_responses_v1:gpt-5".to_owned(),
                enabled: true,
                context_window_tokens: 200_000,
                compact_trigger_tokens: 180_000,
            }],
        };
        let resolved =
            provider::resolve_profile(&configured, &configured.models[0]).expect("openai profile");
        assert!(
            prepare_automatic_title_request(&configured, &resolved, "title me", "t".to_owned())
                .is_err(),
            "a dialect without a verified reasoning-off switch must not send a title request"
        );
    }

    /// Manual compaction on an idle line is authored by the supervisor under
    /// the line lock (D-61 whitelist): checkpoint, then an origin-keyed
    /// `compact` covering the settled history; a retry with the same origin is
    /// the original receipt and appends nothing.
    #[test]
    fn locked_manual_compact_covers_settled_history_and_is_keyed_by_origin() {
        let root = tempfile::tempdir().expect("process host root");
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        let session = "018f0000-0000-7000-8000-00000000c0c0";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("session folder");
        write_test_genesis(&folder.join("main.jsonl"), session);
        let system = AssetStore::new(folder.join("assets"))
            .expect("assets")
            .publish(b"system")
            .expect("system asset");
        {
            let mut ledger =
                store::LockedLedger::open(folder.join("main.jsonl"), schema::READER_VERSION)
                    .expect("ledger");
            for value in [
                json!({"v":1,"seq":2,"kind":"input","ts":"2026-09-05T00:00:01.000Z","content":[{"type":"text","text":"PRE"}],"origin_key":"input-1","origin_tuple":{"principal":"user","client":"test","target":session,"op":"prompt","key":"input-1"}}),
                json!({"v":1,"seq":3,"turn":1,"kind":"turn_open","ts":"2026-09-05T00:00:01.000Z","trigger":{"inputs":[2]}}),
                json!({"v":1,"seq":4,"kind":"epoch","ts":"2026-09-05T00:00:01.000Z","id":"e1","reason":"initial","adapter":"openai_responses_v1","model":"m","system":{"asset":system.asset,"digest":"d"},"tools":{"asset":"sha256-td","digest":"td"},"renderer":1}),
                json!({"v":1,"seq":5,"turn":1,"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"ts":"2026-09-05T00:00:01.000Z","attempt":"a1","epoch":"e1","wire_digest":"w","admits":[{"from":2,"to":2}]}),
                json!({"v":1,"seq":6,"turn":1,"kind":"tool_call","ts":"2026-09-05T00:00:01.000Z","attempt":"a1","call":"skill-1","name":"skill","args":{"name":"uat-format"},"source":"provider"}),
                json!({"v":1,"seq":7,"turn":1,"kind":"output","ts":"2026-09-05T00:00:01.000Z","attempt":"a1","content":[],"sealed":{"version":1,"adapter":"openai_responses_v1","fragments":"[]"},"usage":{"availability":"unavailable"}}),
                json!({"v":1,"seq":8,"turn":1,"kind":"tool_result","ts":"2026-09-05T00:00:01.000Z","call":"skill-1","outcome":"ok","content":[{"type":"text","text":"loaded"}]}),
                json!({"v":1,"seq":9,"turn":1,"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"ts":"2026-09-05T00:00:02.000Z","attempt":"a2","epoch":"e1","wire_digest":"w","admits":[]}),
                json!({"v":1,"seq":10,"turn":1,"kind":"output","ts":"2026-09-05T00:00:02.000Z","attempt":"a2","final_answer":true,"content":[{"type":"text","text":"SKILL_PRE_COMPACT_OK"}],"sealed":{"version":1,"adapter":"openai_responses_v1","fragments":"[]"},"usage":{"availability":"unavailable"}}),
                json!({"v":1,"seq":11,"turn":1,"kind":"settle","ts":"2026-09-05T00:00:02.000Z","outcome":"completed"}),
            ] {
                let event = schema::Event::decode(&serde_json::to_vec(&value).expect("event JSON"))
                    .expect("event");
                ledger
                    .append_contract(event, store::BarrierContext::default())
                    .expect("append");
            }
        }
        let origin = OriginTuple {
            principal: "uid:501".to_owned(),
            client: "tekes-client-resource".to_owned(),
            target: session.to_owned(),
            op: "commands/run".to_owned(),
            key: "compact-1".to_owned(),
        };
        let receipt = host
            .locked_compact(session, "2026-09-05T00:00:03.000Z", &origin)
            .expect("manual compact");
        assert!(!receipt.deduplicated);
        let ledger = fs::read_to_string(folder.join("main.jsonl")).expect("ledger");
        let events: Vec<Value> = ledger
            .lines()
            .map(|line| serde_json::from_str(line).expect("event"))
            .collect();
        let compact = events
            .iter()
            .find(|event| event["kind"] == "compact")
            .expect("compact event");
        assert_eq!(compact["seq"].as_u64(), Some(receipt.seq));
        assert_eq!(compact["origin_tuple"]["op"], "commands/run");
        assert_eq!(compact["origin_key"], "compact-1");
        assert!(
            compact["covers"]
                .as_array()
                .is_some_and(|covers| !covers.is_empty()),
            "settled history is covered: {compact}"
        );
        assert!(
            events
                .iter()
                .any(|event| event["kind"] == "checkpoint"
                    && event["seq"].as_u64() < Some(receipt.seq)),
            "checkpoint precedes the compact"
        );
        let repeat = host
            .locked_compact(session, "2026-09-05T00:00:04.000Z", &origin)
            .expect("repeat");
        assert!(repeat.deduplicated);
        assert_eq!(repeat.seq, receipt.seq);
        assert_eq!(
            fs::read_to_string(folder.join("main.jsonl")).expect("ledger"),
            ledger,
            "a repeated origin appends nothing"
        );
        // The reopened ledger is valid and its next epoch reason will be compaction.
        let projection = schema::validate_ledger(ledger.as_bytes(), schema::READER_VERSION)
            .expect("valid ledger");
        assert!(
            projection
                .events
                .iter()
                .any(|event| *event.kind() == EventKind::Compact)
        );
        host.shutdown();
    }

    #[test]
    fn worker_failure_preserves_first_root_cause() {
        let mut failure = None;

        preserve_first_failure(&mut failure, "publish frame failed".to_owned());
        preserve_first_failure(&mut failure, "worker exited".to_owned());

        assert_eq!(failure.as_deref(), Some("publish frame failed"));
    }

    #[test]
    fn restart_backoff_is_reason_scoped_bounded_and_user_resettable() {
        let root = tempfile::tempdir().expect("process host root");
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        let session = "session-backoff";

        assert_eq!(
            host.note_restart_failure(session, session, b"same failure"),
            Some(Duration::from_secs(1))
        );
        assert_eq!(
            host.note_restart_failure(session, session, b"same failure"),
            Some(Duration::from_secs(2))
        );
        assert_eq!(
            host.note_restart_failure(session, session, b"same failure"),
            Some(Duration::from_secs(4))
        );
        assert_eq!(
            host.note_restart_failure(session, session, b"same failure"),
            Some(Duration::from_secs(8))
        );
        assert_eq!(
            host.note_restart_failure(session, session, b"same failure"),
            None
        );
        assert!(!host.restart_is_due(session));

        assert_eq!(
            host.note_restart_failure(session, session, b"different failure"),
            Some(Duration::from_secs(1))
        );
        host.reset_restart_backoff(session);
        assert!(host.restart_is_due(session));
        host.shutdown();
    }

    #[test]
    fn unavailable_credential_refresh_retains_last_authoritative_generation() {
        let scope = provider::CredentialScope {
            credential_id: "provider-key".to_owned(),
            adapter: "responses".to_owned(),
            endpoint_origin: "https://api.example.test".to_owned(),
            purpose: "provider".to_owned(),
            generation: "7".to_owned(),
            material: "fixture-secret-never-log".to_owned(),
        };
        let previous = provider::ResolvedCredentialBindings {
            active: vec![scope.clone()],
            revoked: Vec::new(),
            availability: [(
                "provider-key".to_owned(),
                provider::CredentialAvailability::Active { generation: 7 },
            )]
            .into_iter()
            .collect(),
        };
        let mut current = provider::ResolvedCredentialBindings {
            active: Vec::new(),
            revoked: Vec::new(),
            availability: [(
                "provider-key".to_owned(),
                provider::CredentialAvailability::Unavailable,
            )]
            .into_iter()
            .collect(),
        };

        retain_prior_for_unknown_credentials(&previous, &mut current);

        assert_eq!(current.availability, previous.availability);
        assert_eq!(current.active, vec![scope]);
        assert!(current.revoked.is_empty());
    }

    #[test]
    fn mcp_launch_notices_name_the_skipped_server_and_its_reason() {
        let reference = mcp::McpServerReference {
            workspace_id: "ws".to_owned(),
            scope: mcp::McpScope::User,
            name: "dxf-editor".to_owned(),
        };
        let notices = mcp_launch_notices(
            None,
            &[
                crate::mcp_runtime::McpServerFailure {
                    server: reference.clone(),
                    code: "projection-failed",
                    detail: Some(
                        "MCP schema projection failed: tool get_entities: unsupported keyword"
                            .to_owned(),
                    ),
                },
                crate::mcp_runtime::McpServerFailure {
                    server: mcp::McpServerReference {
                        name: "offline".to_owned(),
                        ..reference
                    },
                    code: "refresh-failed",
                    detail: None,
                },
            ],
            &[],
            &[],
        );
        assert_eq!(notices.len(), 2);
        assert_eq!(notices[0].classification, "mcp_server");
        assert_eq!(
            notices[0].message,
            "MCP server dxf-editor was skipped for this run: MCP schema projection failed: tool get_entities: unsupported keyword"
        );
        assert_eq!(
            notices[1].message,
            "MCP server offline was skipped for this run: refresh-failed"
        );
    }

    #[test]
    fn refused_tools_produce_one_bounded_notice_per_healthy_server() {
        let reference = mcp::McpServerReference {
            workspace_id: "ws".to_owned(),
            scope: mcp::McpScope::User,
            name: "probe".to_owned(),
        };
        let refusal = |index: usize| crate::mcp_runtime::McpRefusedTool {
            server: reference.clone(),
            refusal: mcp::McpToolRefusal {
                tool: format!("tool{index}"),
                name: Some(format!("mcp__probe__tool{index}")),
                code: "schema-refused",
                pointer: Some("/parameters/properties/operand".to_owned()),
                keyword: Some("maxProperties".to_owned()),
                reason: "uses unsupported keyword \"maxProperties\"".to_owned(),
            },
        };
        let one = mcp_launch_notices(None, &[], &[refusal(0)], &[]);
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].classification, "mcp_tool");
        assert_eq!(
            one[0].message,
            "MCP server probe is available but 1 of its tools were skipped for this run: tool0 at /parameters/properties/operand: uses unsupported keyword \"maxProperties\""
        );
        let many = (0..MAX_NOTICE_TOOLS + 3).map(refusal).collect::<Vec<_>>();
        // The policy names the last tool: it is listed first, not cut.
        let last = format!("mcp__probe__tool{}", MAX_NOTICE_TOOLS + 2);
        let notices = mcp_launch_notices(None, &[], &many, std::slice::from_ref(&last));
        assert_eq!(notices.len(), 1, "one notice per server, not per tool");
        let message = &notices[0].message;
        assert!(message.ends_with("; and 3 more"), "{message}");
        assert!(
            message.contains(&format!("{} of its tools", MAX_NOTICE_TOOLS + 3)),
            "{message}"
        );
        assert!(message.contains(&format!("tool{} at", MAX_NOTICE_TOOLS + 2)));
        assert!(!message.contains("tool7 at"), "{message}");
        assert_eq!(message.matches(" at /parameters").count(), MAX_NOTICE_TOOLS);
    }

    #[cfg(target_os = "macos")]
    fn write_catalog_server(script: &Path, tools_json: &str) {
        fs::write(
            script,
            format!(
                "#!/bin/sh\nwhile IFS= read -r line; do\ncase \"$line\" in\n*'\"method\":\"initialize\"'*) printf '%s\\n' '{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{{\"protocolVersion\":\"{}\",\"capabilities\":{{\"tools\":{{}}}},\"serverInfo\":{{\"name\":\"catalog\",\"version\":\"1\"}}}}}}' ;;\n*'\"method\":\"tools/list\"'*) printf '%s\\n' '{{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{{\"tools\":{tools_json}}}}}' ;;\nesac\ndone\n",
                mcp::LEGACY_PROTOCOL_VERSION
            ),
        )
        .expect("catalog server");
        fs::set_permissions(script, fs::Permissions::from_mode(0o755)).expect("script mode");
    }

    /// Runs one real worker launch (the worker itself exits at once) under
    /// `allowed_tools` and returns the stranded-input notice message. A launch
    /// whose bindings were refused says so; one that reached the worker does not.
    #[cfg(target_os = "macos")]
    fn launch_notice_for_policy(servers: &[(&str, String)], allowed_tools: &[&str]) -> String {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspace config");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({"format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":allowed_tools,"network":false,"writable_roots":[]}}),
        );
        let session = "018f0000-0000-7000-8000-000000000121";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let ledger = folder.join("main.jsonl");
        write_test_genesis(&ledger, session);
        let worker = root.path().join("failing-worker");
        fs::write(&worker, b"#!/bin/sh\nexit 1\n").expect("failing worker");
        fs::set_permissions(&worker, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");
        let host = ProductionProcessHost::open(
            root.path(),
            worker,
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        for (name, tools_json) in servers {
            let script = root.path().join(format!("{name}-server"));
            write_catalog_server(&script, tools_json);
            host.mcp_runtime
                .mutate(
                    &format!("save-{name}"),
                    &mcp::McpManagementMutation::Save {
                        server: mcp::McpServerConfig {
                            reference: mcp::McpServerReference {
                                workspace_id: "ws".to_owned(),
                                scope: mcp::McpScope::User,
                                name: (*name).to_owned(),
                            },
                            transport: mcp::McpTransportConfig::Stdio {
                                command: vec![script.to_string_lossy().into_owned()],
                                cwd: None,
                                environment: BTreeMap::new(),
                            },
                            enabled: true,
                            always_on: false,
                            protocol_mode: mcp::ProtocolMode::Legacy,
                            owner: None,
                            plugin_component: None,
                            project_trusted: true,
                        },
                        credential_fields: BTreeMap::new(),
                    },
                )
                .expect("save server");
        }
        let origin = OriginTuple {
            principal: "test".to_owned(),
            client: "test".to_owned(),
            target: session.to_owned(),
            op: "session.prompt".to_owned(),
            key: "rpc-launch-probe".to_owned(),
        };
        let prompt = MaterializedPrompt {
            blocks: vec![Block::Text {
                text: "launch probe".to_owned(),
            }],
            attachments: Vec::new(),
            files: Vec::new(),
        };
        host.locked_prompt(session, "2026-08-28T00:00:01.000Z", &origin, &prompt, false)
            .expect("durable prompt");
        let notices = session_notices(&fs::read(&ledger).expect("ledger"));
        let launch = notices
            .iter()
            .find(|notice| notice["classification"] == "worker_launch")
            .unwrap_or_else(|| panic!("no worker_launch notice: {notices:?}"));
        launch["message"].as_str().unwrap_or_default().to_owned()
    }

    /// set-accepted implies launch-accepted: every policy below is one
    /// `validate_workspace_policy_candidate` accepts, and the real launch gets
    /// past the bindings (it then fails only because the stub worker exits). The
    /// control shows the probe can tell a refused binding from a started worker.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_policy_set_accepts_never_breaks_the_launch_binding() {
        let bad = |name: &str| {
            format!(
                r#"{{"name":"{name}","description":"Bad","annotations":{{"readOnlyHint":true}},"inputSchema":{{"type":"object","properties":{{"o":{{"type":"object","maxProperties":2,"properties":{{}}}}}},"required":[],"additionalProperties":false}}}}"#
            )
        };
        let good = |name: &str| {
            format!(
                r#"{{"name":"{name}","description":"Good","annotations":{{"readOnlyHint":true}},"inputSchema":{{"type":"object","properties":{{}},"required":[],"additionalProperties":false}}}}"#
            )
        };
        let binding_refused =
            |message: &str| message.contains("launch bindings could not be derived");
        // Control: an unknown name is refused at launch, as at set.
        let message = launch_notice_for_policy(
            &[("probe", format!("[{}]", good("alpha")))],
            &["mcp__probe__nope"],
        );
        assert!(binding_refused(&message), "{message}");
        // An ordinary refused tool.
        let message = launch_notice_for_policy(
            &[("probe", format!("[{},{}]", good("alpha"), bad("beta")))],
            &["mcp__probe__alpha", "mcp__probe__beta"],
        );
        assert!(!binding_refused(&message), "{message}");
        // A refused tool whose projected name collides with a healthy tool's.
        let message = launch_notice_for_policy(
            &[
                ("a", format!("[{}]", bad("b__c"))),
                ("a__b", format!("[{}]", good("c"))),
            ],
            &["mcp__a__b__c"],
        );
        assert!(!binding_refused(&message), "{message}");
        // A refused tool whose projected name is too long for a launch name
        // does not disturb the other tools' launch (a policy cannot name it:
        // `set` refuses it as unknown).
        let long = "x".repeat(130);
        let message = launch_notice_for_policy(
            &[("probe", format!("[{},{}]", good("alpha"), bad(&long)))],
            &["mcp__probe__alpha"],
        );
        assert!(!binding_refused(&message), "{message}");
    }

    #[test]
    fn unavailable_mcp_server_fails_launch_only_when_policy_names_its_tools() {
        let failures = vec![crate::mcp_runtime::McpServerFailure {
            server: mcp::McpServerReference {
                workspace_id: "ws".to_owned(),
                scope: mcp::McpScope::User,
                name: "offline server".to_owned(),
            },
            code: "refresh-failed",
            detail: None,
        }];
        assert!(!mcp_failure_is_required(&failures, &[]));
        assert!(!mcp_failure_is_required(
            &failures,
            &["mcp__other__tool".to_owned()]
        ));
        assert!(mcp_failure_is_required(
            &failures,
            &["mcp__offline_20server__tool".to_owned()]
        ));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn durable_prompt_receipt_survives_spawn_failure_and_exact_retry_deduplicates() {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspace config");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({"format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}}),
        );
        let session = "018f0000-0000-7000-8000-000000000120";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let ledger = folder.join("main.jsonl");
        write_test_genesis(&ledger, session);
        let worker = root.path().join("failing-worker");
        fs::write(&worker, b"#!/bin/sh\nexit 1\n").expect("failing worker");
        fs::set_permissions(&worker, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");
        let host = ProductionProcessHost::open(
            root.path(),
            worker,
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        let origin = OriginTuple {
            principal: "test".to_owned(),
            client: "test".to_owned(),
            target: session.to_owned(),
            op: "session.prompt".to_owned(),
            key: "rpc-durable-spawn-failure".to_owned(),
        };
        let prompt = MaterializedPrompt {
            blocks: vec![Block::Text {
                text: "durable before spawn".to_owned(),
            }],
            attachments: Vec::new(),
            files: Vec::new(),
        };

        let first = host
            .locked_prompt(session, "2026-08-28T00:00:01.000Z", &origin, &prompt, false)
            .expect("durable prompt is accepted despite spawn failure");
        let retry = host
            .locked_prompt(session, "2026-08-28T00:00:01.000Z", &origin, &prompt, false)
            .expect("exact retry returns durable receipt");

        assert!(!first.deduplicated);
        assert!(retry.deduplicated);
        assert_eq!(retry.seq, first.seq);
        let bytes = fs::read(&ledger).expect("durable ledger");
        assert_eq!(
            bytes
                .windows(b"\"kind\":\"input\"".len())
                .filter(|window| *window == b"\"kind\":\"input\"")
                .count(),
            1
        );
        // The stranded input is visible in the transcript: one error notice,
        // keyed by the input seq, so the exact retry did not add a second.
        let notices = session_notices(&bytes);
        assert_eq!(notices.len(), 1, "{notices:?}");
        assert_eq!(notices[0]["severity"], "error");
        assert_eq!(notices[0]["classification"], "worker_launch");
        assert_eq!(notices[0]["operation"], "prompt");
        assert!(
            notices[0]["message"]
                .as_str()
                .unwrap_or_default()
                .starts_with("The worker could not be started: "),
            "{notices:?}"
        );
        host.shutdown();
    }

    /// An unreadable `config/mcp-servers.json` (the observed production case
    /// was a hand edit that dropped the final newline and left a non-canonical
    /// body) must not strand the input: the launch proceeds without the
    /// registry's servers and the session shows a warning naming the file.
    #[cfg(target_os = "macos")]
    #[test]
    fn broken_mcp_registry_degrades_the_launch_and_warns_in_the_session() {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspace config");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({"format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}}),
        );
        fs::create_dir_all(root.path().join("config")).expect("config dir");
        fs::write(
            root.path().join("config/mcp-servers.json"),
            b"{\"format\":1,\"servers\":[]} ",
        )
        .expect("broken registry");
        let session = "018f0000-0000-7000-8000-000000000121";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let ledger = folder.join("main.jsonl");
        write_test_genesis(&ledger, session);
        let worker = root.path().join("failing-worker");
        fs::write(&worker, b"#!/bin/sh\nexit 1\n").expect("failing worker");
        fs::set_permissions(&worker, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let host = ProductionProcessHost::open(
            root.path(),
            worker,
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        let origin = OriginTuple {
            principal: "test".to_owned(),
            client: "test".to_owned(),
            target: session.to_owned(),
            op: "session.prompt".to_owned(),
            key: "rpc-broken-registry".to_owned(),
        };
        let prompt = MaterializedPrompt {
            blocks: vec![Block::Text {
                text: "run without mcp".to_owned(),
            }],
            attachments: Vec::new(),
            files: Vec::new(),
        };
        host.locked_prompt(session, "2026-09-20T00:00:01.000Z", &origin, &prompt, false)
            .expect("prompt is accepted");
        let notices = session_notices(&fs::read(&ledger).expect("ledger"));
        // The launch got past binding resolution (the registry warning was
        // recorded) and only then failed on the fake worker binary.
        let warning = notices
            .iter()
            .find(|notice| notice["severity"] == "warning")
            .unwrap_or_else(|| panic!("registry warning is recorded: {notices:?}"));
        assert_eq!(warning["classification"], "mcp_registry");
        let message = warning["message"].as_str().expect("message");
        assert!(message.contains("mcp-servers.json"), "{message}");
        assert!(
            notices.iter().any(|notice| notice["severity"] == "error"
                && notice["classification"] == "worker_launch"),
            "{notices:?}"
        );
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    fn session_notices(ledger: &[u8]) -> Vec<serde_json::Value> {
        ledger
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .filter_map(|line| serde_json::from_slice::<serde_json::Value>(line).ok())
            .filter(|event| event["kind"] == "meta")
            .filter_map(|event| event.get("notice").cloned())
            .collect()
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn late_provider_frame_does_not_reopen_sealed_output() {
        const SESSION: &str = "018f0000-0000-7000-8000-000000000003";
        let root = tempfile::tempdir().unwrap();
        let folder = root.path().join("threads").join(SESSION);
        fs::create_dir_all(folder.join("assets")).unwrap();
        fs::write(
            folder.join("main.jsonl"),
            include_str!("../../../fixtures/endpoint/projection-source.jsonl"),
        )
        .unwrap();
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().unwrap(),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .unwrap();
        host.publish_appended(SESSION).unwrap();
        let before = fs::read(folder.join("endpoint.jsonl")).unwrap();
        host.publish_frame(
            SESSION,
            Frame {
                arguments_complete: None,
                ledger_seq: None,
                attempt: "a1".into(),
                channel: FrameChannel::Text,
                block: 0,
                delta: "late text".into(),
                call_id: None,
                name: None,
            },
        )
        .unwrap();
        host.publish_appended(SESSION).unwrap();
        assert_eq!(fs::read(folder.join("endpoint.jsonl")).unwrap(), before);
    }

    /// Eager dispatch appends a `tool_call`/`tool_result` for the streaming
    /// attempt and rings the doorbell while later presentation frames are still
    /// to come. Once those records are journaled, a frame stamped at or after
    /// them must still be published: the causal cut is the frame's own
    /// `ledger_seq`, not the attempt's first tool record.
    #[cfg(target_os = "macos")]
    #[test]
    fn eager_tool_records_published_by_doorbell_do_not_drop_later_frames() {
        const SESSION: &str = "018f0000-0000-7000-8000-000000000003";
        let root = tempfile::tempdir().unwrap();
        let folder = root.path().join("threads").join(SESSION);
        fs::create_dir_all(folder.join("assets")).unwrap();
        let source = include_str!("../../../fixtures/endpoint/projection-source.jsonl");
        let mut ledger = source.lines().take(6).collect::<Vec<_>>().join("\n") + "\n";
        ledger.push_str(r#"{"args":{"thought":"eager"},"attempt":"a1","call":"c1","kind":"tool_call","name":"think","seq":7,"source":"provider","ts":"2026-08-26T09:00:01.000Z","turn":1,"v":1}"#);
        ledger.push('\n');
        ledger.push_str(r#"{"call":"c1","content":[{"text":"recorded","type":"text"}],"kind":"tool_result","outcome":"ok","seq":8,"ts":"2026-08-26T09:00:01.000Z","turn":1,"v":1}"#);
        ledger.push('\n');
        fs::write(folder.join("main.jsonl"), ledger).unwrap();
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().unwrap(),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .unwrap();
        host.publish_appended(SESSION).unwrap();
        let journal = EndpointJournal::open(&folder).unwrap();
        assert!(
            journal
                .records()
                .unwrap()
                .iter()
                .any(|record| record.kernel_seqs.contains(&8)),
            "the doorbell published the eager result before the next frame"
        );

        host.publish_frame(
            SESSION,
            Frame {
                arguments_complete: None,
                ledger_seq: Some(8),
                attempt: "a1".into(),
                channel: FrameChannel::Text,
                block: 0,
                delta: "after the tool".into(),
                call_id: None,
                name: None,
            },
        )
        .unwrap();
        // Chunks are transient: nothing is journaled for the frame.
        assert!(
            journal
                .records()
                .unwrap()
                .iter()
                .all(|record| record.event.event_type != "assistant/chunk")
        );

        // A frame stamped before the doorbell's records is still published:
        // the projector catches up to the journal instead of reopening or
        // dropping, because the attempt has no terminal yet.
        host.publish_frame(
            SESSION,
            Frame {
                arguments_complete: None,
                ledger_seq: Some(6),
                attempt: "a1".into(),
                channel: FrameChannel::Text,
                block: 1,
                delta: "stale stamp".into(),
                call_id: None,
                name: None,
            },
        )
        .unwrap();
        assert!(
            EndpointJournal::open(&folder)
                .unwrap()
                .records()
                .unwrap()
                .iter()
                .all(|record| record.event.event_type != "assistant/chunk")
        );
    }

    /// A stamped frame that arrives after its attempt's output was published
    /// is late: the cut never falls behind the journal, so the sealed output
    /// stays closed exactly as for a legacy frame.
    #[cfg(target_os = "macos")]
    #[test]
    fn late_stamped_frame_does_not_reopen_sealed_output() {
        const SESSION: &str = "018f0000-0000-7000-8000-000000000003";
        let root = tempfile::tempdir().unwrap();
        let folder = root.path().join("threads").join(SESSION);
        fs::create_dir_all(folder.join("assets")).unwrap();
        fs::write(
            folder.join("main.jsonl"),
            include_str!("../../../fixtures/endpoint/projection-source.jsonl"),
        )
        .unwrap();
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().unwrap(),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .unwrap();
        host.publish_appended(SESSION).unwrap();
        let before = fs::read(folder.join("endpoint.jsonl")).unwrap();
        host.publish_frame(
            SESSION,
            Frame {
                arguments_complete: None,
                ledger_seq: Some(6),
                attempt: "a1".into(),
                channel: FrameChannel::Text,
                block: 0,
                delta: "late text".into(),
                call_id: None,
                name: None,
            },
        )
        .unwrap();
        host.publish_appended(SESSION).unwrap();
        assert_eq!(fs::read(folder.join("endpoint.jsonl")).unwrap(), before);
    }

    #[test]
    fn provider_frame_reaches_transient_sink_and_is_never_journaled() {
        const SESSION: &str = "018f0000-0000-7000-8000-000000000003";
        let root = tempfile::tempdir().expect("process host root");
        let folder = root.path().join("threads").join(SESSION);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let source = include_str!("../../../fixtures/endpoint/projection-source.jsonl");
        let active_attempt = source.lines().take(6).collect::<Vec<_>>().join("\n") + "\n";
        fs::write(folder.join("main.jsonl"), active_attempt).expect("active attempt ledger");

        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        host.publish_appended(SESSION)
            .expect("prime semantic projection before subscribing");

        let streams = ProductionCarrierStreams::new(root.path());
        let mut mux = block_on_ready(endpoint::CarrierStreamHandler::open_stream(
            &streams,
            endpoint::StreamChannel::Mux,
        ))
        .expect("mux stream");
        host.attach_streams(streams.clone());
        // A new mux generation starts empty; the session registers when it is attached (the
        // same call a first publication makes lazily) and its baseline follows.
        streams.attach_session(SESSION).expect("attach the session");
        let baseline = block_on_ready(mux.recv())
            .expect("baseline item")
            .expect("baseline frame");
        assert_eq!(baseline.method, "session/subscribed");

        // The worker has already appended its terminal while these earlier
        // pipe frames are still unread. Frame cache refresh must not publish it.
        fs::write(folder.join("main.jsonl"), source).unwrap();

        host.publish_frame(
            SESSION,
            Frame {
                arguments_complete: None,
                ledger_seq: None,
                attempt: "a1".to_owned(),
                channel: FrameChannel::Reasoning,
                block: 0,
                delta: "live reasoning".to_owned(),
                call_id: None,
                name: None,
            },
        )
        .expect("publish provider frame");

        let transient = block_on_ready(mux.recv())
            .expect("transient item")
            .expect("transient frame");
        assert_eq!(transient.method, "session/transient");
        let transient_payload: Value = serde_json::from_slice(
            &transient
                .payload
                .canonical_bytes()
                .expect("transient payload"),
        )
        .expect("transient JSON");
        assert_eq!(transient_payload["event"]["type"], "assistant/chunk");
        assert_eq!(
            transient_payload["event"]["data"]["chunk"]["text"],
            "live reasoning"
        );

        // Chunks are transient: the journal never records them, and the
        // terminal output projects on the next doorbell.
        host.publish_appended(SESSION).unwrap();
        let journal = EndpointJournal::open(&folder).expect("reopen endpoint journal");
        let records = journal.records().unwrap();
        assert!(
            records
                .iter()
                .all(|record| record.event.event_type != "assistant/chunk")
        );
        assert!(
            records.iter().any(|record| record.kernel_seqs.contains(&8)),
            "the sealed output is projected"
        );
        host.shutdown();
    }

    fn block_on_ready<F: Future>(future: F) -> F::Output {
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let mut future = std::pin::pin!(future);
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("future unexpectedly pending"),
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn appended_events_are_projected_before_any_endpoint_stream_is_attached() {
        let root = tempfile::tempdir().expect("process host root");
        let session = "018f0000-0000-7000-8000-000000000109";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        write_test_genesis(&folder.join("main.jsonl"), session);
        append_test_input(&folder.join("main.jsonl"), session);
        for value in [
            json!({
                "v":1,"seq":3,"kind":"turn_open","ts":"2026-08-28T00:00:02.000Z",
                "turn":1,"trigger":{"inputs":[2]}
            }),
            json!({
                "v":1,"seq":4,"kind":"error","ts":"2026-08-28T00:00:03.000Z",
                "turn":1,"classification":"provider_terminal","detail":"network_disabled",
                "recoverable":false
            }),
        ] {
            let event = schema::Event::decode(&serde_json::to_vec(&value).expect("event JSON"))
                .expect("semantic event");
            let mut bytes = event.canonical_bytes().expect("canonical event");
            bytes.push(b'\n');
            fs::OpenOptions::new()
                .append(true)
                .open(folder.join("main.jsonl"))
                .expect("open ledger")
                .write_all(&bytes)
                .expect("append semantic event");
        }
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");

        host.publish_appended(session)
            .expect("projection must not require a live subscriber");

        let endpoint = EndpointJournal::open(&folder).expect("endpoint journal");
        assert!(
            !endpoint.records().expect("endpoint records").is_empty(),
            "semantic history must be durable before a client subscribes"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn opening_a_thread_repairs_a_terminal_main_tail_missing_from_the_endpoint_journal() {
        const SESSION: &str = "018f0000-0000-7000-8000-000000000003";
        let root = tempfile::tempdir().expect("process host root");
        let folder = root.path().join("threads").join(SESSION);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let source = include_str!("../../../fixtures/endpoint/projection-source.jsonl");
        let lines = source.lines().collect::<Vec<_>>();
        fs::write(folder.join("main.jsonl"), lines[..6].join("\n") + "\n")
            .expect("active attempt ledger");
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        host.publish_appended(SESSION)
            .expect("project active attempt prefix");
        host.publish_frame(
            SESSION,
            Frame {
                arguments_complete: None,
                ledger_seq: None,
                attempt: "a1".to_owned(),
                channel: FrameChannel::Text,
                block: 0,
                delta: "done".to_owned(),
                call_id: None,
                name: None,
            },
        )
        .expect("stream frame");
        fs::write(folder.join("main.jsonl"), source).expect("terminal semantic tail");

        // Lazy boot: opening the thread (`reconcile_projection`, which every journal open runs
        // first) -- not a boot sweep -- brings its journal up to its ledger.
        SupervisorSessionAuthority::reconcile_projection(&*host, SESSION)
            .expect("open-time projection recovery");

        let endpoint = EndpointJournal::open(&folder).expect("endpoint journal");
        let event_types = endpoint
            .records()
            .expect("endpoint records")
            .into_iter()
            .map(|record| record.event.event_type)
            .collect::<Vec<_>>();
        assert_eq!(
            &event_types[event_types.len() - 3..],
            ["assistant/message", "step/end", "turn/end"]
        );
        assert!(host.live_worker(SESSION).is_none());
        assert_eq!(
            probe_line_lock(&folder.join("main.jsonl")).expect("line lock"),
            LineLockState::Free
        );
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn attached_streams_report_inventory_running_from_the_line_lock_on_status_frames() {
        let root = tempfile::tempdir().expect("process host root");
        let session = "018f0000-0000-7000-8000-000000000110";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[root.path().join("workspace").to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        write_test_genesis(&folder.join("main.jsonl"), session);
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        let streams = ProductionCarrierStreams::new(root.path());
        host.attach_streams(streams.clone());
        let mut inventory = streams
            .open_mux(1, endpoint::SessionStreamTarget::SessionInventory)
            .expect("inventory stream");
        let endpoint::SessionSyncFrame::InventoryBaseline { items, .. } =
            block_on_ready(inventory.recv())
                .expect("baseline item")
                .expect("baseline frame")
        else {
            panic!("expected inventory baseline")
        };
        assert_eq!(items.len(), 1);
        assert!(!items[0].running);
        assert_eq!(items[0].tail, "settled");

        let held = LockedLedger::open(folder.join("main.jsonl"), schema::READER_VERSION)
            .expect("hold the line lock");
        host.publish_session_status(session);
        let endpoint::SessionSyncFrame::InventoryUpsert {
            session: summary, ..
        } = block_on_ready(inventory.recv())
            .expect("running upsert item")
            .expect("running upsert")
        else {
            panic!("expected an inventory upsert while the line lock is held")
        };
        assert!(summary.running);
        assert_eq!(summary.tail, "running");

        drop(held);
        host.publish_session_status(session);
        let endpoint::SessionSyncFrame::InventoryUpsert {
            session: summary, ..
        } = block_on_ready(inventory.recv())
            .expect("idle upsert item")
            .expect("idle upsert")
        else {
            panic!("expected an inventory upsert after the line lock is released")
        };
        assert!(!summary.running);
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn periodic_sweep_repairs_a_stale_endpoint_journal_for_live_subscribers() {
        const SESSION: &str = "018f0000-0000-7000-8000-000000000111";
        let root = tempfile::tempdir().expect("process host root");
        let folder = root.path().join("threads").join(SESSION);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let source = include_str!("../../../fixtures/endpoint/projection-source.jsonl")
            .replace("018f0000-0000-7000-8000-000000000003", SESSION);
        let lines = source.lines().collect::<Vec<_>>();
        fs::write(folder.join("main.jsonl"), lines[..6].join("\n") + "\n")
            .expect("active attempt ledger");
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        host.release_for_test(SESSION);
        let streams = ProductionCarrierStreams::new(root.path());
        host.attach_streams(streams.clone());
        host.publish_appended(SESSION)
            .expect("project active attempt prefix");
        host.publish_frame(
            SESSION,
            Frame {
                arguments_complete: None,
                ledger_seq: None,
                attempt: "a1".to_owned(),
                channel: FrameChannel::Text,
                block: 0,
                delta: "done".to_owned(),
                call_id: None,
                name: None,
            },
        )
        .expect("stream frame");
        let mut journal = streams
            .open_mux(
                1,
                endpoint::SessionStreamTarget::SessionJournal {
                    address: endpoint::SessionAddress {
                        session_id: SESSION.to_owned(),
                    },
                    max_messages: 50,
                },
            )
            .expect("journal stream");
        let endpoint::SessionSyncFrame::JournalSnapshot { snapshot, .. } =
            block_on_ready(journal.recv())
                .expect("snapshot item")
                .expect("snapshot frame")
        else {
            panic!("expected journal snapshot")
        };
        assert!(
            snapshot
                .entries
                .iter()
                .all(|entry| entry.event.event_type != "turn/end"),
            "the active prefix has no terminal projection yet"
        );

        // The worker settled and died without its doorbell reaching the supervisor.
        fs::write(folder.join("main.jsonl"), source).expect("terminal semantic tail");

        assert!(host.periodic_sweep_once().expect("periodic sweep"));

        let mut delivered = Vec::new();
        let mut replayed = Vec::new();
        while delivered.len() < 3 {
            match block_on_ready(journal.recv())
                .expect("live repair item")
                .expect("live repair frame")
            {
                // The context projections newer than the snapshot's cut follow the snapshot at
                // their true sequence (N1); they are not part of this test's subject.
                endpoint::SessionSyncFrame::JournalProjection { key, .. } => replayed.push(key),
                endpoint::SessionSyncFrame::JournalEvent { event, .. } => {
                    delivered.push(event.event_type);
                }
                _ => panic!("expected a live journal event after the sweep repair"),
            }
        }
        assert_eq!(delivered, ["assistant/message", "step/end", "turn/end"]);
        assert!(
            replayed
                .iter()
                .all(|key| key == "contextDetails" || key == "contextUsage"),
            "only the context projections are replayed after the snapshot"
        );
        let records = EndpointJournal::open(&folder)
            .expect("endpoint journal")
            .records()
            .expect("endpoint records");
        assert_eq!(
            records
                .last()
                .map(|record| record.event.event_type.as_str()),
            Some("turn/end")
        );
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn opening_a_mux_journal_reconciles_the_semantic_tail_first() {
        const SESSION: &str = "018f0000-0000-7000-8000-000000000112";
        let root = tempfile::tempdir().expect("process host root");
        let folder = root.path().join("threads").join(SESSION);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let source = include_str!("../../../fixtures/endpoint/projection-source.jsonl")
            .replace("018f0000-0000-7000-8000-000000000003", SESSION);
        fs::write(folder.join("main.jsonl"), source).expect("settled ledger with no journal");
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        let streams = ProductionCarrierStreams::new(root.path());
        host.attach_streams(streams.clone());
        let mut journal = streams
            .open_mux(
                1,
                endpoint::SessionStreamTarget::SessionJournal {
                    address: endpoint::SessionAddress {
                        session_id: SESSION.to_owned(),
                    },
                    max_messages: 50,
                },
            )
            .expect("journal stream");
        let endpoint::SessionSyncFrame::JournalSnapshot { snapshot, .. } =
            block_on_ready(journal.recv())
                .expect("snapshot item")
                .expect("snapshot frame")
        else {
            panic!("expected journal snapshot")
        };
        assert!(
            snapshot.through_sequence >= 0,
            "the snapshot must not be empty"
        );
        assert_eq!(
            snapshot
                .entries
                .last()
                .map(|entry| entry.event.event_type.as_str()),
            Some("turn/end"),
            "opening the stream projected the settled tail before the cut"
        );
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    /// Polls a V3 stream until a frame is ready or the deadline passes. Reaper-driven
    /// repairs run on another thread, so the frame cannot be assumed ready on the first poll.
    fn wait_for_mux_frame(
        stream: &mut endpoint::SessionStreamReceiver,
        timeout: Duration,
    ) -> Option<endpoint::SessionSyncFrame> {
        let deadline = Instant::now() + timeout;
        loop {
            let waker = Waker::noop();
            let mut context = Context::from_waker(waker);
            let ready = {
                let mut future = std::pin::pin!(stream.recv());
                match future.as_mut().poll(&mut context) {
                    Poll::Ready(Some(Ok(frame))) => Some(Some(frame)),
                    Poll::Ready(Some(Err(error))) => panic!("stream frame failed: {error:?}"),
                    Poll::Ready(None) => Some(None),
                    Poll::Pending => None,
                }
            };
            if let Some(frame) = ready {
                return frame;
            }
            if Instant::now() >= deadline {
                return None;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    /// Audit sequences Q2 and Q6: a worker settles and dies without its doorbell. The reaper's
    /// exit path must project the settled tail for live journal subscribers and flip the
    /// inventory summary back to a non-running tail.
    #[cfg(target_os = "macos")]
    #[test]
    fn worker_exit_path_projects_the_settled_tail_and_refreshes_inventory() {
        const SESSION: &str = "018f0000-0000-7000-8000-000000000003";
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        fs::create_dir_all(root.path().join("threads")).expect("threads");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        let fixture = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/endpoint/projection-source.jsonl"
        );
        // The fake worker settles the whole turn on disk and exits without `appended`.
        let bin = root.path().join("fake-worker");
        fs::write(
            &bin,
            format!(
                "#!/bin/sh\nLEDGER=\"$1\"\nprintf '%s\\n' '{{\"hello\":{{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}}}'\nIFS= read -r selected\nsleep 0.5\ncp '{fixture}' \"$LEDGER\"\nexit 0\n"
            ),
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");

        let folder = root.path().join("threads").join(SESSION);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let source = fs::read_to_string(fixture).expect("fixture");
        let lines = source.lines().collect::<Vec<_>>();
        fs::write(folder.join("main.jsonl"), lines[..2].join("\n") + "\n")
            .expect("runnable ledger: genesis + input");

        let host =
            ProductionProcessHost::open(root.path(), &bin, "test", root.path().join(".agent"))
                .expect("process host");
        host.release_for_test(SESSION);
        let streams = ProductionCarrierStreams::new(root.path());
        host.attach_streams(streams.clone());
        let mut inventory = streams
            .open_mux(1, endpoint::SessionStreamTarget::SessionInventory)
            .expect("inventory stream");
        let endpoint::SessionSyncFrame::InventoryBaseline { items, .. } =
            block_on_ready(inventory.recv())
                .expect("baseline item")
                .expect("baseline frame")
        else {
            panic!("expected inventory baseline")
        };
        assert_eq!(items[0].tail, "settled");
        let mut journal = streams
            .open_mux(
                1,
                endpoint::SessionStreamTarget::SessionJournal {
                    address: endpoint::SessionAddress {
                        session_id: SESSION.to_owned(),
                    },
                    max_messages: 50,
                },
            )
            .expect("journal stream");
        assert!(matches!(
            block_on_ready(journal.recv()).expect("snapshot item"),
            Ok(endpoint::SessionSyncFrame::JournalSnapshot { .. })
        ));

        assert!(
            host.periodic_sweep_once()
                .expect("sweep spawns the runnable session")
        );

        let Some(endpoint::SessionSyncFrame::InventoryUpsert { session, .. }) =
            wait_for_mux_frame(&mut inventory, Duration::from_secs(10))
        else {
            panic!("expected an inventory upsert when the worker registered")
        };
        assert_eq!(session.tail, "running");

        let mut delivered = Vec::new();
        let deadline = Instant::now() + Duration::from_secs(10);
        while !delivered.contains(&"turn/end".to_owned()) {
            assert!(
                Instant::now() < deadline,
                "journal never received the settled tail: {delivered:?}"
            );
            match wait_for_mux_frame(&mut journal, Duration::from_secs(10)) {
                Some(endpoint::SessionSyncFrame::JournalEvent { event, .. }) => {
                    delivered.push(event.event_type);
                }
                Some(_) => {}
                None => panic!("journal stream ended before the settled tail: {delivered:?}"),
            }
        }
        assert!(delivered.contains(&"assistant/message".to_owned()));
        assert!(delivered.contains(&"step/end".to_owned()));

        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            assert!(
                Instant::now() < deadline,
                "worker exit never reached inventory"
            );
            match wait_for_mux_frame(
                &mut inventory,
                deadline.saturating_duration_since(Instant::now()),
            ) {
                Some(endpoint::SessionSyncFrame::InventoryUpsert { session, .. })
                    if !session.running =>
                {
                    assert_eq!(session.tail, "settled");
                    break;
                }
                // Context publications can refresh inventory while the worker is still exiting.
                Some(_) => {}
                None => panic!("expected an inventory upsert when the worker was reaped"),
            }
        }
        host.shutdown();
    }

    /// Audit sequence Q3: the doorbell arrives but projection fails. The periodic sweep is the
    /// retry: it repairs once the journal is writable again and clears its failure record.
    #[cfg(target_os = "macos")]
    #[test]
    fn sweep_retries_a_projection_that_failed_on_the_doorbell() {
        const SESSION: &str = "018f0000-0000-7000-8000-000000000003";
        let root = tempfile::tempdir().expect("process host root");
        let folder = root.path().join("threads").join(SESSION);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let source = include_str!("../../../fixtures/endpoint/projection-source.jsonl");
        let lines = source.lines().collect::<Vec<_>>();
        fs::write(folder.join("main.jsonl"), lines[..6].join("\n") + "\n")
            .expect("active attempt ledger");
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        host.release_for_test(SESSION);
        host.publish_appended(SESSION)
            .expect("project the active prefix");
        fs::write(folder.join("main.jsonl"), source).expect("terminal semantic tail");

        let journal_path = folder.join("endpoint.jsonl");
        fs::set_permissions(&journal_path, fs::Permissions::from_mode(0o400))
            .expect("make the journal read-only");
        assert!(
            host.publish_appended(SESSION).is_err(),
            "the doorbell projection must fail while the journal is read-only"
        );
        assert!(host.periodic_sweep_once().expect("sweep runs"));
        assert!(
            host.projection_repair_failures
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .contains(SESSION),
            "the failed repair is recorded once for de-duplicated reporting"
        );
        let stale = fs::read_to_string(&journal_path).expect("read-only journal bytes");
        assert!(
            !stale.contains("turn/end"),
            "nothing was projected while the journal was read-only"
        );

        fs::set_permissions(&journal_path, fs::Permissions::from_mode(0o644))
            .expect("restore the journal");
        assert!(host.periodic_sweep_once().expect("sweep runs again"));
        let repaired = EndpointJournal::open(&folder)
            .expect("endpoint journal")
            .records()
            .expect("records");
        assert_eq!(
            repaired
                .last()
                .map(|record| record.event.event_type.as_str()),
            Some("turn/end")
        );
        assert!(
            host.projection_repair_failures
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .is_empty()
        );
        host.shutdown();
    }

    #[test]
    fn live_stream_time_is_integral_epoch_milliseconds() {
        let time = std::time::UNIX_EPOCH + Duration::from_micros(1_234_567);

        assert_eq!(session_event_time(time).expect("time"), 1_234.0);
    }

    #[test]
    fn terminal_attempt_pipe_tail_is_not_a_worker_failure() {
        let result = accepted_stream_event(Err(endpoint::ProjectionError::TerminalAttempt(
            "attempt-1".to_owned(),
        )))
        .expect("late terminal frame is benign");

        assert!(result.is_none());
        assert!(
            accepted_stream_event(Err(endpoint::ProjectionError::UnknownAttempt(
                "attempt-2".to_owned()
            )))
            .is_err()
        );
    }

    #[test]
    fn tool_launch_policy_uses_selected_folder_without_dropping_other_roots() {
        let root = tempfile::tempdir().expect("root");
        let first = root.path().join("first");
        let second = root.path().join("second");
        fs::create_dir(&first).expect("first root");
        fs::create_dir(&second).expect("second root");
        let repository = profile::ConfigRepository::open(root.path()).expect("repository");
        repository
            .publish_workspace(
                0,
                &profile::WorkspaceConfig {
                    format: 1,
                    revision: 1,
                    id: "workspace-tools".to_owned(),
                    name: "Workspace".to_owned(),
                    cwd: Vec::new(),
                    folders: vec![
                        profile::WorkspaceFolder {
                            id: "primary".to_owned(),
                            path: first.display().to_string(),
                        },
                        profile::WorkspaceFolder {
                            id: "secondary".to_owned(),
                            path: second.display().to_string(),
                        },
                    ],
                    policy: None,
                },
            )
            .expect("workspace");
        let config = repository
            .resolve_for_binding("workspace-tools", "secondary")
            .expect("selected config");
        let instruction = profile::InstructionResolver::new(
            root.path().join("missing-agent"),
            config.workspace.cwd.iter().map(Path::new),
        )
        .capture()
        .expect("instruction snapshot");
        let policy = frozen_tool_launch_policy(&config, &instruction).expect("tool policy");
        assert_eq!(
            policy.primary_workspace_cwd(),
            second
                .canonicalize()
                .expect("canonical second")
                .to_str()
                .unwrap()
        );
        assert_eq!(
            policy.base_sandbox().read_roots,
            vec![
                first
                    .canonicalize()
                    .expect("canonical first")
                    .display()
                    .to_string(),
                second
                    .canonicalize()
                    .expect("canonical second")
                    .display()
                    .to_string(),
            ]
        );
        let scratch = root.path().join("validator/scratch");
        let snapshot = root.path().join("validator/snapshot");
        fs::create_dir_all(&scratch).expect("scratch");
        fs::create_dir_all(&snapshot).expect("snapshot");
        let scratch = scratch.canonicalize().expect("canonical scratch");
        let snapshot = snapshot.canonicalize().expect("canonical snapshot");
        let validator = private_validator_launch_policy(&scratch, &snapshot, &config, &instruction)
            .expect("validator policy");
        assert_eq!(validator.primary_workspace_cwd(), scratch.to_str().unwrap());
        assert_eq!(
            validator.permitted_write_roots(),
            &[scratch.display().to_string()]
        );
        assert!(
            validator
                .base_sandbox()
                .read_roots
                .contains(&second.canonicalize().unwrap().display().to_string())
        );
        assert!(
            validator
                .base_sandbox()
                .read_roots
                .contains(&snapshot.display().to_string())
        );
    }

    #[test]
    fn report_wake_targets_the_exact_nested_parent_line() {
        let root = Path::new("/kernel");
        let (key, ledger) =
            line_schedule_target(root, "session-1", "parent-child", "parent-child.jsonl")
                .expect("nested parent target");
        assert_eq!(key, "session-1:parent-child");
        assert_eq!(ledger, root.join("threads/session-1/parent-child.jsonl"));
    }

    #[test]
    fn report_wake_rejects_parent_line_aliases_and_path_escape() {
        let root = Path::new("/kernel");
        assert!(line_schedule_target(root, "session-1", "parent", "other.jsonl").is_err());
        assert!(line_schedule_target(root, "session-1", "parent", "../parent.jsonl").is_err());
        assert!(line_schedule_target(root, "session-1", "child", "main.jsonl").is_err());
        assert_eq!(
            line_schedule_target(root, "session-1", "session-1", "main.jsonl")
                .expect("main target")
                .0,
            "session-1"
        );
    }

    #[test]
    fn child_dependency_overrides_only_its_live_parent_capacity_slot() {
        assert!(child_dependency_admitted(false, true));
        assert!(
            child_dependency_admitted(false, true),
            "a live child may transfer its occupied slot to one grandchild"
        );
        assert!(
            !child_dependency_admitted(false, false),
            "an unrelated pending root cannot steal the dependency override"
        );
        assert!(child_dependency_admitted(true, false));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn slice14c_gate_103_production_claim_uses_management_and_delivery_authorities() {
        let root = tempfile::tempdir().expect("schedule host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        let workspace = fs::canonicalize(&workspace).expect("canonical workspace");
        let repository = profile::ConfigRepository::open(root.path()).expect("config repository");
        repository
            .publish_workspace(
                0,
                &profile::WorkspaceConfig {
                    format: 1,
                    revision: 1,
                    id: "ws".to_owned(),
                    name: "ws".to_owned(),
                    cwd: vec![workspace.to_string_lossy().into_owned()],
                    folders: Vec::new(),
                    policy: Some(profile::WorkspacePolicy::default()),
                },
            )
            .expect("workspace config");
        let bin = root.path().join("fake-worker");
        fs::write(
            &bin,
            b"#!/bin/sh\nprintf '%s\\n' '{\"hello\":{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}'\nIFS= read -r selected\nwhile IFS= read -r line; do :; done\n",
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");

        let host =
            ProductionProcessHost::open(root.path(), &bin, "test", root.path().join(".agent"))
                .expect("process host");
        let now = DateTime::parse_from_rfc3339("2026-08-29T04:00:00Z")
            .expect("time")
            .with_timezone(&Utc);
        let task_id = "018f0000-0000-7000-8000-0000000000c1";
        host.schedule
            .save(
                schedule::OriginTuple {
                    client_id: "test-client".to_owned(),
                    key: "save".to_owned(),
                },
                schedule::ScheduleDefinition {
                    id: task_id.to_owned(),
                    name: "Scheduled review".to_owned(),
                    workspace_id: "ws".to_owned(),
                    cron: "0 * * * *".to_owned(),
                    time_zone: "UTC".to_owned(),
                    prompt: "Review now".to_owned(),
                    permission_mode: "workspace-write".to_owned(),
                    model_id: None,
                    enabled: true,
                    missed_policy: schedule::MissedPolicy::SkipAndRecord,
                },
                now,
            )
            .expect("save schedule");
        let claim = host
            .schedule
            .run_now(
                schedule::OriginTuple {
                    client_id: "test-client".to_owned(),
                    key: "run".to_owned(),
                },
                task_id,
                now,
            )
            .expect("manual claim");
        host.execute_schedule_claim(&claim, now)
            .expect("production schedule launch");
        let view = host.schedule.list(None).expect("schedule view").remove(0);
        assert_eq!(view.last_status, schedule::RunStatus::Running);
        assert_eq!(view.last_input_seq, Some(2));
        let session = view.last_session_id.expect("bound session");
        let ledger = root
            .path()
            .join("threads")
            .join(&session)
            .join("main.jsonl");
        let projection =
            store::scan_valid_prefix(&fs::read(ledger).expect("ledger"), schema::READER_VERSION)
                .projection
                .expect("valid projection");
        assert_eq!(projection.events.len(), 3);
        assert_eq!(projection.events[0].kind(), &EventKind::Genesis);
        assert_eq!(projection.events[1].kind(), &EventKind::Input);
        assert_eq!(
            projection.events[1].origin_key(),
            Some(claim.claim_id.as_str())
        );
        assert_eq!(projection.events[2].kind(), &EventKind::Meta);
        assert_eq!(
            projection.events[2].string_field("title"),
            Some("Review now")
        );
        assert_eq!(
            projection.events[2]
                .origin_tuple()
                .expect("title origin")
                .expect("automatic title origin")
                .op,
            AUTOMATIC_TITLE_SEED_OPERATION
        );
        host.shutdown();
    }

    #[test]
    fn periodic_sweep_is_single_flight_and_stops_at_drain() {
        let root = tempfile::tempdir().expect("process host root");
        fs::create_dir_all(root.path().join("threads")).expect("threads root");
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");

        let sweep = host
            .sweep_lock
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(
            !host.periodic_sweep_once().expect("overlapping tick"),
            "an overlapping periodic tick must coalesce instead of queue"
        );
        drop(sweep);
        assert!(host.periodic_sweep_once().expect("uncontended tick"));

        host.shutdown();
        assert!(
            !host.periodic_sweep_once().expect("draining tick"),
            "drain must prevent periodic reconciliation from launching work"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn sweep_isolates_one_corrupt_session_and_still_recovers_the_next() {
        check_sweep_recovery(false);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn builtin_recovery_waits_for_explicit_request_and_is_idempotent() {
        check_sweep_recovery(true);
    }

    #[cfg(target_os = "macos")]
    fn check_sweep_recovery(defer_recovery: bool) {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        fs::create_dir_all(root.path().join("threads")).expect("threads");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        let bin = root.path().join("fake-worker");
        fs::write(
            &bin,
            b"#!/bin/sh\nprintf '%s\\n' '{\"hello\":{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}'\nIFS= read -r selected\nwhile IFS= read -r line; do :; done\n",
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");

        let host =
            ProductionProcessHost::open(root.path(), &bin, "test", root.path().join(".agent"))
                .expect("process host");
        let corrupt = "018f0000-0000-7000-8000-000000000105";
        let corrupt_folder = root.path().join("threads").join(corrupt);
        fs::create_dir_all(corrupt_folder.join("assets")).expect("corrupt thread folder");
        fs::write(corrupt_folder.join("main.jsonl"), b"corrupt\n").expect("corrupt ledger");
        let session = "018f0000-0000-7000-8000-000000000106";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        write_test_genesis(&folder.join("main.jsonl"), session);
        append_test_input(&folder.join("main.jsonl"), session);

        if defer_recovery {
            // Pre-existing sessions are deferred without any enumeration.
            host.boot_sweep().expect("repair startup state");
            assert!(host.periodic_sweep_once().expect("deferred periodic sweep"));
            assert!(
                host.workers.lock().unwrap().is_empty(),
                "startup must not launch a worker"
            );
            host.recover_client_sessions(&[session.to_owned()])
                .expect("explicit recovery");
            let first = host
                .workers
                .lock()
                .unwrap()
                .get(session)
                .cloned()
                .expect("worker started");
            host.recover_client_sessions(&[session.to_owned()])
                .expect("repeated recovery");
            let second = host
                .workers
                .lock()
                .unwrap()
                .get(session)
                .cloned()
                .expect("worker retained");
            assert!(
                Arc::ptr_eq(&first, &second),
                "repeat recovery must retain the live worker"
            );
        } else {
            // Both sessions are released (as `sessions.recover` would): the corrupt one must
            // not stop the runnable one from being recovered.
            host.released
                .lock()
                .unwrap()
                .extend([corrupt.to_owned(), session.to_owned()]);
            assert!(host.periodic_sweep_once().expect("isolated sweep"));
        }

        for _ in 0..100 {
            if host
                .workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .get(session)
                .is_some_and(|worker| worker.alive.load(Ordering::Acquire))
            {
                host.shutdown();
                return;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        host.shutdown();
        panic!("sweep did not recover the runnable session after an earlier corruption");
    }

    /// An unchanged ledger is not decoded again on the next tick; an appended one is.
    #[test]
    fn sweep_scans_a_ledger_once_per_file_identity() {
        let root = tempfile::tempdir().expect("process host root");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("threads")).expect("threads");
        let bin = root.path().join("fake-worker");
        fs::write(&bin, b"#!/bin/sh\nexit 0\n").expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let host =
            ProductionProcessHost::open(root.path(), &bin, "test", root.path().join(".agent"))
                .expect("process host");
        let session = "018f0000-0000-7000-8000-000000000107";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(&folder).expect("thread folder");
        let ledger = folder.join("main.jsonl");
        write_test_genesis(&ledger, session);

        let first = host.sweep_ledger_scan(&ledger).expect("first scan");
        let again = host.sweep_ledger_scan(&ledger).expect("cached scan");
        assert!(Arc::ptr_eq(&first, &again), "same bytes, same scan");
        assert_eq!(first.line, session);
        assert_eq!(first.last_seq, 1);

        append_test_input(&ledger, session);
        let grown = host.sweep_ledger_scan(&ledger).expect("rescanned");
        assert!(
            !Arc::ptr_eq(&first, &grown),
            "an appended ledger is scanned again"
        );
        assert_eq!(grown.last_seq, 2);

        fs::write(&ledger, b"corrupt\n").expect("corrupt ledger");
        assert!(host.sweep_ledger_scan(&ledger).is_err());
        assert!(
            !host.sweep_scans.lock().unwrap().contains_key(&ledger),
            "a corrupt ledger holds no cached facts"
        );
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn sweep_quarantines_busy_unknown_until_the_file_lock_is_free() {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        fs::create_dir_all(root.path().join("threads")).expect("threads");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        let bin = root.path().join("fake-worker");
        fs::write(
            &bin,
            b"#!/bin/sh\nprintf '%s\n' '{\"hello\":{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}'\nIFS= read -r selected\nwhile IFS= read -r line; do :; done\n",
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");

        let session = "018f0000-0000-7000-8000-000000000108";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let ledger = folder.join("main.jsonl");
        write_test_genesis(&ledger, session);
        append_test_input(&ledger, session);
        let holder =
            LockedLedger::open(&ledger, schema::READER_VERSION).expect("legacy holder lock");
        let host = ProductionProcessHost::open(
            root.path(),
            &bin,
            "test",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        host.release_for_test(session);

        assert!(host.periodic_sweep_once().expect("busy sweep"));
        assert!(
            host.workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .is_empty(),
            "a busy-unknown file must not spawn a losing candidate"
        );
        drop(holder);
        assert!(host.periodic_sweep_once().expect("post-release sweep"));
        assert!(
            host.workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .get(session)
                .is_some_and(|worker| worker.alive.load(Ordering::Acquire)),
            "the next fresh sweep must launch after the unknown holder exits"
        );
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn ensure_existing_worker_refreshes_adjacent_secret_revocation() {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        write_canonical_test_json(
            &root.path().join("config/providers.json"),
            json!({
                "format":1,"revision":1,"providers":[{
                    "id":"main","adapter":"responses","dialect":"openai_responses_v1",
                    "endpoint_owner":"openai","gateway_translation":"direct",
                    "evidence_revision":"openai-2026-08-01",
                    "endpoint":"https://api.openai.com/v1","credential_key":"shared",
                    "models":[{"id":"gpt-5","profile":"openai_responses_v1:gpt-5","enabled":true,
                        "context_window_tokens":100,"compact_trigger_tokens":50}]
                }]
            }),
        );
        let bin = root.path().join("fake-worker");
        fs::write(
            &bin,
            b"#!/bin/sh\nprintf '%s\\n' '{\"hello\":{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}'\nIFS= read -r selected\nwhile IFS= read -r line; do :; done\n",
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");

        let session = "018f0000-0000-7000-8000-000000000105";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        write_test_genesis(&folder.join("main.jsonl"), session);
        let store = Arc::new(provider::MemorySecretStore::new());
        store
            .publish(
                "shared",
                provider::SecretRecord::Active {
                    generation: 1,
                    material: "fixture-secret-never-log".to_owned(),
                },
            )
            .expect("active secret");
        let host = ProductionProcessHost::open_with_secret_store(
            root.path(),
            &bin,
            "test",
            root.path().join(".agent"),
            Arc::clone(&store) as Arc<dyn provider::SecretStore>,
        )
        .expect("process host");
        let first = host
            .schedule_worker_at(session, session, folder.join("main.jsonl"))
            .expect("first ensure")
            .expect("worker starts");

        store
            .publish("shared", provider::SecretRecord::Revoked { generation: 2 })
            .expect("adjacent revoke");
        let second = host
            .schedule_worker_at(session, session, folder.join("main.jsonl"))
            .expect("second ensure")
            .expect("existing worker");
        assert!(Arc::ptr_eq(&first, &second));
        assert!(matches!(
            second
                .credential_bindings
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .as_ref()
                .and_then(|bindings| bindings.availability.get("shared")),
            Some(provider::CredentialAvailability::Revoked { generation: 2 })
        ));
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn queue_recovery_is_preloaded_during_worker_negotiation() {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        let capture = root.path().join("startup.jsonl");
        let result = QueueTransactionResult {
            delivery: "rpc-queue".to_owned(),
            outcome: worker_control::QueueTransactionOutcome::Committed {
                first_seq: 2,
                last_seq: 2,
                deduplicated: false,
            },
        };
        let result_line = String::from_utf8(
            worker_control::encode_queue_transaction_result(&result).expect("queue result line"),
        )
        .expect("queue result UTF-8");
        let bin = root.path().join("fake-worker");
        fs::write(
            &bin,
            format!(
                "#!/bin/sh\nprintf '%s\\n' '{{\"hello\":{{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}}}'\nIFS= read -r selected\nIFS= read -r startup\nprintf '%s\\n%s\\n' \"$selected\" \"$startup\" > '{}'\nprintf '%s\\n' '{}'\nwhile IFS= read -r line; do :; done\n",
                capture.display(),
                result_line.trim_end()
            ),
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");

        let session = "018f0000-0000-7000-8000-000000000107";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        write_test_genesis(&folder.join("main.jsonl"), session);
        let transaction = QueueTransaction {
            delivery: "rpc-queue".to_owned(),
            rpc_id: "rpc-queue".to_owned(),
            target_seq: 1,
            retract_origin: OriginTuple {
                client: "client".to_owned(),
                principal: "principal".to_owned(),
                op: "session.updateQueue".to_owned(),
                target: session.to_owned(),
                key: "rpc-queue/retract".to_owned(),
            },
            action: worker_control::QueueTransactionAction::Remove,
        };
        let host = ProductionProcessHost::open(
            root.path(),
            &bin,
            "test",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        let observed = QueueTransactionAuthority::execute(&*host, session, &transaction)
            .expect("preloaded transaction result");
        assert_eq!(observed, result);
        let lines = fs::read(&capture).expect("captured startup bytes");
        let mut lines = lines.split_inclusive(|byte| *byte == b'\n');
        let selected = worker_control::decode_selection(lines.next().expect("selection line"))
            .expect("selection");
        assert_eq!(selected.startup, Some(WorkerStartup::QueueTransaction));
        assert_eq!(
            worker_control::decode_queue_transaction(lines.next().expect("queue transaction line"))
                .expect("queue transaction"),
            transaction
        );
        assert!(lines.next().is_none());
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn pending_queue_recovery_survives_an_ordinary_ensure_race() {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        fs::create_dir_all(root.path().join("threads")).expect("threads");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        let capture = root.path().join("startup.jsonl");
        let result = QueueTransactionResult {
            delivery: "rpc-pending".to_owned(),
            outcome: worker_control::QueueTransactionOutcome::Committed {
                first_seq: 2,
                last_seq: 2,
                deduplicated: false,
            },
        };
        let result_line = String::from_utf8(
            worker_control::encode_queue_transaction_result(&result).expect("queue result line"),
        )
        .expect("queue result UTF-8");
        let bin = root.path().join("fake-worker");
        fs::write(
            &bin,
            format!(
                "#!/bin/sh\nprintf '%s\\n' '{{\"hello\":{{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}}}'\nIFS= read -r selected\nIFS= read -r startup\nprintf '%s\\n%s\\n' \"$selected\" \"$startup\" > '{}'\nprintf '%s\\n' '{}'\nwhile IFS= read -r line; do :; done\n",
                capture.display(),
                result_line.trim_end()
            ),
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");

        let session = "018f0000-0000-7000-8000-000000000112";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let ledger = folder.join("main.jsonl");
        write_test_genesis(&ledger, session);
        let transaction = QueueTransaction {
            delivery: "rpc-pending".to_owned(),
            rpc_id: "rpc-pending".to_owned(),
            target_seq: 1,
            retract_origin: OriginTuple {
                client: "client".to_owned(),
                principal: "principal".to_owned(),
                op: "session.updateQueue".to_owned(),
                target: session.to_owned(),
                key: "rpc-pending/retract".to_owned(),
            },
            action: worker_control::QueueTransactionAction::Remove,
        };
        let host = ProductionProcessHost::open(
            root.path(),
            &bin,
            "test",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        host.pending_workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(
                session.to_owned(),
                PendingWorker {
                    session_id: session.to_owned(),
                    ledger: ledger.clone(),
                    startup: Some(transaction.clone()),
                },
            );
        let worker = host
            .schedule_worker_at(session, session, ledger)
            .expect("ordinary ensure")
            .expect("ordinary ensure starts worker");
        assert_eq!(
            worker
                .queue_result(&transaction.delivery)
                .expect("preloaded queue result"),
            result,
        );
        let lines = fs::read(&capture).expect("captured startup bytes");
        let mut lines = lines.split_inclusive(|byte| *byte == b'\n');
        let selected = worker_control::decode_selection(lines.next().expect("selection line"))
            .expect("selection");
        assert_eq!(selected.startup, Some(WorkerStartup::QueueTransaction));
        assert_eq!(
            worker_control::decode_queue_transaction(lines.next().expect("queue transaction line"))
                .expect("queue transaction"),
            transaction,
        );
        assert!(lines.next().is_none());
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn capacity_release_starts_a_pending_queue_transaction_on_a_settled_tail() {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        fs::create_dir_all(root.path().join("threads")).expect("threads");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        write_canonical_test_json(
            &root.path().join("config/settings.json"),
            json!({"format":1,"revision":1,"limits":{"max_workers":1,"max_provider_leases":1}}),
        );
        let result = QueueTransactionResult {
            delivery: "rpc-capacity".to_owned(),
            outcome: worker_control::QueueTransactionOutcome::Committed {
                first_seq: 2,
                last_seq: 2,
                deduplicated: false,
            },
        };
        let result_line = String::from_utf8(
            worker_control::encode_queue_transaction_result(&result).expect("queue result line"),
        )
        .expect("queue result UTF-8");
        let bin = root.path().join("fake-worker");
        // The pending worker stays alive after its queue result until shutdown closes stdin.
        // If it exited at once, a slow waiter could miss the live worker and time out.
        fs::write(
            &bin,
            format!(
                "#!/bin/sh\nprintf '%s\\n' '{{\"hello\":{{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}}}'\nIFS= read -r selected\ncase \"$selected\" in\n  *queue-transaction*) IFS= read -r startup; printf '%s\\n' '{}'; while IFS= read -r line; do :; done ;;\n  *) while IFS= read -r line; do :; done ;;\nesac\n",
                result_line.trim_end()
            ),
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");

        let blocker = "018f0000-0000-7000-8000-000000000120";
        let target = "018f0000-0000-7000-8000-000000000121";
        for session in [blocker, target] {
            let folder = root.path().join("threads").join(session);
            fs::create_dir_all(folder.join("assets")).expect("thread folder");
            write_test_genesis(&folder.join("main.jsonl"), session);
        }
        let transaction = QueueTransaction {
            delivery: "rpc-capacity".to_owned(),
            rpc_id: "rpc-capacity".to_owned(),
            target_seq: 1,
            retract_origin: OriginTuple {
                client: "client".to_owned(),
                principal: "principal".to_owned(),
                op: "session.updateQueue".to_owned(),
                target: target.to_owned(),
                key: "rpc-capacity/retract".to_owned(),
            },
            action: worker_control::QueueTransactionAction::Remove,
        };
        let host = ProductionProcessHost::open(
            root.path(),
            &bin,
            "test",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        let blocker_worker = host
            .schedule_main(blocker)
            .expect("blocker schedule")
            .expect("blocker worker");
        let scheduled = host
            .schedule_worker_at_with_startup(
                target,
                target,
                root.path().join("threads").join(target).join("main.jsonl"),
                Some(&transaction),
            )
            .expect("pending queue schedule");
        assert!(scheduled.worker.is_none());
        assert!(scheduled.startup_preloaded);

        blocker_worker
            .child
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .kill()
            .expect("release capacity");
        let pending_worker = host.wait_for_worker(target).expect("pending worker starts");
        assert_eq!(
            pending_worker
                .queue_result(&transaction.delivery)
                .expect("preloaded queue result"),
            result,
        );
        assert!(
            !host
                .pending_workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .contains_key(target),
            "the transaction leaves pending only after a successful spawn",
        );
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn no_mutual_worker_protocol_is_quarantined_until_binary_bytes_change() {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        fs::create_dir_all(root.path().join("threads")).expect("threads");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        let launches = workspace.join("launches");
        let reject = workspace.join("reject");
        let bin = root.path().join("fake-worker");
        fs::write(
            &bin,
            format!(
                "#!/bin/sh\nprintf x >> '{}'\nprintf '%s\\n' '{{\"hello\":{{\"max\":4,\"min\":3,\"proto\":\"tekes-worker\"}}}}'\nIFS= read -r line\nprintf '%s\\n' \"$line\" > '{}'\nexit 76\n",
                launches.display(),
                reject.display(),
            ),
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");
        let session = "018f0000-0000-7000-8000-000000000122";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let ledger = folder.join("main.jsonl");
        write_test_genesis(&ledger, session);
        let host = ProductionProcessHost::open(
            root.path(),
            &bin,
            "test",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");

        let first = match host.schedule_worker_at(session, session, ledger.clone()) {
            Ok(_) => panic!("incompatible worker unexpectedly started"),
            Err(error) => error,
        };
        assert!(first.to_string().contains("does not support protocol v2"));
        let second = match host.schedule_worker_at(session, session, ledger) {
            Ok(_) => panic!("quarantined worker unexpectedly started"),
            Err(error) => error,
        };
        assert!(second.to_string().contains("quarantined"));
        assert_eq!(fs::read(&launches).expect("launch witness"), b"x");
        assert!(
            fs::read_to_string(&reject)
                .expect("reject witness")
                .contains("no_mutual_version"),
        );
        assert_eq!(
            host.protocol_quarantine
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .len(),
            1,
        );
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn worker_hello_timeout_kills_waits_and_closes_the_broker() {
        assert_handshake_failure_reaps_worker(
            "while :; do :; done",
            Duration::from_secs(5),
            Some("worker hello timed out"),
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn malformed_worker_hello_kills_waits_and_closes_the_broker() {
        assert_handshake_failure_reaps_worker(
            "printf '%s\\n' 'not-json'; while :; do :; done",
            Duration::from_secs(5),
            None,
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn worker_early_eof_is_reaped_before_spawn_returns() {
        assert_handshake_failure_reaps_worker(
            "exit 0",
            Duration::from_secs(5),
            Some("worker exited before hello"),
        );
    }

    #[cfg(target_os = "macos")]
    fn assert_handshake_failure_reaps_worker(
        behavior: &str,
        timeout: Duration,
        expected_message: Option<&str>,
    ) {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        fs::create_dir_all(root.path().join("threads")).expect("threads");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,
                    "writable_roots":[workspace.to_string_lossy()]}
            }),
        );
        write_canonical_test_json(
            &root.path().join("config/providers.json"),
            json!({
                "format":1,"revision":1,"providers":[{
                    "id":"main","adapter":"responses","dialect":"openai_responses_v1",
                    "endpoint_owner":"openai","gateway_translation":"direct",
                    "evidence_revision":"openai-2026-08-01",
                    "endpoint":"https://api.openai.com/v1","credential_key":"shared",
                    "models":[{"id":"gpt-5","profile":"openai_responses_v1:gpt-5","enabled":true,
                        "context_window_tokens":100,"compact_trigger_tokens":50}]
                }]
            }),
        );
        let pid_file = workspace.join("worker.pid");
        let bin = root.path().join("fake-worker");
        fs::write(
            &bin,
            format!(
                "#!/bin/sh\nprintf '%s\\n' \"$$\" > '{}'\n{behavior}\n",
                pid_file.display()
            ),
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");

        let session = "018f0000-0000-7000-8000-000000000113";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let ledger = folder.join("main.jsonl");
        write_test_genesis(&ledger, session);
        let store = Arc::new(provider::MemorySecretStore::new());
        store
            .publish(
                "shared",
                provider::SecretRecord::Active {
                    generation: 1,
                    material: "fixture-secret-never-log".to_owned(),
                },
            )
            .expect("active secret");
        let host = ProductionProcessHost::open_with_secret_store(
            root.path(),
            &bin,
            "test",
            root.path().join("missing-agent-home"),
            store as Arc<dyn provider::SecretStore>,
        )
        .expect("process host");
        let error = match host
            .spawn_worker_at_with_handshake_timeout(session, session, ledger, None, timeout)
        {
            Ok(_) => panic!("invalid handshake unexpectedly succeeded"),
            Err(error) => error,
        };
        assert_eq!(error.bootstrap_code(), "protocol-mismatch");
        if let Some(message) = expected_message {
            assert!(
                error.to_string().contains(message),
                "unexpected error: {error}"
            );
        }
        let pid = fs::read_to_string(&pid_file)
            .expect("worker pid witness")
            .trim()
            .parse::<libc::pid_t>()
            .expect("worker pid");
        // SAFETY: signal 0 performs a liveness probe and carries no pointers.
        assert_eq!(unsafe { libc::kill(pid, 0) }, -1);
        assert_eq!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::ESRCH),
            "the failed pre-adoption child must already be waited and gone",
        );
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn stop_cascade_durably_gates_the_entire_unpaired_spawn_graph() {
        exercise_stop_cascade(false);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn live_root_stop_receipt_precedes_the_same_durable_cascade() {
        exercise_stop_cascade(true);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn sweep_propagates_a_durable_root_stop_before_descendant_triage() {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        fs::create_dir_all(root.path().join("threads")).expect("threads");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        write_canonical_test_json(
            &root.path().join("config/settings.json"),
            json!({"format":1,"revision":1,"limits":{"max_workers":4,"max_provider_leases":1}}),
        );
        let bin = root.path().join("fake-worker");
        fs::write(
            &bin,
            b"#!/bin/sh\nprintf '%s\n' '{\"hello\":{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}'\nIFS= read -r selected\nwhile IFS= read -r line; do :; done\n",
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");

        let session = "018f0000-0000-7000-8000-000000000123";
        let child = "018f0000-0000-7000-8000-000000000124";
        let grandchild = "018f0000-0000-7000-8000-000000000125";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let main = folder.join("main.jsonl");
        let child_ledger = folder.join(format!("{child}.jsonl"));
        let grandchild_ledger = folder.join(format!("{grandchild}.jsonl"));
        write_test_spawn_line(&main, session, child, "main.jsonl", None, 5, "spawn-child");
        write_test_spawn_line(
            &child_ledger,
            child,
            grandchild,
            "main.jsonl",
            Some((5, "spawn-child")),
            3,
            "spawn-grandchild",
        );
        write_test_child_genesis(
            &grandchild_ledger,
            grandchild,
            &format!("{child}.jsonl"),
            3,
            "spawn-grandchild",
        );
        append_test_stop(&main, session, 1, 6);

        let host = ProductionProcessHost::open(
            root.path(),
            &bin,
            "test",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        host.release_for_test(session);
        host.boot_sweep().expect("boot sweep");

        for ledger in [&main, &child_ledger, &grandchild_ledger] {
            let bytes = fs::read(ledger).expect("descendant ledger");
            let projection = scan_valid_prefix(&bytes, schema::READER_VERSION)
                .projection
                .expect("descendant projection");
            assert!(
                projection.events.iter().any(|event| {
                    event.kind() == &EventKind::StopRequested
                        && event.integer_field("generation") == Some(1)
                }),
                "every descendant must be durably gated before its sweep classification",
            );
        }
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    fn exercise_stop_cascade(live_root: bool) {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        fs::create_dir_all(root.path().join("threads")).expect("threads");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        write_canonical_test_json(
            &root.path().join("config/settings.json"),
            json!({"format":1,"revision":1,"limits":{"max_workers":4,"max_provider_leases":1}}),
        );
        let bin = root.path().join("fake-worker");
        fs::write(
            &bin,
            b"#!/bin/sh\nprintf '%s\\n' '{\"hello\":{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}'\nIFS= read -r selected\nwhile IFS= read -r line; do\n  case \"$line\" in\n    *'\"stop\"'*)\n      printf '%s\\n' '{\"generation\":1,\"kind\":\"stop_requested\",\"origin_key\":\"root-stop\",\"origin_tuple\":{\"client\":\"test\",\"key\":\"root-stop\",\"op\":\"session.cancel\",\"principal\":\"test\",\"target\":\"018f0000-0000-7000-8000-000000000109\"},\"seq\":6,\"ts\":\"2026-08-28T00:00:05.000Z\",\"v\":1}' >> \"$1\"\n      sync\n      printf '%s\\n' '{\"receipt\":{\"deduplicated\":false,\"delivery\":\"root-stop\",\"seq\":6}}'\n      ;;\n  esac\ndone\n",
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");

        let session = "018f0000-0000-7000-8000-000000000109";
        let child = "018f0000-0000-7000-8000-000000000110";
        let grandchild = "018f0000-0000-7000-8000-000000000111";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let main = folder.join("main.jsonl");
        let child_ledger = folder.join(format!("{child}.jsonl"));
        let grandchild_ledger = folder.join(format!("{grandchild}.jsonl"));
        write_test_spawn_line(&main, session, child, "main.jsonl", None, 5, "spawn-child");
        write_test_spawn_line(
            &child_ledger,
            child,
            grandchild,
            "main.jsonl",
            Some((5, "spawn-child")),
            3,
            "spawn-grandchild",
        );
        write_test_child_genesis(
            &grandchild_ledger,
            grandchild,
            &format!("{child}.jsonl"),
            3,
            "spawn-grandchild",
        );

        let host = ProductionProcessHost::open(
            root.path(),
            &bin,
            "test",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        let original_root = if live_root {
            Some(
                host.schedule_worker_at(session, session, main.clone())
                    .expect("schedule root")
                    .expect("live root worker"),
            )
        } else {
            None
        };
        let receipt = host
            .cancel(
                session,
                "2026-08-28T00:00:05.000Z",
                &OriginTuple {
                    principal: "test".to_owned(),
                    client: "test".to_owned(),
                    target: session.to_owned(),
                    op: "session.cancel".to_owned(),
                    key: "root-stop".to_owned(),
                },
            )
            .expect("cancel and cascade");
        assert_eq!(receipt.seq, 6);

        for ledger in [&main, &child_ledger, &grandchild_ledger] {
            let bytes = fs::read(ledger).expect("descendant ledger");
            let projection = scan_valid_prefix(&bytes, schema::READER_VERSION)
                .projection
                .expect("descendant projection");
            assert_eq!(
                projection
                    .events
                    .iter()
                    .filter(|event| event.kind() == &EventKind::StopRequested)
                    .filter_map(|event| event.integer_field("generation"))
                    .collect::<Vec<_>>(),
                vec![1],
                "each descendant must durably echo the root generation exactly once",
            );
        }
        if let Some(original_root) = original_root {
            for _ in 0..300 {
                let restarted = host
                    .workers
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .get(session)
                    .filter(|worker| worker.alive.load(Ordering::Acquire))
                    .is_some_and(|worker| !Arc::ptr_eq(worker, &original_root));
                if restarted {
                    host.shutdown();
                    return;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            host.shutdown();
            panic!("root reader exit did not re-drive its stop recovery run");
        }
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn max_one_worker_dependency_chain_progresses_without_admitting_unrelated_work() {
        let root = tempfile::tempdir().expect("process host root");
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");
        fs::create_dir_all(root.path().join("config")).expect("config");
        fs::create_dir_all(root.path().join("workspaces/ws")).expect("workspaces");
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[workspace.to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        write_canonical_test_json(
            &root.path().join("config/settings.json"),
            json!({"format":1,"revision":1,"limits":{"max_workers":1,"max_provider_leases":1}}),
        );
        let bin = root.path().join("fake-worker");
        fs::write(
            &bin,
            b"#!/bin/sh\nprintf '%s\\n' '{\"hello\":{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}'\nIFS= read -r selected\nwhile IFS= read -r line; do :; done\n",
        )
        .expect("fake worker");
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).expect("worker mode");
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").expect("fake helper");
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).expect("helper mode");

        let session = "018f0000-0000-7000-8000-000000000101";
        let child = "018f0000-0000-7000-8000-000000000102";
        let grandchild = "018f0000-0000-7000-8000-000000000103";
        let unrelated = "018f0000-0000-7000-8000-000000000104";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        write_test_genesis(&folder.join("main.jsonl"), session);
        write_test_genesis(&folder.join(format!("{child}.jsonl")), child);
        write_test_genesis(&folder.join(format!("{grandchild}.jsonl")), grandchild);
        write_test_genesis(&folder.join(format!("{unrelated}.jsonl")), unrelated);

        let host =
            ProductionProcessHost::open(root.path(), &bin, "test", root.path().join(".agent"))
                .expect("process host");
        host.schedule_worker_at(session, session, folder.join("main.jsonl"))
            .expect("parent schedule")
            .expect("parent starts");
        let child_key = format!("{session}:{child}");
        assert!(
            host.schedule_line(session, child, &format!("{child}.jsonl"))
                .expect("report wake for crashed nested parent")
                .is_none(),
            "a capacity-blocked report wake must queue the exact nested parent"
        );
        assert!(
            host.pending_workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .contains_key(&child_key)
        );
        // The locked approval answer must resume an already-spawned child even
        // while its waiting parent occupies the single ordinary worker slot.
        let parent_path = folder.join("main.jsonl");
        let child_path = folder.join(format!("{child}.jsonl"));
        append_test_input(&parent_path, session);
        let mut parent_bytes = fs::read(&parent_path).unwrap();
        for value in [
            json!({"v":1,"seq":3,"turn":1,"kind":"turn_open","ts":"2026-08-28T00:00:00.000Z","trigger":{"inputs":[2]}}),
            json!({"v":1,"seq":4,"turn":1,"kind":"spawn","ts":"2026-08-28T00:00:00.000Z","call":"delegated-call","child":format!("{child}.jsonl"),"spawn_id":"dependency-spawn","resume":"never","seed":{"kinds":["state"]}}),
        ] {
            let event = Event::decode(&serde_json::to_vec(&value).unwrap()).unwrap();
            parent_bytes.extend(event.canonical_bytes().unwrap());
            parent_bytes.push(b'\n');
        }
        fs::write(&parent_path, &parent_bytes).unwrap();
        let mut child_genesis: serde_json::Value =
            serde_json::from_slice(&fs::read(&child_path).unwrap()).unwrap();
        child_genesis["parent"] =
            json!({"file":"main.jsonl","seq":4,"spawn_id":"dependency-spawn"});
        let event = Event::decode(&serde_json::to_vec(&child_genesis).unwrap()).unwrap();
        let mut bytes = event.canonical_bytes().unwrap();
        bytes.push(b'\n');
        fs::write(&child_path, bytes).unwrap();
        assert_eq!(
            unresolved_parent_dependency(root.path(), session, child, &child_path).unwrap(),
            Some(session.to_owned())
        );
        host.ensure_after_locked_append(&child_key)
            .expect("approved child resumes despite max_workers=1");
        assert!(host.live_worker(&child_key).is_some());
        // A forged parent reference cannot acquire dependency admission.
        child_genesis["parent"]["spawn_id"] = json!("forged");
        let event = Event::decode(&serde_json::to_vec(&child_genesis).unwrap()).unwrap();
        let mut bytes = event.canonical_bytes().unwrap();
        bytes.push(b'\n');
        fs::write(&child_path, bytes).unwrap();
        assert!(unresolved_parent_dependency(root.path(), session, child, &child_path).is_err());
        let grandchild_key = format!("{session}:{grandchild}");
        host.schedule_child_from_parent(
            &child_key,
            &grandchild_key,
            session,
            folder.join(format!("{grandchild}.jsonl")),
        )
        .expect("grandchild dependency schedule")
        .expect("grandchild starts despite max_workers=1");
        assert_eq!(
            host.workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .values()
                .filter(|worker| worker.alive.load(Ordering::Acquire))
                .count(),
            3
        );

        let unrelated_key = format!("{session}:{unrelated}");
        assert!(
            host.schedule_worker_at(
                &unrelated_key,
                session,
                folder.join(format!("{unrelated}.jsonl")),
            )
            .expect("unrelated schedule")
            .is_none()
        );
        assert!(
            host.pending_workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .contains_key(&unrelated_key)
        );

        let grandchild_worker = host
            .workers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(&grandchild_key)
            .cloned()
            .expect("grandchild worker");
        grandchild_worker
            .child
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .kill()
            .expect("kill grandchild");
        for _ in 0..100 {
            if !grandchild_worker.alive.load(Ordering::Acquire) {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(!grandchild_worker.alive.load(Ordering::Acquire));
        assert!(
            host.pending_workers
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .contains_key(&unrelated_key),
            "child exit must not admit unrelated work while the waiting chain is live"
        );
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn prompt_after_confirmed_worker_exit_reuses_locked_origin() {
        let root = tempfile::tempdir().unwrap();
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).unwrap();
        fs::create_dir_all(root.path().join("workspaces/ws")).unwrap();
        write_canonical_test_json(
            &root.path().join("workspaces/ws/workspace.json"),
            json!({"format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],
                "policy":{"allowed_tools":[],"writable_roots":[],"network":false}}),
        );
        let bin = root.path().join("fake-worker");
        fs::write(&bin,b"#!/bin/sh\nprintf '%s\\n' '{\"hello\":{\"max\":2,\"min\":2,\"proto\":\"tekes-worker\"}}'\nIFS= read -r selected\nexit 0\n").unwrap();
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o700)).unwrap();
        let helper = root.path().join("tekes-helper");
        fs::write(&helper, b"#!/bin/sh\nexit 0\n").unwrap();
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).unwrap();
        let session = "018f0000-0000-7000-8000-000000000139";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).unwrap();
        let path = folder.join("main.jsonl");
        write_test_genesis(&path, session);
        let host =
            ProductionProcessHost::open(root.path(), &bin, "test", root.path().join(".agent"))
                .unwrap();
        let worker = host
            .spawn_worker_at(session, session, path.clone(), None)
            .unwrap();
        assert!(child_exited_within(&worker.child, Duration::from_secs(2)));
        assert!(worker.receipt("never-delivered").is_err());
        // Recreate the observed interval where selection still sees an old handle.
        worker.alive.store(true, Ordering::Release);
        host.workers.lock().unwrap().insert(session.into(), worker);
        let origin = OriginTuple {
            principal: "test".into(),
            client: "test".into(),
            target: session.into(),
            op: "session.prompt".into(),
            key: "exit-input".into(),
        };
        let prompt = MaterializedPrompt {
            blocks: vec![schema::Block::Text {
                text: "next input".into(),
            }],
            attachments: vec![],
            files: Vec::new(),
        };
        let first = SessionDeliveryAuthority::prompt(
            &*host,
            session,
            "2026-09-04T00:00:00.000Z",
            &origin,
            &prompt,
            false,
        )
        .unwrap();
        host.shutdown();
        let ledger = LockedLedger::open(&path, schema::READER_VERSION).unwrap();
        let rows = &ledger.projection().unwrap().events;
        assert_eq!(
            rows.iter()
                .filter(|e| e.kind() == &EventKind::Input)
                .count(),
            1
        );
        assert_eq!(
            ledger.projection().unwrap().origin_tuples.get(&origin),
            Some(&first.seq)
        );
    }

    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    fn write_canonical_test_json(path: &Path, value: Value) {
        let mut bytes = serde_json_canonicalizer::to_vec(&value).expect("canonical JSON");
        bytes.push(b'\n');
        fs::write(path, bytes).expect("canonical test file");
    }

    #[cfg(target_os = "macos")]
    #[test]
    #[ignore = "requires built real worker/helper; TEKES_TEST_REAL_WORKER points to worker"]
    fn real_worker_recovers_final_before_settlement_with_resume_never() {
        for boundary in [
            "output",
            "candidate",
            "decision",
            "validator_death",
            "queued_after_validator_death",
        ] {
            let needs_validator = boundary.contains("validator_death");
            let queued = boundary == "queued_after_validator_death";
            let worker =
                PathBuf::from(std::env::var("TEKES_TEST_REAL_WORKER").expect("real worker path"));
            let root = tempfile::tempdir().unwrap();
            let workspace = root.path().join("workspace");
            fs::create_dir_all(&workspace).unwrap();
            fs::create_dir_all(root.path().join("workspaces/ws")).unwrap();
            write_canonical_test_json(
                &root.path().join("workspaces/ws/workspace.json"),
                json!({"format":1,"revision":1,"id":"ws","name":"ws","cwd":[workspace],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}}),
            );
            let session = "018f0000-0000-7000-8000-000000000125";
            let folder = root.path().join("threads").join(session);
            fs::create_dir_all(folder.join("assets")).unwrap();
            let path = folder.join("main.jsonl");
            write_test_genesis(&path, session);
            append_test_input(&path, session);
            let asset = AssetStore::new(folder.join("assets"))
                .unwrap()
                .publish(b"{}")
                .unwrap();
            let mut bytes = fs::read(&path).unwrap();
            let records = [
                json!({"kind":"run_start","run":"before-crash","mode":"ordinary","binary":"worker","config_digest":"test","instruction_digest":"ins","policy":"p","recovery_ordinal":0}),
                json!({"kind":"turn_open","turn":1,"trigger":{"inputs":[2]}}),
                json!({"kind":"epoch","id":"e1","adapter":"responses","model":"m","reason":"initial","renderer":1,"system":{"asset":asset.asset,"digest":"d"},"tools":{"asset":"sha256-td","digest":"td"}}),
                json!({"kind":"attempt","request":{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096},"turn":1,"attempt":"a1","epoch":"e1","admits":[{"from":2,"to":2}],"wire_digest":"w"}),
                json!({"kind":"output","turn":1,"attempt":"a1","final_answer":true,"content":[{"type":"text","text":"immutable candidate"}],"sealed":{"version":1,"adapter":"responses","fragments":"[]"},"usage":{"availability":"reported","input_tokens":"1","output_tokens":"1"}}),
            ];
            let output_seq = records
                .iter()
                .position(|record| record["kind"] == "output")
                .unwrap() as u64
                + 3;
            for (index, mut raw) in records.into_iter().enumerate() {
                raw["v"] = json!(1);
                raw["seq"] = json!(index + 3);
                raw["ts"] = json!("2026-08-28T00:00:02.000Z");
                let event = schema::Event::decode(&serde_json::to_vec(&raw).unwrap()).unwrap();
                bytes.extend(event.canonical_bytes().unwrap());
                bytes.push(b'\n');
            }
            fs::write(&path, bytes).unwrap();
            if boundary != "output" {
                let mut ledger = store::LockedLedger::open(&path, schema::READER_VERSION).unwrap();
                let mut binding = engine::ValidationBinding {
                    thread: session.into(),
                    worker: session.into(),
                    turn: 1,
                    output_seq,
                    snapshot: Default::default(),
                };
                if needs_validator {
                    use sha2::Digest;
                    let artifact = workspace.join("proof.txt");
                    fs::write(&artifact, b"frozen draft").unwrap();
                    binding.snapshot.insert(
                        artifact.to_string_lossy().into_owned(),
                        format!("sha256-{:x}", sha2::Sha256::digest(b"frozen draft")),
                    );
                }
                let candidate =
                    engine::begin_validation(&mut ledger, "2026-08-28T00:00:03.000Z", &binding)
                        .unwrap();
                if boundary == "decision" {
                    engine::commit_validation_decision(
                        &mut ledger,
                        "2026-08-28T00:00:04.000Z",
                        candidate,
                        candidate,
                    )
                    .unwrap();
                }
            }
            if queued {
                let mut ledger = store::LockedLedger::open(&path, schema::READER_VERSION).unwrap();
                let raw = json!({"v":1,"seq":ledger.next_seq(),"kind":"input","ts":"2026-08-28T00:00:05.000Z",
                "content":[{"type":"text","text":"重试"}],"origin_key":"queued-second",
                "origin_tuple":{"principal":"test","client":"test","target":session,"op":"submit","key":"queued-second"}});
                ledger
                    .append(
                        schema::Event::decode(&serde_json::to_vec(&raw).unwrap()).unwrap(),
                        true,
                    )
                    .unwrap();
            }
            let prefix_before_recovery = fs::read(&path).unwrap();
            assert!(ledger_needs_worker(&path).unwrap());
            let host = ProductionProcessHost::open(
                root.path(),
                worker,
                "test",
                root.path().join(".agent"),
            )
            .unwrap();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                host.schedule_main(session)
                    .unwrap()
                    .expect("real process launched");
                let deadline = Instant::now() + Duration::from_secs(15);
                loop {
                    let text = fs::read_to_string(&path).unwrap();
                    let events = text
                        .lines()
                        .map(|line| serde_json::from_str::<Value>(line).unwrap())
                        .collect::<Vec<_>>();
                    if let Some(settle) = events.iter().find(|e| e["kind"] == "settle") {
                        if boundary == "output" {
                            assert!(settle.get("validation").is_none(), "{text}");
                            assert_eq!(settle["promoted_output_seq"], output_seq);
                        } else {
                            assert_eq!(
                                settle["validation"]["outcome"],
                                if needs_validator {
                                    "inconclusive"
                                } else {
                                    "not_required"
                                },
                                "{text}"
                            );
                            assert_eq!(settle["validation"]["promoted_output_seq"], output_seq);
                        }
                        assert_eq!(
                            events
                                .iter()
                                .filter(|e| e["kind"] == "settle" && e["turn"] == 1)
                                .count(),
                            1
                        );
                        if queued {
                            if let Some(second) = events
                                .iter()
                                .find(|e| e["kind"] == "settle" && e["turn"] == 2)
                            {
                                let opened = events
                                    .iter()
                                    .find(|e| e["kind"] == "turn_open" && e["turn"] == 2)
                                    .unwrap();
                                assert!(
                                    settle["seq"].as_u64().unwrap()
                                        < opened["seq"].as_u64().unwrap(),
                                    "queued input overtook validation"
                                );
                                let queued_input = events
                                    .iter()
                                    .find(|event| {
                                        event["kind"] == "input"
                                            && event["origin_key"] == "queued-second"
                                    })
                                    .unwrap();
                                assert_eq!(queued_input["content"][0]["text"], "重试");
                                assert_eq!(
                                    opened["trigger"]["inputs"],
                                    json!([queued_input["seq"]])
                                );
                                assert_eq!(
                                    second["outcome"], "error",
                                    "no provider configured for second turn"
                                );
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    assert!(
                        Instant::now() < deadline,
                        "real worker failed to settle: {text}"
                    );
                    std::thread::sleep(Duration::from_millis(25));
                }
            }));
            host.shutdown();
            if let Err(error) = result {
                std::panic::resume_unwind(error);
            }
            assert!(
                fs::read(&path)
                    .unwrap()
                    .starts_with(&prefix_before_recovery),
                "recovery must preserve every byte of the existing prefix at {boundary}"
            );
            assert!(!ledger_needs_worker(&path).unwrap());
            let records = fs::read_to_string(&path)
                .unwrap()
                .lines()
                .map(|line| serde_json::from_str::<Value>(line).unwrap())
                .collect::<Vec<_>>();
            for subkind in ["validation.candidate", "validation.decision"] {
                assert_eq!(
                    records.iter().filter(|e| e["subkind"] == subkind).count(),
                    if boundary == "output" {
                        0
                    } else if needs_validator && subkind == "validation.decision" {
                        2
                    } else {
                        1
                    },
                    "recovery duplicated {subkind} at {boundary}"
                );
            }
            if needs_validator {
                assert_eq!(
                    records
                        .iter()
                        .filter(|e| e["subkind"] == "validation.death")
                        .count(),
                    1
                );
                let spawn = records
                    .iter()
                    .find(|e| e["kind"] == "spawn")
                    .expect("actual validator spawn");
                let child =
                    fs::read_to_string(folder.join(spawn["child"].as_str().unwrap())).unwrap();
                let child = child
                    .lines()
                    .map(|line| serde_json::from_str::<Value>(line).unwrap())
                    .collect::<Vec<_>>();
                assert!(
                    child.iter().any(|e| e["kind"] == "run_start"),
                    "validator process never ran"
                );
                assert!(
                    child
                        .iter()
                        .any(|e| e["kind"] == "settle" && e["outcome"] == "error")
                );
                assert_eq!(
                    records
                        .iter()
                        .filter(|e| e["kind"] == "child_result")
                        .count(),
                    1
                );
            }
        }
    }

    const HELD_TS: &str = "2026-08-26T09:00:00.000Z";

    /// A main ledger parked on an unanswered `ask_user` hold (call `ask1`, seq 9).
    fn held_ledger_lines(thread: &str) -> Vec<String> {
        vec![
            format!(
                r#"{{"config":{{"digest":"cfg"}},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,"origin_key":"create","origin_tuple":{{"client":"cli","key":"create","op":"create","principal":"p","target":"{thread}"}},"resume":"never","seq":1,"thread":"{thread}","ts":"{HELD_TS}","v":1,"workspace":"ws"}}"#
            ),
            format!(
                r#"{{"content":[{{"text":"start","type":"text"}}],"kind":"input","origin_key":"i1","origin_tuple":{{"client":"cli","key":"i1","op":"submit","principal":"p","target":"{thread}"}},"seq":2,"ts":"{HELD_TS}","v":1}}"#
            ),
            format!(
                r#"{{"binary":"tekes-worker-1","config_digest":"cfg","instruction_digest":"ins","kind":"run_start","mode":"ordinary","policy":"default","recovery_ordinal":0,"run":"r1","seq":3,"ts":"{HELD_TS}","v":1}}"#
            ),
            format!(
                r#"{{"kind":"turn_open","seq":4,"trigger":{{"inputs":[2]}},"ts":"{HELD_TS}","turn":1,"v":1}}"#
            ),
            format!(
                r#"{{"adapter":"fake","id":"e1","kind":"epoch","model":"fake","reason":"initial","renderer":1,"seq":5,"system":{{"asset":"sha256-aa","digest":"d1"}},"tools":{{"asset":"sha256-td","digest":"td"}},"ts":"{HELD_TS}","v":1}}"#
            ),
            format!(
                r#"{{"admits":[{{"from":2,"to":2}}],"attempt":"a1","epoch":"e1","kind":"attempt","request":{{"asset":"sha256-abababababababababababababababababababababababababababababababab","bytes":4096}},"seq":6,"ts":"{HELD_TS}","turn":1,"v":1,"wire_digest":"wd"}}"#
            ),
            format!(
                r#"{{"attempt":"a1","content":[{{"args":{{"question":"continue?"}},"call":"ask1","name":"ask_user","type":"tool-call"}}],"kind":"output","sealed":{{"adapter":"fake","fragments":"ask","version":1}},"seq":7,"ts":"{HELD_TS}","turn":1,"usage":{{"availability":"reported","input_tokens":"5","output_tokens":"1"}},"v":1}}"#
            ),
            format!(
                r#"{{"args":{{"question":"continue?"}},"attempt":"a1","call":"ask1","kind":"tool_call","name":"ask_user","seq":8,"source":"provider","ts":"{HELD_TS}","turn":1,"v":1}}"#
            ),
            format!(
                r#"{{"call":"ask1","kind":"approval_request","scope":"answer","seq":9,"ts":"{HELD_TS}","turn":1,"v":1}}"#
            ),
        ]
    }

    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    fn held_response_line(thread: &str) -> String {
        format!(
            r#"{{"call":"ask1","grant":true,"kind":"approval_response","origin_key":"r1","origin_tuple":{{"client":"cli","key":"r1","op":"approve","principal":"p","target":"{thread}"}},"seq":10,"ts":"{HELD_TS}","turn":1,"v":1}}"#
        )
    }

    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    fn held_host(
        root: &Path,
        session: &str,
        extra: &str,
    ) -> (
        Arc<ProductionProcessHost>,
        ProductionCarrierStreams,
        PathBuf,
    ) {
        let folder = root.join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        let ledger = folder.join("main.jsonl");
        fs::write(
            &ledger,
            held_ledger_lines(session).join("\n") + "\n" + extra,
        )
        .expect("held ledger");
        let host = ProductionProcessHost::open(
            root,
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.join("missing-agent-home"),
        )
        .expect("process host");
        let streams = ProductionCarrierStreams::new(root);
        host.attach_streams(streams.clone());
        (host, streams, ledger)
    }

    /// `publish_appended` leaves a card the reader trusts: it classifies Fresh, its summary
    /// equals the scan-derived inventory row, and its holds follow the ledger (unresolved, then
    /// resolved after the response is appended and the thread is projected again).
    #[cfg(target_os = "macos")]
    #[test]
    fn publish_appended_writes_a_fresh_card_that_matches_the_inventory_scan() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000a4";
        let (host, _streams, ledger) = held_host(root.path(), session, "");
        host.publish_appended(session).expect("publish");
        host.card_writer.shutdown();
        let folder = root.path().join("threads").join(session);
        let card = store::card::read_card(&folder).expect("card written");
        let classified = store::card::classify(&folder, &card).expect("classify");
        assert_eq!(classified.freshness(), store::card::Freshness::Fresh);
        assert_eq!(classified.journal, store::card::JournalVerdict::Fresh);
        assert_eq!(card.lines[0].holds.len(), 1);
        assert!(card.lines[0].holds[0].resolved.is_none());
        assert_eq!(card.lines[0].last_seq, 9);
        // The List row derived from the card equals the row a full scan produces.
        let scanned = NativeEndpoint::open(root.path())
            .expect("endpoint")
            .list_sessions(&HashSet::new())
            .expect("scan")
            .into_iter()
            .find(|item| item.session_id == session)
            .expect("row");
        assert_eq!(card.summary.workspace, scanned.workspace_id);
        assert_eq!(card.summary.title, scanned.title);
        assert_eq!(card.summary.updated_ms, scanned.updated_at);
        assert_eq!(card.summary.blank, scanned.blank);
        assert_eq!(card.summary.ephemeral, scanned.ephemeral);
        assert_eq!(card.summary.identity_profile, scanned.identity_profile);
        assert_eq!(card.lines[0].facts, scanned.lifecycle);
        assert_eq!(card.lines[0].last_seq, scanned.as_of_seq);

        // A later append makes the card Ahead (never wrong), and a new projection re-writes it
        // with the hold resolved.
        {
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(&ledger)
                .expect("append");
            file.write_all((held_response_line(session) + "\n").as_bytes())
                .expect("approval_response");
        }
        let classified = store::card::classify(&folder, &card).expect("classify");
        assert_eq!(classified.freshness(), store::card::Freshness::Ahead);
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn card_writer_coalesces_and_flushes_at_shutdown() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000a5";
        let (host, _streams, ledger) = held_host(root.path(), session, "");
        host.publish_appended(session).expect("first publish");
        // Appends while the debounce window is open are coalesced into one final write.
        {
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(&ledger)
                .expect("append");
            file.write_all((held_response_line(session) + "\n").as_bytes())
                .expect("approval_response");
        }
        host.publish_appended(session).expect("second publish");
        host.shutdown();
        let folder = root.path().join("threads").join(session);
        let card = store::card::read_card(&folder).expect("card");
        assert_eq!(
            card.lines[0].last_seq, 10,
            "the flush wrote the newest state"
        );
        assert!(card.lines[0].holds[0].resolved.is_some());
        assert_eq!(
            store::card::classify(&folder, &card)
                .expect("classify")
                .freshness(),
            store::card::Freshness::Fresh
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn publish_appended_fast_path_publishes_the_pending_hold_from_one_decode() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000a1";
        let (host, streams, _) = held_host(root.path(), session, "");
        host.publish_appended(session).expect("publish");
        assert_eq!(
            streams.published_pending_for_test(session, None).len(),
            1,
            "the unanswered hold is pending"
        );
        host.shutdown();
    }

    /// A respond appends `approval_response` and reconciles the actionables while
    /// `publish_appended` is between its ledger decode and its actionables diff. The stale
    /// decode must not re-publish the resolved hold (the regression `reconcile_after_response`
    /// exists to prevent).
    #[cfg(target_os = "macos")]
    #[test]
    fn publish_appended_with_a_grown_ledger_does_not_resurrect_a_resolved_hold() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000a2";
        let (host, streams, ledger) = held_host(root.path(), session, "");
        // Baseline: hold published as pending.
        host.publish_appended(session).expect("first publish");
        assert_eq!(streams.published_pending_for_test(session, None).len(), 1);
        let respond = || {
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(&ledger)
                .expect("ledger append");
            file.write_all((held_response_line(session) + "\n").as_bytes())
                .expect("append approval_response");
            drop(file);
            streams
                .reconcile_actionables(session, None)
                .expect("reconcile after response");
        };
        host.publish_appended_with(session, Some(&respond))
            .expect("publish with a concurrent respond");
        assert!(
            streams.published_pending_for_test(session, None).is_empty(),
            "a resolved hold must stay resolved after a stale decode"
        );
        host.shutdown();
    }

    /// A torn tail keeps the pre-existing behaviour: the separate-read paths reject it
    /// ("requires tail recovery") and the call reports the error after the journal reconcile.
    /// E831 E1: after a discard, a thread created with the same id starts at a small ledger
    /// `last_seq`; the monotonic guard must not freeze its actionable set at the old value.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_reused_session_id_does_not_inherit_the_old_actionable_high_water_mark() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000a7";
        let (host, streams, ledger) = held_host(root.path(), session, "");
        // Ephemeral genesis so the thread can be discarded.
        let text = fs::read_to_string(&ledger).expect("ledger");
        // canonical key order: "config" < "ephemeral" < "format"
        let ephemeral = text.replacen(r#""format":1,"#, r#""ephemeral":true,"format":1,"#, 1);
        assert_ne!(text, ephemeral);
        fs::write(&ledger, &ephemeral).expect("ephemeral ledger");
        // old life: the hold is answered, the published set is empty at last_seq 10
        {
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(&ledger)
                .expect("append");
            file.write_all((held_response_line(session) + "\n").as_bytes())
                .expect("approval_response");
        }
        host.publish_appended(session).expect("old life");
        assert!(streams.published_pending_for_test(session, None).is_empty());
        // discard (announces Removed), then the id is reused by a thread whose ledger is
        // shorter (last_seq 9) and has its hold pending
        store::ThreadStore::open(root.path())
            .expect("store")
            .discard_ephemeral(session)
            .expect("discard");
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("new folder");
        fs::write(
            folder.join("main.jsonl"),
            held_ledger_lines(session).join("\n") + "\n",
        )
        .expect("new ledger");
        host.publish_appended(session).expect("new life");
        assert_eq!(
            streams.published_pending_for_test(session, None).len(),
            1,
            "the reused id's pending hold is published, not refused as 'older'"
        );
        host.shutdown();
    }

    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    fn held_thread_on_disk(root: &Path, session: &str) -> PathBuf {
        let folder = root.join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        fs::write(
            folder.join("main.jsonl"),
            held_ledger_lines(session).join("\n") + "\n",
        )
        .expect("ledger");
        folder
    }

    /// T3: the List built from the catalog (cards after verification) is the List a full scan
    /// produces, for every field the row carries; a thread whose verification has not finished
    /// inside the wait budget is listed with a neutral placeholder and becomes the real row
    /// afterwards.
    #[cfg(target_os = "macos")]
    #[test]
    fn the_catalog_list_equals_the_scan_list_and_a_late_thread_is_a_placeholder() {
        let root = tempfile::tempdir().expect("root");
        let ids = (0..14)
            .map(|n| format!("018f0000-0000-7000-8000-0000000000b{n:x}"))
            .collect::<Vec<_>>();
        for (n, id) in ids.iter().enumerate() {
            match n % 7 {
                // a pending approval hold
                0 | 1 => {
                    held_thread_on_disk(root.path(), id);
                }
                // genesis only: a blank thread
                2 | 3 => {
                    let folder = root.path().join("threads").join(id);
                    fs::create_dir_all(folder.join("assets")).expect("folder");
                    write_test_genesis(&folder.join("main.jsonl"), id);
                }
                // genesis and a submitted input
                4 => {
                    let folder = root.path().join("threads").join(id);
                    fs::create_dir_all(folder.join("assets")).expect("folder");
                    write_test_genesis(&folder.join("main.jsonl"), id);
                    append_test_input(&folder.join("main.jsonl"), id);
                }
                // an ephemeral scratch thread this process may list
                5 => {
                    let folder = held_thread_on_disk(root.path(), id);
                    let text = fs::read_to_string(folder.join("main.jsonl")).expect("ledger");
                    fs::write(
                        folder.join("main.jsonl"),
                        text.replacen(r#""format":1,"#, r#""ephemeral":true,"format":1,"#, 1),
                    )
                    .expect("ephemeral");
                }
                // a durable stop
                _ => {
                    let folder = root.path().join("threads").join(id);
                    fs::create_dir_all(folder.join("assets")).expect("folder");
                    write_test_genesis(&folder.join("main.jsonl"), id);
                    append_test_input(&folder.join("main.jsonl"), id);
                    append_test_stop(&folder.join("main.jsonl"), id, 1, 3);
                }
            }
        }
        // one archived thread: the catalog lists its id only
        let archived = "018f0000-0000-7000-8000-0000000000ba";
        let archived_folder = held_thread_on_disk(root.path(), archived);
        fs::create_dir_all(root.path().join("archive")).expect("archive");
        store::move_thread_folder(
            &archived_folder,
            &root.path().join("archive").join(archived),
        )
        .expect("archive move");
        let (host, _streams) = open_host(root.path());
        // This process may list the ephemeral thread it owns.
        host.arm_admission();
        store::folder_events::arm_admission(root.path());
        // Nothing is verified yet: a zero budget lists placeholders (neutral tail, no title)
        // without failing.
        let early = host
            .build_inventory(false, Duration::ZERO)
            .expect("early list");
        assert!(early.iter().all(|item| item.workspace_id == "ws"));
        // Verification proceeds in the background; a patient list is the scan list.
        let patient = host
            .build_inventory(true, Duration::from_secs(120))
            .expect("patient list");
        let scanned = NativeEndpoint::open(root.path())
            .expect("endpoint")
            .list_sessions(&HashSet::new())
            .expect("scan");
        let active = |items: &[SessionInventoryItem]| {
            items
                .iter()
                .filter(|item| !item.archived)
                .cloned()
                .collect::<Vec<_>>()
        };
        let (left, right) = (active(&patient), active(&scanned));
        // A previous process's ephemeral scratch threads are hidden until cleanup removes them.
        assert_eq!(right.iter().filter(|item| item.ephemeral).count(), 2);
        let right = right
            .into_iter()
            .filter(|item| !item.ephemeral)
            .collect::<Vec<_>>();
        let names = |rows: &[SessionInventoryItem]| {
            rows.iter()
                .map(|row| row.session_id.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(names(&left), names(&right));
        for (left, right) in left.iter().zip(&right) {
            assert_eq!(left, right, "row {}", left.session_id);
        }
        let archived_ids = patient
            .iter()
            .filter(|item| item.archived)
            .map(|item| item.session_id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(archived_ids, [archived]);
        // The single-row path agrees with the list.
        let one = host
            .inventory_item(&ids[1])
            .expect("catalog backed")
            .expect("ok")
            .expect("row");
        assert_eq!(one, left[1]);
        // At least one row of each lifecycle shape was compared.
        assert!(left.iter().any(|item| item.blank));
        assert!(left.iter().any(|item| !item.blank));
        assert!(
            left.iter()
                .any(|item| item.lifecycle.stop_active || item.lifecycle.open_hold)
        );
        host.shutdown();
    }

    /// T6b/T6d: a session that existed before this process is deferred; one this process
    /// created is not; an archived thread unarchived through the store chokepoint is admitted,
    /// and nothing else is.
    #[cfg(target_os = "macos")]
    #[test]
    fn only_threads_this_process_published_or_unarchived_are_admitted() {
        let root = tempfile::tempdir().expect("root");
        let old = "018f0000-0000-7000-8000-0000000000c1";
        let archived = "018f0000-0000-7000-8000-0000000000c2";
        held_thread_on_disk(root.path(), old);
        let archived_folder = held_thread_on_disk(root.path(), archived);
        fs::create_dir_all(root.path().join("archive")).expect("archive");
        let in_archive = root.path().join("archive").join(archived);
        store::move_thread_folder(&archived_folder, &in_archive).expect("archive move");
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        host.arm_admission();
        assert!(host.is_deferred(old), "a pre-existing thread is deferred");
        assert!(host.is_deferred(archived));
        store::move_thread_folder(&in_archive, &root.path().join("threads").join(archived))
            .expect("unarchive move");
        assert!(!host.is_deferred(archived), "unarchive admits");
        assert!(host.is_deferred(old), "the other thread stays deferred");
        host.release_for_test(old);
        assert!(!host.is_deferred(old), "a released thread is not deferred");
        host.shutdown();
    }

    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    fn open_host(root: &Path) -> (Arc<ProductionProcessHost>, ProductionCarrierStreams) {
        fs::create_dir_all(root.join("workspaces/ws")).expect("workspaces");
        write_canonical_test_json(
            &root.join("workspaces/ws/workspace.json"),
            json!({
                "format":1,"revision":1,"id":"ws","name":"ws",
                "cwd":[root.join("workspace").to_string_lossy()],
                "policy":{"allowed_tools":[],"network":false,"writable_roots":[]}
            }),
        );
        let host = ProductionProcessHost::open(
            root,
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.join("missing-agent-home"),
        )
        .expect("process host");
        let streams = ProductionCarrierStreams::new(root);
        host.attach_streams(streams.clone());
        (host, streams)
    }

    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    fn poll_until_ready<F: Future>(future: F) -> F::Output {
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let mut future = std::pin::pin!(future);
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
                return value;
            }
            assert!(Instant::now() < deadline, "future stayed pending");
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    /// E832 M1/M4: a thread created after the catalog loaded is in the List, in the sweep set
    /// and in a second client's inventory baseline at once, and its single-row inventory entry
    /// is the verified blank row, not a placeholder.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_thread_published_after_the_catalog_loaded_is_listed_swept_and_blank() {
        let root = tempfile::tempdir().expect("root");
        let old = "018f0000-0000-7000-8000-0000000000d1";
        let fresh = "018f0000-0000-7000-8000-0000000000d2";
        held_thread_on_disk(root.path(), old);
        let (host, streams) = open_host(root.path());
        host.arm_admission();
        host.catalog.ensure_loaded();
        assert_eq!(host.catalog.entries_now().len(), 1);
        let genesis = schema::Event::decode_canonical(held_ledger_lines(fresh)[0].as_bytes())
            .expect("genesis");
        store::ThreadStore::open(root.path())
            .expect("store")
            .create_thread(fresh, genesis)
            .expect("create after load");
        // nobody asked for it: it is in the catalog, the sweep and the List already
        assert!(host.catalog.entries_now().iter().any(|(id, _)| id == fresh));
        assert!(host.sweep_ids().iter().any(|id| id == fresh));
        assert!(!host.is_deferred(fresh));
        let list = host
            .build_inventory(false, Duration::from_secs(60))
            .expect("list");
        let row = list
            .iter()
            .find(|item| item.session_id == fresh)
            .expect("new session listed");
        assert!(row.blank);
        // the single-row path (host/session-added) serves the verified row
        let one = host
            .inventory_item(fresh)
            .expect("catalog backed")
            .expect("ok")
            .expect("row");
        assert!(one.blank);
        assert_eq!(one.workspace_id, "ws");
        // a second client's inventory baseline lists both
        let mut inventory = streams
            .open_mux(7, endpoint::SessionStreamTarget::SessionInventory)
            .expect("inventory stream");
        let endpoint::SessionSyncFrame::InventoryBaseline { items, .. } =
            poll_until_ready(inventory.recv())
                .expect("baseline item")
                .expect("baseline frame")
        else {
            panic!("expected an inventory baseline")
        };
        assert_eq!(items.len(), 2);
        host.shutdown();
    }

    /// E832 M2: hydrations of different threads fold concurrently (they all reach the point
    /// after the fold before any proceeds), and a doorbell for another thread completes while a
    /// cold hydration is held outside the lock.
    #[cfg(target_os = "macos")]
    #[test]
    fn hydrations_run_in_parallel_and_never_hold_up_a_doorbell() {
        let root = tempfile::tempdir().expect("root");
        let ids = (0..8)
            .map(|n| format!("018f0000-0000-7000-8000-0000000000f{n}"))
            .collect::<Vec<_>>();
        for id in &ids {
            held_thread_on_disk(root.path(), id);
        }
        let (host, _streams) = open_host(root.path());
        let arrived = std::sync::atomic::AtomicUsize::new(0);
        std::thread::scope(|scope| {
            for id in &ids {
                let (host, arrived) = (&host, &arrived);
                scope.spawn(move || {
                    let rendezvous = || {
                        arrived.fetch_add(1, Ordering::SeqCst);
                        let deadline = Instant::now() + Duration::from_secs(30);
                        while arrived.load(Ordering::SeqCst) < 8 {
                            assert!(
                                Instant::now() < deadline,
                                "hydrations were serialised: only {} reached the fold",
                                arrived.load(Ordering::SeqCst)
                            );
                            std::thread::sleep(Duration::from_millis(5));
                        }
                    };
                    host.hydrate_with(id, Some(&rendezvous)).expect("hydrate");
                });
            }
        });
        for id in &ids {
            assert_eq!(
                host.catalog.get(id).expect("entry").trust,
                crate::catalog::Trust::Fresh
            );
        }
        // a doorbell is not queued behind a cold hydration
        let cold = "018f0000-0000-7000-8000-0000000000fa";
        let live = "018f0000-0000-7000-8000-0000000000fb";
        held_thread_on_disk(root.path(), cold);
        held_thread_on_disk(root.path(), live);
        let release = std::sync::atomic::AtomicBool::new(false);
        let holding = std::sync::atomic::AtomicBool::new(false);
        std::thread::scope(|scope| {
            let (host_ref, release, holding) = (&host, &release, &holding);
            let slow = scope.spawn(move || {
                let hold = || {
                    holding.store(true, Ordering::SeqCst);
                    let deadline = Instant::now() + Duration::from_secs(30);
                    while !release.load(Ordering::SeqCst) {
                        assert!(Instant::now() < deadline, "hydration never released");
                        std::thread::sleep(Duration::from_millis(5));
                    }
                };
                host_ref
                    .hydrate_with(cold, Some(&hold))
                    .expect("slow hydrate");
            });
            let deadline = Instant::now() + Duration::from_secs(30);
            while !holding.load(Ordering::SeqCst) {
                assert!(Instant::now() < deadline, "cold hydration never started");
                std::thread::sleep(Duration::from_millis(5));
            }
            host.publish_appended(live)
                .expect("a doorbell completes while a cold hydration is in flight");
            release.store(true, Ordering::SeqCst);
            slow.join().expect("slow hydrate thread");
        });
        host.shutdown();
    }

    /// E832 M3: a complete but invalid line reaches Corrupt through hydrate, is listed as a
    /// quarantined row, answers with a quarantine error and is not re-hydrated by every List.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_complete_invalid_line_quarantines_and_is_not_rehydrated() {
        let root = tempfile::tempdir().expect("root");
        let good = "018f0000-0000-7000-8000-0000000000c9";
        let bad = "018f0000-0000-7000-8000-0000000000c8";
        held_thread_on_disk(root.path(), good);
        let folder = held_thread_on_disk(root.path(), bad);
        {
            let mut ledger = fs::OpenOptions::new()
                .append(true)
                .open(folder.join("main.jsonl"))
                .expect("ledger");
            // complete (LF-terminated) but not a valid record, followed by a valid-looking one
            ledger
                .write_all(b"{\"not\":\"an event\"}\n")
                .expect("invalid line");
        }
        let (host, _streams) = open_host(root.path());
        let error = host.hydrate(bad).expect_err("quarantined");
        assert!(error.to_string().contains("quarantined"), "{error}");
        assert!(matches!(
            host.catalog.get(bad).expect("entry").trust,
            crate::catalog::Trust::Corrupt(_)
        ));
        let runs = host.hydrate_runs.load(Ordering::Relaxed);
        for _ in 0..3 {
            let list = host
                .build_inventory(false, Duration::from_secs(60))
                .expect("list");
            assert_eq!(list.len(), 2, "the quarantined row stays in the List");
        }
        // only the healthy thread was hydrated by the Lists
        assert_eq!(host.hydrate_runs.load(Ordering::Relaxed), runs + 1);
        // the gate says so, with the quarantine marker the wire error is built from
        let gate = host.ensure_verified(bad).expect_err("gate refuses");
        assert!(gate.to_string().contains("quarantined"), "{gate}");
        host.shutdown();
    }

    /// O-final m3: a complete invalid line in a ledger a live holder owns is not decided on an
    /// unlocked read; the thread is quarantined only once the lock is free and the read under it
    /// shows the damage.
    #[cfg(target_os = "macos")]
    #[test]
    fn damage_in_a_held_ledger_is_deferred_until_the_lock_is_free() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000c6";
        let folder = held_thread_on_disk(root.path(), session);
        fs::OpenOptions::new()
            .append(true)
            .open(folder.join("main.jsonl"))
            .expect("ledger")
            .write_all(b"{\"not\":\"an event\"}\n")
            .expect("invalid line");
        let (host, _streams) = open_host(root.path());
        let holder = fs::OpenOptions::new()
            .read(true)
            .append(true)
            .open(folder.join("main.jsonl"))
            .expect("holder");
        hold_line_lock(&holder);
        let _ = host.hydrate(session);
        assert!(
            !matches!(
                host.catalog.get(session).map(|entry| entry.trust),
                Some(crate::catalog::Trust::Corrupt(_))
            ),
            "a line a live holder owns is not quarantined from an unlocked read"
        );
        drop(holder);
        host.shutdown();
        // The lock is free: the read under it decides. (A descriptor another test's fork holds
        // for an instant can still defer once, so a fresh host asks again.)
        let mut quarantined = false;
        for _ in 0..50 {
            let (host, _streams) = open_host(root.path());
            let outcome = host.hydrate(session);
            host.shutdown();
            if outcome.is_err_and(|error| error.to_string().contains("quarantined")) {
                quarantined = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(quarantined, "quarantined once the lock is free");
    }

    /// A placeholder row is replaced by the real row when verification finishes: the
    /// inventory stream announces it as an upsert.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_placeholder_row_is_followed_by_the_real_row_as_an_upsert() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000c7";
        held_thread_on_disk(root.path(), session);
        let (host, streams) = open_host(root.path());
        *host
            .list_wait_override
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Duration::ZERO);
        let mut inventory = streams
            .open_mux(3, endpoint::SessionStreamTarget::SessionInventory)
            .expect("inventory stream");
        let endpoint::SessionSyncFrame::InventoryBaseline { items, .. } =
            poll_until_ready(inventory.recv())
                .expect("baseline item")
                .expect("baseline frame")
        else {
            panic!("expected an inventory baseline")
        };
        assert_eq!(items.len(), 1);
        let placeholder = &items[0];
        assert_eq!(placeholder.tail, "settled");
        assert!(
            placeholder.projections.is_none(),
            "a placeholder has no title"
        );
        // verification finishes in the background and the stream upserts the real row
        let endpoint::SessionSyncFrame::InventoryUpsert { session: real, .. } =
            poll_until_ready(inventory.recv())
                .expect("upsert item")
                .expect("upsert frame")
        else {
            panic!("expected an inventory upsert")
        };
        assert_eq!(real.session_id, session);
        // the held ask_user call makes the real tail differ from the neutral placeholder
        assert_ne!(real.tail, placeholder.tail);
        // the placeholder carries the ledger's own last-record time, so the real row does not
        // move in the list when it replaces it (O-final m6)
        assert_eq!(real.updated_at, placeholder.updated_at);
        host.shutdown();
    }

    /// The client unarchive path (`ManagementStore::unarchive_session` ->
    /// `drive_folder_move`) admits the thread and brings it back into the catalog.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_client_unarchive_admits_the_thread_and_lists_it_again() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000c6";
        held_thread_on_disk(root.path(), session);
        let (host, _streams) = open_host(root.path());
        host.arm_admission();
        host.catalog.ensure_loaded();
        assert!(host.is_deferred(session));
        let management = endpoint::ManagementStore::open_light(root.path()).expect("management");
        let sha = "a".repeat(64);
        management
            .archive_session("rpc-archive-1", &sha, session, "2026-10-12T00:00:00.000Z")
            .expect("archive");
        assert!(host.catalog.get(session).is_none());
        management
            .unarchive_session("rpc-unarchive-1", &sha, session, "2026-10-12T00:00:00.000Z")
            .expect("unarchive");
        assert!(!host.is_deferred(session), "the client unarchive admits");
        assert!(
            host.catalog
                .entries_now()
                .iter()
                .any(|(id, _)| id == session)
        );
        host.shutdown();
    }

    /// E831 shutdown order: workers are stopped first, then the cards of the threads that were
    /// live are re-folded and flushed, so what a killed worker had made durable is in the card.
    #[cfg(target_os = "macos")]
    #[test]
    fn shutdown_stops_workers_before_it_flushes_the_cards_of_live_threads() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000c5";
        let (host, _streams, ledger) = held_host(root.path(), session, "");
        host.publish_appended(session).expect("first fold");
        let folder = root.path().join("threads").join(session);
        let append_last_words = || {
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(&ledger)
                .expect("append");
            file.write_all((held_response_line(session) + "\n").as_bytes())
                .expect("the killed worker's last durable append");
        };
        host.shutdown_with(
            Some(&append_last_words),
            &HashSet::from([session.to_owned()]),
        );
        let card = store::card::read_card(&folder).expect("card");
        assert_eq!(
            card.lines[0].last_seq, 10,
            "the card describes the final ledger"
        );
        assert_eq!(
            store::card::classify(&folder, &card)
                .expect("classify")
                .freshness(),
            store::card::Freshness::Fresh
        );
    }

    /// E833 F1: between card writes a fold advances the catalog entry without rebuilding the
    /// card view; a record that can change the title or the holds still rebuilds it.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_fold_between_card_writes_does_not_rebuild_the_card_view() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000c4";
        let folder = root.path().join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("folder");
        write_test_genesis(&folder.join("main.jsonl"), session);
        let (host, _streams) = open_host(root.path());
        host.publish_appended(session).expect("first fold");
        assert_eq!(host.card_builds.load(Ordering::Relaxed), 1);
        assert!(!host.card_writer.is_due(session), "the window is open");
        // a plain input: advanced in place
        append_test_input(&folder.join("main.jsonl"), session);
        host.publish_appended(session).expect("second fold");
        assert_eq!(
            host.card_builds.load(Ordering::Relaxed),
            1,
            "no view was built for the second fold"
        );
        let entry = host.catalog.get(session).expect("entry");
        let main = entry
            .view
            .as_ref()
            .and_then(|view| view.main())
            .expect("main");
        assert_eq!(main.last_seq, 2, "the entry follows the ledger");
        // the advanced row equals the scan row
        let scanned = NativeEndpoint::open(root.path())
            .expect("endpoint")
            .list_sessions(&HashSet::new())
            .expect("scan")
            .into_iter()
            .find(|item| item.session_id == session)
            .expect("row");
        let live = HashSet::new();
        let row = host.catalog_item(session, &entry, &live).expect("row");
        assert_eq!(row, scanned);
        // a record that can change the holds forces a full view
        {
            let mut ledger = fs::OpenOptions::new()
                .append(true)
                .open(folder.join("main.jsonl"))
                .expect("ledger");
            ledger
                .write_all(
                    (r#"{"kind":"meta","seq":3,"title":"A title","ts":"2026-08-28T00:00:02.000Z","v":1}"#.to_owned()
                        + "\n")
                        .as_bytes(),
                )
                .expect("title record");
        }
        host.publish_appended(session).expect("third fold");
        assert_eq!(host.card_builds.load(Ordering::Relaxed), 2);
        let entry = host.catalog.get(session).expect("entry");
        assert_eq!(
            entry.view.as_ref().expect("view").summary.title.as_deref(),
            Some("A title")
        );
        host.shutdown();
    }

    /// E833 F5: a complete invalid line in a child ledger quarantines the thread too.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_complete_invalid_line_in_a_child_ledger_quarantines_the_thread() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000c3";
        let folder = held_thread_on_disk(root.path(), session);
        fs::write(
            folder.join("018f0000-0000-7000-8000-0000000000c0.jsonl"),
            b"{\"not\":\"a record\"}\n",
        )
        .expect("child ledger");
        let (host, _streams) = open_host(root.path());
        let error = host.hydrate(session).expect_err("quarantined");
        assert!(error.to_string().contains("quarantined"), "{error}");
        assert!(matches!(
            host.catalog.get(session).expect("entry").trust,
            crate::catalog::Trust::Corrupt(_)
        ));
        host.shutdown();
    }

    /// E833 F6: the folder-event observer reads nothing (it runs under the store's locks); the
    /// card is read by the first `get`.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_published_folder_enters_the_catalog_unread() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000c2";
        let (host, _streams) = open_host(root.path());
        host.arm_admission();
        let genesis = schema::Event::decode_canonical(held_ledger_lines(session)[0].as_bytes())
            .expect("genesis");
        store::ThreadStore::open(root.path())
            .expect("store")
            .create_thread(session, genesis)
            .expect("create");
        let listed = host.catalog.entries_now();
        let entry = &listed
            .iter()
            .find(|(id, _)| id == session)
            .expect("listed")
            .1;
        assert!(entry.unread, "not read under the store lock");
        let resolved = host.catalog.get(session).expect("entry");
        assert!(!resolved.unread);
        host.shutdown();
    }

    /// The 30 s management recovery tick runs the recovering open on an idle root without error
    /// and without touching any thread (all thread folders are unreadable).
    #[cfg(target_os = "macos")]
    #[test]
    fn the_management_recovery_tick_runs_on_an_idle_root_and_reads_no_thread() {
        use std::os::unix::fs::PermissionsExt;
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000c1";
        let folder = held_thread_on_disk(root.path(), session);
        let (host, _streams) = open_host(root.path());
        // The first recovering open may reconcile workspace metadata (named exception X1).
        let mut validated = endpoint::CompleteOperations::default();
        crate::endpoint_host::recover_pending_management(root.path(), &*host, &mut validated)
            .expect("first");
        fs::set_permissions(&folder, fs::Permissions::from_mode(0o000)).expect("lock");
        let outcome =
            crate::endpoint_host::recover_pending_management(root.path(), &*host, &mut validated);
        fs::set_permissions(&folder, fs::Permissions::from_mode(0o700)).expect("unlock");
        outcome.expect("recovery tick");
        host.shutdown();
    }

    /// A thread that cannot be verified (here: its journal path is a directory) keeps its
    /// placeholder row and is not retried by every List.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_thread_that_fails_verification_is_not_retried_by_every_list() {
        let root = tempfile::tempdir().expect("root");
        let broken = "018f0000-0000-7000-8000-0000000000b8";
        let folder = held_thread_on_disk(root.path(), broken);
        fs::create_dir(folder.join("endpoint.jsonl")).expect("journal path is a directory");
        let (host, _streams) = open_host(root.path());
        let first = host
            .build_inventory(false, Duration::from_secs(30))
            .expect("list");
        assert_eq!(first.len(), 1, "the unverifiable thread keeps a row");
        let runs = host.hydrate_runs.load(Ordering::Relaxed);
        assert!(runs >= 1);
        assert_eq!(
            host.catalog.get(broken).expect("entry").trust,
            crate::catalog::Trust::Cold,
            "it could not be verified"
        );
        for _ in 0..3 {
            host.build_inventory(false, Duration::from_secs(30))
                .expect("list again");
        }
        assert_eq!(
            host.hydrate_runs.load(Ordering::Relaxed),
            runs,
            "no new attempt inside the retry window"
        );
        host.shutdown();
    }

    /// The 30 s verification backoff is for Lists only: an open, a respond or a spawn that
    /// needs the thread tries it again at once.
    #[cfg(target_os = "macos")]
    #[test]
    fn opening_a_thread_bypasses_the_list_backoff() {
        let root = tempfile::tempdir().expect("root");
        let broken = "018f0000-0000-7000-8000-0000000000b7";
        let folder = held_thread_on_disk(root.path(), broken);
        fs::create_dir(folder.join("endpoint.jsonl")).expect("journal path is a directory");
        let (host, _streams) = open_host(root.path());
        host.build_inventory(false, Duration::from_secs(30))
            .expect("list");
        let after_list = host.hydrate_runs.load(Ordering::Relaxed);
        assert!(host.recently_failed(broken));
        // fix the thread; a List still leaves it alone, an open verifies it
        fs::remove_dir(folder.join("endpoint.jsonl")).expect("repair");
        host.build_inventory(false, Duration::from_secs(30))
            .expect("list again");
        assert_eq!(host.hydrate_runs.load(Ordering::Relaxed), after_list);
        host.ensure_verified(broken)
            .expect("the open verifies it now");
        assert_eq!(host.hydrate_runs.load(Ordering::Relaxed), after_list + 1);
        assert!(!host.recently_failed(broken), "success clears the backoff");
        host.shutdown();
    }

    /// E835 N2: the List verifies the newest ledgers first, so the rows at the top of a sidebar
    /// resolve first.
    #[cfg(target_os = "macos")]
    #[test]
    fn the_list_verifies_the_newest_ledgers_first() {
        let root = tempfile::tempdir().expect("root");
        let ids = [
            "018f0000-0000-7000-8000-0000000000e1",
            "018f0000-0000-7000-8000-0000000000e2",
            "018f0000-0000-7000-8000-0000000000e3",
            "018f0000-0000-7000-8000-0000000000e4",
        ];
        // the catalog iterates ids in ascending order, so the newest ledger is the highest id:
        // only a real sort by modification time produces the expected order
        let base = std::time::SystemTime::now() - Duration::from_secs(10_000);
        for (n, id) in ids.iter().enumerate() {
            let folder = held_thread_on_disk(root.path(), id);
            let file = fs::OpenOptions::new()
                .write(true)
                .open(folder.join("main.jsonl"))
                .expect("ledger");
            file.set_modified(base + Duration::from_secs(1_000 * (n as u64 + 1)))
                .expect("mtime");
        }
        let (host, _streams) = open_host(root.path());
        host.catalog.ensure_loaded();
        let newest_first = ids
            .iter()
            .rev()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        assert_eq!(host.catalog.unverified(), newest_first, "e4 is the newest");
        host.build_inventory(false, Duration::from_secs(30))
            .expect("list");
        assert_eq!(
            *host
                .verify_order
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
            newest_first,
            "the queue is consumed newest first"
        );
        host.shutdown();
    }

    /// P-3 tripwire (in process): with every thread and archive folder unreadable (mode 000),
    /// the ready preparation -- storage preflight, host open, admission, the first periodic
    /// sweep tick's id selection -- succeeds, so nothing on the readiness path reads a thread.
    /// (The out-of-process variant with `sandbox-exec` runs against the real binary.)
    #[cfg(target_os = "macos")]
    #[test]
    fn the_ready_path_does_not_read_any_thread_or_archive_folder() {
        use std::os::unix::fs::PermissionsExt;
        let root_dir = tempfile::tempdir().expect("root");
        let root_path = fs::canonicalize(root_dir.path()).expect("canonical root");
        let mut locked = Vec::new();
        for (index, area) in ["threads", "archive"].into_iter().enumerate() {
            for n in 0..4 {
                let id = format!("018f0000-0000-7000-8000-00000000d{index}{n:02}");
                let folder = root_path.join(area).join(&id);
                fs::create_dir_all(folder.join("assets")).expect("folder");
                fs::write(
                    folder.join("main.jsonl"),
                    held_ledger_lines(&id).join("\n") + "\n",
                )
                .expect("ledger");
                locked.push(folder);
            }
        }
        struct Restore(Vec<PathBuf>);
        impl Drop for Restore {
            fn drop(&mut self) {
                for folder in &self.0 {
                    let _ = fs::set_permissions(folder, fs::Permissions::from_mode(0o700));
                }
            }
        }
        let _restore = Restore(locked.clone());
        for folder in &locked {
            fs::set_permissions(folder, fs::Permissions::from_mode(0o000)).expect("lock");
        }
        for area in ["", "threads", "archive"] {
            fs::set_permissions(root_path.join(area), fs::Permissions::from_mode(0o700))
                .expect("area mode");
        }
        crate::host_runtime::prepare_storage(&root_path.join("threads"))
            .expect("storage preflight");
        let host = ProductionProcessHost::open(
            &root_path,
            std::env::current_exe().expect("test executable"),
            "test-build",
            root_path.join("missing-agent-home"),
        )
        .expect("process host opens without reading threads");
        host.arm_admission();
        assert!(
            host.sweep_ids().is_empty(),
            "nothing admitted, nothing swept"
        );
        host.shutdown();
    }

    /// K9: a projection decoded before a newer publication cannot undo it, even when it
    /// reaches the diff after the newer one (the residual S1-a window).
    #[cfg(target_os = "macos")]
    #[test]
    fn a_stale_projection_cannot_overwrite_a_newer_published_actionable_set() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000a6";
        let (host, streams, ledger) = held_host(root.path(), session, "");
        let stale = scan_valid_prefix(&fs::read(&ledger).expect("ledger"), schema::READER_VERSION)
            .projection
            .expect("stale projection");
        assert_eq!(stale.last_seq, 9);
        {
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(&ledger)
                .expect("append");
            file.write_all((held_response_line(session) + "\n").as_bytes())
                .expect("approval_response");
        }
        host.publish_appended(session)
            .expect("publish the resolved state");
        assert!(streams.published_pending_for_test(session, None).is_empty());
        // The stale decode arrives late: it must be refused, not re-publish the hold.
        streams
            .reconcile_actionables_main(session, &stale)
            .expect("stale reconcile is a no-op");
        assert!(
            streams.published_pending_for_test(session, None).is_empty(),
            "a shorter projection must not resurrect a resolved hold"
        );
        host.shutdown();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn publish_appended_with_a_torn_tail_keeps_the_previous_error_path() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000a3";
        let (host, streams, _) = held_host(root.path(), session, r#"{"torn":"#);
        let error = host
            .publish_appended(session)
            .expect_err("torn tail is rejected by the actionables read");
        assert!(
            error.to_string().contains("tail recovery"),
            "unexpected error: {error}"
        );
        assert!(streams.published_pending_for_test(session, None).is_empty());
        host.shutdown();
    }

    fn write_test_genesis(path: &Path, thread: &str) {
        write_test_genesis_with_identity(path, thread, None);
    }

    /// Genesis with an explicit `identity_profile`; `None` writes the legacy
    /// record without the field, which reads as `coding`.
    fn write_test_genesis_with_identity(path: &Path, thread: &str, identity: Option<&str>) {
        let mut genesis = json!({
            "v":1,"seq":1,"kind":"genesis","ts":"2026-08-28T00:00:00.000Z",
            "format":1,"min_reader":1,"min_writer":1,"thread":thread,"workspace":"ws",
            "origin_key":format!("create-{thread}"),
            "origin_tuple":{"principal":"test","client":"test","target":thread,"op":"create","key":format!("create-{thread}")},
            "resume":"never","config":{"digest":"test"}
        });
        if let Some(identity) = identity {
            genesis["identity_profile"] = json!(identity);
        }
        let event = schema::Event::decode(&serde_json::to_vec(&genesis).expect("genesis JSON"))
            .expect("genesis event");
        let mut bytes = event.canonical_bytes().expect("canonical genesis");
        bytes.push(b'\n');
        fs::write(path, bytes).expect("genesis ledger");
    }

    fn append_test_input(path: &Path, thread: &str) {
        let event = schema::Event::decode(
            &serde_json::to_vec(&json!({
                "v":1,"seq":2,"kind":"input","ts":"2026-08-28T00:00:01.000Z",
                "content":[{"type":"text","text":"recover"}],
                "origin_key":"periodic-input",
                "origin_tuple":{"principal":"test","client":"test","target":thread,
                    "op":"submit","key":"periodic-input"}
            }))
            .expect("input JSON"),
        )
        .expect("input event");
        let mut bytes = event.canonical_bytes().expect("canonical input");
        bytes.push(b'\n');
        fs::OpenOptions::new()
            .append(true)
            .open(path)
            .expect("open ledger")
            .write_all(&bytes)
            .expect("append input");
    }

    #[cfg(target_os = "macos")]
    fn append_test_stop(path: &Path, thread: &str, generation: u64, seq: u64) {
        let event = schema::Event::decode(
            &serde_json::to_vec(&json!({
                "v":1,"seq":seq,"kind":"stop_requested",
                "ts":"2026-08-28T00:00:05.000Z","generation":generation,
                "origin_key":"root-stop",
                "origin_tuple":{"principal":"test","client":"test","target":thread,
                    "op":"session.cancel","key":"root-stop"}
            }))
            .expect("stop JSON"),
        )
        .expect("stop event");
        let mut bytes = event.canonical_bytes().expect("canonical stop");
        bytes.push(b'\n');
        fs::OpenOptions::new()
            .append(true)
            .open(path)
            .expect("open ledger")
            .write_all(&bytes)
            .expect("append stop");
    }

    #[cfg(target_os = "macos")]
    fn write_test_spawn_line(
        path: &Path,
        thread: &str,
        child: &str,
        parent_file: &str,
        parent: Option<(u64, &str)>,
        spawn_seq: u64,
        spawn_id: &str,
    ) {
        let mut genesis = json!({
            "v":1,"seq":1,"kind":"genesis","ts":"2026-08-28T00:00:00.000Z",
            "format":1,"min_reader":1,"min_writer":1,"thread":thread,"workspace":"ws",
            "origin_key":format!("create-{thread}"),
            "origin_tuple":{"principal":"test","client":"test","target":thread,"op":"create","key":format!("create-{thread}")},
            "resume":"never","config":{"digest":"test"}
        });
        if let Some((parent_seq, parent_spawn_id)) = parent {
            genesis["parent"] = json!({
                "file":parent_file,"seq":parent_seq,"spawn_id":parent_spawn_id
            });
            genesis["seed"] = json!({
                "source":"parent","kinds":[],
                "snapshot":{"asset":format!("sha256-{}", "0".repeat(64)),"digest":"0".repeat(64)}
            });
        }
        let mut events = vec![genesis];
        if parent.is_none() {
            events.extend([
                json!({
                    "v":1,"seq":2,"kind":"input","ts":"2026-08-28T00:00:01.000Z",
                    "content":[{"type":"text","text":"spawn"}],
                    "origin_key":"spawn-input",
                    "origin_tuple":{"principal":"test","client":"test","target":thread,"op":"submit","key":"spawn-input"}
                }),
                json!({
                    "v":1,"seq":3,"kind":"run_start","ts":"2026-08-28T00:00:02.000Z",
                    "run":"run-root","mode":"ordinary","recovery_ordinal":0,"binary":"test",
                    "config_digest":"test","instruction_digest":"test","policy":"test"
                }),
                json!({
                    "v":1,"seq":4,"kind":"turn_open","ts":"2026-08-28T00:00:03.000Z",
                    "turn":1,"trigger":{"inputs":[2]}
                }),
            ]);
        } else {
            events.push(json!({
                "v":1,"seq":2,"kind":"turn_open","ts":"2026-08-28T00:00:01.000Z",
                "turn":1,"trigger":"genesis"
            }));
        }
        events.push(json!({
            "v":1,"seq":spawn_seq,"kind":"spawn","ts":"2026-08-28T00:00:04.000Z",
            "turn":1,"child":format!("{child}.jsonl"),"call":format!("call-{spawn_id}"),
            "spawn_id":spawn_id,"resume":"never","seed":{"kinds":[]}
        }));
        write_test_events(path, events);
    }

    #[cfg(target_os = "macos")]
    fn write_test_child_genesis(
        path: &Path,
        thread: &str,
        parent_file: &str,
        parent_seq: u64,
        spawn_id: &str,
    ) {
        write_test_events(
            path,
            vec![json!({
                "v":1,"seq":1,"kind":"genesis","ts":"2026-08-28T00:00:00.000Z",
                "format":1,"min_reader":1,"min_writer":1,"thread":thread,"workspace":"ws",
                "origin_key":format!("create-{thread}"),
                "origin_tuple":{"principal":"test","client":"test","target":thread,"op":"create","key":format!("create-{thread}")},
                "resume":"never","config":{"digest":"test"},
                "parent":{"file":parent_file,"seq":parent_seq,"spawn_id":spawn_id},
                "seed":{"source":"parent","kinds":[],
                    "snapshot":{"asset":format!("sha256-{}", "0".repeat(64)),"digest":"0".repeat(64)}}
            })],
        );
    }

    #[cfg(target_os = "macos")]
    fn write_test_events(path: &Path, events: Vec<Value>) {
        let mut bytes = Vec::new();
        for value in events {
            let event = schema::Event::decode(&serde_json::to_vec(&value).expect("event JSON"))
                .expect("event shape");
            bytes.extend_from_slice(&event.canonical_bytes().expect("canonical event"));
            bytes.push(b'\n');
        }
        schema::validate_ledger(&bytes, schema::READER_VERSION).expect("test ledger validates");
        fs::write(path, bytes).expect("test ledger");
    }

    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    const CHILD_TS: &str = "2026-08-28T00:00:00.000Z";

    /// main: genesis, input, run_start, turn_open, spawn(child) at seq 5; child: a tool call
    /// (seq 6) and, when `held`, a pending approval request (seq 7).
    #[cfg(target_os = "macos")]
    fn write_thread_with_child(root: &Path, session: &str, child: &str, held: bool) -> PathBuf {
        let folder = root.join("threads").join(session);
        fs::create_dir_all(folder.join("assets")).expect("thread folder");
        write_test_spawn_line(
            &folder.join("main.jsonl"),
            session,
            child,
            "main.jsonl",
            None,
            5,
            "spawn-1",
        );
        let mut events = vec![
            json!({"v":1,"seq":1,"kind":"genesis","ts":CHILD_TS,"format":1,"min_reader":1,"min_writer":1,
                "thread":child,"workspace":"ws","origin_key":"create-child",
                "origin_tuple":{"principal":"p","client":"cli","target":child,"op":"create","key":"create-child"},
                "resume":"never","config":{"digest":"cfg"},
                "parent":{"file":"main.jsonl","seq":5,"spawn_id":"spawn-1"},
                "seed":{"source":"parent","kinds":[],
                    "snapshot":{"asset":format!("sha256-{}", "0".repeat(64)),"digest":"0".repeat(64)}}}),
            json!({"v":1,"seq":2,"kind":"turn_open","ts":CHILD_TS,"turn":1,"trigger":"genesis"}),
            json!({"v":1,"seq":3,"kind":"epoch","ts":CHILD_TS,"adapter":"fake","id":"e1","model":"fake","reason":"initial",
                "renderer":1,"system":{"asset":"sha256-aa","digest":"d1"},"tools":{"asset":"sha256-td","digest":"td"}}),
            json!({"v":1,"seq":4,"kind":"attempt","ts":CHILD_TS,"admits":[],"attempt":"a1","epoch":"e1",
                "request":{"asset":format!("sha256-{}", "ab".repeat(32)),"bytes":4096},"turn":1,"wire_digest":"wd"}),
            json!({"v":1,"seq":5,"kind":"output","ts":CHILD_TS,"attempt":"a1",
                "content":[{"args":{"cmd":"ls"},"call":"c1","name":"shell","type":"tool-call"}],
                "sealed":{"adapter":"fake","fragments":"x","version":1},"turn":1,
                "usage":{"availability":"reported","input_tokens":"5","output_tokens":"1"}}),
            json!({"v":1,"seq":6,"kind":"tool_call","ts":CHILD_TS,"args":{"cmd":"ls"},"attempt":"a1","call":"c1",
                "name":"shell","source":"provider","turn":1}),
        ];
        if held {
            events.push(child_request_event());
        }
        let path = folder.join(format!("{child}.jsonl"));
        write_test_events(&path, events);
        path
    }

    #[cfg(target_os = "macos")]
    fn child_request_event() -> Value {
        json!({"v":1,"seq":7,"kind":"approval_request","ts":CHILD_TS,"call":"c1","scope":"exec","turn":1})
    }

    #[cfg(target_os = "macos")]
    fn child_response_event(child: &str) -> Value {
        json!({"v":1,"seq":8,"kind":"approval_response","ts":CHILD_TS,"call":"c1","grant":true,"turn":1,
            "origin_key":"r1","origin_tuple":{"principal":"p","client":"cli","target":child,"op":"approve","key":"r1"}})
    }

    #[cfg(target_os = "macos")]
    fn append_test_events(path: &Path, events: Vec<Value>) {
        let mut bytes = fs::read(path).expect("ledger");
        for value in events {
            let event = schema::Event::decode(&serde_json::to_vec(&value).expect("event JSON"))
                .expect("event shape");
            bytes.extend_from_slice(&event.canonical_bytes().expect("canonical event"));
            bytes.push(b'\n');
        }
        schema::validate_ledger(&bytes, schema::READER_VERSION).expect("grown ledger validates");
        fs::write(path, bytes).expect("grown ledger");
    }

    #[cfg(target_os = "macos")]
    fn child_rpc_id(session: &str, child: &str) -> String {
        endpoint::derive_line_request_rpc_id(
            session,
            Some(&format!("{child}.jsonl")),
            endpoint::RequestFrameType::Approval,
            7,
        )
    }

    /// O-final B1, live path: a thread verified in this process folded its child lines once;
    /// a hold a child worker raises afterwards is announced through the published set, and the
    /// actionables baseline lists it without any ledger being re-read for the card. Answering
    /// it removes it again.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_live_child_hold_is_in_the_baseline_of_a_verified_thread_until_it_is_answered() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000c1";
        let child = "018f0000-0000-7000-8000-0000000000c2";
        let child_path = write_thread_with_child(root.path(), session, child, false);
        let (host, streams) = open_host(root.path());
        host.hydrate(session).expect("hydrate");
        let view = host
            .catalog
            .get(session)
            .and_then(|entry| entry.view)
            .expect("verified view");
        assert!(view.children_exact, "verification folds the child lines");
        assert_eq!(view.lines.len(), 2, "main and its child");
        assert!(streams.actionable_ids_for_test().is_empty());

        append_test_events(&child_path, vec![child_request_event()]);
        let line = format!("{child}.jsonl");
        streams
            .reconcile_actionables(session, Some(&line))
            .expect("the child worker's hold is announced");
        assert_eq!(
            streams.actionable_ids_for_test(),
            vec![child_rpc_id(session, child)]
        );

        append_test_events(&child_path, vec![child_response_event(child)]);
        streams
            .reconcile_actionables(session, Some(&line))
            .expect("the answer is announced");
        assert!(
            streams.actionable_ids_for_test().is_empty(),
            "an answered hold leaves the baseline"
        );
        host.shutdown();
    }

    /// O-final n1: a child worker that dies between a durable append and its `Appended` message
    /// leaves a request no doorbell announced; the exit path looks at its line once more.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_child_exit_announces_a_hold_no_doorbell_announced() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000d2";
        let child = "018f0000-0000-7000-8000-0000000000d3";
        let child_path = write_thread_with_child(root.path(), session, child, false);
        let (host, streams) = open_host(root.path());
        host.hydrate(session).expect("hydrate");
        append_test_events(&child_path, vec![child_request_event()]);
        assert!(
            streams.actionable_ids_for_test().is_empty(),
            "nothing announced it: the verified view is exact and no message arrived"
        );
        host.reconcile_child_line(session, &child_path);
        assert_eq!(
            streams.actionable_ids_for_test(),
            vec![child_rpc_id(session, child)]
        );
        host.shutdown();
    }

    /// O-final B1: the doorbell fold of the main ledger carries the previous child views forward
    /// and says so (`children_exact` false); the catalog then asks for the child files to be
    /// prefiltered, and a hold that arrived in the meantime is found.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_doorbell_fold_clears_child_exactness_and_the_prefilter_finds_the_new_hold() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000c3";
        let child = "018f0000-0000-7000-8000-0000000000c4";
        let child_path = write_thread_with_child(root.path(), session, child, false);
        let (host, streams) = open_host(root.path());
        host.hydrate(session).expect("hydrate");
        assert!(
            host.catalog
                .get(session)
                .unwrap()
                .view
                .unwrap()
                .children_exact
        );
        assert!(
            SupervisorSessionAuthority::hold_catalog(&*host)
                .expect("catalog")
                .children_unverified
                .is_empty()
        );

        append_test_events(&child_path, vec![child_request_event()]);
        host.publish_appended(session).expect("doorbell");
        assert!(
            !host
                .catalog
                .get(session)
                .unwrap()
                .view
                .unwrap()
                .children_exact
        );
        assert_eq!(
            SupervisorSessionAuthority::hold_catalog(&*host)
                .expect("catalog")
                .children_unverified,
            vec![session.to_owned()]
        );
        assert_eq!(
            streams.actionable_ids_for_test(),
            vec![child_rpc_id(session, child)],
            "the child file's bytes hold a request, so the thread is a candidate"
        );
        host.shutdown();
    }

    /// O-final n2: after the first main-line doorbell the catalog cannot vouch for the child
    /// lines, so every actionables recompute asks about them; an unchanged child file is not read
    /// again, and a child that grows is.
    #[cfg(target_os = "macos")]
    #[test]
    fn actionables_recomputes_do_not_reread_unchanged_child_files() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000c1";
        let child = "018f0000-0000-7000-8000-0000000000c2";
        let child_path = write_thread_with_child(root.path(), session, child, false);
        let (host, streams) = open_host(root.path());
        host.hydrate(session).expect("hydrate");
        host.publish_appended(session).expect("doorbell");
        assert_eq!(
            SupervisorSessionAuthority::hold_catalog(&*host)
                .expect("catalog")
                .children_unverified,
            vec![session.to_owned()],
            "the doorbell fold left the child lines unverified"
        );
        for _ in 0..5 {
            assert!(streams.actionable_ids_for_test().is_empty());
        }
        assert_eq!(
            streams.requests().children_scans(),
            1,
            "five recomputes read the unchanged child file once"
        );
        append_test_events(&child_path, vec![child_request_event()]);
        assert_eq!(
            streams.actionable_ids_for_test(),
            vec![child_rpc_id(session, child)],
            "a child that grew with a request is read again and found"
        );
        assert_eq!(streams.requests().children_scans(), 2);
        host.shutdown();
    }

    /// O-final m4: a scratch thread that was busy at the post-ready cleanup (a lock holder) is
    /// remembered, and the next cleanup (the recovery tick runs it while any are left) removes it
    /// once the holder is gone.
    #[cfg(target_os = "macos")]
    #[test]
    fn a_busy_stale_scratch_thread_is_removed_by_the_retry() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000c6";
        let folder = held_thread_on_disk(root.path(), session);
        let ledger = folder.join("main.jsonl");
        let text = fs::read_to_string(&ledger).expect("ledger");
        fs::write(
            &ledger,
            text.replacen(r#""format":1,"#, r#""ephemeral":true,"format":1,"#, 1),
        )
        .expect("ephemeral ledger");
        let (host, _streams) = open_host(root.path());
        let holder = store::LockedLedger::open(ledger.clone(), schema::READER_VERSION)
            .expect("a lock holder");
        host.cleanup_stale_ephemeral();
        assert_eq!(host.stale_ephemeral_busy.load(Ordering::Acquire), 1);
        assert!(folder.exists(), "a busy scratch thread is left alone");
        drop(holder);
        host.cleanup_stale_ephemeral();
        assert_eq!(host.stale_ephemeral_busy.load(Ordering::Acquire), 0);
        assert!(!folder.exists(), "the retry removed it");
        host.shutdown();
    }

    /// O-final m9: verifying a thread removes the temporary card file a crashed process left.
    #[cfg(target_os = "macos")]
    #[test]
    fn hydration_removes_a_dead_processs_card_temporary() {
        let root = tempfile::tempdir().expect("root");
        let session = "018f0000-0000-7000-8000-0000000000d1";
        let folder = held_thread_on_disk(root.path(), session);
        let mut gone = std::process::Command::new("true").spawn().expect("spawn");
        let dead = gone.id();
        gone.wait().expect("wait");
        let stale = folder.join(format!(".thread-card.json.{dead}.1.tmp"));
        fs::write(&stale, b"half a card").expect("stale temporary");
        let (host, _streams) = open_host(root.path());
        host.hydrate(session).expect("hydrate");
        assert!(!stale.exists(), "the dead process's temporary is gone");
        host.shutdown();
    }

    /// Takes the exclusive line lock for the test. A descriptor of an earlier lock can be
    /// inherited for an instant by a child another test forks, which keeps the lock busy until
    /// the child's exec, so a busy lock is retried.
    fn hold_line_lock(holder: &fs::File) {
        for _ in 0..200 {
            // SAFETY: holder owns a live descriptor and flock retains no pointers.
            if unsafe { libc::flock(holder.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
                return;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        panic!("the line lock stayed busy");
    }

    /// O-final m3: damage is decided on a read under the line's lock. A first picture of damage
    /// that the file does not have quarantines nothing, and a line a live holder owns is deferred.
    #[test]
    fn damage_is_decided_under_the_line_lock() {
        // A descriptor of this test can be inherited for an instant by a child another test
        // forks, which keeps the lock busy until the child's exec; such a Deferred is retried.
        let settled = |path: &Path| {
            for _ in 0..200 {
                let verdict = damage_under_lock(path);
                if verdict != DamageVerdict::Deferred {
                    return verdict;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            DamageVerdict::Deferred
        };
        let dir = tempfile::tempdir().expect("dir");
        let path = dir.path().join("main.jsonl");
        let good = format!(
            "{}\n",
            held_ledger_lines("018f0000-0000-7000-8000-0000000000c5")[0]
        );
        fs::write(&path, &good).expect("good ledger");
        assert!(ledger_damage(good.as_bytes()).is_none());
        assert_eq!(
            settled(&path),
            DamageVerdict::Sound,
            "a sound file under its lock is not damaged, whatever an earlier read showed"
        );
        let damaged = format!("{good}{{\"not\":\"a record\"}}\n");
        fs::write(&path, &damaged).expect("damaged ledger");
        let offset = ledger_damage(damaged.as_bytes()).expect("complete invalid record");
        assert_eq!(
            settled(&path),
            DamageVerdict::Damaged(offset),
            "real damage is read under the lock"
        );
        // A live holder owns the line: no decision, and the holder is not disturbed.
        let holder = fs::OpenOptions::new()
            .read(true)
            .append(true)
            .open(&path)
            .expect("holder");
        hold_line_lock(&holder);
        assert_eq!(damage_under_lock(&path), DamageVerdict::Deferred);
        drop(holder);
        assert_eq!(
            settled(&path),
            DamageVerdict::Damaged(offset),
            "the lock is free again, so the decision is made"
        );
        assert_eq!(
            damage_under_lock(&dir.path().join("absent.jsonl")),
            DamageVerdict::Deferred
        );
    }

    #[test]
    fn provider_admission_waits_fairly_and_drain_denies_waiter() {
        let root = tempfile::tempdir().expect("process host root");
        let host = ProductionProcessHost::open(
            root.path(),
            std::env::current_exe().expect("test executable"),
            "test-build",
            root.path().join("missing-agent-home"),
        )
        .expect("process host");
        host.max_provider_leases.store(1, Ordering::Release);
        assert!(host.request_provider_lease(AdmissionLease {
            attempt: "attempt-a".to_owned(),
            class: "provider".to_owned(),
        }));

        let (sent, received) = mpsc::channel();
        let waiter = Arc::clone(&host);
        let join = std::thread::spawn(move || {
            sent.send(waiter.request_provider_lease(AdmissionLease {
                attempt: "attempt-b".to_owned(),
                class: "provider".to_owned(),
            }))
            .expect("lease result receiver");
        });
        assert!(
            received.recv_timeout(Duration::from_millis(50)).is_err(),
            "capacity exhaustion must wait instead of replying false"
        );
        host.admission
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .settle("attempt-a");
        host.admission_changed.notify_all();
        assert!(
            received
                .recv_timeout(Duration::from_secs(1))
                .expect("queued lease result")
        );
        join.join().expect("queued lease thread");

        let (sent, received) = mpsc::channel();
        let waiter = Arc::clone(&host);
        let join = std::thread::spawn(move || {
            sent.send(waiter.request_provider_lease(AdmissionLease {
                attempt: "attempt-c".to_owned(),
                class: "provider".to_owned(),
            }))
            .expect("drain result receiver");
        });
        assert!(received.recv_timeout(Duration::from_millis(50)).is_err());
        host.draining.store(true, Ordering::Release);
        host.admission_changed.notify_all();
        assert!(
            !received
                .recv_timeout(Duration::from_secs(1))
                .expect("draining lease result")
        );
        join.join().expect("draining lease thread");
    }
}
