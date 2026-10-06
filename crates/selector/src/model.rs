use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Selection {
    pub manifest_sha256: String,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionFile {
    pub format: u8,
    pub generation: u64,
    pub selection: Selection,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PreviousFile {
    pub format: u8,
    pub generation: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<Selection>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Compatibility {
    pub authority_registry_sha256: String,
    pub reader_profile: String,
    pub writer_profile: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Signing {
    pub requirement: String,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BundleFile {
    pub bytes: u64,
    pub mode: String,
    pub path: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BundleManifest {
    pub architectures: Vec<String>,
    pub compatibility: Compatibility,
    pub files: Vec<BundleFile>,
    pub format: u8,
    pub minimum_os: String,
    pub signing: Signing,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectorFile {
    pub bytes: u64,
    pub mode: String,
    pub path: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectorManifest {
    pub architecture: String,
    pub conformance_sha256: String,
    pub file: SelectorFile,
    pub format: u8,
    pub minimum_os: String,
    pub signing: Signing,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallIdentity {
    pub access_group: String,
    pub client_requirement: String,
    pub format: u8,
    pub installer_requirement: String,
    pub selector_requirement: String,
    pub supervisor_requirement: String,
    pub team_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ObservationState {
    Pending,
    Ready,
    Failed,
    Promoted,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub attempt: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_deadline_at: Option<String>,
    pub canary_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_run: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_session: Option<String>,
    pub consecutive_failures: u64,
    pub deadline_at: String,
    pub format: u8,
    pub generation: u64,
    pub last_code: String,
    pub launch_id: String,
    pub manifest_sha256: String,
    pub rollback_eligible: bool,
    pub started_at: String,
    pub state: ObservationState,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_closes_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OperationActor {
    Cli,
    Serve,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OperationType {
    Stage,
    Activate,
    Rollback,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectorOperation {
    pub actor: OperationActor,
    pub command_sha256: String,
    pub format: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Selection>,
    pub generation: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_id: Option<String>,
    pub op_id: String,
    pub phase: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Value>,
    pub to: Selection,
    #[serde(rename = "type")]
    pub operation_type: OperationType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StageReply {
    pub format: u8,
    pub operation: String,
    pub selection: Selection,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PruneReply {
    pub format: u8,
    pub operation: String,
    pub removed: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MutationReply {
    pub current: SelectionFile,
    pub format: u8,
    pub operation: String,
    pub previous: PreviousFile,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RecoverReply {
    pub format: u8,
    pub operation: String,
    pub recovered: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<SelectionFile>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Failure {
    pub code: String,
    pub launch_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrelaunchFailure {
    pub code: String,
    pub format: u8,
    pub generation: u64,
    pub manifest_sha256: String,
    pub observed_at: String,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StatusReply {
    pub format: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure: Option<Failure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation: Option<Observation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prelaunch_failure: Option<PrelaunchFailure>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<PreviousFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<SelectionFile>,
    pub service: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CanaryReply {
    pub format: u8,
    pub operation: String,
    pub run: String,
    pub session: String,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InstallHealthReply {
    pub format: u8,
    pub operation: String,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateReply {
    pub format: u8,
    pub operation: String,
    pub sha256: String,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConformanceReply {
    pub architecture: String,
    pub conformance_sha256: String,
    pub format: u8,
    pub operation: String,
    pub version: String,
}
