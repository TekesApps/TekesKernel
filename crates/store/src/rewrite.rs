use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};

use schema::{Event, EventKind, OriginTuple, Visibility, validate_ledger};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::asset::hex_digest;
use crate::{AssetStore, AtomicPublisher, DirectoryLock, NamedLock, StoreError, ThreadStore};

const OP_FORMAT: u64 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RewriteKind {
    Fork,
    Redact,
}

impl RewriteKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Fork => "fork",
            Self::Redact => "redact",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RewritePhase {
    Prepared,
    Built,
    Validated,
    Published,
    SourceRetired,
    SourceRemoved,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RewriteEventRef {
    pub file: String,
    pub seq: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RewriteFileMap {
    pub source: String,
    pub dest: String,
    pub thread: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RedactionInventory {
    pub events: Vec<RewriteEventRef>,
    pub assets: Vec<String>,
    pub fingerprints: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RewriteOperation {
    pub format: u64,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: RewriteKind,
    pub source: String,
    pub dest: String,
    pub created_at: String,
    pub source_digest: String,
    pub phase: RewritePhase,
    pub files: Vec<RewriteFileMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_anchor: Option<RewriteEventRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub genesis_origin: Option<OriginTuple>,
    /// A fork whose destination is a scratch ledger: its genesis carries
    /// `ephemeral: true`, startup sweeps it, and `discard_ephemeral` removes it.
    /// Recorded on the operation so crash recovery rebuilds the same genesis.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub ephemeral: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redaction: Option<RedactionInventory>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForkGenesisBinding {
    pub thread: String,
    pub origin: OriginTuple,
    pub ephemeral: bool,
}

impl RewriteOperation {
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self, StoreError> {
        let canonical = bytes.strip_suffix(b"\n").ok_or_else(|| {
            StoreError::RewriteOperationCorrupt("record must end in one LF".to_owned())
        })?;
        let operation: Self = serde_json::from_slice(canonical).map_err(|error| {
            StoreError::RewriteOperationCorrupt(format!("invalid JSON: {error}"))
        })?;
        if serde_json_canonicalizer::to_vec(&operation)
            .map_err(|error| StoreError::RewriteOperationCorrupt(error.to_string()))?
            != canonical
        {
            return Err(StoreError::RewriteOperationCorrupt(
                "record is not canonical".to_owned(),
            ));
        }
        validate_operation(&operation)
            .map_err(|error| StoreError::RewriteOperationCorrupt(error.to_string()))?;
        Ok(operation)
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, StoreError> {
        operation_bytes(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RewriteProgress {
    pub operation_id: String,
    pub phase: Option<RewritePhase>,
}

#[derive(Debug)]
struct BuiltFile {
    bytes: Vec<u8>,
    seq_map: BTreeMap<u64, u64>,
}

struct GenesisContext<'a> {
    source_file: &'a str,
    mapped: &'a RewriteFileMap,
    operation: &'a RewriteOperation,
    file_map: &'a BTreeMap<String, RewriteFileMap>,
    built_parents: &'a BTreeMap<String, BuiltFile>,
    poisoned_assets: &'a BTreeSet<String>,
    minima: (u64, u64),
}

enum RewriteMode<'a> {
    Fork(Option<(u64, OriginTuple, bool)>),
    Redact(&'a [Vec<u8>]),
}

impl ThreadStore {
    pub fn begin_fork(
        &self,
        operation_id: &str,
        source: &str,
        destination: &str,
        created_at: &str,
    ) -> Result<RewriteOperation, StoreError> {
        self.begin_rewrite(
            operation_id,
            source,
            destination,
            created_at,
            RewriteMode::Fork(None),
        )
    }

    /// Starts a fork from a complete, terminal prefix of `main.jsonl`.
    /// `binding` fixes both the new durable UUID and its keyed genesis author;
    /// neither value is synthesized during crash recovery.
    pub fn begin_fork_at_kernel_anchor(
        &self,
        operation_id: &str,
        source: &str,
        kernel_anchor: u64,
        binding: ForkGenesisBinding,
        created_at: &str,
    ) -> Result<RewriteOperation, StoreError> {
        if kernel_anchor == 0 {
            return Err(StoreError::InvalidForkAnchor(kernel_anchor));
        }
        self.begin_rewrite(
            operation_id,
            source,
            &binding.thread,
            created_at,
            RewriteMode::Fork(Some((kernel_anchor, binding.origin, binding.ephemeral))),
        )
    }

    pub fn begin_redact(
        &self,
        operation_id: &str,
        source: &str,
        destination: &str,
        created_at: &str,
        forbidden: &[Vec<u8>],
    ) -> Result<RewriteOperation, StoreError> {
        if forbidden.is_empty() || forbidden.iter().any(Vec::is_empty) {
            return Err(StoreError::Corruption(
                "redact requires non-empty forbidden byte strings".to_owned(),
            ));
        }
        self.begin_rewrite(
            operation_id,
            source,
            destination,
            created_at,
            RewriteMode::Redact(forbidden),
        )
    }

    fn begin_rewrite(
        &self,
        operation_id: &str,
        source: &str,
        destination: &str,
        created_at: &str,
        mode: RewriteMode<'_>,
    ) -> Result<RewriteOperation, StoreError> {
        let (kind, forbidden, fork_anchor) = match mode {
            RewriteMode::Fork(anchor) => (RewriteKind::Fork, &[][..], anchor),
            RewriteMode::Redact(forbidden) => (RewriteKind::Redact, forbidden, None),
        };
        let _rewrite_namespace = NamedLock::exclusive(self.root().join(".rewrite.lock"))?;
        let _catalog = NamedLock::exclusive(self.root().join(".thread-catalog.lock"))?;
        validate_path_id(operation_id, "operation id")?;
        validate_uuid(source, "source")?;
        validate_uuid(destination, "destination")?;
        if source == destination {
            return Err(StoreError::RewriteDestinationExists);
        }
        if self.source_has_live_rewrite(source)? {
            return Err(StoreError::RewriteInProgress);
        }
        if self.destination_has_live_rewrite(destination)? {
            return Err(StoreError::RewriteDestinationExists);
        }
        for area in ["threads", "archive"] {
            if self.root().join(area).join(destination).exists() {
                return Err(StoreError::RewriteDestinationExists);
            }
        }
        if fs::read_dir(self.root().join(".rewrite-trash"))?.any(|entry| {
            entry.is_ok_and(|entry| entry.file_name().to_string_lossy().ends_with(destination))
        }) {
            return Err(StoreError::RewriteDestinationExists);
        }
        let source_folder = self.root().join("threads").join(source);
        if !source_folder.is_dir() {
            return Err(StoreError::NotFound);
        }
        let _lifecycle = DirectoryLock::exclusive(&source_folder)?;
        let ledgers = read_source_ledgers(&source_folder)?;
        let materialized_ledgers = match fork_anchor.as_ref() {
            Some((anchor, _, _)) => source_prefix_ledgers(&ledgers, *anchor)?,
            None => ledgers.clone(),
        };
        validate_terminal_source(&materialized_ledgers)?;
        let source_digest = tree_digest(&ledgers);
        let files = build_file_map(&materialized_ledgers, destination)?;
        let redaction = match kind {
            RewriteKind::Fork => None,
            RewriteKind::Redact => Some(scan_redaction(&source_folder, &ledgers, forbidden)?),
        };
        let operation = RewriteOperation {
            format: OP_FORMAT,
            id: operation_id.to_owned(),
            kind,
            source: source.to_owned(),
            dest: destination.to_owned(),
            created_at: created_at.to_owned(),
            source_digest,
            phase: RewritePhase::Prepared,
            files,
            source_anchor: fork_anchor.as_ref().map(|(seq, _, _)| RewriteEventRef {
                file: "main.jsonl".to_owned(),
                seq: *seq,
            }),
            ephemeral: fork_anchor
                .as_ref()
                .is_some_and(|(_, _, ephemeral)| *ephemeral),
            genesis_origin: fork_anchor.map(|(_, origin, _)| origin),
            redaction,
        };
        validate_operation(&operation)?;
        // Deterministic projection errors must reject before `prepared`;
        // otherwise an operation with no legal next phase would own the
        // source forever. Asset corruption remains fail-stop and repairable.
        project_ledgers(&materialized_ledgers, &operation)?;
        let stage = self.root().join("staging").join(operation_id);
        fs::create_dir(&stage)?;
        write_operation(&stage, &operation)?;
        File::open(self.root().join("staging"))?.sync_all()?;
        Ok(operation)
    }

    pub fn advance_rewrite(&self, operation_id: &str) -> Result<RewriteProgress, StoreError> {
        let _rewrite_namespace = NamedLock::exclusive(self.root().join(".rewrite.lock"))?;
        let _catalog = NamedLock::exclusive(self.root().join(".thread-catalog.lock"))?;
        validate_path_id(operation_id, "operation id")?;
        let stage = self.root().join("staging").join(operation_id);
        let mut operation = read_operation(&stage)?;
        if operation.id != operation_id {
            return Err(StoreError::Corruption(
                "operation directory/id mismatch".to_owned(),
            ));
        }
        let source = self.root().join("threads").join(&operation.source);
        let tombstone = self.tombstone_path(&operation);
        let _lifecycle = if source.is_dir() {
            Some(DirectoryLock::exclusive(&source)?)
        } else if tombstone.is_dir() {
            Some(DirectoryLock::exclusive(&tombstone)?)
        } else {
            None
        };

        match operation.phase {
            RewritePhase::Prepared => {
                verify_source_digest(&source, &operation.source_digest)?;
                build_payload(self.root(), &stage, &operation)?;
                operation.phase = RewritePhase::Built;
                write_operation(&stage, &operation)?;
                progress(&operation)
            }
            RewritePhase::Built => {
                verify_source_digest(&source, &operation.source_digest)?;
                validate_payload(self.root(), &stage, &operation)?;
                operation.phase = RewritePhase::Validated;
                write_operation(&stage, &operation)?;
                progress(&operation)
            }
            RewritePhase::Validated => {
                verify_source_digest(&source, &operation.source_digest)?;
                let payload = stage.join("payload");
                let destination = self.root().join("threads").join(&operation.dest);
                match (payload.is_dir(), destination.is_dir()) {
                    (true, false) => {
                        fs::rename(&payload, &destination)?;
                        File::open(self.root().join("threads"))?.sync_all()?;
                    }
                    (false, true) => {}
                    _ => {
                        return Err(StoreError::Corruption(
                            "rewrite publication has ambiguous payload/destination state"
                                .to_owned(),
                        ));
                    }
                }
                operation.phase = RewritePhase::Published;
                write_operation(&stage, &operation)?;
                progress(&operation)
            }
            RewritePhase::Published if operation.kind == RewriteKind::Fork => {
                verify_source_digest(&source, &operation.source_digest)?;
                close_operation(self.root(), &stage)?;
                Ok(RewriteProgress {
                    operation_id: operation.id,
                    phase: None,
                })
            }
            RewritePhase::Published => {
                match (source.is_dir(), tombstone.is_dir()) {
                    (true, false) => {
                        verify_source_digest(&source, &operation.source_digest)?;
                        fs::rename(&source, &tombstone)?;
                        File::open(self.root().join("threads"))?.sync_all()?;
                        File::open(self.root().join(".rewrite-trash"))?.sync_all()?;
                    }
                    (false, true) => {}
                    _ => {
                        return Err(StoreError::Corruption(
                            "redact retirement has ambiguous source/tombstone state".to_owned(),
                        ));
                    }
                }
                verify_source_digest(&tombstone, &operation.source_digest)?;
                operation.phase = RewritePhase::SourceRetired;
                write_operation(&stage, &operation)?;
                progress(&operation)
            }
            RewritePhase::SourceRetired if operation.kind == RewriteKind::Redact => {
                if tombstone.exists() {
                    verify_source_digest(&tombstone, &operation.source_digest)?;
                    fs::remove_dir_all(&tombstone)?;
                    File::open(self.root().join(".rewrite-trash"))?.sync_all()?;
                }
                operation.phase = RewritePhase::SourceRemoved;
                write_operation(&stage, &operation)?;
                progress(&operation)
            }
            RewritePhase::SourceRemoved if operation.kind == RewriteKind::Redact => {
                if source.exists() || tombstone.exists() {
                    return Err(StoreError::Corruption(
                        "redact source reappeared after removal".to_owned(),
                    ));
                }
                close_operation(self.root(), &stage)?;
                Ok(RewriteProgress {
                    operation_id: operation.id,
                    phase: None,
                })
            }
            RewritePhase::SourceRetired | RewritePhase::SourceRemoved => Err(
                StoreError::Corruption("fork entered a redact-only phase".to_owned()),
            ),
        }
    }

    pub fn recover_rewrites(&self) -> Result<Vec<RewriteProgress>, StoreError> {
        self.recover_rewrites_impl(false)
    }

    /// An unpublished fork cannot damage its source. Preserve failed fork staging for
    /// diagnosis while allowing unrelated sessions to start. Redactions remain strict.
    pub fn recover_rewrites_for_startup(&self) -> Result<Vec<RewriteProgress>, StoreError> {
        self.recover_rewrites_impl(true)
    }

    fn recover_rewrites_impl(
        &self,
        isolate_failed_forks: bool,
    ) -> Result<Vec<RewriteProgress>, StoreError> {
        let mut ids = Vec::new();
        for entry in fs::read_dir(self.root().join("staging"))? {
            let entry = entry?;
            if entry.file_type()?.is_dir() && entry.path().join("op.json").is_file() {
                ids.push(entry.file_name().to_string_lossy().into_owned());
            }
        }
        ids.sort();
        let mut completed = Vec::new();
        for id in ids {
            loop {
                let value = match self.advance_rewrite(&id) {
                    Ok(value) => value,
                    Err(error @ StoreError::RewriteCarrierCorrupt(_)) if isolate_failed_forks => {
                        if !self.quarantine_unpublished_fork(&id, &error)? {
                            return Err(error);
                        }
                        break;
                    }
                    Err(error) => return Err(error),
                };
                let closed = value.phase.is_none();
                if closed {
                    completed.push(value);
                    break;
                }
            }
        }
        Ok(completed)
    }

    fn quarantine_unpublished_fork(
        &self,
        id: &str,
        error: &StoreError,
    ) -> Result<bool, StoreError> {
        let _rewrite_namespace = NamedLock::exclusive(self.root().join(".rewrite.lock"))?;
        let _catalog = NamedLock::exclusive(self.root().join(".thread-catalog.lock"))?;
        let stage = self.root().join("staging").join(id);
        let operation = read_operation(&stage)?;
        if operation.id != id
            || operation.kind != RewriteKind::Fork
            || operation.phase >= RewritePhase::Validated
            || self.root().join("threads").join(&operation.dest).exists()
        {
            return Ok(false);
        }
        let source = self.root().join("threads").join(&operation.source);
        let _lifecycle = DirectoryLock::exclusive(&source)?;
        verify_source_digest(&source, &operation.source_digest)?;
        let quarantine = self.root().join(".rewrite-quarantine");
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(&quarantine)?;
        File::open(self.root())?.sync_all()?;
        let destination = quarantine.join(id);
        if destination.exists() {
            return Ok(false);
        }
        let diagnostic = stage.join("recovery-error.txt");
        fs::write(&diagnostic, format!("{error}\n"))?;
        File::open(diagnostic)?.sync_all()?;
        fs::rename(&stage, &destination)?;
        File::open(quarantine)?.sync_all()?;
        File::open(self.root().join("staging"))?.sync_all()?;
        eprintln!(
            "rewrite recovery: preserved failed unpublished fork {id} in {}: {error}",
            destination.display()
        );
        Ok(true)
    }

    pub fn gc_rewrite_debris(&self) -> Result<Vec<PathBuf>, StoreError> {
        let _rewrite_namespace = NamedLock::exclusive(self.root().join(".rewrite.lock"))?;
        let staging = self.root().join("staging");
        let mut removed = Vec::new();
        for entry in fs::read_dir(&staging)? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() && path.join("op.json").is_file() {
                continue;
            }
            if entry.file_type()?.is_dir() {
                fs::remove_dir_all(&path)?;
            } else {
                fs::remove_file(&path)?;
            }
            removed.push(path);
        }
        if !removed.is_empty() {
            File::open(staging)?.sync_all()?;
        }
        Ok(removed)
    }

    pub(crate) fn source_has_live_rewrite(&self, thread_id: &str) -> Result<bool, StoreError> {
        for entry in fs::read_dir(self.root().join("staging"))? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() || !entry.path().join("op.json").is_file() {
                continue;
            }
            if read_operation(&entry.path())?.source == thread_id {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(crate) fn destination_has_live_rewrite(&self, thread_id: &str) -> Result<bool, StoreError> {
        for entry in fs::read_dir(self.root().join("staging"))? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() || !entry.path().join("op.json").is_file() {
                continue;
            }
            if read_operation(&entry.path())?.dest == thread_id {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn tombstone_path(&self, operation: &RewriteOperation) -> PathBuf {
        self.root()
            .join(".rewrite-trash")
            .join(format!("{}-{}", operation.id, operation.source))
    }
}

fn progress(operation: &RewriteOperation) -> Result<RewriteProgress, StoreError> {
    Ok(RewriteProgress {
        operation_id: operation.id.clone(),
        phase: Some(operation.phase),
    })
}

fn validate_path_id(value: &str, name: &str) -> Result<(), StoreError> {
    if value.is_empty()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(StoreError::Corruption(format!("invalid {name}")));
    }
    Ok(())
}

fn validate_uuid(value: &str, name: &str) -> Result<(), StoreError> {
    let parsed = Uuid::parse_str(value)
        .map_err(|_| StoreError::Corruption(format!("invalid {name} UUID")))?;
    if parsed.hyphenated().to_string() != value {
        return Err(StoreError::Corruption(format!(
            "{name} UUID is not canonical"
        )));
    }
    Ok(())
}

fn validate_operation(operation: &RewriteOperation) -> Result<(), StoreError> {
    if operation.format != OP_FORMAT {
        return Err(StoreError::Corruption(
            "rewrite operation format must equal 1".to_owned(),
        ));
    }
    validate_path_id(&operation.id, "operation id")?;
    validate_uuid(&operation.source, "source")?;
    validate_uuid(&operation.dest, "destination")?;
    let timestamp_probe = json!({
        "v": 1, "seq": 2, "kind": "meta", "ts": operation.created_at,
        "title": "probe"
    });
    Event::decode_canonical(
        &serde_json_canonicalizer::to_vec(&timestamp_probe)
            .map_err(|error| StoreError::Corruption(error.to_string()))?,
    )?;
    if operation
        .source_digest
        .strip_prefix("sha256-")
        .is_none_or(|digest| {
            digest.len() != 64
                || !digest
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        })
    {
        return Err(StoreError::Corruption(
            "invalid rewrite source digest".to_owned(),
        ));
    }
    if (operation.kind == RewriteKind::Redact) != operation.redaction.is_some() {
        return Err(StoreError::Corruption(
            "redaction inventory/type mismatch".to_owned(),
        ));
    }
    match (
        operation.kind,
        operation.source_anchor.as_ref(),
        operation.genesis_origin.as_ref(),
    ) {
        (RewriteKind::Fork, None, None) => {}
        (RewriteKind::Fork, Some(anchor), Some(origin))
            if anchor.file == "main.jsonl"
                && anchor.seq > 0
                && origin.target == operation.dest
                && [
                    origin.principal.as_str(),
                    origin.client.as_str(),
                    origin.target.as_str(),
                    origin.op.as_str(),
                    origin.key.as_str(),
                ]
                .iter()
                .all(|value| !value.is_empty()) => {}
        (RewriteKind::Fork, Some(_), Some(_)) => {
            return Err(StoreError::Corruption(
                "invalid anchored-fork genesis binding".to_owned(),
            ));
        }
        (RewriteKind::Fork, _, _) => {
            return Err(StoreError::Corruption(
                "fork anchor and genesis origin must appear together".to_owned(),
            ));
        }
        (RewriteKind::Redact, None, None) => {}
        (RewriteKind::Redact, _, _) => {
            return Err(StoreError::Corruption(
                "redact cannot carry a fork anchor or genesis binding".to_owned(),
            ));
        }
    }
    if operation.ephemeral
        && (operation.kind != RewriteKind::Fork || operation.genesis_origin.is_none())
    {
        return Err(StoreError::Corruption(
            "only an anchored fork can publish an ephemeral destination".to_owned(),
        ));
    }
    if operation.files.is_empty()
        || operation.files[0].source != "main.jsonl"
        || operation.files[0].dest != "main.jsonl"
        || operation.files[0].thread != operation.dest
    {
        return Err(StoreError::Corruption(
            "rewrite file map lacks canonical main entry".to_owned(),
        ));
    }
    let mut source_names = BTreeSet::new();
    let mut dest_names = BTreeSet::new();
    let mut threads = BTreeSet::new();
    for file in &operation.files {
        if !source_names.insert(&file.source)
            || !dest_names.insert(&file.dest)
            || !threads.insert(&file.thread)
            || Path::new(&file.source).file_name().and_then(|v| v.to_str())
                != Some(file.source.as_str())
            || Path::new(&file.dest).file_name().and_then(|v| v.to_str())
                != Some(file.dest.as_str())
        {
            return Err(StoreError::Corruption(
                "rewrite file map is not one-to-one".to_owned(),
            ));
        }
        validate_uuid(&file.thread, "mapped thread")?;
    }
    if operation.files[1..]
        .windows(2)
        .any(|pair| pair[0].source >= pair[1].source)
    {
        return Err(StoreError::Corruption(
            "rewrite child file map must be source-sorted".to_owned(),
        ));
    }
    if operation.kind == RewriteKind::Fork
        && matches!(
            operation.phase,
            RewritePhase::SourceRetired | RewritePhase::SourceRemoved
        )
    {
        return Err(StoreError::Corruption(
            "fork operation entered a redact-only phase".to_owned(),
        ));
    }
    if let Some(redaction) = &operation.redaction {
        if redaction.events.windows(2).any(|pair| pair[0] >= pair[1])
            || redaction.assets.windows(2).any(|pair| pair[0] >= pair[1])
            || redaction
                .fingerprints
                .windows(2)
                .any(|pair| pair[0] >= pair[1])
        {
            return Err(StoreError::Corruption(
                "redaction inventory must be sorted and unique".to_owned(),
            ));
        }
        if redaction.assets.iter().any(|name| !valid_asset_name(name))
            || redaction
                .fingerprints
                .iter()
                .any(|name| !valid_asset_name(name))
            || redaction
                .events
                .iter()
                .any(|event| !source_names.contains(&event.file) || event.seq == 0)
        {
            return Err(StoreError::Corruption(
                "redaction inventory contains an invalid carrier".to_owned(),
            ));
        }
    }
    Ok(())
}

fn read_source_ledgers(folder: &Path) -> Result<BTreeMap<String, Vec<u8>>, StoreError> {
    let mut ledgers = BTreeMap::new();
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        if !entry.file_type()?.is_file()
            || entry.path().extension().and_then(|value| value.to_str()) != Some("jsonl")
        {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "endpoint.jsonl" {
            continue;
        }
        let bytes = fs::read(entry.path())?;
        let projection = validate_ledger(&bytes, 1)?;
        if projection.last_seq == 0 {
            return Err(StoreError::Corruption("empty rewrite ledger".to_owned()));
        }
        crate::tail::check_write_versions(&projection.events, 1)?;
        ledgers.insert(name, bytes);
    }
    if !ledgers.contains_key("main.jsonl") {
        return Err(StoreError::Corruption(
            "rewrite source lacks main.jsonl".to_owned(),
        ));
    }
    Ok(ledgers)
}

fn source_ledgers_for_operation(
    ledgers: &BTreeMap<String, Vec<u8>>,
    operation: &RewriteOperation,
) -> Result<BTreeMap<String, Vec<u8>>, StoreError> {
    match operation.source_anchor.as_ref() {
        Some(anchor) if anchor.file == "main.jsonl" => source_prefix_ledgers(ledgers, anchor.seq),
        Some(_) => Err(StoreError::Corruption(
            "v1 fork anchor must name main.jsonl".to_owned(),
        )),
        None => Ok(ledgers.clone()),
    }
}

fn source_prefix_ledgers(
    ledgers: &BTreeMap<String, Vec<u8>>,
    anchor: u64,
) -> Result<BTreeMap<String, Vec<u8>>, StoreError> {
    let main = ledgers
        .get("main.jsonl")
        .ok_or_else(|| StoreError::Corruption("rewrite source lacks main.jsonl".to_owned()))?;
    let main_events = decode_events(main)?;
    if !main_events.iter().any(|event| event.seq() == anchor) {
        return Err(StoreError::InvalidForkAnchor(anchor));
    }
    let mut result = BTreeMap::new();
    result.insert(
        "main.jsonl".to_owned(),
        encoded_prefix(&main_events, anchor)?,
    );

    loop {
        let mut added = false;
        for (name, bytes) in ledgers {
            if name == "main.jsonl" || result.contains_key(name) {
                continue;
            }
            let events = decode_events(bytes)?;
            let genesis = events
                .first()
                .ok_or_else(|| StoreError::Corruption(format!("source child {name} is empty")))?;
            let genesis_value = event_value(genesis)?;
            let parent = genesis_value
                .get("parent")
                .and_then(Value::as_object)
                .ok_or_else(|| {
                    StoreError::Corruption(format!("source child {name} lacks parent"))
                })?;
            let Some(parent_file) = parent.get("file").and_then(Value::as_str) else {
                return Err(StoreError::Corruption(format!(
                    "source child {name} parent lacks file"
                )));
            };
            let Some(parent_bytes) = result.get(parent_file) else {
                continue;
            };
            let parent_seq = parent.get("seq").and_then(Value::as_u64).ok_or_else(|| {
                StoreError::Corruption(format!("source child {name} parent lacks seq"))
            })?;
            let parent_events = decode_events(parent_bytes)?;
            if parent_events.iter().any(|event| event.seq() == parent_seq) {
                result.insert(name.clone(), bytes.clone());
                added = true;
            }
        }
        if !added {
            break;
        }
    }
    Ok(result)
}

fn encoded_prefix(events: &[Event], anchor: u64) -> Result<Vec<u8>, StoreError> {
    let mut bytes = Vec::new();
    for event in events.iter().take_while(|event| event.seq() <= anchor) {
        bytes.extend_from_slice(&event.canonical_bytes()?);
        bytes.push(b'\n');
    }
    validate_ledger(&bytes, 1).map_err(|_| StoreError::InvalidForkAnchor(anchor))?;
    Ok(bytes)
}

fn validate_terminal_source(ledgers: &BTreeMap<String, Vec<u8>>) -> Result<(), StoreError> {
    let decoded = ledgers
        .iter()
        .map(|(name, bytes)| Ok((name.clone(), decode_events(bytes)?)))
        .collect::<Result<BTreeMap<_, _>, StoreError>>()?;
    for (name, bytes) in ledgers {
        let projection = validate_ledger(bytes, 1)?;
        let Some(turn) = projection.lifecycle.latest_turn else {
            if name != "main.jsonl" {
                return Err(StoreError::RewriteSourceNotTerminal(format!(
                    "child ledger {name} is unstarted"
                )));
            }
            continue;
        };
        if !projection.lifecycle.terminal_tail || projection.lifecycle.open_hold {
            return Err(StoreError::RewriteSourceNotTerminal(format!(
                "rewrite source {name} turn {turn} is not terminal"
            )));
        }
    }
    for (name, events) in decoded
        .iter()
        .filter(|(name, _)| name.as_str() != "main.jsonl")
    {
        let genesis = events.first().ok_or_else(|| {
            StoreError::RewriteSourceNotTerminal(format!("child ledger {name} is empty"))
        })?;
        let genesis_object = event_value(genesis)?;
        let thread = genesis.string_field("thread").ok_or_else(|| {
            StoreError::RewriteSourceNotTerminal(format!("child ledger {name} lacks thread"))
        })?;
        let parent = genesis_object
            .get("parent")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                StoreError::RewriteSourceNotTerminal(format!("child ledger {name} lacks parent"))
            })?;
        let parent_file = parent.get("file").and_then(Value::as_str).ok_or_else(|| {
            StoreError::RewriteSourceNotTerminal(format!("child ledger {name} parent lacks file"))
        })?;
        let parent_seq = parent.get("seq").and_then(Value::as_u64).ok_or_else(|| {
            StoreError::RewriteSourceNotTerminal(format!("child ledger {name} parent lacks seq"))
        })?;
        let spawn_id = parent
            .get("spawn_id")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                StoreError::RewriteSourceNotTerminal(format!(
                    "child ledger {name} parent lacks spawn_id"
                ))
            })?;
        let parent_events = decoded.get(parent_file).ok_or_else(|| {
            StoreError::RewriteSourceNotTerminal(format!(
                "child ledger {name} names absent parent {parent_file}"
            ))
        })?;
        let spawn = parent_events
            .iter()
            .find(|event| event.seq() == parent_seq)
            .ok_or_else(|| {
                StoreError::RewriteSourceNotTerminal(format!(
                    "child ledger {name} parent seq {parent_seq} is absent"
                ))
            })?;
        let spawn_object = event_value(spawn)?;
        if !matches!(spawn.kind(), EventKind::Spawn)
            || spawn_object.get("spawn_id").and_then(Value::as_str) != Some(spawn_id)
            || spawn_object.get("child").and_then(Value::as_str) != Some(thread)
        {
            return Err(StoreError::RewriteSourceNotTerminal(format!(
                "child ledger {name} parent binding does not name its spawn"
            )));
        }
        let call = spawn_object
            .get("call")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                StoreError::RewriteSourceNotTerminal(format!("parent spawn for {name} lacks call"))
            })?;
        let result_count = parent_events
            .iter()
            .filter(|event| matches!(event.kind(), EventKind::ChildResult))
            .filter_map(|event| event_value(event).ok())
            .filter(|event| {
                event.get("child").and_then(Value::as_str) == Some(thread)
                    && event.get("spawn_id").and_then(Value::as_str) == Some(spawn_id)
                    && event.get("call").and_then(Value::as_str) == Some(call)
            })
            .count();
        if result_count != 1 {
            return Err(StoreError::RewriteSourceNotTerminal(format!(
                "child ledger {name} requires exactly one terminal child_result"
            )));
        }
    }
    Ok(())
}

fn build_file_map(
    ledgers: &BTreeMap<String, Vec<u8>>,
    destination: &str,
) -> Result<Vec<RewriteFileMap>, StoreError> {
    let mut files = vec![RewriteFileMap {
        source: "main.jsonl".to_owned(),
        dest: "main.jsonl".to_owned(),
        thread: destination.to_owned(),
    }];
    for name in ledgers.keys().filter(|name| name.as_str() != "main.jsonl") {
        let thread = mapped_child_uuid(destination, name)?;
        files.push(RewriteFileMap {
            source: name.clone(),
            dest: format!("{thread}.jsonl"),
            thread,
        });
    }
    Ok(files)
}

fn mapped_child_uuid(destination: &str, source_file: &str) -> Result<String, StoreError> {
    let destination = Uuid::parse_str(destination)
        .map_err(|_| StoreError::Corruption("invalid destination UUID".to_owned()))?;
    let digest =
        Sha256::digest([destination.as_bytes().as_slice(), source_file.as_bytes()].concat());
    let mut bytes = [0_u8; 16];
    bytes[..6].copy_from_slice(&destination.as_bytes()[..6]);
    bytes[6..].copy_from_slice(&digest[6..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x70;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Ok(Uuid::from_bytes(bytes).hyphenated().to_string())
}

fn tree_digest(ledgers: &BTreeMap<String, Vec<u8>>) -> String {
    let mut digest = Sha256::new();
    for (name, bytes) in ledgers {
        digest.update(name.as_bytes());
        digest.update([0]);
        digest.update(bytes);
        digest.update([0]);
    }
    format!("sha256-{:x}", digest.finalize())
}

fn verify_source_digest(folder: &Path, expected: &str) -> Result<(), StoreError> {
    if !folder.is_dir() {
        return Err(StoreError::RewriteSourceChanged);
    }
    let actual = tree_digest(&read_source_ledgers(folder)?);
    if actual != expected {
        return Err(StoreError::RewriteSourceChanged);
    }
    Ok(())
}

fn scan_redaction(
    folder: &Path,
    ledgers: &BTreeMap<String, Vec<u8>>,
    forbidden: &[Vec<u8>],
) -> Result<RedactionInventory, StoreError> {
    let mut events = BTreeSet::new();
    for (file, bytes) in ledgers {
        for line in bytes
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
        {
            if forbidden.iter().any(|needle| contains_bytes(line, needle)) {
                let event = Event::decode_canonical(line)?;
                events.insert(RewriteEventRef {
                    file: file.clone(),
                    seq: event.seq(),
                });
            }
        }
    }
    let mut assets = BTreeSet::new();
    let asset_root = folder.join("assets");
    if asset_root.is_dir() {
        for entry in fs::read_dir(&asset_root)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let bytes = fs::read(entry.path())?;
                if forbidden
                    .iter()
                    .any(|needle| contains_bytes(&bytes, needle))
                {
                    assets.insert(entry.file_name().to_string_lossy().into_owned());
                }
            }
        }
    }
    let fingerprints = forbidden
        .iter()
        .map(|needle| format!("sha256-{}", hex_digest(needle)))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    Ok(RedactionInventory {
        events: events.into_iter().collect(),
        assets: assets.into_iter().collect(),
        fingerprints,
    })
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|candidate| candidate == needle)
}

fn operation_bytes(operation: &RewriteOperation) -> Result<Vec<u8>, StoreError> {
    validate_operation(operation)?;
    let mut bytes = serde_json_canonicalizer::to_vec(operation)
        .map_err(|error| StoreError::Corruption(format!("operation encode: {error}")))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn write_operation(stage: &Path, operation: &RewriteOperation) -> Result<(), StoreError> {
    AtomicPublisher::replace(stage.join("op.json"), &operation_bytes(operation)?)
}

fn read_operation(stage: &Path) -> Result<RewriteOperation, StoreError> {
    let bytes = fs::read(stage.join("op.json"))?;
    RewriteOperation::decode_canonical(&bytes)
}

fn build_payload(
    root: &Path,
    stage: &Path,
    operation: &RewriteOperation,
) -> Result<(), StoreError> {
    let payload = stage.join("payload");
    if payload.exists() {
        fs::remove_dir_all(&payload)?;
    }
    fs::create_dir(&payload)?;
    fs::create_dir(payload.join("assets"))?;
    let source_folder = root.join("threads").join(&operation.source);
    let source_ledgers = read_source_ledgers(&source_folder)?;
    if tree_digest(&source_ledgers) != operation.source_digest {
        return Err(StoreError::RewriteSourceChanged);
    }
    let ledgers = source_ledgers_for_operation(&source_ledgers, operation)?;

    let built = project_ledgers(&ledgers, operation)?;
    for file in &operation.files {
        let result = built.get(&file.source).ok_or_else(|| {
            StoreError::Corruption(format!("projection omitted source file {}", file.source))
        })?;
        let path = payload.join(&file.dest);
        fs::write(&path, &result.bytes)?;
        File::open(&path)?.sync_all()?;
    }
    copy_live_assets(&source_folder, &payload, operation)?;
    let session_settings = crate::session_settings_path(&source_folder)?;
    if session_settings.is_file() {
        let destination = payload.join(crate::SESSION_SETTINGS_FILE);
        fs::write(&destination, fs::read(&session_settings)?)?;
        File::open(&destination)?.sync_all()?;
    }
    if operation.source_anchor.is_some() {
        for name in ["endpoint.jsonl", "endpoint.lock"] {
            let file = File::create(payload.join(name))?;
            file.sync_all()?;
        }
    }
    File::open(payload.join("assets"))?.sync_all()?;
    File::open(&payload)?.sync_all()?;
    File::open(stage)?.sync_all()?;
    Ok(())
}

fn project_ledgers(
    ledgers: &BTreeMap<String, Vec<u8>>,
    operation: &RewriteOperation,
) -> Result<BTreeMap<String, BuiltFile>, StoreError> {
    let file_map = operation
        .files
        .iter()
        .map(|entry| (entry.source.clone(), entry.clone()))
        .collect::<BTreeMap<_, _>>();
    let source_thread_to_dest = source_thread_map(ledgers, &file_map)?;
    let mut built = BTreeMap::<String, BuiltFile>::new();
    let mut pending = operation
        .files
        .iter()
        .map(|entry| entry.source.clone())
        .collect::<BTreeSet<_>>();
    while !pending.is_empty() {
        let mut progress = false;
        for name in pending.clone() {
            let bytes = ledgers.get(&name).ok_or_else(|| {
                StoreError::Corruption(format!("operation maps missing source file {name}"))
            })?;
            let events = decode_events(bytes)?;
            let parent_file = source_parent_file(&events)?;
            if parent_file
                .as_ref()
                .is_some_and(|parent| !built.contains_key(parent))
            {
                continue;
            }
            let result = project_file(
                &name,
                &events,
                operation,
                &file_map,
                &source_thread_to_dest,
                &built,
            )?;
            built.insert(name.clone(), result);
            pending.remove(&name);
            progress = true;
        }
        if !progress {
            return Err(StoreError::Corruption(
                "rewrite child parent graph is cyclic or incomplete".to_owned(),
            ));
        }
    }
    Ok(built)
}

fn decode_events(bytes: &[u8]) -> Result<Vec<Event>, StoreError> {
    bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(Event::decode_canonical)
        .collect::<Result<Vec<_>, _>>()
        .map_err(StoreError::from)
}

fn event_value(event: &Event) -> Result<Map<String, Value>, StoreError> {
    serde_json::to_value(event.raw())
        .map_err(|error| StoreError::Corruption(format!("event value encode: {error}")))?
        .as_object()
        .cloned()
        .ok_or_else(|| StoreError::Corruption("event is not an object".to_owned()))
}

fn source_thread_map(
    ledgers: &BTreeMap<String, Vec<u8>>,
    file_map: &BTreeMap<String, RewriteFileMap>,
) -> Result<BTreeMap<String, String>, StoreError> {
    let mut result = BTreeMap::new();
    for (name, bytes) in ledgers {
        let events = decode_events(bytes)?;
        let genesis = events
            .first()
            .ok_or_else(|| StoreError::Corruption(format!("source file {name} has no genesis")))?;
        let source_thread = genesis
            .string_field("thread")
            .ok_or_else(|| StoreError::Corruption("genesis lacks thread".to_owned()))?;
        let mapped = file_map
            .get(name)
            .ok_or_else(|| StoreError::Corruption(format!("source file {name} is not mapped")))?;
        result.insert(source_thread.to_owned(), mapped.thread.clone());
    }
    Ok(result)
}

fn source_parent_file(events: &[Event]) -> Result<Option<String>, StoreError> {
    let Some(genesis) = events.first() else {
        return Ok(None);
    };
    let object = event_value(genesis)?;
    Ok(object
        .get("parent")
        .and_then(Value::as_object)
        .and_then(|parent| parent.get("file"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned))
}

fn project_file(
    source_file: &str,
    events: &[Event],
    operation: &RewriteOperation,
    file_map: &BTreeMap<String, RewriteFileMap>,
    source_thread_to_dest: &BTreeMap<String, String>,
    built_parents: &BTreeMap<String, BuiltFile>,
) -> Result<BuiltFile, StoreError> {
    let mapped = file_map
        .get(source_file)
        .ok_or_else(|| StoreError::Corruption(format!("missing file map for {source_file}")))?;
    let redaction_events = operation
        .redaction
        .as_ref()
        .map(|inventory| {
            inventory
                .events
                .iter()
                .filter(|entry| entry.file == source_file)
                .map(|entry| entry.seq)
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let poisoned_assets = operation
        .redaction
        .as_ref()
        .map(|inventory| inventory.assets.iter().cloned().collect::<BTreeSet<_>>())
        .unwrap_or_default();
    let mut dropped = redaction_events;
    let mut compact_summaries = Vec::new();
    for event in events {
        let object = event_value(event)?;
        if value_references_any(&Value::Object(object.clone()), &poisoned_assets) {
            dropped.insert(event.seq());
        }
        if object.contains_key("supersedes") || matches!(event.kind(), EventKind::Compact) {
            for (from, to) in value_ranges(
                &object,
                if matches!(event.kind(), EventKind::Compact) {
                    "covers"
                } else {
                    "supersedes"
                },
            )? {
                dropped.extend(from..=to);
            }
        }
        if matches!(event.kind(), EventKind::Compact) && !dropped.contains(&event.seq()) {
            compact_summaries.push((event.seq(), object.get("summary").cloned()));
        }
    }
    let mut dropped_turns = BTreeSet::new();
    for event in events {
        if !matches!(event.kind(), EventKind::TurnOpen) {
            continue;
        }
        let object = event_value(event)?;
        let inputs = object
            .get("trigger")
            .and_then(Value::as_object)
            .and_then(|trigger| trigger.get("inputs"))
            .and_then(Value::as_array);
        if inputs.is_some_and(|inputs| {
            inputs
                .iter()
                .filter_map(Value::as_u64)
                .any(|seq| dropped.contains(&seq))
        }) {
            dropped_turns.insert(event.turn().expect("turn_open has turn"));
        }
    }
    for event in events {
        if !matches!(event.kind(), EventKind::TurnOpen)
            || !event
                .turn()
                .is_some_and(|turn| dropped_turns.contains(&turn))
        {
            continue;
        }
        let object = event_value(event)?;
        if let Some(inputs) = object
            .get("trigger")
            .and_then(Value::as_object)
            .and_then(|trigger| trigger.get("inputs"))
            .and_then(Value::as_array)
        {
            dropped.extend(inputs.iter().filter_map(Value::as_u64));
        }
    }
    let mut active_turn = None;
    for event in events {
        if matches!(event.kind(), EventKind::TurnOpen) {
            active_turn = event.turn();
        } else if matches!(event.kind(), EventKind::Input)
            && active_turn.is_some_and(|turn| dropped_turns.contains(&turn))
            && event_value(event)?.get("steer").and_then(Value::as_bool) == Some(true)
        {
            dropped.insert(event.seq());
        } else if matches!(event.kind(), EventKind::Settle) && event.turn() == active_turn {
            active_turn = None;
        }
    }
    for event in events {
        if event
            .turn()
            .is_some_and(|turn| dropped_turns.contains(&turn))
        {
            dropped.insert(event.seq());
        }
    }
    for event in events {
        if dropped.contains(&event.seq())
            && matches!(event.kind(), EventKind::Spawn | EventKind::ChildResult)
        {
            return Err(StoreError::RewriteUnsupportedVisibleFact(format!(
                "rewrite cannot retain only half of child edge at {source_file}:{}",
                event.seq()
            )));
        }
    }

    let mut out = Vec::new();
    let mut seq_map = BTreeMap::new();
    let mut turn_map = BTreeMap::new();
    let mut next_seq = 1_u64;
    let mut next_turn = 1_u64;
    let mut min_reader = events[0].integer_field("min_reader").unwrap_or(1);
    let mut min_writer = events[0].integer_field("min_writer").unwrap_or(1);
    for event in events {
        if !matches!(event.kind(), EventKind::Meta) {
            continue;
        }
        let object = event_value(event)?;
        if let Some(upgrade) = object.get("upgrade").and_then(Value::as_object) {
            min_reader = min_reader.max(
                upgrade
                    .get("min_reader")
                    .and_then(Value::as_u64)
                    .unwrap_or(min_reader),
            );
            min_writer = min_writer.max(
                upgrade
                    .get("min_writer")
                    .and_then(Value::as_u64)
                    .unwrap_or(min_writer),
            );
        }
    }
    let genesis = build_genesis(
        &events[0],
        &GenesisContext {
            source_file,
            mapped,
            operation,
            file_map,
            built_parents,
            poisoned_assets: &poisoned_assets,
            minima: (min_reader, min_writer),
        },
    )?;
    push_event(&mut out, genesis)?;
    seq_map.insert(events[0].seq(), next_seq);
    next_seq += 1;
    let mut latest_epoch = None;
    let mut effective_meta = Map::new();

    for event in events.iter().skip(1) {
        let old_seq = event.seq();
        let object = event_value(event)?;
        if matches!(event.kind(), EventKind::Epoch) {
            if !dropped.contains(&old_seq) {
                latest_epoch = Some(object);
            }
            continue;
        }
        if matches!(event.kind(), EventKind::Meta) {
            if !dropped.contains(&old_seq) {
                let mut changed = false;
                for field in ["title", "labels"] {
                    if let Some(value) = object.get(field) {
                        effective_meta.insert(field.to_owned(), value.clone());
                        changed = true;
                    }
                }
                if changed {
                    effective_meta.insert(
                        "ts".to_owned(),
                        object
                            .get("ts")
                            .cloned()
                            .unwrap_or_else(|| json!(operation.created_at)),
                    );
                }
            }
            continue;
        }
        if dropped.contains(&old_seq) {
            continue;
        }
        let value = match event.kind() {
            EventKind::Input => Some(materialize_input(
                object,
                next_seq,
                operation,
                source_file,
                old_seq,
                &mapped.thread,
            )),
            EventKind::TurnOpen => {
                let old_turn = event.turn().expect("turn_open has turn");
                turn_map.insert(old_turn, next_turn);
                let value = materialize_turn_open(object, next_seq, next_turn, &seq_map)?;
                next_turn += 1;
                Some(value)
            }
            EventKind::Spawn => Some(materialize_spawn(
                object,
                next_seq,
                mapped_turn(event, &turn_map)?,
                source_thread_to_dest,
            )?),
            EventKind::ChildResult => Some(materialize_child_result(
                object,
                next_seq,
                mapped_turn(event, &turn_map)?,
                source_thread_to_dest,
            )?),
            EventKind::State => Some(materialize_state(
                object,
                next_seq,
                mapped_turn(event, &turn_map)?,
            )),
            EventKind::Output
            | EventKind::Reasoning
            | EventKind::ToolCall
            | EventKind::ToolResult => Some(materialize_historical_state(
                event,
                object,
                source_file,
                next_seq,
                mapped_turn(event, &turn_map)?,
            )),
            EventKind::Settle => Some(materialize_settle(
                object,
                next_seq,
                mapped_turn(event, &turn_map)?,
            )),
            EventKind::Extension(_) if event.effective_visibility() == Visibility::Model => {
                return Err(StoreError::RewriteUnsupportedVisibleFact(format!(
                    "unsupported model-visible extension at {source_file}:{old_seq}"
                )));
            }
            EventKind::Extension(_) if event.has_field("anchor") => {
                Some(materialize_historical_state(
                    event,
                    object,
                    source_file,
                    next_seq,
                    mapped_turn(event, &turn_map)?,
                ))
            }
            EventKind::Genesis
            | EventKind::RunStart
            | EventKind::Epoch
            | EventKind::QueueEdit
            | EventKind::EffectiveExecution
            | EventKind::ToolExecutionStarted
            | EventKind::ApprovalRequest
            | EventKind::ApprovalResponse
            | EventKind::Attempt
            | EventKind::AttemptDispatched
            | EventKind::AttemptRecovery
            | EventKind::Error
            | EventKind::Checkpoint
            | EventKind::Compact
            | EventKind::StopRequested
            | EventKind::Meta
            | EventKind::Extension(_) => None,
        };
        if let Some(value) = value {
            push_event(&mut out, value)?;
            seq_map.insert(old_seq, next_seq);
            next_seq += 1;
        }
    }

    if !effective_meta.is_empty() {
        if let Some(value) = materialize_meta(effective_meta, next_seq) {
            push_event(&mut out, value)?;
            next_seq += 1;
        }
    }
    if !compact_summaries.is_empty() {
        let key = format!("rewrite:{}:{source_file}:compact", operation.id);
        push_event(
            &mut out,
            json!({
                "v": 1, "seq": next_seq, "kind": "input", "ts": operation.created_at,
                "visibility": "runtime", "origin_key": key,
                "origin_tuple": rewrite_origin(&key, &mapped.thread, operation.kind),
                "content": [{"type":"text", "text":"Imported compacted context"}],
                "source": "cross_thread"
            }),
        )?;
        let trigger_seq = next_seq;
        next_seq += 1;
        push_event(
            &mut out,
            json!({
                "v": 1, "seq": next_seq, "kind": "turn_open", "ts": operation.created_at,
                "turn": next_turn, "trigger": {"inputs":[trigger_seq]}
            }),
        )?;
        next_seq += 1;
        for (source_seq, summary) in compact_summaries {
            push_event(
                &mut out,
                json!({
                    "v": 1, "seq": next_seq, "kind": "state", "ts": operation.created_at,
                    "turn": next_turn, "subkind": "rewrite.compact",
                    "payload": {"source_file":source_file,"source_seq":source_seq,"summary":summary}
                }),
            )?;
            next_seq += 1;
        }
        push_event(
            &mut out,
            json!({
                "v": 1, "seq": next_seq, "kind": "settle", "ts": operation.created_at,
                "turn": next_turn, "outcome": "completed"
            }),
        )?;
        next_seq += 1;
    }
    if let Some(epoch) = latest_epoch {
        push_event(
            &mut out,
            materialize_epoch(epoch, next_seq, operation, source_file)?,
        )?;
    }
    validate_ledger(&out, 1)?;
    Ok(BuiltFile {
        bytes: out,
        seq_map,
    })
}

fn build_genesis(source: &Event, context: &GenesisContext<'_>) -> Result<Value, StoreError> {
    let object = event_value(source)?;
    let supplied_origin = (context.source_file == "main.jsonl")
        .then_some(context.operation.genesis_origin.as_ref())
        .flatten();
    let key = supplied_origin.map_or_else(
        || {
            format!(
                "rewrite:{}:{}:genesis",
                context.operation.id, context.source_file
            )
        },
        |origin| origin.key.clone(),
    );
    let origin = supplied_origin.map_or_else(
        || rewrite_origin(&key, &context.mapped.thread, context.operation.kind),
        |origin| serde_json::to_value(origin).expect("origin tuple serializes"),
    );
    let mut genesis = json!({
        "v": 1, "seq": 1, "kind": "genesis", "ts": context.operation.created_at,
        "format": object.get("format").cloned().unwrap_or(json!(1)),
        "min_reader": context.minima.0,
        "min_writer": context.minima.1,
        "thread": context.mapped.thread,
        "workspace": object.get("workspace").cloned().unwrap_or(json!("default")),
        "resume": object.get("resume").cloned().unwrap_or(json!("never")),
        "origin_key": key,
        "origin_tuple": origin,
        "config": object.get("config").cloned().unwrap_or(json!({"digest":"unknown"}))
    });
    let target = genesis.as_object_mut().expect("genesis object");
    // The flag is decided by this operation, never inherited: forking a
    // scratch ledger into a durable session drops it, and child ledgers are
    // never ephemeral on their own.
    if context.operation.ephemeral && context.source_file == "main.jsonl" {
        target.insert("ephemeral".to_owned(), json!(true));
    }
    if let Some(instruction) = object.get("instruction") {
        target.insert("instruction".to_owned(), instruction.clone());
    }
    if let Some(folder_binding) = object.get("folder_binding") {
        target.insert("folder_binding".to_owned(), folder_binding.clone());
    }
    if let Some(identity) = object.get("identity_profile") {
        target.insert("identity_profile".to_owned(), identity.clone());
    }
    if let Some(seed) = object.get("seed") {
        if !value_references_any(seed, context.poisoned_assets) {
            target.insert("seed".to_owned(), seed.clone());
        }
    }
    if let Some(parent) = object.get("parent").and_then(Value::as_object) {
        let parent_file = parent
            .get("file")
            .and_then(Value::as_str)
            .ok_or_else(|| StoreError::Corruption("child parent lacks file".to_owned()))?;
        let parent_seq = parent
            .get("seq")
            .and_then(Value::as_u64)
            .ok_or_else(|| StoreError::Corruption("child parent lacks seq".to_owned()))?;
        let mapped_parent = context.file_map.get(parent_file).ok_or_else(|| {
            StoreError::Corruption(format!("child parent file {parent_file} is not mapped"))
        })?;
        let mapped_seq = context
            .built_parents
            .get(parent_file)
            .and_then(|built| built.seq_map.get(&parent_seq))
            .copied()
            .ok_or_else(|| {
                StoreError::Corruption(format!(
                    "child parent spawn {parent_file}:{parent_seq} was dropped"
                ))
            })?;
        target.insert(
            "parent".to_owned(),
            json!({
                "file": mapped_parent.dest,
                "seq": mapped_seq,
                "spawn_id": parent.get("spawn_id").cloned().unwrap_or(Value::Null)
            }),
        );
    }
    Ok(genesis)
}

fn rewrite_origin(key: &str, target: &str, kind: RewriteKind) -> Value {
    json!({
        "principal":"kernel", "client":"rewrite", "target":target,
        "op":kind.as_str(), "key":key
    })
}

fn materialize_input(
    source: Map<String, Value>,
    seq: u64,
    operation: &RewriteOperation,
    source_file: &str,
    source_seq: u64,
    target_thread: &str,
) -> Value {
    let key = format!("rewrite:{}:{source_file}:{source_seq}", operation.id);
    let mut value = json!({
        "v":1,"seq":seq,"kind":"input","ts":source["ts"],
        "origin_key":key,
        "origin_tuple":rewrite_origin(&key,target_thread,operation.kind),
        "content":source["content"]
    });
    let object = value.as_object_mut().expect("input object");
    for field in ["steer", "source", "visibility", "anchor", "min_reader"] {
        if let Some(field_value) = source.get(field) {
            object.insert(field.to_owned(), field_value.clone());
        }
    }
    value
}

fn materialize_turn_open(
    source: Map<String, Value>,
    seq: u64,
    turn: u64,
    seq_map: &BTreeMap<u64, u64>,
) -> Result<Value, StoreError> {
    let trigger = match source.get("trigger") {
        Some(Value::String(value)) if value == "genesis" => json!("genesis"),
        Some(Value::Object(trigger)) => {
            let inputs = trigger
                .get("inputs")
                .and_then(Value::as_array)
                .ok_or_else(|| StoreError::Corruption("turn trigger lacks inputs".to_owned()))?
                .iter()
                .map(|value| {
                    value
                        .as_u64()
                        .and_then(|old| seq_map.get(&old).copied())
                        .ok_or_else(|| {
                            StoreError::Corruption(
                                "turn trigger input was dropped during rewrite".to_owned(),
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            json!({"inputs":inputs})
        }
        _ => return Err(StoreError::Corruption("invalid turn trigger".to_owned())),
    };
    Ok(json!({
        "v":1,"seq":seq,"kind":"turn_open","ts":source["ts"],"turn":turn,
        "trigger":trigger
    }))
}

fn materialize_spawn(
    mut source: Map<String, Value>,
    seq: u64,
    turn: u64,
    thread_map: &BTreeMap<String, String>,
) -> Result<Value, StoreError> {
    let ts = source
        .get("ts")
        .cloned()
        .unwrap_or_else(|| json!("1970-01-01T00:00:00.000Z"));
    let child = source
        .get("child")
        .and_then(Value::as_str)
        .and_then(|value| thread_map.get(value))
        .ok_or_else(|| StoreError::Corruption("spawn child is not mapped".to_owned()))?
        .clone();
    retain_payload_fields(&mut source, &["call", "spawn_id", "resume", "seed"]);
    source.insert("v".to_owned(), json!(1));
    source.insert("seq".to_owned(), json!(seq));
    source.insert("kind".to_owned(), json!("spawn"));
    source.insert("ts".to_owned(), ts);
    source.insert("turn".to_owned(), json!(turn));
    source.insert("child".to_owned(), json!(child));
    Ok(Value::Object(source))
}

fn materialize_child_result(
    mut source: Map<String, Value>,
    seq: u64,
    turn: u64,
    thread_map: &BTreeMap<String, String>,
) -> Result<Value, StoreError> {
    let ts = source
        .get("ts")
        .cloned()
        .unwrap_or_else(|| json!("1970-01-01T00:00:00.000Z"));
    let child = source
        .get("child")
        .and_then(Value::as_str)
        .and_then(|value| thread_map.get(value))
        .ok_or_else(|| StoreError::Corruption("child_result child is not mapped".to_owned()))?
        .clone();
    retain_payload_fields(
        &mut source,
        &["call", "spawn_id", "outcome", "summary", "artifacts"],
    );
    source.insert("v".to_owned(), json!(1));
    source.insert("seq".to_owned(), json!(seq));
    source.insert("kind".to_owned(), json!("child_result"));
    source.insert("ts".to_owned(), ts);
    source.insert("turn".to_owned(), json!(turn));
    source.insert("child".to_owned(), json!(child));
    Ok(Value::Object(source))
}

fn materialize_state(mut source: Map<String, Value>, seq: u64, turn: u64) -> Value {
    let ts = source
        .get("ts")
        .cloned()
        .unwrap_or_else(|| json!("1970-01-01T00:00:00.000Z"));
    retain_payload_fields(&mut source, &["subkind", "payload", "visibility", "anchor"]);
    source.insert("v".to_owned(), json!(1));
    source.insert("seq".to_owned(), json!(seq));
    source.insert("kind".to_owned(), json!("state"));
    source.insert("ts".to_owned(), ts);
    source.insert("turn".to_owned(), json!(turn));
    Value::Object(source)
}

fn materialize_historical_state(
    event: &Event,
    source: Map<String, Value>,
    source_file: &str,
    seq: u64,
    turn: u64,
) -> Value {
    let mut payload = source;
    for field in
        envelope_fields()
            .into_iter()
            .chain(["attempt", "sealed", "continuation", "origin_tuple"])
    {
        payload.remove(field);
    }
    let mut value = json!({
        "v":1,"seq":seq,"kind":"state","ts":event_value_ts(event),"turn":turn,
        "subkind":format!("rewrite.{}",event.kind().as_str()),
        "payload":{"source_file":source_file,"source_seq":event.seq(),"value":payload}
    });
    if event.effective_visibility() == Visibility::Runtime {
        value
            .as_object_mut()
            .expect("state object")
            .insert("visibility".to_owned(), json!("runtime"));
    }
    value
}

fn event_value_ts(event: &Event) -> Value {
    serde_json::to_value(event.raw())
        .ok()
        .and_then(|value| value.get("ts").cloned())
        .unwrap_or_else(|| json!("1970-01-01T00:00:00.000Z"))
}

fn materialize_settle(mut source: Map<String, Value>, seq: u64, turn: u64) -> Value {
    let ts = source
        .get("ts")
        .cloned()
        .unwrap_or_else(|| json!("1970-01-01T00:00:00.000Z"));
    retain_payload_fields(&mut source, &["outcome", "reason", "classification"]);
    source.insert("v".to_owned(), json!(1));
    source.insert("seq".to_owned(), json!(seq));
    source.insert("kind".to_owned(), json!("settle"));
    source.insert("ts".to_owned(), ts);
    source.insert("turn".to_owned(), json!(turn));
    Value::Object(source)
}

fn materialize_meta(mut source: Map<String, Value>, seq: u64) -> Option<Value> {
    let ts = source
        .get("ts")
        .cloned()
        .unwrap_or_else(|| json!("1970-01-01T00:00:00.000Z"));
    let title = source.get("title").cloned();
    let labels = source.get("labels").cloned();
    if title.is_none() && labels.is_none() {
        return None;
    }
    source.clear();
    source.insert("v".to_owned(), json!(1));
    source.insert("seq".to_owned(), json!(seq));
    source.insert("kind".to_owned(), json!("meta"));
    source.insert("ts".to_owned(), ts);
    if let Some(title) = title {
        source.insert("title".to_owned(), title);
    }
    if let Some(labels) = labels {
        source.insert("labels".to_owned(), labels);
    }
    Some(Value::Object(source))
}

fn materialize_epoch(
    mut source: Map<String, Value>,
    seq: u64,
    operation: &RewriteOperation,
    source_file: &str,
) -> Result<Value, StoreError> {
    retain_payload_fields(
        &mut source,
        &["adapter", "model", "system", "tools", "renderer"],
    );
    source.insert("v".to_owned(), json!(1));
    source.insert("seq".to_owned(), json!(seq));
    source.insert("kind".to_owned(), json!("epoch"));
    source.insert("ts".to_owned(), json!(operation.created_at));
    source.insert(
        "id".to_owned(),
        json!(format!("rewrite:{}:{source_file}", operation.id)),
    );
    source.insert("reason".to_owned(), json!(operation.kind.as_str()));
    Ok(Value::Object(source))
}

fn mapped_turn(event: &Event, turn_map: &BTreeMap<u64, u64>) -> Result<u64, StoreError> {
    event
        .turn()
        .and_then(|turn| turn_map.get(&turn).copied())
        .ok_or_else(|| StoreError::Corruption("turn-bound fact precedes turn_open".to_owned()))
}

fn retain_payload_fields(object: &mut Map<String, Value>, fields: &[&str]) {
    let keep = fields.iter().copied().collect::<BTreeSet<_>>();
    object.retain(|key, _| keep.contains(key.as_str()));
}

fn envelope_fields() -> [&'static str; 10] {
    [
        "v",
        "seq",
        "kind",
        "ts",
        "turn",
        "visibility",
        "anchor",
        "supersedes",
        "origin_key",
        "min_reader",
    ]
}

fn push_event(output: &mut Vec<u8>, value: Value) -> Result<(), StoreError> {
    let bytes = serde_json_canonicalizer::to_vec(&value)
        .map_err(|error| StoreError::Corruption(format!("rewrite event encode: {error}")))?;
    Event::decode_canonical(&bytes)?;
    output.extend_from_slice(&bytes);
    output.push(b'\n');
    Ok(())
}

fn value_ranges(object: &Map<String, Value>, field: &str) -> Result<Vec<(u64, u64)>, StoreError> {
    let Some(values) = object.get(field) else {
        return Ok(Vec::new());
    };
    values
        .as_array()
        .ok_or_else(|| StoreError::Corruption(format!("{field} is not an array")))?
        .iter()
        .map(|value| {
            let range = value
                .as_object()
                .ok_or_else(|| StoreError::Corruption(format!("{field} range is not object")))?;
            let from = range
                .get("from")
                .and_then(Value::as_u64)
                .ok_or_else(|| StoreError::Corruption(format!("{field} range lacks from")))?;
            let to = range
                .get("to")
                .and_then(Value::as_u64)
                .ok_or_else(|| StoreError::Corruption(format!("{field} range lacks to")))?;
            Ok((from, to))
        })
        .collect()
}

fn valid_asset_name(value: &str) -> bool {
    value.strip_prefix("sha256-").is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

fn collect_asset_names(value: &Value, output: &mut BTreeSet<String>) {
    match value {
        Value::String(value) if valid_asset_name(value) => {
            output.insert(value.clone());
        }
        Value::Array(values) => {
            for value in values {
                collect_asset_names(value, output);
            }
        }
        Value::Object(values) => {
            for value in values.values() {
                collect_asset_names(value, output);
            }
        }
        _ => {}
    }
}

fn raw_digest_asset(value: &Value) -> Option<String> {
    let digest = value.as_str()?;
    if digest.len() == 64
        && digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Some(format!("sha256-{digest}"))
    } else {
        None
    }
}

fn collect_event_asset_names(value: &Value, output: &mut BTreeSet<String>) {
    collect_asset_names(value, output);
    let Some(object) = value.as_object() else {
        return;
    };
    match object.get("kind").and_then(Value::as_str) {
        Some("genesis") => {
            for carrier in ["config", "instruction"] {
                if let Some(asset) = object
                    .get(carrier)
                    .and_then(Value::as_object)
                    .and_then(|carrier| carrier.get("digest"))
                    .and_then(raw_digest_asset)
                {
                    output.insert(asset);
                }
            }
        }
        Some("run_start") => {
            if let Some(asset) = object
                .get("launch_bindings_digest")
                .and_then(raw_digest_asset)
            {
                output.insert(asset);
            }
        }
        _ => {}
    }
}

fn collect_launch_binding_asset_name(value: &Value, output: &mut BTreeSet<String>) {
    let Some(object) = value.as_object() else {
        return;
    };
    if object.get("kind").and_then(Value::as_str) == Some("run_start") {
        if let Some(asset) = object
            .get("launch_bindings_digest")
            .and_then(raw_digest_asset)
        {
            output.insert(asset);
        }
    }
}

fn value_references_any(value: &Value, assets: &BTreeSet<String>) -> bool {
    let mut found = BTreeSet::new();
    collect_asset_names(value, &mut found);
    found.iter().any(|name| assets.contains(name))
}

fn copy_live_assets(
    source_folder: &Path,
    payload: &Path,
    operation: &RewriteOperation,
) -> Result<(), StoreError> {
    let poisoned = operation
        .redaction
        .as_ref()
        .map(|inventory| inventory.assets.iter().cloned().collect::<BTreeSet<_>>())
        .unwrap_or_default();
    let mut pending = BTreeSet::new();
    // Rewrites intentionally materialize durable history rather than copying run_start events.
    // Their launch-binding assets still remain provenance carriers and must survive a fork or a
    // redaction unless the carrier itself is part of the redaction inventory.
    let source_ledgers = operation
        .source_anchor
        .as_ref()
        .map(|_| source_ledgers_for_operation(&read_source_ledgers(source_folder)?, operation))
        .transpose()?;
    for file in &operation.files {
        let bytes = match source_ledgers.as_ref() {
            Some(ledgers) => ledgers.get(&file.source).cloned().ok_or_else(|| {
                StoreError::Corruption(format!(
                    "operation maps missing source file {}",
                    file.source
                ))
            })?,
            None => fs::read(source_folder.join(&file.source))?,
        };
        for line in bytes
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
        {
            let value: Value = serde_json::from_slice(line)
                .map_err(|error| StoreError::Corruption(format!("source ledger JSON: {error}")))?;
            collect_launch_binding_asset_name(&value, &mut pending);
        }
    }
    pending.retain(|name| !poisoned.contains(name));
    for file in &operation.files {
        let bytes = fs::read(payload.join(&file.dest))?;
        for line in bytes
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
        {
            let value: Value = serde_json::from_slice(line).map_err(|error| {
                StoreError::Corruption(format!("rewritten ledger JSON: {error}"))
            })?;
            collect_event_asset_names(&value, &mut pending);
        }
    }
    let source_assets = AssetStore::new(source_folder.join("assets"))?;
    let destination_assets = AssetStore::new(payload.join("assets"))?;
    let mut copied = BTreeSet::new();
    while let Some(name) = pending.pop_first() {
        if !copied.insert(name.clone()) {
            continue;
        }
        if poisoned.contains(&name) {
            return Err(StoreError::RewriteCarrierCorrupt(format!(
                "retained event references poisoned asset {name}"
            )));
        }
        let bytes = source_assets
            .read_verified(&name)
            .map_err(|error| StoreError::RewriteCarrierCorrupt(format!("asset {name}: {error}")))?;
        let published = destination_assets.publish(&bytes)?;
        if published.asset != name {
            return Err(StoreError::RewriteCarrierCorrupt(
                "asset publication changed digest name".to_owned(),
            ));
        }
        if let Ok(value) = serde_json::from_slice::<Value>(&bytes) {
            collect_event_asset_names(&value, &mut pending);
        } else {
            for line in bytes
                .split(|byte| *byte == b'\n')
                .filter(|line| !line.is_empty())
            {
                if let Ok(value) = serde_json::from_slice::<Value>(line) {
                    collect_event_asset_names(&value, &mut pending);
                }
            }
        }
    }
    Ok(())
}

fn validate_payload(
    root: &Path,
    stage: &Path,
    operation: &RewriteOperation,
) -> Result<(), StoreError> {
    let payload = stage.join("payload");
    if !payload.is_dir() {
        return Err(StoreError::Corruption(
            "validated rewrite lacks payload".to_owned(),
        ));
    }
    for file in &operation.files {
        let bytes = fs::read(payload.join(&file.dest))?;
        let projection = validate_ledger(&bytes, 1)?;
        crate::tail::check_write_versions(&projection.events, 1)?;
        let genesis = projection.events.first().ok_or_else(|| {
            StoreError::Corruption(format!("rewritten file {} is empty", file.dest))
        })?;
        if genesis.string_field("thread") != Some(file.thread.as_str()) {
            return Err(StoreError::Corruption(format!(
                "rewritten file {} has wrong genesis identity",
                file.dest
            )));
        }
        if file.dest != "main.jsonl" && projection.lifecycle.latest_turn.is_none() {
            return Err(StoreError::Corruption(format!(
                "rewritten child {} cannot become unstarted",
                file.dest
            )));
        }
        if bytes
            .windows(b"\"sealed\"".len())
            .any(|window| window == b"\"sealed\"")
        {
            return Err(StoreError::Corruption(
                "rewritten ledger retained sealed carrier".to_owned(),
            ));
        }
    }
    copy_live_assets(
        &root.join("threads").join(&operation.source),
        &payload,
        operation,
    )?;
    let assets = AssetStore::new(payload.join("assets"))?;
    for file in &operation.files {
        let bytes = fs::read(payload.join(&file.dest))?;
        let events = decode_events(&bytes)?;
        if let Some(genesis) = events.first() {
            validate_seed_snapshot(genesis, &assets)?;
        }
    }
    Ok(())
}

fn validate_seed_snapshot(genesis: &Event, assets: &AssetStore) -> Result<(), StoreError> {
    let object = event_value(genesis)?;
    let Some(seed) = object.get("seed").and_then(Value::as_object) else {
        return Ok(());
    };
    let snapshot = seed
        .get("snapshot")
        .and_then(Value::as_object)
        .ok_or_else(|| StoreError::RewriteCarrierCorrupt("seed lacks snapshot".to_owned()))?;
    let asset = snapshot
        .get("asset")
        .and_then(Value::as_str)
        .ok_or_else(|| StoreError::RewriteCarrierCorrupt("seed snapshot lacks asset".to_owned()))?;
    let digest = snapshot
        .get("digest")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            StoreError::RewriteCarrierCorrupt("seed snapshot lacks digest".to_owned())
        })?;
    if asset != format!("sha256-{digest}") {
        return Err(StoreError::RewriteCarrierCorrupt(
            "seed snapshot asset/digest mismatch".to_owned(),
        ));
    }
    let bytes = assets
        .read_verified(asset)
        .map_err(|error| StoreError::RewriteCarrierCorrupt(error.to_string()))?;
    if !bytes.is_empty() && !bytes.ends_with(b"\n") {
        return Err(StoreError::RewriteCarrierCorrupt(
            "seed snapshot is not LF terminated".to_owned(),
        ));
    }
    let declared = seed
        .get("kinds")
        .and_then(Value::as_array)
        .ok_or_else(|| StoreError::RewriteCarrierCorrupt("seed lacks kinds".to_owned()))?
        .iter()
        .map(|value| {
            value.as_str().map(ToOwned::to_owned).ok_or_else(|| {
                StoreError::RewriteCarrierCorrupt("seed kind is not a string".to_owned())
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut observed = BTreeSet::new();
    let mut previous_seq = 0;
    for line in bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
    {
        let event = Event::decode_canonical(line)
            .map_err(|error| StoreError::RewriteCarrierCorrupt(error.to_string()))?;
        if event.seq() <= previous_seq {
            return Err(StoreError::RewriteCarrierCorrupt(
                "seed snapshot seqs are not strictly increasing".to_owned(),
            ));
        }
        previous_seq = event.seq();
        observed.insert(
            event
                .string_field("kind")
                .expect("validated event has kind")
                .to_owned(),
        );
    }
    if declared.iter().cloned().collect::<BTreeSet<_>>() != observed
        || declared.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(StoreError::RewriteCarrierCorrupt(
            "seed kinds do not equal the sorted snapshot kind set".to_owned(),
        ));
    }
    Ok(())
}

fn close_operation(root: &Path, stage: &Path) -> Result<(), StoreError> {
    fs::remove_file(stage.join("op.json"))?;
    File::open(stage)?.sync_all()?;
    fs::remove_dir(stage)?;
    File::open(root.join("staging"))?.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_canonical_events(path: &Path, events: &[Value]) {
        let mut bytes = Vec::new();
        for event in events {
            bytes.extend_from_slice(
                &serde_json_canonicalizer::to_vec(event).expect("canonical event"),
            );
            bytes.push(b'\n');
        }
        validate_ledger(&bytes, 1).expect("valid test ledger");
        fs::write(path, bytes).expect("write ledger");
    }

    fn origin(target: &str, key: &str, op: &str) -> OriginTuple {
        OriginTuple {
            principal: "uid:501".to_owned(),
            client: "session-endpoint".to_owned(),
            target: target.to_owned(),
            op: op.to_owned(),
            key: key.to_owned(),
        }
    }

    #[test]
    fn startup_isolates_missing_fork_carrier_without_losing_source() {
        let directory = tempfile::tempdir().unwrap();
        let store = ThreadStore::open(directory.path()).unwrap();
        let source_id = "018f0000-0000-7000-8000-000000000003";
        let destination_id = "018f0000-0000-7000-8000-000000000004";
        let source = store.root().join("threads").join(source_id);
        fs::create_dir(&source).unwrap();
        let baseline = include_bytes!("../../../fixtures/invalid/_valid-baseline.jsonl");
        let mut events = baseline
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| serde_json::from_slice::<Value>(line).unwrap())
            .collect::<Vec<_>>();
        let run = events
            .iter_mut()
            .find(|event| event["kind"] == "run_start")
            .unwrap();
        run.as_object_mut()
            .unwrap()
            .insert("launch_bindings_digest".into(), json!("a".repeat(64)));
        write_canonical_events(&source.join("main.jsonl"), &events);
        let original = fs::read(source.join("main.jsonl")).unwrap();
        store
            .begin_fork(
                "failed-fork",
                source_id,
                destination_id,
                "2026-10-04T00:00:00.000Z",
            )
            .unwrap();
        assert!(matches!(
            store.recover_rewrites(),
            Err(StoreError::RewriteCarrierCorrupt(_))
        ));
        let stage = store.root().join("staging/failed-fork");
        let operation = read_operation(&stage).unwrap();
        let mut protected = operation.clone();
        protected.phase = RewritePhase::Validated;
        write_operation(&stage, &protected).unwrap();
        let error = StoreError::RewriteCarrierCorrupt("missing asset".into());
        assert!(
            !store
                .quarantine_unpublished_fork("failed-fork", &error)
                .unwrap()
        );
        protected.phase = RewritePhase::Prepared;
        protected.kind = RewriteKind::Redact;
        protected.redaction = Some(RedactionInventory {
            events: Vec::new(),
            assets: Vec::new(),
            fingerprints: Vec::new(),
        });
        write_operation(&stage, &protected).unwrap();
        assert!(
            !store
                .quarantine_unpublished_fork("failed-fork", &error)
                .unwrap()
        );
        assert!(stage.join("op.json").is_file());
        write_operation(&stage, &operation).unwrap();
        assert!(store.recover_rewrites_for_startup().unwrap().is_empty());
        assert_eq!(fs::read(source.join("main.jsonl")).unwrap(), original);
        assert!(!store.root().join("threads").join(destination_id).exists());
        let quarantine = store.root().join(".rewrite-quarantine/failed-fork");
        assert!(quarantine.join("op.json").is_file());
        assert!(quarantine.join("recovery-error.txt").is_file());
        assert!(!store.source_has_live_rewrite(source_id).unwrap());
        assert!(store.recover_rewrites_for_startup().unwrap().is_empty());
        store.gc_rewrite_debris().unwrap();
        assert!(quarantine.join("op.json").is_file());
    }

    #[test]
    fn anchored_fork_uses_terminal_prefix_binding_and_empty_endpoint_journal() {
        let directory = tempfile::tempdir().expect("tempdir");
        let store = ThreadStore::open(directory.path()).expect("thread store");
        let source_id = "018f0000-0000-7000-8000-000000000003";
        let destination_id = "018f0000-0000-7000-8000-000000000004";
        let source = store.root().join("threads").join(source_id);
        fs::create_dir(&source).expect("source folder");
        let assets = AssetStore::new(source.join("assets")).expect("assets");
        let retained = assets
            .publish(b"retained launch profile")
            .expect("retained");
        let later = assets.publish(b"later launch profile").expect("later");

        let baseline = include_bytes!("../../../fixtures/invalid/_valid-baseline.jsonl");
        let mut events = baseline
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| serde_json::from_slice::<Value>(line).expect("baseline event"))
            .collect::<Vec<_>>();
        events[2].as_object_mut().unwrap().insert(
            "launch_bindings_digest".to_owned(),
            json!(retained.asset.trim_start_matches("sha256-")),
        );
        events.push(json!({
            "v":1,"seq":9,"kind":"run_start","ts":"2026-08-28T09:00:00.000Z",
            "run":"r2","mode":"ordinary","recovery_ordinal":0,"binary":"worker",
            "config_digest":"cfg","instruction_digest":"ins","policy":"default",
            "launch_bindings_digest":later.asset.trim_start_matches("sha256-")
        }));
        events.push(json!({
            "v":1,"seq":10,"kind":"input","ts":"2026-08-28T09:00:00.000Z",
            "origin_key":"later-input","origin_tuple":{
                "principal":"p","client":"c","target":source_id,"op":"submit","key":"later-input"
            },"content":[{"type":"text","text":"later"}]
        }));
        events.push(json!({
            "v":1,"seq":11,"kind":"turn_open","ts":"2026-08-28T09:00:00.000Z",
            "turn":2,"trigger":{"inputs":[10]}
        }));
        events.push(json!({
            "v":1,"seq":12,"kind":"settle","ts":"2026-08-28T09:00:00.000Z",
            "turn":2,"outcome":"completed"
        }));
        write_canonical_events(&source.join("main.jsonl"), &events);
        fs::write(source.join("endpoint.jsonl"), b"not a semantic ledger\n")
            .expect("source endpoint carrier");
        let settings = br#"{"format":1,"model":"deepseek-chat","provider":"deepseek","revision":3}
"#;
        fs::write(source.join(crate::SESSION_SETTINGS_FILE), settings)
            .expect("source session settings");

        let invalid = store.begin_fork_at_kernel_anchor(
            "fork-open-prefix",
            source_id,
            7,
            ForkGenesisBinding {
                thread: "018f0000-0000-7000-8000-000000000005".to_owned(),
                origin: origin(
                    "018f0000-0000-7000-8000-000000000005",
                    "fork-open",
                    "session.fork",
                ),
                ephemeral: false,
            },
            "2026-08-28T10:00:00.000Z",
        );
        assert!(matches!(
            invalid,
            Err(StoreError::RewriteSourceNotTerminal(_))
        ));

        let binding = ForkGenesisBinding {
            thread: destination_id.to_owned(),
            origin: origin(destination_id, "fork-rpc", "session.fork"),
            ephemeral: false,
        };
        let operation = store
            .begin_fork_at_kernel_anchor(
                "fork-prefix",
                source_id,
                8,
                binding.clone(),
                "2026-08-28T10:00:00.000Z",
            )
            .expect("begin anchored fork");
        assert_eq!(operation.source_anchor.as_ref().unwrap().seq, 8);
        assert_eq!(operation.genesis_origin, Some(binding.origin.clone()));
        store.recover_rewrites().expect("publish anchored fork");

        let destination = store.root().join("threads").join(destination_id);
        let projected = validate_ledger(
            &fs::read(destination.join("main.jsonl")).expect("destination ledger"),
            1,
        )
        .expect("projected prefix");
        let genesis = projected.events.first().expect("genesis");
        assert_eq!(genesis.string_field("thread"), Some(destination_id));
        assert_eq!(genesis.origin_tuple().unwrap(), Some(binding.origin));
        assert!(
            projected
                .events
                .iter()
                .all(|event| event.string_field("run") != Some("r2"))
        );
        assert!(destination.join("assets").join(&retained.asset).is_file());
        assert!(!destination.join("assets").join(&later.asset).exists());
        assert_eq!(
            fs::read(destination.join("endpoint.jsonl")).expect("empty endpoint journal"),
            b""
        );
        assert_eq!(
            fs::read(destination.join("endpoint.lock")).expect("empty endpoint lock"),
            b""
        );
        assert_eq!(
            fs::read(destination.join(crate::SESSION_SETTINGS_FILE))
                .expect("forked session settings"),
            settings
        );

        let full_destination = "018f0000-0000-7000-8000-000000000006";
        let full = store
            .begin_fork(
                "fork-full-tail",
                source_id,
                full_destination,
                "2026-08-28T10:01:00.000Z",
            )
            .expect("legacy full-tail fork");
        assert!(full.source_anchor.is_none());
        assert!(full.genesis_origin.is_none());
        store.recover_rewrites().expect("publish full-tail fork");
        assert!(
            store
                .root()
                .join("threads")
                .join(full_destination)
                .join("main.jsonl")
                .is_file()
        );
    }

    #[test]
    fn fork_preserves_runtime_identity_selection() {
        let directory = tempfile::tempdir().expect("tempdir");
        let store = ThreadStore::open(directory.path()).expect("thread store");
        let source_id = "018f0000-0000-7000-8000-000000000003";
        let destination_id = "018f0000-0000-7000-8000-000000000004";
        let source = store.root().join("threads").join(source_id);
        fs::create_dir(&source).expect("source folder");
        let baseline = include_bytes!("../../../fixtures/invalid/_valid-baseline.jsonl");
        let mut events = baseline
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| serde_json::from_slice::<Value>(line).expect("baseline event"))
            .collect::<Vec<_>>();
        events[0]
            .as_object_mut()
            .unwrap()
            .insert("identity_profile".to_owned(), json!("auto"));
        for event in &mut events[4..] {
            let seq = event["seq"].as_u64().unwrap();
            event
                .as_object_mut()
                .unwrap()
                .insert("seq".to_owned(), json!(seq + 1));
        }
        events.insert(
            4,
            json!({"v":1,"seq":5,"turn":1,"kind":"state",
            "ts":"2026-08-27T09:00:00.000Z","subkind":"identity.selected",
            "payload":{"profile":"general"},"visibility":"runtime"}),
        );
        write_canonical_events(&source.join("main.jsonl"), &events);
        store
            .begin_fork(
                "fork-identity",
                source_id,
                destination_id,
                "2026-09-27T00:00:00.000Z",
            )
            .expect("begin fork");
        store.recover_rewrites().expect("publish fork");
        let destination = store.root().join("threads").join(destination_id);
        let projected = validate_ledger(&fs::read(destination.join("main.jsonl")).unwrap(), 1)
            .expect("fork ledger");
        assert_eq!(
            projected.events[0].string_field("identity_profile"),
            Some("auto")
        );
        let choice = projected
            .events
            .iter()
            .find(|event| event.string_field("subkind") == Some("identity.selected"))
            .expect("selection");
        assert_eq!(choice.effective_visibility(), Visibility::Runtime);
        let raw = serde_json::to_value(choice.raw()).unwrap();
        assert_eq!(raw["payload"]["profile"], "general");
    }

    #[test]
    fn anchored_fork_remaps_complete_child_edges() {
        let directory = tempfile::tempdir().expect("tempdir");
        let store = ThreadStore::open(directory.path()).expect("thread store");
        let source_id = "018f0000-0000-7000-8000-000000000010";
        let child_id = "018f0000-0000-7000-8000-000000000011";
        let destination_id = "018f0000-0000-7000-8000-000000000012";
        let source = store.root().join("threads").join(source_id);
        fs::create_dir(&source).expect("source folder");
        write_canonical_events(
            &source.join("main.jsonl"),
            &[
                json!({
                    "v":1,"seq":1,"kind":"genesis","ts":"2026-08-28T09:00:00.000Z",
                    "format":1,"min_reader":1,"min_writer":1,"thread":source_id,
                    "workspace":"ws","resume":"never","config":{"digest":"cfg"},
                    "origin_key":"create","origin_tuple":{
                        "principal":"p","client":"c","target":source_id,"op":"create","key":"create"
                    }
                }),
                json!({
                    "v":1,"seq":2,"kind":"input","ts":"2026-08-28T09:00:00.000Z",
                    "origin_key":"i1","origin_tuple":{
                        "principal":"p","client":"c","target":source_id,"op":"submit","key":"i1"
                    },"content":[{"type":"text","text":"spawn"}]
                }),
                json!({
                    "v":1,"seq":3,"kind":"turn_open","ts":"2026-08-28T09:00:00.000Z",
                    "turn":1,"trigger":{"inputs":[2]}
                }),
                json!({
                    "v":1,"seq":4,"kind":"spawn","ts":"2026-08-28T09:00:00.000Z",
                    "turn":1,"call":"call-child","child":child_id,"spawn_id":"spawn-1",
                    "resume":"never","seed":{"kinds":[]}
                }),
                json!({
                    "v":1,"seq":5,"kind":"child_result","ts":"2026-08-28T09:00:00.000Z",
                    "turn":1,"call":"call-child","child":child_id,"spawn_id":"spawn-1",
                    "outcome":"completed","summary":"ok"
                }),
                json!({
                    "v":1,"seq":6,"kind":"settle","ts":"2026-08-28T09:00:00.000Z",
                    "turn":1,"outcome":"completed"
                }),
            ],
        );
        write_canonical_events(
            &source.join(format!("{child_id}.jsonl")),
            &[
                json!({
                    "v":1,"seq":1,"kind":"genesis","ts":"2026-08-28T09:00:00.000Z",
                    "format":1,"min_reader":1,"min_writer":1,"thread":child_id,
                    "workspace":"ws","resume":"never","config":{"digest":"cfg"},
                    "parent":{"file":"main.jsonl","seq":4,"spawn_id":"spawn-1"},
                    "origin_key":"child-create","origin_tuple":{
                        "principal":"p","client":"c","target":child_id,"op":"spawn","key":"child-create"
                    }
                }),
                json!({
                    "v":1,"seq":2,"kind":"turn_open","ts":"2026-08-28T09:00:00.000Z",
                    "turn":1,"trigger":"genesis"
                }),
                json!({
                    "v":1,"seq":3,"kind":"settle","ts":"2026-08-28T09:00:00.000Z",
                    "turn":1,"outcome":"completed"
                }),
            ],
        );

        let operation = store
            .begin_fork_at_kernel_anchor(
                "fork-child-prefix",
                source_id,
                6,
                ForkGenesisBinding {
                    thread: destination_id.to_owned(),
                    origin: origin(destination_id, "fork-child", "session.fork"),
                    ephemeral: false,
                },
                "2026-08-28T10:00:00.000Z",
            )
            .expect("begin child fork");
        let mapped_child = operation.files[1].clone();
        store.recover_rewrites().expect("publish child fork");
        let destination = store.root().join("threads").join(destination_id);
        let parent = validate_ledger(&fs::read(destination.join("main.jsonl")).unwrap(), 1)
            .expect("mapped parent");
        let child = validate_ledger(&fs::read(destination.join(&mapped_child.dest)).unwrap(), 1)
            .expect("mapped child");
        let spawn = parent
            .events
            .iter()
            .find(|event| matches!(event.kind(), EventKind::Spawn))
            .expect("mapped spawn");
        let child_result = parent
            .events
            .iter()
            .find(|event| matches!(event.kind(), EventKind::ChildResult))
            .expect("mapped child result");
        assert_eq!(
            spawn.string_field("child"),
            Some(mapped_child.thread.as_str())
        );
        assert_eq!(
            child_result.string_field("child"),
            Some(mapped_child.thread.as_str())
        );
        let child_genesis = event_value(&child.events[0]).expect("child genesis");
        let parent_binding = child_genesis
            .get("parent")
            .and_then(Value::as_object)
            .expect("parent binding");
        assert_eq!(
            parent_binding.get("file").and_then(Value::as_str),
            Some("main.jsonl")
        );
        assert_eq!(
            parent_binding.get("seq").and_then(Value::as_u64),
            Some(spawn.seq())
        );
    }

    #[test]
    fn fork_and_redact_copy_unpoisoned_launch_binding_carriers() {
        for kind in [RewriteKind::Fork, RewriteKind::Redact] {
            let directory = tempfile::tempdir().expect("tempdir");
            let source = directory.path().join("source");
            let payload = directory.path().join("payload");
            fs::create_dir_all(source.join("assets")).expect("source assets");
            fs::create_dir_all(payload.join("assets")).expect("payload assets");
            let published = AssetStore::new(source.join("assets"))
                .expect("source store")
                .publish(b"launch bindings\n")
                .expect("binding asset");
            let digest = published
                .asset
                .strip_prefix("sha256-")
                .expect("asset digest");
            fs::write(
                source.join("main.jsonl"),
                format!("{{\"kind\":\"run_start\",\"launch_bindings_digest\":\"{digest}\"}}\n"),
            )
            .expect("source ledger");
            fs::write(payload.join("main.jsonl"), b"{}\n").expect("projected ledger");
            let operation = RewriteOperation {
                format: OP_FORMAT,
                id: "rewrite-test".to_owned(),
                kind,
                source: "source".to_owned(),
                dest: "dest".to_owned(),
                created_at: "2026-08-28T00:00:00.000Z".to_owned(),
                source_digest: format!("sha256-{}", "0".repeat(64)),
                phase: RewritePhase::Prepared,
                files: vec![RewriteFileMap {
                    source: "main.jsonl".to_owned(),
                    dest: "main.jsonl".to_owned(),
                    thread: "018f0000-0000-7000-8000-000000000003".to_owned(),
                }],
                source_anchor: None,
                genesis_origin: None,
                ephemeral: false,
                redaction: (kind == RewriteKind::Redact).then_some(RedactionInventory {
                    events: Vec::new(),
                    assets: Vec::new(),
                    fingerprints: vec![format!("sha256-{}", "1".repeat(64))],
                }),
            };

            copy_live_assets(&source, &payload, &operation).expect("copy carriers");
            assert_eq!(
                AssetStore::new(payload.join("assets"))
                    .expect("payload store")
                    .read_verified(&published.asset)
                    .expect("copied binding"),
                b"launch bindings\n"
            );
        }
    }
}
