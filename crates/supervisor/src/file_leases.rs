//! Short-lived immutable preview snapshots. Lease IDs are never filesystem paths.
use base64::Engine as _;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const MAX_BYTES: usize = 64 * 1024 * 1024;
const MAX_LEASES: usize = 64;
const TTL: Duration = Duration::from_secs(120);

struct Lease {
    principal: String,
    bytes: Arc<[u8]>,
    expires: Instant,
}

#[derive(Default)]
pub struct FileLeases(Mutex<HashMap<String, Lease>>);

pub struct WorkspaceFileTransfer {
    management: endpoint::ManagementStore,
    attachments: endpoint::AttachmentAuthority,
    leases: FileLeases,
}

impl WorkspaceFileTransfer {
    pub fn open(root: &std::path::Path) -> Result<Self, String> {
        Ok(Self {
            management: endpoint::ManagementStore::open(root).map_err(|error| error.to_string())?,
            attachments: endpoint::AttachmentAuthority::open(root),
            leases: FileLeases::default(),
        })
    }
}

impl transport::FileTransferAuthority for WorkspaceFileTransfer {
    fn prepare(&self, request: serde_json::Value) -> Result<serde_json::Value, String> {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        struct WorkspaceRequest {
            workspace_id: String,
            path: String,
            maximum_bytes: usize,
            prefer_local: bool,
        }
        #[derive(serde::Deserialize)]
        #[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
        enum Source {
            SessionAttachment {
                #[serde(rename = "sessionId")]
                session_id: String,
                #[serde(rename = "attachmentId")]
                attachment_id: String,
            },
        }
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        struct AttachmentRequest {
            source: Source,
            maximum_bytes: usize,
            prefer_local: bool,
        }
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum Request {
            Workspace(WorkspaceRequest),
            Attachment(AttachmentRequest),
        }
        let request: Request =
            serde_json::from_value(request).map_err(|_| "Invalid preview request")?;
        match request {
            Request::Workspace(request) => {
                let root = self
                    .management
                    .workspace_path(&request.workspace_id)
                    .map_err(|_| "Unknown workspace")?;
                let _ = request.prefer_local; // Remote snapshots avoid disclosing local paths.
                let bytes = workspace_service::file_snapshot(
                    std::path::Path::new(&root),
                    &request.path,
                    request.maximum_bytes.min(MAX_BYTES),
                )
                .map_err(|error| error.code.to_owned())?;
                self.leases.insert("authenticated-loopback", bytes)
            }
            Request::Attachment(request) => {
                if request.maximum_bytes == 0 {
                    return Err("Invalid snapshot limit".into());
                }
                let _ = request.prefer_local;
                let Source::SessionAttachment {
                    session_id,
                    attachment_id,
                } = request.source;
                // One digest may be referenced as an image or as an uploaded
                // file; the image read is authoritative and the file read is
                // the fallback so Client previews use a single source.
                let (metadata, data) = match self
                    .attachments
                    .read_authorized(&session_id, &attachment_id)
                {
                    Ok(image) => (
                        serde_json::to_value(image.attachment)
                            .map_err(|_| "Invalid attachment metadata")?,
                        image.data,
                    ),
                    Err(_) => {
                        let file = self
                            .attachments
                            .read_authorized_file(&session_id, &attachment_id)
                            .map_err(|_| "Attachment unavailable")?;
                        (
                            serde_json::to_value(file.attachment)
                                .map_err(|_| "Invalid attachment metadata")?,
                            file.data,
                        )
                    }
                };
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(data)
                    .map_err(|_| "Invalid attachment data")?;
                if bytes.len() > request.maximum_bytes {
                    return Err("Attachment exceeds snapshot limit".into());
                }
                let mut descriptor = self.leases.insert("authenticated-loopback", bytes)?;
                descriptor["attachment"] = metadata;
                Ok(descriptor)
            }
        }
    }

    fn read(&self, lease: &str) -> Option<Arc<[u8]>> {
        self.leases.read("authenticated-loopback", lease)
    }

