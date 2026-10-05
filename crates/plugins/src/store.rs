use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use rustix::fs::{Mode, OFlags, fchmod, open};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use store::FullSync;
use thiserror::Error;

use crate::model::{ComponentKind, ComponentProjection};
use crate::signature::{
    NativeHelperIdentity, NativeHelperVerifier, PUBLISHER_ATTESTATION_PATH, PublisherIdentity,
    SignaturePolicy, hex, verify_publisher,
};
use crate::{
    HostEnvironment, Manifest, PLUGIN_MANIFEST_FILE, PluginComponentReference, PluginSource,
    PluginVersion, ResolvedPluginExecutable,
};

static OPERATION_ORDINAL: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum PluginError {
    #[error("invalid semantic version: {0}")]
    InvalidVersion(String),
    #[error("invalid plugin manifest: {0}")]
    InvalidManifest(String),
    #[error("plugin is incompatible with this host: {0}")]
    Incompatible(String),
    #[error("invalid .tekesplugin archive: {0}")]
    InvalidArchive(String),
    #[error("plugin signature validation failed: {0}")]
    Signature(String),
    #[error("publisher trust is required: {0}")]
    PublisherTrustRequired(String),
    #[error("plugin is not installed: {0}")]
    NotInstalled(String),
    #[error("plugin downgrade from {installed} to {candidate} requires authorization")]
    DowngradeRejected {
        installed: String,
        candidate: String,
    },
    #[error("equal-precedence package content changed for {0}")]
    SameVersionChanged(String),
    #[error("capability was not requested by the plugin: {0}")]
    UndeclaredGrant(String),
    #[error("plugin cannot be enabled without capability: {0}")]
    MissingGrant(String),
    #[error("component collision for {kind:?}/{id} between {first} and {second}")]
    ComponentCollision {
        kind: ComponentKind,
        id: String,
        first: String,
        second: String,
    },
    #[error("plugin registry is corrupt: {0}")]
    CorruptRegistry(String),
    #[error("plugin storage failure: {0}")]
    Storage(String),
    #[error("injected crash after {0:?}")]
    InjectedCrash(FaultPoint),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FaultPoint {
    PackageDurableBeforeJournal,
    PackagePublished,
    RegistryPublished,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InstallOptions {
    pub grants: BTreeSet<String>,
    pub enable: bool,
    pub allow_downgrade: bool,
    pub allow_same_version_replacement: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IntegrityStatus {
    LocalUnverified,
    PublisherVerified,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginReceipt {
    pub plugin_id: String,
    pub version: PluginVersion,
    pub display_name: String,
    pub package_digest: String,
    pub package_relative_path: String,
    pub source: String,
    pub integrity: IntegrityStatus,
    pub requested_capabilities: Vec<crate::CapabilityRequest>,
    pub granted_capabilities: Vec<String>,
    pub native_helper_identities: Vec<NativeHelperIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher_identity: Option<PublisherIdentity>,
    pub enabled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginInspection {
    pub manifest: Manifest,
    pub package_digest: String,
    pub integrity: IntegrityStatus,
    pub requested_capabilities: Vec<crate::CapabilityRequest>,
}

impl PluginReceipt {
    fn requested_ids(&self) -> BTreeSet<String> {
        self.requested_capabilities
            .iter()
            .map(|capability| capability.id.clone())
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManagementRequest {
    Install {
        source: PluginSource,
        options: InstallOptions,
    },
    SetEnabled {
        plugin_id: String,
        enabled: bool,
    },
    SetGrants {
        plugin_id: String,
        grants: BTreeSet<String>,
    },
    Remove {
        plugin_id: String,
    },
    List,
    Components,
    Recover,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManagementResult {
    Receipt(Box<PluginReceipt>),
    Removed(bool),
    Receipts(Vec<PluginReceipt>),
    Components(Vec<ComponentProjection>),
    Recovered,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Registry {
    format: u32,
    plugins: Vec<PluginReceipt>,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            format: 1,
            plugins: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum OperationKind {
    Install,
    SetEnabled,
    SetGrants,
    Remove,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum OperationPhase {
    PackagePublished,
    RegistryPublished,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Operation {
    format: u32,
    id: String,
    kind: OperationKind,
    phase: OperationPhase,
    plugin_id: String,
    previous: Option<PluginReceipt>,
    next: Option<PluginReceipt>,
}

pub struct PluginStore<V> {
    root: PathBuf,
    host: HostEnvironment,
    signature_policy: SignaturePolicy,
    verifier: V,
    fault: Option<FaultPoint>,
}

impl<V: NativeHelperVerifier> PluginStore<V> {
    pub fn open(
        root: impl Into<PathBuf>,
        host: HostEnvironment,
        signature_policy: SignaturePolicy,
        verifier: V,
    ) -> Result<Self, PluginError> {
        let mut store = Self {
            root: root.into(),
            host,
            signature_policy,
            verifier,
            fault: None,
        };
        store.create_layout()?;
        store.recover()?;
        Ok(store)
    }

    #[must_use]
    pub fn injecting_fault(mut self, fault: FaultPoint) -> Self {
        self.fault = Some(fault);
        self
    }

    pub fn manage(&mut self, request: ManagementRequest) -> Result<ManagementResult, PluginError> {
        match request {
            ManagementRequest::Install { source, options } => self
                .install(&source, options)
                .map(Box::new)
                .map(ManagementResult::Receipt),
            ManagementRequest::SetEnabled { plugin_id, enabled } => self
                .set_enabled(&plugin_id, enabled)
                .map(Box::new)
                .map(ManagementResult::Receipt),
            ManagementRequest::SetGrants { plugin_id, grants } => self
                .set_grants(&plugin_id, grants)
                .map(Box::new)
                .map(ManagementResult::Receipt),
            ManagementRequest::Remove { plugin_id } => {
                self.remove(&plugin_id).map(ManagementResult::Removed)
            }
            ManagementRequest::List => self.list().map(ManagementResult::Receipts),
            ManagementRequest::Components => self.components().map(ManagementResult::Components),
            ManagementRequest::Recover => self.recover().map(|()| ManagementResult::Recovered),
        }
    }

    pub fn install(
        &mut self,
        source: &PluginSource,
        options: InstallOptions,
    ) -> Result<PluginReceipt, PluginError> {
        self.recover()?;
        let operation_id = operation_id();
        let stage = self.staging_root().join(&operation_id);
        source.stage(&stage)?;
        let package = self.load_package(&stage)?;
        validate_grants(&options.grants, &package.manifest)?;
        if options.enable {
            require_all_grants(&options.grants, &package.manifest)?;
        }
        let helpers = self.verify_native_helpers(&package)?;
        let publisher = verify_publisher(&stage, || {
            package_digest(&stage, Some(PUBLISHER_ATTESTATION_PATH))
        })?;
        let publisher_is_trusted = publisher.as_ref().is_some_and(|identity| {
            self.signature_policy
                .trusted_publishers
                .get(&identity.publisher_id)
                == Some(&identity.public_key_fingerprint)
        });
        if (publisher.is_some() && !publisher_is_trusted)
            || (publisher.is_none() && !self.signature_policy.allow_unsigned_local)
        {
            return Err(PluginError::PublisherTrustRequired(
                "package has no publisher key pinned by the trust policy".to_owned(),
            ));
        }
        let digest = package_digest(&stage, None)?;
        let mut registry = self.read_registry()?;
        let previous = registry
            .plugins
            .iter()
            .find(|receipt| receipt.plugin_id == package.manifest.id)
            .cloned();
        if let Some(installed) = &previous {
            let precedence = package.manifest.version.precedence_cmp(&installed.version);
            if precedence.is_lt() && !options.allow_downgrade {
                return Err(PluginError::DowngradeRejected {
                    installed: installed.version.to_string(),
                    candidate: package.manifest.version.to_string(),
                });
            }
            if precedence.is_eq()
                && digest != installed.package_digest
                && !options.allow_same_version_replacement
            {
                return Err(PluginError::SameVersionChanged(
                    package.manifest.version.to_string(),
                ));
            }
        }
        let relative = format!(
            "packages/{}/{}/{}",
            package.manifest.id, package.manifest.version, digest
        );
        let final_path = self.root.join(&relative);
        if path_is_present(&final_path)? {
            require_directory(&final_path)?;
            if package_digest(&final_path, None)? != digest {
                return Err(PluginError::CorruptRegistry(
                    "existing package digest mismatch".to_owned(),
                ));
            }
            remove_tree(&stage)?;
        } else {
            let plugin_root = self.packages_root().join(&package.manifest.id);
            ensure_directory(&plugin_root)?;
            let version_root = plugin_root.join(package.manifest.version.to_string());
            ensure_directory(&version_root)?;
            sync_tree(&stage)?;
            sync_directory(&self.staging_root())?;
            sync_directory(&version_root)?;
            fs::rename(&stage, &final_path)
                .map_err(|error| PluginError::Storage(error.to_string()))?;
            sync_directory(&self.staging_root())?;
            sync_directory(&version_root)?;
            seal_tree(&final_path)?;
            sync_tree(&final_path)?;
            sync_directory(&version_root)?;
        }
        let receipt = PluginReceipt {
            plugin_id: package.manifest.id.clone(),
            version: package.manifest.version.clone(),
            display_name: package.manifest.display_name.clone(),
            package_digest: digest,
            package_relative_path: relative,
            source: source.description(),
            integrity: if publisher_is_trusted {
                IntegrityStatus::PublisherVerified
            } else {
                IntegrityStatus::LocalUnverified
            },
            requested_capabilities: sorted_capabilities(&package.manifest),
            granted_capabilities: options.grants.iter().cloned().collect(),
            native_helper_identities: helpers,
            publisher_identity: publisher,
            enabled: options.enable,
        };
        ensure_directory(&self.data_root().join(&receipt.plugin_id))?;
        self.crash(FaultPoint::PackageDurableBeforeJournal)?;
        let operation = Operation {
            format: 1,
            id: operation_id,
            kind: OperationKind::Install,
            phase: OperationPhase::PackagePublished,
            plugin_id: receipt.plugin_id.clone(),
            previous: previous.clone(),
            next: Some(receipt.clone()),
        };
        self.write_operation(&operation)?;
        self.crash(FaultPoint::PackagePublished)?;
        registry
            .plugins
            .retain(|item| item.plugin_id != receipt.plugin_id);
        registry.plugins.push(receipt.clone());
        self.write_registry(&registry)?;
        self.crash(FaultPoint::RegistryPublished)?;
        self.write_operation(&Operation {
            phase: OperationPhase::RegistryPublished,
            ..operation
        })?;
        self.recover()?;
        Ok(receipt)
    }

    /// Validates an archive through the same package, signature, native-helper
    /// and host checks as install, but stages outside the authority root and
    /// publishes no registry or operation bytes.
    pub fn inspect(&self, source: &PluginSource) -> Result<PluginInspection, PluginError> {
        let ordinal = OPERATION_ORDINAL.fetch_add(1, Ordering::Relaxed);
        let stage = std::env::temp_dir().join(format!(
            "tekes-plugin-inspect-{}-{ordinal}",
            std::process::id()
        ));
        source.stage(&stage)?;
        let result = self.inspect_staged(&stage);
        let cleanup = remove_tree(&stage);
        match (result, cleanup) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), _) => Err(error),
            (_, Err(error)) => Err(error),
        }
    }

    /// Copies a package into an authority-owned durable directory and returns
    /// the same validated inspection used by `inspect`. Retrying with an
    /// existing destination revalidates those immutable bytes rather than
    /// reopening the caller-owned source path.
    pub fn freeze_source(
        &self,
        source: &PluginSource,
        destination: &Path,
    ) -> Result<PluginInspection, PluginError> {
        if path_is_present(destination)? {
            require_directory(destination)?;
            return self.inspect_staged(destination);
        }
        if let Err(error) = source.stage(destination) {
            let _ = remove_tree(destination);
            return Err(error);
        }
        let result = self.inspect_staged(destination);
        if result.is_ok() {
            sync_tree(destination)?;
            let parent = destination
                .parent()
                .ok_or_else(|| PluginError::Storage("frozen source has no parent".to_owned()))?;
            sync_directory(parent)?;
        } else {
            let _ = remove_tree(destination);
        }
        result
    }

    fn inspect_staged(&self, stage: &Path) -> Result<PluginInspection, PluginError> {
        let package = self.load_package(stage)?;
        let _helpers = self.verify_native_helpers(&package)?;
        let publisher = verify_publisher(stage, || {
            package_digest(stage, Some(PUBLISHER_ATTESTATION_PATH))
        })?;
        let publisher_is_trusted = publisher.as_ref().is_some_and(|identity| {
            self.signature_policy
                .trusted_publishers
                .get(&identity.publisher_id)
                == Some(&identity.public_key_fingerprint)
        });
        if (publisher.is_some() && !publisher_is_trusted)
            || (publisher.is_none() && !self.signature_policy.allow_unsigned_local)
        {
            return Err(PluginError::PublisherTrustRequired(
                "package has no publisher key pinned by the trust policy".to_owned(),
            ));
        }
        Ok(PluginInspection {
            requested_capabilities: sorted_capabilities(&package.manifest),
            manifest: package.manifest,
            package_digest: package_digest(stage, None)?,
            integrity: if publisher_is_trusted {
                IntegrityStatus::PublisherVerified
            } else {
                IntegrityStatus::LocalUnverified
            },
        })
    }

    pub fn list(&mut self) -> Result<Vec<PluginReceipt>, PluginError> {
        self.recover()?;
        Ok(self.read_registry()?.plugins)
    }

    pub fn set_enabled(
        &mut self,
        plugin_id: &str,
        enabled: bool,
    ) -> Result<PluginReceipt, PluginError> {
        self.mutate_receipt(plugin_id, OperationKind::SetEnabled, |receipt| {
            if enabled {
                let grants = receipt
                    .granted_capabilities
                    .iter()
                    .cloned()
                    .collect::<BTreeSet<_>>();
                if let Some(missing) = receipt.requested_ids().difference(&grants).next() {
                    return Err(PluginError::MissingGrant(missing.clone()));
                }
            }
            receipt.enabled = enabled;
            Ok(())
        })
    }

    pub fn set_grants(
        &mut self,
        plugin_id: &str,
        grants: BTreeSet<String>,
    ) -> Result<PluginReceipt, PluginError> {
        self.mutate_receipt(plugin_id, OperationKind::SetGrants, |receipt| {
            let requested = receipt.requested_ids();
            if let Some(undeclared) = grants.difference(&requested).next() {
                return Err(PluginError::UndeclaredGrant(undeclared.clone()));
            }
            receipt.granted_capabilities = grants.iter().cloned().collect();
            if !requested.is_subset(&grants) {
                receipt.enabled = false;
            }
            Ok(())
        })
    }

    pub fn remove(&mut self, plugin_id: &str) -> Result<bool, PluginError> {
        self.recover()?;
        let mut registry = self.read_registry()?;
        let Some(previous) = registry
            .plugins
            .iter()
            .find(|receipt| receipt.plugin_id == plugin_id)
            .cloned()
        else {
            return Ok(false);
        };
        let operation = Operation {
            format: 1,
            id: operation_id(),
            kind: OperationKind::Remove,
            phase: OperationPhase::PackagePublished,
            plugin_id: plugin_id.to_owned(),
            previous: Some(previous),
            next: None,
        };
        self.write_operation(&operation)?;
        self.crash(FaultPoint::PackagePublished)?;
        registry
            .plugins
            .retain(|receipt| receipt.plugin_id != plugin_id);
        self.write_registry(&registry)?;
        self.crash(FaultPoint::RegistryPublished)?;
        self.write_operation(&Operation {
            phase: OperationPhase::RegistryPublished,
            ..operation
        })?;
        self.recover()?;
        Ok(true)
    }

    pub fn components(&mut self) -> Result<Vec<ComponentProjection>, PluginError> {
        self.recover()?;
        let mut projections = Vec::new();
        let mut owners = BTreeMap::<(ComponentKind, String), String>::new();
        for receipt in self.read_registry()?.plugins {
            if !receipt.enabled {
                continue;
            }
            self.verify_stored_receipt(&receipt)?;
            let package_root = self.root.join(&receipt.package_relative_path);
            let package = self.load_package(&package_root)?;
            for component in package.manifest.components {
                let key = (component.kind, component.id.clone());
                if let Some(first) = owners.insert(key.clone(), receipt.plugin_id.clone()) {
                    return Err(PluginError::ComponentCollision {
                        kind: key.0,
                        id: key.1,
                        first,
                        second: receipt.plugin_id,
                    });
                }
                projections.push(ComponentProjection {
                    owner_plugin_id: receipt.plugin_id.clone(),
                    owner_version: receipt.version.clone(),
                    component_id: component.id,
                    component_type: component.kind,
                    package_path: package_root.clone(),
                    component_path: package_root.join(component.path),
                    data_path: self.data_root().join(&receipt.plugin_id),
                    granted_capabilities: receipt.granted_capabilities.clone(),
                    launchable: false,
                });
            }
        }
        projections.sort_by(|left, right| {
            left.owner_plugin_id
                .cmp(&right.owner_plugin_id)
                .then_with(|| left.component_id.cmp(&right.component_id))
        });
        Ok(projections)
    }

    /// Resolves a plugin/component relation to the immutable executable in the
    /// currently enabled, fully granted package generation.
    ///
    /// `None` means that the relation is not currently runtime-eligible. This
    /// method performs no launch and contains no product-specific mapping.
    pub fn resolve_executable_component(
        &mut self,
        reference: &PluginComponentReference,
    ) -> Result<Option<ResolvedPluginExecutable>, PluginError> {
        self.recover()?;
        let registry = self.read_registry()?;
        let Some(receipt) = registry
            .plugins
            .iter()
            .find(|receipt| receipt.plugin_id == reference.plugin_id && receipt.enabled)
        else {
            return Ok(None);
        };
        self.verify_stored_receipt(receipt)?;
        let package_root = self.root.join(&receipt.package_relative_path);
        let package = self.load_package(&package_root)?;
        let Some(component) = package
            .manifest
            .components
            .iter()
            .find(|component| component.id == reference.component_id)
        else {
            return Ok(None);
        };
        let executable_path = package_root.join(&component.path);
        let metadata = fs::symlink_metadata(&executable_path)
            .map_err(|error| PluginError::Storage(error.to_string()))?;
        if !metadata.file_type().is_file() || metadata.permissions().mode() & 0o111 == 0 {
            return Ok(None);
        }
        let data_path = self.ensure_plugin_data_directory(&receipt.plugin_id)?;
        Ok(Some(ResolvedPluginExecutable {
            reference: reference.clone(),
            component_type: component.kind,
            executable_path,
            data_path,
            plugin_generation: receipt.package_digest.clone(),
            granted_capabilities: receipt.granted_capabilities.clone(),
        }))
    }

    pub fn recover(&mut self) -> Result<(), PluginError> {
        self.create_layout()?;
        let registry = self.read_registry()?;
        for receipt in &registry.plugins {
            self.verify_stored_receipt(receipt)?;
            self.ensure_plugin_data_directory(&receipt.plugin_id)?;
        }
        if path_is_present(&self.operation_path())? {
            let operation: Operation = read_canonical(&self.operation_path())?;
            validate_operation(&operation)?;
            let current = registry
                .plugins
                .iter()
                .find(|receipt| receipt.plugin_id == operation.plugin_id);
            let committed = current == operation.next.as_ref();
            if committed {
                if let Some(previous) = &operation.previous {
                    if operation.next.as_ref().is_none_or(|next| {
                        next.package_relative_path != previous.package_relative_path
                    }) {
                        remove_tree(&self.root.join(&previous.package_relative_path))?;
                    }
                }
                if operation.next.is_none() {
                    remove_tree(&self.data_root().join(&operation.plugin_id))?;
                }
            } else if let Some(next) = &operation.next {
                if operation.previous.as_ref().is_none_or(|previous| {
                    previous.package_relative_path != next.package_relative_path
                }) {
                    remove_tree(&self.root.join(&next.package_relative_path))?;
                }
            }
            fs::remove_file(self.operation_path())
                .map_err(|error| PluginError::Storage(error.to_string()))?;
            sync_directory(&self.operations_root())?;
        }
        clear_directory(&self.staging_root())?;
        self.cleanup_publication_temporaries()?;
        self.collect_orphans()?;
        for receipt in &self.read_registry()?.plugins {
            self.verify_stored_receipt(receipt)?;
        }
        Ok(())
    }

    fn mutate_receipt(
        &mut self,
        plugin_id: &str,
        kind: OperationKind,
        mutate: impl FnOnce(&mut PluginReceipt) -> Result<(), PluginError>,
    ) -> Result<PluginReceipt, PluginError> {
        self.recover()?;
        let mut registry = self.read_registry()?;
        let index = registry
            .plugins
            .iter()
            .position(|receipt| receipt.plugin_id == plugin_id)
            .ok_or_else(|| PluginError::NotInstalled(plugin_id.to_owned()))?;
        let previous = registry.plugins[index].clone();
        let mut next = previous.clone();
        mutate(&mut next)?;
        if next.enabled {
            self.verify_stored_receipt(&next)?;
        }
        let operation = Operation {
            format: 1,
            id: operation_id(),
            kind,
            phase: OperationPhase::PackagePublished,
            plugin_id: plugin_id.to_owned(),
            previous: Some(previous),
            next: Some(next.clone()),
        };
        self.write_operation(&operation)?;
        self.crash(FaultPoint::PackagePublished)?;
        registry.plugins[index] = next.clone();
        self.write_registry(&registry)?;
        self.crash(FaultPoint::RegistryPublished)?;
        self.write_operation(&Operation {
            phase: OperationPhase::RegistryPublished,
            ..operation
        })?;
        self.recover()?;
        Ok(next)
    }

    fn verify_stored_receipt(&self, receipt: &PluginReceipt) -> Result<(), PluginError> {
        let root = self.root.join(&receipt.package_relative_path);
        if package_digest(&root, None)? != receipt.package_digest {
            return Err(PluginError::CorruptRegistry(format!(
                "package digest changed for {}",
                receipt.plugin_id
            )));
        }
        let package = self.load_package(&root)?;
        let expected_capabilities = sorted_capabilities(&package.manifest);
        if package.manifest.id != receipt.plugin_id
            || package.manifest.version != receipt.version
            || package.manifest.display_name != receipt.display_name
            || expected_capabilities != receipt.requested_capabilities
        {
            return Err(PluginError::CorruptRegistry(format!(
                "manifest-derived receipt fields changed for {}",
                receipt.plugin_id
            )));
        }
        let grants = receipt
            .granted_capabilities
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        validate_grants(&grants, &package.manifest).map_err(|error| {
            PluginError::CorruptRegistry(format!(
                "receipt grants differ from manifest for {}: {error}",
                receipt.plugin_id
            ))
        })?;
        if receipt.enabled {
            require_all_grants(&grants, &package.manifest).map_err(|error| {
                PluginError::CorruptRegistry(format!(
                    "enabled receipt grants differ from manifest for {}: {error}",
                    receipt.plugin_id
                ))
            })?;
        }
        if self.verify_native_helpers(&package)? != receipt.native_helper_identities {
            return Err(PluginError::Signature(format!(
                "native helper identity changed for {}",
                receipt.plugin_id
            )));
        }
        let publisher = verify_publisher(&root, || {
            package_digest(&root, Some(PUBLISHER_ATTESTATION_PATH))
        })?;
        if publisher != receipt.publisher_identity {
            return Err(PluginError::Signature(format!(
                "publisher identity changed for {}",
                receipt.plugin_id
            )));
        }
        let publisher_is_trusted = publisher.as_ref().is_some_and(|identity| {
            self.signature_policy
                .trusted_publishers
                .get(&identity.publisher_id)
                == Some(&identity.public_key_fingerprint)
        });
        let expected_integrity = if publisher_is_trusted {
            IntegrityStatus::PublisherVerified
        } else {
            IntegrityStatus::LocalUnverified
        };
        if receipt.integrity != expected_integrity {
            return Err(PluginError::CorruptRegistry(format!(
                "receipt integrity differs from manifest trust for {}",
                receipt.plugin_id
            )));
        }
        if (!self.signature_policy.allow_unsigned_local
            || receipt.integrity == IntegrityStatus::PublisherVerified)
            && !publisher_is_trusted
        {
            return Err(PluginError::PublisherTrustRequired(format!(
                "publisher trust changed for {}",
                receipt.plugin_id
            )));
        }
        Ok(())
    }

    fn load_package(&self, root: &Path) -> Result<LoadedPackage, PluginError> {
        let manifest_path = root.join(PLUGIN_MANIFEST_FILE);
        let metadata = fs::symlink_metadata(&manifest_path)
            .map_err(|error| PluginError::InvalidManifest(error.to_string()))?;
        if !metadata.file_type().is_file() {
            return Err(PluginError::InvalidManifest(
                "tekes-plugin.json is not a regular file".to_owned(),
            ));
        }
        let manifest: Manifest = serde_json::from_slice(
            &fs::read(&manifest_path).map_err(|error| PluginError::Storage(error.to_string()))?,
        )
        .map_err(|error| PluginError::InvalidManifest(error.to_string()))?;
        manifest.validate(&self.host)?;
        for component in &manifest.components {
            let path = root.join(&component.path);
            if !path.starts_with(root) {
                return Err(PluginError::InvalidManifest(format!(
                    "component {} escapes package",
                    component.id
                )));
            }
            let metadata = fs::symlink_metadata(&path).map_err(|_| {
                PluginError::InvalidManifest(format!("missing component {}", component.id))
            })?;
            if metadata.file_type().is_symlink()
                || component.kind.expects_directory() != metadata.file_type().is_dir()
                || (!component.kind.expects_directory() && !metadata.file_type().is_file())
                || (component.kind == ComponentKind::NativeHelper
                    && metadata.permissions().mode() & 0o111 == 0)
                || (component.kind == ComponentKind::Skill && !path.join("SKILL.md").is_file())
            {
                return Err(PluginError::InvalidManifest(format!(
                    "component {} has wrong file-system kind",
                    component.id
                )));
            }
        }
        Ok(LoadedPackage {
            root: root.to_owned(),
            manifest,
        })
    }

    fn verify_native_helpers(
        &self,
        package: &LoadedPackage,
    ) -> Result<Vec<NativeHelperIdentity>, PluginError> {
        let mut identities = package
            .manifest
            .components
            .iter()
            .filter(|component| {
                component
                    .capabilities
                    .iter()
                    .any(|capability| capability == "native-helper.execute")
            })
            .map(|component| {
                self.verifier.verify(
                    &package.root.join(&component.path),
                    &component.id,
                    &component.path,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        identities.sort_by(|left, right| left.component_id.cmp(&right.component_id));
        Ok(identities)
    }

    fn create_layout(&self) -> Result<(), PluginError> {
        for path in [
            self.root.clone(),
            self.packages_root(),
            self.data_root(),
            self.staging_root(),
            self.operations_root(),
        ] {
            ensure_directory(&path)?;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o700))
                .map_err(|error| PluginError::Storage(error.to_string()))?;
            sync_directory(&path)?;
        }
        Ok(())
    }

    fn read_registry(&self) -> Result<Registry, PluginError> {
        if !path_is_present(&self.registry_path())? {
            return Ok(Registry::default());
        }
        let registry: Registry = read_canonical(&self.registry_path())?;
        if registry.format != 1 {
            return Err(PluginError::CorruptRegistry(
                "unsupported registry format".to_owned(),
            ));
        }
        let mut ids = BTreeSet::new();
        for receipt in &registry.plugins {
            validate_receipt(receipt)?;
            if !ids.insert(receipt.plugin_id.clone()) {
                return Err(PluginError::CorruptRegistry(format!(
                    "duplicate receipt {}",
                    receipt.plugin_id
                )));
            }
        }
        Ok(registry)
    }

    fn write_registry(&self, registry: &Registry) -> Result<(), PluginError> {
        let mut normalized = registry.clone();
        normalized
            .plugins
            .sort_by(|left, right| left.plugin_id.cmp(&right.plugin_id));
        write_canonical(&self.registry_path(), &normalized)
    }

    fn write_operation(&self, operation: &Operation) -> Result<(), PluginError> {
        write_canonical(&self.operation_path(), operation)
    }

    fn collect_orphans(&self) -> Result<(), PluginError> {
        let registry = self.read_registry()?;
        let referenced = registry
            .plugins
            .iter()
            .map(|receipt| self.root.join(&receipt.package_relative_path))
            .collect::<BTreeSet<_>>();
        for plugin in read_dirs(&self.packages_root())? {
            for version in read_dirs(&plugin)? {
                for digest in read_dirs(&version)? {
                    if !referenced.contains(&digest) {
                        remove_tree(&digest)?;
                    }
                }
                remove_empty(&version)?;
            }
            remove_empty(&plugin)?;
        }
        let installed = registry
            .plugins
            .iter()
            .map(|receipt| receipt.plugin_id.as_str())
            .collect::<BTreeSet<_>>();
        for data in read_dirs(&self.data_root())? {
            if data
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| !installed.contains(name))
            {
                remove_tree(&data)?;
            }
        }
        Ok(())
    }

    fn cleanup_publication_temporaries(&self) -> Result<(), PluginError> {
        cleanup_temporaries(&self.root, ".registry.canonical.json.")?;
        cleanup_temporaries(&self.operations_root(), ".current.canonical.json.")
    }

    fn ensure_plugin_data_directory(&self, plugin_id: &str) -> Result<PathBuf, PluginError> {
        let path = self.data_root().join(plugin_id);
        ensure_directory(&path)?;
        let directory = File::from(
            open(
                &path,
                OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                Mode::empty(),
            )
            .map_err(|error| {
                PluginError::Storage(format!(
                    "plugin data path is not a stable directory for {plugin_id}: {error}"
                ))
            })?,
        );
        fchmod(&directory, Mode::RWXU).map_err(|error| PluginError::Storage(error.to_string()))?;
        FullSync::full_sync(&directory).map_err(|error| PluginError::Storage(error.to_string()))?;
        Ok(path)
    }

    fn crash(&self, point: FaultPoint) -> Result<(), PluginError> {
        if self.fault == Some(point) {
            Err(PluginError::InjectedCrash(point))
        } else {
            Ok(())
        }
    }

    fn packages_root(&self) -> PathBuf {
        self.root.join("packages")
    }
    fn data_root(&self) -> PathBuf {
        self.root.join("data")
    }
    fn staging_root(&self) -> PathBuf {
        self.root.join(".staging")
    }
    fn operations_root(&self) -> PathBuf {
        self.root.join("operations")
    }
    fn operation_path(&self) -> PathBuf {
        self.operations_root().join("current.canonical.json")
    }
    fn registry_path(&self) -> PathBuf {
        self.root.join("registry.canonical.json")
    }
}

struct LoadedPackage {
    root: PathBuf,
    manifest: Manifest,
}

fn sorted_capabilities(manifest: &Manifest) -> Vec<crate::CapabilityRequest> {
    let mut values = manifest.capabilities.clone();
    values.sort_by(|left, right| left.id.cmp(&right.id));
    values
}

fn validate_operation(operation: &Operation) -> Result<(), PluginError> {
    if operation.format != 1
        || !crate::model::qualified_id(&operation.plugin_id, true)
        || operation.id.is_empty()
        || operation.id.len() > 256
    {
        return Err(PluginError::CorruptRegistry(
            "invalid operation identity or format".to_owned(),
        ));
    }
    for receipt in [operation.previous.as_ref(), operation.next.as_ref()]
        .into_iter()
        .flatten()
    {
        validate_receipt(receipt)?;
        if receipt.plugin_id != operation.plugin_id {
            return Err(PluginError::CorruptRegistry(
                "operation receipt has a different plugin id".to_owned(),
            ));
        }
    }
    let shape_is_valid = match operation.kind {
        OperationKind::Install => operation.next.is_some(),
        OperationKind::Remove => operation.previous.is_some() && operation.next.is_none(),
        OperationKind::SetEnabled => operation
            .previous
            .as_ref()
            .zip(operation.next.as_ref())
            .is_some_and(|(previous, next)| {
                let mut expected = previous.clone();
                expected.enabled = next.enabled;
                expected == *next
            }),
        OperationKind::SetGrants => operation
            .previous
            .as_ref()
            .zip(operation.next.as_ref())
            .is_some_and(|(previous, next)| {
                let mut expected = previous.clone();
                expected.granted_capabilities = next.granted_capabilities.clone();
                expected.enabled = next.enabled;
                expected == *next
            }),
    };
    if !shape_is_valid {
        return Err(PluginError::CorruptRegistry(
            "operation mutation shape is invalid".to_owned(),
        ));
    }
    Ok(())
}

fn validate_receipt(receipt: &PluginReceipt) -> Result<(), PluginError> {
    let requested = receipt.requested_ids();
    let grants = receipt
        .granted_capabilities
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let expected_path = format!(
        "packages/{}/{}/{}",
        receipt.plugin_id, receipt.version, receipt.package_digest
    );
    let requested_is_sorted = receipt
        .requested_capabilities
        .windows(2)
        .all(|pair| pair[0].id < pair[1].id);
    let grants_are_sorted = receipt
        .granted_capabilities
        .windows(2)
        .all(|pair| pair[0] < pair[1]);
    let helpers_are_sorted = receipt
        .native_helper_identities
        .windows(2)
        .all(|pair| pair[0].component_id < pair[1].component_id);
    let publisher_is_consistent = match receipt.integrity {
        IntegrityStatus::LocalUnverified => receipt.publisher_identity.is_none(),
        IntegrityStatus::PublisherVerified => receipt.publisher_identity.is_some(),
    };
    let valid = crate::model::qualified_id(&receipt.plugin_id, true)
        && !receipt.display_name.trim().is_empty()
        && receipt.display_name.chars().count() <= 128
        && !receipt.source.is_empty()
        && is_lower_hex_digest(&receipt.package_digest)
        && crate::model::safe_relative(&receipt.package_relative_path)
        && receipt.package_relative_path == expected_path
        && requested.len() == receipt.requested_capabilities.len()
        && requested_is_sorted
        && receipt.requested_capabilities.iter().all(|capability| {
            crate::model::qualified_id(&capability.id, true)
                && capability
                    .reason
                    .as_ref()
                    .is_none_or(|reason| !reason.trim().is_empty() && reason.chars().count() <= 512)
        })
        && grants.len() == receipt.granted_capabilities.len()
        && grants_are_sorted
        && grants.is_subset(&requested)
        && (!receipt.enabled || requested.is_subset(&grants))
        && helpers_are_sorted
        && receipt.native_helper_identities.iter().all(|identity| {
            crate::model::qualified_id(&identity.component_id, false)
                && crate::model::safe_relative(&identity.relative_path)
                && !identity.designated_requirement.is_empty()
                && !identity.signing_identifier.is_empty()
        })
        && publisher_is_consistent
        && receipt.publisher_identity.as_ref().is_none_or(|publisher| {
            !publisher.publisher_id.is_empty()
                && is_lower_hex_digest(&publisher.public_key_fingerprint)
                && is_lower_hex_digest(&publisher.signed_content_digest)
        });
    if valid {
        Ok(())
    } else {
        Err(PluginError::CorruptRegistry(format!(
            "invalid receipt {}",
            receipt.plugin_id
        )))
    }
}

fn is_lower_hex_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn validate_grants(grants: &BTreeSet<String>, manifest: &Manifest) -> Result<(), PluginError> {
    let requested = manifest
        .capabilities
        .iter()
        .map(|capability| capability.id.clone())
        .collect::<BTreeSet<_>>();
    if let Some(grant) = grants.difference(&requested).next() {
        Err(PluginError::UndeclaredGrant(grant.clone()))
    } else {
        Ok(())
    }
}

fn require_all_grants(grants: &BTreeSet<String>, manifest: &Manifest) -> Result<(), PluginError> {
    let requested = manifest
        .capabilities
        .iter()
        .map(|capability| capability.id.clone())
        .collect::<BTreeSet<_>>();
    if let Some(missing) = requested.difference(grants).next() {
        Err(PluginError::MissingGrant(missing.clone()))
    } else {
        Ok(())
    }
}

fn package_digest(root: &Path, excluded: Option<&str>) -> Result<String, PluginError> {
    let mut entries = vec![(".".to_owned(), root.to_owned())];
    collect_entries(root, root, &mut entries)?;
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    let mut hasher = Sha256::new();
    for (relative, path) in entries {
        if excluded == Some(relative.as_str()) {
            continue;
        }
        let metadata =
            fs::symlink_metadata(&path).map_err(|error| PluginError::Storage(error.to_string()))?;
        if metadata.file_type().is_dir() {
            hash_fields(&mut hasher, &["directory", &relative]);
        } else if metadata.file_type().is_file() {
            let executable = if metadata.permissions().mode() & 0o111 == 0 {
                "0"
            } else {
                "1"
            };
            hash_fields(&mut hasher, &["file", &relative, executable]);
            hash_value(
                &mut hasher,
                &fs::read(&path).map_err(|error| PluginError::Storage(error.to_string()))?,
            );
        } else {
            return Err(PluginError::InvalidArchive(format!(
                "unsupported package entry {relative}"
            )));
        }
    }
    Ok(hex(&hasher.finalize()))
}

fn collect_entries(
    root: &Path,
    current: &Path,
    entries: &mut Vec<(String, PathBuf)>,
) -> Result<(), PluginError> {
    for entry in fs::read_dir(current).map_err(|error| PluginError::Storage(error.to_string()))? {
        let entry = entry.map_err(|error| PluginError::Storage(error.to_string()))?;
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .map_err(|error| PluginError::Storage(error.to_string()))?
            .to_string_lossy()
            .to_string();
        let metadata =
            fs::symlink_metadata(&path).map_err(|error| PluginError::Storage(error.to_string()))?;
        if metadata.file_type().is_symlink() {
            return Err(PluginError::InvalidArchive(format!(
                "symbolic links are not supported: {relative}"
            )));
        }
        entries.push((relative, path.clone()));
        if metadata.file_type().is_dir() {
            collect_entries(root, &path, entries)?;
        }
    }
    Ok(())
}

fn hash_fields(hasher: &mut Sha256, fields: &[&str]) {
    for field in fields {
        hash_value(hasher, field.as_bytes());
    }
}

fn hash_value(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update(u64::try_from(bytes.len()).unwrap_or(u64::MAX).to_be_bytes());
    hasher.update(bytes);
}

fn write_canonical(path: &Path, value: &impl Serialize) -> Result<(), PluginError> {
    let mut bytes = serde_json_canonicalizer::to_vec(value)
        .map_err(|error| PluginError::Storage(error.to_string()))?;
    bytes.push(b'\n');
    let parent = path
        .parent()
        .ok_or_else(|| PluginError::Storage("publication has no parent".to_owned()))?;
    ensure_directory(parent)?;
    let temporary = parent.join(format!(
        ".{}.{}.tmp",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("plugin"),
        operation_id()
    ));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&temporary)
        .map_err(|error| PluginError::Storage(error.to_string()))?;
    file.write_all(&bytes)
        .map_err(|error| PluginError::Storage(error.to_string()))?;
    FullSync::full_sync(&file).map_err(|error| PluginError::Storage(error.to_string()))?;
    fs::rename(&temporary, path).map_err(|error| PluginError::Storage(error.to_string()))?;
    sync_directory(parent)
}

fn read_canonical<T: DeserializeOwned + Serialize>(path: &Path) -> Result<T, PluginError> {
    let metadata =
        fs::symlink_metadata(path).map_err(|error| PluginError::Storage(error.to_string()))?;
    if !metadata.file_type().is_file() {
        return Err(PluginError::CorruptRegistry(format!(
            "{} is not a regular file",
            path.display()
        )));
    }
    let bytes = fs::read(path).map_err(|error| PluginError::Storage(error.to_string()))?;
    let value = serde_json::from_slice(&bytes)
        .map_err(|error| PluginError::CorruptRegistry(error.to_string()))?;
    let mut canonical = serde_json_canonicalizer::to_vec(&value)
        .map_err(|error| PluginError::CorruptRegistry(error.to_string()))?;
    canonical.push(b'\n');
    if canonical != bytes {
        return Err(PluginError::CorruptRegistry(format!(
            "{} is not canonical JSON plus LF",
            path.display()
        )));
    }
    Ok(value)
}

fn operation_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let ordinal = OPERATION_ORDINAL.fetch_add(1, Ordering::Relaxed);
    format!("{}-{nanos}-{ordinal}", std::process::id())
}

fn seal_tree(root: &Path) -> Result<(), PluginError> {
    let mut entries = Vec::new();
    collect_paths(root, &mut entries)?;
    entries.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    for path in entries {
        let metadata =
            fs::symlink_metadata(&path).map_err(|error| PluginError::Storage(error.to_string()))?;
        let mode = if metadata.file_type().is_dir() {
            0o555
        } else if metadata.permissions().mode() & 0o111 == 0 {
            0o444
        } else {
            0o555
        };
        fs::set_permissions(path, fs::Permissions::from_mode(mode))
            .map_err(|error| PluginError::Storage(error.to_string()))?;
    }
    Ok(())
}

fn sync_tree(root: &Path) -> Result<(), PluginError> {
    let mut entries = Vec::new();
    collect_paths(root, &mut entries)?;
    entries.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    for path in entries {
        let entry = File::open(path).map_err(|error| PluginError::Storage(error.to_string()))?;
        FullSync::full_sync(&entry).map_err(|error| PluginError::Storage(error.to_string()))?;
    }
    Ok(())
}

fn collect_paths(root: &Path, output: &mut Vec<PathBuf>) -> Result<(), PluginError> {
    let metadata =
        fs::symlink_metadata(root).map_err(|error| PluginError::Storage(error.to_string()))?;
    if metadata.file_type().is_symlink() {
        return Err(PluginError::Storage(format!(
            "managed tree contains a symbolic link: {}",
            root.display()
        )));
    }
    output.push(root.to_owned());
    if metadata.file_type().is_dir() {
        for entry in fs::read_dir(root).map_err(|error| PluginError::Storage(error.to_string()))? {
            collect_paths(
                &entry
                    .map_err(|error| PluginError::Storage(error.to_string()))?
                    .path(),
                output,
            )?;
        }
    }
    Ok(())
}

fn remove_tree(path: &Path) -> Result<(), PluginError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            fs::remove_file(path).map_err(|error| PluginError::Storage(error.to_string()))?;
            if let Some(parent) = path.parent() {
                sync_directory(parent)?;
            }
            return Ok(());
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(PluginError::Storage(error.to_string())),
    }
    let mut paths = Vec::new();
    collect_paths(path, &mut paths)?;
    for item in &paths {
        let metadata =
            fs::symlink_metadata(item).map_err(|error| PluginError::Storage(error.to_string()))?;
        if !metadata.file_type().is_symlink() {
            fs::set_permissions(
                item,
                fs::Permissions::from_mode(if metadata.file_type().is_dir() {
                    0o700
                } else {
                    0o600
                }),
            )
            .map_err(|error| PluginError::Storage(error.to_string()))?;
        }
    }
    fs::remove_dir_all(path).map_err(|error| PluginError::Storage(error.to_string()))?;
    if let Some(parent) = path.parent() {
        sync_directory(parent)?;
    }
    Ok(())
}

fn clear_directory(path: &Path) -> Result<(), PluginError> {
    for entry in fs::read_dir(path).map_err(|error| PluginError::Storage(error.to_string()))? {
        remove_tree(
            &entry
                .map_err(|error| PluginError::Storage(error.to_string()))?
                .path(),
        )?;
    }
    sync_directory(path)
}

fn cleanup_temporaries(path: &Path, prefix: &str) -> Result<(), PluginError> {
    let mut removed = false;
    for entry in fs::read_dir(path).map_err(|error| PluginError::Storage(error.to_string()))? {
        let entry = entry.map_err(|error| PluginError::Storage(error.to_string()))?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if !name.starts_with(prefix) || !name.ends_with(".tmp") {
            continue;
        }
        let metadata = fs::symlink_metadata(entry.path())
            .map_err(|error| PluginError::Storage(error.to_string()))?;
        if !metadata.file_type().is_file() {
            return Err(PluginError::Storage(format!(
                "publication temporary is not a regular file: {}",
                entry.path().display()
            )));
        }
        fs::remove_file(entry.path()).map_err(|error| PluginError::Storage(error.to_string()))?;
        removed = true;
    }
    if removed {
        sync_directory(path)?;
    }
    Ok(())
}

fn read_dirs(path: &Path) -> Result<Vec<PathBuf>, PluginError> {
    let mut values = Vec::new();
    for entry in fs::read_dir(path).map_err(|error| PluginError::Storage(error.to_string()))? {
        let path = entry
            .map_err(|error| PluginError::Storage(error.to_string()))?
            .path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| PluginError::Storage(error.to_string()))?
            .file_type();
        if !metadata.is_dir() {
            return Err(PluginError::Storage(format!(
                "managed directory contains a non-directory entry: {}",
                path.display()
            )));
        }
        values.push(path);
    }
    values.sort();
    Ok(values)
}

fn path_is_present(path: &Path) -> Result<bool, PluginError> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(PluginError::Storage(error.to_string())),
    }
}

fn require_directory(path: &Path) -> Result<(), PluginError> {
    let metadata =
        fs::symlink_metadata(path).map_err(|error| PluginError::Storage(error.to_string()))?;
    if metadata.file_type().is_dir() {
        Ok(())
    } else {
        Err(PluginError::Storage(format!(
            "managed path is not a directory: {}",
            path.display()
        )))
    }
}

fn ensure_directory(path: &Path) -> Result<(), PluginError> {
    match fs::create_dir(path) {
        Ok(()) => {
            fs::set_permissions(path, fs::Permissions::from_mode(0o700))
                .map_err(|error| PluginError::Storage(error.to_string()))?;
            sync_directory(path)?;
            if let Some(parent) = path.parent() {
                sync_directory(parent)?;
            }
            Ok(())
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => require_directory(path),
        Err(error) => Err(PluginError::Storage(error.to_string())),
    }
}

fn remove_empty(path: &Path) -> Result<(), PluginError> {
    if fs::read_dir(path)
        .map_err(|error| PluginError::Storage(error.to_string()))?
        .next()
        .is_none()
    {
        fs::remove_dir(path).map_err(|error| PluginError::Storage(error.to_string()))?;
        if let Some(parent) = path.parent() {
            sync_directory(parent)?;
        }
    }
    Ok(())
}

fn sync_directory(path: &Path) -> Result<(), PluginError> {
    let directory = File::open(path).map_err(|error| PluginError::Storage(error.to_string()))?;
    FullSync::full_sync(&directory).map_err(|error| PluginError::Storage(error.to_string()))
}
