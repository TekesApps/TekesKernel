//! Deployment-v1 operational records, metrics and redacted support bundles.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use store::FullSync;
use thiserror::Error;
use transport::{AccessLogRecord, AccessLogSink};

pub const LOG_ROTATION_BYTES: u64 = 10 * 1024 * 1024;
pub const LOG_RETAINED_GENERATIONS: usize = 5;

pub const METRIC_NAMES: [&str; 17] = [
    "readiness",
    "active_workers",
    "threads_parked",
    "threads_running",
    "threads_settled",
    "provider_leases",
    "append_latency_ms",
    "barrier_latency_ms",
    "tail_repairs_total",
    "corruptions_total",
    "provider_outcomes_total",
    "tool_outcomes_total",
    "endpoint_requests_total",
    "endpoint_stream_pressure",
    "sweep_duration_ms",
    "restarts_total",
    "rollbacks_total",
];

const FORBIDDEN_SUPPORT_BYTES: [&str; 5] = [
    "secret-provider-key",
    "secret-prompt",
    "secret-tool-output",
    "Authorization: Bearer",
    "endpoint-token-canary",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenAttribution {
    pub build: String,
    pub launch_id: String,
    pub attempt: u64,
    pub generation: u64,
    pub manifest_sha256: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Correlation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_seq: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

impl From<&FrozenAttribution> for Correlation {
    fn from(value: &FrozenAttribution) -> Self {
        Self {
            launch_id: Some(value.launch_id.clone()),
            attempt: Some(value.attempt),
            generation: Some(value.generation),
            manifest_sha256: Some(value.manifest_sha256.clone()),
            ..Self::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogRecord {
    pub v: u8,
    pub ts: String,
    pub severity: Severity,
    pub component: String,
    pub build: String,
    pub code: String,
    pub message: String,
    pub correlation: Correlation,
    pub fields: BTreeMap<String, Value>,
}

impl LogRecord {
    pub fn canonical_line(&self) -> Result<Vec<u8>, ObservabilityError> {
        self.validate()?;
        let mut bytes = serde_json_canonicalizer::to_vec(self)
            .map_err(|error| ObservabilityError::Json(error.to_string()))?;
        bytes.push(b'\n');
        reject_forbidden(&bytes)?;
        Ok(bytes)
    }

    pub fn validate(&self) -> Result<(), ObservabilityError> {
        if self.v != 1 || !COMPONENTS.contains(&self.component.as_str()) {
            return Err(ObservabilityError::InvalidLog("invalid version/component"));
        }
        if self.build.is_empty() || self.code.is_empty() || self.message.is_empty() {
            return Err(ObservabilityError::InvalidLog("empty required string"));
        }
        if !self.fields.values().all(is_safe_scalar) {
            return Err(ObservabilityError::InvalidLog(
                "fields must contain safe scalar values only",
            ));
        }
        let allowed = field_keys(self.component.as_str(), self.code.as_str())
            .ok_or(ObservabilityError::InvalidLog("unknown operational code"))?;
        let actual_fields = self
            .fields
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let expected_fields = allowed.iter().copied().collect::<BTreeSet<_>>();
        if actual_fields != expected_fields {
            return Err(ObservabilityError::InvalidLog(
                "field keys do not match the closed code registry",
            ));
        }
        if !valid_correlation(self) {
            return Err(ObservabilityError::InvalidLog(
                "record correlation does not match the closed code registry",
            ));
        }
        if let Some(expected) = selector_severity(self.component.as_str(), self.code.as_str()) {
            if self.severity != expected {
                return Err(ObservabilityError::InvalidLog(
                    "record severity does not match the closed code registry",
                ));
            }
        }
        Ok(())
    }
}

const COMPONENTS: [&str; 6] = [
    "installer",
    "supervisor",
    "worker",
    "helper",
    "selector",
    "transport",
];

fn is_safe_scalar(value: &Value) -> bool {
    match value {
        Value::String(_) | Value::Bool(_) => true,
        Value::Number(number) => number
            .as_u64()
            .is_some_and(|value| value <= 9_007_199_254_740_991),
        _ => false,
    }
}

fn valid_correlation(record: &LogRecord) -> bool {
    if record.component == "selector" {
        return match record.code.as_str() {
            "selector-predecessor-draining"
            | "orphan-owner-timeout"
            | "selector-prelaunch-cleared"
            | "selector-update-complete" => record.correlation == Correlation::default(),
            "selector-recovered" => {
                record.correlation.attempt.is_none()
                    && record.correlation.generation.is_none()
                    && record.correlation.launch_id.is_none()
                    && record.correlation.manifest_sha256.is_none()
                    && record
                        .correlation
                        .operation_id
                        .as_deref()
                        .is_some_and(|value| !value.is_empty())
            }
            "rollback-complete" => {
                complete_child_correlation(&record.correlation)
                    && record
                        .correlation
                        .operation_id
                        .as_deref()
                        .is_some_and(|value| !value.is_empty())
            }
            "selector-child-launch" | "promotion-complete" => {
                complete_child_correlation(&record.correlation)
                    && record.correlation.operation_id.is_none()
            }
            code if selector_failure(code) => {
                complete_child_correlation(&record.correlation)
                    && record.correlation.operation_id.is_none()
            }
            _ => true,
        };
    }
    if matches!(
        record.code.as_str(),
        "installer-recovered" | "installer-recovery-required" | "invalid-bundle"
    ) {
        true
    } else {
        complete_child_correlation(&record.correlation)
    }
}

fn complete_child_correlation(correlation: &Correlation) -> bool {
    let Some(attempt) = correlation.attempt.filter(|value| *value >= 1) else {
        return false;
    };
    let Some(generation) = correlation.generation.filter(|value| *value >= 1) else {
        return false;
    };
    correlation
        .launch_id
        .as_deref()
        .is_some_and(|value| valid_launch_id(value, generation, attempt))
        && correlation
            .manifest_sha256
            .as_deref()
            .is_some_and(|value| validate_hex(value).is_ok())
}

fn valid_launch_id(value: &str, generation: u64, attempt: u64) -> bool {
    let mut parts = value.split('-');
    parts.next().and_then(|part| part.parse().ok()) == Some(generation)
        && parts.next().and_then(|part| part.parse().ok()) == Some(attempt)
        && parts.next().is_some_and(|random| {
            random.len() == 32
                && random
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        })
        && parts.next().is_none()
}

fn selector_failure(code: &str) -> bool {
    matches!(
        code,
        "canary-attribution-failed"
            | "canary-timeout"
            | "invalid-install"
            | "protocol-mismatch"
            | "readiness-timeout"
            | "selector-mismatch"
            | "unexpected-child-exit"
            | "already-running"
            | "corrupt-ledger"
            | "endpoint-credential-unavailable"
            | "invalid-config"
            | "io"
            | "listener-unavailable"
            | "required-broker-unavailable"
            | "unsupported-filesystem"
            | "launcher-interrupted"
            | "readiness-crash-loop"
    )
}

fn selector_severity(component: &str, code: &str) -> Option<Severity> {
    (component == "selector").then_some(match code {
        "selector-predecessor-draining" | "rollback-complete" => Severity::Warn,
        "orphan-owner-timeout" => Severity::Error,
        "selector-prelaunch-cleared"
        | "selector-recovered"
        | "selector-update-complete"
        | "selector-child-launch"
        | "promotion-complete" => Severity::Info,
        "already-running"
        | "corrupt-ledger"
        | "endpoint-credential-unavailable"
        | "invalid-config"
        | "io"
        | "listener-unavailable"
        | "required-broker-unavailable"
        | "unsupported-filesystem"
        | "launcher-interrupted" => Severity::Warn,
        _ => Severity::Error,
    })
}

fn field_keys(component: &str, code: &str) -> Option<&'static [&'static str]> {
    if component == "selector" {
        return Some(match code {
            "selector-predecessor-draining" | "selector-prelaunch-cleared" => &["root_lock"],
            "selector-recovered" => &["operation", "phase"],
            "selector-update-complete" => &["sha256", "version"],
            "orphan-owner-timeout" => &[
                "deadline_ms",
                "generation",
                "manifest_sha256",
                "root_lock",
                "version",
            ],
            "selector-child-launch" => &["canary_required"],
            "promotion-complete" => &["from_state"],
            "rollback-complete" => &["from", "reason", "to"],
            failure if selector_failure(failure) => &["classification"],
            _ => return None,
        });
    }
    Some(match code {
        "already-running" => &["storage_root"],
        "boot-recovery-complete" => &["generation", "version"],
        "corrupt-ledger" => &["file", "offset"],
        "endpoint-credential-unavailable" => &["keychain_status"],
        "installer-recovered" | "installer-recovery-required" => &["operation", "phase"],
        "invalid-bundle" | "invalid-install" => &["path"],
        "invalid-config" => &["file"],
        "invalid-request" => &["method"],
        "io" => &["operation"],
        "launch-failed" | "launcher-interrupted" => &["version"],
        "listener-unavailable" => &["address"],
        "protocol-mismatch" => &["channel"],
        "provider-terminal" | "tool-error" => &["classification"],
        "readiness-crash-loop" => &["last_code", "version"],
        "required-broker-unavailable" => &["broker"],
        "rollback-complete" => &["from", "reason", "to"],
        "selector-mismatch" => &["actual", "expected"],
        "server-draining" => &["deadline_seconds"],
        "unsupported-filesystem" => &["filesystem"],
        _ => return None,
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metric {
    pub name: String,
    #[serde(rename = "type")]
    pub metric_type: String,
    pub value: f64,
    pub labels: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetricSnapshot {
    pub v: u8,
    pub ts: String,
    pub metrics: Vec<Metric>,
}

impl MetricSnapshot {
    pub fn validate(&self) -> Result<(), ObservabilityError> {
        if self.v != 1 {
            return Err(ObservabilityError::InvalidMetric("invalid metric version"));
        }
        let names = METRIC_NAMES.into_iter().collect::<BTreeSet<_>>();
        let actual = self
            .metrics
            .iter()
            .map(|metric| metric.name.as_str())
            .collect::<Vec<_>>();
        let expected = names.iter().copied().collect::<Vec<_>>();
        if actual != expected {
            return Err(ObservabilityError::InvalidMetric(
                "metrics must be the complete registry in lexical order",
            ));
        }
        for metric in &self.metrics {
            if !names.contains(metric.name.as_str())
                || metric.metric_type != metric_type(&metric.name)
                || !metric.value.is_finite()
                || metric.value < 0.0
                || metric
                    .labels
                    .keys()
                    .any(|key| !matches!(key.as_str(), "component" | "classification"))
            {
                return Err(ObservabilityError::InvalidMetric(
                    "metric violates the closed schema",
                ));
            }
        }
        Ok(())
    }

    pub fn canonical_line(&self) -> Result<Vec<u8>, ObservabilityError> {
        self.validate()?;
        let mut bytes = serde_json_canonicalizer::to_vec(self)
            .map_err(|error| ObservabilityError::Json(error.to_string()))?;
        bytes.push(b'\n');
        Ok(bytes)
    }
}

pub fn publish_metric_snapshot(
    path: &Path,
    snapshot: &MetricSnapshot,
) -> Result<(), ObservabilityError> {
    let bytes = snapshot.canonical_line()?;
    reject_forbidden(&bytes)?;
    let parent = path
        .parent()
        .ok_or(ObservabilityError::DestinationWithoutParent)?;
    let name = path
        .file_name()
        .ok_or(ObservabilityError::DestinationWithoutParent)?
        .to_string_lossy();
    let staging = parent.join(format!(".{name}.tmp-{}", std::process::id()));
    if staging.exists() {
        return Err(ObservabilityError::DestinationExists);
    }
    write_private(&staging, &bytes)?;
    fs::rename(&staging, path)?;
    sync_parent(path)?;
    Ok(())
}

/// Thread-safe projection used by the transport carrier. Request content and
/// Authorization bytes are never accepted by this boundary.
pub struct OperationalMetrics {
    values: Mutex<BTreeMap<&'static str, f64>>,
}

impl Default for OperationalMetrics {
    fn default() -> Self {
        Self {
            values: Mutex::new(METRIC_NAMES.into_iter().map(|name| (name, 0.0)).collect()),
        }
    }
}

impl OperationalMetrics {
    pub fn snapshot(&self, ts: impl Into<String>, ready: bool) -> MetricSnapshot {
        let values = self
            .values
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        let labels = BTreeMap::from([("component".to_owned(), "supervisor".to_owned())]);
        let mut metrics = METRIC_NAMES
            .into_iter()
            .map(|name| Metric {
                name: name.to_owned(),
                metric_type: metric_type(name).to_owned(),
                value: if name == "readiness" {
                    if ready { 1.0 } else { 0.0 }
                } else {
                    values.get(name).copied().unwrap_or(0.0)
                },
                labels: labels.clone(),
            })
            .collect::<Vec<_>>();
        metrics.sort_by(|left, right| left.name.cmp(&right.name));
        MetricSnapshot {
            v: 1,
            ts: ts.into(),
            metrics,
        }
    }

    pub fn set(&self, name: &'static str, value: f64) {
        if METRIC_NAMES.contains(&name) && value.is_finite() && value >= 0.0 {
            self.values
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .insert(name, value);
        }
    }

    pub fn increment(&self, name: &'static str) {
        if METRIC_NAMES.contains(&name) {
            let mut values = self
                .values
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let value = values.entry(name).or_insert(0.0);
            *value += 1.0;
        }
    }
}

impl AccessLogSink for OperationalMetrics {
    fn record(&self, _record: &AccessLogRecord) {
        self.increment("endpoint_requests_total");
    }
}

/// Production transport sink: bounded metrics plus closed, redacted failure
/// records carrying the immutable selector attribution. It deliberately never
/// logs request bodies, headers, paths containing user data, or host messages.
pub struct ProductionAccessLog {
    metrics: Arc<OperationalMetrics>,
    log: Arc<RotatingJsonlLog>,
    attribution: FrozenAttribution,
    faulted: AtomicBool,
    health_hook: Mutex<Option<HealthHook>>,
}

type HealthHook = Arc<dyn Fn(bool) + Send + Sync>;

impl ProductionAccessLog {
    #[must_use]
    pub fn new(
        metrics: Arc<OperationalMetrics>,
        log: Arc<RotatingJsonlLog>,
        attribution: FrozenAttribution,
    ) -> Self {
        Self {
            metrics,
            log,
            attribution,
            faulted: AtomicBool::new(false),
            health_hook: Mutex::new(None),
        }
    }

    /// Installs the production health hook. A failed operational append is a
    /// fail-stop observability condition: the bound endpoint immediately
    /// becomes not-ready and a closed fallback record is written to stderr.
    pub fn install_health_hook(&self, hook: HealthHook) {
        *self
            .health_hook
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(hook);
        if self.faulted.load(Ordering::Acquire) {
            self.invoke_health_hook(false);
        }
    }

    #[must_use]
    pub fn is_faulted(&self) -> bool {
        self.faulted.load(Ordering::Acquire)
    }

    fn append(&self, record: &LogRecord) {
        if self.log.append(record).is_ok() {
            if self.faulted.swap(false, Ordering::AcqRel) {
                self.invoke_health_hook(true);
            }
            return;
        }
        let first = !self.faulted.swap(true, Ordering::AcqRel);
        self.invoke_health_hook(false);
        if first {
            // This deliberately contains no dynamic error or request data.
            // It remains machine-readable when the canonical log path itself
            // is unavailable or unsafe.
            eprintln!(
                "{{\"code\":\"operational-log-unavailable\",\"format\":1,\"message\":\"Operational log append failed\"}}"
            );
        }
    }

    fn invoke_health_hook(&self, healthy: bool) {
        let hook = self
            .health_hook
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        if let Some(hook) = hook {
            hook(healthy);
        }
    }

    pub fn record_semantic_failure(
        &self,
        code: &'static str,
        message: &'static str,
        classification: &str,
        session_id: &str,
        event_seq: u64,
    ) {
        if !matches!(code, "provider-terminal" | "tool-error") {
            return;
        }
        let mut correlation = Correlation::from(&self.attribution);
        correlation.session_id = Some(session_id.to_owned());
        correlation.event_seq = Some(event_seq);
        let record = LogRecord {
            v: 1,
            ts: observability_timestamp(),
            severity: Severity::Error,
            component: "supervisor".to_owned(),
            build: self.attribution.build.clone(),
            code: code.to_owned(),
            message: message.to_owned(),
            correlation,
            fields: [(
                "classification".to_owned(),
                Value::String(classification.to_owned()),
            )]
            .into_iter()
            .collect(),
        };
        self.append(&record);
    }

    pub fn record_corruption(&self, session_id: &str, file: &str, offset: u64) {
        let mut correlation = Correlation::from(&self.attribution);
        correlation.session_id = Some(session_id.to_owned());
        let record = LogRecord {
            v: 1,
            ts: observability_timestamp(),
            severity: Severity::Error,
            component: "supervisor".to_owned(),
            build: self.attribution.build.clone(),
            code: "corrupt-ledger".to_owned(),
            message: "Ledger validation failed".to_owned(),
            correlation,
            fields: [
                ("file".to_owned(), Value::String(file.to_owned())),
                ("offset".to_owned(), Value::from(offset)),
            ]
            .into_iter()
            .collect(),
        };
        self.append(&record);
    }

    pub fn record_owner_io_failure(&self, operation: &'static str, session_id: &str) {
        let mut correlation = Correlation::from(&self.attribution);
        correlation.session_id = Some(session_id.to_owned());
        let record = LogRecord {
            v: 1,
            ts: observability_timestamp(),
            severity: Severity::Error,
            component: "supervisor".to_owned(),
            build: self.attribution.build.clone(),
            code: "io".to_owned(),
            message: "Owner operation failed".to_owned(),
            correlation,
            fields: [("operation".to_owned(), Value::String(operation.to_owned()))]
                .into_iter()
                .collect(),
        };
        self.append(&record);
    }
}

impl AccessLogSink for ProductionAccessLog {
    fn record(&self, record: &AccessLogRecord) {
        self.metrics.increment("endpoint_requests_total");
        self.metrics.set(
            "endpoint_stream_pressure",
            if record.status == 429 { 1.0 } else { 0.0 },
        );
        if record.error_code.as_deref() != Some("invalid-request") {
            return;
        }
        let mut correlation = Correlation::from(&self.attribution);
        correlation.request_id = Some(record.request_id.clone());
        let operational = LogRecord {
            v: 1,
            ts: observability_timestamp(),
            severity: Severity::Warn,
            component: "transport".to_owned(),
            build: self.attribution.build.clone(),
            code: "invalid-request".to_owned(),
            message: "Endpoint request was rejected".to_owned(),
            correlation,
            fields: [("method".to_owned(), Value::String(record.operation.clone()))]
                .into_iter()
                .collect(),
        };
        self.append(&operational);
    }
}

fn observability_timestamp() -> String {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let seconds = duration.as_secs();
    let days = i64::try_from(seconds / 86_400).unwrap_or_default() + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    let day_seconds = seconds % 86_400;
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:09}Z",
        day_seconds / 3_600,
        day_seconds % 3_600 / 60,
        day_seconds % 60,
        duration.subsec_nanos(),
    )
}

fn metric_type(name: &str) -> &'static str {
    match name {
        "readiness"
        | "active_workers"
        | "threads_parked"
        | "threads_running"
        | "threads_settled"
        | "provider_leases"
        | "endpoint_stream_pressure" => "gauge",
        "append_latency_ms" | "barrier_latency_ms" | "sweep_duration_ms" => "histogram",
        _ => "counter",
    }
}

pub struct RotatingJsonlLog {
    path: PathBuf,
    rotation_bytes: u64,
    append_lock: Mutex<()>,
}

impl RotatingJsonlLog {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            rotation_bytes: LOG_ROTATION_BYTES,
            append_lock: Mutex::new(()),
        }
    }

    pub fn append(&self, record: &LogRecord) -> Result<(), ObservabilityError> {
        let bytes = record.canonical_line()?;
        let _guard = self
            .append_lock
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let existing = open_existing_secure(&self.path, true)?;
        if existing.as_ref().is_some_and(|(file, _)| {
            file.metadata().is_ok_and(|metadata| {
                metadata.len().saturating_add(bytes.len() as u64) > self.rotation_bytes
            })
        }) {
            drop(existing);
            self.rotate()?;
        }
        let mut file = open_active_secure(&self.path)?;
        file.write_all(&bytes)?;
        FullSync::full_sync(&file)?;
        Ok(())
    }

    fn rotate(&self) -> io::Result<()> {
        for generation in (1..LOG_RETAINED_GENERATIONS).rev() {
            let from = rotated_path(&self.path, generation);
            let to = rotated_path(&self.path, generation + 1);
            if let Some((file, identity)) = open_existing_secure(&from, false)? {
                drop(file);
                validate_destination_if_present(&to)?;
                fs::rename(&from, &to)?;
                validate_path_identity(&to, identity)?;
            }
        }
        if let Some((file, identity)) = open_existing_secure(&self.path, true)? {
            drop(file);
            let to = rotated_path(&self.path, 1);
            validate_destination_if_present(&to)?;
            fs::rename(&self.path, rotated_path(&self.path, 1))?;
            validate_path_identity(&to, identity)?;
        }
        sync_parent(&self.path)
    }

    #[cfg(test)]
    fn with_rotation_bytes(path: impl Into<PathBuf>, rotation_bytes: u64) -> Self {
        Self {
            path: path.into(),
            rotation_bytes,
            append_lock: Mutex::new(()),
        }
    }
}

#[derive(Clone, Copy)]
struct FileIdentity {
    device: u64,
    inode: u64,
}

fn effective_uid() -> u32 {
    // SAFETY: geteuid has no preconditions and retains no memory.
    unsafe { libc::geteuid() }
}

fn open_active_secure(path: &Path) -> io::Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .mode(0o600)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)?;
    validate_open_file(path, &file)?;
    Ok(file)
}

fn open_existing_secure(path: &Path, append: bool) -> io::Result<Option<(File, FileIdentity)>> {
    let mut options = OpenOptions::new();
    options
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW);
    if append {
        options.append(true);
    }
    let file = match options.open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    let identity = validate_open_file(path, &file)?;
    Ok(Some((file, identity)))
}