    fn release(&self, lease: &str) {
        self.leases.release("authenticated-loopback", lease);
    }
}

impl FileLeases {
    pub fn insert(&self, principal: &str, bytes: Vec<u8>) -> Result<serde_json::Value, String> {
        let mut leases = self.0.lock().map_err(|_| "Lease lock poisoned")?;
        let now = Instant::now();
        leases.retain(|_, value| value.expires > now);
        if leases.len() >= MAX_LEASES
            || bytes.len() > MAX_BYTES
            || leases
                .values()
                .map(|value| value.bytes.len())
                .sum::<usize>()
                > MAX_BYTES - bytes.len()
        {
            return Err("Preview lease capacity exceeded".into());
        }
        let mut random = [0u8; 32];
        let id = loop {
            std::fs::File::open("/dev/urandom")
                .and_then(|mut file| file.read_exact(&mut random))
                .map_err(|_| "Cannot generate preview lease")?;
            let id = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(random);
            if !leases.contains_key(&id) {
                break id;
            }
        };
        let descriptor = serde_json::json!({"version":1, "lease":id, "size":bytes.len(),
            "revision":format!("{:x}", Sha256::digest(&bytes))});
        leases.insert(
            id,
            Lease {
                principal: principal.to_owned(),
                bytes: bytes.into(),
                expires: now + TTL,
            },
        );
        Ok(descriptor)
    }

    pub fn read(&self, principal: &str, id: &str) -> Option<Arc<[u8]>> {
        let mut leases = self.0.lock().ok()?;
        leases.retain(|_, value| value.expires > Instant::now());
        leases
            .get(id)
            .filter(|value| value.principal == principal)
            .map(|value| value.bytes.clone())
    }

