use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::error::{IoContext, SelectorError};
use crate::fs::{FileLock, atomic_json, full_sync, read_canonical, sync_directory, validate_hex};

pub const INSTALLER_PHASES: &[(&str, &[&str])] = &[
    (
        "install",
        &[
            "prepared",
            "identity-published",
            "credential-published",
            "bundles-published",
            "selection-published",
            "plist-published",
            "service-started",
            "closed",
        ],
    ),
    (
        "rotate-credential",
        &[
            "prepared",
            "service-stopped",
            "credential-replaced",
            "service-started",
            "closed",
        ],
    ),
    (
        "uninstall",
        &[
            "prepared",
            "service-stopped",
            "credential-deleted",
            "plist-removed",
            "binaries-removed",
            "closed",
        ],
    ),
];

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum InstallerOperationType {
    #[serde(rename = "install")]
    Install,
    #[serde(rename = "rotate-credential")]
    RotateCredential,
    #[serde(rename = "uninstall")]
    Uninstall,
}

impl InstallerOperationType {
    const fn phases(self) -> &'static [&'static str] {
        match self {
            Self::Install => INSTALLER_PHASES[0].1,
            Self::RotateCredential => INSTALLER_PHASES[1].1,
            Self::Uninstall => INSTALLER_PHASES[2].1,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallerOperation {
    pub format: u8,
    pub install_identity_sha256: String,
    pub op_id: String,
    pub phase: String,
    pub request_sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Value>,
    #[serde(rename = "type")]
    pub operation_type: InstallerOperationType,
}

#[derive(Clone, Debug)]
pub struct InstallRequest {
    pub install_identity_sha256: String,
    pub op_id: String,
    pub request_sha256: String,
    pub operation_type: InstallerOperationType,
}

pub trait InstallerEffects {
    fn publish_identity(&mut self) -> Result<(), SelectorError>;
    fn ensure_credential(&mut self) -> Result<(), SelectorError>;
    fn publish_bundles(&mut self) -> Result<(), SelectorError>;
    fn publish_selection(&mut self) -> Result<(), SelectorError>;
    fn publish_plist(&mut self) -> Result<(), SelectorError>;
    fn start_service(&mut self) -> Result<(), SelectorError>;
    fn stop_service(&mut self) -> Result<(), SelectorError>;
    fn replace_credential(&mut self) -> Result<(), SelectorError>;
    fn delete_credential(&mut self) -> Result<(), SelectorError>;
    fn remove_plist(&mut self) -> Result<(), SelectorError>;
    fn remove_binaries(&mut self) -> Result<(), SelectorError>;

    /// Verifies the durable external fact represented by `phase`. Production
    /// adapters override this with filesystem, launchd, and launch-credential checks.
    /// The default preserves source compatibility for test-only effect
    /// adapters; release integration must exercise an overriding adapter.
    fn verify_completed_phase(
        &mut self,
        _operation: &InstallerOperation,
        _phase: &str,
    ) -> Result<bool, SelectorError> {
        Ok(true)
    }
}

#[derive(Clone, Debug)]
pub struct InstallerStateMachine {
    directory: PathBuf,
    lock: PathBuf,
    operation: PathBuf,
}

impl InstallerStateMachine {
    #[must_use]
    pub fn new(directory: impl Into<PathBuf>) -> Self {
        let directory = directory.into();
        Self {
            lock: directory.join(".lock"),
            operation: directory.join("operation.json"),
            directory,
        }
    }

    pub fn initialize(&self) -> Result<(), SelectorError> {
        if !self.directory.exists() {
            fs::DirBuilder::new()
                .recursive(false)
                .mode(0o700)
                .create(&self.directory)
                .selector_io("installer-directory-create")?;
        }
        let metadata =
            fs::symlink_metadata(&self.directory).selector_io("installer-directory-stat")?;
        if !metadata.is_dir()
            || metadata.file_type().is_symlink()
            || metadata.permissions().mode() & 0o7777 != 0o700
        {
            return Err(SelectorError::corruption(
                self.directory.display().to_string(),
            ));
        }
        let lock = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
            .open(&self.lock)
            .selector_io("installer-lock-create")?;
        lock.set_permissions(fs::Permissions::from_mode(0o600))
            .selector_io("installer-lock-mode")?;
        sync_directory(&self.directory)
    }

    pub fn begin(&self, request: InstallRequest) -> Result<Option<Value>, SelectorError> {
        if !validate_hex(&request.request_sha256)
            || !validate_hex(&request.install_identity_sha256)
            || request.op_id.is_empty()
        {
            return Err(SelectorError::invalid_state("invalid-installer-request"));
        }
        let _lock = FileLock::try_exclusive(&self.lock)?;
        if let Some(existing) = self.read_optional()? {
            validate_operation(&existing)?;
            if existing.phase != "closed" {
                if existing.request_sha256 == request.request_sha256
                    && existing.operation_type == request.operation_type
                    && existing.install_identity_sha256 == request.install_identity_sha256
                {
                    return Ok(None);
                }
                return Err(SelectorError::invalid_state(
                    "installer-operation-in-progress",
                ));
            }
            if existing.request_sha256 == request.request_sha256
                && existing.operation_type == request.operation_type
                && existing.install_identity_sha256 == request.install_identity_sha256
            {
                return Ok(existing.response);
            }
        }
        atomic_json(
            &self.operation,
            &InstallerOperation {
                format: 1,
                install_identity_sha256: request.install_identity_sha256,
                op_id: request.op_id,
                operation_type: request.operation_type,
                phase: "prepared".to_owned(),
                request_sha256: request.request_sha256,
                response: None,
            },
        )?;
        Ok(None)
    }

    pub fn recover<E: InstallerEffects>(&self, effects: &mut E) -> Result<Value, SelectorError> {
        let _lock = FileLock::try_exclusive(&self.lock)?;
        let mut operation = self
            .read_optional()?
            .ok_or_else(|| SelectorError::invalid_state("no-installer-operation"))?;
        validate_operation(&operation)?;
        if operation.phase == "closed" {
            return operation
                .response
                .ok_or_else(|| SelectorError::corruption(self.operation.display().to_string()));
        }
        let phases = operation.operation_type.phases();
        let mut position = phases
            .iter()
            .position(|phase| *phase == operation.phase)
            .ok_or_else(|| SelectorError::corruption(self.operation.display().to_string()))?;
        if operation.phase != "prepared"
            && operation.phase != "closed"
            && !effects.verify_completed_phase(&operation, &operation.phase)?
        {
            return Err(SelectorError::corruption(
                self.operation.display().to_string(),
            ));
        }
        while phases[position] != "closed" {
            let next = phases[position + 1];
            apply_effect(operation.operation_type, next, effects)?;
            if next != "closed" && !effects.verify_completed_phase(&operation, next)? {
                return Err(SelectorError::corruption(
                    self.operation.display().to_string(),
                ));
            }
            operation.phase = next.to_owned();
            if next == "closed" {
                operation.response = Some(terminal_response(operation.operation_type));
            }
            atomic_json(&self.operation, &operation)?;
            position += 1;
        }
        operation
            .response
            .ok_or_else(|| SelectorError::corruption(self.operation.display().to_string()))
    }

    pub fn current(&self) -> Result<Option<InstallerOperation>, SelectorError> {
        let _lock = FileLock::try_exclusive(&self.lock)?;
        let operation = self.read_optional()?;
        if let Some(operation) = &operation {
            validate_operation(operation)?;
        }
        Ok(operation)
    }

    fn read_optional(&self) -> Result<Option<InstallerOperation>, SelectorError> {
        match read_canonical(&self.operation) {
            Ok(operation) => Ok(Some(operation)),
            Err(error) if error.code == crate::ErrorCode::Io && !self.operation.exists() => {
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }
}

fn validate_operation(operation: &InstallerOperation) -> Result<(), SelectorError> {
    let valid_phase = operation
        .operation_type
        .phases()
        .contains(&operation.phase.as_str());
    let response_valid = (operation.phase == "closed") == operation.response.is_some();
    let terminal_valid = operation.phase != "closed"
        || operation.response.as_ref() == Some(&terminal_response(operation.operation_type));
    if operation.format != 1
        || !validate_hex(&operation.request_sha256)
        || !validate_hex(&operation.install_identity_sha256)
        || operation.op_id.is_empty()
        || !valid_phase
        || !response_valid
        || !terminal_valid
    {
        return Err(SelectorError::corruption("Installer/operation.json"));
    }
    Ok(())
}

fn apply_effect<E: InstallerEffects>(
    operation_type: InstallerOperationType,
    phase: &str,
    effects: &mut E,
) -> Result<(), SelectorError> {
    match (operation_type, phase) {
        (InstallerOperationType::Install, "identity-published") => effects.publish_identity(),
        (InstallerOperationType::Install, "credential-published") => effects.ensure_credential(),
        (InstallerOperationType::Install, "bundles-published") => effects.publish_bundles(),
        (InstallerOperationType::Install, "selection-published") => effects.publish_selection(),
        (InstallerOperationType::Install, "plist-published") => effects.publish_plist(),
        (
            InstallerOperationType::Install | InstallerOperationType::RotateCredential,
            "service-started",
        ) => effects.start_service(),
        (
            InstallerOperationType::RotateCredential | InstallerOperationType::Uninstall,
            "service-stopped",
        ) => effects.stop_service(),
        (InstallerOperationType::RotateCredential, "credential-replaced") => {
            effects.replace_credential()
        }
        (InstallerOperationType::Uninstall, "credential-deleted") => effects.delete_credential(),
        (InstallerOperationType::Uninstall, "plist-removed") => effects.remove_plist(),
        (InstallerOperationType::Uninstall, "binaries-removed") => effects.remove_binaries(),
        (_, "closed") => Ok(()),
        _ => Err(SelectorError::corruption("Installer/operation.json")),
    }
}

fn terminal_response(operation_type: InstallerOperationType) -> Value {
    match operation_type {
        InstallerOperationType::Install => {
            json!({"format":1,"operation":"install","service":"running"})
        }
        InstallerOperationType::RotateCredential => {
            json!({"format":1,"operation":"rotate-credential","service":"running"})
        }
        InstallerOperationType::Uninstall => {
            json!({"format":1,"operation":"uninstall","service":"absent"})
        }
    }
}

pub fn random_credential() -> Result<[u8; 32], SelectorError> {
    let mut entropy = [0_u8; 32];
    let mut source = File::open("/dev/urandom").selector_io("credential-random-open")?;
    source
        .read_exact(&mut entropy)
        .selector_io("credential-random-read")?;
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut credential = [0_u8; 32];
    for (output, random) in credential.iter_mut().zip(entropy) {
        *output = ALPHABET[usize::from(random & 63)];
    }
    Ok(credential)
}

pub fn durable_credential_file_for_test(path: &Path, bytes: &[u8]) -> Result<(), SelectorError> {
    if bytes.len() != 32 {
        return Err(SelectorError::invalid_state("credential-byte-count"));
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
        .selector_io("credential-create")?;
    file.write_all(bytes).selector_io("credential-write")?;
    full_sync(&file)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        InstallRequest, InstallerEffects, InstallerOperation, InstallerOperationType,
        InstallerStateMachine,
    };
    use crate::SelectorError;

    #[derive(Default)]
    struct Effects {
        completed: Vec<&'static str>,
        fail_once: Option<&'static str>,
    }

    impl Effects {
        fn effect(&mut self, name: &'static str) -> Result<(), SelectorError> {
            if self.fail_once == Some(name) {
                self.fail_once = None;
                return Err(SelectorError::invalid_state(format!("injected-{name}")));
            }
            if !self.completed.contains(&name) {
                self.completed.push(name);
            }
            Ok(())
        }
    }

    impl InstallerEffects for Effects {
        fn publish_identity(&mut self) -> Result<(), SelectorError> {
            self.effect("identity-published")
        }
        fn ensure_credential(&mut self) -> Result<(), SelectorError> {
            self.effect("credential-published")
        }
        fn publish_bundles(&mut self) -> Result<(), SelectorError> {
            self.effect("bundles-published")
        }
        fn publish_selection(&mut self) -> Result<(), SelectorError> {
            self.effect("selection-published")
        }
        fn publish_plist(&mut self) -> Result<(), SelectorError> {
            self.effect("plist-published")
        }
        fn start_service(&mut self) -> Result<(), SelectorError> {
            self.effect("service-started")
        }
        fn stop_service(&mut self) -> Result<(), SelectorError> {
            self.effect("service-stopped")
        }
        fn replace_credential(&mut self) -> Result<(), SelectorError> {
            self.effect("credential-replaced")
        }
        fn delete_credential(&mut self) -> Result<(), SelectorError> {
            self.effect("credential-deleted")
        }
        fn remove_plist(&mut self) -> Result<(), SelectorError> {
            self.effect("plist-removed")
        }
        fn remove_binaries(&mut self) -> Result<(), SelectorError> {
            self.effect("binaries-removed")
        }
        fn verify_completed_phase(
            &mut self,
            _operation: &InstallerOperation,
            phase: &str,
        ) -> Result<bool, SelectorError> {
            Ok(self.completed.contains(&phase))
        }
    }

    #[test]
    fn installer_recovers_last_durable_phase_and_replays_closed_response() {
        let temp = tempfile::tempdir().expect("temporary installer root");
        let machine = InstallerStateMachine::new(temp.path().join("Installer"));
        machine.initialize().expect("initialize installer state");
        let request = InstallRequest {
            install_identity_sha256: "a".repeat(64),
            op_id: "install-0001".to_owned(),
            request_sha256: "b".repeat(64),
            operation_type: InstallerOperationType::Install,
        };
        assert_eq!(machine.begin(request.clone()).expect("begin"), None);
        let mut effects = Effects {
            completed: Vec::new(),
            fail_once: Some("bundles-published"),
        };
        assert!(machine.recover(&mut effects).is_err());
        assert_eq!(
            machine
                .current()
                .expect("current")
                .expect("operation")
                .phase,
            "credential-published"
        );
        let response = machine.recover(&mut effects).expect("resume install");
        assert_eq!(
            response,
            json!({"format":1,"operation":"install","service":"running"})
        );
        assert_eq!(machine.begin(request).expect("exact retry"), Some(response));
    }

    #[test]
    fn installer_fault_injection_covers_every_effect_boundary() {
        let cases = [
            (
                InstallerOperationType::Install,
                [
                    "identity-published",
                    "credential-published",
                    "bundles-published",
                    "selection-published",
                    "plist-published",
                    "service-started",
                ]
                .as_slice(),
            ),
            (
                InstallerOperationType::RotateCredential,
                ["service-stopped", "credential-replaced", "service-started"].as_slice(),
            ),
            (
                InstallerOperationType::Uninstall,
                [
                    "service-stopped",
                    "credential-deleted",
                    "plist-removed",
                    "binaries-removed",
                ]
                .as_slice(),
            ),
        ];
        for (case_index, (operation_type, boundaries)) in cases.into_iter().enumerate() {
            for (boundary_index, boundary) in boundaries.iter().enumerate() {
                let temp = tempfile::tempdir().expect("temporary installer root");
                let machine = InstallerStateMachine::new(temp.path().join("Installer"));
                machine.initialize().expect("initialize installer state");
                let request = InstallRequest {
                    install_identity_sha256: "a".repeat(64),
                    op_id: format!("operation-{case_index}-{boundary_index}"),
                    request_sha256: format!("{:064x}", case_index * 16 + boundary_index + 1),
                    operation_type,
                };
                machine.begin(request.clone()).expect("begin");
                let mut effects = Effects {
                    completed: Vec::new(),
                    fail_once: Some(boundary),
                };
                machine
                    .recover(&mut effects)
                    .expect_err("injected boundary must stop recovery");
                let durable_phase = machine
                    .current()
                    .expect("current")
                    .expect("operation")
                    .phase;
                assert_ne!(durable_phase, *boundary);
                let response = machine.recover(&mut effects).expect("resume");
                assert_eq!(machine.begin(request).expect("retry"), Some(response));
            }
        }
    }

    #[test]
    fn installer_retry_identity_must_match_the_retained_operation() {
        let temp = tempfile::tempdir().expect("temporary installer root");
        let machine = InstallerStateMachine::new(temp.path().join("Installer"));
        machine.initialize().expect("initialize installer state");
        let request = InstallRequest {
            install_identity_sha256: "a".repeat(64),
            op_id: "install-identity-a".to_owned(),
            request_sha256: "b".repeat(64),
            operation_type: InstallerOperationType::Install,
        };
        machine.begin(request.clone()).expect("begin");
        let mut mismatched = request;
        mismatched.install_identity_sha256 = "c".repeat(64);
        assert!(machine.begin(mismatched).is_err());
    }

    #[test]
    fn installer_recovery_refuses_a_missing_durable_phase_fact() {
        let temp = tempfile::tempdir().expect("temporary installer root");
        let machine = InstallerStateMachine::new(temp.path().join("Installer"));
        machine.initialize().expect("initialize installer state");
        machine
            .begin(InstallRequest {
                install_identity_sha256: "a".repeat(64),
                op_id: "install-fact-check".to_owned(),
                request_sha256: "f".repeat(64),
                operation_type: InstallerOperationType::Install,
            })
            .expect("begin");
        let mut effects = Effects {
            completed: Vec::new(),
            fail_once: Some("bundles-published"),
        };
        machine
            .recover(&mut effects)
            .expect_err("injected crash after credential phase");
        effects
            .completed
            .retain(|phase| *phase != "credential-published");
        let error = machine
            .recover(&mut effects)
            .expect_err("missing durable credential fact must fail closed");
        assert_eq!(error.code, crate::ErrorCode::Corruption);
        assert_eq!(
            machine
                .current()
                .expect("current")
                .expect("operation")
                .phase,
            "credential-published"
        );
    }
}
