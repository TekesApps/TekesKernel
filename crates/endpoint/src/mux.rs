use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::history::HistoryError;
use crate::types::EndpointTypeError;
use crate::{
    EndpointJournal, EndpointSubscription, EndpointSubscriptionHub, HistoryPage, JournalRecord,
    SessionEvent, SessionHistoryEntry, SessionToolEventView, history_page, validate_session_id,
};

pub const SESSION_ENDPOINT_PROTOCOL_VERSION: u16 = 3;
pub const DEFAULT_JOURNAL_WINDOW_MESSAGES: usize = 50;
pub const MAX_JOURNAL_WINDOW_MESSAGES: usize = 500;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SessionEndpointCapability {
    WorkspaceSync,
    SessionInventorySync,
    SessionJournal,
    SessionControlSync,
    ActionableSync,
    Models,
    Attachments,
    Management,
}

impl SessionEndpointCapability {
    #[must_use]
    pub fn required() -> BTreeSet<Self> {
        [
            Self::WorkspaceSync,
            Self::SessionInventorySync,
            Self::SessionJournal,
            Self::SessionControlSync,
            Self::ActionableSync,
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MuxHostProduct {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MuxHostDescription {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: u16,
    pub product: MuxHostProduct,
    pub capabilities: BTreeSet<SessionEndpointCapability>,
    pub cwd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "attachedSessions")]
    pub attached_sessions: usize,
    pub home: String,
    #[serde(rename = "canOpenPath")]
    pub can_open_path: bool,
}

impl MuxHostDescription {
    pub fn validate(&self) -> Result<(), MuxProtocolError> {
        if self.protocol_version != SESSION_ENDPOINT_PROTOCOL_VERSION {
            return Err(MuxProtocolError::ProtocolVersion(self.protocol_version));
        }
        if self.product.name.is_empty()
            || self.product.version.is_empty()
            || self.cwd.is_empty()
            || self.home.is_empty()
        {
            return Err(MuxProtocolError::Description);
        }
        let missing = SessionEndpointCapability::required()
            .difference(&self.capabilities)
            .copied()
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            return Err(MuxProtocolError::MissingCapabilities(missing));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SessionErrorCategory {
    NotFound,
    Conflict,
    Busy,
    InvalidRequest,
    Unsupported,
    Cancelled,
    Transport,
    Internal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Retryability {
    Never,
    AfterReconnect,
    AfterDelay,
    Reconcile,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionRemoteError {
    pub category: SessionErrorCategory,
    pub code: String,
    pub message: String,
    pub details: IJsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retryability: Option<Retryability>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionAddress {
    #[serde(rename = "sessionId")]
    pub session_id: String,
}

impl SessionAddress {
    pub fn validate(&self) -> Result<(), MuxProtocolError> {
        validate_session_id(&self.session_id).map_err(MuxProtocolError::Session)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum SessionStreamTarget {
    Workspace,
    SessionInventory,
    SessionJournal {
        address: SessionAddress,
        #[serde(rename = "maxMessages")]
        max_messages: usize,
    },
    SessionControl,
    Actionables,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
pub enum SessionMuxClientFrame {
    Open {
        #[serde(rename = "streamId")]
        stream_id: String,
        target: SessionStreamTarget,
    },
    Close {
        #[serde(rename = "streamId")]
        stream_id: String,
    },
    JournalPage {
        #[serde(rename = "requestId")]
        request_id: String,
        address: SessionAddress,
        #[serde(rename = "throughSequence")]
        through_sequence: i64,
        #[serde(rename = "beforeSequence", skip_serializing_if = "Option::is_none")]
        before_sequence: Option<i64>,
        #[serde(rename = "maxMessages")]
        max_messages: usize,
    },
    ActionableRespond {
        #[serde(rename = "requestId")]
        request_id: String,
        #[serde(rename = "actionableId")]
        actionable_id: String,
        #[serde(rename = "expectedRevision")]
        expected_revision: u64,
        outcome: IJsonValue,
    },
}

impl SessionMuxClientFrame {
    pub fn validate(&self) -> Result<(), MuxProtocolError> {
        match self {
            Self::Open { stream_id, target } => {
                validate_identifier(stream_id)?;
                if let SessionStreamTarget::SessionJournal {
                    address,
                    max_messages,
                } = target
                {
                    address.validate()?;
                    validate_window(*max_messages)?;
                }
            }
            Self::Close { stream_id } => validate_identifier(stream_id)?,
            Self::JournalPage {
                request_id,
                address,
                through_sequence,
                before_sequence,
                max_messages,
            } => {
                validate_identifier(request_id)?;
                address.validate()?;
                validate_window(*max_messages)?;
                if *through_sequence < -1
                    || before_sequence
                        .is_some_and(|before| before < -1 || before > *through_sequence)
                {
                    return Err(MuxProtocolError::JournalCut);
                }
            }
            Self::ActionableRespond {
                request_id,
                actionable_id,
                expected_revision,
                ..
            } => {
                validate_identifier(request_id)?;
                validate_identifier(actionable_id)?;
                let _ = expected_revision;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceSummary {
    pub id: String,
    pub path: String,
    pub title: String,
    #[serde(rename = "sessionIds")]
    pub session_ids: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceBaseline {
    pub items: Vec<WorkspaceSummary>,
    #[serde(rename = "archivedSessionIds")]
    pub archived_session_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionSummary {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
    /// `tail == "running"`; kept for readers that only need liveness.
    pub running: bool,
    /// The classified spec/tail-lifecycle state: `running`, `stopped_active`,
    /// `answered_hold`, `parked_hold`, `recovery_needed`, `unstarted`, or
    /// `settled`. The only wire fact that separates a parked hold from a run.
    pub tail: String,
    pub blank: bool,
    pub cwd: String,
    #[serde(rename = "parentSessionId", skip_serializing_if = "Option::is_none")]
    pub parent_session_id: Option<String>,
    /// `"fork"` exactly when `parentSessionId` is present (spec inventory rules).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// A scratch ledger (`session.fork{ephemeral:true}`): hidden from durable
    /// navigation by clients, refused by archive, removed by `session.discard`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub ephemeral: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projections: Option<IJsonValue>,
    /// The session's durable permission mode (`read-only`, `workspace-write`,
    /// `danger-full-access`). Always present on Kernel inventory; a Client
    /// treats absence as "host does not report it".
    #[serde(
        rename = "permissionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_mode: Option<String>,
    /// Immutable resolved opening profile; absent while automatic selection is pending.
    #[serde(
        rename = "identityProfile",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_profile: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionJournalSnapshot {
    #[serde(default = "default_window_limit", skip_serializing)]
    pub window_limit: usize,
    pub address: SessionAddress,
    #[serde(rename = "throughSequence")]
    pub through_sequence: i64,
    pub entries: Vec<SessionHistoryEntry>,
    #[serde(rename = "hasMoreBefore")]
    pub has_more_before: bool,
    pub projections: IJsonValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionJournalPage {
    pub address: SessionAddress,
    #[serde(rename = "throughSequence")]
    pub through_sequence: i64,
    pub entries: Vec<SessionHistoryEntry>,
    #[serde(rename = "hasMoreBefore")]
    pub has_more_before: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionControlItem {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub queue: Vec<IJsonValue>,
    pub jobs: Vec<IJsonValue>,
    pub projections: IJsonValue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SessionActionableKind {
    Approval,
    Question,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionActionable {
    pub id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub revision: u64,
    pub kind: SessionActionableKind,
    pub payload: IJsonValue,
}

impl SessionActionable {
    pub fn validate(&self) -> Result<(), MuxProtocolError> {
        validate_identifier(&self.id)?;
        validate_session_id(&self.session_id).map_err(MuxProtocolError::Session)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
pub enum SessionSyncFrame {
    WorkspaceBaseline {
        generation: u64,
        baseline: WorkspaceBaseline,
    },
    WorkspaceUpsert {
        generation: u64,
        workspace: WorkspaceSummary,
    },
    WorkspaceRemove {
        generation: u64,
        #[serde(rename = "workspaceId")]
        workspace_id: String,
    },
    WorkspaceOrder {
        generation: u64,
        #[serde(rename = "workspaceIds")]
        workspace_ids: Vec<String>,
    },
    ArchivedSessions {
        generation: u64,
        #[serde(rename = "sessionIds")]
        session_ids: Vec<String>,
    },
    InventoryBaseline {
        generation: u64,
        items: Vec<SessionSummary>,
    },
    InventoryUpsert {
        generation: u64,
        session: SessionSummary,
    },
    InventoryRemove {
        generation: u64,
        #[serde(rename = "sessionId")]
        session_id: String,
    },
    JournalSnapshot {
        generation: u64,
        snapshot: SessionJournalSnapshot,
    },
    JournalEvent {
        generation: u64,
        address: SessionAddress,
        event: SessionEvent,
        #[serde(skip_serializing_if = "Option::is_none")]
        view: Option<SessionToolEventView>,
    },
    JournalTransient {
        generation: u64,
        address: SessionAddress,
        event: SessionEvent,
    },
    JournalProjection {
        generation: u64,
        address: SessionAddress,
        key: String,
        value: IJsonValue,
        sequence: u64,
    },
    ControlBaseline {
        generation: u64,
        items: Vec<SessionControlItem>,
    },
    ControlUpsert {
        generation: u64,
        control: SessionControlItem,
    },
    ActionableBaseline {
        generation: u64,
        items: Vec<SessionActionable>,
    },
    ActionableUpsert {
        generation: u64,
        actionable: SessionActionable,
    },
    ActionableResolved {
        generation: u64,
        id: String,
        revision: u64,
    },
}

impl SessionSyncFrame {
    #[must_use]
    pub const fn generation(&self) -> u64 {
        match self {
            Self::WorkspaceBaseline { generation, .. }
            | Self::WorkspaceUpsert { generation, .. }
            | Self::WorkspaceRemove { generation, .. }
            | Self::WorkspaceOrder { generation, .. }
            | Self::ArchivedSessions { generation, .. }
            | Self::InventoryBaseline { generation, .. }
            | Self::InventoryUpsert { generation, .. }
            | Self::InventoryRemove { generation, .. }
            | Self::JournalSnapshot { generation, .. }
            | Self::JournalEvent { generation, .. }
            | Self::JournalTransient { generation, .. }
            | Self::JournalProjection { generation, .. }
            | Self::ControlBaseline { generation, .. }
            | Self::ControlUpsert { generation, .. }
            | Self::ActionableBaseline { generation, .. }
            | Self::ActionableUpsert { generation, .. }
            | Self::ActionableResolved { generation, .. } => *generation,
        }
    }

    #[must_use]
    pub const fn is_baseline(&self) -> bool {
        matches!(
            self,
            Self::WorkspaceBaseline { .. }
                | Self::InventoryBaseline { .. }
                | Self::JournalSnapshot { .. }
                | Self::ControlBaseline { .. }
                | Self::ActionableBaseline { .. }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
pub enum SessionMuxServerFrame {
    Ready {
        generation: u64,
        host: MuxHostDescription,
    },
    Stream {
        #[serde(rename = "streamId")]
        stream_id: String,
        frame: SessionSyncFrame,
    },
    JournalPageResult {
        #[serde(rename = "requestId")]
        request_id: String,
        page: SessionJournalPage,
    },
    ActionableResponseResult {
        #[serde(rename = "requestId")]
        request_id: String,
        #[serde(rename = "actionableId")]
        actionable_id: String,
        revision: u64,
    },
    Error {
        #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
        request_id: Option<String>,
        #[serde(rename = "streamId", skip_serializing_if = "Option::is_none")]
        stream_id: Option<String>,
        error: SessionRemoteError,
    },
}

/// Connection-scoped validator. It deliberately stores no Workspace, Session,
/// journal, control, or actionable business snapshot.
#[derive(Debug)]
pub struct SessionMuxGeneration {
    generation: u64,
    streams: BTreeMap<String, StreamState>,
}

#[derive(Clone, Debug)]
struct StreamState {
    kind: MuxStreamKind,
    baselined: bool,
    journal_tail: Option<i64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MuxStreamKind {
    Workspace,
    Inventory,
    Journal,
    Control,
    Actionables,
}

impl SessionMuxGeneration {
    pub fn new(generation: u64) -> Result<Self, MuxProtocolError> {
        if generation == 0 {
            return Err(MuxProtocolError::Generation);
        }
        Ok(Self {
            generation,
            streams: BTreeMap::new(),
        })
    }

    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub fn open(
        &mut self,
        stream_id: impl Into<String>,
        target: &SessionStreamTarget,
    ) -> Result<(), MuxProtocolError> {
        let stream_id = stream_id.into();
        validate_identifier(&stream_id)?;
        if self.streams.contains_key(&stream_id) {
            return Err(MuxProtocolError::DuplicateStream(stream_id));
        }
        let kind = match target {
            SessionStreamTarget::Workspace => MuxStreamKind::Workspace,
            SessionStreamTarget::SessionInventory => MuxStreamKind::Inventory,
            SessionStreamTarget::SessionJournal {
                address,
                max_messages,
            } => {
                address.validate()?;
                validate_window(*max_messages)?;
                MuxStreamKind::Journal
            }
            SessionStreamTarget::SessionControl => MuxStreamKind::Control,
            SessionStreamTarget::Actionables => MuxStreamKind::Actionables,
        };
        self.streams.insert(
            stream_id,
            StreamState {
                kind,
                baselined: false,
                journal_tail: None,
            },
        );
        Ok(())
    }

    pub fn close(&mut self, stream_id: &str) -> Result<(), MuxProtocolError> {
        self.streams
            .remove(stream_id)
            .ok_or_else(|| MuxProtocolError::UnknownStream(stream_id.to_owned()))?;
        Ok(())
    }

    pub fn accept(
        &mut self,
        stream_id: &str,
        frame: &SessionSyncFrame,
    ) -> Result<(), MuxProtocolError> {
        if frame.generation() != self.generation {
            return Err(MuxProtocolError::StaleGeneration {
                expected: self.generation,
                actual: frame.generation(),
            });
        }
        let state = self
            .streams
            .get_mut(stream_id)
            .ok_or_else(|| MuxProtocolError::UnknownStream(stream_id.to_owned()))?;
        if !state.baselined && !frame.is_baseline() {
            return Err(MuxProtocolError::BaselineRequired(stream_id.to_owned()));
        }
        if state.baselined && frame.is_baseline() {
            return Err(MuxProtocolError::DuplicateBaseline(stream_id.to_owned()));
        }
        let actual_kind = frame_kind(frame);
        if actual_kind != state.kind {
            return Err(MuxProtocolError::StreamKind(stream_id.to_owned()));
        }
        match frame {
            SessionSyncFrame::JournalSnapshot { snapshot, .. } => {
                snapshot.address.validate()?;
                validate_journal_entries(snapshot.through_sequence, &snapshot.entries)?;
                state.journal_tail = Some(snapshot.through_sequence);
            }
            SessionSyncFrame::JournalEvent { address, event, .. } => {
                address.validate()?;
                event.validate().map_err(MuxProtocolError::SessionEvent)?;
                let expected = state
                    .journal_tail
                    .ok_or_else(|| MuxProtocolError::BaselineRequired(stream_id.to_owned()))?
                    .checked_add(1)
                    .ok_or(MuxProtocolError::JournalCut)?;
                if i64::try_from(event.seq).ok() != Some(expected) {
                    return Err(MuxProtocolError::JournalSequence {
                        expected,
                        actual: event.seq,
                    });
                }
                state.journal_tail = Some(expected);
            }
            SessionSyncFrame::JournalTransient { address, event, .. } => {
                address.validate()?;
                event.validate().map_err(MuxProtocolError::SessionEvent)?;
                if event.event_type != "assistant/chunk" {
                    return Err(MuxProtocolError::StreamKind(stream_id.to_owned()));
                }
            }
            SessionSyncFrame::ActionableBaseline { items, .. } => {
                for actionable in items {
                    actionable.validate()?;
                }
            }
            SessionSyncFrame::ActionableUpsert { actionable, .. } => {
                actionable.validate()?;
            }
            SessionSyncFrame::ActionableResolved { id, revision, .. } => {
                validate_identifier(id)?;
                let _ = revision;
            }
            _ => {}
        }
        state.baselined = true;
        Ok(())
    }
}

pub struct JournalFollow {
    pub snapshot: SessionJournalSnapshot,
    pub subscription: EndpointSubscription,
}

pub fn open_journal_follow(
    hub: &EndpointSubscriptionHub,
    address: SessionAddress,
    journal: &EndpointJournal,
    projections: IJsonValue,
    max_messages: usize,
    subscribed_rpc_id: impl Into<String>,
) -> Result<JournalFollow, MuxProtocolError> {
    address.validate()?;
    validate_window(max_messages)?;
    // subscribe() snapshots the durable tail while excluding hub publishers.
    // Reading the frozen page afterwards cannot create a live gap because
    // every event after that cut is already queued in the subscription.
    let subscription = hub
        .subscribe(&address.session_id, journal, subscribed_rpc_id.into())
        .map_err(MuxProtocolError::Hub)?;
    let through_sequence = subscription.baseline().last_seq;
    let page = frozen_history_page(journal, through_sequence, None, max_messages)?;
    Ok(JournalFollow {
        snapshot: SessionJournalSnapshot {
            window_limit: max_messages,
            address,
            through_sequence,
            entries: page.events,
            has_more_before: page.has_more,
            projections,
        },
        subscription,
    })
}

pub fn frozen_history_page(
    journal: &EndpointJournal,
    through_sequence: i64,
    before_sequence: Option<i64>,
    max_messages: usize,
) -> Result<HistoryPage, MuxProtocolError> {
    validate_window(max_messages)?;
    if through_sequence < -1
        || before_sequence.is_some_and(|before| before < -1 || before > through_sequence)
    {
        return Err(MuxProtocolError::JournalCut);
    }
    let records = journal.records().map_err(MuxProtocolError::Journal)?;
    let current_tail = records
        .last()
        .map(|record| record.event.seq as i64)
        .unwrap_or(-1);
    if through_sequence > current_tail {
        return Err(MuxProtocolError::JournalCut);
    }
    let frozen = records
        .into_iter()
        .filter(|record| record.event.seq as i64 <= through_sequence)
        .collect::<Vec<JournalRecord>>();
    history_page(
        &frozen,
        before_sequence.map(|before| before.max(0) as u64),
        Some(max_messages),
    )
    .map_err(MuxProtocolError::History)
}

#[derive(Debug, Default)]
pub struct ActionableRegistry {
    pending: Mutex<BTreeMap<String, SessionActionable>>,
}

impl ActionableRegistry {
    pub fn upsert(&self, actionable: SessionActionable) -> Result<(), MuxProtocolError> {
        actionable.validate()?;
        let mut pending = self
            .pending
            .lock()
            .map_err(|_| MuxProtocolError::Poisoned)?;
        match pending.get(&actionable.id) {
            Some(current) if current == &actionable => Ok(()),
            Some(current) if actionable.revision <= current.revision => {
                Err(MuxProtocolError::StaleActionable {
                    id: actionable.id,
                    expected: current.revision,
                    actual: actionable.revision,
                })
            }
            Some(_) => {
                pending.insert(actionable.id.clone(), actionable);
                Ok(())
            }
            None => {
                pending.insert(actionable.id.clone(), actionable);
                Ok(())
            }
        }
    }

    pub fn baseline(&self) -> Result<Vec<SessionActionable>, MuxProtocolError> {
        Ok(self
            .pending
            .lock()
            .map_err(|_| MuxProtocolError::Poisoned)?
            .values()
            .cloned()
            .collect())
    }

    pub fn resolve(
        &self,
        actionable_id: &str,
        expected_revision: u64,
    ) -> Result<SessionActionable, MuxProtocolError> {
        validate_identifier(actionable_id)?;
        let mut pending = self
            .pending
            .lock()
            .map_err(|_| MuxProtocolError::Poisoned)?;
        let actionable = pending
            .get(actionable_id)
            .ok_or_else(|| MuxProtocolError::UnknownActionable(actionable_id.to_owned()))?;
        if actionable.revision != expected_revision {
            return Err(MuxProtocolError::StaleActionable {
                id: actionable_id.to_owned(),
                expected: actionable.revision,
                actual: expected_revision,
            });
        }
        pending
            .remove(actionable_id)
            .ok_or_else(|| MuxProtocolError::UnknownActionable(actionable_id.to_owned()))
    }
}

fn frame_kind(frame: &SessionSyncFrame) -> MuxStreamKind {
    match frame {
        SessionSyncFrame::WorkspaceBaseline { .. }
        | SessionSyncFrame::WorkspaceUpsert { .. }
        | SessionSyncFrame::WorkspaceRemove { .. }
        | SessionSyncFrame::WorkspaceOrder { .. }
        | SessionSyncFrame::ArchivedSessions { .. } => MuxStreamKind::Workspace,
        SessionSyncFrame::InventoryBaseline { .. }
        | SessionSyncFrame::InventoryUpsert { .. }
        | SessionSyncFrame::InventoryRemove { .. } => MuxStreamKind::Inventory,
        SessionSyncFrame::JournalSnapshot { .. }
        | SessionSyncFrame::JournalEvent { .. }
        | SessionSyncFrame::JournalTransient { .. }
        | SessionSyncFrame::JournalProjection { .. } => MuxStreamKind::Journal,
        SessionSyncFrame::ControlBaseline { .. } | SessionSyncFrame::ControlUpsert { .. } => {
            MuxStreamKind::Control
        }
        SessionSyncFrame::ActionableBaseline { .. }
        | SessionSyncFrame::ActionableUpsert { .. }
        | SessionSyncFrame::ActionableResolved { .. } => MuxStreamKind::Actionables,
    }
}

fn validate_identifier(value: &str) -> Result<(), MuxProtocolError> {
    if value.is_empty() || value.len() > 128 || value.chars().any(char::is_whitespace) {
        return Err(MuxProtocolError::Identifier);
    }
    Ok(())
}

fn validate_window(max_messages: usize) -> Result<(), MuxProtocolError> {
    if !(1..=MAX_JOURNAL_WINDOW_MESSAGES).contains(&max_messages) {
        return Err(MuxProtocolError::JournalWindow(max_messages));
    }
    Ok(())
}

fn validate_journal_entries(
    through_sequence: i64,
    entries: &[SessionHistoryEntry],
) -> Result<(), MuxProtocolError> {
    if through_sequence < -1 {
        return Err(MuxProtocolError::JournalCut);
    }
    let mut previous = None;
    for entry in entries {
        entry
            .event
            .validate()
            .map_err(MuxProtocolError::SessionEvent)?;
        if entry.event.seq as i64 > through_sequence
            || previous.is_some_and(|value| value >= entry.event.seq)
        {
            return Err(MuxProtocolError::JournalCut);
        }
        previous = Some(entry.event.seq);
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum MuxProtocolError {
    #[error("unsupported Session Endpoint protocol version {0}")]
    ProtocolVersion(u16),
    #[error("Session Endpoint host description is incomplete")]
    Description,
    #[error("Session Endpoint host is missing required capabilities: {0:?}")]
    MissingCapabilities(Vec<SessionEndpointCapability>),
    #[error("connection generation must be positive")]
    Generation,
    #[error("identifier is empty, too long, or contains whitespace")]
    Identifier,
    #[error("stream {0} is already open")]
    DuplicateStream(String),
    #[error("stream {0} is not open")]
    UnknownStream(String),
    #[error("stream {0} must begin with its baseline")]
    BaselineRequired(String),
    #[error("stream {0} received more than one baseline")]
    DuplicateBaseline(String),
    #[error("stream {0} received a frame for another semantic stream")]
    StreamKind(String),
    #[error("frame belongs to generation {actual}, expected {expected}")]
    StaleGeneration { expected: u64, actual: u64 },
    #[error("journal cut is invalid")]
    JournalCut,
    #[error("journal window {0} is outside the supported range")]
    JournalWindow(usize),
    #[error("journal sequence gap: expected {expected}, received {actual}")]
    JournalSequence { expected: i64, actual: u64 },
    #[error("actionable {id} revision {actual} does not match {expected}")]
    StaleActionable {
        id: String,
        expected: u64,
        actual: u64,
    },
    #[error("actionable {0} is no longer pending")]
    UnknownActionable(String),
    #[error("V3 registry mutex was poisoned")]
    Poisoned,
    #[error("session identity failed: {0}")]
    Session(#[source] EndpointTypeError),
    #[error("session event failed: {0}")]
    SessionEvent(#[source] EndpointTypeError),
    #[error("subscription failed: {0}")]
    Hub(#[source] crate::HubError),
    #[error("endpoint journal failed: {0}")]
    Journal(#[source] crate::JournalError),
    #[error("history page failed: {0}")]
    History(#[source] HistoryError),
}

fn default_window_limit() -> usize {
    DEFAULT_JOURNAL_WINDOW_MESSAGES
}