fn validate_open_file(path: &Path, file: &File) -> io::Result<FileIdentity> {
    let descriptor = file.metadata()?;
    let path_metadata = fs::symlink_metadata(path)?;
    if !descriptor.file_type().is_file()
        || !path_metadata.file_type().is_file()
        || descriptor.uid() != effective_uid()
        || path_metadata.uid() != effective_uid()
        || descriptor.mode() & 0o777 != 0o600
        || path_metadata.mode() & 0o777 != 0o600
        || descriptor.dev() != path_metadata.dev()
        || descriptor.ino() != path_metadata.ino()
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "operational log is not one owner-only regular inode",
        ));
    }
    Ok(FileIdentity {
        device: descriptor.dev(),
        inode: descriptor.ino(),
    })
}

fn validate_path_identity(path: &Path, expected: FileIdentity) -> io::Result<()> {
    let (file, actual) = open_existing_secure(path, false)?.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "rotated operational log disappeared",
        )
    })?;
    drop(file);
    if actual.device != expected.device || actual.inode != expected.inode {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "operational log inode changed during rotation",
        ));
    }
    Ok(())
}

fn validate_destination_if_present(path: &Path) -> io::Result<()> {
    if let Some((file, _)) = open_existing_secure(path, false)? {
        drop(file);
    }
    Ok(())
}

