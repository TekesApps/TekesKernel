//! Private launcher pipe, consumed before any host side effects. No secret
//! enters launch JSON, argv, initial environment, readiness or diagnostics.
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::os::fd::RawFd;
use std::time::{Duration, Instant};

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use zeroize::{Zeroize, Zeroizing};

pub const VERSION: u64 = 1;
pub const MAGIC: &[u8] = b"TEKES-KERNEL-BOOTSTRAP/1\n";
pub const MAX_FRAME_BYTES: usize = 1024 * 1024;
pub const DEADLINE: Duration = Duration::from_secs(10);

#[derive(Deserialize)]
struct SecretText(String);

impl Drop for SecretText {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

#[derive(Default)]
struct Credentials(BTreeMap<String, SecretText>);

impl<'de> Deserialize<'de> for Credentials {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Unique;
        impl<'de> Visitor<'de> for Unique {
            type Value = Credentials;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("unique credential identifiers")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut result = Credentials::default();
                while let Some((key, value)) = map.next_entry::<String, SecretText>()? {
                    if result.0.contains_key(&key) {
                        return Err(serde::de::Error::custom("duplicate credential identifier"));
                    }
                    result.0.insert(key, value);
                }
                Ok(result)
            }
        }
        deserializer.deserialize_map(Unique)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Frame {
    endpoint_token: SecretText,
    credentials: Credentials,
}

pub struct LaunchSecrets {
    pub token: Zeroizing<[u8; 32]>,
    pub store: provider::MemorySecretStore,
}

fn rejected() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "private-bootstrap-rejected")
}

/// The owner keeps `fd` open for the existing EOF lifetime contract. Each read
/// consumes exactly the requested bytes; no buffered reader can eat later data.
pub fn read(
    fd: RawFd,
    bindings: &BTreeMap<String, String>,
    timeout: Duration,
) -> io::Result<LaunchSecrets> {
    // A regular launch/config file must never become a secret carrier.
    let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
    // SAFETY: fstat initializes the live out-parameter on success.
    if unsafe { libc::fstat(fd, stat.as_mut_ptr()) } != 0 {
        return Err(rejected());
    }
    // SAFETY: the successful fstat above initialized every field.
    let kind = unsafe { stat.assume_init() }.st_mode & libc::S_IFMT;
    if kind != libc::S_IFIFO && kind != libc::S_IFSOCK {
        return Err(rejected());
    }
    let deadline = Instant::now().checked_add(timeout).ok_or_else(rejected)?;
    let mut magic = [0; MAGIC.len()];
    read_exact(fd, &mut magic, deadline)?;
    if magic != MAGIC {
        return Err(rejected());
    }
    let mut length = [0; 4];
    read_exact(fd, &mut length, deadline)?;
    let length = u32::from_be_bytes(length) as usize;
    if length == 0 || length > MAX_FRAME_BYTES {
        return Err(rejected());
    }
    let mut bytes = Zeroizing::new(vec![0; length]);
    read_exact(fd, &mut bytes, deadline)?;
    decode(&bytes, bindings)
}

fn read_exact(fd: RawFd, mut target: &mut [u8], deadline: Instant) -> io::Result<()> {
    while !target.is_empty() {
        let remaining = deadline
            .checked_duration_since(Instant::now())
            .ok_or_else(rejected)?;
        let millis = remaining.as_millis().max(1).min(i32::MAX as u128) as i32;
        let mut descriptor = libc::pollfd {
            fd,
            events: libc::POLLIN,
            revents: 0,
        };
        // SAFETY: descriptor is initialized and poll touches one descriptor.
        let ready = unsafe { libc::poll(&mut descriptor, 1, millis) };
        if ready < 0 && io::Error::last_os_error().kind() == io::ErrorKind::Interrupted {
            continue;
        }
        if ready <= 0 || descriptor.revents & libc::POLLNVAL != 0 {
            return Err(rejected());
        }
        // SAFETY: target is writable for exactly this bounded read. No other
        // code reads the bootstrap pipe before the later EOF watcher starts.
        let count = unsafe { libc::read(fd, target.as_mut_ptr().cast(), target.len()) };
        if count < 0 && io::Error::last_os_error().kind() == io::ErrorKind::Interrupted {
            continue;
        }
        if count <= 0 {
            return Err(rejected());
        }
        target = &mut target[count as usize..];
    }
    Ok(())
}

fn decode(bytes: &[u8], bindings: &BTreeMap<String, String>) -> io::Result<LaunchSecrets> {
    let frame: Frame = serde_json::from_slice(bytes).map_err(|_| rejected())?;
    if frame.credentials.0.keys().collect::<BTreeSet<_>>() != bindings.keys().collect() {
        return Err(rejected());
    }
    let hex = frame.endpoint_token.0.as_bytes();
    if hex.len() != 64 || !hex.iter().all(u8::is_ascii_hexdigit) {
        return Err(rejected());
    }
    let mut token = Zeroizing::new([0; 32]);
    for (index, pair) in hex.chunks_exact(2).enumerate() {
        token[index] = u8::from_str_radix(std::str::from_utf8(pair).map_err(|_| rejected())?, 16)
            .map_err(|_| rejected())?;
    }
    let store = provider::MemorySecretStore::new();
    for (id, mut material) in frame.credentials.0 {
        store
            .publish(
                &id,
                provider::SecretRecord::Active {
                    generation: 1,
                    material: std::mem::take(&mut material.0),
                },
            )
            .map_err(|_| rejected())?;
    }
    Ok(LaunchSecrets { token, store })
}

