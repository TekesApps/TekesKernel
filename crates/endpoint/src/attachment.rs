//! Image prompt materialization, session-scoped file uploads, and attachment
//! reads.

use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use image::codecs::gif::GifDecoder;
use image::codecs::png::PngDecoder;
use image::codecs::webp::WebPDecoder;
use image::{AnimationDecoder, ImageDecoder, ImageFormat, ImageReader, Limits};
use schema::Block;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use store::{AssetStore, AtomicPublisher, DirectoryLock, StoreError, scan_valid_prefix};
use thiserror::Error;

use crate::{EndpointJournal, JournalError, validate_session_id};

pub const MAX_ATTACHMENT_BYTES: usize = 8_388_608;
pub const DEFAULT_FILE_MEDIA_TYPE: &str = "application/octet-stream";
pub const UPLOADS_DIR: &str = "uploads";
pub const MAX_IMAGE_DIMENSION: u32 = 16_384;
pub const MAX_IMAGE_PIXELS: u64 = 67_108_864;

const MAX_DECODE_ALLOCATION: u64 = MAX_IMAGE_PIXELS * 8 + 16 * 1024 * 1024;

/// Wire policy derives from the same constants enforced by materialization.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentPolicy {
    pub inline_media_types: [&'static str; 4],
    pub max_attachment_bytes: usize,
    pub max_image_pixels: u64,
    pub max_image_dimension: u32,
}

