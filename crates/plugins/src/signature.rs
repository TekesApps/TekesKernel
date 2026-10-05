use std::path::Path;
#[cfg(target_os = "macos")]
use std::process::Command;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use ring::signature::{ED25519, UnparsedPublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::PluginError;

pub const PUBLISHER_ATTESTATION_PATH: &str = ".codex-plugin/publisher.sig.json";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeHelperIdentity {
    pub component_id: String,
    pub relative_path: String,
    pub designated_requirement: String,
    pub signing_identifier: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_identifier: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublisherIdentity {
    pub publisher_id: String,
    pub public_key_fingerprint: String,
    pub signed_content_digest: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SignaturePolicy {
    pub allow_unsigned_local: bool,
    /// Publisher id to lowercase SHA-256 fingerprint of the trusted Ed25519 key.
    pub trusted_publishers: std::collections::BTreeMap<String, String>,
}

pub trait NativeHelperVerifier: Send + Sync {
    fn verify(
        &self,
        executable: &Path,
        component_id: &str,
        relative_path: &str,
    ) -> Result<NativeHelperIdentity, PluginError>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MacOsNativeHelperVerifier;

impl NativeHelperVerifier for MacOsNativeHelperVerifier {
    fn verify(
        &self,
        executable: &Path,
        component_id: &str,
        relative_path: &str,
    ) -> Result<NativeHelperIdentity, PluginError> {
        #[cfg(target_os = "macos")]
        {
            let verified = Command::new("/usr/bin/codesign")
                .args(["--verify", "--strict", "--verbose=4"])
                .arg(executable)
                .output()
                .map_err(|error| PluginError::Signature(error.to_string()))?;
            if !verified.status.success() {
                return Err(PluginError::Signature(format!(
                    "native helper {component_id} failed strict code-signature validation"
                )));
            }
            let displayed = Command::new("/usr/bin/codesign")
                .args(["--display", "--verbose=4", "-r-"])
                .arg(executable)
                .output()
                .map_err(|error| PluginError::Signature(error.to_string()))?;
            if !displayed.status.success() {
                return Err(PluginError::Signature(format!(
                    "native helper {component_id} has no signing identity"
                )));
            }
            let text = format!(
                "{}\n{}",
                String::from_utf8_lossy(&displayed.stdout),
                String::from_utf8_lossy(&displayed.stderr)
            );
            let requirement = text
                .lines()
                .find_map(|line| {
                    line.strip_prefix("# designated => ")
                        .or_else(|| line.strip_prefix("designated => "))
                })
                .ok_or_else(|| {
                    PluginError::Signature("missing designated requirement".to_owned())
                })?;
            let field = |name: &str| {
                text.lines()
                    .find_map(|line| line.strip_prefix(name))
                    .map(str::to_owned)
            };
            let signing_identifier = field("Identifier=")
                .ok_or_else(|| PluginError::Signature("missing signing identifier".to_owned()))?;
            Ok(NativeHelperIdentity {
                component_id: component_id.to_owned(),
                relative_path: relative_path.to_owned(),
                designated_requirement: requirement.to_owned(),
                signing_identifier,
                team_identifier: field("TeamIdentifier=").filter(|value| value != "not set"),
            })
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (executable, component_id, relative_path);
            Err(PluginError::Signature(
                "native helper signatures require macOS".to_owned(),
            ))
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PublisherAttestation {
    #[serde(rename = "publisherID")]
    publisher_id: String,
    public_key: String,
    signature: String,
    content_digest: String,
}

pub(crate) fn verify_publisher(
    root: &Path,
    digest_without_attestation: impl FnOnce() -> Result<String, PluginError>,
) -> Result<Option<PublisherIdentity>, PluginError> {
    let path = root.join(PUBLISHER_ATTESTATION_PATH);
    if !path.is_file() {
        return Ok(None);
    }
    let attestation: PublisherAttestation = serde_json::from_slice(
        &std::fs::read(&path).map_err(|error| PluginError::Storage(error.to_string()))?,
    )
    .map_err(|error| PluginError::Signature(error.to_string()))?;
    if attestation.publisher_id.is_empty()
        || attestation.publisher_id.len() > 256
        || !attestation
            .publisher_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b':' | b'_' | b'-'))
        || attestation.content_digest.len() != 64
        || !attestation
            .content_digest
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(PluginError::Signature(
            "invalid publisher attestation fields".to_owned(),
        ));
    }
    let public_key = STANDARD
        .decode(attestation.public_key)
        .map_err(|error| PluginError::Signature(error.to_string()))?;
    let signature = STANDARD
        .decode(attestation.signature)
        .map_err(|error| PluginError::Signature(error.to_string()))?;
    let actual = digest_without_attestation()?;
    if actual != attestation.content_digest {
        return Err(PluginError::Signature(
            "publisher content digest mismatch".to_owned(),
        ));
    }
    UnparsedPublicKey::new(&ED25519, &public_key)
        .verify(attestation.content_digest.as_bytes(), &signature)
        .map_err(|_| PluginError::Signature("publisher signature rejected".to_owned()))?;
    Ok(Some(PublisherIdentity {
        publisher_id: attestation.publisher_id,
        public_key_fingerprint: hex(&Sha256::digest(&public_key)),
        signed_content_digest: attestation.content_digest,
    }))
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}
