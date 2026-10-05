//! Durable supervisor tool-control records (intents and receipts).

use std::fs::{self, OpenOptions};
use std::io::Read;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use schema::{EventKind, IJsonValue};
use serde_json::{Map, Value};
use store::{AtomicPublisher, NamedLock, StoreError};
use thiserror::Error;
use worker_control::Selected;
use worker_control::{
    DurableControlError, ToolControl, ToolControlBinding, ToolControlError, ToolControlErrorCode,
    ToolControlResult, decode_tool_control, decode_tool_control_result, encode_tool_control_result,
};

const RECORD_FORMAT: u64 = 1;
/// Per-thread exact-retry carrier: `control/<first-two-hex>/<request_id>.json`.
pub const CONTROL_DIR: &str = "control";
/// Pre-merge layout folded into `control/` on first use.
const LEGACY_CONTROL_DIRS: [&str; 2] = ["control-receipts", "control-intents"];

pub trait ToolControlHandler {
    fn execute(&mut self, request: &ToolControl) -> ToolControlResult;

    fn recovery(&self, _request: &ToolControl) -> ToolControlRecovery {
        ToolControlRecovery::ReplaySafe
    }

    fn reconcile(&mut self, request: &ToolControl) -> ExternalEffectResolution {
        ExternalEffectResolution::Unknown {
            message: format!(
                "tool {} has no authoritative external-effect query",
                request.name
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToolControlRecovery {
    /// Repeating the handler cannot duplicate an external business effect.
    ReplaySafe,
    /// Execute only behind a durable intent and reconcile an interrupted or
    /// ambiguous attempt by the stable request-id key.
    ExternalEffect,
    /// The operation is effectful but exposes no authoritative query. It must
    /// not start in production.
    UnqueryableExternalEffect,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExternalEffectResolution {
    Confirmed(ToolControlResult),
    NotFound,
    Unknown { message: String },
    Conflicted { message: String },
}

enum ToolControlAttempt {
    Execute,
    Reconcile,
}

enum ToolControlAttemptResult {
    Executed(ToolControlResult),
    Reconciled(ExternalEffectResolution),
}

pub trait ToolControlPolicy {
    fn apply(&mut self, result: ToolControlResult) -> ToolControlResult;
}

/// Production default until a named supervisor authority is injected. It is
/// deliberately unavailable rather than a successful placeholder.
#[derive(Clone, Copy, Debug, Default)]
pub struct UnavailableToolControlHandler;

impl ToolControlHandler for UnavailableToolControlHandler {
    fn execute(&mut self, request: &ToolControl) -> ToolControlResult {
        ToolControlResult::failure(
            request.request_id.clone(),
            request.call_id.clone(),
            ToolControlError {
                code: ToolControlErrorCode::Unavailable,
                message: "supervisor tool-control authority is not configured".to_owned(),
                retryable: false,
            },
        )
    }
}

/// Fail-closed default for executable assembly. Errors contain no backend
/// value; successful values are withheld until the real secret policy exists.
#[derive(Clone, Copy, Debug, Default)]
pub struct FailClosedToolControlPolicy;

impl ToolControlPolicy for FailClosedToolControlPolicy {
    fn apply(&mut self, result: ToolControlResult) -> ToolControlResult {
        if result.value.is_none() {
            return result;
        }
        ToolControlResult::failure(
            result.request_id,
            result.call_id,
            ToolControlError {
                code: ToolControlErrorCode::Internal,
                message: "tool-control result policy is not configured".to_owned(),
                retryable: false,
            },
        )
    }
}

pub struct ToolControlSession<H, P> {
    root: PathBuf,
    selected: Selected,
    handler: H,
    policy: P,
}

impl<H: ToolControlHandler, P: ToolControlPolicy> ToolControlSession<H, P> {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>, selected: Selected, handler: H, policy: P) -> Self {
        Self {
            root: root.into(),
            selected,
            handler,
            policy,
        }
    }

    /// Decodes one worker request and returns the canonical result line. The
    /// protocol version requirement is checked before decoding or invoking
    /// any authority.
    pub fn handle_line(&mut self, line: &[u8]) -> Result<Vec<u8>, ToolControlReceiptError> {
        worker_control::require_version(&self.selected)?;
        let request = decode_tool_control(line)?;
        let thread_folder = self.root.join("threads").join(&request.session);
        let thread_metadata = fs::symlink_metadata(&thread_folder)
            .map_err(|_| ToolControlReceiptError::ThreadNotFound)?;
        if !thread_metadata.is_dir() || thread_metadata.file_type().is_symlink() {
            return Err(ToolControlReceiptError::ThreadNotFound);
        }
        let store = ToolControlReceiptStore::new(thread_folder, &request.session);
        let binding_root = store.thread_folder.clone();
        let handler = &mut self.handler;
        let policy = &mut self.policy;
        let recovery = handler.recovery(&request);
        let resolution = store.publish_or_reconcile(
            &request,
            recovery,
            |request, attempt| match resolve_durable_binding(&binding_root, request) {
                Ok(_) => match attempt {
                    ToolControlAttempt::Execute => {
                        ToolControlAttemptResult::Executed(handler.execute(request))
                    }
                    ToolControlAttempt::Reconcile => {
                        ToolControlAttemptResult::Reconciled(handler.reconcile(request))
                    }
                },
                Err(error) => ToolControlAttemptResult::Executed(ToolControlResult::failure(
                    request.request_id.clone(),
                    request.call_id.clone(),
                    ToolControlError {
                        code: ToolControlErrorCode::Conflict,
                        message: format!("durable tool-control binding failed: {error}"),
                        retryable: false,
                    },
                )),
            },
            |result| policy.apply(result),
        )?;
        Ok(resolution.response().to_vec())
    }
}

#[derive(Clone, Debug)]
pub struct ToolControlReceiptStore {
    thread_folder: PathBuf,
    bound_session: String,
}

impl ToolControlReceiptStore {
    #[must_use]
    pub fn new(thread_folder: impl Into<PathBuf>, bound_session: impl Into<String>) -> Self {
        Self {
            thread_folder: thread_folder.into(),
            bound_session: bound_session.into(),
        }
    }

    /// The one durable record for a request id: the request tuple (an intent,
    /// published before an external effect may begin) and, once settled, the
    /// exact response bytes (the receipt).
    #[must_use]
    pub fn record_path(&self, request_id: &str) -> PathBuf {
        let prefix = request_id.get(..2).unwrap_or("invalid");
        self.control_root()
            .join(prefix)
            .join(format!("{request_id}.json"))
    }

    /// Reads the durable record for a request id, if one exists.
    pub fn read_record(
        &self,
        request_id: &str,
    ) -> Result<Option<ControlRecord>, ToolControlReceiptError> {
        let path = self.record_path(request_id);
        if !path.exists() {
            return Ok(None);
        }
        read_record(&path).map(Some)
    }

    fn publish_or_reconcile<B, P>(
        &self,
        request: &ToolControl,
        recovery: ToolControlRecovery,
        mut backend: B,
        apply_policy: P,
    ) -> Result<ToolControlResolution, ToolControlReceiptError>
    where
        B: FnMut(&ToolControl, ToolControlAttempt) -> ToolControlAttemptResult,
        P: FnOnce(ToolControlResult) -> ToolControlResult,
    {
        request.validate_for_receipt_lookup()?;
        if request.session != self.bound_session {
            return Err(ToolControlReceiptError::ThreadBinding);
        }
        let _lock = self.lock()?;
        if let Some(resolution) = self.lookup_settled(request)? {
            return Ok(resolution);
        }
        request.validate()?;

        match recovery {
            ToolControlRecovery::ReplaySafe => {
                let ToolControlAttemptResult::Executed(response) =
                    backend(request, ToolControlAttempt::Execute)
                else {
                    return Err(ToolControlReceiptError::ReceiptShape);
                };
                self.publish_response(request, apply_policy(response))
            }
            ToolControlRecovery::UnqueryableExternalEffect => self.publish_response(
                request,
                ToolControlResult::failure(
                    request.request_id.clone(),
                    request.call_id.clone(),
                    ToolControlError {
                        code: ToolControlErrorCode::EffectUnknown,
                        message: "effectful tool has no idempotency-reconcile authority; manual execution is required".to_owned(),
                        retryable: false,
                    },
                ),
            ),
            ToolControlRecovery::ExternalEffect => {
                // `lookup_settled` returned nothing, so any existing record is
                // an intent whose effect may have begun.
                let interrupted = if let Some(intent) = self.read_record(&request.request_id)? {
                    if intent.request != *request {
                        return Ok(ToolControlResolution::Conflict {
                            response: encode_tool_control_result(&ToolControlResult::conflict(
                                request,
                            ))?,
                        });
                    }
                    true
                } else {
                    let intent = ControlRecord {
                        request: request.clone(),
                        response: None,
                    };
                    AtomicPublisher::replace(
                        self.record_path(&request.request_id),
                        &intent.canonical_bytes()?,
                    )?;
                    false
                };

                if interrupted {
                    let ToolControlAttemptResult::Reconciled(resolution) =
                        backend(request, ToolControlAttempt::Reconcile)
                    else {
                        return Err(ToolControlReceiptError::ReceiptShape);
                    };
                    return self.resolve_external(
                        request,
                        resolution,
                        &mut backend,
                        apply_policy,
                    );
                }

                let ToolControlAttemptResult::Executed(response) =
                    backend(request, ToolControlAttempt::Execute)
                else {
                    return Err(ToolControlReceiptError::ReceiptShape);
                };
                if response.error.as_ref().is_some_and(|error| {
                    error.code == ToolControlErrorCode::EffectUnknown
                }) {
                    let ToolControlAttemptResult::Reconciled(resolution) =
                        backend(request, ToolControlAttempt::Reconcile)
                    else {
                        return Err(ToolControlReceiptError::ReceiptShape);
                    };
                    return self.resolve_external(
                        request,
                        resolution,
                        &mut backend,
                        apply_policy,
                    );
                }
                if let Some(error) = response
                    .error
                    .as_ref()
                    .filter(|error| error.code == ToolControlErrorCode::EffectConflicted)
                {
                    return self.unresolved(
                        request,
                        ToolControlErrorCode::EffectConflicted,
                        &error.message,
                    );
                }
                self.publish_response(request, apply_policy(response))
            }
        }
    }

    fn resolve_external<B, P>(
        &self,
        request: &ToolControl,
        resolution: ExternalEffectResolution,
        backend: &mut B,
        apply_policy: P,
    ) -> Result<ToolControlResolution, ToolControlReceiptError>
    where
        B: FnMut(&ToolControl, ToolControlAttempt) -> ToolControlAttemptResult,
        P: FnOnce(ToolControlResult) -> ToolControlResult,
    {
        match resolution {
            ExternalEffectResolution::Confirmed(response) => {
                self.publish_response(request, apply_policy(response))
            }
            ExternalEffectResolution::NotFound => {
                let ToolControlAttemptResult::Executed(response) =
                    backend(request, ToolControlAttempt::Execute)
                else {
                    return Err(ToolControlReceiptError::ReceiptShape);
                };
                if let Some(error) = response.error.as_ref() {
                    match error.code {
                        ToolControlErrorCode::EffectUnknown => {
                            return self.unresolved(
                                request,
                                ToolControlErrorCode::EffectUnknown,
                                "external effect remains unknown after authoritative not_found and one retry",
                            );
                        }
                        ToolControlErrorCode::EffectConflicted => {
                            return self.unresolved(
                                request,
                                ToolControlErrorCode::EffectConflicted,
                                &error.message,
                            );
                        }
                        _ => {}
                    }
                }
                self.publish_response(request, apply_policy(response))
            }
            ExternalEffectResolution::Unknown { message } => {
                self.unresolved(request, ToolControlErrorCode::EffectUnknown, &message)
            }
            ExternalEffectResolution::Conflicted { message } => {
                self.unresolved(request, ToolControlErrorCode::EffectConflicted, &message)
            }
        }
    }

    fn unresolved(
        &self,
        request: &ToolControl,
        code: ToolControlErrorCode,
        message: &str,
    ) -> Result<ToolControlResolution, ToolControlReceiptError> {
        let response = ToolControlResult::failure(
            request.request_id.clone(),
            request.call_id.clone(),
            ToolControlError {
                code,
                message: format!(
                    "{message}; automatic execution stopped for reconciliation/manual review"
                ),
                retryable: false,
            },
        );
        Ok(ToolControlResolution::Unresolved {
            response: encode_tool_control_result(&response)?,
        })
    }

    /// Replays or conflicts on a settled record; an unsettled intent yields
    /// nothing so the caller can drive reconciliation.
    fn lookup_settled(
        &self,
        request: &ToolControl,
    ) -> Result<Option<ToolControlResolution>, ToolControlReceiptError> {
        let Some(record) = self.read_record(&request.request_id)? else {
            return Ok(None);
        };
        let Some(response) = record.response else {
            return Ok(None);
        };
        if record.request == *request {
            response.validate_for(request)?;
            return Ok(Some(ToolControlResolution::Replayed {
                response: encode_tool_control_result(&response)?,
            }));
        }
        Ok(Some(ToolControlResolution::Conflict {
            response: encode_tool_control_result(&ToolControlResult::conflict(request))?,
        }))
    }

    fn publish_response(
        &self,
        request: &ToolControl,
        response: ToolControlResult,
    ) -> Result<ToolControlResolution, ToolControlReceiptError> {
        response.validate_for(request)?;
        let receipt = ControlRecord {
            request: request.clone(),
            response: Some(response),
        };
        let receipt_path = self.record_path(&request.request_id);
        AtomicPublisher::replace(&receipt_path, &receipt.canonical_bytes()?)?;
        Ok(ToolControlResolution::Published {
            response: encode_tool_control_result(receipt.response.as_ref().expect("settled"))?,
            receipt_path,
        })
    }

    /// Returns only after a new canonical receipt has passed its publication
    /// barrier, or after an existing receipt has been validated and replayed.
    /// `apply_policy` is deliberately inside this boundary so unredacted backend
    /// material can never become receipt authority.
    pub fn publish_or_replay<B, P>(
        &self,
        request: &ToolControl,
        backend: B,
        apply_policy: P,
    ) -> Result<ToolControlResolution, ToolControlReceiptError>
    where
        B: FnOnce(&ToolControl) -> ToolControlResult,
        P: FnOnce(ToolControlResult) -> ToolControlResult,
    {
        request.validate_for_receipt_lookup()?;
        if request.session != self.bound_session {
            return Err(ToolControlReceiptError::ThreadBinding);
        }

        let _lock = self.lock()?;
        if let Some(resolution) = self.lookup_settled(request)? {
            return Ok(resolution);
        }
        let receipt_path = self.record_path(&request.request_id);

        // A novel id must equal its request tuple hash before any backend runs.
        request.validate()?;
        let response = apply_policy(backend(request));
        response.validate_for(request)?;
        let receipt = ControlRecord {
            request: request.clone(),
            response: Some(response),
        };
        let receipt_bytes = receipt.canonical_bytes()?;
        AtomicPublisher::replace(&receipt_path, &receipt_bytes)?;
        let response = encode_tool_control_result(receipt.response.as_ref().expect("settled"))?;
        Ok(ToolControlResolution::Published {
            response,
            receipt_path,
        })
    }

    fn control_root(&self) -> PathBuf {
        self.thread_folder.join(CONTROL_DIR)
    }

    /// Takes the carrier's exclusive lock and folds a pre-merge layout
    /// (`control-receipts/` and `control-intents/`) into `control/` first.
    fn lock(&self) -> Result<NamedLock, ToolControlReceiptError> {
        let root = self.control_root();
        fs::create_dir_all(&root)?;
        let lock = NamedLock::exclusive(root.join(".lock"))?;
        self.retire_legacy_layout()?;
        Ok(lock)
    }

    fn retire_legacy_layout(&self) -> Result<(), ToolControlReceiptError> {
        // Receipts first: a receipt supersedes the intent it settled, and the
        // record bytes of both legacy files are already the merged shape.
        for legacy in LEGACY_CONTROL_DIRS {
            let legacy_root = self.thread_folder.join(legacy);
            if !legacy_root.is_dir() {
                continue;
            }
            for prefix in fs::read_dir(&legacy_root)? {
                let prefix = prefix?;
                if !prefix.file_type()?.is_dir() {
                    continue;
                }
                for entry in fs::read_dir(prefix.path())? {
                    let entry = entry?;
                    let name = entry.file_name();
                    let Some(request_id) =
                        name.to_str().and_then(|name| name.strip_suffix(".json"))
                    else {
                        continue;
                    };
                    let target = self.record_path(request_id);
                    if target.exists() {
                        // A receipt already moved for this id; the intent is stale.
                        fs::remove_file(entry.path())?;
                        continue;
                    }
                    fs::create_dir_all(target.parent().expect("record parent"))?;
                    fs::rename(entry.path(), &target)?;
                }
            }
            fs::remove_dir_all(&legacy_root)?;
        }
        Ok(())
    }
}

pub(crate) struct ResolvedToolControlLine {
    pub(crate) thread_folder: PathBuf,
    pub(crate) line_path: PathBuf,
    pub(crate) projection: schema::LedgerProjection,
}

pub(crate) fn resolve_durable_binding(
    thread_folder: &Path,
    request: &ToolControl,
) -> Result<ResolvedToolControlLine, ToolControlReceiptError> {
    let (line_path, projection) = resolve_line(thread_folder, &request.thread)?;
    let mut invocation = None;
    let mut name = None;
    let mut tool_call_seq = None;
    for event in &projection.events {
        if event.turn() != Some(request.turn)
            || event.string_field("call") != Some(request.call_id.as_str())
        {
            continue;
        }
        match event.kind() {
            EventKind::ToolCall => {
                if tool_call_seq.is_some() {
                    return Err(ToolControlReceiptError::DurableBinding(
                        "tool_call is duplicated",
                    ));
                }
                let value = event_value(event)?;
                invocation = Some(materialize_json(
                    thread_folder,
                    value
                        .get("args")
                        .ok_or(ToolControlReceiptError::DurableBinding(
                            "tool_call args are missing",
                        ))?,
                )?);
                name = event.string_field("name").map(ToOwned::to_owned);
                tool_call_seq = Some(event.seq());
            }
            EventKind::EffectiveExecution if tool_call_seq.is_some_and(|seq| event.seq() > seq) => {
                let value = event_value(event)?;
                invocation = Some(materialize_json(
                    thread_folder,
                    value
                        .get("invocation")
                        .ok_or(ToolControlReceiptError::DurableBinding(
                            "effective_execution invocation is missing",
                        ))?,
                )?);
            }
            _ => {}
        }
    }
    let invocation = invocation.ok_or(ToolControlReceiptError::DurableBinding(
        "paired durable tool_call was not found",
    ))?;
    let name = name.ok_or(ToolControlReceiptError::DurableBinding(
        "paired durable tool_call name is missing",
    ))?;
    request.validate_against(&ToolControlBinding {
        session: &request.session,
        thread: &request.thread,
        turn: request.turn,
        call_id: &request.call_id,
        name: &name,
        arguments: &invocation,
    })?;

    // The resolved path is deliberately used, even though the binding is
    // entirely event-derived. This prevents a future optimizer from dropping
    // the line-identity lookup and validating a call from another child file.
    if !line_path.is_file() {
        return Err(ToolControlReceiptError::ThreadNotFound);
    }
    Ok(ResolvedToolControlLine {
        thread_folder: thread_folder.to_path_buf(),
        line_path,
        projection,
    })
}

fn resolve_line(
    thread_folder: &Path,
    line_id: &str,
) -> Result<(PathBuf, schema::LedgerProjection), ToolControlReceiptError> {
    let mut matches = Vec::new();
    for entry in fs::read_dir(thread_folder)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("jsonl")
            || path.file_name().and_then(|value| value.to_str()) == Some(endpoint::JOURNAL_FILE)
        {
            continue;
        }
        let bytes = fs::read(&path)?;
        let scan = store::scan_valid_prefix(&bytes, 1);
        let Some(projection) = scan.projection else {
            continue;
        };
        if projection.events.first().is_some_and(|genesis| {
            matches!(genesis.kind(), EventKind::Genesis)
                && genesis.string_field("thread") == Some(line_id)
        }) {
            matches.push((path, projection));
        }
    }
    match matches.len() {
        1 => Ok(matches.pop().expect("one line match")),
        0 => Err(ToolControlReceiptError::LineNotFound),
        _ => Err(ToolControlReceiptError::DurableBinding(
            "line identity is duplicated in the session folder",
        )),
    }
}

fn event_value(event: &schema::Event) -> Result<Value, ToolControlReceiptError> {
    serde_json::to_value(event.raw()).map_err(ToolControlReceiptError::Json)
}

fn materialize_json(
    thread_folder: &Path,
    value: &Value,
) -> Result<IJsonValue, ToolControlReceiptError> {
    if let Some(spill) = value.get("$spill").and_then(Value::as_object) {
        if value.as_object().is_none_or(|object| object.len() != 1) {
            return Err(ToolControlReceiptError::DurableBinding(
                "spill discriminant has sibling fields",
            ));
        }
        let asset = spill.get("asset").and_then(Value::as_str).ok_or(
            ToolControlReceiptError::DurableBinding("spill asset is missing"),
        )?;
        let expected = spill.get("bytes").and_then(Value::as_u64).ok_or(
            ToolControlReceiptError::DurableBinding("spill byte length is missing"),
        )?;
        let bytes = store::AssetStore::new(thread_folder.join("assets"))?.read_verified(asset)?;
        if u64::try_from(bytes.len()).ok() != Some(expected) {
            return Err(ToolControlReceiptError::DurableBinding(
                "spill byte length does not match the asset",
            ));
        }
        return Ok(IJsonValue::parse(&bytes)?);
    }
    Ok(IJsonValue::parse(&serde_json::to_vec(value)?)?)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ToolControlResolution {
    Published {
        response: Vec<u8>,
        receipt_path: PathBuf,
    },
    Replayed {
        response: Vec<u8>,
    },
    Conflict {
        response: Vec<u8>,
    },
    /// No receipt is published. A later identical recovery re-runs only the
    /// authoritative query; it never blindly repeats the business operation.
    Unresolved {
        response: Vec<u8>,
    },
}

impl ToolControlResolution {
    #[must_use]
    pub fn response(&self) -> &[u8] {
        match self {
            Self::Published { response, .. }
            | Self::Replayed { response }
            | Self::Conflict { response }
            | Self::Unresolved { response } => response,
        }
    }

    #[must_use]
    pub const fn is_replay(&self) -> bool {
        matches!(self, Self::Replayed { .. })
    }
}

/// One durable tool-control record: the request tuple, published as an
/// intent before an external effect may begin, and the exact response once
/// the request settled (the receipt).
#[derive(Clone, Debug, PartialEq)]
pub struct ControlRecord {
    pub request: ToolControl,
    pub response: Option<ToolControlResult>,
}

impl ControlRecord {
    fn canonical_bytes(&self) -> Result<Vec<u8>, ToolControlReceiptError> {
        let mut value = serde_json::json!({
            "format": RECORD_FORMAT,
            "request": self.request,
        });
        if let Some(response) = &self.response {
            let mut envelope = Map::new();
            envelope.insert(
                "tool_control_result".to_owned(),
                serde_json::to_value(response)?,
            );
            value["response"] = Value::Object(envelope);
        }
        Ok(serde_json_canonicalizer::to_vec(&value)?)
    }
}

fn read_record(path: &Path) -> Result<ControlRecord, ToolControlReceiptError> {
    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)?;
    if !file.metadata()?.file_type().is_file() {
        return Err(ToolControlReceiptError::ReceiptShape);
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let value: Value = serde_json::from_slice(&bytes)?;
    if serde_json_canonicalizer::to_vec(&value)? != bytes {
        return Err(ToolControlReceiptError::NonCanonicalReceipt);
    }
    let object = value
        .as_object()
        .ok_or(ToolControlReceiptError::ReceiptShape)?;
    let settled = match object.len() {
        2 => false,
        3 if object.contains_key("response") => true,
        _ => return Err(ToolControlReceiptError::ReceiptShape),
    };
    if object.get("format").and_then(Value::as_u64) != Some(RECORD_FORMAT) {
        return Err(ToolControlReceiptError::ReceiptShape);
    }
    let request: ToolControl = serde_json::from_value(
        object
            .get("request")
            .cloned()
            .ok_or(ToolControlReceiptError::ReceiptShape)?,
    )?;
    request.validate()?;
    let response = if settled {
        let response_bytes = {
            let mut bytes = serde_json_canonicalizer::to_vec(&object["response"])?;
            bytes.push(b'\n');
            bytes
        };
        Some(decode_tool_control_result(&response_bytes)?)
    } else {
        None
    };
    Ok(ControlRecord { request, response })
}

#[derive(Debug, Error)]
pub enum ToolControlReceiptError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("store error: {0}")]
    Store(#[from] StoreError),
    #[error("worker-control durable control error: {0}")]
    Protocol(#[from] DurableControlError),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("control receipt is not canonical JSON")]
    NonCanonicalReceipt,
    #[error("control receipt has an invalid closed shape")]
    ReceiptShape,
    #[error("tool-control request does not match the receipt store's thread binding")]
    ThreadBinding,
    #[error("tool-control request targets an unknown thread")]
    ThreadNotFound,
    #[error("tool-control request targets an unknown line in its session folder")]
    LineNotFound,
    #[error("tool-control durable binding failed: {0}")]
    DurableBinding(&'static str),
    #[error("tool-control schema error: {0}")]
    Schema(#[from] schema::SchemaError),
}