impl Default for AttachmentPolicy {
    fn default() -> Self {
        Self {
            inline_media_types: ["image/png", "image/jpeg", "image/gif", "image/webp"],
            max_attachment_bytes: MAX_ATTACHMENT_BYTES,
            max_image_pixels: MAX_IMAGE_PIXELS,
            max_image_dimension: MAX_IMAGE_DIMENSION,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ImageMediaType {
    #[serde(rename = "image/png")]
    Png,
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[serde(rename = "image/webp")]
    Webp,
    #[serde(rename = "image/gif")]
    Gif,
}

impl ImageMediaType {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Webp => "image/webp",
            Self::Gif => "image/gif",
        }
    }

    const fn image_format(self) -> ImageFormat {
        match self {
            Self::Png => ImageFormat::Png,
            Self::Jpeg => ImageFormat::Jpeg,
            Self::Webp => ImageFormat::WebP,
            Self::Gif => ImageFormat::Gif,
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "image/png" => Some(Self::Png),
            "image/jpeg" => Some(Self::Jpeg),
            "image/webp" => Some(Self::Webp),
            "image/gif" => Some(Self::Gif),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum PromptPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image", rename_all = "camelCase")]
    Image {
        media_type: ImageMediaType,
        data: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "file", rename_all = "camelCase")]
    File { receipt_id: String },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttachmentErrorReason {
    InvalidBase64,
    MediaTypeMismatch,
    TooLarge,
    InvalidDimensions,
    DecodeFailed,
    NotReferenced,
    Corrupt,
    QueueEditNonText,
    InlineMediaType,
    UnknownReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImageAttachmentRef {
    pub attachment_id: String,
    pub media_type: ImageMediaType,
    pub bytes: u64,
    pub width: u32,
    pub height: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageAttachment {
    pub attachment: ImageAttachmentRef,
    pub data: String,
}

/// Public reference to a non-image file attachment.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FileAttachmentRef {
    pub attachment_id: String,
    pub name: String,
    pub bytes: u64,
    pub media_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileAttachment {
    pub attachment: FileAttachmentRef,
    pub data: String,
}

/// `session.uploadFile` result: `{receiptId, file:{attachmentId,name,bytes}}`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UploadedFile {
    pub attachment_id: String,
    pub name: String,
    pub bytes: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UploadReceipt {
    pub receipt_id: String,
    pub file: UploadedFile,
}

/// Durable receipt at `threads/<sessionId>/uploads/<receiptId>.json`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct UploadReceiptRecord {
    format: u32,
    receipt_id: String,
    attachment_id: String,
    name: String,
    bytes: u64,
    media_type: String,
    created_at: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MaterializedPrompt {
    pub blocks: Vec<Block>,
    pub attachments: Vec<ImageAttachmentRef>,
    pub files: Vec<FileAttachmentRef>,
}

#[derive(Debug, Error)]
pub enum PromptMaterializeError {
    #[error("prompt content must not be empty")]
    EmptyContent,
    #[error("image name must be 1..255 UTF-8 bytes without control scalars")]
    InvalidName,
    #[error("file media type must be an RFC 2045 type/subtype token")]
    InvalidMediaType,
    #[error("image attachment validation failed: {0:?}")]
    Attachment(AttachmentErrorReason),
    #[error("session was not found: {0}")]
    SessionNotFound(String),
    #[error("session is archived: {0}")]
    Archived(String),
    #[error(transparent)]
    EndpointType(#[from] crate::types::EndpointTypeError),
    #[error(transparent)]
    Store(#[from] StoreError),
}

#[derive(Debug, Error)]
pub enum AttachmentReadError {
    #[error("session was not found: {0}")]
    SessionNotFound(String),
    #[error("session is archived: {0}")]
    Archived(String),
    #[error("image attachment is unavailable: {0:?}")]
    Attachment(AttachmentErrorReason),
    #[error("session ledger is corrupt: {0}")]
    Ledger(String),
    #[error(transparent)]
    EndpointType(#[from] crate::types::EndpointTypeError),
    #[error(transparent)]
    Journal(#[from] JournalError),
}

#[derive(Clone, Debug)]
pub struct AttachmentAuthority {
    storage_root: PathBuf,
}

impl AttachmentAuthority {
    #[must_use]
    pub fn open(storage_root: impl AsRef<Path>) -> Self {
        Self {
            storage_root: storage_root.as_ref().to_path_buf(),
        }
    }

    /// Validates every part before publishing any image asset. Publication is
    /// complete before the returned image blocks may be appended to the input
    /// barrier.
    pub fn materialize_prompt_parts(
        &self,
        session_id: &str,
        parts: &[PromptPart],
    ) -> Result<MaterializedPrompt, PromptMaterializeError> {
        validate_session_id(session_id)?;
        if parts.is_empty() {
            return Err(PromptMaterializeError::EmptyContent);
        }
        let mut validated = Vec::with_capacity(parts.len());
        for part in parts {
            validated.push(validate_part(part)?);
        }
        self.publish_validated(session_id, validated)
    }

    fn publish_validated(
        &self,
        session_id: &str,
        validated: Vec<ValidatedPart>,
    ) -> Result<MaterializedPrompt, PromptMaterializeError> {
        let folder = self.storage_root.join("threads").join(session_id);
        let lifecycle = match DirectoryLock::shared(&folder) {
            Ok(lock) => lock,
            Err(error) => {
                return match self.active_folder_for_prompt(session_id) {
                    Err(classified) => Err(classified),
                    Ok(_) => Err(PromptMaterializeError::Store(error)),
                };
            }
        };
        let rechecked = self.active_folder_for_prompt(session_id)?;
        if rechecked != folder || lifecycle.path() != folder {
            return Err(PromptMaterializeError::SessionNotFound(
                session_id.to_owned(),
            ));
        }
        let assets = AssetStore::new(folder.join("assets"))?;
        let mut blocks = Vec::with_capacity(validated.len());
        let mut attachments = Vec::new();
        let mut files = Vec::new();
        for part in validated {
            match part {
                ValidatedPart::Text(text) => blocks.push(Block::Text { text }),
                ValidatedPart::File(receipt_id) => {
                    let receipt = load_receipt(&folder, &receipt_id)?;
                    let digest = receipt
                        .attachment_id
                        .strip_prefix("sha256:")
                        .filter(|value| valid_lower_sha256(value))
                        .ok_or(PromptMaterializeError::Attachment(
                            AttachmentErrorReason::Corrupt,
                        ))?;
                    let asset = format!("sha256-{digest}");
                    let stored = assets.verify_named(&asset).map_err(|_| {
                        PromptMaterializeError::Attachment(AttachmentErrorReason::Corrupt)
                    })?;
                    if stored != receipt.bytes || receipt.bytes == 0 {
                        return Err(PromptMaterializeError::Attachment(
                            AttachmentErrorReason::Corrupt,
                        ));
                    }
                    files.push(FileAttachmentRef {
                        attachment_id: receipt.attachment_id,
                        name: receipt.name.clone(),
                        bytes: receipt.bytes,
                        media_type: receipt.media_type.clone(),
                    });
                    blocks.push(Block::File {
                        asset,
                        mime: receipt.media_type,
                        name: receipt.name,
                        bytes: receipt.bytes,
                    });
                }
                ValidatedPart::Image(image) => {
                    let published = assets.publish(&image.bytes)?;
                    attachments.push(ImageAttachmentRef {
                        attachment_id: attachment_id(&image.bytes),
                        media_type: image.media_type,
                        bytes: published.bytes,
                        width: image.width,
                        height: image.height,
                        name: image.name.clone(),
                    });
                    blocks.push(Block::Image {
                        asset: published.asset,
                        mime: image.media_type.as_str().to_owned(),
                        name: image.name,
                    });
                }
            }
        }
        Ok(MaterializedPrompt {
            blocks,
            attachments,
            files,
        })
    }

    /// `session.uploadFile`: validates, publishes the bytes into the session's
    /// content-addressed asset store, and writes a durable receipt under
    /// `threads/<sessionId>/uploads/<receiptId>.json`.
    pub fn upload_file(
        &self,
        session_id: &str,
        name: &str,
        media_type: Option<&str>,
        data_base64: &str,
    ) -> Result<UploadReceipt, PromptMaterializeError> {
        validate_session_id(session_id)?;
        if !valid_file_name(name) {
            return Err(PromptMaterializeError::InvalidName);
        }
        let media_type = match media_type {
            None => DEFAULT_FILE_MEDIA_TYPE.to_owned(),
            Some(value) => {
                normalize_media_type(value).ok_or(PromptMaterializeError::InvalidMediaType)?
            }
        };
        if ImageMediaType::parse(&media_type).is_some() {
            return Err(PromptMaterializeError::Attachment(
                AttachmentErrorReason::InlineMediaType,
            ));
        }
        let bytes =
            decode_padded_base64(data_base64).map_err(PromptMaterializeError::Attachment)?;
        if bytes.is_empty() {
            return Err(PromptMaterializeError::Attachment(
                AttachmentErrorReason::InvalidBase64,
            ));
        }

        let folder = self.storage_root.join("threads").join(session_id);
        let lifecycle = match DirectoryLock::shared(&folder) {
            Ok(lock) => lock,
            Err(error) => {
                return match self.active_folder_for_prompt(session_id) {
                    Err(classified) => Err(classified),
                    Ok(_) => Err(PromptMaterializeError::Store(error)),
                };
            }
        };
        let rechecked = self.active_folder_for_prompt(session_id)?;
        if rechecked != folder || lifecycle.path() != folder {
            return Err(PromptMaterializeError::SessionNotFound(
                session_id.to_owned(),
            ));
        }
        let assets = AssetStore::new(folder.join("assets"))?;
        let published = assets.publish(&bytes)?;
        let attachment_id = attachment_id(&bytes);
        let receipt_id = uuid::Uuid::new_v4().hyphenated().to_string();
        let record = UploadReceiptRecord {
            format: 1,
            receipt_id: receipt_id.clone(),
            attachment_id: attachment_id.clone(),
            name: name.to_owned(),
            bytes: published.bytes,
            media_type,
            created_at: rfc3339_millis_now(),
        };
        let mut line = serde_json_canonicalizer::to_vec(&record).map_err(|error| {
            StoreError::Corruption(format!("cannot canonicalize upload receipt: {error}"))
        })?;
        line.push(b'\n');
        AtomicPublisher::replace(
            folder.join(UPLOADS_DIR).join(format!("{receipt_id}.json")),
            &line,
        )?;
        Ok(UploadReceipt {
            receipt_id,
            file: UploadedFile {
                attachment_id,
                name: name.to_owned(),
                bytes: published.bytes,
            },
        })
    }

    /// Reads a file attachment only after this session's ledger, endpoint
    /// journal, or an upload receipt references the exact digest.
    pub fn read_authorized_file(
        &self,
        session_id: &str,
        attachment_id: &str,
    ) -> Result<FileAttachment, AttachmentReadError> {
        validate_session_id(session_id)?;
        let folder = self.active_folder_for_read(session_id)?;
        let digest = attachment_id
            .strip_prefix("sha256:")
            .filter(|value| valid_lower_sha256(value))
            .ok_or(AttachmentReadError::Attachment(
                AttachmentErrorReason::NotReferenced,
            ))?;
        let asset_name = format!("sha256-{digest}");

        let journal = EndpointJournal::open(&folder)?;
        let ledger = fs::read(folder.join("main.jsonl")).map_err(|error| {
            AttachmentReadError::Ledger(format!("cannot read main.jsonl: {error}"))
        })?;
        let scan = scan_valid_prefix(&ledger, 1);
        let projection = scan.projection.ok_or_else(|| {
            AttachmentReadError::Ledger("main.jsonl has no valid ledger prefix".to_owned())
        })?;

        let mut reference = None;
        for event in &projection.events {
            let value = serde_json::to_value(event.raw()).map_err(|error| {
                AttachmentReadError::Ledger(format!("cannot inspect event: {error}"))
            })?;
            collect_file_reference(&value, &asset_name, &mut reference);
        }
        if reference.is_none() {
            for record in journal.records()? {
                let value = serde_json::to_value(record.event.data).map_err(|error| {
                    AttachmentReadError::Ledger(format!("cannot inspect endpoint event: {error}"))
                })?;
                collect_file_reference(&value, &asset_name, &mut reference);
            }
        }
        if reference.is_none() {
            reference = receipt_reference(&folder, attachment_id)?;
        }
        let Some(reference) = reference else {
            return Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::NotReferenced,
            ));
        };

        let path = folder.join("assets").join(&asset_name);
        let metadata = fs::metadata(&path)
            .map_err(|_| AttachmentReadError::Attachment(AttachmentErrorReason::Corrupt))?;
        if !metadata.is_file()
            || metadata.len() == 0
            || metadata.len() > MAX_ATTACHMENT_BYTES as u64
            || metadata.len() != reference.bytes
        {
            return Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::Corrupt,
            ));
        }
        let bytes = AssetStore::new(folder.join("assets"))
            .map_err(|_| AttachmentReadError::Attachment(AttachmentErrorReason::Corrupt))?
            .read_verified(&asset_name)
            .map_err(|_| AttachmentReadError::Attachment(AttachmentErrorReason::Corrupt))?;
        if bytes.len() as u64 != reference.bytes {
            return Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::Corrupt,
            ));
        }
        Ok(FileAttachment {
            attachment: FileAttachmentRef {
                attachment_id: attachment_id.to_owned(),
                name: reference.name,
                bytes: reference.bytes,
                media_type: reference.media_type,
            },
            data: BASE64.encode(bytes),
        })
    }

    /// Reads an image only after a structured reference is found in this
    /// session's valid semantic-ledger prefix or validated endpoint journal.
    pub fn read_authorized(
        &self,
        session_id: &str,
        attachment_id: &str,
    ) -> Result<ImageAttachment, AttachmentReadError> {
        validate_session_id(session_id)?;
        let folder = self.active_folder_for_read(session_id)?;
        let digest = attachment_id
            .strip_prefix("sha256:")
            .filter(|value| valid_lower_sha256(value))
            .ok_or(AttachmentReadError::Attachment(
                AttachmentErrorReason::NotReferenced,
            ))?;
        let asset_name = format!("sha256-{digest}");

        // EndpointJournal owns a shared folder-lifecycle lock for the whole
        // authorization and read, preventing archive from moving the folder.
        let journal = EndpointJournal::open(&folder)?;
        let ledger = fs::read(folder.join("main.jsonl")).map_err(|error| {
            AttachmentReadError::Ledger(format!("cannot read main.jsonl: {error}"))
        })?;
        let scan = scan_valid_prefix(&ledger, 1);
        let projection = scan.projection.ok_or_else(|| {
            AttachmentReadError::Ledger("main.jsonl has no valid ledger prefix".to_owned())
        })?;

        let mut expected = ReferenceExpectation::default();
        for event in &projection.events {
            let value = serde_json::to_value(event.raw()).map_err(|error| {
                AttachmentReadError::Ledger(format!("cannot inspect event: {error}"))
            })?;
            collect_references(&value, attachment_id, &asset_name, &mut expected)?;
        }
        for record in journal.records()? {
            let value = serde_json::to_value(record.event.data).map_err(|error| {
                AttachmentReadError::Ledger(format!("cannot inspect endpoint event: {error}"))
            })?;
            collect_references(&value, attachment_id, &asset_name, &mut expected)?;
        }
        if !expected.referenced {
            return Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::NotReferenced,
            ));
        }

        let path = folder.join("assets").join(&asset_name);
        let metadata = fs::metadata(&path)
            .map_err(|_| AttachmentReadError::Attachment(AttachmentErrorReason::Corrupt))?;
        if !metadata.is_file()
            || metadata.len() == 0
            || metadata.len() > MAX_ATTACHMENT_BYTES as u64
        {
            return Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::Corrupt,
            ));
        }
        let bytes = AssetStore::new(folder.join("assets"))
            .map_err(|_| AttachmentReadError::Attachment(AttachmentErrorReason::Corrupt))?
            .read_verified(&asset_name)
            .map_err(|_| AttachmentReadError::Attachment(AttachmentErrorReason::Corrupt))?;
        if expected
            .bytes
            .is_some_and(|value| value != bytes.len() as u64)
        {
            return Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::Corrupt,
            ));
        }
        let media_type = expected.media_type.ok_or(AttachmentReadError::Attachment(
            AttachmentErrorReason::Corrupt,
        ))?;
        if !signature_matches(&bytes, media_type) {
            return Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::Corrupt,
            ));
        }
        let (width, height) = decode_dimensions(&bytes, media_type)
            .map_err(|_| AttachmentReadError::Attachment(AttachmentErrorReason::Corrupt))?;
        if expected.width.is_some_and(|value| value != width)
            || expected.height.is_some_and(|value| value != height)
        {
            return Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::Corrupt,
            ));
        }

        Ok(ImageAttachment {
            attachment: ImageAttachmentRef {
                attachment_id: attachment_id.to_owned(),
                media_type,
                bytes: bytes.len() as u64,
                width,
                height,
                name: None,
            },
            data: BASE64.encode(bytes),
        })
    }

    fn active_folder_for_prompt(
        &self,
        session_id: &str,
    ) -> Result<PathBuf, PromptMaterializeError> {
        if self.storage_root.join("archive").join(session_id).is_dir() {
            return Err(PromptMaterializeError::Archived(session_id.to_owned()));
        }
        let folder = self.storage_root.join("threads").join(session_id);
        if !folder.is_dir() {
            return Err(PromptMaterializeError::SessionNotFound(
                session_id.to_owned(),
            ));
        }
        Ok(folder)
    }

    fn active_folder_for_read(&self, session_id: &str) -> Result<PathBuf, AttachmentReadError> {
        if self.storage_root.join("archive").join(session_id).is_dir() {
            return Err(AttachmentReadError::Archived(session_id.to_owned()));
        }
        let folder = self.storage_root.join("threads").join(session_id);
        if !folder.is_dir() {
            return Err(AttachmentReadError::SessionNotFound(session_id.to_owned()));
        }
        Ok(folder)
    }
}

