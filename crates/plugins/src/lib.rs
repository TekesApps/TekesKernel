//! Slice-12 plugin package carriage and lifecycle authority.
//!
//! This crate deliberately has no process-launching dependency. An enabled component is a
//! validated projection, not an executable registration; Slice 13/14 own runtime activation.

mod archive;
mod model;
mod signature;
mod store;

pub use archive::{PluginArchive, PluginSource};
pub use model::{
    CapabilityRequest, Component, ComponentKind, ComponentProjection, HostEnvironment, Manifest,
    OperatingSystem, PlatformRequirement, PluginComponentReference, PluginVersion,
    ResolvedPluginExecutable,
};
pub use signature::{
    MacOsNativeHelperVerifier, NativeHelperIdentity, NativeHelperVerifier, PublisherIdentity,
    SignaturePolicy,
};
pub use store::{
    FaultPoint, InstallOptions, IntegrityStatus, ManagementRequest, ManagementResult, PluginError,
    PluginInspection, PluginReceipt, PluginStore,
};

pub const PLUGIN_MANIFEST_FILE: &str = "tekes-plugin.json";
pub const PLUGIN_ARCHIVE_EXTENSION: &str = "tekesplugin";