fn rotated_path(path: &Path, generation: usize) -> PathBuf {
    let mut name = path.as_os_str().to_os_string();
    name.push(format!(".{generation}"));
    PathBuf::from(name)
}

#[derive(Serialize)]
struct SupportManifest<'a> {
    format: u8,
    created_at: &'a str,
    build: &'a str,
    capability_digest: &'a str,
    config_digests: &'a [String],
    files: Vec<SupportFile>,
}

#[derive(Serialize)]
struct SupportFile {
    path: &'static str,
    bytes: u64,
    sha256: String,
}

pub fn publish_support_bundle(
    destination: &Path,
    created_at: &str,
    build: &str,
    capability_digest: &str,
    config_digests: &[String],
    logs: &[LogRecord],
    metrics: &MetricSnapshot,
) -> Result<(), ObservabilityError> {
    validate_hex(capability_digest)?;
    if config_digests
        .iter()
        .any(|digest| validate_hex(digest).is_err())
    {
        return Err(ObservabilityError::InvalidDigest);
    }
    if destination.exists() {
        return Err(ObservabilityError::DestinationExists);
    }
    let parent = destination
        .parent()
        .ok_or(ObservabilityError::DestinationWithoutParent)?;
    let name = destination
        .file_name()
        .ok_or(ObservabilityError::DestinationWithoutParent)?
        .to_string_lossy();
    let staging = parent.join(format!(".{name}.tmp-{}", std::process::id()));
    if staging.exists() {
        return Err(ObservabilityError::DestinationExists);
    }
    fs::create_dir(&staging)?;
    fs::set_permissions(&staging, fs::Permissions::from_mode(0o700))?;

    let result = (|| {
        let mut log_bytes = Vec::new();
        for record in logs {
            log_bytes.extend(record.canonical_line()?);
        }
        let metric_bytes = metrics.canonical_line()?;
        reject_forbidden(&log_bytes)?;
        reject_forbidden(&metric_bytes)?;
        write_private(&staging.join("logs.jsonl"), &log_bytes)?;
        write_private(&staging.join("metrics.canonical.json"), &metric_bytes)?;
        let files = vec![
            SupportFile {
                path: "logs.jsonl",
                bytes: log_bytes.len() as u64,
                sha256: digest(&log_bytes),
            },
            SupportFile {
                path: "metrics.canonical.json",
                bytes: metric_bytes.len() as u64,
                sha256: digest(&metric_bytes),
            },
        ];
        let mut manifest = serde_json_canonicalizer::to_vec(&SupportManifest {
            format: 1,
            created_at,
            build,
            capability_digest,
            config_digests,
            files,
        })
        .map_err(|error| ObservabilityError::Json(error.to_string()))?;
        manifest.push(b'\n');
        reject_forbidden(&manifest)?;
        write_private(&staging.join("manifest.canonical.json"), &manifest)?;
        FullSync::full_sync(&File::open(&staging)?)?;
        fs::rename(&staging, destination)?;
        sync_parent(destination)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_dir_all(&staging);
    }
    result
}

/// Publishes a support bundle from the production files maintained by the
/// selector and supervisor. This is deliberately an offline operation: the
/// service must be drained before callers invoke it, so every accepted JSONL
/// line and the metric snapshot belong to one stable prefix.
pub fn publish_support_bundle_from_files(
    destination: &Path,
    created_at: &str,
    build: &str,
    capability_digest: &str,
    config_digests: &[String],
    log_root: &Path,
) -> Result<(), ObservabilityError> {
    let mut logs = Vec::new();
    for base in ["selector.jsonl", "supervisor.jsonl"] {
        let path = log_root.join(base);
        for generation in (1..LOG_RETAINED_GENERATIONS).rev() {
            read_log_file(&rotated_path(&path, generation), &mut logs)?;
        }
        read_log_file(&path, &mut logs)?;
    }
    let metrics_path = log_root.join("metrics.canonical.json");
    let metric_bytes = fs::read(&metrics_path)?;
    let metrics: MetricSnapshot = serde_json::from_slice(&metric_bytes)
        .map_err(|error| ObservabilityError::Json(error.to_string()))?;
    if metrics.canonical_line()? != metric_bytes {
        return Err(ObservabilityError::InvalidMetric(
            "persisted metric snapshot is not canonical JSON plus LF",
        ));
    }
    publish_support_bundle(
        destination,
        created_at,
        build,
        capability_digest,
        config_digests,
        &logs,
        &metrics,
    )
}

pub fn operational_code_count(
    log_root: &Path,
    base: &str,
    code: &str,
) -> Result<u64, ObservabilityError> {
    let path = log_root.join(base);
    let mut logs = Vec::new();
    for generation in (1..LOG_RETAINED_GENERATIONS).rev() {
        read_log_file(&rotated_path(&path, generation), &mut logs)?;
    }
    read_log_file(&path, &mut logs)?;
    Ok(logs.iter().filter(|record| record.code == code).count() as u64)
}

fn read_log_file(path: &Path, records: &mut Vec<LogRecord>) -> Result<(), ObservabilityError> {
    let file = match open_existing_secure(path, false)? {
        Some((file, _)) => file,
        None => return Ok(()),
    };
    for line in BufReader::new(file).split(b'\n') {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let record: LogRecord = serde_json::from_slice(&line)
            .map_err(|error| ObservabilityError::Json(error.to_string()))?;
        let mut expected = record.canonical_line()?;
        expected.pop();
        if expected != line {
            return Err(ObservabilityError::InvalidLog(
                "persisted operational record is not canonical JSON",
            ));
        }
        records.push(record);
    }
    Ok(())
}

fn write_private(path: &Path, bytes: &[u8]) -> Result<(), ObservabilityError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    file.write_all(bytes)?;
    FullSync::full_sync(&file)?;
    Ok(())
}