enum ValidatedPart {
    Text(String),
    Image(ValidatedImage),
    File(String),
}

fn valid_receipt_id(value: &str) -> bool {
    uuid::Uuid::parse_str(value).is_ok_and(|uuid| uuid.hyphenated().to_string() == value)
        && value == value.to_ascii_lowercase()
}

fn valid_file_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 255
        && !value.chars().any(char::is_control)
        && !value.contains('/')
}

/// RFC 2045 `type "/" subtype` where each side is a token; lowercased.
fn normalize_media_type(value: &str) -> Option<String> {
    let (kind, subtype) = value.split_once('/')?;
    let is_token = |part: &str| {
        !part.is_empty()
            && part.len() <= 127
            && part.bytes().all(|byte| {
                byte > 0x20
                    && byte < 0x7f
                    && !matches!(
                        byte,
                        b'(' | b')'
                            | b'<'
                            | b'>'
                            | b'@'
                            | b','
                            | b';'
                            | b':'
                            | b'\\'
                            | b'"'
                            | b'/'
                            | b'['
                            | b']'
                            | b'?'
                            | b'='
                    )
            })
    };
    (is_token(kind) && is_token(subtype)).then(|| value.to_ascii_lowercase())
}

fn load_receipt(
    folder: &Path,
    receipt_id: &str,
) -> Result<UploadReceiptRecord, PromptMaterializeError> {
    let unknown = || PromptMaterializeError::Attachment(AttachmentErrorReason::UnknownReceipt);
    let path = folder.join(UPLOADS_DIR).join(format!("{receipt_id}.json"));
    let bytes = fs::read(&path).map_err(|_| unknown())?;
    let record: UploadReceiptRecord = serde_json::from_slice(&bytes)
        .map_err(|_| PromptMaterializeError::Attachment(AttachmentErrorReason::Corrupt))?;
    if record.format != 1 || record.receipt_id != receipt_id {
        return Err(PromptMaterializeError::Attachment(
            AttachmentErrorReason::Corrupt,
        ));
    }
    Ok(record)
}

struct FileReference {
    name: String,
    media_type: String,
    bytes: u64,
}

fn collect_file_reference(value: &Value, asset_name: &str, found: &mut Option<FileReference>) {
    if found.is_some() {
        return;
    }
    match value {
        Value::Array(values) => {
            for value in values {
                collect_file_reference(value, asset_name, found);
            }
        }
        Value::Object(object) => {
            if object.get("type").and_then(Value::as_str) == Some("file")
                && object.get("asset").and_then(Value::as_str) == Some(asset_name)
            {
                let name = object.get("name").and_then(Value::as_str);
                let media_type = object.get("mime").and_then(Value::as_str);
                let bytes = object.get("bytes").and_then(Value::as_u64);
                if let (Some(name), Some(media_type), Some(bytes)) = (name, media_type, bytes) {
                    *found = Some(FileReference {
                        name: name.to_owned(),
                        media_type: media_type.to_owned(),
                        bytes,
                    });
                    return;
                }
            }
            for value in object.values() {
                collect_file_reference(value, asset_name, found);
            }
        }
        _ => {}
    }
}

/// A receipt in `uploads/` naming the attachment id authorizes a read of a
/// freshly uploaded, not-yet-prompted file.
fn receipt_reference(
    folder: &Path,
    attachment_id: &str,
) -> Result<Option<FileReference>, AttachmentReadError> {
    let directory = match fs::read_dir(folder.join(UPLOADS_DIR)) {
        Ok(directory) => directory,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(AttachmentReadError::Ledger(format!(
                "cannot read uploads: {error}"
            )));
        }
    };
    let mut entries: Vec<PathBuf> = directory
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    entries.sort();
    for path in entries {
        let Ok(bytes) = fs::read(&path) else { continue };
        let Ok(record) = serde_json::from_slice::<UploadReceiptRecord>(&bytes) else {
            continue;
        };
        if record.format == 1 && record.attachment_id == attachment_id {
            return Ok(Some(FileReference {
                name: record.name,
                media_type: record.media_type,
                bytes: record.bytes,
            }));
        }
    }
    Ok(None)
}

fn rfc3339_millis_now() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    rfc3339_from_millis(now.as_millis() as u64)
}

fn rfc3339_from_millis(value: u64) -> String {
    let seconds = value / 1_000;
    let millis = value % 1_000;
    let days = (seconds / 86_400) as i64;
    let day_seconds = seconds % 86_400;
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let doe = days - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{millis:03}Z",
        day_seconds / 3_600,
        (day_seconds % 3_600) / 60,
        day_seconds % 60
    )
}

struct ValidatedImage {
    media_type: ImageMediaType,
    bytes: Vec<u8>,
    width: u32,
    height: u32,
    name: Option<String>,
}

