use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fmt::Write as FmtWrite;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::canary::verify_canary_ledger;
use crate::cli::{rfc3339_after, rfc3339_now};
use crate::error::{IoContext, SelectorError};
use crate::fs::{
    ExistingLockState, FileLock, atomic_bytes, atomic_json, atomic_symlink, canonical_line,
    copy_regular, create_private_dir, full_sync, mode, probe_existing_lock, read_active,
    read_canonical, read_regular, relative_active_target, remove_dir_if_exists, sha256,
    sync_directory, validate_hex, validate_id,
};
use crate::model::{
    BundleManifest, CanaryReply, ConformanceReply, InstallHealthReply, InstallIdentity,
    MutationReply, Observation, ObservationState, OperationActor, OperationType, PrelaunchFailure,
    PreviousFile, RecoverReply, Selection, SelectionFile, SelectorManifest, SelectorOperation,
    StageReply, StatusReply, UpdateReply,
};
use crate::signature::CodeSignatureVerifier;

const BUNDLE_FILES: [(&str, &str); 7] = [
    ("apps/TekesKernelSupervisor.app/Contents/Info.plist", "0644"),
    (
        "apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor",
        "0755",
    ),
    (
        "apps/TekesKernelSupervisor.app/Contents/Resources/WebClientManifest.canonical.json",
        "0644",
    ),
    (
        "apps/TekesKernelSupervisor.app/Contents/_CodeSignature/CodeResources",
        "0644",
    ),
    (
        "apps/TekesKernelSupervisor.app/Contents/embedded.provisionprofile",
        "0644",
    ),
    ("bin/tekes-helper", "0755"),
    ("bin/tekes-worker", "0755"),
];

// Existing installations before the optional Web Client contain the same
// signed supervisor product without the embedded resource manifest. Keep that
// exact historical file registry admissible so a newly published selector can
// validate the current and previous selections while staging the new format.
const LEGACY_BUNDLE_FILES: [(&str, &str); 6] = [
    ("apps/TekesKernelSupervisor.app/Contents/Info.plist", "0644"),
    (
        "apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor",
        "0755",
    ),
    (
        "apps/TekesKernelSupervisor.app/Contents/_CodeSignature/CodeResources",
        "0644",
    ),
    (
        "apps/TekesKernelSupervisor.app/Contents/embedded.provisionprofile",
        "0644",
    ),
    ("bin/tekes-helper", "0755"),
    ("bin/tekes-worker", "0755"),
];
pub const EMBEDDED_AUTHORITY_REGISTRY_SHA256: &str =
    "f1f084f11ee379fd19ff0b62db653e245a088f7055f2146a5e1adf49decf5469";
pub const EMBEDDED_SELECTOR_CONFORMANCE_SHA256: Option<&str> =
    option_env!("TEKES_SELECTOR_CONFORMANCE_SHA256");
static SIGNAL_COUNT: AtomicU8 = AtomicU8::new(0);

