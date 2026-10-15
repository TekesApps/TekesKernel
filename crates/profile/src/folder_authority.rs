//! Host-owned folder identity and trust. There is intentionally no RPC grant
//! operation: the local administrator writes this store outside worker sandboxes.
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Read};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};
use store::{AtomicPublisher, NamedLock};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FolderIdentity {
    // Decimal strings preserve the exact filesystem integers through JCS,
    // whose JSON numbers otherwise round through IEEE-754.
    device: String,
    inode: String,
    created_nanos: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FolderTrust {
    pub id: String,
    pub path: String,
    pub identity: FolderIdentity,
    pub revision: u64,
    pub trusted: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FolderTrustReceipt {
    pub revision: u64,
    pub folder: FolderTrust,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RecordedOperation {
    digest: String,
    receipt: FolderTrustReceipt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorityFile {
    format: u64,
    revision: u64,
    folders: BTreeMap<String, FolderTrust>,
    requests: BTreeMap<String, FolderTrust>,
    operations: BTreeMap<String, RecordedOperation>,
}

impl Default for AuthorityFile {
    fn default() -> Self {
        Self {
            format: 1,
            revision: 0,
            folders: BTreeMap::new(),
            requests: BTreeMap::new(),
            operations: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FolderAuthorityStore {
    root: PathBuf,
}

/// An admitted effect keeps this shared barrier until completion. Revocation
/// durably closes admission first; cleanup is acknowledged only after leases end.
pub struct FolderEffectLease {
    _lock: NamedLock,
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

impl FolderAuthorityStore {
    #[must_use]
    pub fn new(config_root: impl Into<PathBuf>) -> Self {
        Self {
            root: config_root.into(),
        }
    }

    fn directory(&self) -> PathBuf {
        self.root.join("config")
    }
    fn path(&self) -> PathBuf {
        self.directory().join("folder-authority.json")
    }

    fn load(&self) -> io::Result<AuthorityFile> {
        let mut file = match OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(self.path())
        {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Ok(AuthorityFile::default());
            }
            Err(error) => return Err(error),
        };
        if !file.metadata()?.is_file() || file.metadata()?.len() > 64 * 1024 * 1024 {
            return Err(invalid("invalid-folder-authority"));
        }
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        let value: AuthorityFile =
            serde_json::from_slice(&bytes).map_err(|_| invalid("invalid-folder-authority"))?;
        if serde_json_canonicalizer::to_vec(&value).map_err(io::Error::other)? != bytes {
            return Err(invalid("invalid-folder-authority"));
        }
        if value.format != 1
            || value
                .folders
                .iter()
                .any(|(id, record)| id != &record.id || !valid_id(id))
        {
            return Err(invalid("invalid-folder-authority"));
        }
        Ok(value)
    }

    /// Pure identity lookup. Missing, corrupt, or replaced records never grant.
    pub fn status(&self, path: &Path) -> io::Result<FolderTrust> {
        let mut current = identify(path)?;
        let state = self.load()?;
        if let Some(record) = state.folders.get(&current.id)
            && record.path == current.path
            && record.identity == current.identity
        {
            current = record.clone();
        }
        Ok(current)
    }

    pub fn record(&self, id: &str) -> io::Result<FolderTrust> {
        let record = self
            .load()?
            .folders
            .get(id)
            .cloned()
            .ok_or_else(|| invalid("folder-not-found"))?;
        let current = self.status(Path::new(&record.path))?;
        if current.id != id {
            return Err(invalid("folder-identity-changed"));
        }
        Ok(current)
    }

    pub fn registry_revision(&self) -> io::Result<u64> {
        Ok(self.load()?.revision)
    }

    pub fn pending(&self, request: &str) -> io::Result<FolderTrust> {
        self.load()?
            .requests
            .get(request)
            .cloned()
            .ok_or_else(|| invalid("folder-request-not-found"))
    }

    pub fn request(
        &self,
        operation: &str,
        expected: u64,
        path: &Path,
    ) -> io::Result<FolderTrustReceipt> {
        let folder = identify(path)?;
        self.mutate(
            operation,
            expected,
            serde_json::json!({"request":folder}),
            |state| {
                let mut current = folder.clone();
                if let Some(existing) = state.folders.get(&folder.id) {
                    if existing.path == folder.path {
                        current = existing.clone();
                    } else {
                        current.revision = existing
                            .revision
                            .checked_add(1)
                            .ok_or_else(|| invalid("folder-revision-overflow"))?;
                    }
                }
                if !state.folders.contains_key(&current.id) && state.folders.len() >= 1024 {
                    return Err(invalid("folder-limit"));
                }
                state.folders.insert(current.id.clone(), current.clone());
                state.requests.retain(|_, prior| prior.id != current.id);
                state.requests.insert(operation.to_owned(), current.clone());
                Ok(current)
            },
        )
    }

    /// Host administration only. The proof is control of the protected store,
    /// not a TTY, client boolean, permission mode or serializable bearer.
    pub fn prior_grant(
        &self,
        operation: &str,
        expected: u64,
        request: &str,
    ) -> io::Result<Option<FolderTrustReceipt>> {
        let digest = crate::digest(
            &serde_json_canonicalizer::to_vec(
                &serde_json::json!({"expected":expected,"request":{"grant":request}}),
            )
            .map_err(io::Error::other)?,
        );
        match self.load()?.operations.get(operation) {
            Some(prior) if prior.digest == digest => Ok(Some(prior.receipt.clone())),
            Some(_) => Err(invalid("folder-idempotency-conflict")),
            None => Ok(None),
        }
    }

    pub fn grant_from_host(
        &self,
        operation: &str,
        expected: u64,
        request: &str,
    ) -> io::Result<FolderTrustReceipt> {
        self.mutate(
            operation,
            expected,
            serde_json::json!({"grant":request}),
            |state| {
                let requested = state
                    .requests
                    .get(request)
                    .ok_or_else(|| invalid("folder-request-not-found"))?
                    .clone();
                let current_identity = identify(Path::new(&requested.path))?;
                let current = state
                    .folders
                    .get_mut(&requested.id)
                    .ok_or_else(|| invalid("folder-not-found"))?;
                if current_identity.id != requested.id
                    || current_identity.identity != requested.identity
                    || current.revision != requested.revision
                {
                    return Err(invalid("stale-folder-request"));
                }
                for protected in tools::kernel_protected_roots(&self.root) {
                    if Path::new(&requested.path).starts_with(&protected) {
                        return Err(invalid("protected-folder"));
                    }
                }
                current.trusted = true;
                current.revision = current
                    .revision
                    .checked_add(1)
                    .ok_or_else(|| invalid("folder-revision-overflow"))?;
                let result = current.clone();
                state.requests.remove(request);
                Ok(result)
            },
        )
    }

    pub fn revoke(
        &self,
        operation: &str,
        expected: u64,
        folder: &str,
    ) -> io::Result<FolderTrustReceipt> {
        self.mutate(
            operation,
            expected,
            serde_json::json!({"revoke":folder}),
            |state| {
                let current = state
                    .folders
                    .get_mut(folder)
                    .ok_or_else(|| invalid("folder-not-found"))?;
                current.trusted = false;
                current.revision = current
                    .revision
                    .checked_add(1)
                    .ok_or_else(|| invalid("folder-revision-overflow"))?;
                Ok(current.clone())
            },
        )
    }

    fn mutate(
        &self,
        operation: &str,
        expected: u64,
        request: serde_json::Value,
        apply: impl FnOnce(&mut AuthorityFile) -> io::Result<FolderTrust>,
    ) -> io::Result<FolderTrustReceipt> {
        if operation.is_empty() || operation.len() > 128 || operation.chars().any(char::is_control)
        {
            return Err(invalid("invalid-folder-operation"));
        }
        // ConfigRepository owns preparation and symlink/ownership validation.
        crate::ConfigRepository::open(&self.root).map_err(io::Error::other)?;
        let _lock = NamedLock::exclusive(self.directory().join(".folder-authority.lock"))
            .map_err(io::Error::other)?;
        let mut state = self.load()?;
        let digest = crate::digest(
            &serde_json_canonicalizer::to_vec(
                &serde_json::json!({"expected":expected,"request":request}),
            )
            .map_err(io::Error::other)?,
        );
        if let Some(prior) = state.operations.get(operation) {
            return if prior.digest == digest {
                Ok(prior.receipt.clone())
            } else {
                Err(invalid("folder-idempotency-conflict"))
            };
        }
        if state.revision != expected {
            return Err(invalid("stale-folder-revision"));
        }
        let folder = apply(&mut state)?;
        state.revision = state
            .revision
            .checked_add(1)
            .ok_or_else(|| invalid("folder-revision-overflow"))?;
        let receipt = FolderTrustReceipt {
            revision: state.revision,
            folder,
        };
        state.operations.insert(
            operation.to_owned(),
            RecordedOperation {
                digest,
                receipt: receipt.clone(),
            },
        );
        // Expired retries still carry an older expected revision and cannot
        // reapply. Never let request receipt exhaustion disable revocation.
        while state.operations.len() > 4096 {
            let oldest = state
                .operations
                .iter()
                .min_by_key(|(_, record)| record.receipt.revision)
                .map(|(id, _)| id.clone())
                .ok_or_else(|| invalid("folder-operation-limit"))?;
            state.operations.remove(&oldest);
        }
        let bytes = serde_json_canonicalizer::to_vec(&state).map_err(io::Error::other)?;
        if bytes.len() > 64 * 1024 * 1024 {
            return Err(invalid("folder-authority-limit"));
        }
        AtomicPublisher::replace(self.path(), &bytes).map_err(io::Error::other)?;
        Ok(receipt)
    }

    pub fn admit(&self, captured: &FolderTrust) -> io::Result<FolderEffectLease> {
        if !valid_id(&captured.id) {
            return Err(invalid("invalid-folder-id"));
        }
        let lock = NamedLock::shared(
            self.directory()
                .join(format!(".folder-effect-{}.lock", captured.id)),
        )
        .map_err(io::Error::other)?;
        let current = self.status(Path::new(&captured.path))?;
        if !captured.trusted || !current.trusted || current != *captured {
            return Err(invalid("folder-trust-revoked"));
        }
        Ok(FolderEffectLease { _lock: lock })
    }

    /// This is an effect-barrier ACK, not an assertion about idle pool cleanup.
    pub fn effects_drained(&self, id: &str) -> io::Result<bool> {
        if !valid_id(id) {
            return Err(invalid("invalid-folder-id"));
        }
        match NamedLock::try_exclusive(self.directory().join(format!(".folder-effect-{id}.lock"))) {
            Ok(_) => Ok(true),
            Err(store::StoreError::Busy) => Ok(false),
            Err(error) => Err(io::Error::other(error)),
        }
    }
}

fn valid_id(id: &str) -> bool {
    id.len() == 64 && id.bytes().all(|b| b.is_ascii_hexdigit())
}

fn identify(path: &Path) -> io::Result<FolderTrust> {
    let path = path.canonicalize()?;
    if path.parent().is_none() {
        return Err(invalid("root-folder-refused"));
    }
    let meta = fs::metadata(&path)?;
    if !meta.is_dir() {
        return Err(invalid("not-a-folder"));
    }
    let created_nanos = meta
        .created()?
        .duration_since(UNIX_EPOCH)
        .map_err(|_| invalid("unsupported-folder-identity"))?
        .as_nanos();
    let identity = FolderIdentity {
        device: meta.dev().to_string(),
        inode: meta.ino().to_string(),
        created_nanos: created_nanos.to_string(),
    };
    let path = path
        .to_str()
        .ok_or_else(|| invalid("non-utf8-folder"))?
        .to_owned();
    let id = crate::digest(
        &serde_json_canonicalizer::to_vec(&("folder-identity/v1", &identity))
            .map_err(io::Error::other)?,
    );
    Ok(FolderTrust {
        id,
        path,
        identity,
        revision: 0,
        trusted: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grants_are_identity_bound_cas_idempotent_and_revocation_closes_admission() {
        let state = tempfile::tempdir().unwrap();
        let project = tempfile::tempdir().unwrap();
        let authority = FolderAuthorityStore::new(state.path());
        assert!(!authority.status(project.path()).unwrap().trusted);
        let requested = authority.request("request", 0, project.path()).unwrap();
        let grant = authority.grant_from_host("grant", 1, "request").unwrap();
        let lease = authority.admit(&grant.folder).unwrap();
        let revoked = authority.revoke("revoke", 2, &requested.folder.id).unwrap();
        assert!(!revoked.folder.trusted);
        assert!(!authority.effects_drained(&requested.folder.id).unwrap());
        assert!(authority.admit(&grant.folder).is_err());
        assert_eq!(
            authority.grant_from_host("grant", 1, "request").unwrap(),
            grant
        );
        assert!(!authority.status(project.path()).unwrap().trusted);
        assert_eq!(
            authority.prior_grant("grant", 1, "request").unwrap(),
            Some(grant)
        );
        assert!(authority.prior_grant("grant", 2, "request").is_err());
        assert!(authority.request("request", 0, state.path()).is_err());
        drop(lease);
        assert!(authority.effects_drained(&requested.folder.id).unwrap());
    }

    #[test]
    fn aliases_share_identity_but_replacement_and_corruption_do_not_inherit_grants() {
        let state = tempfile::tempdir().unwrap();
        let base = tempfile::tempdir().unwrap();
        let path = base.path().join("project");
        fs::create_dir(&path).unwrap();
        let alias = base.path().join("alias");
        std::os::unix::fs::symlink(&path, &alias).unwrap();
        let authority = FolderAuthorityStore::new(state.path());
        let r = authority.request("r", 0, &path).unwrap();
        authority.grant_from_host("g", 1, "r").unwrap();
        assert_eq!(authority.status(&alias).unwrap().id, r.folder.id);
        fs::rename(&path, base.path().join("old")).unwrap();
        fs::create_dir(&path).unwrap();
        assert!(!authority.status(&path).unwrap().trusted);
        fs::write(authority.path(), b"broken").unwrap();
        assert!(authority.status(&path).is_err());
    }
    #[test]
    fn retained_receipt_capacity_cannot_disable_revocation() {
        let root = tempfile::tempdir().unwrap();
        let project = tempfile::tempdir().unwrap();
        let authority = FolderAuthorityStore::new(root.path());
        authority.request("r", 0, project.path()).unwrap();
        let grant = authority.grant_from_host("g", 1, "r").unwrap();
        let mut state = authority.load().unwrap();
        state.revision = 4096;
        state.operations.clear();
        for revision in 1..=4096 {
            state.operations.insert(
                format!("old-{revision}"),
                RecordedOperation {
                    digest: "0".repeat(64),
                    receipt: FolderTrustReceipt {
                        revision,
                        folder: grant.folder.clone(),
                    },
                },
            );
        }
        AtomicPublisher::replace(
            authority.path(),
            &serde_json_canonicalizer::to_vec(&state).unwrap(),
        )
        .unwrap();
        let revoked = authority.revoke("revoke", 4096, &grant.folder.id).unwrap();
        assert!(!revoked.folder.trusted);
        assert_eq!(authority.load().unwrap().operations.len(), 4096);
        assert_eq!(
            authority.revoke("revoke", 4096, &grant.folder.id).unwrap(),
            revoked
        );
        assert!(authority.grant_from_host("old-1", 0, "r").is_err());
        assert!(!authority.status(project.path()).unwrap().trusted);
    }
}