fn validate_part(part: &PromptPart) -> Result<ValidatedPart, PromptMaterializeError> {
    match part {
        PromptPart::Text { text } => Ok(ValidatedPart::Text(text.clone())),
        PromptPart::File { receipt_id } => {
            if !valid_receipt_id(receipt_id) {
                return Err(PromptMaterializeError::Attachment(
                    AttachmentErrorReason::UnknownReceipt,
                ));
            }
            Ok(ValidatedPart::File(receipt_id.clone()))
        }
        PromptPart::Image {
            media_type,
            data,
            name,
        } => {
            if name.as_ref().is_some_and(|value| {
                value.is_empty() || value.len() > 255 || value.chars().any(char::is_control)
            }) {
                return Err(PromptMaterializeError::InvalidName);
            }
            let bytes = decode_padded_base64(data).map_err(PromptMaterializeError::Attachment)?;
            if !signature_matches(&bytes, *media_type) {
                return Err(PromptMaterializeError::Attachment(
                    AttachmentErrorReason::MediaTypeMismatch,
                ));
            }
            let (width, height) = decode_dimensions(&bytes, *media_type)
                .map_err(PromptMaterializeError::Attachment)?;
            Ok(ValidatedPart::Image(ValidatedImage {
                media_type: *media_type,
                bytes,
                width,
                height,
                name: name.clone(),
            }))
        }
    }
}

fn decode_padded_base64(value: &str) -> Result<Vec<u8>, AttachmentErrorReason> {
    let bytes = value.as_bytes();
    if bytes.is_empty() || bytes.len() % 4 != 0 {
        return Err(AttachmentErrorReason::InvalidBase64);
    }
    let padding = if bytes.ends_with(b"==") {
        2
    } else if bytes.ends_with(b"=") {
        1
    } else {
        0
    };
    let body_len = bytes.len() - padding;
    if body_len == 0
        || bytes[..body_len]
            .iter()
            .any(|byte| base64_value(*byte).is_none())
        || bytes[body_len..].iter().any(|byte| *byte != b'=')
        || bytes[..body_len].contains(&b'=')
    {
        return Err(AttachmentErrorReason::InvalidBase64);
    }
    if (padding == 2 && base64_value(bytes[body_len - 1]).is_none_or(|value| value & 0x0f != 0))
        || (padding == 1 && base64_value(bytes[body_len - 1]).is_none_or(|value| value & 0x03 != 0))
    {
        return Err(AttachmentErrorReason::InvalidBase64);
    }
    let decoded_len = bytes
        .len()
        .checked_div(4)
        .and_then(|groups| groups.checked_mul(3))
        .and_then(|length| length.checked_sub(padding))
        .ok_or(AttachmentErrorReason::TooLarge)?;
    if decoded_len > MAX_ATTACHMENT_BYTES {
        return Err(AttachmentErrorReason::TooLarge);
    }
    let decoded = BASE64
        .decode(value)
        .map_err(|_| AttachmentErrorReason::InvalidBase64)?;
    if BASE64.encode(&decoded) != value {
        return Err(AttachmentErrorReason::InvalidBase64);
    }
    Ok(decoded)
}