    pub fn release(&self, principal: &str, id: &str) {
        if let Ok(mut leases) = self.0.lock() {
            if leases
                .get(id)
                .is_some_and(|value| value.principal == principal)
            {
                leases.remove(id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use transport::FileTransferAuthority;

    #[test]
    fn attachment_transfer_preserves_metadata_and_requires_session_reference() {
        use serde_json::json;
        let root = tempfile::tempdir().unwrap();
        let session = "018f0000-0000-7000-8000-000000000061";
        let folder = root.path().join("threads").join(session);
        std::fs::create_dir_all(folder.join("assets")).unwrap();
        let genesis = json!({"config":{"digest":"cfg-1"},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,
            "origin_key":"create","origin_tuple":{"client":"client","key":"create","op":"create","principal":"principal","target":session},
            "resume":"never","seq":1,"thread":session,"ts":"2026-08-26T09:00:00.000Z","v":1,"workspace":"workspace"});
        let mut ledger = serde_json_canonicalizer::to_vec(&genesis).unwrap();
        ledger.push(b'\n');
        std::fs::write(folder.join("main.jsonl"), &ledger).unwrap();
        let data = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=";
        let authority = endpoint::AttachmentAuthority::open(root.path());
        let image = authority
            .materialize_prompt_parts(
                session,
                &[endpoint::PromptPart::Image {
                    media_type: endpoint::ImageMediaType::Png,
                    data: data.into(),
                    name: None,
                }],
            )
            .unwrap();
        let transfer = WorkspaceFileTransfer::open(root.path()).unwrap();
        let id = &image.attachments[0].attachment_id;
        let request = json!({"source":{"kind":"sessionAttachment","sessionId":session,"attachmentId":id},"maximumBytes":4096,"preferLocal":false});
        assert!(transfer.prepare(request.clone()).is_err());
        let input = json!({"content":image.blocks,"kind":"input","origin_key":"prompt",
            "origin_tuple":{"client":"client","key":"prompt","op":"submit","principal":"principal","target":session},
            "seq":2,"ts":"2026-08-26T09:00:01.000Z","v":1});
        ledger.extend(serde_json_canonicalizer::to_vec(&input).unwrap());
        ledger.push(b'\n');
        std::fs::write(folder.join("main.jsonl"), ledger).unwrap();
        let descriptor = transfer.prepare(request.clone()).unwrap();
        assert_eq!(descriptor["attachment"]["attachmentId"], *id);
        assert_eq!(descriptor["revision"], id.strip_prefix("sha256:").unwrap());
        assert_eq!(descriptor["attachment"]["width"], 1);
        let lease = descriptor["lease"].as_str().unwrap();
        assert_eq!(
            transfer.read(lease).unwrap().as_ref(),
            base64::engine::general_purpose::STANDARD
                .decode(data)
                .unwrap()
        );
        let mut small = request;
        small["maximumBytes"] = json!(1);
        assert!(transfer.prepare(small).is_err());
        transfer.release(lease);
        assert!(transfer.read(lease).is_none());
    }

    /// A digest that is not an image falls back to the file attachment read;
    /// the descriptor carries the file metadata instead of image dimensions.
    #[test]
    fn attachment_transfer_serves_uploaded_files_through_the_same_path() {
        use serde_json::json;
        let root = tempfile::tempdir().unwrap();
        let session = "018f0000-0000-7000-8000-000000000062";
        let folder = root.path().join("threads").join(session);
        std::fs::create_dir_all(folder.join("assets")).unwrap();
        let genesis = json!({"config":{"digest":"cfg-1"},"format":1,"kind":"genesis","min_reader":1,"min_writer":1,
            "origin_key":"create","origin_tuple":{"client":"client","key":"create","op":"create","principal":"principal","target":session},
            "resume":"never","seq":1,"thread":session,"ts":"2026-08-26T09:00:00.000Z","v":1,"workspace":"workspace"});
        let mut ledger = serde_json_canonicalizer::to_vec(&genesis).unwrap();
        ledger.push(b'\n');
        std::fs::write(folder.join("main.jsonl"), &ledger).unwrap();
        let transfer = WorkspaceFileTransfer::open(root.path()).unwrap();
        let authority = endpoint::AttachmentAuthority::open(root.path());
        let receipt = authority
            .upload_file(session, "notes.txt", Some("text/plain"), "aGVsbG8gZmlsZQ==")
            .unwrap();
        let id = &receipt.file.attachment_id;
        let request = json!({"source":{"kind":"sessionAttachment","sessionId":session,"attachmentId":id},"maximumBytes":4096,"preferLocal":false});
        let descriptor = transfer.prepare(request.clone()).unwrap();
        assert_eq!(descriptor["attachment"]["attachmentId"], *id);
        assert_eq!(descriptor["attachment"]["name"], "notes.txt");
        assert_eq!(descriptor["attachment"]["mediaType"], "text/plain");
        assert_eq!(descriptor["attachment"]["bytes"], 10);
        assert!(descriptor["attachment"].get("width").is_none());
        let lease = descriptor["lease"].as_str().unwrap();
        assert_eq!(transfer.read(lease).unwrap().as_ref(), b"hello file");
        let mut small = request;
        small["maximumBytes"] = json!(1);
        assert!(transfer.prepare(small).is_err());
        transfer.release(lease);
        assert!(transfer.read(lease).is_none());
    }

    #[test]
    fn lease_identity_owner_release_and_expiry() {
        let leases = FileLeases::default();
        let descriptor = leases.insert("owner", b"snapshot".to_vec()).unwrap();
        let id = descriptor["lease"].as_str().unwrap();
        assert_eq!(id.len(), 43);
        assert_eq!(descriptor["size"], 8);
        assert_eq!(leases.read("owner", id).unwrap().as_ref(), b"snapshot");
        assert!(leases.read("other", id).is_none());
        leases.release("other", id);
        assert!(leases.read("owner", id).is_some());
        leases.release("owner", id);
        assert!(leases.read("owner", id).is_none());
        let descriptor = leases.insert("owner", vec![]).unwrap();
        let id = descriptor["lease"].as_str().unwrap();
        leases.0.lock().unwrap().get_mut(id).unwrap().expires = Instant::now();
        assert!(leases.read("owner", id).is_none());
    }
}