fn sync_parent(path: &Path) -> io::Result<()> {
    FullSync::full_sync(&File::open(path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "path has no parent")
    })?)?)
}

fn digest(bytes: &[u8]) -> String {
    use std::fmt::Write as _;

    let digest = Sha256::digest(bytes);
    digest
        .iter()
        .fold(String::with_capacity(64), |mut hex, byte| {
            write!(&mut hex, "{byte:02x}").expect("writing to String cannot fail");
            hex
        })
}

fn validate_hex(value: &str) -> Result<(), ObservabilityError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        Ok(())
    } else {
        Err(ObservabilityError::InvalidDigest)
    }
}

fn reject_forbidden(bytes: &[u8]) -> Result<(), ObservabilityError> {
    if FORBIDDEN_SUPPORT_BYTES.iter().any(|needle| {
        bytes
            .windows(needle.len())
            .any(|window| window == needle.as_bytes())
    }) {
        Err(ObservabilityError::ForbiddenBytes)
    } else {
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ObservabilityError {
    #[error("invalid operational log: {0}")]
    InvalidLog(&'static str),
    #[error("invalid metric snapshot: {0}")]
    InvalidMetric(&'static str),
    #[error("digest is not 64 lowercase hexadecimal characters")]
    InvalidDigest,
    #[error("support bundle contains forbidden bytes")]
    ForbiddenBytes,
    #[error("support bundle destination already exists")]
    DestinationExists,
    #[error("support bundle destination has no parent")]
    DestinationWithoutParent,
    #[error("JSON serialization failed: {0}")]
    Json(String),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;
    use std::os::unix::fs::{PermissionsExt, symlink};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Barrier};

    use super::{
        Correlation, FrozenAttribution, LogRecord, ObservabilityError, OperationalMetrics,
        ProductionAccessLog, RotatingJsonlLog, Severity, rotated_path,
    };

    fn record() -> LogRecord {
        LogRecord {
            v: 1,
            ts: "2026-08-28T00:00:00.000000000Z".to_owned(),
            severity: Severity::Info,
            component: "supervisor".to_owned(),
            build: "2.0.0".to_owned(),
            code: "boot-recovery-complete".to_owned(),
            message: "Supervisor boot recovery completed".to_owned(),
            correlation: Correlation {
                launch_id: Some("2-2-0123456789abcdef0123456789abcdef".to_owned()),
                attempt: Some(2),
                generation: Some(2),
                manifest_sha256: Some(
                    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_owned(),
                ),
                ..Correlation::default()
            },
            fields: BTreeMap::from([
                ("generation".to_owned(), 2.into()),
                ("version".to_owned(), "2.0.0".into()),
            ]),
        }
    }

    fn attribution() -> FrozenAttribution {
        FrozenAttribution {
            build: "2.0.0".to_owned(),
            launch_id: "2-2-0123456789abcdef0123456789abcdef".to_owned(),
            attempt: 2,
            generation: 2,
            manifest_sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .to_owned(),
        }
    }

    #[test]
    fn operational_log_rejects_a_symlink_to_a_semantic_ledger() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let ledger = directory.path().join("main.jsonl");
        fs::write(&ledger, b"semantic-ledger\n").expect("ledger fixture");
        fs::set_permissions(&ledger, fs::Permissions::from_mode(0o600))
            .expect("private ledger mode");
        let log_path = directory.path().join("supervisor.jsonl");
        symlink(&ledger, &log_path).expect("log symlink");

        let error = RotatingJsonlLog::new(&log_path)
            .append(&record())
            .expect_err("symlink log path must fail closed");
        assert!(matches!(
            error,
            ObservabilityError::Io(ref error) if error.raw_os_error() == Some(libc::ELOOP)
        ));
        assert_eq!(
            fs::read(&ledger).expect("ledger remains readable"),
            b"semantic-ledger\n"
        );
    }

    #[test]
    fn concurrent_append_and_rotation_preserve_both_canonical_records() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let path = directory.path().join("supervisor.jsonl");
        let log = Arc::new(RotatingJsonlLog::with_rotation_bytes(&path, 1));
        let barrier = Arc::new(Barrier::new(2));
        let mut threads = Vec::new();
        for _ in 0..2 {
            let log = Arc::clone(&log);
            let barrier = Arc::clone(&barrier);
            threads.push(std::thread::spawn(move || {
                barrier.wait();
                log.append(&record()).expect("concurrent append");
            }));
        }
        for thread in threads {
            thread.join().expect("append thread");
        }

        let active = fs::read(&path).expect("active log");
        let rotated = fs::read(rotated_path(&path, 1)).expect("rotated log");
        assert_eq!(active, record().canonical_line().expect("canonical line"));
        assert_eq!(rotated, record().canonical_line().expect("canonical line"));
    }

    #[test]
    fn production_append_failure_marks_fault_and_invokes_readiness_hook() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let path = directory.path().join("supervisor.jsonl");
        fs::write(&path, b"").expect("log fixture");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).expect("unsafe fixture mode");
        let production = ProductionAccessLog::new(
            Arc::new(OperationalMetrics::default()),
            Arc::new(RotatingJsonlLog::new(&path)),
            attribution(),
        );
        let healthy = Arc::new(AtomicBool::new(true));
        let observed = Arc::clone(&healthy);
        production.install_health_hook(Arc::new(move |value| {
            observed.store(value, Ordering::Release);
        }));

        production.record_semantic_failure(
            "provider-terminal",
            "Provider attempt reached a terminal error",
            "http-terminal",
            "session-a",
            7,
        );

        assert!(production.is_faulted());
        assert!(!healthy.load(Ordering::Acquire));
        assert!(
            fs::read(&path)
                .expect("unsafe log remains readable")
                .is_empty()
        );

        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
            .expect("repair log permissions");
        production.record_semantic_failure(
            "provider-terminal",
            "Provider attempt reached a terminal error",
            "http-terminal",
            "session-a",
            8,
        );
        assert!(!production.is_faulted());
        assert!(healthy.load(Ordering::Acquire));
    }
}