fn base64_value(value: u8) -> Option<u8> {
    match value {
        b'A'..=b'Z' => Some(value - b'A'),
        b'a'..=b'z' => Some(value - b'a' + 26),
        b'0'..=b'9' => Some(value - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn signature_matches(bytes: &[u8], media_type: ImageMediaType) -> bool {
    match media_type {
        ImageMediaType::Png => bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
        ImageMediaType::Jpeg => bytes.starts_with(&[0xff, 0xd8, 0xff]),
        ImageMediaType::Gif => bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a"),
        ImageMediaType::Webp => {
            bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP"
        }
    }
}

fn decode_dimensions(
    bytes: &[u8],
    media_type: ImageMediaType,
) -> Result<(u32, u32), AttachmentErrorReason> {
    let dimensions = ImageReader::with_format(Cursor::new(bytes), media_type.image_format())
        .into_dimensions()
        .map_err(|_| AttachmentErrorReason::DecodeFailed)?;
    let pixels = u64::from(dimensions.0) * u64::from(dimensions.1);
    if dimensions.0 == 0
        || dimensions.1 == 0
        || dimensions.0 > MAX_IMAGE_DIMENSION
        || dimensions.1 > MAX_IMAGE_DIMENSION
        || pixels > MAX_IMAGE_PIXELS
    {
        return Err(AttachmentErrorReason::InvalidDimensions);
    }

    decode_all_frames(bytes, media_type, dimensions)?;
    Ok(dimensions)
}

fn decode_all_frames(
    bytes: &[u8],
    media_type: ImageMediaType,
    canvas: (u32, u32),
) -> Result<(), AttachmentErrorReason> {
    match media_type {
        ImageMediaType::Gif => {
            let mut decoder = GifDecoder::new(Cursor::new(bytes))
                .map_err(|_| AttachmentErrorReason::DecodeFailed)?;
            decoder
                .set_limits(decode_limits())
                .map_err(|_| AttachmentErrorReason::DecodeFailed)?;
            decode_animation(decoder, canvas)
        }
        ImageMediaType::Webp => {
            let decoder = WebPDecoder::new(Cursor::new(bytes))
                .map_err(|_| AttachmentErrorReason::DecodeFailed)?;
            if decoder.has_animation() {
                decode_animation(decoder, canvas)
            } else {
                decode_static(bytes, media_type, canvas)
            }
        }
        ImageMediaType::Png => {
            let decoder = PngDecoder::with_limits(Cursor::new(bytes), decode_limits())
                .map_err(|_| AttachmentErrorReason::DecodeFailed)?;
            let animated = decoder
                .is_apng()
                .map_err(|_| AttachmentErrorReason::DecodeFailed)?;
            if animated {
                let decoder = decoder
                    .apng()
                    .map_err(|_| AttachmentErrorReason::DecodeFailed)?;
                decode_animation(decoder, canvas)
            } else {
                decode_static(bytes, media_type, canvas)
            }
        }
        ImageMediaType::Jpeg => decode_static(bytes, media_type, canvas),
    }
}

fn decode_static(
    bytes: &[u8],
    media_type: ImageMediaType,
    canvas: (u32, u32),
) -> Result<(), AttachmentErrorReason> {
    let mut reader = ImageReader::with_format(Cursor::new(bytes), media_type.image_format());
    reader.limits(decode_limits());
    let decoded = reader
        .decode()
        .map_err(|_| AttachmentErrorReason::DecodeFailed)?;
    if decoded.width() != canvas.0 || decoded.height() != canvas.1 {
        return Err(AttachmentErrorReason::DecodeFailed);
    }
    Ok(())
}

fn decode_animation<'a>(
    decoder: impl AnimationDecoder<'a>,
    canvas: (u32, u32),
) -> Result<(), AttachmentErrorReason> {
    let mut total_pixels = 0_u64;
    let mut frames = 0_u64;
    for frame in decoder.into_frames() {
        let frame = frame.map_err(|_| AttachmentErrorReason::DecodeFailed)?;
        let buffer = frame.buffer();
        record_frame_pixels(canvas, (buffer.width(), buffer.height()), &mut total_pixels)?;
        frames += 1;
    }
    if frames == 0 {
        return Err(AttachmentErrorReason::DecodeFailed);
    }
    Ok(())
}

fn record_frame_pixels(
    canvas: (u32, u32),
    frame: (u32, u32),
    total: &mut u64,
) -> Result<(), AttachmentErrorReason> {
    if frame.0 == 0 || frame.1 == 0 || frame.0 > canvas.0 || frame.1 > canvas.1 {
        return Err(AttachmentErrorReason::InvalidDimensions);
    }
    *total = total
        .checked_add(u64::from(frame.0) * u64::from(frame.1))
        .ok_or(AttachmentErrorReason::InvalidDimensions)?;
    if *total > MAX_IMAGE_PIXELS {
        return Err(AttachmentErrorReason::InvalidDimensions);
    }
    Ok(())
}

fn decode_limits() -> Limits {
    let mut limits = Limits::default();
    limits.max_image_width = Some(MAX_IMAGE_DIMENSION);
    limits.max_image_height = Some(MAX_IMAGE_DIMENSION);
    limits.max_alloc = Some(MAX_DECODE_ALLOCATION);
    limits
}

#[derive(Default)]
struct ReferenceExpectation {
    referenced: bool,
    media_type: Option<ImageMediaType>,
    bytes: Option<u64>,
    width: Option<u32>,
    height: Option<u32>,
}

fn collect_references(
    value: &Value,
    attachment_id: &str,
    asset_name: &str,
    expected: &mut ReferenceExpectation,
) -> Result<(), AttachmentReadError> {
    match value {
        Value::Array(values) => {
            for value in values {
                collect_references(value, attachment_id, asset_name, expected)?;
            }
        }
        Value::Object(object) => {
            if object.get("type").and_then(Value::as_str) == Some("image")
                && object.get("asset").and_then(Value::as_str) == Some(asset_name)
            {
                let media_type = object
                    .get("mime")
                    .and_then(Value::as_str)
                    .and_then(ImageMediaType::parse)
                    .ok_or_else(corrupt_read)?;
                merge_media(expected, media_type)?;
                expected.referenced = true;
            }
            if object.get("attachmentId").and_then(Value::as_str) == Some(attachment_id) {
                let media_type = object
                    .get("mediaType")
                    .and_then(Value::as_str)
                    .and_then(ImageMediaType::parse)
                    .ok_or_else(corrupt_read)?;
                let bytes = object
                    .get("bytes")
                    .and_then(Value::as_u64)
                    .filter(|value| *value > 0)
                    .ok_or_else(corrupt_read)?;
                let width = object
                    .get("width")
                    .and_then(Value::as_u64)
                    .and_then(|value| u32::try_from(value).ok())
                    .filter(|value| *value > 0)
                    .ok_or_else(corrupt_read)?;
                let height = object
                    .get("height")
                    .and_then(Value::as_u64)
                    .and_then(|value| u32::try_from(value).ok())
                    .filter(|value| *value > 0)
                    .ok_or_else(corrupt_read)?;
                merge_media(expected, media_type)?;
                merge_value(&mut expected.bytes, bytes)?;
                merge_value(&mut expected.width, width)?;
                merge_value(&mut expected.height, height)?;
                expected.referenced = true;
            }
            for value in object.values() {
                collect_references(value, attachment_id, asset_name, expected)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn merge_media(
    expected: &mut ReferenceExpectation,
    value: ImageMediaType,
) -> Result<(), AttachmentReadError> {
    merge_value(&mut expected.media_type, value)
}

fn merge_value<T: Copy + Eq>(slot: &mut Option<T>, value: T) -> Result<(), AttachmentReadError> {
    if slot.is_some_and(|existing| existing != value) {
        return Err(corrupt_read());
    }
    *slot = Some(value);
    Ok(())
}

fn corrupt_read() -> AttachmentReadError {
    AttachmentReadError::Attachment(AttachmentErrorReason::Corrupt)
}

fn valid_lower_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

/// Rewrites durable `file` blocks into their public projection
/// `{type:"file", attachment:{attachmentId,name,bytes}, mediaType}`; every
/// other block (including images) is passed through verbatim.
#[must_use]
pub fn project_content_blocks(content: &Value) -> Value {
    match content {
        Value::Array(values) => Value::Array(values.iter().map(project_content_block).collect()),
        other => other.clone(),
    }
}

fn project_content_block(block: &Value) -> Value {
    let Some(object) = block.as_object() else {
        return block.clone();
    };
    if object.get("type").and_then(Value::as_str) != Some("file") {
        return block.clone();
    }
    let asset = object.get("asset").and_then(Value::as_str);
    let digest = asset.and_then(|value| value.strip_prefix("sha256-"));
    let (Some(digest), Some(name), Some(bytes), Some(mime)) = (
        digest,
        object.get("name").and_then(Value::as_str),
        object.get("bytes").and_then(Value::as_u64),
        object.get("mime").and_then(Value::as_str),
    ) else {
        return block.clone();
    };
    serde_json::json!({
        "type": "file",
        "attachment": {
            "attachmentId": format!("sha256:{digest}"),
            "name": name,
            "bytes": bytes,
        },
        "mediaType": mime,
    })
}

#[must_use]
fn attachment_id(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use image::codecs::gif::GifEncoder;
    use image::{DynamicImage, Frame, GrayImage, RgbaImage};
    use schema::IJsonValue;
    use serde_json::json;

    use super::*;
    use crate::{SessionEvent, SurfaceOperation};

    const SESSION: &str = "018f0000-0000-7000-8000-000000000061";
    const OTHER_SESSION: &str = "018f0000-0000-7000-8000-000000000062";

    #[test]
    fn four_declared_formats_are_fully_decoded_and_materialized() {
        let root = tempfile::tempdir().expect("root");
        create_session(root.path(), SESSION, &[]);
        let authority = AttachmentAuthority::open(root.path());
        let formats = [
            (ImageMediaType::Png, ImageFormat::Png),
            (ImageMediaType::Jpeg, ImageFormat::Jpeg),
            (ImageMediaType::Webp, ImageFormat::WebP),
            (ImageMediaType::Gif, ImageFormat::Gif),
        ];
        for (media_type, format) in formats {
            let bytes = encoded_image(1, 1, format);
            let result = authority
                .materialize_prompt_parts(
                    SESSION,
                    &[PromptPart::Image {
                        media_type,
                        data: BASE64.encode(&bytes),
                        name: Some("pixel".to_owned()),
                    }],
                )
                .expect("materialized image");
            let Block::Image { asset, mime, name } = &result.blocks[0] else {
                panic!("expected image block");
            };
            assert_eq!(asset, &format!("sha256-{}", &attachment_id(&bytes)[7..]));
            assert_eq!(mime, media_type.as_str());
            assert_eq!(name.as_deref(), Some("pixel"));
            assert_eq!(
                result.attachments,
                vec![ImageAttachmentRef {
                    attachment_id: attachment_id(&bytes),
                    media_type,
                    bytes: bytes.len() as u64,
                    width: 1,
                    height: 1,
                    name: Some("pixel".to_owned()),
                }]
            );
        }
    }

    #[test]
    fn animated_gif_decodes_every_frame_and_rejects_a_broken_later_frame() {
        let complete = animated_gif(3, 4, 4);
        assert_eq!(
            decode_dimensions(&complete, ImageMediaType::Gif),
            Ok((4, 4))
        );

        let mut broken = complete;
        broken.truncate(broken.len().saturating_sub(8));
        assert_eq!(
            decode_dimensions(&broken, ImageMediaType::Gif),
            Err(AttachmentErrorReason::DecodeFailed)
        );
    }

    #[test]
    fn cumulative_animation_pixels_are_bounded() {
        let mut total = 0;
        for _ in 0..4 {
            record_frame_pixels((4096, 4096), (4096, 4096), &mut total)
                .expect("within cumulative limit");
        }
        assert_eq!(total, MAX_IMAGE_PIXELS);
        assert_eq!(
            record_frame_pixels((4096, 4096), (1, 1), &mut total),
            Err(AttachmentErrorReason::InvalidDimensions)
        );
        assert_eq!(
            record_frame_pixels((10, 10), (11, 1), &mut 0),
            Err(AttachmentErrorReason::InvalidDimensions)
        );
    }

    #[test]
    fn prompt_validation_is_ordered_and_publishes_nothing_on_failure() {
        let root = tempfile::tempdir().expect("root");
        create_session(root.path(), SESSION, &[]);
        let authority = AttachmentAuthority::open(root.path());
        let png = encoded_image(1, 1, ImageFormat::Png);
        let error = authority
            .materialize_prompt_parts(
                SESSION,
                &[
                    PromptPart::Image {
                        media_type: ImageMediaType::Png,
                        data: BASE64.encode(png),
                        name: None,
                    },
                    PromptPart::Image {
                        media_type: ImageMediaType::Png,
                        data: "not-base64".to_owned(),
                        name: None,
                    },
                ],
            )
            .expect_err("invalid base64");
        assert!(matches!(
            error,
            PromptMaterializeError::Attachment(AttachmentErrorReason::InvalidBase64)
        ));
        assert_eq!(
            fs::read_dir(root.path().join("threads").join(SESSION).join("assets"))
                .expect("assets")
                .count(),
            0
        );
    }

    #[test]
    fn size_signature_decode_and_dimension_failures_are_distinct() {
        let oversized = BASE64.encode(vec![0_u8; MAX_ATTACHMENT_BYTES + 1]);
        assert_eq!(
            decode_padded_base64(&oversized),
            Err(AttachmentErrorReason::TooLarge)
        );
        let png = encoded_image(1, 1, ImageFormat::Png);
        assert!(!signature_matches(&png, ImageMediaType::Jpeg));
        assert_eq!(
            decode_dimensions(b"\x89PNG\r\n\x1a\nbroken", ImageMediaType::Png),
            Err(AttachmentErrorReason::DecodeFailed)
        );
        let too_wide = encoded_image(MAX_IMAGE_DIMENSION + 1, 1, ImageFormat::Png);
        assert_eq!(
            decode_dimensions(&too_wide, ImageMediaType::Png),
            Err(AttachmentErrorReason::InvalidDimensions)
        );
    }

    #[test]
    fn read_requires_a_reference_in_the_selected_session_and_revalidates_asset() {
        let root = tempfile::tempdir().expect("root");
        create_session(root.path(), SESSION, &[]);
        create_session(root.path(), OTHER_SESSION, &[]);
        let authority = AttachmentAuthority::open(root.path());
        let bytes = encoded_image(2, 3, ImageFormat::Png);
        let materialized = authority
            .materialize_prompt_parts(
                SESSION,
                &[PromptPart::Image {
                    media_type: ImageMediaType::Png,
                    data: BASE64.encode(&bytes),
                    name: Some("same bytes, local name".to_owned()),
                }],
            )
            .expect("materialized");
        append_input(root.path(), SESSION, &materialized.blocks);
        let id = attachment_id(&bytes);
        let read = authority
            .read_authorized(SESSION, &id)
            .expect("authorized read");
        assert_eq!(read.attachment.attachment_id, id);
        assert_eq!(read.attachment.media_type, ImageMediaType::Png);
        assert_eq!((read.attachment.width, read.attachment.height), (2, 3));
        assert_eq!(read.attachment.name, None);
        assert_eq!(BASE64.decode(read.data).expect("base64"), bytes);

        let cross_session = authority
            .read_authorized(OTHER_SESSION, &id)
            .expect_err("cross-session digest guess");
        assert!(matches!(
            cross_session,
            AttachmentReadError::Attachment(AttachmentErrorReason::NotReferenced)
        ));
    }

    #[test]
    fn endpoint_journal_reference_authorizes_the_selected_session() {
        let root = tempfile::tempdir().expect("root");
        create_session(root.path(), SESSION, &[]);
        let folder = root.path().join("threads").join(SESSION);
        let bytes = encoded_image(3, 2, ImageFormat::Png);
        let published = AssetStore::new(folder.join("assets"))
            .expect("assets")
            .publish(&bytes)
            .expect("published");
        let id = attachment_id(&bytes);
        EndpointJournal::open(&folder)
            .expect("journal")
            .append_kernel(
                vec![1],
                "attachment",
                SessionEvent {
                    event_type: "user/message".to_owned(),
                    seq: 0,
                    time: 1.0,
                    data: IJsonValue::parse(
                        &serde_json::to_vec(&json!({
                            "content":[{
                                "type":"image",
                                "attachmentId":id,
                                "mediaType":"image/png",
                                "bytes":published.bytes,
                                "width":3,
                                "height":2
                            }]
                        }))
                        .expect("json"),
                    )
                    .expect("I-JSON"),
                    ignorable: None,
                    source_event_seqs: None,
                    surface_op: Some(SurfaceOperation::Append("append".to_owned())),
                },
            )
            .expect("journal reference");

        let attachment = AttachmentAuthority::open(root.path())
            .read_authorized(SESSION, &id)
            .expect("journal-authorized read");
        assert_eq!(attachment.attachment.bytes, bytes.len() as u64);
        assert_eq!(
            (attachment.attachment.width, attachment.attachment.height),
            (3, 2)
        );
    }

    #[test]
    fn corrupt_asset_is_not_returned_even_when_referenced() {
        let root = tempfile::tempdir().expect("root");
        create_session(root.path(), SESSION, &[]);
        let authority = AttachmentAuthority::open(root.path());
        let bytes = encoded_image(1, 1, ImageFormat::Png);
        let materialized = authority
            .materialize_prompt_parts(
                SESSION,
                &[PromptPart::Image {
                    media_type: ImageMediaType::Png,
                    data: BASE64.encode(&bytes),
                    name: None,
                }],
            )
            .expect("materialized");
        append_input(root.path(), SESSION, &materialized.blocks);
        let id = attachment_id(&bytes);
        fs::write(
            root.path()
                .join("threads")
                .join(SESSION)
                .join("assets")
                .join(id.replacen("sha256:", "sha256-", 1)),
            b"tampered",
        )
        .expect("tamper");
        assert!(matches!(
            authority.read_authorized(SESSION, &id),
            Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::Corrupt
            ))
        ));
    }

    #[test]
    fn archive_between_validation_and_publication_cannot_recreate_active_folder() {
        let root = tempfile::tempdir().expect("root");
        create_session(root.path(), SESSION, &[]);
        let bytes = encoded_image(1, 1, ImageFormat::Png);
        let validated = vec![
            validate_part(&PromptPart::Image {
                media_type: ImageMediaType::Png,
                data: BASE64.encode(bytes),
                name: None,
            })
            .expect("validated in memory"),
        ];
        fs::create_dir_all(root.path().join("archive")).expect("archive root");
        fs::rename(
            root.path().join("threads").join(SESSION),
            root.path().join("archive").join(SESSION),
        )
        .expect("archive wins race");

        let authority = AttachmentAuthority::open(root.path());
        assert!(matches!(
            authority.publish_validated(SESSION, validated),
            Err(PromptMaterializeError::Archived(value)) if value == SESSION
        ));
        assert!(matches!(
            authority.materialize_prompt_parts(
                SESSION,
                &[PromptPart::Text {
                    text: "valid".to_owned()
                }]
            ),
            Err(PromptMaterializeError::Archived(value)) if value == SESSION
        ));
        assert!(!root.path().join("threads").join(SESSION).exists());
        assert_eq!(
            fs::read_dir(root.path().join("archive").join(SESSION).join("assets"))
                .expect("archived assets")
                .count(),
            0
        );
    }

    // ---- file uploads -------------------------------------------------------

    const FILE_BYTES: &[u8] = b"hello, file attachment\n";

    fn upload(root: &Path, session: &str, name: &str, media: Option<&str>) -> UploadReceipt {
        AttachmentAuthority::open(root)
            .upload_file(session, name, media, &BASE64.encode(FILE_BYTES))
            .expect("upload")
    }

    fn file_reason(error: &PromptMaterializeError) -> Option<AttachmentErrorReason> {
        match error {
            PromptMaterializeError::Attachment(reason) => Some(*reason),
            _ => None,
        }
    }

    #[test]
    fn upload_file_publishes_asset_and_durable_receipt() {
        let root = tempfile::tempdir().expect("root");
        create_session(root.path(), SESSION, &[]);
        let before = rfc3339_millis_now();
        let receipt = upload(root.path(), SESSION, "notes.md", Some("Text/Markdown"));
        assert!(valid_receipt_id(&receipt.receipt_id));
        assert_eq!(receipt.file.attachment_id, attachment_id(FILE_BYTES));
        assert_eq!(receipt.file.name, "notes.md");
        assert_eq!(receipt.file.bytes, FILE_BYTES.len() as u64);
        assert_eq!(
            serde_json::to_value(&receipt).expect("json"),
            json!({"receiptId":receipt.receipt_id,"file":{"attachmentId":receipt.file.attachment_id,"name":"notes.md","bytes":FILE_BYTES.len()}})
        );

        let folder = root.path().join("threads").join(SESSION);
        let digest = receipt.file.attachment_id.strip_prefix("sha256:").unwrap();
        assert_eq!(
            fs::read(folder.join("assets").join(format!("sha256-{digest}"))).expect("asset"),
            FILE_BYTES
        );
        let path = folder
            .join("uploads")
            .join(format!("{}.json", receipt.receipt_id));
        let raw = fs::read(&path).expect("receipt file");
        assert!(raw.ends_with(b"\n"));
        let value: Value = serde_json::from_slice(&raw).expect("receipt json");
        assert_eq!(value["format"], 1);
        assert_eq!(value["receiptId"], receipt.receipt_id);
        assert_eq!(value["attachmentId"], receipt.file.attachment_id);
        assert_eq!(value["name"], "notes.md");
        assert_eq!(value["bytes"], FILE_BYTES.len());
        assert_eq!(
            value["mediaType"], "text/markdown",
            "media type is lowercased"
        );
        let created = value["createdAt"].as_str().expect("createdAt");
        assert_eq!(created.len(), 24);
        assert!(created.ends_with('Z') && created >= before.as_str());
        assert_eq!(value.as_object().unwrap().len(), 7);
        assert_eq!(
            serde_json_canonicalizer::to_vec(&value).expect("canonical"),
            raw[..raw.len() - 1].to_vec(),
            "receipt is canonical JSON + LF"
        );
        use std::os::unix::fs::PermissionsExt as _;
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600
        );

        let default = upload(root.path(), SESSION, "blob.bin", None);
        let raw = fs::read(
            folder
                .join("uploads")
                .join(format!("{}.json", default.receipt_id)),
        )
        .unwrap();
        let value: Value = serde_json::from_slice(&raw).unwrap();
        assert_eq!(value["mediaType"], DEFAULT_FILE_MEDIA_TYPE);
    }

    #[test]
    fn upload_file_rejects_every_invalid_request_without_publishing() {
        let root = tempfile::tempdir().expect("root");
        create_session(root.path(), SESSION, &[]);
        let authority = AttachmentAuthority::open(root.path());
        let data = BASE64.encode(FILE_BYTES);

        assert!(matches!(
            authority.upload_file("not-a-session", "a.txt", None, &data),
            Err(PromptMaterializeError::EndpointType(_))
        ));
        assert!(matches!(
            authority.upload_file(OTHER_SESSION, "a.txt", None, &data),
            Err(PromptMaterializeError::SessionNotFound(_))
        ));
        for name in ["", "dir/a.txt", "a\u{7}b", &"x".repeat(256)] {
            assert!(
                matches!(
                    authority.upload_file(SESSION, name, None, &data),
                    Err(PromptMaterializeError::InvalidName)
                ),
                "name {name:?}"
            );
        }
        for media in [
            "",
            "text",
            "text/",
            "/plain",
            "text/plain; charset=utf-8",
            "te xt/plain",
            "text/pl\u{e9}in",
        ] {
            assert!(
                matches!(
                    authority.upload_file(SESSION, "a.txt", Some(media), &data),
                    Err(PromptMaterializeError::InvalidMediaType)
                ),
                "media {media:?}"
            );
        }
        for media in [
            "image/png",
            "image/jpeg",
            "image/webp",
            "image/gif",
            "IMAGE/PNG",
        ] {
            assert_eq!(
                file_reason(
                    &authority
                        .upload_file(SESSION, "a.png", Some(media), &data)
                        .unwrap_err()
                ),
                Some(AttachmentErrorReason::InlineMediaType),
                "media {media:?}"
            );
        }
        for bad in ["", "abc", "ab=c", "a b c d", "AAA=="] {
            assert_eq!(
                file_reason(
                    &authority
                        .upload_file(SESSION, "a.txt", None, bad)
                        .unwrap_err()
                ),
                Some(AttachmentErrorReason::InvalidBase64),
                "base64 {bad:?}"
            );
        }
        let too_large = BASE64.encode(vec![0_u8; MAX_ATTACHMENT_BYTES + 1]);
        assert_eq!(
            file_reason(
                &authority
                    .upload_file(SESSION, "a.bin", None, &too_large)
                    .unwrap_err()
            ),
            Some(AttachmentErrorReason::TooLarge)
        );
        let folder = root.path().join("threads").join(SESSION);
        assert!(
            !folder.join("uploads").exists(),
            "no receipt written on failure"
        );
        assert_eq!(
            fs::read_dir(folder.join("assets")).unwrap().count(),
            0,
            "no asset published on failure"
        );

        let max = BASE64.encode(vec![0_u8; MAX_ATTACHMENT_BYTES]);
        authority
            .upload_file(SESSION, "a.bin", None, &max)
            .expect("8 MiB is the inclusive maximum");

        fs::create_dir_all(root.path().join("archive")).unwrap();
        fs::rename(&folder, root.path().join("archive").join(SESSION)).unwrap();
        assert!(matches!(
            authority.upload_file(SESSION, "a.txt", None, &data),
            Err(PromptMaterializeError::Archived(_))
        ));
    }

    #[test]
    fn file_receipt_part_materializes_a_file_block() {
        let root = tempfile::tempdir().expect("root");
        create_session(root.path(), SESSION, &[]);
        let receipt = upload(root.path(), SESSION, "spec.pdf", Some("application/pdf"));
        let authority = AttachmentAuthority::open(root.path());
        let parts = [
            PromptPart::Text {
                text: "read this".to_owned(),
            },
            PromptPart::File {
                receipt_id: receipt.receipt_id.clone(),
            },
        ];
        let prompt = authority
            .materialize_prompt_parts(SESSION, &parts)
            .expect("materialized");
        let digest = receipt.file.attachment_id.strip_prefix("sha256:").unwrap();
        assert_eq!(
            prompt.blocks,
            vec![
                Block::Text {
                    text: "read this".to_owned()
                },
                Block::File {
                    asset: format!("sha256-{digest}"),
                    mime: "application/pdf".to_owned(),
                    name: "spec.pdf".to_owned(),
                    bytes: FILE_BYTES.len() as u64,
                },
            ]
        );
        assert!(prompt.attachments.is_empty());
        assert_eq!(
            prompt.files,
            vec![FileAttachmentRef {
                attachment_id: receipt.file.attachment_id.clone(),
                name: "spec.pdf".to_owned(),
                bytes: FILE_BYTES.len() as u64,
                media_type: "application/pdf".to_owned(),
            }]
        );
        assert_eq!(
            serde_json::to_value(&prompt.files[0]).unwrap(),
            json!({"attachmentId":receipt.file.attachment_id,"name":"spec.pdf","bytes":FILE_BYTES.len(),"mediaType":"application/pdf"})
        );
        // A receipt is reusable across prompts of its session.
        authority
            .materialize_prompt_parts(SESSION, &parts)
            .expect("receipt reused");

        // Wire shape of the part itself.
        assert_eq!(
            serde_json::from_value::<PromptPart>(
                json!({"type":"file","receiptId":receipt.receipt_id})
            )
            .unwrap(),
            parts[1]
        );
    }

    #[test]
    fn unknown_cross_session_and_archived_receipts_are_rejected() {
        let root = tempfile::tempdir().expect("root");
        create_session(root.path(), SESSION, &[]);
        create_session(root.path(), OTHER_SESSION, &[]);
        let authority = AttachmentAuthority::open(root.path());
        let receipt = upload(root.path(), OTHER_SESSION, "other.txt", None);

        for id in [
            "nope",
            "../x",
            "018F0000-0000-4000-8000-000000000000",
            "018f0000-0000-4000-8000-000000000000",
        ] {
            let error = authority
                .materialize_prompt_parts(
                    SESSION,
                    &[PromptPart::File {
                        receipt_id: id.to_owned(),
                    }],
                )
                .unwrap_err();
            assert_eq!(
                file_reason(&error),
                Some(AttachmentErrorReason::UnknownReceipt),
                "{id}"
            );
        }
        let cross = authority
            .materialize_prompt_parts(
                SESSION,
                &[PromptPart::File {
                    receipt_id: receipt.receipt_id.clone(),
                }],
            )
            .unwrap_err();
        assert_eq!(
            file_reason(&cross),
            Some(AttachmentErrorReason::UnknownReceipt)
        );

        // A receipt whose asset vanished is CORRUPT, not unknown.
        let digest = receipt.file.attachment_id.strip_prefix("sha256:").unwrap();
        fs::remove_file(
            root.path()
                .join("threads")
                .join(OTHER_SESSION)
                .join("assets")
                .join(format!("sha256-{digest}")),
        )
        .unwrap();
        let gone = authority
            .materialize_prompt_parts(
                OTHER_SESSION,
                &[PromptPart::File {
                    receipt_id: receipt.receipt_id.clone(),
                }],
            )
            .unwrap_err();
        assert_eq!(file_reason(&gone), Some(AttachmentErrorReason::Corrupt));

        fs::create_dir_all(root.path().join("archive")).unwrap();
        fs::rename(
            root.path().join("threads").join(OTHER_SESSION),
            root.path().join("archive").join(OTHER_SESSION),
        )
        .unwrap();
        assert!(matches!(
            authority.materialize_prompt_parts(
                OTHER_SESSION,
                &[PromptPart::File {
                    receipt_id: receipt.receipt_id
                }]
            ),
            Err(PromptMaterializeError::Archived(_))
        ));
    }

    #[test]
    fn read_authorized_file_requires_a_file_block_or_receipt_reference() {
        let root = tempfile::tempdir().expect("root");
        create_session(root.path(), SESSION, &[]);
        create_session(root.path(), OTHER_SESSION, &[]);
        let authority = AttachmentAuthority::open(root.path());
        let receipt = upload(root.path(), SESSION, "data.csv", Some("text/csv"));
        let id = receipt.file.attachment_id.clone();

        // Receipt-only: freshly uploaded, never prompted.
        let read = authority
            .read_authorized_file(SESSION, &id)
            .expect("receipt authorizes");
        assert_eq!(
            read.attachment,
            FileAttachmentRef {
                attachment_id: id.clone(),
                name: "data.csv".into(),
                bytes: FILE_BYTES.len() as u64,
                media_type: "text/csv".into()
            }
        );
        assert_eq!(read.data, BASE64.encode(FILE_BYTES));
        assert_eq!(
            serde_json::to_value(&read).unwrap(),
            json!({"attachment":{"attachmentId":id,"name":"data.csv","bytes":FILE_BYTES.len(),"mediaType":"text/csv"},"data":read.data})
        );

        // Cross-session digest guess is NOT_REFERENCED.
        assert!(matches!(
            authority.read_authorized_file(OTHER_SESSION, &id),
            Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::NotReferenced
            ))
        ));
        assert!(matches!(
            authority.read_authorized_file(SESSION, "sha256:zz"),
            Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::NotReferenced
            ))
        ));

        // Ledger file block authorizes, and its name/mime win over the receipt.
        let folder = root.path().join("threads").join(SESSION);
        fs::remove_dir_all(folder.join("uploads")).unwrap();
        assert!(matches!(
            authority.read_authorized_file(SESSION, &id),
            Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::NotReferenced
            ))
        ));
        let digest = id.strip_prefix("sha256:").unwrap();
        append_input(
            root.path(),
            SESSION,
            &[Block::File {
                asset: format!("sha256-{digest}"),
                mime: "text/plain".into(),
                name: "renamed.txt".into(),
                bytes: FILE_BYTES.len() as u64,
            }],
        );
        let read = authority
            .read_authorized_file(SESSION, &id)
            .expect("ledger authorizes");
        assert_eq!(read.attachment.name, "renamed.txt");
        assert_eq!(read.attachment.media_type, "text/plain");

        // Image reads are untouched by file references.
        assert!(matches!(
            authority.read_authorized(SESSION, &id),
            Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::NotReferenced
            ))
        ));

        // Corrupt asset bytes are never returned.
        fs::write(
            folder.join("assets").join(format!("sha256-{digest}")),
            b"tampered",
        )
        .unwrap();
        assert!(matches!(
            authority.read_authorized_file(SESSION, &id),
            Err(AttachmentReadError::Attachment(
                AttachmentErrorReason::Corrupt
            ))
        ));
    }

    #[test]
    fn endpoint_journal_file_block_authorizes_file_read() {
        let root = tempfile::tempdir().expect("root");
        create_session(root.path(), SESSION, &[]);
        let folder = root.path().join("threads").join(SESSION);
        let published = AssetStore::new(folder.join("assets"))
            .unwrap()
            .publish(FILE_BYTES)
            .unwrap();
        let id = attachment_id(FILE_BYTES);
        EndpointJournal::open(&folder)
            .unwrap()
            .append_kernel(
                vec![1],
                "attachment",
                SessionEvent {
                    event_type: "user/message".to_owned(),
                    seq: 0,
                    time: 1.0,
                    data: IJsonValue::parse(
                        &serde_json::to_vec(&json!({"content":[{"type":"file","asset":published.asset,"mime":"text/plain","name":"j.txt","bytes":published.bytes}]})).unwrap(),
                    )
                    .unwrap(),
                    ignorable: None,
                    source_event_seqs: None,
                    surface_op: Some(SurfaceOperation::Append("append".to_owned())),
                },
            )
            .unwrap();
        let read = AttachmentAuthority::open(root.path())
            .read_authorized_file(SESSION, &id)
            .expect("journal authorizes");
        assert_eq!(read.attachment.name, "j.txt");
        assert_eq!(read.attachment.bytes, FILE_BYTES.len() as u64);
    }

    #[test]
    fn file_blocks_project_to_public_attachment_references() {
        let digest = "a".repeat(64);
        let content = json!([
            {"type":"text","text":"hi"},
            {"type":"image","asset":format!("sha256-{digest}"),"mime":"image/png","name":"i.png"},
            {"type":"file","asset":format!("sha256-{digest}"),"mime":"application/pdf","name":"s.pdf","bytes":42}
        ]);
        assert_eq!(
            project_content_blocks(&content),
            json!([
                {"type":"text","text":"hi"},
                {"type":"image","asset":format!("sha256-{digest}"),"mime":"image/png","name":"i.png"},
                {"type":"file","attachment":{"attachmentId":format!("sha256:{digest}"),"name":"s.pdf","bytes":42},"mediaType":"application/pdf"}
            ])
        );
        let spill = json!({"$spill":{"asset":"x","bytes":1}});
        assert_eq!(project_content_blocks(&spill), spill);
    }

    #[test]
    fn rfc3339_formatter_matches_known_instants() {
        assert_eq!(rfc3339_from_millis(0), "1970-01-01T00:00:00.000Z");
        assert_eq!(
            rfc3339_from_millis(1_772_064_000_123),
            "2026-02-26T00:00:00.123Z"
        );
        assert_eq!(
            rfc3339_from_millis(1_709_251_199_001),
            "2024-02-29T23:59:59.001Z"
        );
        assert_eq!(
            rfc3339_from_millis(1_757_721_600_999),
            "2025-09-13T00:00:00.999Z"
        );
    }

    fn encoded_image(width: u32, height: u32, format: ImageFormat) -> Vec<u8> {
        let image = DynamicImage::ImageLuma8(GrayImage::new(width, height));
        let mut cursor = Cursor::new(Vec::new());
        image.write_to(&mut cursor, format).expect("encode image");
        cursor.into_inner()
    }

    fn animated_gif(frame_count: usize, width: u32, height: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        {
            let mut encoder = GifEncoder::new(&mut bytes);
            encoder
                .encode_frames((0..frame_count).map(|_| Frame::new(RgbaImage::new(width, height))))
                .expect("animated gif");
        }
        bytes
    }

    fn create_session(root: &Path, session_id: &str, blocks: &[Block]) {
        let folder = root.join("threads").join(session_id);
        fs::create_dir_all(folder.join("assets")).expect("session folder");
        let genesis = json!({
            "config":{"digest":"cfg-1"},
            "format":1,
            "kind":"genesis",
            "min_reader":1,
            "min_writer":1,
            "origin_key":"create",
            "origin_tuple":{
                "client":"client",
                "key":"create",
                "op":"create",
                "principal":"principal",
                "target":session_id
            },
            "resume":"never",
            "seq":1,
            "thread":session_id,
            "ts":"2026-08-26T09:00:00.000Z",
            "v":1,
            "workspace":"workspace"
        });
        let mut bytes = canonical_line(&genesis);
        if !blocks.is_empty() {
            bytes.extend(canonical_line(&input_event(session_id, blocks)));
        }
        fs::write(folder.join("main.jsonl"), bytes).expect("ledger");
    }

    fn append_input(root: &Path, session_id: &str, blocks: &[Block]) {
        let path = root.join("threads").join(session_id).join("main.jsonl");
        let mut bytes = fs::read(&path).expect("ledger");
        bytes.extend(canonical_line(&input_event(session_id, blocks)));
        fs::write(path, bytes).expect("input");
    }

    fn input_event(session_id: &str, blocks: &[Block]) -> Value {
        json!({
            "content":blocks,
            "kind":"input",
            "origin_key":"prompt",
            "origin_tuple":{
                "client":"client",
                "key":"prompt",
                "op":"submit",
                "principal":"principal",
                "target":session_id
            },
            "seq":2,
            "ts":"2026-08-26T09:00:01.000Z",
            "v":1
        })
    }

    fn canonical_line(value: &Value) -> Vec<u8> {
        let mut bytes = serde_json_canonicalizer::to_vec(value).expect("canonical");
        bytes.push(b'\n');
        bytes
    }
}