fn embedded_selector_version() -> &'static str {
    option_env!("TEKES_SELECTED_BUILD").unwrap_or(env!("CARGO_PKG_VERSION"))
}
const ATTRIBUTABLE: [&str; 7] = [
    "canary-attribution-failed",
    "canary-timeout",
    "invalid-install",
    "protocol-mismatch",
    "readiness-timeout",
    "selector-mismatch",
    "unexpected-child-exit",
];
const ENVIRONMENT: [&str; 8] = [
    "already-running",
    "corrupt-ledger",
    "endpoint-credential-unavailable",
    "invalid-config",
    "io",
    "listener-unavailable",
    "required-broker-unavailable",
    "unsupported-filesystem",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
enum LogScalar {
    Bool(bool),
    Integer(u64),
    String(String),
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct LogCorrelation {
    #[serde(skip_serializing_if = "Option::is_none")]
    attempt: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manifest_sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct SelectorLogRecord {
    build: String,
    code: String,
    component: String,
    correlation: LogCorrelation,
    fields: BTreeMap<String, LogScalar>,
    message: String,
    severity: String,
    ts: String,
    v: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RetryFingerprint {
    config_sha256: String,
    identity_sha256: String,
    installer_operation_sha256: String,
    listener_bindable: bool,
    root_lock: ExistingLockState,
    storage_identity: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LogCorrelationShape {
    Empty,
    Operation,
    Child,
    ChildOperation,
}

fn selector_log_contract(
    code: &str,
) -> Option<(
    &'static str,
    &'static str,
    &'static [&'static str],
    LogCorrelationShape,
)> {
    Some(match code {
        "selector-predecessor-draining" => (
            "warn",
            "Predecessor is still draining",
            &["root_lock"],
            LogCorrelationShape::Empty,
        ),
        "orphan-owner-timeout" => (
            "error",
            "Predecessor ownership did not drain before deadline",
            &[
                "deadline_ms",
                "generation",
                "manifest_sha256",
                "root_lock",
                "version",
            ],
            LogCorrelationShape::Empty,
        ),
        "selector-prelaunch-cleared" => (
            "info",
            "Prelaunch ownership failure cleared",
            &["root_lock"],
            LogCorrelationShape::Empty,
        ),
        "selector-recovered" => (
            "info",
            "Selector operation recovered",
            &["operation", "phase"],
            LogCorrelationShape::Operation,
        ),
        "selector-update-complete" => (
            "info",
            "Selector executable update completed",
            &["sha256", "version"],
            LogCorrelationShape::Empty,
        ),
        "selector-child-launch" => (
            "info",
            "Selected supervisor launch started",
            &["canary_required"],
            LogCorrelationShape::Child,
        ),
        "promotion-complete" => (
            "info",
            "Selected build promotion completed",
            &["from_state"],
            LogCorrelationShape::Child,
        ),
        "rollback-complete" => (
            "warn",
            "Rollback completed",
            &["from", "reason", "to"],
            LogCorrelationShape::ChildOperation,
        ),
        "readiness-crash-loop" => (
            "error",
            "Selected supervisor reached the crash-loop threshold",
            &["classification"],
            LogCorrelationShape::Child,
        ),
        failure if ATTRIBUTABLE.contains(&failure) => (
            "error",
            "Selected supervisor launch failed",
            &["classification"],
            LogCorrelationShape::Child,
        ),
        failure if ENVIRONMENT.contains(&failure) || failure == "launcher-interrupted" => (
            "warn",
            "Supervisor launch environment is unavailable",
            &["classification"],
            LogCorrelationShape::Child,
        ),
        _ => return None,
    })
}

const fn operation_type_name(operation: &OperationType) -> &'static str {
    match operation {
        OperationType::Stage => "stage",
        OperationType::Activate => "activate",
        OperationType::Rollback => "rollback",
    }
}

#[derive(Clone, Debug)]
pub struct SelectorPaths {
    pub install_root: PathBuf,
    pub bundles: PathBuf,
    pub selector: PathBuf,
    pub operations: PathBuf,
    pub observations: PathBuf,
    pub current: PathBuf,
    pub previous: PathBuf,
    pub active: PathBuf,
    pub lock: PathBuf,
    pub service_lock: PathBuf,
    pub operation: PathBuf,
    pub install_identity: PathBuf,
    pub installer_operation: PathBuf,
    pub operational_log: PathBuf,
    pub storage_root: PathBuf,
}

impl SelectorPaths {
    #[must_use]
    pub fn new(install_root: impl Into<PathBuf>) -> Self {
        let install_root = install_root.into();
        let selector = install_root.join("selector");
        let installer = install_root.parent().map_or_else(
            || install_root.join("Installer"),
            |kernel_parent| kernel_parent.join("Installer"),
        );
        let data_root = install_root
            .parent()
            .map_or_else(|| install_root.clone(), Path::to_path_buf);
        let user_home = install_root
            .parent()
            .and_then(Path::parent)
            .and_then(Path::parent)
            .and_then(Path::parent);
        let production_layout = install_root.file_name() == Some(OsStr::new("Kernel"))
            && install_root.parent().and_then(Path::file_name) == Some(OsStr::new("Tekes"))
            && install_root
                .parent()
                .and_then(Path::parent)
                .and_then(Path::file_name)
                == Some(OsStr::new("Application Support"));
        let storage_root = if production_layout {
            user_home
                .map(|home| home.join(".agents/threads"))
                .unwrap_or_else(|| data_root.join("threads"))
        } else {
            data_root.join("threads")
        };
        let log_root = if production_layout {
            user_home
                .map(|home| home.join(".agents/logs/kernel"))
                .unwrap_or_else(|| install_root.join("logs"))
        } else {
            install_root
                .parent()
                .and_then(Path::parent)
                .and_then(Path::parent)
                .map_or_else(
                    || install_root.join("logs"),
                    |library| library.join("Logs/Tekes/Kernel"),
                )
        };
        Self {
            bundles: install_root.join("bundles"),
            operations: selector.join("operations"),
            observations: selector.join("observations"),
            current: selector.join("current.json"),
            previous: selector.join("previous.json"),
            active: selector.join("active"),
            lock: selector.join(".lock"),
            service_lock: selector.join(".service-lock"),
            operation: selector.join("operations/current.json"),
            install_identity: installer.join("install-identity.json"),
            installer_operation: installer.join("operation.json"),
            operational_log: log_root.join("selector.jsonl"),
            storage_root,
            selector,
            install_root,
        }
    }

    pub fn initialize_for_install(&self) -> Result<(), SelectorError> {
        for path in [
            &self.install_root,
            &self.bundles,
            &self.selector,
            &self.operations,
            &self.observations,
            &self.selector.join("bin"),
        ] {
            if !path.exists() {
                fs::DirBuilder::new()
                    .recursive(false)
                    .mode(0o700)
                    .create(path)
                    .selector_io("directory-create")?;
            }
            if fs::symlink_metadata(path)
                .selector_io("directory-stat")?
                .file_type()
                .is_symlink()
                || !path.is_dir()
                || mode(path)? != 0o700
            {
                return Err(SelectorError::corruption(path.display().to_string()));
            }
        }
        for path in [&self.lock, &self.service_lock] {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .mode(0o600)
                .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
                .open(path)
                .selector_io("lock-create")?;
            file.set_permissions(fs::Permissions::from_mode(0o600))
                .selector_io("lock-mode")?;
        }
        sync_directory(&self.selector)
    }
}

pub struct Selector<V> {
    paths: SelectorPaths,
    verifier: V,
}

impl<V: CodeSignatureVerifier> Selector<V> {
    #[must_use]
    pub fn new(install_root: impl Into<PathBuf>, verifier: V) -> Self {
        Self {
            paths: SelectorPaths::new(install_root),
            verifier,
        }
    }

    #[must_use]
    pub fn paths(&self) -> &SelectorPaths {
        &self.paths
    }

    fn emit_selector_log(
        &self,
        build: &str,
        code: &str,
        correlation: LogCorrelation,
        fields: BTreeMap<String, LogScalar>,
    ) -> Result<(), SelectorError> {
        let (severity, message, allowed, _) = selector_log_contract(code)
            .ok_or_else(|| SelectorError::corruption("selector-log-code"))?;
        if fields.len() != allowed.len()
            || fields
                .keys()
                .map(String::as_str)
                .ne(allowed.iter().copied())
        {
            return Err(SelectorError::corruption("selector-log-fields"));
        }
        let record = SelectorLogRecord {
            build: build.to_owned(),
            code: code.to_owned(),
            component: "selector".to_owned(),
            correlation,
            fields,
            message: message.to_owned(),
            severity: severity.to_owned(),
            ts: rfc3339_now()?,
            v: 1,
        };
        if !valid_selector_log_record(&record) {
            return Err(SelectorError::corruption("selector-log-record"));
        }
        append_rotating_log(&self.paths.operational_log, &canonical_line(&record)?)
    }

    fn child_correlation(observation: &Observation) -> LogCorrelation {
        LogCorrelation {
            attempt: Some(observation.attempt),
            generation: Some(observation.generation),
            launch_id: Some(observation.launch_id.clone()),
            manifest_sha256: Some(observation.manifest_sha256.clone()),
            operation_id: None,
        }
    }

    fn retry_fingerprint(
        &self,
        storage_root: &Path,
        listen: &str,
    ) -> Result<RetryFingerprint, SelectorError> {
        let data_root = storage_root
            .parent()
            .ok_or_else(|| SelectorError::corruption(storage_root.display().to_string()))?;
        let config_sha256 = tree_fingerprint(&data_root.join("config"))?;
        let identity_sha256 = sha256(&read_regular(&self.paths.install_identity)?);
        let installer_operation_sha256 = if self.paths.installer_operation.exists() {
            sha256(&read_regular(&self.paths.installer_operation)?)
        } else {
            sha256(b"missing")
        };
        let metadata = fs::symlink_metadata(storage_root).selector_io("retry-storage-stat")?;
        let storage_identity = format!(
            "{}:{}:{:o}",
            metadata.dev(),
            metadata.ino(),
            metadata.permissions().mode() & 0o7777
        );
        let listener_bindable = TcpListener::bind(listen).is_ok();
        Ok(RetryFingerprint {
            config_sha256,
            identity_sha256,
            installer_operation_sha256,
            listener_bindable,
            root_lock: probe_existing_lock(&storage_root.join(".root-lock"))?,
            storage_identity,
        })
    }

    fn wait_environment_retry(
        &self,
        storage_root: &Path,
        listen: &str,
        delay: Duration,
    ) -> Result<bool, SelectorError> {
        let initial = self.retry_fingerprint(storage_root, listen)?;
        let deadline = Instant::now() + delay;
        while Instant::now() < deadline {
            if SIGNAL_COUNT.load(Ordering::Relaxed) != 0 {
                return Ok(false);
            }
            std::thread::sleep(Duration::from_millis(100));
            if self.retry_fingerprint(storage_root, listen)? != initial {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn stage(
        &self,
        bundle: &Path,
        version: &str,
        command_sha256: String,
    ) -> Result<StageReply, SelectorError> {
        validate_id(version)?;
        let (manifest, selection) = self.validate_bundle(bundle, Some(version))?;
        let _lock = FileLock::try_exclusive(&self.paths.lock)?;
        self.recover_locked()?;
        if let Some(reply) = self.retry_reply::<StageReply>(&command_sha256)? {
            return Ok(reply);
        }
        let generation = self
            .read_selection_set()?
            .0
            .map_or(0, |file| file.generation);
        let final_path = self.paths.bundles.join(version);
        let final_already_published = if final_path.exists() {
            let (_, published) = self.validate_bundle(&final_path, Some(version))?;
            if published != selection {
                return Err(SelectorError::invalid_bundle(
                    final_path.display().to_string(),
                ));
            }
            true
        } else {
            false
        };
        let op_id = format!("stage-{}", &command_sha256[..16]);
        let mut operation = SelectorOperation {
            actor: OperationActor::Cli,
            command_sha256,
            format: 1,
            from: None,
            generation,
            launch_id: None,
            op_id: op_id.clone(),
            operation_type: OperationType::Stage,
            phase: "prepared".to_owned(),
            reason: None,
            response: None,
            to: selection.clone(),
        };
        atomic_json(&self.paths.operation, &operation)?;
        if !final_already_published {
            let staging = self.paths.bundles.join(format!(".{op_id}.staging"));
            remove_dir_if_exists(&staging)?;
            self.copy_bundle(bundle, &staging, &manifest)?;
            operation.phase = "copied".to_owned();
            atomic_json(&self.paths.operation, &operation)?;
            self.validate_bundle(&staging, Some(version))?;
            operation.phase = "verified".to_owned();
            atomic_json(&self.paths.operation, &operation)?;
            fs::rename(&staging, &final_path).selector_io("bundle-rename")?;
            sync_directory(&self.paths.bundles)?;
        }
        operation.phase = "bundle-published".to_owned();
        atomic_json(&self.paths.operation, &operation)?;
        let reply = StageReply {
            format: 1,
            operation: "stage".to_owned(),
            selection,
        };
        operation.phase = "closed".to_owned();
        operation.response = Some(to_value(&reply)?);
        atomic_json(&self.paths.operation, &operation)?;
        Ok(reply)
    }

    pub fn activate(
        &self,
        version: &str,
        command_sha256: String,
    ) -> Result<MutationReply, SelectorError> {
        validate_id(version)?;
        let _service = self.acquire_offline_service()?;
        let _lock = FileLock::try_exclusive(&self.paths.lock)?;
        self.recover_locked()?;
        if let Some(reply) = self.retry_reply::<MutationReply>(&command_sha256)? {
            return Ok(reply);
        }
        let (_, to) = self.validate_bundle(&self.paths.bundles.join(version), Some(version))?;
        let (current, _) = self.read_selection_set()?;
        if current
            .as_ref()
            .is_some_and(|current| current.selection == to)
        {
            return self.close_no_effect_activate(to, command_sha256, current.expect("checked"));
        }
        if let Some(current) = &current {
            let old: BundleManifest = read_canonical(
                &self
                    .paths
                    .bundles
                    .join(&current.selection.version)
                    .join("manifest.canonical.json"),
            )?;
            let new: BundleManifest = read_canonical(
                &self
                    .paths
                    .bundles
                    .join(version)
                    .join("manifest.canonical.json"),
            )?;
            if old.compatibility != new.compatibility {
                return Err(SelectorError::invalid_bundle(version));
            }
        }
        let generation = current.as_ref().map_or(1, |file| file.generation + 1);
        let operation = SelectorOperation {
            actor: OperationActor::Cli,
            command_sha256,
            format: 1,
            from: current.map(|file| file.selection),
            generation,
            launch_id: None,
            op_id: format!("activate-{generation}"),
            operation_type: OperationType::Activate,
            phase: "prepared".to_owned(),
            reason: None,
            response: None,
            to,
        };
        atomic_json(&self.paths.operation, &operation)?;
        self.finish_selection_operation(operation)
    }

    pub fn rollback(
        &self,
        reason: &str,
        command_sha256: String,
    ) -> Result<MutationReply, SelectorError> {
        validate_id(reason)?;
        let _service = self.acquire_offline_service()?;
        let _lock = FileLock::try_exclusive(&self.paths.lock)?;
        self.recover_locked()?;
        if let Some(reply) = self.retry_reply::<MutationReply>(&command_sha256)? {
            return Ok(reply);
        }
        let (current, previous) = self.read_selection_set()?;
        let current = current.ok_or_else(|| SelectorError::invalid_state("not-activated"))?;
        let previous = previous
            .and_then(|file| file.selection)
            .ok_or_else(|| SelectorError::invalid_state("no-previous"))?;
        let observation = self
            .read_observation(&current.selection.version)?
            .ok_or_else(|| SelectorError::invalid_state("no-failed-observation"))?;
        if !matches!(observation.state, ObservationState::Failed) {
            return Err(SelectorError::invalid_state("observation-not-failed"));
        }
        if !observation.rollback_eligible {
            return Err(SelectorError::invalid_state("rollback-window-closed"));
        }
        let generation = current.generation + 1;
        let operation = SelectorOperation {
            actor: OperationActor::Cli,
            command_sha256,
            format: 1,
            from: Some(current.selection),
            generation,
            launch_id: None,
            op_id: format!("rollback-cli-{generation}"),
            operation_type: OperationType::Rollback,
            phase: "prepared".to_owned(),
            reason: Some(reason.to_owned()),
            response: None,
            to: previous,
        };
        let operation_id = operation.op_id.clone();
        let from = operation
            .from
            .as_ref()
            .expect("rollback has from")
            .version
            .clone();
        let to = operation.to.version.clone();
        atomic_json(&self.paths.operation, &operation)?;
        let reply = self.finish_selection_operation(operation)?;
        self.emit_selector_log(
            &from,
            "rollback-complete",
            LogCorrelation {
                operation_id: Some(operation_id),
                ..Self::child_correlation(&observation)
            },
            BTreeMap::from([
                ("from".to_owned(), LogScalar::String(from.clone())),
                ("reason".to_owned(), LogScalar::String(reason.to_owned())),
                ("to".to_owned(), LogScalar::String(to)),
            ]),
        )?;
        Ok(reply)
    }

    pub fn recover(&self) -> Result<RecoverReply, SelectorError> {
        let _lock = FileLock::try_exclusive(&self.paths.lock)?;
        let pending: Option<SelectorOperation> = if self.paths.operation.exists() {
            Some(read_canonical(&self.paths.operation)?)
        } else {
            None
        };
        let recovered = self.recover_locked()?;
        if recovered {
            let operation = pending.ok_or_else(|| {
                SelectorError::corruption(self.paths.operation.display().to_string())
            })?;
            self.emit_selector_log(
                embedded_selector_version(),
                "selector-recovered",
                LogCorrelation {
                    operation_id: Some(operation.op_id),
                    ..LogCorrelation::default()
                },
                BTreeMap::from([
                    (
                        "operation".to_owned(),
                        LogScalar::String(
                            operation_type_name(&operation.operation_type).to_owned(),
                        ),
                    ),
                    ("phase".to_owned(), LogScalar::String("closed".to_owned())),
                ]),
            )?;
        }
        let selection = self.read_selection_set()?.0;
        Ok(RecoverReply {
            format: 1,
            operation: "recover".to_owned(),
            recovered,
            selection,
        })
    }

    pub fn status(&self) -> Result<StatusReply, SelectorError> {
        let _lock = FileLock::try_exclusive(&self.paths.lock)?;
        self.recover_locked()?;
        let (selection, previous) = self.read_selection_set()?;
        let observation = match &selection {
            Some(current) => self.read_observation(&current.selection.version)?,
            None => None,
        };
        let service = match FileLock::try_exclusive(&self.paths.service_lock) {
            Ok(_lock) => "stopped",
            Err(error) if error.code == crate::ErrorCode::InvalidState => "running",
            Err(error) => return Err(error),
        };
        let last_failure = observation.as_ref().and_then(|value| {
            matches!(value.state, ObservationState::Failed).then(|| crate::Failure {
                code: value.last_code.clone(),
                launch_id: value.launch_id.clone(),
            })
        });
        let prelaunch_failure = self.read_prelaunch_failure()?;
        Ok(StatusReply {
            format: 1,
            last_failure,
            observation,
            prelaunch_failure,
            previous,
            selection,
            service: service.to_owned(),
        })
    }

    pub fn attest_canary(
        &self,
        version: &str,
        session: &str,
        run: &str,
        storage_root: &Path,
        accepted_at: &str,
        window_closes_at: &str,
    ) -> Result<CanaryReply, SelectorError> {
        validate_id(version)?;
        validate_id(run)?;
        if !valid_rfc3339_nano(accepted_at) || !valid_rfc3339_nano(window_closes_at) {
            return Err(SelectorError::invalid_state("canary-clock-invalid"));
        }
        verify_canary_ledger(storage_root, version, session, run)?;
        let _lock = self.acquire_transaction_lock()?;
        self.recover_locked()?;
        let current = self
            .read_selection_set()?
            .0
            .ok_or_else(|| SelectorError::invalid_state("not-activated"))?;
        if current.selection.version != version {
            return Err(SelectorError::invalid_state("canary-version-mismatch"));
        }
        let mut observation = self
            .read_observation(version)?
            .ok_or_else(|| SelectorError::invalid_state("no-pending-observation"))?;
        if observation.generation == current.generation
            && observation.canary_session.as_deref() == Some(session)
            && observation.canary_run.as_deref() == Some(run)
            && matches!(
                observation.state,
                ObservationState::Ready | ObservationState::Promoted
            )
        {
            return Ok(CanaryReply {
                format: 1,
                operation: "attest-canary".to_owned(),
                run: run.to_owned(),
                session: session.to_owned(),
                version: version.to_owned(),
            });
        }
        if !matches!(observation.state, ObservationState::Pending)
            || observation.generation != current.generation
            || !observation.canary_required
        {
            return Err(SelectorError::invalid_state("no-pending-canary"));
        }
        if observation
            .canary_deadline_at
            .as_deref()
            .is_none_or(|deadline| accepted_at >= deadline)
        {
            return Err(SelectorError::invalid_state("canary-timeout"));
        }
        observation.canary_session = Some(session.to_owned());
        observation.canary_run = Some(run.to_owned());
        observation.consecutive_failures = 0;
        observation.last_code = "ready".to_owned();
        if observation.rollback_eligible {
            observation.state = ObservationState::Ready;
            observation.window_closes_at = Some(window_closes_at.to_owned());
        } else {
            observation.state = ObservationState::Promoted;
            observation.window_closes_at = None;
        }
        self.publish_observation(&observation)?;
        Ok(CanaryReply {
            format: 1,
            operation: "attest-canary".to_owned(),
            run: run.to_owned(),
            session: session.to_owned(),
            version: version.to_owned(),
        })
    }

    /// Admits the exact installed build on the consumer-app lane after the independently signed
    /// product installer has observed its frozen build/generation readiness response. This lane
    /// cannot require a provider-backed model turn: a clean Tekes install has no provider secret
    /// yet. Production deployment continues to use `attest_canary` and its durable run ledger.
    pub fn attest_install_health(
        &self,
        version: &str,
        listen: &str,
        accepted_at: &str,
        window_closes_at: &str,
    ) -> Result<InstallHealthReply, SelectorError> {
        validate_id(version)?;
        if !valid_rfc3339_nano(accepted_at) || !valid_rfc3339_nano(window_closes_at) {
            return Err(SelectorError::invalid_state(
                "health-admission-clock-invalid",
            ));
        }
        let _lock = self.acquire_transaction_lock()?;
        self.recover_locked()?;
        let current = self
            .read_selection_set()?
            .0
            .ok_or_else(|| SelectorError::invalid_state("not-activated"))?;
        if current.selection.version != version {
            return Err(SelectorError::invalid_state(
                "health-admission-version-mismatch",
            ));
        }
        let mut observation = self
            .read_observation(version)?
            .ok_or_else(|| SelectorError::invalid_state("no-pending-observation"))?;
        if observation.generation == current.generation
            && !observation.canary_required
            && matches!(
                observation.state,
                ObservationState::Ready | ObservationState::Promoted
            )
        {
            return Ok(InstallHealthReply {
                format: 1,
                operation: "attest-install-health".to_owned(),
                version: version.to_owned(),
            });
        }
        if !matches!(observation.state, ObservationState::Pending)
            || observation.generation != current.generation
            || !observation.canary_required
            || observation.canary_session.is_some()
            || observation.canary_run.is_some()
        {
            return Err(SelectorError::invalid_state("no-pending-health-admission"));
        }
        if observation
            .canary_deadline_at
            .as_deref()
            .is_none_or(|deadline| accepted_at >= deadline)
        {
            return Err(SelectorError::invalid_state("canary-timeout"));
        }
        if !health_ready(listen, version, current.generation)? {
            return Err(SelectorError::invalid_state("readiness-mismatch"));
        }
        observation.canary_required = false;
        observation.canary_deadline_at = None;
        observation.consecutive_failures = 0;
        observation.last_code = "ready".to_owned();
        if observation.rollback_eligible {
            observation.state = ObservationState::Ready;
            observation.window_closes_at = Some(window_closes_at.to_owned());
        } else {
            observation.state = ObservationState::Promoted;
            observation.window_closes_at = None;
        }
        self.publish_observation(&observation)?;
        Ok(InstallHealthReply {
            format: 1,
            operation: "attest-install-health".to_owned(),
            version: version.to_owned(),
        })
    }

    pub fn update_selector(
        &self,
        artifact: &Path,
        manifest_path: &Path,
    ) -> Result<UpdateReply, SelectorError> {
        let _service = self.acquire_offline_service()?;
        {
            let _lock = FileLock::try_exclusive(&self.paths.lock)?;
            self.recover_locked()?;
        }
        let identity: InstallIdentity = read_canonical(&self.paths.install_identity)?;
        let manifest: SelectorManifest = read_canonical(manifest_path)?;
        self.validate_selector_manifest(&manifest, artifact, &identity)?;
        let bytes = read_regular(artifact)?;
        let stable = self.paths.selector.join("bin/tekes-selector");
        if stable.exists()
            && mode(&stable)? == 0o755
            && sha256(&read_regular(&stable)?) == manifest.file.sha256
        {
            self.validate_selector_manifest(&manifest, &stable, &identity)?;
            let reply = UpdateReply {
                format: 1,
                operation: "update-selector".to_owned(),
                sha256: manifest.file.sha256.clone(),
                version: manifest.version.clone(),
            };
            self.emit_selector_update_complete(&reply)?;
            return Ok(reply);
        }
        atomic_bytes(&stable, &bytes, 0o755)?;
        self.validate_selector_manifest(&manifest, &stable, &identity)?;
        let reply = UpdateReply {
            format: 1,
            operation: "update-selector".to_owned(),
            sha256: manifest.file.sha256,
            version: manifest.version,
        };
        self.emit_selector_update_complete(&reply)?;
        Ok(reply)
    }

    fn emit_selector_update_complete(&self, reply: &UpdateReply) -> Result<(), SelectorError> {
        self.emit_selector_log(
            embedded_selector_version(),
            "selector-update-complete",
            LogCorrelation::default(),
            BTreeMap::from([
                ("sha256".to_owned(), LogScalar::String(reply.sha256.clone())),
                (
                    "version".to_owned(),
                    LogScalar::String(reply.version.clone()),
                ),
            ]),
        )
    }

    fn acquire_offline_service(&self) -> Result<FileLock, SelectorError> {
        let service = FileLock::try_exclusive(&self.paths.service_lock)
            .map_err(|_| SelectorError::invalid_state("service-running"))?;
        match probe_existing_lock(&self.paths.storage_root.join(".root-lock"))? {
            ExistingLockState::Available => Ok(service),
            ExistingLockState::Busy => Err(SelectorError::invalid_state("child-group-running")),
            ExistingLockState::Missing => Err(SelectorError::corruption(
                self.paths
                    .storage_root
                    .join(".root-lock")
                    .display()
                    .to_string(),
            )),
        }
    }

    fn acquire_transaction_lock(&self) -> Result<FileLock, SelectorError> {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            match FileLock::try_exclusive(&self.paths.lock) {
                Ok(lock) => return Ok(lock),
                Err(error)
                    if error.code == crate::ErrorCode::InvalidState
                        && Instant::now() < deadline =>
                {
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(error) => return Err(error),
            }
        }
    }

    /// Runs the production resident launcher. The service lock is held for the
    /// complete lifetime; the transaction lock is acquired only by the short
    /// state calls made before/after process and network waits.
    pub fn serve(&self, storage_root: &Path, listen: &str) -> Result<(), SelectorError> {
        self.serve_with_web(storage_root, listen, None)
    }

    /// Runs the resident launcher with an optional, separately bound Web
    /// Client service. Absence is the production default and leaves no Web
    /// listener behind.
    pub fn serve_with_web(
        &self,
        storage_root: &Path,
        listen: &str,
        web_listen: Option<&str>,
    ) -> Result<(), SelectorError> {
        if listen != "127.0.0.1:7347" {
            return Err(SelectorError::usage(listen));
        }
        if web_listen.is_some_and(|value| value != "127.0.0.1:7357") {
            return Err(SelectorError::usage(web_listen.unwrap_or_default()));
        }
        self.serve_validated(storage_root, listen, web_listen)
    }

    /// Runs the resident launcher on an isolated, OS-assigned loopback port.
    ///
    /// This is an explicit integration-test seam. Production callers must use
    /// [`Selector::serve`], whose fixed endpoint contract remains
    /// `127.0.0.1:7347`.
    #[doc(hidden)]
    pub fn serve_test_loopback(
        &self,
        storage_root: &Path,
        listen: &str,
    ) -> Result<(), SelectorError> {
        let address = listen
            .parse::<SocketAddr>()
            .map_err(|_| SelectorError::usage(listen))?;
        if address.ip() != IpAddr::V4(Ipv4Addr::LOCALHOST)
            || address.port() == 0
            || address.port() == 7347
        {
            return Err(SelectorError::usage(listen));
        }
        self.serve_validated(storage_root, listen, None)
    }

    fn serve_validated(
        &self,
        storage_root: &Path,
        listen: &str,
        web_listen: Option<&str>,
    ) -> Result<(), SelectorError> {
        let _service = FileLock::try_exclusive(&self.paths.service_lock)
            .map_err(|_| SelectorError::invalid_state("service-running"))?;
        install_signal_handlers()?;
        let mut environment_delay_index = 0_usize;
        loop {
            if SIGNAL_COUNT.load(Ordering::Relaxed) != 0 {
                return Ok(());
            }
            if self.installer_recovery_required()? {
                std::thread::sleep(Duration::from_secs(1));
                continue;
            }
            let (current, previous, manifest, prior) = {
                let _lock = self.acquire_transaction_lock()?;
                self.recover_locked()?;
                let (current, previous) = self.read_selection_set()?;
                let current =
                    current.ok_or_else(|| SelectorError::invalid_state("not-activated"))?;
                // `read_selection_set` has already hash- and signature-validated both selected
                // bundles. Read the manifest needed for child launch from that validated path;
                // running the complete bundle verifier again here doubled cold-start work.
                let manifest: BundleManifest = read_canonical(
                    &self
                        .paths
                        .bundles
                        .join(&current.selection.version)
                        .join("manifest.canonical.json"),
                )
                .map_err(|_| SelectorError::corruption(self.paths.current.display().to_string()))?;
                let prior = self.read_observation(&current.selection.version)?;
                (current, previous, manifest, prior)
            };
            let canary_required = prior.as_ref().is_none_or(|observation| {
                matches!(
                    observation.state,
                    ObservationState::Pending | ObservationState::Failed
                ) && observation.canary_session.is_none()
            });
            if matches!(
                probe_existing_lock(&storage_root.join(".root-lock"))?,
                ExistingLockState::Busy
            ) {
                self.emit_selector_log(
                    embedded_selector_version(),
                    "selector-predecessor-draining",
                    LogCorrelation::default(),
                    BTreeMap::from([(
                        "root_lock".to_owned(),
                        LogScalar::String("busy".to_owned()),
                    )]),
                )?;
            }
            if !self.wait_for_predecessor(storage_root)? {
                if SIGNAL_COUNT.load(Ordering::Relaxed) != 0 {
                    return Ok(());
                }
                self.record_predecessor_timeout(&current)?;
                return self.park_without_child();
            }
            self.clear_prelaunch_failure()?;
            let prior_deadline = prior
                .as_ref()
                .and_then(|observation| observation.canary_deadline_at.clone());
            let attempt = prior
                .as_ref()
                .map_or(1, |observation| observation.attempt + 1);
            let launch_id = launch_id(current.generation, attempt)?;
            let canary_deadline_at = canary_required.then_some(prior_deadline).flatten();
            let observation = Observation {
                attempt,
                canary_deadline_at,
                canary_required,
                canary_run: prior
                    .as_ref()
                    .and_then(|observation| observation.canary_run.clone()),
                canary_session: prior
                    .as_ref()
                    .and_then(|observation| observation.canary_session.clone()),
                consecutive_failures: prior
                    .as_ref()
                    .map_or(0, |observation| observation.consecutive_failures),
                deadline_at: rfc3339_after(30)?,
                format: 1,
                generation: current.generation,
                last_code: "launching".to_owned(),
                launch_id: launch_id.clone(),
                manifest_sha256: current.selection.manifest_sha256.clone(),
                rollback_eligible: previous
                    .as_ref()
                    .and_then(|file| file.selection.as_ref())
                    .is_some()
                    && prior
                        .as_ref()
                        .is_none_or(|observation| observation.rollback_eligible),
                started_at: rfc3339_now()?,
                state: ObservationState::Pending,
                version: current.selection.version.clone(),
                window_closes_at: prior
                    .as_ref()
                    .and_then(|observation| observation.window_closes_at.clone()),
            };
            let canary_deadline_at = observation.canary_deadline_at.clone();
            self.begin_observation_for_validated_selection(observation.clone(), &current)?;
            self.emit_selector_log(
                &observation.version,
                "selector-child-launch",
                Self::child_correlation(&observation),
                BTreeMap::from([(
                    "canary_required".to_owned(),
                    LogScalar::Bool(observation.canary_required),
                )]),
            )?;
            let executable = self
                .paths
                .bundles
                .join(&current.selection.version)
                .join("apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor");
            let mut health_recorded = false;
            let outcome = launch_and_observe(
                &NativeLaunchSpec {
                    executable: &executable,
                    install_root: &self.paths.install_root,
                    storage_root,
                    listen,
                    web_listen,
                    selection: &current,
                    authority_registry_sha256: &manifest.compatibility.authority_registry_sha256,
                    launch_id: &launch_id,
                    canary_required,
                    canary_deadline_at: canary_deadline_at.as_deref(),
                },
                || {
                    self.record_listener_bound(&current.selection.version, &launch_id)?;
                    environment_delay_index = 0;
                    Ok(())
                },
                || {
                    if !health_recorded {
                        self.mark_ready_after_health(&current.selection.version, &launch_id)?;
                        health_recorded = true;
                    }
                    let mut observation =
                        self.read_observation_unlocked(&current.selection.version)?;
                    let now = rfc3339_now()?;
                    let promotion_due = observation.as_ref().is_some_and(|value| {
                        matches!(value.state, ObservationState::Ready)
                            && value.rollback_eligible
                            && value
                                .window_closes_at
                                .as_deref()
                                .is_some_and(|closes| now.as_str() >= closes)
                    });
                    if promotion_due {
                        self.promote_validated_observation_if_due(
                            &current.selection.version,
                            &now,
                        )?;
                        observation = self.read_observation_unlocked(&current.selection.version)?;
                    }
                    Ok(observation)
                },
            )?;
            match outcome {
                NativeLaunchOutcome::PlannedExit => return Ok(()),
                NativeLaunchOutcome::ReadyThenExited => {
                    let disposition = self.record_failure(
                        &current.selection.version,
                        &launch_id,
                        "unexpected-child-exit",
                    )?;
                    if disposition == FailureDisposition::AutomaticRollbackRequired {
                        let previous_selection = previous
                            .and_then(|file| file.selection)
                            .ok_or_else(|| SelectorError::invalid_state("no-previous"))?;
                        let digest = automatic_rollback_sha256(
                            current.generation + 1,
                            &launch_id,
                            "readiness-crash-loop",
                            &current.selection,
                            &previous_selection,
                        )?;
                        self.automatic_rollback(&launch_id, "readiness-crash-loop", digest)?;
                    } else if disposition == FailureDisposition::CrashLoop {
                        return self.park_without_child();
                    }
                    std::thread::sleep(Duration::from_secs(1));
                }
                NativeLaunchOutcome::Failed(code) => {
                    let disposition =
                        self.record_failure(&current.selection.version, &launch_id, code)?;
                    match disposition {
                        FailureDisposition::AutomaticRollbackRequired => {
                            let previous_selection = previous
                                .and_then(|file| file.selection)
                                .ok_or_else(|| SelectorError::invalid_state("no-previous"))?;
                            let digest = automatic_rollback_sha256(
                                current.generation + 1,
                                &launch_id,
                                "readiness-crash-loop",
                                &current.selection,
                                &previous_selection,
                            )?;
                            self.automatic_rollback(&launch_id, "readiness-crash-loop", digest)?;
                            environment_delay_index = 0;
                        }
                        FailureDisposition::CrashLoop => {
                            return self.park_without_child();
                        }
                        FailureDisposition::EnvironmentRetry => {
                            const DELAYS: [u64; 5] = [1, 2, 4, 8, 30];
                            let changed = self.wait_environment_retry(
                                storage_root,
                                listen,
                                Duration::from_secs(DELAYS[environment_delay_index.min(4)]),
                            )?;
                            if changed {
                                environment_delay_index = 0;
                            } else {
                                environment_delay_index = (environment_delay_index + 1).min(4);
                            }
                        }
                        _ => std::thread::sleep(Duration::from_secs(1)),
                    }
                }
            }
        }
    }

    pub fn begin_observation(&self, observation: Observation) -> Result<(), SelectorError> {
        let _lock = self.acquire_transaction_lock()?;
        self.recover_locked()?;
        let current = self
            .read_selection_set()?
            .0
            .ok_or_else(|| SelectorError::invalid_state("not-activated"))?;
        if current.generation != observation.generation
            || current.selection.version != observation.version
        {
            return Err(SelectorError::invalid_state(
                "observation-selection-mismatch",
            ));
        }
        self.begin_observation_locked(observation)
    }

    /// Service startup has already validated the exact current bundle immediately before this
    /// call. Re-check only the small selection records under the transaction lock; repeating all
    /// hashes, code-signature checks and provisioning inspection here was the dominant cold-start
    /// CPU spike. A changed generation fails closed and the outer service loop can restart.
    fn begin_observation_for_validated_selection(
        &self,
        observation: Observation,
        expected: &SelectionFile,
    ) -> Result<(), SelectorError> {
        let _lock = self.acquire_transaction_lock()?;
        self.ensure_current_selection_unlocked(expected)?;
        self.begin_observation_locked(observation)
    }

    fn begin_observation_locked(&self, mut observation: Observation) -> Result<(), SelectorError> {
        let prior = self.read_observation(&observation.version)?;
        observation.attempt = prior.as_ref().map_or(1, |old| old.attempt + 1);
        observation.consecutive_failures = prior.as_ref().map_or(0, |old| old.consecutive_failures);
        observation.state = ObservationState::Pending;
        if let Some(old) = prior {
            if old.generation == observation.generation {
                observation.canary_session = old.canary_session;
                observation.canary_run = old.canary_run;
                if observation.canary_required {
                    observation.canary_deadline_at =
                        old.canary_deadline_at.or(observation.canary_deadline_at);
                }
                observation.window_closes_at =
                    old.window_closes_at.or(observation.window_closes_at);
            }
        }
        self.publish_observation(&observation)
    }

    fn mark_ready_after_health(
        &self,
        version: &str,
        launch_id: &str,
    ) -> Result<bool, SelectorError> {
        let _lock = self.acquire_transaction_lock()?;
        let mut observation = self
            .read_observation(version)?
            .ok_or_else(|| SelectorError::invalid_state("no-observation"))?;
        self.ensure_observation_selection_current_unlocked(&observation)?;
        if observation.launch_id != launch_id {
            return Err(SelectorError::invalid_state("stale-launch"));
        }
        if observation.canary_required || !matches!(observation.state, ObservationState::Pending) {
            return Ok(false);
        }
        observation.consecutive_failures = 0;
        observation.last_code = "ready".to_owned();
        if observation.rollback_eligible {
            if observation.window_closes_at.is_none() {
                return Err(SelectorError::corruption("selector/observations"));
            }
            observation.state = ObservationState::Ready;
        } else {
            observation.state = ObservationState::Promoted;
            observation.window_closes_at = None;
        }
        self.publish_observation(&observation)?;
        Ok(true)
    }

    fn record_listener_bound(&self, version: &str, launch_id: &str) -> Result<(), SelectorError> {
        let _lock = self.acquire_transaction_lock()?;
        let mut observation = self
            .read_observation(version)?
            .ok_or_else(|| SelectorError::invalid_state("no-observation"))?;
        self.ensure_observation_selection_current_unlocked(&observation)?;
        if observation.launch_id != launch_id
            || !matches!(observation.state, ObservationState::Pending)
        {
            return Err(SelectorError::invalid_state("stale-launch"));
        }
        observation.last_code = "listener-bound".to_owned();
        if observation.canary_required && observation.canary_deadline_at.is_none() {
            observation.canary_deadline_at = Some(rfc3339_after(120)?);
        }
        self.publish_observation(&observation)
    }

    fn park_without_child(&self) -> Result<(), SelectorError> {
        while SIGNAL_COUNT.load(Ordering::Relaxed) == 0 {
            std::thread::sleep(Duration::from_secs(1));
        }
        Ok(())
    }

    fn wait_for_predecessor(&self, storage_root: &Path) -> Result<bool, SelectorError> {
        self.wait_for_predecessor_until(storage_root, Instant::now() + Duration::from_secs(30))
    }

    fn wait_for_predecessor_until(
        &self,
        storage_root: &Path,
        deadline: Instant,
    ) -> Result<bool, SelectorError> {
        let root_lock = storage_root.join(".root-lock");
        loop {
            match probe_existing_lock(&root_lock)? {
                ExistingLockState::Available | ExistingLockState::Missing => return Ok(true),
                ExistingLockState::Busy if Instant::now() < deadline => {
                    if SIGNAL_COUNT.load(Ordering::Relaxed) != 0 {
                        return Ok(false);
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
                ExistingLockState::Busy => return Ok(false),
            }
        }
    }

    fn record_predecessor_timeout(&self, current: &SelectionFile) -> Result<(), SelectorError> {
        let _lock = self.acquire_transaction_lock()?;
        self.recover_locked()?;
        self.emit_selector_log(
            embedded_selector_version(),
            "orphan-owner-timeout",
            LogCorrelation::default(),
            BTreeMap::from([
                ("deadline_ms".to_owned(), LogScalar::Integer(30_000)),
                (
                    "generation".to_owned(),
                    LogScalar::Integer(current.generation),
                ),
                (
                    "manifest_sha256".to_owned(),
                    LogScalar::String(current.selection.manifest_sha256.clone()),
                ),
                ("root_lock".to_owned(), LogScalar::String("busy".to_owned())),
                (
                    "version".to_owned(),
                    LogScalar::String(current.selection.version.clone()),
                ),
            ]),
        )
    }

    fn clear_prelaunch_failure(&self) -> Result<(), SelectorError> {
        if self.read_prelaunch_failure()?.is_some() {
            self.emit_selector_log(
                embedded_selector_version(),
                "selector-prelaunch-cleared",
                LogCorrelation::default(),
                BTreeMap::from([(
                    "root_lock".to_owned(),
                    LogScalar::String("available".to_owned()),
                )]),
            )?;
        }
        Ok(())
    }

    fn read_prelaunch_failure(&self) -> Result<Option<PrelaunchFailure>, SelectorError> {
        let mut active = None;
        for record in read_rotating_logs(&self.paths.operational_log)? {
            match record.code.as_str() {
                "orphan-owner-timeout" => {
                    let generation = match record.fields.get("generation") {
                        Some(LogScalar::Integer(value)) if *value >= 1 => *value,
                        _ => return Err(SelectorError::corruption("selector-log-fields")),
                    };
                    let manifest_sha256 = match record.fields.get("manifest_sha256") {
                        Some(LogScalar::String(value)) if validate_hex(value) => value.clone(),
                        _ => return Err(SelectorError::corruption("selector-log-fields")),
                    };
                    let version = match record.fields.get("version") {
                        Some(LogScalar::String(value)) if validate_id(value).is_ok() => {
                            value.clone()
                        }
                        _ => return Err(SelectorError::corruption("selector-log-fields")),
                    };
                    active = Some(PrelaunchFailure {
                        code: record.code,
                        format: 1,
                        generation,
                        manifest_sha256,
                        observed_at: record.ts,
                        version,
                    });
                }
                "selector-prelaunch-cleared" => active = None,
                _ => {}
            }
        }
        Ok(active)
    }

    pub fn record_failure(
        &self,
        version: &str,
        launch_id: &str,
        code: &str,
    ) -> Result<FailureDisposition, SelectorError> {
        if !ATTRIBUTABLE.contains(&code)
            && !ENVIRONMENT.contains(&code)
            && code != "launcher-interrupted"
        {
            return Err(SelectorError::corruption("failure-attribution"));
        }
        let _lock = self.acquire_transaction_lock()?;
        let mut observation = self
            .read_observation(version)?
            .ok_or_else(|| SelectorError::invalid_state("no-observation"))?;
        self.ensure_observation_selection_current_unlocked(&observation)?;
        if observation.launch_id != launch_id {
            return Err(SelectorError::invalid_state("stale-launch"));
        }
        if matches!(observation.state, ObservationState::Failed) {
            return Ok(FailureDisposition::AlreadyRecorded);
        }
        observation.last_code = code.to_owned();
        observation.state = ObservationState::Failed;
        let attributable = ATTRIBUTABLE.contains(&code);
        if attributable {
            observation.consecutive_failures += 1;
        }
        self.publish_observation(&observation)?;
        self.emit_selector_log(
            &observation.version,
            code,
            Self::child_correlation(&observation),
            BTreeMap::from([(
                "classification".to_owned(),
                LogScalar::String(if attributable {
                    "candidate".to_owned()
                } else {
                    "environment".to_owned()
                }),
            )]),
        )?;
        if attributable && observation.consecutive_failures >= 3 {
            self.emit_selector_log(
                &observation.version,
                "readiness-crash-loop",
                Self::child_correlation(&observation),
                BTreeMap::from([(
                    "classification".to_owned(),
                    LogScalar::String("candidate".to_owned()),
                )]),
            )?;
        }
        if attributable && observation.rollback_eligible && observation.consecutive_failures >= 3 {
            return Ok(FailureDisposition::AutomaticRollbackRequired);
        }
        if attributable && observation.consecutive_failures >= 3 {
            return Ok(FailureDisposition::CrashLoop);
        }
        Ok(if attributable {
            FailureDisposition::RetryAfterOneSecond
        } else {
            FailureDisposition::EnvironmentRetry
        })
    }

    /// The resident launcher already validated the frozen bundle before spawning this exact
    /// generation. Promotion therefore re-checks only the durable selection identity under the
    /// transaction lock. This preserves fail-closed selection replacement detection without
    /// repeating hashes, code-signature commands and provisioning inspection on the timer path.
    fn promote_validated_observation_if_due(
        &self,
        version: &str,
        now: &str,
    ) -> Result<bool, SelectorError> {
        if !valid_rfc3339_nano(now) {
            return Err(SelectorError::invalid_state("clock-unavailable"));
        }
        let _lock = self.acquire_transaction_lock()?;
        let mut observation = self
            .read_observation(version)?
            .ok_or_else(|| SelectorError::invalid_state("no-observation"))?;
        self.ensure_observation_selection_current_unlocked(&observation)?;
        self.promote_observation_if_due_locked(&mut observation, now)
    }

    fn promote_observation_if_due_locked(
        &self,
        observation: &mut Observation,
        now: &str,
    ) -> Result<bool, SelectorError> {
        if !matches!(observation.state, ObservationState::Ready) || !observation.rollback_eligible {
            return Ok(false);
        }
        let closes = observation
            .window_closes_at
            .as_deref()
            .ok_or_else(|| SelectorError::corruption("selector/observations"))?;
        if now < closes {
            return Ok(false);
        }
        observation.state = ObservationState::Promoted;
        observation.rollback_eligible = false;
        observation.consecutive_failures = 0;
        observation.last_code = "promoted".to_owned();
        observation.window_closes_at = None;
        self.publish_observation(observation)?;
        self.emit_selector_log(
            &observation.version,
            "promotion-complete",
            Self::child_correlation(observation),
            BTreeMap::from([(
                "from_state".to_owned(),
                LogScalar::String("ready".to_owned()),
            )]),
        )?;
        Ok(true)
    }

    pub fn automatic_rollback(
        &self,
        launch_id: &str,
        reason: &str,
        command_sha256: String,
    ) -> Result<(), SelectorError> {
        let _lock = self.acquire_transaction_lock()?;
        self.recover_locked()?;
        let (current, previous) = self.read_selection_set()?;
        let current = current.ok_or_else(|| SelectorError::invalid_state("not-activated"))?;
        let previous = previous
            .and_then(|file| file.selection)
            .ok_or_else(|| SelectorError::invalid_state("no-previous"))?;
        let observation = self
            .read_observation(&current.selection.version)?
            .ok_or_else(|| SelectorError::invalid_state("no-observation"))?;
        if observation.launch_id != launch_id
            || !matches!(observation.state, ObservationState::Failed)
            || !observation.rollback_eligible
            || observation.consecutive_failures < 3
        {
            return Err(SelectorError::invalid_state("automatic-rollback-guard"));
        }
        let generation = current.generation + 1;
        let operation = SelectorOperation {
            actor: OperationActor::Serve,
            command_sha256,
            format: 1,
            from: Some(current.selection),
            generation,
            launch_id: Some(launch_id.to_owned()),
            op_id: format!("rollback-auto-{generation}"),
            operation_type: OperationType::Rollback,
            phase: "prepared".to_owned(),
            reason: Some(reason.to_owned()),
            response: None,
            to: previous,
        };
        atomic_json(&self.paths.operation, &operation)?;
        let log_observation = observation;
        let log_operation_id = operation.op_id.clone();
        let from = operation
            .from
            .as_ref()
            .expect("automatic rollback has from")
            .version
            .clone();
        let to = operation.to.version.clone();
        self.finish_selection_operation(operation)?;
        self.emit_selector_log(
            &from,
            "rollback-complete",
            LogCorrelation {
                operation_id: Some(log_operation_id),
                ..Self::child_correlation(&log_observation)
            },
            BTreeMap::from([
                ("from".to_owned(), LogScalar::String(from.clone())),
                ("reason".to_owned(), LogScalar::String(reason.to_owned())),
                ("to".to_owned(), LogScalar::String(to)),
            ]),
        )?;
        Ok(())
    }

    fn validate_bundle(
        &self,
        root: &Path,
        expected_version: Option<&str>,
    ) -> Result<(BundleManifest, Selection), SelectorError> {
        if fs::symlink_metadata(root).map_or(true, |metadata| {
            !metadata.is_dir() || metadata.file_type().is_symlink()
        }) {
            return Err(SelectorError::invalid_bundle(root.display().to_string()));
        }
        let manifest_path = root.join("manifest.canonical.json");
        let manifest_bytes = read_regular(&manifest_path)
            .map_err(|_| SelectorError::invalid_bundle(root.display().to_string()))?;
        let manifest: BundleManifest = read_canonical(&manifest_path)
            .map_err(|_| SelectorError::invalid_bundle(root.display().to_string()))?;
        let expected_files = if manifest
            .files
            .iter()
            .map(|file| (file.path.as_str(), file.mode.as_str()))
            .eq(BUNDLE_FILES)
        {
            &BUNDLE_FILES[..]
        } else if manifest
            .files
            .iter()
            .map(|file| (file.path.as_str(), file.mode.as_str()))
            .eq(LEGACY_BUNDLE_FILES)
        {
            &LEGACY_BUNDLE_FILES[..]
        } else {
            return Err(SelectorError::invalid_bundle(root.display().to_string()));
        };
        if manifest.format != 1
            || manifest.minimum_os != "15.0"
            || manifest.architectures != ["aarch64"]
            || manifest.compatibility.reader_profile != "v1"
            || manifest.compatibility.writer_profile != "v1"
            || manifest.compatibility.authority_registry_sha256
                != EMBEDDED_AUTHORITY_REGISTRY_SHA256
            || expected_version.is_some_and(|version| version != manifest.version)
        {
            return Err(SelectorError::invalid_bundle(root.display().to_string()));
        }
        validate_bundle_entries(root, expected_files.len() == BUNDLE_FILES.len())?;
        validate_id(&manifest.version)
            .map_err(|_| SelectorError::invalid_bundle(root.display().to_string()))?;
        let identity: InstallIdentity = read_canonical(&self.paths.install_identity)
            .map_err(|_| SelectorError::invalid_bundle(root.display().to_string()))?;
        if !valid_install_identity(&identity)
            || manifest.signing.team_id != identity.team_id
            || manifest.signing.requirement != identity.supervisor_requirement
        {
            return Err(SelectorError::invalid_bundle(root.display().to_string()));
        }
        for (file, (expected_path, expected_mode)) in
            manifest.files.iter().zip(expected_files.iter().copied())
        {
            if file.path != expected_path
                || file.mode != expected_mode
                || !validate_hex(&file.sha256)
            {
                return Err(SelectorError::invalid_bundle(root.display().to_string()));
            }
            let artifact = root.join(&file.path);
            let bytes = read_regular(&artifact)
                .map_err(|_| SelectorError::invalid_bundle(root.display().to_string()))?;
            let expected_mode = if expected_mode == "0755" {
                0o755
            } else {
                0o644
            };
            if bytes.len() as u64 != file.bytes
                || sha256(&bytes) != file.sha256
                || mode(&artifact)? != expected_mode
            {
                return Err(SelectorError::invalid_bundle(root.display().to_string()));
            }
            if expected_mode == 0o755 {
                let requirement = match file.path.as_str() {
                    "apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor" => {
                        identity.supervisor_requirement.as_str()
                    }
                    "bin/tekes-worker" => {
                        "anchor apple generic and identifier com.tekes.kernel.worker"
                    }
                    "bin/tekes-helper" => {
                        "anchor apple generic and identifier com.tekes.kernel.helper"
                    }
                    _ => return Err(SelectorError::invalid_bundle(root.display().to_string())),
                };
                let signature = self.verifier.verify(&artifact, requirement)?;
                if signature.team_id != manifest.signing.team_id
                    || !signature.architectures.iter().any(|arch| arch == "aarch64")
                {
                    return Err(SelectorError::invalid_bundle(root.display().to_string()));
                }
            }
        }
        self.verifier.verify_provisioned_app(
            &root.join("apps/TekesKernelSupervisor.app"),
            &identity.supervisor_requirement,
            &identity.team_id,
            "com.tekes.kernel.supervisor",
            &[
                identity.access_group.clone(),
                format!("{}.com.tekes.kernel.provider-secrets", identity.team_id),
            ],
        )?;
        let selection = Selection {
            manifest_sha256: sha256(&manifest_bytes),
            version: manifest.version.clone(),
        };
        Ok((manifest, selection))
    }

    fn validate_selector_manifest(
        &self,
        manifest: &SelectorManifest,
        artifact: &Path,
        identity: &InstallIdentity,
    ) -> Result<(), SelectorError> {
        let bytes = read_regular(artifact)
            .map_err(|_| SelectorError::invalid_bundle(artifact.display().to_string()))?;
        let signature = self
            .verifier
            .verify(artifact, &identity.selector_requirement)?;
        if manifest.format != 1
            || !valid_install_identity(identity)
            || manifest.minimum_os != "15.0"
            || manifest.architecture != "aarch64"
            || validate_id(&manifest.version).is_err()
            || manifest.file.path != "selector/bin/tekes-selector"
            || manifest.file.mode != "0755"
            || mode(artifact).map_or(true, |actual| actual != 0o755)
            || manifest.file.bytes != bytes.len() as u64
            || manifest.file.sha256 != sha256(&bytes)
            || !validate_hex(&manifest.conformance_sha256)
            || manifest.signing.team_id != identity.team_id
            || manifest.signing.requirement != identity.selector_requirement
            || signature.team_id != identity.team_id
            || !signature.architectures.iter().any(|arch| arch == "aarch64")
        {
            return Err(SelectorError::invalid_bundle(
                artifact.display().to_string(),
            ));
        }
        let described = query_candidate_conformance(artifact)?;
        if described.format != 1
            || described.operation != "describe-conformance"
            || described.architecture != manifest.architecture
            || described.version != manifest.version
            || described.conformance_sha256 != manifest.conformance_sha256
        {
            return Err(SelectorError::invalid_bundle(
                artifact.display().to_string(),
            ));
        }
        Ok(())
    }

    fn copy_bundle(
        &self,
        source: &Path,
        staging: &Path,
        manifest: &BundleManifest,
    ) -> Result<(), SelectorError> {
        create_private_dir(staging)?;
        create_private_dir(&staging.join("apps"))?;
        create_private_dir(&staging.join("apps/TekesKernelSupervisor.app"))?;
        create_private_dir(&staging.join("apps/TekesKernelSupervisor.app/Contents"))?;
        create_private_dir(&staging.join("apps/TekesKernelSupervisor.app/Contents/MacOS"))?;
        let has_web_client = manifest.files.iter().any(|file| {
            file.path
                == "apps/TekesKernelSupervisor.app/Contents/Resources/WebClientManifest.canonical.json"
        });
        if has_web_client {
            create_private_dir(&staging.join("apps/TekesKernelSupervisor.app/Contents/Resources"))?;
        }
        create_private_dir(
            &staging.join("apps/TekesKernelSupervisor.app/Contents/_CodeSignature"),
        )?;
        create_private_dir(&staging.join("bin"))?;
        copy_regular(
            &source.join("manifest.canonical.json"),
            &staging.join("manifest.canonical.json"),
            0o600,
        )?;
        for file in &manifest.files {
            let file_mode = if file.mode == "0755" { 0o755 } else { 0o644 };
            copy_regular(
                &source.join(&file.path),
                &staging.join(&file.path),
                file_mode,
            )?;
        }
        sync_directory(&staging.join("apps/TekesKernelSupervisor.app/Contents/MacOS"))?;
        if has_web_client {
            sync_directory(&staging.join("apps/TekesKernelSupervisor.app/Contents/Resources"))?;
        }
        sync_directory(&staging.join("apps/TekesKernelSupervisor.app/Contents/_CodeSignature"))?;
        sync_directory(&staging.join("apps/TekesKernelSupervisor.app/Contents"))?;
        sync_directory(&staging.join("apps/TekesKernelSupervisor.app"))?;
        sync_directory(&staging.join("apps"))?;
        sync_directory(&staging.join("bin"))?;
        sync_directory(staging)
    }

    fn finish_selection_operation(
        &self,
        mut operation: SelectorOperation,
    ) -> Result<MutationReply, SelectorError> {
        let target = relative_active_target(&operation.to.version);
        atomic_symlink(&self.paths.active, &target)?;
        operation.phase = "link-published".to_owned();
        atomic_json(&self.paths.operation, &operation)?;
        let previous = PreviousFile {
            format: 1,
            generation: operation.generation,
            selection: operation.from.clone(),
        };
        atomic_json(&self.paths.previous, &previous)?;
        operation.phase = "previous-published".to_owned();
        atomic_json(&self.paths.operation, &operation)?;
        let current = SelectionFile {
            format: 1,
            generation: operation.generation,
            selection: operation.to.clone(),
        };
        atomic_json(&self.paths.current, &current)?;
        operation.phase = "current-published".to_owned();
        atomic_json(&self.paths.operation, &operation)?;
        let operation_name = match operation.operation_type {
            OperationType::Activate => "activate",
            OperationType::Rollback => "rollback",
            OperationType::Stage => {
                return Err(SelectorError::corruption(
                    "selector/operations/current.json",
                ));
            }
        };
        let reply = MutationReply {
            current,
            format: 1,
            operation: operation_name.to_owned(),
            previous,
        };
        operation.phase = "closed".to_owned();
        operation.response = matches!(operation.actor, OperationActor::Cli)
            .then(|| to_value(&reply))
            .transpose()?;
        atomic_json(&self.paths.operation, &operation)?;
        Ok(reply)
    }

    fn recover_locked(&self) -> Result<bool, SelectorError> {
        self.validate_operations_directory()?;
        let mut operation: SelectorOperation = match read_canonical(&self.paths.operation) {
            Ok(operation) => operation,
            Err(error) if error.code == crate::ErrorCode::Io && !self.paths.operation.exists() => {
                return Ok(false);
            }
            Err(error) => return Err(error),
        };
        self.validate_operation_shape(&operation)?;
        if operation.phase == "closed" {
            self.validate_closed_operation(&operation)?;
            return Ok(false);
        }
        match operation.operation_type {
            OperationType::Stage => {
                let selected_generation = self
                    .read_selection_set()?
                    .0
                    .map_or(0, |current| current.generation);
                if operation.generation != selected_generation {
                    return Err(SelectorError::corruption(
                        self.paths.operation.display().to_string(),
                    ));
                }
                let final_path = self.paths.bundles.join(&operation.to.version);
                let staging = self
                    .paths
                    .bundles
                    .join(format!(".{}.staging", operation.op_id));
                if operation.phase == "prepared" && !final_path.exists() {
                    if staging.exists()
                        && self
                            .validate_bundle(&staging, Some(&operation.to.version))
                            .map(|(_, selection)| selection != operation.to)
                            .unwrap_or(true)
                    {
                        remove_dir_if_exists(&staging)?;
                    }
                    if !staging.exists() {
                        fs::remove_file(&self.paths.operation).selector_io("operation-abort")?;
                        sync_directory(&self.paths.operations)?;
                        return Ok(true);
                    }
                }
                if matches!(operation.phase.as_str(), "bundle-published") && !final_path.exists() {
                    return Err(SelectorError::corruption(
                        self.paths.operation.display().to_string(),
                    ));
                }
                if !final_path.exists() {
                    self.validate_bundle(&staging, Some(&operation.to.version))?;
                    if operation.phase == "copied" {
                        operation.phase = "verified".to_owned();
                        atomic_json(&self.paths.operation, &operation)?;
                    }
                    fs::rename(&staging, &final_path).selector_io("bundle-rename")?;
                    sync_directory(&self.paths.bundles)?;
                }
                let (_, selection) =
                    self.validate_bundle(&final_path, Some(&operation.to.version))?;
                if selection != operation.to {
                    return Err(SelectorError::corruption(
                        self.paths.operation.display().to_string(),
                    ));
                }
                remove_dir_if_exists(&staging)?;
                operation.phase = "bundle-published".to_owned();
                atomic_json(&self.paths.operation, &operation)?;
                let reply = StageReply {
                    format: 1,
                    operation: "stage".to_owned(),
                    selection,
                };
                operation.phase = "closed".to_owned();
                operation.response = Some(to_value(&reply)?);
                atomic_json(&self.paths.operation, &operation)?;
            }
            OperationType::Activate | OperationType::Rollback => {
                self.validate_selected_bundle(&operation.to)?;
                if let Some(from) = &operation.from {
                    self.validate_selected_bundle(from)?;
                }
                let active = read_active(&self.paths.active)?;
                let from_target = operation
                    .from
                    .as_ref()
                    .map(|selection| relative_active_target(&selection.version));
                let to_target = relative_active_target(&operation.to.version);
                let active_is_to = active.as_ref() == Some(&to_target);
                let active_is_from = match (&active, &from_target) {
                    (Some(target), Some(from)) => target == from,
                    (None, None) => true,
                    _ => false,
                };
                match (active_is_to, active_is_from) {
                    (true, _) => {}
                    (false, true) if operation.phase == "prepared" => {
                        self.validate_predecision_selection_files(&operation)?;
                        atomic_symlink(&self.paths.active, &to_target)?;
                    }
                    _ => {
                        return Err(SelectorError::corruption(
                            self.paths.active.display().to_string(),
                        ));
                    }
                }
                self.validate_postdecision_selection_files(&operation)?;
                operation.phase = "link-published".to_owned();
                atomic_json(&self.paths.operation, &operation)?;
                // Rebuild the two derived files from the durable decision.
                let previous = PreviousFile {
                    format: 1,
                    generation: operation.generation,
                    selection: operation.from.clone(),
                };
                atomic_json(&self.paths.previous, &previous)?;
                operation.phase = "previous-published".to_owned();
                atomic_json(&self.paths.operation, &operation)?;
                let current = SelectionFile {
                    format: 1,
                    generation: operation.generation,
                    selection: operation.to.clone(),
                };
                atomic_json(&self.paths.current, &current)?;
                operation.phase = "current-published".to_owned();
                atomic_json(&self.paths.operation, &operation)?;
                let name = if matches!(operation.operation_type, OperationType::Activate) {
                    "activate"
                } else {
                    "rollback"
                };
                let reply = MutationReply {
                    current,
                    format: 1,
                    operation: name.to_owned(),
                    previous,
                };
                operation.phase = "closed".to_owned();
                operation.response = matches!(operation.actor, OperationActor::Cli)
                    .then(|| to_value(&reply))
                    .transpose()?;
                atomic_json(&self.paths.operation, &operation)?;
            }
        }
        Ok(true)
    }

    fn validate_operations_directory(&self) -> Result<(), SelectorError> {
        for entry in fs::read_dir(&self.paths.operations).selector_io("operations-read")? {
            let entry = entry.selector_io("operations-entry")?;
            if entry.file_name() != "current.json" {
                return Err(SelectorError::corruption(
                    self.paths.operations.display().to_string(),
                ));
            }
            if !entry
                .file_type()
                .selector_io("operations-entry-type")?
                .is_file()
            {
                return Err(SelectorError::corruption(
                    self.paths.operation.display().to_string(),
                ));
            }
        }
        Ok(())
    }

    fn validate_predecision_selection_files(
        &self,
        operation: &SelectorOperation,
    ) -> Result<(), SelectorError> {
        let current: Option<SelectionFile> = optional_canonical(&self.paths.current)?;
        let previous: Option<PreviousFile> = optional_canonical(&self.paths.previous)?;
        match &operation.from {
            None if current.is_none() && previous.is_none() => Ok(()),
            Some(from)
                if current.as_ref().is_some_and(|file| {
                    file.generation.checked_add(1) == Some(operation.generation)
                        && file.selection == *from
                }) && previous.as_ref().is_some_and(|file| {
                    file.generation.checked_add(1) == Some(operation.generation)
                }) =>
            {
                Ok(())
            }
            _ => Err(SelectorError::corruption("selector/current.json")),
        }
    }

    fn validate_postdecision_selection_files(
        &self,
        operation: &SelectorOperation,
    ) -> Result<(), SelectorError> {
        let current: Option<SelectionFile> = optional_canonical(&self.paths.current)?;
        let previous: Option<PreviousFile> = optional_canonical(&self.paths.previous)?;
        let current_valid = match (&current, &operation.from) {
            (None, None) => true,
            (Some(file), _) if file.generation == operation.generation => {
                file.selection == operation.to
            }
            (Some(file), Some(from)) => {
                file.generation.checked_add(1) == Some(operation.generation)
                    && file.selection == *from
            }
            _ => false,
        };
        let previous_valid = match &previous {
            None => operation.from.is_none(),
            Some(file) if file.generation == operation.generation => {
                file.selection == operation.from
            }
            Some(file) => file.generation.checked_add(1) == Some(operation.generation),
        };
        if current_valid && previous_valid {
            Ok(())
        } else {
            Err(SelectorError::corruption("selector/current.json"))
        }
    }

    fn validate_operation_shape(&self, operation: &SelectorOperation) -> Result<(), SelectorError> {
        let valid_phase = match operation.operation_type {
            OperationType::Stage => [
                "prepared",
                "copied",
                "verified",
                "bundle-published",
                "closed",
            ]
            .contains(&operation.phase.as_str()),
            OperationType::Activate | OperationType::Rollback => [
                "prepared",
                "link-published",
                "previous-published",
                "current-published",
                "closed",
            ]
            .contains(&operation.phase.as_str()),
        };
        let actor_fields = match operation.actor {
            OperationActor::Cli => operation.launch_id.is_none(),
            OperationActor::Serve => {
                matches!(operation.operation_type, OperationType::Rollback)
                    && operation.launch_id.is_some()
                    && operation.response.is_none()
            }
        };
        let response_shape = (operation.phase == "closed")
            == (matches!(operation.actor, OperationActor::Cli) && operation.response.is_some())
            || (operation.phase == "closed"
                && matches!(operation.actor, OperationActor::Serve)
                && operation.response.is_none());
        let type_fields = match operation.operation_type {
            OperationType::Stage => {
                matches!(operation.actor, OperationActor::Cli)
                    && operation.from.is_none()
                    && operation.reason.is_none()
                    && operation.launch_id.is_none()
            }
            OperationType::Activate => {
                matches!(operation.actor, OperationActor::Cli)
                    && operation.reason.is_none()
                    && operation.launch_id.is_none()
                    && operation.generation >= 1
            }
            OperationType::Rollback => {
                operation.from.is_some() && operation.reason.is_some() && operation.generation >= 2
            }
        };
        let selections_valid = validate_id(&operation.to.version).is_ok()
            && validate_hex(&operation.to.manifest_sha256)
            && operation.from.as_ref().is_none_or(|selection| {
                validate_id(&selection.version).is_ok() && validate_hex(&selection.manifest_sha256)
            });
        let reason_valid = operation
            .reason
            .as_deref()
            .is_none_or(|reason| validate_id(reason).is_ok());
        let serve_identity_valid = if matches!(operation.actor, OperationActor::Serve) {
            match (
                operation.launch_id.as_deref(),
                operation.reason.as_deref(),
                operation.from.as_ref(),
            ) {
                (Some(launch_id), Some(reason), Some(from)) => automatic_rollback_sha256(
                    operation.generation,
                    launch_id,
                    reason,
                    from,
                    &operation.to,
                )
                .is_ok_and(|digest| digest == operation.command_sha256),
                _ => false,
            }
        } else {
            true
        };
        if operation.format != 1
            || !valid_phase
            || !actor_fields
            || !type_fields
            || !selections_valid
            || !reason_valid
            || !serve_identity_valid
            || !response_shape
            || !validate_hex(&operation.command_sha256)
            || operation.op_id.is_empty()
        {
            return Err(SelectorError::corruption(
                self.paths.operation.display().to_string(),
            ));
        }
        Ok(())
    }

    fn validate_closed_operation(
        &self,
        operation: &SelectorOperation,
    ) -> Result<(), SelectorError> {
        match operation.operation_type {
            OperationType::Stage => {
                let (_, selected) = self.validate_bundle(
                    &self.paths.bundles.join(&operation.to.version),
                    Some(&operation.to.version),
                )?;
                if selected != operation.to {
                    return Err(SelectorError::corruption(
                        self.paths.operation.display().to_string(),
                    ));
                }
                let expected = to_value(&StageReply {
                    format: 1,
                    operation: "stage".to_owned(),
                    selection: selected,
                })?;
                if operation.response.as_ref() != Some(&expected) {
                    return Err(SelectorError::corruption(
                        self.paths.operation.display().to_string(),
                    ));
                }
            }
            OperationType::Activate | OperationType::Rollback => {
                let (current, previous) = self.read_selection_set()?;
                if current
                    .as_ref()
                    .map(|file| (&file.selection, file.generation))
                    != Some((&operation.to, operation.generation))
                    || previous
                        .as_ref()
                        .map(|file| (&file.selection, file.generation))
                        != Some((&operation.from, operation.generation))
                {
                    return Err(SelectorError::corruption(
                        self.paths.operation.display().to_string(),
                    ));
                }
                if matches!(operation.actor, OperationActor::Cli) {
                    let expected = to_value(&MutationReply {
                        current: current.expect("validated current"),
                        format: 1,
                        operation: if matches!(operation.operation_type, OperationType::Activate) {
                            "activate".to_owned()
                        } else {
                            "rollback".to_owned()
                        },
                        previous: previous.expect("validated previous"),
                    })?;
                    if operation.response.as_ref() != Some(&expected) {
                        return Err(SelectorError::corruption(
                            self.paths.operation.display().to_string(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn read_selection_set(
        &self,
    ) -> Result<(Option<SelectionFile>, Option<PreviousFile>), SelectorError> {
        let current: Option<SelectionFile> = optional_canonical(&self.paths.current)?;
        let previous: Option<PreviousFile> = optional_canonical(&self.paths.previous)?;
        let active = read_active(&self.paths.active)?;
        match (&current, &previous, &active) {
            (None, None, None) => Ok((None, None)),
            (Some(current), Some(previous), Some(target))
                if current.format == 1
                    && previous.format == 1
                    && current.generation >= 1
                    && current.generation == previous.generation
                    && target == &relative_active_target(&current.selection.version)
                    && validate_id(&current.selection.version).is_ok()
                    && validate_hex(&current.selection.manifest_sha256)
                    && previous.selection.as_ref().is_none_or(|selection| {
                        validate_id(&selection.version).is_ok()
                            && validate_hex(&selection.manifest_sha256)
                    }) =>
            {
                self.validate_selected_bundle(&current.selection)?;
                if let Some(selection) = &previous.selection {
                    self.validate_selected_bundle(selection)?;
                }
                Ok((Some(current.clone()), Some(previous.clone())))
            }
            _ => Err(SelectorError::corruption("selector/current.json")),
        }
    }

    fn read_observation(&self, version: &str) -> Result<Option<Observation>, SelectorError> {
        let observation: Option<Observation> =
            optional_canonical(&self.paths.observations.join(format!("{version}.json")))?;
        if let Some(observation) = &observation {
            self.validate_observation(observation)?;
            if observation.version != version {
                return Err(SelectorError::corruption("selector/observations"));
            }
        }
        Ok(observation)
    }

    fn ensure_current_selection_unlocked(
        &self,
        expected: &SelectionFile,
    ) -> Result<(), SelectorError> {
        let current: SelectionFile = read_canonical(&self.paths.current)
            .map_err(|_| SelectorError::corruption(self.paths.current.display().to_string()))?;
        let active = read_active(&self.paths.active)?;
        if current != *expected
            || active != Some(relative_active_target(&expected.selection.version))
        {
            return Err(SelectorError::invalid_state(
                "observation-selection-mismatch",
            ));
        }
        Ok(())
    }

    fn ensure_observation_selection_current_unlocked(
        &self,
        observation: &Observation,
    ) -> Result<(), SelectorError> {
        self.ensure_current_selection_unlocked(&SelectionFile {
            format: 1,
            generation: observation.generation,
            selection: Selection {
                manifest_sha256: observation.manifest_sha256.clone(),
                version: observation.version.clone(),
            },
        })
    }

    fn validate_selected_bundle(&self, selection: &Selection) -> Result<(), SelectorError> {
        let (_, actual) = self
            .validate_bundle(
                &self.paths.bundles.join(&selection.version),
                Some(&selection.version),
            )
            .map_err(|_| SelectorError::corruption("selector/current.json"))?;
        if actual != *selection {
            return Err(SelectorError::corruption("selector/current.json"));
        }
        Ok(())
    }

    fn read_observation_unlocked(
        &self,
        version: &str,
    ) -> Result<Option<Observation>, SelectorError> {
        let _lock = self.acquire_transaction_lock()?;
        let observation = self.read_observation(version)?;
        if let Some(value) = &observation {
            self.ensure_observation_selection_current_unlocked(value)?;
        }
        Ok(observation)
    }

    fn installer_recovery_required(&self) -> Result<bool, SelectorError> {
        if !self.paths.installer_operation.exists() {
            return Ok(false);
        }
        let operation: crate::InstallerOperation = read_canonical(&self.paths.installer_operation)?;
        Ok(operation.phase != "closed")
    }

    fn publish_observation(&self, observation: &Observation) -> Result<(), SelectorError> {
        self.validate_observation(observation)?;
        atomic_json(
            &self
                .paths
                .observations
                .join(format!("{}.json", observation.version)),
            observation,
        )
    }

    fn validate_observation(&self, observation: &Observation) -> Result<(), SelectorError> {
        let state_fields_valid = match observation.state {
            ObservationState::Ready => {
                observation.rollback_eligible && observation.window_closes_at.is_some()
            }
            ObservationState::Promoted => {
                !observation.rollback_eligible && observation.window_closes_at.is_none()
            }
            ObservationState::Pending | ObservationState::Failed => true,
        };
        if observation.format != 1
            || observation.attempt == 0
            || observation.generation == 0
            || validate_id(&observation.version).is_err()
            || validate_id(&observation.last_code).is_err()
            || observation.canary_session.is_some() != observation.canary_run.is_some()
            || !validate_hex(&observation.manifest_sha256)
            || !valid_launch_id(
                &observation.launch_id,
                observation.generation,
                observation.attempt,
            )
            || !valid_rfc3339_nano(&observation.started_at)
            || !valid_rfc3339_nano(&observation.deadline_at)
            || observation
                .canary_deadline_at
                .as_deref()
                .is_some_and(|value| !valid_rfc3339_nano(value))
            || observation
                .window_closes_at
                .as_deref()
                .is_some_and(|value| !valid_rfc3339_nano(value))
            || !state_fields_valid
        {
            return Err(SelectorError::corruption("selector/observations"));
        }
        Ok(())
    }

    fn retry_reply<T: serde::de::DeserializeOwned>(
        &self,
        command_sha256: &str,
    ) -> Result<Option<T>, SelectorError> {
        let operation: Option<SelectorOperation> = optional_canonical(&self.paths.operation)?;
        if let Some(operation) = operation {
            if operation.phase == "closed" && operation.command_sha256 == command_sha256 {
                let response = operation.response.ok_or_else(|| {
                    SelectorError::corruption(self.paths.operation.display().to_string())
                })?;
                return serde_json::from_value(response).map(Some).map_err(|_| {
                    SelectorError::corruption(self.paths.operation.display().to_string())
                });
            }
            if operation.phase != "closed" {
                return Err(SelectorError::invalid_state("operation-in-progress"));
            }
        }
        Ok(None)
    }

    fn close_no_effect_activate(
        &self,
        to: Selection,
        command_sha256: String,
        current: SelectionFile,
    ) -> Result<MutationReply, SelectorError> {
        let previous: PreviousFile = read_canonical(&self.paths.previous)?;
        let reply = MutationReply {
            current,
            format: 1,
            operation: "activate".to_owned(),
            previous: previous.clone(),
        };
        let operation = SelectorOperation {
            actor: OperationActor::Cli,
            command_sha256,
            format: 1,
            from: previous.selection.clone(),
            generation: reply.current.generation,
            launch_id: None,
            op_id: format!("activate-noop-{}", reply.current.generation),
            operation_type: OperationType::Activate,
            phase: "closed".to_owned(),
            reason: None,
            response: Some(to_value(&reply)?),
            to,
        };
        atomic_json(&self.paths.operation, &operation)?;
        Ok(reply)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureDisposition {
    AlreadyRecorded,
    RetryAfterOneSecond,
    EnvironmentRetry,
    AutomaticRollbackRequired,
    CrashLoop,
}

fn optional_canonical<T: serde::de::DeserializeOwned + Serialize>(
    path: &Path,
) -> Result<Option<T>, SelectorError> {
    match read_canonical(path) {
        Ok(value) => Ok(Some(value)),
        Err(error) if error.code == crate::ErrorCode::Io && !path.exists() => Ok(None),
        Err(error) => Err(error),
    }
}

fn exact_directory_entries(path: &Path, expected: &[&str]) -> Result<(), SelectorError> {
    let metadata = fs::symlink_metadata(path).selector_io("bundle-directory-stat")?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(SelectorError::invalid_bundle(path.display().to_string()));
    }
    let mut entries = fs::read_dir(path)
        .selector_io("bundle-read-directory")?
        .map(|entry| {
            entry
                .selector_io("bundle-read-entry")?
                .file_name()
                .into_string()
                .map_err(|_| SelectorError::invalid_bundle(path.display().to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort();
    if entries != expected {
        return Err(SelectorError::invalid_bundle(path.display().to_string()));
    }
    Ok(())
}

fn validate_bundle_entries(root: &Path, has_web_client: bool) -> Result<(), SelectorError> {
    exact_directory_entries(root, &["apps", "bin", "manifest.canonical.json"])?;
    exact_directory_entries(&root.join("apps"), &["TekesKernelSupervisor.app"])?;
    exact_directory_entries(&root.join("apps/TekesKernelSupervisor.app"), &["Contents"])?;
    if has_web_client {
        exact_directory_entries(
            &root.join("apps/TekesKernelSupervisor.app/Contents"),
            &[
                "Info.plist",
                "MacOS",
                "Resources",
                "_CodeSignature",
                "embedded.provisionprofile",
            ],
        )?;
    } else {
        exact_directory_entries(
            &root.join("apps/TekesKernelSupervisor.app/Contents"),
            &[
                "Info.plist",
                "MacOS",
                "_CodeSignature",
                "embedded.provisionprofile",
            ],
        )?;
    }
    exact_directory_entries(
        &root.join("apps/TekesKernelSupervisor.app/Contents/MacOS"),
        &["tekes-supervisor"],
    )?;
    if has_web_client {
        exact_directory_entries(
            &root.join("apps/TekesKernelSupervisor.app/Contents/Resources"),
            &["WebClientManifest.canonical.json"],
        )?;
    }
    exact_directory_entries(
        &root.join("apps/TekesKernelSupervisor.app/Contents/_CodeSignature"),
        &["CodeResources"],
    )?;
    exact_directory_entries(&root.join("bin"), &["tekes-helper", "tekes-worker"])?;
    Ok(())
}

fn to_value<T: Serialize>(value: &T) -> Result<Value, SelectorError> {
    serde_json::to_value(value).map_err(|_| SelectorError::corruption("response"))
}

pub fn cli_command_sha256(argv: &[String]) -> Result<String, SelectorError> {
    Ok(sha256(
        &serde_json_canonicalizer::to_vec(&json!({"actor":"cli","argv":argv}))
            .map_err(|_| SelectorError::usage("argv"))?,
    ))
}

pub fn automatic_rollback_sha256(
    generation: u64,
    launch_id: &str,
    reason: &str,
    from: &Selection,
    to: &Selection,
) -> Result<String, SelectorError> {
    Ok(sha256(
        &serde_json_canonicalizer::to_vec(&json!({
            "actor":"serve",
            "action":"automatic-rollback",
            "generation":generation,
            "launch_id":launch_id,
            "reason":reason,
            "from":from,
            "to":to
        }))
        .map_err(|_| SelectorError::corruption("automatic-rollback"))?,
    ))
}

pub fn reply_bytes<T: Serialize>(reply: &T) -> Result<Vec<u8>, SelectorError> {
    canonical_line(reply)
}

pub fn describe_conformance() -> Result<ConformanceReply, SelectorError> {
    let digest = EMBEDDED_SELECTOR_CONFORMANCE_SHA256
        .filter(|value| validate_hex(value))
        .ok_or_else(|| SelectorError::invalid_state("conformance-evidence-missing"))?;
    Ok(ConformanceReply {
        architecture: "aarch64".to_owned(),
        conformance_sha256: digest.to_owned(),
        format: 1,
        operation: "describe-conformance".to_owned(),
        version: embedded_selector_version().to_owned(),
    })
}

fn query_candidate_conformance(artifact: &Path) -> Result<ConformanceReply, SelectorError> {
    let mut child = Command::new(artifact)
        .arg("describe-conformance")
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| SelectorError::invalid_bundle(artifact.display().to_string()))?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or_else(|| SelectorError::invalid_bundle(artifact.display().to_string()))?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or_else(|| SelectorError::invalid_bundle(artifact.display().to_string()))?;
    if set_nonblocking(stdout.as_raw_fd()).is_err() || set_nonblocking(stderr.as_raw_fd()).is_err()
    {
        let _ = child.kill();
        let _ = child.wait();
        return Err(SelectorError::invalid_bundle(
            artifact.display().to_string(),
        ));
    }
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut bytes = Vec::new();
    let mut stderr_bytes = Vec::new();
    let status = loop {
        read_bounded_nonblocking(&mut stdout, &mut bytes, 4096, artifact, &mut child)?;
        read_bounded_nonblocking(&mut stderr, &mut stderr_bytes, 4096, artifact, &mut child)?;
        if let Some(status) = child
            .try_wait()
            .map_err(|_| SelectorError::invalid_bundle(artifact.display().to_string()))?
        {
            // Drain the finite remainder after child exit.
            set_blocking(stdout.as_raw_fd());
            set_blocking(stderr.as_raw_fd());
            stdout
                .read_to_end(&mut bytes)
                .map_err(|_| SelectorError::invalid_bundle(artifact.display().to_string()))?;
            stderr
                .read_to_end(&mut stderr_bytes)
                .map_err(|_| SelectorError::invalid_bundle(artifact.display().to_string()))?;
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(SelectorError::invalid_bundle(
                artifact.display().to_string(),
            ));
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    if !status.success() || bytes.len() > 4096 || !stderr_bytes.is_empty() {
        return Err(SelectorError::invalid_bundle(
            artifact.display().to_string(),
        ));
    }
    let body = bytes
        .strip_suffix(b"\n")
        .filter(|body| !body.ends_with(b"\n"))
        .ok_or_else(|| SelectorError::invalid_bundle(artifact.display().to_string()))?;
    let reply: ConformanceReply = serde_json::from_slice(body)
        .map_err(|_| SelectorError::invalid_bundle(artifact.display().to_string()))?;
    if serde_json_canonicalizer::to_vec(&reply).map_or(true, |canonical| canonical != body) {
        return Err(SelectorError::invalid_bundle(
            artifact.display().to_string(),
        ));
    }
    Ok(reply)
}

fn set_nonblocking(descriptor: libc::c_int) -> Result<(), ()> {
    let flags = unsafe { libc::fcntl(descriptor, libc::F_GETFL) };
    if flags < 0 || unsafe { libc::fcntl(descriptor, libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0
    {
        return Err(());
    }
    Ok(())
}

fn set_blocking(descriptor: libc::c_int) {
    let flags = unsafe { libc::fcntl(descriptor, libc::F_GETFL) };
    if flags >= 0 {
        let _ = unsafe { libc::fcntl(descriptor, libc::F_SETFL, flags & !libc::O_NONBLOCK) };
    }
}

fn read_bounded_nonblocking<R: Read>(
    reader: &mut R,
    bytes: &mut Vec<u8>,
    limit: usize,
    artifact: &Path,
    child: &mut std::process::Child,
) -> Result<(), SelectorError> {
    let mut buffer = [0_u8; 512];
    match reader.read(&mut buffer) {
        Ok(0) => {}
        Ok(count) => {
            bytes.extend_from_slice(&buffer[..count]);
            if bytes.len() > limit {
                let _ = child.kill();
                let _ = child.wait();
                return Err(SelectorError::invalid_bundle(
                    artifact.display().to_string(),
                ));
            }
        }
        Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
        Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
        Err(_) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(SelectorError::invalid_bundle(
                artifact.display().to_string(),
            ));
        }
    }
    Ok(())
}

fn tree_fingerprint(root: &Path) -> Result<String, SelectorError> {
    if !root.exists() {
        return Ok(sha256(b"missing"));
    }
    let metadata = fs::symlink_metadata(root).selector_io("retry-config-stat")?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(SelectorError::corruption(root.display().to_string()));
    }
    let mut entries = Vec::new();
    collect_tree_fingerprint(root, root, &mut entries)?;
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    let mut bytes = Vec::new();
    for (path, kind, mode, content) in entries {
        bytes.extend_from_slice(&(path.len() as u64).to_be_bytes());
        bytes.extend_from_slice(&path);
        bytes.push(kind);
        bytes.extend_from_slice(&mode.to_be_bytes());
        bytes.extend_from_slice(&(content.len() as u64).to_be_bytes());
        bytes.extend_from_slice(&content);
    }
    Ok(sha256(&bytes))
}

fn collect_tree_fingerprint(
    root: &Path,
    directory: &Path,
    entries: &mut Vec<(Vec<u8>, u8, u32, Vec<u8>)>,
) -> Result<(), SelectorError> {
    use std::os::unix::ffi::OsStrExt;

    for entry in fs::read_dir(directory).selector_io("retry-config-read")? {
        let entry = entry.selector_io("retry-config-entry")?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).selector_io("retry-config-entry-stat")?;
        let relative = path
            .strip_prefix(root)
            .map_err(|_| SelectorError::corruption(root.display().to_string()))?
            .as_os_str()
            .as_bytes()
            .to_vec();
        if metadata.file_type().is_symlink() {
            return Err(SelectorError::corruption(path.display().to_string()));
        }
        if metadata.is_dir() {
            entries.push((
                relative,
                b'd',
                metadata.permissions().mode() & 0o7777,
                Vec::new(),
            ));
            collect_tree_fingerprint(root, &path, entries)?;
        } else if metadata.is_file() {
            entries.push((
                relative,
                b'f',
                metadata.permissions().mode() & 0o7777,
                read_regular(&path)?,
            ));
        } else {
            return Err(SelectorError::corruption(path.display().to_string()));
        }
    }
    Ok(())
}

fn append_rotating_log(path: &Path, line: &[u8]) -> Result<(), SelectorError> {
    append_rotating_log_with_sync(path, line, full_sync)
}

fn append_rotating_log_with_sync<F>(
    path: &Path,
    line: &[u8],
    sync_file: F,
) -> Result<(), SelectorError>
where
    F: FnOnce(&File) -> Result<(), SelectorError>,
{
    const LIMIT: u64 = 10 * 1024 * 1024;
    let parent = path
        .parent()
        .ok_or_else(|| SelectorError::corruption(path.display().to_string()))?;
    if line.len() as u64 > LIMIT {
        return Err(SelectorError::corruption("selector-log-record-size"));
    }
    let existing = fs::metadata(path).map_or(0, |metadata| metadata.len());
    if existing.saturating_add(line.len() as u64) > LIMIT {
        let base = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| SelectorError::corruption(path.display().to_string()))?;
        let oldest = parent.join(format!("{base}.5"));
        if oldest.exists() {
            fs::remove_file(&oldest).selector_io("selector-log-remove-oldest")?;
        }
        for generation in (1..5).rev() {
            let from = parent.join(format!("{base}.{generation}"));
            if from.exists() {
                fs::rename(&from, parent.join(format!("{base}.{}", generation + 1)))
                    .selector_io("selector-log-rotate")?;
            }
        }
        if path.exists() {
            fs::rename(path, parent.join(format!("{base}.1")))
                .selector_io("selector-log-rotate")?;
        }
        sync_directory(parent)?;
    }
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .mode(0o600)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
        .selector_io("selector-log-open")?;
    if file
        .metadata()
        .selector_io("selector-log-stat")?
        .permissions()
        .mode()
        & 0o7777
        != 0o600
    {
        return Err(SelectorError::corruption(path.display().to_string()));
    }
    file.write_all(line).selector_io("selector-log-write")?;
    sync_file(&file)?;
    drop(file);
    sync_directory(parent)
}

fn read_rotating_logs(path: &Path) -> Result<Vec<SelectorLogRecord>, SelectorError> {
    let parent = path
        .parent()
        .ok_or_else(|| SelectorError::corruption(path.display().to_string()))?;
    let base = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| SelectorError::corruption(path.display().to_string()))?;
    let mut files = (1..=5)
        .rev()
        .map(|generation| parent.join(format!("{base}.{generation}")))
        .collect::<Vec<_>>();
    files.push(path.to_path_buf());
    let mut records = Vec::new();
    for file in files {
        if !file.exists() {
            continue;
        }
        let bytes = read_regular(&file)?;
        if !bytes.ends_with(b"\n") {
            return Err(SelectorError::corruption(file.display().to_string()));
        }
        for line in bytes
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
        {
            let record: SelectorLogRecord = serde_json::from_slice(line)
                .map_err(|_| SelectorError::corruption(file.display().to_string()))?;
            if serde_json_canonicalizer::to_vec(&record).map_or(true, |canonical| canonical != line)
                || !valid_selector_log_record(&record)
            {
                return Err(SelectorError::corruption(file.display().to_string()));
            }
            records.push(record);
        }
    }
    Ok(records)
}

fn valid_selector_log_record(record: &SelectorLogRecord) -> bool {
    let Some((severity, message, allowed, shape)) = selector_log_contract(&record.code) else {
        return false;
    };
    if record.v != 1
        || record.component != "selector"
        || record.severity != severity
        || record.message != message
        || validate_id(&record.build).is_err()
        || !valid_rfc3339_nano(&record.ts)
        || record.fields.len() != allowed.len()
        || record
            .fields
            .keys()
            .map(String::as_str)
            .ne(allowed.iter().copied())
    {
        return false;
    }
    let child = record
        .correlation
        .attempt
        .zip(record.correlation.generation)
        .zip(record.correlation.launch_id.as_deref())
        .zip(record.correlation.manifest_sha256.as_deref())
        .is_some_and(|(((attempt, generation), launch_id), manifest)| {
            attempt >= 1
                && generation >= 1
                && valid_launch_id(launch_id, generation, attempt)
                && validate_hex(manifest)
        });
    let correlation_valid = match shape {
        LogCorrelationShape::Empty => record.correlation == LogCorrelation::default(),
        LogCorrelationShape::Operation => {
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
        LogCorrelationShape::Child => child && record.correlation.operation_id.is_none(),
        LogCorrelationShape::ChildOperation => {
            child
                && record
                    .correlation
                    .operation_id
                    .as_deref()
                    .is_some_and(|value| !value.is_empty())
        }
    };
    if !correlation_valid {
        return false;
    }
    match record.code.as_str() {
        "selector-predecessor-draining" => {
            record.fields.get("root_lock") == Some(&LogScalar::String("busy".to_owned()))
        }
        "orphan-owner-timeout" => {
            record.fields.get("deadline_ms") == Some(&LogScalar::Integer(30_000))
                && matches!(record.fields.get("generation"), Some(LogScalar::Integer(value)) if *value >= 1)
                && matches!(record.fields.get("manifest_sha256"), Some(LogScalar::String(value)) if validate_hex(value))
                && record.fields.get("root_lock")
                    == Some(&LogScalar::String("busy".to_owned()))
                && matches!(record.fields.get("version"), Some(LogScalar::String(value)) if validate_id(value).is_ok())
        }
        "selector-prelaunch-cleared" => {
            record.fields.get("root_lock") == Some(&LogScalar::String("available".to_owned()))
        }
        "selector-recovered" => {
            matches!(
                record.fields.get("operation"),
                Some(LogScalar::String(value))
                    if matches!(value.as_str(), "stage" | "activate" | "rollback")
            ) && record.fields.get("phase") == Some(&LogScalar::String("closed".to_owned()))
        }
        "selector-update-complete" => {
            matches!(record.fields.get("sha256"), Some(LogScalar::String(value)) if validate_hex(value))
                && matches!(record.fields.get("version"), Some(LogScalar::String(value)) if validate_id(value).is_ok())
        }
        "selector-child-launch" => matches!(
            record.fields.get("canary_required"),
            Some(LogScalar::Bool(_))
        ),
        "promotion-complete" => {
            record.fields.get("from_state") == Some(&LogScalar::String("ready".to_owned()))
        }
        "rollback-complete" => ["from", "reason", "to"].into_iter().all(|key| {
            matches!(record.fields.get(key), Some(LogScalar::String(value)) if validate_id(value).is_ok())
        }),
        failure if ATTRIBUTABLE.contains(&failure) || failure == "readiness-crash-loop" => {
            record.fields.get("classification")
                == Some(&LogScalar::String("candidate".to_owned()))
        }
        failure if ENVIRONMENT.contains(&failure) || failure == "launcher-interrupted" => {
            record.fields.get("classification")
                == Some(&LogScalar::String("environment".to_owned()))
        }
        _ => false,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NativeLaunchOutcome {
    PlannedExit,
    ReadyThenExited,
    Failed(&'static str),
}

struct NativeLaunchSpec<'a> {
    executable: &'a Path,
    install_root: &'a Path,
    storage_root: &'a Path,
    listen: &'a str,
    web_listen: Option<&'a str>,
    selection: &'a SelectionFile,
    authority_registry_sha256: &'a str,
    launch_id: &'a str,
    canary_required: bool,
    canary_deadline_at: Option<&'a str>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct BootstrapStatus {
    format: u8,
    launch_id: String,
    selection: Selection,
    state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

fn launch_and_observe<B, F>(
    spec: &NativeLaunchSpec<'_>,
    mut listener_bound: B,
    mut observation: F,
) -> Result<NativeLaunchOutcome, SelectorError>
where
    B: FnMut() -> Result<(), SelectorError>,
    F: FnMut() -> Result<Option<Observation>, SelectorError>,
{
    let readiness_deadline = Instant::now() + Duration::from_secs(30);
    let bootstrap = create_pipe()?;
    let lifetime = create_pipe()?;
    let bootstrap_read = bootstrap[0];
    let bootstrap_write = bootstrap[1];
    let lifetime_read = lifetime[0];
    let lifetime_write = lifetime[1];
    let mut command = closed_command(spec.executable);
    command
        .args([
            "--install-root",
            spec.install_root
                .to_str()
                .ok_or_else(|| SelectorError::usage(spec.install_root.display().to_string()))?,
            "--storage-root",
            spec.storage_root
                .to_str()
                .ok_or_else(|| SelectorError::usage(spec.storage_root.display().to_string()))?,
            "--listen",
            spec.listen,
            "--selected-version",
            &spec.selection.selection.version,
            "--selector-generation",
            &spec.selection.generation.to_string(),
            "--launch-id",
            spec.launch_id,
            "--manifest-sha256",
            &spec.selection.selection.manifest_sha256,
            "--bootstrap-status-fd",
            "3",
            "--authority-registry-sha256",
            spec.authority_registry_sha256,
            "--launcher-lifetime-fd",
            "4",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    if let Some(web_listen) = spec.web_listen {
        command.args(["--web-listen", web_listen]);
    }
    // SAFETY: before exec, only async-signal-safe fd/process-group operations
    // are performed; captured descriptors are plain integers.
    unsafe {
        command.pre_exec(move || {
            if libc::dup2(bootstrap_write, 3) < 0 || libc::dup2(lifetime_read, 4) < 0 {
                return Err(io::Error::last_os_error());
            }
            if libc::fcntl(3, libc::F_SETFD, 0) < 0 || libc::fcntl(4, libc::F_SETFD, 0) < 0 {
                return Err(io::Error::last_os_error());
            }
            if libc::setpgid(0, 0) < 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            close_fd(bootstrap_read);
            close_fd(bootstrap_write);
            close_fd(lifetime_read);
            close_fd(lifetime_write);
            return Err(SelectorError::io("supervisor-spawn", error));
        }
    };
    close_fd(bootstrap_write);
    close_fd(lifetime_read);
    // SAFETY: ownership of this still-open descriptor transfers to File.
    let mut bootstrap_file = unsafe { File::from_raw_fd(bootstrap_read) };
    // SAFETY: ownership transfers to File; keeping it alive defines launcher lifetime.
    let _lifetime_file = unsafe { File::from_raw_fd(lifetime_write) };
    let bootstrap_deadline = Instant::now() + Duration::from_secs(10);
    let line = match read_line_deadline(&mut bootstrap_file, bootstrap_deadline) {
        Ok(line) => line,
        Err(error) if error.code == crate::ErrorCode::InvalidState => {
            terminate_group(&mut child)?;
            return Ok(NativeLaunchOutcome::Failed("unexpected-child-exit"));
        }
        Err(error) => return Err(error),
    };
    let body = line
        .strip_suffix(b"\n")
        .ok_or_else(|| SelectorError::invalid_state("protocol-mismatch"))?;
    let status: BootstrapStatus = match serde_json::from_slice(body) {
        Ok(status) => status,
        Err(_) => {
            terminate_group(&mut child)?;
            return Ok(NativeLaunchOutcome::Failed("protocol-mismatch"));
        }
    };
    if serde_json_canonicalizer::to_vec(&status).map_or(true, |canonical| canonical != body)
        || status.format != 1
        || status.launch_id != spec.launch_id
        || status.selection != spec.selection.selection
    {
        terminate_group(&mut child)?;
        return Ok(NativeLaunchOutcome::Failed("protocol-mismatch"));
    }
    if status.state == "failed" {
        let code = status.code.as_deref().unwrap_or("protocol-mismatch");
        terminate_group(&mut child)?;
        return Ok(NativeLaunchOutcome::Failed(closed_bootstrap_code(code)));
    }
    if status.state != "listener-bound" || status.code.is_some() {
        terminate_group(&mut child)?;
        return Ok(NativeLaunchOutcome::Failed("protocol-mismatch"));
    }
    listener_bound()?;
    let mut poll_delay = Duration::from_millis(100);
    let mut next_observation_at = Instant::now();
    loop {
        if SIGNAL_COUNT.load(Ordering::Relaxed) != 0 {
            drain_group(&mut child)?;
            return Ok(NativeLaunchOutcome::PlannedExit);
        }
        if child.try_wait().selector_io("child-wait")?.is_some() {
            return Ok(NativeLaunchOutcome::Failed("unexpected-child-exit"));
        }
        if health_ready(
            spec.listen,
            &spec.selection.selection.version,
            spec.selection.generation,
        )? {
            break;
        }
        if Instant::now() >= readiness_deadline {
            terminate_group(&mut child)?;
            return Ok(NativeLaunchOutcome::Failed("readiness-timeout"));
        }
        std::thread::sleep(poll_delay);
        poll_delay = match poll_delay.as_millis() {
            100 => Duration::from_millis(200),
            200 => Duration::from_millis(400),
            400 => Duration::from_millis(800),
            _ => Duration::from_secs(1),
        };
    }
    let mut observation_complete = false;
    if spec.canary_required {
        let canary_deadline = Instant::now() + Duration::from_secs(120);
        loop {
            if SIGNAL_COUNT.load(Ordering::Relaxed) != 0 {
                drain_group(&mut child)?;
                return Ok(NativeLaunchOutcome::PlannedExit);
            }
            if child.try_wait().selector_io("child-wait")?.is_some() {
                return Ok(NativeLaunchOutcome::Failed("unexpected-child-exit"));
            }
            if let Some(value) = observation()? {
                if matches!(
                    value.state,
                    ObservationState::Ready | ObservationState::Promoted
                ) {
                    observation_complete = matches!(value.state, ObservationState::Promoted);
                    break;
                }
            }
            let durable_deadline_reached = spec
                .canary_deadline_at
                .is_some_and(|deadline| rfc3339_now().map_or(true, |now| now.as_str() >= deadline));
            if Instant::now() >= canary_deadline || durable_deadline_reached {
                terminate_group(&mut child)?;
                return Ok(NativeLaunchOutcome::Failed("canary-timeout"));
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    loop {
        if SIGNAL_COUNT.load(Ordering::Relaxed) != 0 {
            drain_group(&mut child)?;
            return Ok(NativeLaunchOutcome::PlannedExit);
        }
        if let Some(status) = child.try_wait().selector_io("child-wait")? {
            return if status.success() {
                Ok(NativeLaunchOutcome::PlannedExit)
            } else {
                Ok(NativeLaunchOutcome::ReadyThenExited)
            };
        }
        // Drive the promotion boundary without holding selector/.lock across the child/timer
        // wait. Once the observation is promoted it is immutable for this launch, so stop
        // revalidating the signed bundle on every resident poll. That verification is intentionally
        // expensive and previously kept an otherwise idle selector busy indefinitely.
        if !observation_complete && Instant::now() >= next_observation_at {
            observation_complete = observation()?
                .is_some_and(|value| matches!(value.state, ObservationState::Promoted));
            next_observation_at = Instant::now() + Duration::from_secs(30);
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn closed_command(program: &Path) -> Command {
    let mut command = Command::new(program);
    command.env_clear();
    command
}

fn create_pipe() -> Result<[libc::c_int; 2], SelectorError> {
    let mut descriptors = [-1, -1];
    // SAFETY: `descriptors` points to storage for two descriptors.
    if unsafe { libc::pipe(descriptors.as_mut_ptr()) } != 0 {
        return Err(SelectorError::io("pipe-create", io::Error::last_os_error()));
    }
    for descriptor in descriptors {
        // SAFETY: descriptor was returned by pipe and remains open.
        if unsafe { libc::fcntl(descriptor, libc::F_SETFD, libc::FD_CLOEXEC) } < 0 {
            close_fd(descriptors[0]);
            close_fd(descriptors[1]);
            return Err(SelectorError::io(
                "pipe-cloexec",
                io::Error::last_os_error(),
            ));
        }
    }
    Ok(descriptors)
}

fn close_fd(descriptor: libc::c_int) {
    if descriptor >= 0 {
        // SAFETY: close consumes no Rust-owned object here.
        let _ = unsafe { libc::close(descriptor) };
    }
}

fn read_line_deadline(file: &mut File, deadline: Instant) -> Result<Vec<u8>, SelectorError> {
    let flags = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETFL) };
    if flags < 0
        || unsafe { libc::fcntl(file.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0
    {
        return Err(SelectorError::io(
            "bootstrap-nonblock",
            io::Error::last_os_error(),
        ));
    }
    let mut bytes = Vec::new();
    loop {
        let mut buffer = [0_u8; 512];
        match file.read(&mut buffer) {
            Ok(0) => return Err(SelectorError::invalid_state("launch-failed")),
            Ok(count) => {
                bytes.extend_from_slice(&buffer[..count]);
                if bytes.ends_with(b"\n") {
                    return Ok(bytes);
                }
                if bytes.len() > 4096 {
                    return Err(SelectorError::invalid_state("protocol-mismatch"));
                }
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                if Instant::now() >= deadline {
                    return Err(SelectorError::invalid_state("launch-failed"));
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(SelectorError::io("bootstrap-read", error)),
        }
    }
}

fn health_ready(listen: &str, version: &str, generation: u64) -> Result<bool, SelectorError> {
    let Ok(mut stream) = TcpStream::connect_timeout(
        &listen.parse().map_err(|_| SelectorError::usage(listen))?,
        Duration::from_millis(100),
    ) else {
        return Ok(false);
    };
    stream
        .set_read_timeout(Some(Duration::from_millis(200)))
        .selector_io("health-timeout")?;
    stream
        .write_all(
            b"GET /health/ready HTTP/1.1\r\nHost: 127.0.0.1:7347\r\nConnection: close\r\n\r\n",
        )
        .selector_io("health-write")?;
    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .selector_io("health-read")?;
    let Some(position) = response.windows(4).position(|window| window == b"\r\n\r\n") else {
        return Ok(false);
    };
    if !response.starts_with(b"HTTP/1.1 200 ") && !response.starts_with(b"HTTP/1.0 200 ") {
        return Ok(false);
    }
    let body = &response[position + 4..];
    let expected = serde_json_canonicalizer::to_vec(
        &json!({"build":version,"generation":generation,"ready":true}),
    )
    .map_err(|_| SelectorError::corruption("health-response"))?;
    Ok(body == expected)
}

fn terminate_group(child: &mut std::process::Child) -> Result<(), SelectorError> {
    drain_group(child)
}

fn drain_group(child: &mut std::process::Child) -> Result<(), SelectorError> {
    // SAFETY: negative pid addresses the child process group created in pre_exec.
    let _ = unsafe { libc::kill(-(child.id() as libc::pid_t), libc::SIGTERM) };
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if child.try_wait().selector_io("child-drain-wait")?.is_some() {
            return Ok(());
        }
        if SIGNAL_COUNT.load(Ordering::Relaxed) > 1 || Instant::now() >= deadline {
            // SAFETY: same live process-group identity as above.
            let _ = unsafe { libc::kill(-(child.id() as libc::pid_t), libc::SIGKILL) };
            child.wait().selector_io("child-kill-wait")?;
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

extern "C" fn selector_signal(_signal: libc::c_int) {
    SIGNAL_COUNT.fetch_add(1, Ordering::Relaxed);
}

fn install_signal_handlers() -> Result<(), SelectorError> {
    SIGNAL_COUNT.store(0, Ordering::Relaxed);
    // SAFETY: `selector_signal` has C signal-handler ABI and only performs an
    // atomic increment. `signal` retains only its function address.
    if unsafe { libc::signal(libc::SIGTERM, selector_signal as libc::sighandler_t) }
        == libc::SIG_ERR
        // SAFETY: identical reasoning for SIGINT.
        || unsafe { libc::signal(libc::SIGINT, selector_signal as libc::sighandler_t) }
            == libc::SIG_ERR
    {
        return Err(SelectorError::io(
            "signal-handler",
            io::Error::last_os_error(),
        ));
    }
    Ok(())
}

fn closed_bootstrap_code(code: &str) -> &'static str {
    match code {
        "invalid-install" => "invalid-install",
        "selector-mismatch" => "selector-mismatch",
        "protocol-mismatch" => "protocol-mismatch",
        "already-running" => "already-running",
        "unsupported-filesystem" => "unsupported-filesystem",
        "invalid-config" => "invalid-config",
        "corrupt-ledger" => "corrupt-ledger",
        "required-broker-unavailable" => "required-broker-unavailable",
        "listener-unavailable" => "listener-unavailable",
        "endpoint-credential-unavailable" => "endpoint-credential-unavailable",
        "io" => "io",
        _ => "protocol-mismatch",
    }
}

fn launch_id(generation: u64, attempt: u64) -> Result<String, SelectorError> {
    let mut random = [0_u8; 16];
    File::open("/dev/urandom")
        .selector_io("launch-id-open")?
        .read_exact(&mut random)
        .selector_io("launch-id-read")?;
    let mut hex = String::with_capacity(32);
    for byte in random {
        write!(&mut hex, "{byte:02x}").expect("writing to a String cannot fail");
    }
    Ok(format!("{generation}-{attempt}-{hex}"))
}

fn valid_launch_id(value: &str, generation: u64, attempt: u64) -> bool {
    let prefix = format!("{generation}-{attempt}-");
    value.strip_prefix(&prefix).is_some_and(|random| {
        random.len() == 32
            && random
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

fn valid_rfc3339_nano(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 30
        && [4, 7].into_iter().all(|index| bytes[index] == b'-')
        && bytes[10] == b'T'
        && [13, 16].into_iter().all(|index| bytes[index] == b':')
        && bytes[19] == b'.'
        && bytes[29] == b'Z'
        && bytes.iter().enumerate().all(|(index, byte)| {
            matches!(index, 4 | 7 | 10 | 13 | 16 | 19 | 29) || byte.is_ascii_digit()
        })
}

fn valid_install_identity(identity: &InstallIdentity) -> bool {
    identity.format == 1
        && identity.team_id.len() == 10
        && identity
            .team_id
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
        && identity.access_group == format!("{}.com.tekes.shared.endpoint", identity.team_id)
        && [
            &identity.installer_requirement,
            &identity.client_requirement,
            &identity.selector_requirement,
            &identity.supervisor_requirement,
        ]
        .into_iter()
        .all(|requirement| !requirement.is_empty())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::OpenOptionsExt;
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::fs::symlink;
    use std::time::{Duration, Instant};

    use super::{
        NativeLaunchOutcome, NativeLaunchSpec, Observation, ObservationState, Selection,
        SelectionFile, Selector, append_rotating_log_with_sync, closed_command, launch_and_observe,
    };
    use crate::{CodeSignature, CodeSignatureVerifier, InstallIdentity, SelectorError};

    #[derive(Clone, Copy)]
    struct UnusedVerifier;

    #[test]
    fn supervisor_command_drops_the_complete_ambient_environment() {
        let output = closed_command(std::path::Path::new("/usr/bin/env"))
            .output()
            .expect("run environment probe");
        assert!(output.status.success());
        assert!(
            output.stdout.is_empty(),
            "closed command inherited environment"
        );
        assert!(output.stderr.is_empty());
    }

    impl CodeSignatureVerifier for UnusedVerifier {
        fn verify(
            &self,
            _executable: &std::path::Path,
            _requirement: &str,
        ) -> Result<CodeSignature, SelectorError> {
            unreachable!("root-lock wait does not verify executables")
        }

        fn verify_provisioned_app(
            &self,
            _app_bundle: &std::path::Path,
            _requirement: &str,
            _team_id: &str,
            _bundle_identifier: &str,
            _required_access_groups: &[String],
        ) -> Result<(), SelectorError> {
            unreachable!("root-lock wait does not verify app bundles")
        }
    }

    #[test]
    fn resident_observation_guard_accepts_only_the_validated_selection_generation() {
        let temp = tempfile::tempdir().expect("tempdir");
        let selector = Selector::new(temp.path().join("Kernel"), UnusedVerifier);
        selector
            .paths()
            .initialize_for_install()
            .expect("selector layout");
        let expected = SelectionFile {
            format: 1,
            generation: 7,
            selection: Selection {
                manifest_sha256: "a".repeat(64),
                version: "2.0.0".to_owned(),
            },
        };
        super::atomic_json(&selector.paths().current, &expected).expect("current selection");
        symlink(
            super::relative_active_target(&expected.selection.version),
            &selector.paths().active,
        )
        .expect("active selection");

        selector
            .ensure_current_selection_unlocked(&expected)
            .expect("unchanged validated selection");

        let changed = SelectionFile {
            generation: expected.generation + 1,
            ..expected.clone()
        };
        super::atomic_json(&selector.paths().current, &changed).expect("changed selection");
        let error = selector
            .ensure_current_selection_unlocked(&expected)
            .expect_err("stale launch must fail closed");
        assert_eq!(error.code, crate::ErrorCode::InvalidState);
    }

    #[test]
    fn resident_promotion_uses_the_frozen_selection_without_bundle_reverification() {
        let temp = tempfile::tempdir().expect("tempdir");
        let kernel = temp.path().join("Library/Application Support/Tekes/Kernel");
        fs::create_dir_all(kernel.parent().expect("data root")).expect("data root");
        let selector = Selector::new(&kernel, UnusedVerifier);
        selector
            .paths()
            .initialize_for_install()
            .expect("selector layout");
        fs::create_dir_all(
            selector
                .paths()
                .operational_log
                .parent()
                .expect("operational log parent"),
        )
        .expect("operational log directory");
        let expected = SelectionFile {
            format: 1,
            generation: 7,
            selection: Selection {
                manifest_sha256: "a".repeat(64),
                version: "2.0.0".to_owned(),
            },
        };
        super::atomic_json(&selector.paths().current, &expected).expect("current selection");
        symlink(
            super::relative_active_target(&expected.selection.version),
            &selector.paths().active,
        )
        .expect("active selection");
        let observation = Observation {
            attempt: 1,
            canary_deadline_at: None,
            canary_required: false,
            canary_run: None,
            canary_session: None,
            consecutive_failures: 0,
            deadline_at: "2026-09-01T00:00:30.000000000Z".to_owned(),
            format: 1,
            generation: expected.generation,
            last_code: "ready".to_owned(),
            launch_id: "7-1-0123456789abcdef0123456789abcdef".to_owned(),
            manifest_sha256: expected.selection.manifest_sha256.clone(),
            rollback_eligible: true,
            started_at: "2026-09-01T00:00:00.000000000Z".to_owned(),
            state: ObservationState::Ready,
            version: expected.selection.version.clone(),
            window_closes_at: Some("2026-09-01T00:00:20.000000000Z".to_owned()),
        };
        selector
            .publish_observation(&observation)
            .expect("ready observation");

        assert!(
            selector
                .promote_validated_observation_if_due(
                    &expected.selection.version,
                    "2026-09-01T00:00:21.000000000Z",
                )
                .expect("resident promotion")
        );
        let promoted = selector
            .read_observation(&expected.selection.version)
            .expect("read observation")
            .expect("promoted observation");
        assert_eq!(promoted.state, ObservationState::Promoted);
        assert!(!promoted.rollback_eligible);
        assert_eq!(promoted.last_code, "promoted");
    }

    #[test]
    fn operational_log_propagates_full_sync_failure() {
        let temp = tempfile::tempdir().expect("tempdir");
        let log = temp.path().join("selector.jsonl");
        let error = append_rotating_log_with_sync(&log, b"{}\n", |_| {
            Err(SelectorError::io(
                "injected-log-full-sync",
                std::io::Error::other("fault"),
            ))
        })
        .expect_err("full-sync failure must reject the operational record");
        assert_eq!(error.code, crate::ErrorCode::Io);
        assert!(error.details.get("operation").is_some());
    }

    #[test]
    fn bootstrap_record_must_bind_the_complete_frozen_selection() {
        let temp = tempfile::tempdir().expect("tempdir");
        let script = temp.path().join("supervisor");
        fs::write(
            &script,
            b"#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$2/argv.txt\"\nprintf '%s\\n' '{\"format\":1,\"launch_id\":\"1-1-0123456789abcdef0123456789abcdef\",\"state\":\"listener-bound\"}' >&3\nwhile :; do sleep 1; done\n",
        )
        .expect("write mock supervisor");
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).expect("script mode");
        let selection = SelectionFile {
            format: 1,
            generation: 1,
            selection: Selection {
                manifest_sha256: "a".repeat(64),
                version: "1.0.0".to_owned(),
            },
        };
        let outcome = launch_and_observe(
            &NativeLaunchSpec {
                executable: &script,
                install_root: temp.path(),
                storage_root: temp.path(),
                listen: "127.0.0.1:7347",
                web_listen: None,
                selection: &selection,
                authority_registry_sha256: &"b".repeat(64),
                launch_id: "1-1-0123456789abcdef0123456789abcdef",
                canary_required: false,
                canary_deadline_at: None,
            },
            || Ok(()),
            || Ok(None),
        )
        .expect("observe malformed bootstrap record");
        assert_eq!(outcome, NativeLaunchOutcome::Failed("protocol-mismatch"));
        let argv = fs::read_to_string(temp.path().join("argv.txt")).expect("captured argv");
        let argv = argv.lines().collect::<Vec<_>>();
        assert_eq!(
            argv.windows(2)
                .find(|pair| pair[0] == "--launch-id")
                .expect("launch-id pair"),
            ["--launch-id", "1-1-0123456789abcdef0123456789abcdef"]
        );
    }

    #[test]
    fn predecessor_probe_waits_in_100ms_intervals_and_times_out_without_spawn() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root_lock = temp.path().join(".root-lock");
        let owner = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&root_lock)
            .expect("root lock");
        // SAFETY: owner keeps the descriptor live through the bounded wait.
        assert_eq!(
            unsafe { libc::flock(owner.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
            0
        );
        let selector = Selector::new(temp.path().join("Kernel"), UnusedVerifier);
        let started = Instant::now();
        assert!(
            !selector
                .wait_for_predecessor_until(temp.path(), started + Duration::from_millis(150))
                .expect("bounded predecessor wait")
        );
        assert!(started.elapsed() >= Duration::from_millis(100));
    }

    #[test]
    fn transaction_lock_waits_for_a_short_resident_selector_tenure() {
        let temp = tempfile::tempdir().expect("tempdir");
        let selector = Selector::new(temp.path().join("Kernel"), UnusedVerifier);
        selector
            .paths()
            .initialize_for_install()
            .expect("selector layout");
        let owner = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&selector.paths().lock)
            .expect("transaction lock");
        // SAFETY: owner keeps the descriptor live until the holder thread releases it.
        assert_eq!(
            unsafe { libc::flock(owner.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
            0
        );
        let holder = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(150));
            drop(owner);
        });
        let started = Instant::now();
        let acquired = selector
            .acquire_transaction_lock()
            .expect("bounded transaction-lock wait");
        assert!(started.elapsed() >= Duration::from_millis(100));
        assert!(started.elapsed() < Duration::from_secs(5));
        drop(acquired);
        holder.join().expect("holder");
    }

    #[test]
    fn prelaunch_timeout_is_a_synced_log_carrier_without_a_spawn_attempt() {
        let temp = tempfile::tempdir().expect("tempdir");
        let kernel = temp.path().join("Library/Application Support/Tekes/Kernel");
        fs::create_dir_all(kernel.parent().expect("data root")).expect("data root");
        let selector = Selector::new(&kernel, UnusedVerifier);
        selector
            .paths()
            .initialize_for_install()
            .expect("selector layout");
        fs::create_dir_all(
            selector
                .paths()
                .operational_log
                .parent()
                .expect("log parent"),
        )
        .expect("log root");
        let current = SelectionFile {
            format: 1,
            generation: 7,
            selection: Selection {
                manifest_sha256: "a".repeat(64),
                version: "2.0.0".to_owned(),
            },
        };
        selector
            .record_predecessor_timeout(&current)
            .expect("record prelaunch timeout");
        let failure = selector
            .read_prelaunch_failure()
            .expect("read carrier")
            .expect("active failure");
        assert_eq!(failure.generation, 7);
        assert_eq!(failure.version, "2.0.0");
        assert!(
            !fs::read_to_string(&selector.paths().operational_log)
                .expect("log")
                .contains("launch_id")
        );
        selector
            .clear_prelaunch_failure()
            .expect("append clear record");
        assert!(
            selector
                .read_prelaunch_failure()
                .expect("read cleared carrier")
                .is_none()
        );
    }

    #[test]
    fn environment_retry_resets_on_a_detected_config_fact_change() {
        let temp = tempfile::tempdir().expect("tempdir");
        let data_root = temp.path().join("Library/Application Support/Tekes");
        let kernel = data_root.join("Kernel");
        fs::create_dir_all(&data_root).expect("data root");
        let selector = Selector::new(&kernel, UnusedVerifier);
        selector
            .paths()
            .initialize_for_install()
            .expect("selector layout");
        let installer = data_root.join("Installer");
        fs::create_dir(&installer).expect("installer");
        super::atomic_json(
            &selector.paths().install_identity,
            &InstallIdentity {
                access_group: "TEKESAPP01.com.tekes.shared.endpoint".to_owned(),
                client_requirement: "client".to_owned(),
                format: 1,
                installer_requirement: "installer".to_owned(),
                selector_requirement: "selector".to_owned(),
                supervisor_requirement: "supervisor".to_owned(),
                team_id: "TEKESAPP01".to_owned(),
            },
        )
        .expect("identity");
        let storage = data_root.join("threads");
        fs::create_dir(&storage).expect("storage");
        fs::write(storage.join(".root-lock"), b"").expect("root lock");
        fs::set_permissions(
            storage.join(".root-lock"),
            fs::Permissions::from_mode(0o600),
        )
        .expect("root lock mode");
        let config = data_root.join("config");
        fs::create_dir(&config).expect("config");
        fs::write(config.join("settings.json"), b"{}\n").expect("config file");
        let changed = config.join("settings.json");
        let writer = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(150));
            fs::write(changed, b"{\"changed\":true}\n").expect("change config");
        });
        let started = Instant::now();
        assert!(
            selector
                .wait_environment_retry(&storage, "127.0.0.1:0", Duration::from_secs(2))
                .expect("fact-polled retry")
        );
        writer.join().expect("writer");
        assert!(started.elapsed() < Duration::from_secs(2));
    }

    #[test]
    fn candidate_conformance_probe_rejects_stderr_and_extra_lines() {
        const REPLY: &str = "{\"architecture\":\"aarch64\",\"conformance_sha256\":\"dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd\",\"format\":1,\"operation\":\"describe-conformance\",\"version\":\"2.0.0\"}";
        let temp = tempfile::tempdir().expect("tempdir");
        let good = temp.path().join("good-selector");
        fs::write(&good, format!("#!/bin/sh\nprintf '%s\\n' '{REPLY}'\n")).expect("good candidate");
        fs::set_permissions(&good, fs::Permissions::from_mode(0o755)).expect("good mode");
        let reply = super::query_candidate_conformance(&good).expect("closed candidate reply");
        assert_eq!(reply.version, "2.0.0");

        let noisy = temp.path().join("noisy-selector");
        fs::write(
            &noisy,
            format!("#!/bin/sh\nprintf diagnostic >&2\nprintf '%s\\n' '{REPLY}'\n"),
        )
        .expect("noisy candidate");
        fs::set_permissions(&noisy, fs::Permissions::from_mode(0o755)).expect("noisy mode");
        assert!(super::query_candidate_conformance(&noisy).is_err());

        let extra = temp.path().join("extra-selector");
        fs::write(
            &extra,
            format!("#!/bin/sh\nprintf '%s\\n' '{REPLY}' '{{}}'\n"),
        )
        .expect("extra candidate");
        fs::set_permissions(&extra, fs::Permissions::from_mode(0o755)).expect("extra mode");
        assert!(super::query_candidate_conformance(&extra).is_err());
    }
}
