use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use store::{FullSync, NamedLock};
use thiserror::Error;

use crate::{
    ClientRequest, RespondReceipt, RpcResult, ServerResponse, validate_response, validate_rpc_id,
};

const VERSION: u64 = 2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RpcClaim {
    rpc_id: String,
    operation: String,
    request_sha256: String,
    target_session: Option<String>,
    projection_metadata: Option<ProjectionMetadata>,
}

impl RpcClaim {
    #[must_use]
    pub fn rpc_id(&self) -> &str {
        &self.rpc_id
    }

    #[must_use]
    pub fn method(&self) -> &str {
        &self.operation
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RpcBegin {
    Execute { claim: RpcClaim, recovering: bool },
    Completed(RpcResult),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RpcLookup {
    Missing,
    Pending(RpcClaim),
    Completed(RpcResult),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Phase {
    Prepared,
    HandedOff,
    Complete,
    Retired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProjectionMetadata {
    client_time_zone: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RpcDurableIdentity {
    pub kind: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    v: u64,
    ordinal: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    rpc_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rpc_sha256: Option<String>,
    operation: String,
    request_sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_session: Option<String>,
    phase: Phase,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    durable_identity: Option<RpcDurableIdentity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    projection_metadata: Option<ProjectionMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_b64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retired_reason: Option<String>,
}

enum Entry {
    Active {
        claim: Box<RpcClaim>,
        phase: Phase,
        result: Option<RpcResult>,
        handoff: Option<(String, Option<RpcDurableIdentity>)>,
    },
    Retired,
}

/// Contract-shaped endpoint-wide mutation carrier. Each rpcId owns a
/// canonical phase journal under `endpoint-management/rpc/`.
pub struct RpcRegistry {
    root: PathBuf,
    rpc_root: PathBuf,
    lock_path: PathBuf,
}

impl RpcRegistry {
    pub fn open(endpoint_root: impl AsRef<Path>) -> Result<Self, RpcRegistryError> {
        let root = store::endpoint_management_root(endpoint_root.as_ref())?;
        let rpc_root = root.join("rpc");
        fs::create_dir_all(&rpc_root)?;
        let lock_path = root.join("rpc.lock");
        let missing = !lock_path.exists();
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&lock_path)?;
        if missing {
            fs::File::open(&root)?.sync_all()?;
        }
        let registry = Self {
            root,
            rpc_root,
            lock_path,
        };
        registry.repair_all()?;
        Ok(registry)
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.root
    }

    pub fn begin(&self, request: &ClientRequest) -> Result<RpcBegin, RpcRegistryError> {
        let claim = make_claim(request)?;
        let _lock = NamedLock::exclusive(&self.lock_path)?;
        match self.read_unlocked(&claim.rpc_id, false)? {
            Some(Entry::Active {
                claim: existing,
                result,
                ..
            }) => {
                same_claim(&existing, &claim)?;
                Ok(match result {
                    Some(result) => RpcBegin::Completed(result),
                    None => RpcBegin::Execute {
                        claim,
                        recovering: true,
                    },
                })
            }
            Some(Entry::Retired) => Err(RpcRegistryError::Conflict(claim.rpc_id)),
            None => {
                self.publish_prepared(&claim)?;
                Ok(RpcBegin::Execute {
                    claim,
                    recovering: false,
                })
            }
        }
    }

    pub fn lookup(&self, request: &ClientRequest) -> Result<RpcLookup, RpcRegistryError> {
        let claim = make_claim(request)?;
        let _lock = NamedLock::shared(&self.lock_path)?;
        match self.read_unlocked(&claim.rpc_id, false)? {
            None => Ok(RpcLookup::Missing),
            Some(Entry::Retired) => Err(RpcRegistryError::Conflict(claim.rpc_id)),
            Some(Entry::Active {
                claim: existing,
                result,
                ..
            }) => {
                same_claim(&existing, &claim)?;
                Ok(match result {
                    Some(result) => RpcLookup::Completed(result),
                    None => RpcLookup::Pending(claim),
                })
            }
        }
    }

    pub fn handoff(
        &self,
        claim: &RpcClaim,
    ) -> Result<Option<(String, Option<RpcDurableIdentity>)>, RpcRegistryError> {
        let _lock = NamedLock::shared(&self.lock_path)?;
        let entry = self
            .read_unlocked(&claim.rpc_id, false)?
            .ok_or_else(|| RpcRegistryError::MissingClaim(claim.rpc_id.clone()))?;
        let Entry::Active {
            claim: existing,
            handoff,
            ..
        } = entry
        else {
            return Err(RpcRegistryError::Conflict(claim.rpc_id.clone()));
        };
        same_claim(&existing, claim)?;
        Ok(handoff)
    }

    pub fn complete(
        &self,
        claim: &RpcClaim,
        result: &RpcResult,
    ) -> Result<RpcResult, RpcRegistryError> {
        result.validate()?;
        let _lock = NamedLock::exclusive(&self.lock_path)?;
        let entry = self
            .read_unlocked(&claim.rpc_id, false)?
            .ok_or_else(|| RpcRegistryError::MissingClaim(claim.rpc_id.clone()))?;
        let Entry::Active {
            claim: existing,
            phase,
            result: old,
            handoff,
        } = entry
        else {
            return Err(RpcRegistryError::Conflict(claim.rpc_id.clone()));
        };
        same_claim(&existing, claim)?;
        if let Some(old) = old {
            return if old == *result {
                Ok(old)
            } else {
                Err(RpcRegistryError::ResultConflict(claim.rpc_id.clone()))
            };
        }
        if !matches!(phase, Phase::Prepared | Phase::HandedOff) {
            return Err(RpcRegistryError::Corruption(
                "unsupported completion phase".to_owned(),
            ));
        }
        let response_bytes = if claim.operation == "respond" {
            let value = result.value.as_ref().ok_or_else(|| {
                RpcRegistryError::Corruption("respond completion omitted receipt".to_owned())
            })?;
            let bytes = value.canonical_bytes()?;
            let receipt: RespondReceipt = serde_json::from_slice(&bytes)?;
            if !receipt.accepted || receipt.reason.is_some() {
                return Err(RpcRegistryError::Corruption(
                    "respond carrier may cache only accepted:true".to_owned(),
                ));
            }
            bytes
        } else {
            serde_json_canonicalizer::to_vec(&ServerResponse {
                envelope_type: "server-response".to_owned(),
                rpc_id: claim.rpc_id.clone(),
                result: result.clone(),
            })
            .map_err(|error| RpcRegistryError::Canonical(error.to_string()))?
        };
        let (delivery, durable_identity) = match (phase, handoff) {
            (Phase::Prepared, None) => (
                None,
                RpcDurableIdentity {
                    kind: "rpc-result".to_owned(),
                    id: digest(&response_bytes),
                    seq: None,
                },
            ),
            (Phase::HandedOff, Some((delivery, durable_identity))) => (
                Some(delivery),
                durable_identity.unwrap_or_else(|| RpcDurableIdentity {
                    kind: "rpc-result".to_owned(),
                    id: digest(&response_bytes),
                    seq: None,
                }),
            ),
            _ => {
                return Err(RpcRegistryError::Corruption(
                    "rpc phase omitted its handoff proof".to_owned(),
                ));
            }
        };
        self.append(
            &claim.rpc_id,
            &Record {
                v: VERSION,
                ordinal: if phase == Phase::Prepared { 1 } else { 2 },
                rpc_id: Some(claim.rpc_id.clone()),
                rpc_sha256: None,
                operation: claim.operation.clone(),
                request_sha256: claim.request_sha256.clone(),
                target_session: claim.target_session.clone(),
                phase: Phase::Complete,
                delivery,
                durable_identity: Some(durable_identity),
                projection_metadata: claim.projection_metadata.clone(),
                response_b64: Some(BASE64.encode(response_bytes)),
                retired_reason: None,
            },
        )?;
        Ok(result.clone())
    }

    pub fn mark_handed_off(
        &self,
        claim: &RpcClaim,
        delivery: &str,
        durable_identity: Option<RpcDurableIdentity>,
    ) -> Result<(), RpcRegistryError> {
        if delivery.is_empty() {
            return Err(RpcRegistryError::Corruption(
                "handoff delivery must be nonempty".to_owned(),
            ));
        }
        if let Some(identity) = durable_identity.as_ref() {
            validate_durable_identity(identity)?;
        }
        let _lock = NamedLock::exclusive(&self.lock_path)?;
        let entry = self
            .read_unlocked(&claim.rpc_id, false)?
            .ok_or_else(|| RpcRegistryError::MissingClaim(claim.rpc_id.clone()))?;
        let Entry::Active {
            claim: existing,
            phase,
            handoff,
            ..
        } = entry
        else {
            return Err(RpcRegistryError::Conflict(claim.rpc_id.clone()));
        };
        same_claim(&existing, claim)?;
        if phase == Phase::HandedOff {
            return if handoff == Some((delivery.to_owned(), durable_identity)) {
                Ok(())
            } else {
                Err(RpcRegistryError::Corruption(
                    "handoff proof changed during retry".to_owned(),
                ))
            };
        }
        if phase != Phase::Prepared {
            return Err(RpcRegistryError::Corruption(
                "handoff cannot follow terminal completion".to_owned(),
            ));
        }
        self.append(
            &claim.rpc_id,
            &Record {
                v: VERSION,
                ordinal: 1,
                rpc_id: Some(claim.rpc_id.clone()),
                rpc_sha256: None,
                operation: claim.operation.clone(),
                request_sha256: claim.request_sha256.clone(),
                target_session: claim.target_session.clone(),
                phase: Phase::HandedOff,
                delivery: Some(delivery.to_owned()),
                durable_identity,
                projection_metadata: claim.projection_metadata.clone(),
                response_b64: None,
                retired_reason: None,
            },
        )
    }

    fn publish_prepared(&self, claim: &RpcClaim) -> Result<(), RpcRegistryError> {
        let hash = rpc_hash(&claim.rpc_id);
        let shard = self.rpc_root.join(&hash[..2]);
        let new_shard = !shard.exists();
        fs::create_dir_all(&shard)?;
        if new_shard {
            fs::File::open(&self.rpc_root)?.sync_all()?;
        }
        let path = self.record_path(&hash);
        let record = Record {
            v: VERSION,
            ordinal: 0,
            rpc_id: Some(claim.rpc_id.clone()),
            rpc_sha256: None,
            operation: claim.operation.clone(),
            request_sha256: claim.request_sha256.clone(),
            target_session: claim.target_session.clone(),
            phase: Phase::Prepared,
            delivery: None,
            durable_identity: None,
            projection_metadata: claim.projection_metadata.clone(),
            response_b64: None,
            retired_reason: None,
        };
        let mut bytes = canonical(&record)?;
        bytes.push(b'\n');
        let temp = shard.join(format!(".{hash}.tmp"));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        file.write_all(&bytes)?;
        FullSync::full_sync(&file)?;
        drop(file);
        fs::rename(temp, path)?;
        fs::File::open(shard)?.sync_all()?;
        Ok(())
    }

    fn append(&self, rpc_id: &str, record: &Record) -> Result<(), RpcRegistryError> {
        let mut bytes = canonical(record)?;
        bytes.push(b'\n');
        let mut file = OpenOptions::new()
            .append(true)
            .open(self.record_path(&rpc_hash(rpc_id)))?;
        file.write_all(&bytes)?;
        FullSync::full_sync(&file)?;
        Ok(())
    }

    fn read_unlocked(
        &self,
        rpc_id: &str,
        partial: bool,
    ) -> Result<Option<Entry>, RpcRegistryError> {
        let hash = rpc_hash(rpc_id);
        let path = self.record_path(&hash);
        if !path.exists() {
            return Ok(None);
        }
        Ok(Some(parse(&fs::read(path)?, &hash, partial)?.0))
    }

    fn repair_all(&self) -> Result<(), RpcRegistryError> {
        let _lock = NamedLock::exclusive(&self.lock_path)?;
        for shard in fs::read_dir(&self.rpc_root)? {
            let shard = shard?;
            if !shard.file_type()?.is_dir() {
                return Err(RpcRegistryError::Corruption(
                    "rpc root contains non-shard entry".to_owned(),
                ));
            }
            for file in fs::read_dir(shard.path())? {
                let file = file?;
                let name = file.file_name().to_string_lossy().into_owned();
                if let Some(hash) = name
                    .strip_prefix('.')
                    .and_then(|name| name.strip_suffix(".tmp"))
                {
                    let bytes = fs::read(file.path())?;
                    let target = self.record_path(hash);
                    if target.exists() {
                        return Err(RpcRegistryError::Corruption(
                            "rpc temp and published carrier coexist".to_owned(),
                        ));
                    }
                    if bytes.last() == Some(&b'\n') && parse(&bytes, hash, false).is_ok() {
                        fs::rename(file.path(), target)?;
                    } else {
                        fs::remove_file(file.path())?;
                    }
                    fs::File::open(shard.path())?.sync_all()?;
                    continue;
                }
                let hash = name.strip_suffix(".jsonl").ok_or_else(|| {
                    RpcRegistryError::Corruption("rpc shard contains unknown entry".to_owned())
                })?;
                let bytes = fs::read(file.path())?;
                let (_, valid) = parse(&bytes, hash, true)?;
                if valid != bytes.len() {
                    let handle = OpenOptions::new().write(true).open(file.path())?;
                    handle.set_len(valid as u64)?;
                    FullSync::full_sync(&handle)?;
                }
            }
        }
        Ok(())
    }

    fn record_path(&self, hash: &str) -> PathBuf {
        self.rpc_root.join(&hash[..2]).join(format!("{hash}.jsonl"))
    }
}

fn make_claim(request: &ClientRequest) -> Result<RpcClaim, RpcRegistryError> {
    if request.envelope_type != "client-request" {
        return Err(crate::rpc::RequestError::Envelope.into());
    }
    validate_rpc_id(&request.rpc_id)?;
    if request.method.is_empty() {
        return Err(RpcRegistryError::InvalidMethod);
    }
    let request_bytes = serde_json_canonicalizer::to_vec(request)
        .map_err(|error| RpcRegistryError::Canonical(error.to_string()))?;
    let payload: serde_json::Value = serde_json::from_slice(&request.payload.canonical_bytes()?)?;
    let target_session = payload
        .get("sessionId")
        .or_else(|| payload.pointer("/result/value/sessionId"))
        .and_then(serde_json::Value::as_str)
        .filter(|value| uuid::Uuid::parse_str(value).is_ok())
        .map(str::to_owned);
    let projection_metadata = (request.method == "session.prompt")
        .then(|| {
            payload
                .get("clientTimeZone")
                .and_then(serde_json::Value::as_str)
                .map(|value| ProjectionMetadata {
                    client_time_zone: value.to_owned(),
                })
        })
        .flatten();
    Ok(RpcClaim {
        rpc_id: request.rpc_id.clone(),
        operation: request.method.clone(),
        request_sha256: digest(&request_bytes),
        target_session,
        projection_metadata,
    })
}

fn same_claim(left: &RpcClaim, right: &RpcClaim) -> Result<(), RpcRegistryError> {
    if left == right {
        Ok(())
    } else {
        Err(RpcRegistryError::Conflict(right.rpc_id.clone()))
    }
}

fn parse(
    bytes: &[u8],
    expected_hash: &str,
    partial: bool,
) -> Result<(Entry, usize), RpcRegistryError> {
    let mut rows = Vec::new();
    let mut offset = 0;
    while offset < bytes.len() {
        let Some(end) = bytes[offset..].iter().position(|byte| *byte == b'\n') else {
            if partial && !rows.is_empty() {
                break;
            }
            return Err(RpcRegistryError::Corruption("partial rpc tail".to_owned()));
        };
        let end = offset + end;
        let line = &bytes[offset..end];
        let row: Record = serde_json::from_slice(line)?;
        if line.is_empty() || canonical(&row)? != line || row.ordinal != rows.len() as u64 {
            return Err(RpcRegistryError::Corruption(
                "noncanonical row or ordinal gap".to_owned(),
            ));
        }
        rows.push(row);
        offset = end + 1;
    }
    let first = rows
        .first()
        .ok_or_else(|| RpcRegistryError::Corruption("empty rpc carrier".to_owned()))?;
    if first.v != VERSION {
        return Err(RpcRegistryError::Corruption(
            "unsupported rpc version".to_owned(),
        ));
    }
    if first.phase == Phase::Retired {
        if rows.len() == 1
            && first.rpc_id.is_none()
            && first.rpc_sha256.as_deref() == Some(expected_hash)
            && first.retired_reason.as_deref() == Some("redact")
            && first.response_b64.is_none()
        {
            return Ok((Entry::Retired, offset));
        }
        return Err(RpcRegistryError::Corruption(
            "invalid retired rpc".to_owned(),
        ));
    }
    if first.phase != Phase::Prepared
        || first.rpc_sha256.is_some()
        || first.delivery.is_some()
        || first.durable_identity.is_some()
        || first.response_b64.is_some()
        || first.retired_reason.is_some()
    {
        return Err(RpcRegistryError::Corruption(
            "invalid prepared row".to_owned(),
        ));
    }
    let rpc_id = first
        .rpc_id
        .as_ref()
        .ok_or_else(|| RpcRegistryError::Corruption("prepared omitted rpc_id".to_owned()))?;
    validate_rpc_id(rpc_id)?;
    if rpc_hash(rpc_id) != expected_hash {
        return Err(RpcRegistryError::Corruption(
            "rpc filename mismatch".to_owned(),
        ));
    }
    valid_hex(&first.request_sha256)?;
    let claim = RpcClaim {
        rpc_id: rpc_id.clone(),
        operation: first.operation.clone(),
        request_sha256: first.request_sha256.clone(),
        target_session: first.target_session.clone(),
        projection_metadata: first.projection_metadata.clone(),
    };
    let mut phase = Phase::Prepared;
    let mut result = None;
    let mut handoff = None;
    for row in rows.iter().skip(1) {
        if row.v != VERSION
            || row.rpc_id.as_deref() != Some(&claim.rpc_id)
            || row.rpc_sha256.is_some()
            || row.operation != claim.operation
            || row.request_sha256 != claim.request_sha256
            || row.target_session != claim.target_session
            || row.projection_metadata != claim.projection_metadata
            || row.retired_reason.is_some()
        {
            return Err(RpcRegistryError::Corruption(
                "rpc fields changed".to_owned(),
            ));
        }
        match (phase, row.phase) {
            (Phase::Prepared, Phase::HandedOff)
                if row
                    .delivery
                    .as_deref()
                    .is_some_and(|delivery| !delivery.is_empty())
                    && row.response_b64.is_none() =>
            {
                handoff = Some((
                    row.delivery.clone().expect("guarded"),
                    row.durable_identity.clone(),
                ));
            }
            (Phase::Prepared, Phase::Complete)
                if row.delivery.is_none()
                    && row.durable_identity.is_some()
                    && row.response_b64.is_some() =>
            {
                let response_bytes = decode_response(row)?;
                let identity = row.durable_identity.as_ref().expect("guarded");
                if identity.kind != "rpc-result"
                    || identity.id != digest(&response_bytes)
                    || identity.seq.is_some()
                {
                    return Err(RpcRegistryError::Corruption(
                        "response identity mismatch".to_owned(),
                    ));
                }
                result = Some(parse_cached_result(&claim, &response_bytes)?);
            }
            (Phase::HandedOff, Phase::Complete)
                if handoff.as_ref().is_some_and(|(delivery, identity)| {
                    row.delivery.as_deref() == Some(delivery)
                        && identity
                            .as_ref()
                            .is_none_or(|identity| row.durable_identity.as_ref() == Some(identity))
                }) && row.durable_identity.is_some()
                    && row.response_b64.is_some() =>
            {
                validate_durable_identity(row.durable_identity.as_ref().expect("guarded"))?;
                let response_bytes = decode_response(row)?;
                result = Some(parse_cached_result(&claim, &response_bytes)?);
            }
            _ => {
                return Err(RpcRegistryError::Corruption(
                    "invalid phase transition".to_owned(),
                ));
            }
        }
        phase = row.phase;
    }
    Ok((
        Entry::Active {
            claim: Box::new(claim),
            phase,
            result,
            handoff,
        },
        offset,
    ))
}

fn decode_response(row: &Record) -> Result<Vec<u8>, RpcRegistryError> {
    let encoded = row.response_b64.as_ref().expect("guarded");
    let response_bytes = BASE64
        .decode(encoded)
        .map_err(|_| RpcRegistryError::Corruption("invalid response base64".to_owned()))?;
    if BASE64.encode(&response_bytes) != *encoded {
        return Err(RpcRegistryError::Corruption(
            "noncanonical base64".to_owned(),
        ));
    }
    Ok(response_bytes)
}

fn parse_cached_result(
    claim: &RpcClaim,
    response_bytes: &[u8],
) -> Result<RpcResult, RpcRegistryError> {
    if claim.operation == "respond" {
        let value = schema::IJsonValue::parse(response_bytes)?;
        let receipt: RespondReceipt = serde_json::from_slice(response_bytes)?;
        if !receipt.accepted || receipt.reason.is_some() {
            return Err(RpcRegistryError::Corruption(
                "invalid cached respond receipt".to_owned(),
            ));
        }
        Ok(RpcResult {
            ok: true,
            value: Some(value),
            error: None,
        })
    } else {
        let response: ServerResponse = serde_json::from_slice(response_bytes)?;
        validate_response(&response, &claim.rpc_id)?;
        if serde_json_canonicalizer::to_vec(&response)
            .map_err(|error| RpcRegistryError::Canonical(error.to_string()))?
            != response_bytes
        {
            return Err(RpcRegistryError::Corruption(
                "noncanonical cached response".to_owned(),
            ));
        }
        Ok(response.result)
    }
}

fn validate_durable_identity(identity: &RpcDurableIdentity) -> Result<(), RpcRegistryError> {
    if identity.kind.is_empty() || identity.id.is_empty() {
        Err(RpcRegistryError::Corruption(
            "durable identity fields must be nonempty".to_owned(),
        ))
    } else {
        Ok(())
    }
}

fn canonical(record: &Record) -> Result<Vec<u8>, RpcRegistryError> {
    serde_json_canonicalizer::to_vec(record)
        .map_err(|error| RpcRegistryError::Canonical(error.to_string()))
}

fn rpc_hash(value: &str) -> String {
    digest(value.as_bytes())
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn valid_hex(value: &str) -> Result<(), RpcRegistryError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        Ok(())
    } else {
        Err(RpcRegistryError::Corruption("invalid sha256".to_owned()))
    }
}

#[derive(Debug, Error)]
pub enum RpcRegistryError {
    #[error("rpc registry IO failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("rpc registry JSON failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("rpc registry store failed: {0}")]
    Store(#[from] store::StoreError),
    #[error("rpc registry schema failed: {0}")]
    Schema(#[from] schema::SchemaError),
    #[error("rpc request failed validation: {0}")]
    Request(#[from] crate::rpc::RequestError),
    #[error("rpc method must not be empty")]
    InvalidMethod,
    #[error("rpcId {0} was reused with another operation or payload")]
    Conflict(String),
    #[error("rpcId {0} has no durable prepared row")]
    MissingClaim(String),
    #[error("rpcId {0} completed with different result bytes")]
    ResultConflict(String),
    #[error("rpc registry canonicalization failed: {0}")]
    Canonical(String),
    #[error("rpc registry corruption: {0}")]
    Corruption(String),
}
