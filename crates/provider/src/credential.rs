use std::collections::BTreeMap;
use std::io::{self, Read, Write};
use std::os::fd::{FromRawFd, RawFd};
use std::os::unix::net::UnixStream;
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::thread::JoinHandle;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use zeroize::Zeroize;

const MAX_BODY: usize = 65_536;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CredentialGet {
    pub request_id: String,
    pub attempt: String,
    pub credential_id: String,
    pub purpose: String,
    pub adapter: String,
    pub endpoint_origin: String,
}

/// The material answering one `credential_get`: resolved for that request
/// (attempt or tool call) and held only in the worker's owned memory until the
/// attempt settles; zeroed on drop. Rotation and revocation reach the next
/// request — there is no lease to track or revoke in flight.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CredentialMaterial {
    pub request_id: String,
    pub material: String,
    pub generation: String,
}

impl std::fmt::Debug for CredentialMaterial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CredentialMaterial")
            .field("request_id", &self.request_id)
            .field("material", &"<redacted>")
            .field("generation", &self.generation)
            .finish()
    }
}

impl Drop for CredentialMaterial {
    fn drop(&mut self) {
        self.material.zeroize();
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CredentialError {
    pub request_id: String,
    pub code: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialMessage {
    CredentialGet(CredentialGet),
    Credential(CredentialMaterial),
    CredentialError(CredentialError),
}

#[derive(Debug, Error)]
pub enum CredentialFrameError {
    #[error("credential frame length is invalid")]
    InvalidLength,
    #[error("credential frame is invalid JSON: {0}")]
    InvalidJson(String),
}

#[derive(Default)]
pub struct CredentialDecoder {
    bytes: Vec<u8>,
}

impl CredentialDecoder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, bytes: &[u8]) -> Result<Vec<CredentialMessage>, CredentialFrameError> {
        self.bytes.extend_from_slice(bytes);
        let mut messages = Vec::new();
        loop {
            if self.bytes.len() < 4 {
                break;
            }
            let length = u32::from_be_bytes(
                self.bytes[..4]
                    .try_into()
                    .map_err(|_| CredentialFrameError::InvalidLength)?,
            ) as usize;
            if length == 0 || length > MAX_BODY {
                return Err(CredentialFrameError::InvalidLength);
            }
            if self.bytes.len() < length + 4 {
                break;
            }
            let frame = self.bytes.drain(..length + 4).collect::<Vec<_>>();
            messages.push(decode_credential_frame(&frame)?);
        }
        Ok(messages)
    }

    pub fn finish(self) -> Result<(), CredentialFrameError> {
        if self.bytes.is_empty() {
            Ok(())
        } else {
            Err(CredentialFrameError::InvalidLength)
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct CredentialScope {
    pub credential_id: String,
    pub adapter: String,
    pub endpoint_origin: String,
    pub purpose: String,
    pub generation: String,
    pub material: String,
}

impl std::fmt::Debug for CredentialScope {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CredentialScope")
            .field("credential_id", &self.credential_id)
            .field("adapter", &self.adapter)
            .field("endpoint_origin", &self.endpoint_origin)
            .field("purpose", &self.purpose)
            .field("generation", &self.generation)
            .field("material", &"<redacted>")
            .finish()
    }
}

impl Drop for CredentialScope {
    fn drop(&mut self) {
        self.material.zeroize();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevokedCredentialScope {
    pub credential_id: String,
    pub adapter: String,
    pub endpoint_origin: String,
    pub purpose: String,
    pub generation: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BrokerDecision {
    Material(CredentialMaterial),
    Error(CredentialError),
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum BrokerError {
    #[error("credential request id does not match its request tuple")]
    InvalidRequestId,
    #[error("request id was reused for different request bytes")]
    RequestConflict,
}

#[derive(Debug, Error)]
pub enum CredentialClientError {
    #[error("credential channel IO failed: {0}")]
    Io(#[from] io::Error),
    #[error(transparent)]
    Frame(#[from] CredentialFrameError),
    #[error(transparent)]
    Broker(#[from] BrokerError),
    #[error("credential broker rejected request: {0}")]
    Rejected(String),
    #[error("credential broker sent an unexpected message")]
    UnexpectedMessage,
}

#[derive(Debug, Error)]
pub enum CredentialControlError {
    #[error("credential broker service is closed")]
    Closed,
    #[error("credential broker rotation failed: {0}")]
    Rotation(String),
}

enum BrokerCommand {
    Rotate {
        credential_id: String,
        generation: String,
        material: String,
        reply: SyncSender<Result<(), String>>,
    },
    Revoke {
        credential_id: String,
        generation: String,
        reply: SyncSender<Result<(), String>>,
    },
}

/// Supervisor-side control for a live private credential channel.
#[derive(Clone)]
pub struct CredentialBrokerControl {
    commands: Sender<BrokerCommand>,
}

impl CredentialBrokerControl {
    /// Publishes a new generation: every later `credential_get` for the id
    /// answers with the new material. Requests already answered keep what they
    /// hold until their attempt settles.
    pub fn rotate(
        &self,
        credential_id: impl Into<String>,
        generation: impl Into<String>,
        material: impl Into<String>,
    ) -> Result<(), CredentialControlError> {
        let (reply, result) = mpsc::sync_channel(1);
        self.commands
            .send(BrokerCommand::Rotate {
                credential_id: credential_id.into(),
                generation: generation.into(),
                material: material.into(),
                reply,
            })
            .map_err(|_| CredentialControlError::Closed)?;
        result
            .recv()
            .map_err(|_| CredentialControlError::Closed)?
            .map_err(CredentialControlError::Rotation)
    }

    /// Revokes the named configured scope: every later `credential_get` for
    /// the id answers `revoked`.
    pub fn revoke(
        &self,
        credential_id: impl Into<String>,
        generation: impl Into<String>,
    ) -> Result<(), CredentialControlError> {
        let (reply, result) = mpsc::sync_channel(1);
        self.commands
            .send(BrokerCommand::Revoke {
                credential_id: credential_id.into(),
                generation: generation.into(),
                reply,
            })
            .map_err(|_| CredentialControlError::Closed)?;
        result
            .recv()
            .map_err(|_| CredentialControlError::Closed)?
            .map_err(CredentialControlError::Rotation)
    }
}

pub struct CredentialClient {
    stream: UnixStream,
}

impl CredentialClient {
    #[must_use]
    pub fn new(stream: UnixStream) -> Self {
        Self { stream }
    }

    pub fn from_inherited_fd(fd: RawFd) -> Result<Self, CredentialClientError> {
        if fd < 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "negative descriptor").into());
        }
        let mut socket_type: libc::c_int = 0;
        let mut length = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
        // SAFETY: `socket_type` and `length` are valid writable pointers and
        // the untrusted fd is only queried. Failure is returned before
        // ownership is assumed.
        if unsafe {
            libc::getsockopt(
                fd,
                libc::SOL_SOCKET,
                libc::SO_TYPE,
                (&raw mut socket_type).cast(),
                &raw mut length,
            )
        } == -1
            || socket_type != libc::SOCK_STREAM
        {
            return Err(io::Error::last_os_error().into());
        }
        let mut peer: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
        let mut peer_length = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
        // SAFETY: `peer` and `peer_length` are valid output storage. A
        // connected AF_UNIX peer is required; unconnected or foreign sockets
        // are rejected before ownership transfer.
        if unsafe { libc::getpeername(fd, (&raw mut peer).cast(), &raw mut peer_length) } == -1
            || i32::from(peer.ss_family) != libc::AF_UNIX
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "credential descriptor is not a connected AF_UNIX socket",
            )
            .into());
        }
        // SAFETY: the worker process receives exclusive ownership of the
        // descriptor named by `--credential-fd`; the successful socket probe
        // above establishes that it is a live stream socket.
        let stream = unsafe { UnixStream::from_raw_fd(fd) };
        let current = unsafe { libc::fcntl(fd, libc::F_GETFD) };
        if current == -1
            || unsafe { libc::fcntl(fd, libc::F_SETFD, current | libc::FD_CLOEXEC) } == -1
        {
            return Err(io::Error::last_os_error().into());
        }
        Ok(Self::new(stream))
    }

    pub fn get(
        &mut self,
        request: CredentialGet,
    ) -> Result<CredentialMaterial, CredentialClientError> {
        write_message(
            &mut self.stream,
            &CredentialMessage::CredentialGet(request.clone()),
        )?;
        match read_message(&mut self.stream)? {
            CredentialMessage::Credential(material)
                if material.request_id == request.request_id =>
            {
                Ok(material)
            }
            CredentialMessage::CredentialError(error) if error.request_id == request.request_id => {
                Err(CredentialClientError::Rejected(error.code))
            }
            _ => Err(CredentialClientError::UnexpectedMessage),
        }
    }
}

#[derive(Default)]
pub struct CredentialBroker {
    scopes: BTreeMap<String, Vec<CredentialScope>>,
    revoked_scopes: BTreeMap<String, Vec<RevokedCredentialScope>>,
    requests: BTreeMap<String, (CredentialGet, BrokerDecision)>,
}

impl CredentialBroker {
    #[must_use]
    pub fn new(scopes: impl IntoIterator<Item = CredentialScope>) -> Self {
        Self::with_revoked(scopes, [])
    }

    #[must_use]
    pub fn with_revoked(
        scopes: impl IntoIterator<Item = CredentialScope>,
        revoked_scopes: impl IntoIterator<Item = RevokedCredentialScope>,
    ) -> Self {
        let mut active = BTreeMap::<String, Vec<CredentialScope>>::new();
        for scope in scopes {
            active
                .entry(scope.credential_id.clone())
                .or_default()
                .push(scope);
        }
        let mut revoked = BTreeMap::<String, Vec<RevokedCredentialScope>>::new();
        for scope in revoked_scopes {
            revoked
                .entry(scope.credential_id.clone())
                .or_default()
                .push(scope);
        }
        Self {
            scopes: active,
            revoked_scopes: revoked,
            ..Self::default()
        }
    }

    pub fn get(&mut self, request: CredentialGet) -> Result<BrokerDecision, BrokerError> {
        let expected = credential_request_id(
            &request.attempt,
            &request.credential_id,
            &request.endpoint_origin,
        );
        if request.request_id != expected {
            return Err(BrokerError::InvalidRequestId);
        }
        if let Some((prior, decision)) = self.requests.get(&request.request_id) {
            return if prior == &request {
                Ok(decision.clone())
            } else {
                Err(BrokerError::RequestConflict)
            };
        }
        let active = self.scopes.get(&request.credential_id).and_then(|scopes| {
            scopes.iter().find(|scope| {
                scope.adapter == request.adapter
                    && scope.endpoint_origin == request.endpoint_origin
                    && scope.purpose == request.purpose
            })
        });
        let revoked = self
            .revoked_scopes
            .get(&request.credential_id)
            .and_then(|scopes| {
                scopes.iter().find(|scope| {
                    scope.adapter == request.adapter
                        && scope.endpoint_origin == request.endpoint_origin
                        && scope.purpose == request.purpose
                })
            });
        let known_id = self.scopes.contains_key(&request.credential_id)
            || self.revoked_scopes.contains_key(&request.credential_id);
        let decision = match (active, revoked, known_id) {
            (Some(scope), _, _) => BrokerDecision::Material(CredentialMaterial {
                request_id: request.request_id.clone(),
                material: scope.material.clone(),
                generation: scope.generation.clone(),
            }),
            (None, Some(_), _) => BrokerDecision::Error(CredentialError {
                request_id: request.request_id.clone(),
                code: "revoked".to_owned(),
            }),
            (None, None, true) => BrokerDecision::Error(CredentialError {
                request_id: request.request_id.clone(),
                code: "scope_mismatch".to_owned(),
            }),
            (None, None, false) => BrokerDecision::Error(CredentialError {
                request_id: request.request_id.clone(),
                code: "not_found".to_owned(),
            }),
        };
        self.requests
            .insert(request.request_id.clone(), (request, decision.clone()));
        Ok(decision)
    }

    /// Publishes a new generation for every scope of `credential_id`; later
    /// requests answer with it. Returns whether the id was known.
    pub fn rotate(
        &mut self,
        credential_id: &str,
        generation: impl Into<String>,
        material: impl Into<String>,
    ) -> bool {
        let generation = generation.into();
        let material = material.into();
        let mut found = false;
        if let Some(scopes) = self.scopes.get_mut(credential_id) {
            found = true;
            for scope in scopes {
                scope.generation.clone_from(&generation);
                scope.material.clone_from(&material);
            }
        }
        if let Some(revoked) = self.revoked_scopes.remove(credential_id) {
            found = true;
            self.scopes
                .entry(credential_id.to_owned())
                .or_default()
                .extend(revoked.into_iter().map(|scope| CredentialScope {
                    credential_id: scope.credential_id,
                    adapter: scope.adapter,
                    endpoint_origin: scope.endpoint_origin,
                    purpose: scope.purpose,
                    generation: generation.clone(),
                    material: material.clone(),
                }));
        }
        if found {
            self.requests.clear();
        }
        found
    }

    /// Revokes every scope of `credential_id`; later requests answer
    /// `revoked`. Returns whether the id was known.
    pub fn revoke(&mut self, credential_id: &str, generation: impl Into<String>) -> bool {
        let generation = generation.into();
        if let Some(active) = self.scopes.remove(credential_id) {
            self.revoked_scopes
                .entry(credential_id.to_owned())
                .or_default()
                .extend(active.into_iter().map(|scope| RevokedCredentialScope {
                    credential_id: scope.credential_id.clone(),
                    adapter: scope.adapter.clone(),
                    endpoint_origin: scope.endpoint_origin.clone(),
                    purpose: scope.purpose.clone(),
                    generation: generation.clone(),
                }));
        } else if let Some(scopes) = self.revoked_scopes.get_mut(credential_id) {
            for scope in scopes {
                scope.generation.clone_from(&generation);
            }
        } else {
            return false;
        }
        self.requests.clear();
        true
    }
}

pub fn serve_credential_channel(
    mut stream: UnixStream,
    mut broker: CredentialBroker,
) -> Result<(), CredentialClientError> {
    serve_credential_channel_inner(&mut stream, &mut broker, None)
}

/// Starts a live broker service and returns its supervisor-side rotation
/// control together with the service join handle.
#[must_use]
pub fn start_credential_channel(
    mut stream: UnixStream,
    mut broker: CredentialBroker,
) -> (
    CredentialBrokerControl,
    JoinHandle<Result<(), CredentialClientError>>,
) {
    let (commands, receiver) = mpsc::channel();
    let control = CredentialBrokerControl { commands };
    let join = std::thread::spawn(move || {
        serve_credential_channel_inner(&mut stream, &mut broker, Some(&receiver))
    });
    (control, join)
}

fn serve_credential_channel_inner(
    stream: &mut UnixStream,
    broker: &mut CredentialBroker,
    commands: Option<&Receiver<BrokerCommand>>,
) -> Result<(), CredentialClientError> {
    stream.set_read_timeout(Some(Duration::from_millis(25)))?;
    let mut decoder = CredentialDecoder::new();
    let mut bytes = [0_u8; 4096];
    loop {
        if let Some(commands) = commands {
            while let Ok(command) = commands.try_recv() {
                match command {
                    BrokerCommand::Rotate {
                        credential_id,
                        generation,
                        material,
                        reply,
                    } => {
                        broker.rotate(&credential_id, generation, material);
                        let _ = reply.send(Ok(()));
                    }
                    BrokerCommand::Revoke {
                        credential_id,
                        generation,
                        reply,
                    } => {
                        broker.revoke(&credential_id, generation);
                        let _ = reply.send(Ok(()));
                    }
                }
            }
        }
        match stream.read(&mut bytes) {
            Ok(0) => {
                decoder.finish()?;
                return Ok(());
            }
            Ok(count) => {
                for message in decoder.push(&bytes[..count])? {
                    match message {
                        CredentialMessage::CredentialGet(request) => {
                            let response = match broker.get(request)? {
                                BrokerDecision::Material(material) => {
                                    CredentialMessage::Credential(material)
                                }
                                BrokerDecision::Error(error) => {
                                    CredentialMessage::CredentialError(error)
                                }
                            };
                            write_message(stream, &response)?;
                        }
                        CredentialMessage::Credential(_)
                        | CredentialMessage::CredentialError(_) => {
                            return Err(CredentialClientError::UnexpectedMessage);
                        }
                    }
                }
            }
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                ) => {}
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::UnexpectedEof | io::ErrorKind::ConnectionReset
                ) =>
            {
                return Ok(());
            }
            Err(error) => return Err(error.into()),
        }
    }
}

fn write_message(
    stream: &mut UnixStream,
    message: &CredentialMessage,
) -> Result<(), CredentialClientError> {
    stream.write_all(&encode_credential_frame(message)?)?;
    Ok(())
}

fn read_message(stream: &mut UnixStream) -> Result<CredentialMessage, CredentialClientError> {
    let mut length = [0_u8; 4];
    stream.read_exact(&mut length)?;
    let body_length = u32::from_be_bytes(length) as usize;
    if body_length == 0 || body_length > MAX_BODY {
        return Err(CredentialFrameError::InvalidLength.into());
    }
    let mut body = vec![0_u8; body_length];
    stream.read_exact(&mut body)?;
    let mut frame = length.to_vec();
    frame.extend(body);
    Ok(decode_credential_frame(&frame)?)
}

pub fn encode_credential_frame(
    message: &CredentialMessage,
) -> Result<Vec<u8>, CredentialFrameError> {
    validate_message(message)?;
    let body = serde_json_canonicalizer::to_vec(message)
        .map_err(|error| CredentialFrameError::InvalidJson(error.to_string()))?;
    if body.is_empty() || body.len() > MAX_BODY {
        return Err(CredentialFrameError::InvalidLength);
    }
    let length = u32::try_from(body.len()).map_err(|_| CredentialFrameError::InvalidLength)?;
    let mut frame = Vec::with_capacity(4 + body.len());
    frame.extend_from_slice(&length.to_be_bytes());
    frame.extend_from_slice(&body);
    Ok(frame)
}

pub fn decode_credential_frame(frame: &[u8]) -> Result<CredentialMessage, CredentialFrameError> {
    let length_bytes: [u8; 4] = frame
        .get(..4)
        .ok_or(CredentialFrameError::InvalidLength)?
        .try_into()
        .map_err(|_| CredentialFrameError::InvalidLength)?;
    let length = usize::try_from(u32::from_be_bytes(length_bytes))
        .map_err(|_| CredentialFrameError::InvalidLength)?;
    if length == 0 || length > MAX_BODY || frame.len() != length + 4 {
        return Err(CredentialFrameError::InvalidLength);
    }
    let message: CredentialMessage = serde_json::from_slice(&frame[4..])
        .map_err(|error| CredentialFrameError::InvalidJson(error.to_string()))?;
    validate_message(&message)?;
    Ok(message)
}

fn validate_message(message: &CredentialMessage) -> Result<(), CredentialFrameError> {
    let invalid = || CredentialFrameError::InvalidJson("credential value is outside v1".into());
    match message {
        CredentialMessage::CredentialGet(value) => {
            if !valid_request_id(&value.request_id)
                || value.attempt.is_empty()
                || value.credential_id.is_empty()
                || !matches!(value.purpose.as_str(), "provider" | "web_search")
                || value.adapter.is_empty()
                || value.endpoint_origin.is_empty()
            {
                return Err(invalid());
            }
        }
        CredentialMessage::Credential(value) => {
            if !valid_request_id(&value.request_id)
                || value.material.is_empty()
                || value.generation.is_empty()
            {
                return Err(invalid());
            }
        }
        CredentialMessage::CredentialError(value) => {
            if !valid_request_id(&value.request_id)
                || !matches!(
                    value.code.as_str(),
                    "not_found" | "revoked" | "scope_mismatch" | "unavailable"
                )
            {
                return Err(invalid());
            }
        }
    }
    Ok(())
}

fn valid_request_id(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

#[must_use]
pub fn credential_request_id(attempt: &str, credential_id: &str, endpoint_origin: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"tekes-credential-v1\0");
    hasher.update(attempt.as_bytes());
    hasher.update([0]);
    hasher.update(credential_id.as_bytes());
    hasher.update([0]);
    hasher.update(endpoint_origin.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_round_trip_is_exact() {
        let message = CredentialMessage::CredentialError(CredentialError {
            request_id: "0".repeat(64),
            code: "revoked".to_owned(),
        });
        let frame = encode_credential_frame(&message).expect("encode");
        assert_eq!(decode_credential_frame(&frame).expect("decode"), message);
    }

    #[test]
    fn decoder_handles_partial_and_coalesced_frames() {
        let first = CredentialMessage::CredentialError(CredentialError {
            request_id: "1".repeat(64),
            code: "not_found".to_owned(),
        });
        let second = CredentialMessage::CredentialError(CredentialError {
            request_id: "2".repeat(64),
            code: "revoked".to_owned(),
        });
        let mut bytes = encode_credential_frame(&first).expect("first");
        bytes.extend(encode_credential_frame(&second).expect("second"));
        for split in 0..=bytes.len() {
            let mut decoder = CredentialDecoder::new();
            let mut messages = decoder.push(&bytes[..split]).expect("prefix");
            messages.extend(decoder.push(&bytes[split..]).expect("suffix"));
            assert_eq!(messages, [first.clone(), second.clone()]);
            decoder.finish().expect("complete");
        }
    }

    #[test]
    fn broker_deduplicates_and_rotates() {
        let scope = CredentialScope {
            credential_id: "provider-main".to_owned(),
            adapter: "responses".to_owned(),
            endpoint_origin: "https://api.example:443".to_owned(),
            purpose: "provider".to_owned(),
            generation: "g1".to_owned(),
            material: "secret".to_owned(),
        };
        let mut broker = CredentialBroker::new([scope]);
        let request_id = credential_request_id("a1", "provider-main", "https://api.example:443");
        let request = CredentialGet {
            request_id,
            attempt: "a1".to_owned(),
            credential_id: "provider-main".to_owned(),
            purpose: "provider".to_owned(),
            adapter: "responses".to_owned(),
            endpoint_origin: "https://api.example:443".to_owned(),
        };
        let first = broker.get(request.clone()).expect("first");
        assert_eq!(broker.get(request.clone()).expect("dedup"), first);
        assert!(broker.rotate("provider-main", "g2", "rotated"));
        // The rotation reaches the next request: the same request id answers
        // with the new generation once the dedup window is cleared.
        match broker.get(request).expect("after rotation") {
            BrokerDecision::Material(material) => {
                assert_eq!(material.generation, "g2");
                assert_eq!(material.material, "rotated");
            }
            other => panic!("{other:?}"),
        }
        assert!(broker.revoke("provider-main", "g3"));
        let revoked = CredentialGet {
            request_id: credential_request_id("a2", "provider-main", "https://api.example:443"),
            attempt: "a2".to_owned(),
            credential_id: "provider-main".to_owned(),
            purpose: "provider".to_owned(),
            adapter: "responses".to_owned(),
            endpoint_origin: "https://api.example:443".to_owned(),
        };
        assert!(matches!(
            broker.get(revoked).expect("revoked answer"),
            BrokerDecision::Error(CredentialError { code, .. }) if code == "revoked"
        ));
    }

    #[test]
    fn private_socket_channel_round_trips() {
        let (server, client) = UnixStream::pair().expect("socket pair");
        let broker = CredentialBroker::new([CredentialScope {
            credential_id: "main".to_owned(),
            adapter: "responses".to_owned(),
            endpoint_origin: "https://api.example:443".to_owned(),
            purpose: "provider".to_owned(),
            generation: "g1".to_owned(),
            material: "secret".to_owned(),
        }]);
        let server_thread =
            std::thread::spawn(move || serve_credential_channel(server, broker).expect("serve"));
        let mut client = CredentialClient::new(client);
        let request = CredentialGet {
            request_id: credential_request_id("a1", "main", "https://api.example:443"),
            attempt: "a1".to_owned(),
            credential_id: "main".to_owned(),
            purpose: "provider".to_owned(),
            adapter: "responses".to_owned(),
            endpoint_origin: "https://api.example:443".to_owned(),
        };
        let material = client.get(request).expect("material");
        assert_eq!(material.material, "secret");
        drop(client);
        server_thread.join().expect("join");
    }

    #[test]
    fn live_rotation_and_revocation_reach_the_next_request() {
        let (server, client_stream) = UnixStream::pair().expect("socket pair");
        let broker = CredentialBroker::new([CredentialScope {
            credential_id: "main".to_owned(),
            adapter: "responses".to_owned(),
            endpoint_origin: "https://api.example:443".to_owned(),
            purpose: "provider".to_owned(),
            generation: "g1".to_owned(),
            material: "secret".to_owned(),
        }]);
        let (control, service) = start_credential_channel(server, broker);
        let mut client = CredentialClient::new(client_stream);
        let get = |attempt: &str| CredentialGet {
            request_id: credential_request_id(attempt, "main", "https://api.example:443"),
            attempt: attempt.to_owned(),
            credential_id: "main".to_owned(),
            purpose: "provider".to_owned(),
            adapter: "responses".to_owned(),
            endpoint_origin: "https://api.example:443".to_owned(),
        };
        let first = client.get(get("a1")).expect("first material");
        assert_eq!(
            (first.generation.as_str(), first.material.as_str()),
            ("g1", "secret")
        );
        control.rotate("main", "g2", "rotated").expect("rotate");
        let second = client.get(get("a2")).expect("rotated material");
        assert_eq!(
            (second.generation.as_str(), second.material.as_str()),
            ("g2", "rotated")
        );
        assert_eq!(
            first.material, "secret",
            "an answered request keeps what it holds"
        );
        control.revoke("main", "g3").expect("revoke");
        assert!(matches!(
            client.get(get("a3")),
            Err(CredentialClientError::Rejected(code)) if code == "revoked"
        ));
        drop(client);
        service.join().expect("join").expect("clean EOF");
    }

    #[test]
    fn shared_key_keeps_every_exact_scope_and_rotates_them_together() {
        let definitions = [
            ("responses", "https://one.example:443", "provider"),
            ("anthropic", "https://two.example:443", "provider"),
            ("responses", "https://one.example:443", "web_search"),
        ];
        let scopes = definitions.map(|(adapter, endpoint_origin, purpose)| CredentialScope {
            credential_id: "shared".to_owned(),
            adapter: adapter.to_owned(),
            endpoint_origin: endpoint_origin.to_owned(),
            purpose: purpose.to_owned(),
            generation: "1".to_owned(),
            material: "fixture-secret-never-log".to_owned(),
        });
        let mut broker = CredentialBroker::new(scopes);
        for (index, (adapter, endpoint_origin, purpose)) in definitions.into_iter().enumerate() {
            let attempt = format!("a{index}");
            assert!(matches!(
                broker
                    .get(CredentialGet {
                        request_id: credential_request_id(&attempt, "shared", endpoint_origin,),
                        attempt,
                        credential_id: "shared".to_owned(),
                        purpose: purpose.to_owned(),
                        adapter: adapter.to_owned(),
                        endpoint_origin: endpoint_origin.to_owned(),
                    })
                    .expect("exact scope"),
                BrokerDecision::Material(_)
            ));
        }
        let mismatch_origin = "https://wrong.example:443";
        let mismatch = broker
            .get(CredentialGet {
                request_id: credential_request_id("wrong", "shared", mismatch_origin),
                attempt: "wrong".to_owned(),
                credential_id: "shared".to_owned(),
                purpose: "provider".to_owned(),
                adapter: "responses".to_owned(),
                endpoint_origin: mismatch_origin.to_owned(),
            })
            .expect("typed mismatch");
        assert!(matches!(
            mismatch,
            BrokerDecision::Error(CredentialError { code, .. }) if code == "scope_mismatch"
        ));
        assert!(broker.rotate("shared", "2", "rotated"));
    }

    #[test]
    fn wire_rejects_unknown_fields_and_open_string_unions() {
        fn frame(body: &[u8]) -> Vec<u8> {
            let mut frame = (body.len() as u32).to_be_bytes().to_vec();
            frame.extend_from_slice(body);
            frame
        }
        let request_id = "0".repeat(64);
        let unknown = format!(
            "{{\"credential_error\":{{\"code\":\"revoked\",\"request_id\":\"{request_id}\",\"unknown\":true}}}}"
        );
        assert!(decode_credential_frame(&frame(unknown.as_bytes())).is_err());
        let open_error = format!(
            "{{\"credential_error\":{{\"code\":\"future\",\"request_id\":\"{request_id}\"}}}}"
        );
        assert!(decode_credential_frame(&frame(open_error.as_bytes())).is_err());
        let open_purpose = format!(
            "{{\"credential_get\":{{\"adapter\":\"responses\",\"attempt\":\"a1\",\"credential_id\":\"main\",\"endpoint_origin\":\"https://api.example:443\",\"purpose\":\"future\",\"request_id\":\"{request_id}\"}}}}"
        );
        assert!(decode_credential_frame(&frame(open_purpose.as_bytes())).is_err());
    }
}