/// Presence is rejected without logging or retaining the environment value.
pub fn rejects_legacy_environment(bindings: &BTreeMap<String, String>) -> bool {
    std::iter::once("TEKES_KERNEL_ENDPOINT_TOKEN")
        .chain(bindings.values().map(String::as_str))
        .any(|name| match std::env::var_os(name) {
            Some(value) => {
                Zeroizing::new(value.into_encoded_bytes());
                true
            }
            None => false,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use provider::SecretStore;
    use std::io::{Read, Write};
    use std::os::fd::AsRawFd;
    use std::os::unix::net::UnixStream;

    fn body() -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({
            "endpointToken":"ab".repeat(32),"credentials":{"provider-key":"synthetic-key"}
        }))
        .unwrap()
    }

    fn bindings() -> BTreeMap<String, String> {
        [("provider-key".into(), "OLD_SELECTED_ENV".into())].into()
    }

    fn framed(body: &[u8]) -> Vec<u8> {
        [MAGIC, &(body.len() as u32).to_be_bytes(), body].concat()
    }

    #[test]
    fn short_reads_keep_the_exact_frame_and_leave_lifetime_bytes_unconsumed() {
        let (mut launcher, mut host) = UnixStream::pair().unwrap();
        let bytes = framed(&body());
        let writer = std::thread::spawn(move || {
            for part in bytes.chunks(3) {
                launcher.write_all(part).unwrap();
            }
            launcher.write_all(b"lifetime").unwrap();
        });
        let secrets = read(host.as_raw_fd(), &bindings(), Duration::from_secs(2)).unwrap();
        assert_eq!(*secrets.token, [0xab; 32]);
        assert!(
            matches!(secrets.store.resolve("provider-key").unwrap(), provider::SecretResolution::Active(record) if record.generation() == 1)
        );
        assert_eq!(
            secrets.store.resolve("unselected").unwrap(),
            provider::SecretResolution::NotFound
        );
        let mut rest = Vec::new();
        host.read_to_end(&mut rest).unwrap();
        writer.join().unwrap();
        assert_eq!(rest, b"lifetime");
    }

    #[test]
    fn malformed_duplicate_and_nonclosed_secret_frames_have_fixed_errors() {
        let hex = "ab".repeat(32);
        for bytes in [
            b"not-json".to_vec(),
            format!(r#"{{"endpointToken":"{hex}","credentials":{{"provider-key":"one","provider-key":"two"}}}}"#).into_bytes(),
            format!(r#"{{"endpointToken":"{hex}","endpointToken":"{hex}","credentials":{{"provider-key":"synthetic"}}}}"#).into_bytes(),
            format!(r#"{{"endpointToken":"{hex}","credentials":{{}}}}"#).into_bytes(),
            format!(r#"{{"endpointToken":"{hex}","credentials":{{"provider-key":"synthetic","extra":"unselected"}}}}"#).into_bytes(),
            format!(r#"{{"endpointToken":"{hex}","credentials":{{"provider-key":"synthetic"}},"unknown":"do-not-echo"}}"#).into_bytes(),
            br#"{"endpointToken":"bad-token","credentials":{"provider-key":"synthetic"}}"#.to_vec(),
        ] {
            assert_eq!(decode(&bytes, &bindings()).err().unwrap().to_string(), "private-bootstrap-rejected");
        }
    }

    #[test]
    fn total_deadline_eof_and_size_limits_fail_without_reading_unbounded_payloads() {
        let (mut launcher, host) = UnixStream::pair().unwrap();
        launcher.write_all(&MAGIC[..3]).unwrap();
        let began = Instant::now();
        assert!(read(host.as_raw_fd(), &bindings(), Duration::from_millis(30)).is_err());
        assert!(began.elapsed() < Duration::from_secs(1));
        drop(launcher);
        assert!(read(host.as_raw_fd(), &bindings(), Duration::from_secs(1)).is_err());
        for size in [0, MAX_FRAME_BYTES as u32 + 1] {
            let (mut launcher, host) = UnixStream::pair().unwrap();
            launcher.write_all(MAGIC).unwrap();
            launcher.write_all(&size.to_be_bytes()).unwrap();
            assert!(read(host.as_raw_fd(), &bindings(), Duration::from_secs(1)).is_err());
        }
        let file = tempfile::tempfile().unwrap();
        assert!(read(file.as_raw_fd(), &bindings(), Duration::from_secs(1)).is_err());
    }
}
