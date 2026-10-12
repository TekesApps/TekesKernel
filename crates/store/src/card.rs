//! `thread-card.json`: a rebuildable, per-thread cache of what a full fold of the ledger
//! produced (G-BOOT-1, ADDENDUM-02 M4/S7, ADDENDUM-03 C2).
//!
//! A card is **never truth**. It lets a reader skip decoding a ledger it has already
//! verified. Invariants:
//!
//! * **C1** a card only claims bytes `[0, valid_bytes)` of a line that were verified by a full
//!   fold *and* made durable with `F_FULLFSYNC` before the card was renamed into place.
//! * **Content identity**: freshness is decided from content (length, first line, final line
//!   and sampled windows), never from inode, device or mtime, so `cp -c` clones, backup
//!   restores and archive moves of the folder stay fresh.
//! * **No locks**: this module never takes a ledger or directory lock. Writers read through a
//!   pinned descriptor and sync that same descriptor; the only file it creates is the card.
//!
//! The module is deliberately independent of `endpoint` and `supervisor`: callers fold the
//! ledger (they already have the decoded projection) and pass in the derived facts.
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use schema::LifecycleFacts;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::StoreError;
use crate::platform::FullSync;

pub const CARD_FILE: &str = "thread-card.json";
pub const CARD_VERSION: u64 = 1;
const JOURNAL_FILE: &str = "endpoint.jsonl";
/// Windows hashed per file in addition to the first and last line.
const SAMPLE_COUNT: u64 = 8;
const SAMPLE_LEN: u64 = 256;
/// Largest card accepted on read; a genuine card is a few KB.
const MAX_CARD_BYTES: u64 = 4 * 1024 * 1024;

static TEMP_ORDINAL: AtomicU64 = AtomicU64::new(0);

/// A hashed window of a file.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sample {
    pub off: u64,
    pub len: u64,
    pub sha256: String,
}

/// One hold (approval or question) of a ledger line, resolved or not. The request id is a pure
/// hash of `(session, line, kind, hold_seq)`, so only these fields are stored.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hold {
    pub hold_seq: u64,
    /// `approval` or `question`.
    pub kind: String,
    /// `(seq, outcome)` of the resolution when the ledger already carries one.
    pub resolved: Option<HoldResolution>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HoldResolution {
    pub seq: u64,
    pub outcome: String,
}

/// What the card claims about one ledger file.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LineCard {
    pub file: String,
    pub valid_bytes: u64,
    pub last_seq: u64,
    pub genesis_len: u64,
    pub genesis_sha256: String,
    pub tail_len: u64,
    pub tail_sha256: String,
    pub samples: Vec<Sample>,
    /// `genesis.thread` of this line.
    pub line: String,
    pub parent_file: Option<String>,
    pub facts: LifecycleFacts,
    pub holds: Vec<Hold>,
}

/// The List row source (main line only). Reproduces `NativeEndpoint::list_sessions`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardSummary {
    pub workspace: String,
    pub folder_binding: Option<String>,
    pub created: String,
    pub updated_ms: u64,
    pub title: Option<String>,
    pub identity_profile: Option<String>,
    pub ephemeral: bool,
    pub blank: bool,
}

/// Identity of the endpoint journal at card time (the journal is a projection, not truth).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JournalCard {
    pub len: u64,
    pub rows: u64,
    pub last_endpoint_seq: i64,
    pub last_kernel_seq: u64,
    pub head_sha256: String,
    pub tail_sha256: String,
    pub tail_len: u64,
    pub samples: Vec<Sample>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThreadCard {
    pub v: u64,
    pub kernel: String,
    pub thread: String,
    pub written: String,
    pub lines: Vec<LineCard>,
    pub summary: CardSummary,
    pub journal: Option<JournalCard>,
    /// SHA-256 of the canonical body with this field set to `""` (accident/bit-rot guard).
    pub body_sha256: String,
}

impl ThreadCard {
    fn body_digest(&self) -> Result<String, StoreError> {
        let mut blank = self.clone();
        blank.body_sha256 = String::new();
        let bytes = serde_json_canonicalizer::to_vec(&blank)
            .map_err(|error| StoreError::Corruption(error.to_string()))?;
        Ok(hex(&Sha256::digest(bytes)))
    }

    /// Canonical bytes (one line, LF-terminated) with the digest filled in.
    pub fn encode(&self) -> Result<Vec<u8>, StoreError> {
        let mut sealed = self.clone();
        sealed.body_sha256 = sealed.body_digest()?;
        let mut bytes = serde_json_canonicalizer::to_vec(&sealed)
            .map_err(|error| StoreError::Corruption(error.to_string()))?;
        bytes.push(b'\n');
        Ok(bytes)
    }

    /// `Ok(None)` for anything that is not a valid version-1 card: bad JSON, unknown fields,
    /// wrong version, digest mismatch. A damaged card is simply absent.
    #[must_use]
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let line = bytes.strip_suffix(b"\n")?;
        if line.contains(&b'\n') {
            return None;
        }
        let card: Self = serde_json::from_slice(line).ok()?;
        if card.v != CARD_VERSION || card.body_digest().ok()? != card.body_sha256 {
            return None;
        }
        Some(card)
    }
}

/// Reads `<folder>/thread-card.json`; a missing, oversized or invalid card is `None`.
#[must_use]
pub fn read_card(folder: &Path) -> Option<ThreadCard> {
    let file = File::open(folder.join(CARD_FILE)).ok()?;
    if file.metadata().ok()?.len() > MAX_CARD_BYTES {
        return None;
    }
    let mut bytes = Vec::new();
    io::Read::read_to_end(&mut io::Read::take(&file, MAX_CARD_BYTES), &mut bytes).ok()?;
    ThreadCard::decode(&bytes)
}

// ---------------------------------------------------------------------------------------------
// identity hashing

fn hex(digest: &[u8]) -> String {
    use std::fmt::Write as _;
    digest
        .iter()
        .fold(String::with_capacity(64), |mut out, byte| {
            let _ = write!(out, "{byte:02x}");
            out
        })
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

/// Offsets of the sampled windows of a `len`-byte prefix (deterministic, deduplicated).
fn sample_offsets(len: u64) -> Vec<(u64, u64)> {
    let mut windows: Vec<(u64, u64)> = Vec::new();
    for index in 0..SAMPLE_COUNT {
        let width = SAMPLE_LEN.min(len);
        let off = (len.saturating_sub(width) / SAMPLE_COUNT.max(1)).saturating_mul(index);
        let off = off.min(len.saturating_sub(width));
        if width > 0 && !windows.contains(&(off, width)) {
            windows.push((off, width));
        }
    }
    windows
}

/// Where the first and last lines of `bytes[..len]` lie: `(genesis_len, tail_len)`, both
/// including the terminating LF. `bytes[..len]` must end with LF.
fn line_extents(bytes: &[u8]) -> Option<(u64, u64)> {
    let first = bytes.iter().position(|b| *b == b'\n')?;
    let body = bytes.strip_suffix(b"\n")?;
    let last_start = body.iter().rposition(|b| *b == b'\n').map_or(0, |p| p + 1);
    Some((first as u64 + 1, (bytes.len() - last_start) as u64))
}

fn samples_of(bytes: &[u8]) -> Vec<Sample> {
    sample_offsets(bytes.len() as u64)
        .into_iter()
        .map(|(off, len)| Sample {
            off,
            len,
            sha256: sha256_hex(&bytes[off as usize..(off + len) as usize]),
        })
        .collect()
}

fn pread(file: &File, off: u64, len: u64) -> io::Result<Vec<u8>> {
    let mut buffer = vec![0_u8; usize::try_from(len).map_err(io::Error::other)?];
    file.read_exact_at(&mut buffer, off)?;
    Ok(buffer)
}

/// Identity fields computed from the exact bytes of a verified prefix (ends with LF).
#[derive(Clone, Debug, Eq, PartialEq)]
struct Identity {
    genesis_len: u64,
    genesis_sha256: String,
    tail_len: u64,
    tail_sha256: String,
    samples: Vec<Sample>,
}

impl Identity {
    fn of(prefix: &[u8]) -> Option<Self> {
        let (genesis_len, tail_len) = line_extents(prefix)?;
        Some(Self {
            genesis_len,
            genesis_sha256: sha256_hex(&prefix[..genesis_len as usize]),
            tail_len,
            tail_sha256: sha256_hex(&prefix[prefix.len() - tail_len as usize..]),
            samples: samples_of(prefix),
        })
    }
}

// ---------------------------------------------------------------------------------------------
// freshness

/// Verdict for one ledger file named by the card.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineVerdict {
    /// Byte-for-byte what the card verified; nothing appended since.
    Fresh,
    /// The first `valid_bytes` still match; more bytes were appended after the card.
    Ahead,
    /// Shorter, replaced, edited, or missing: the card says nothing trustworthy about it.
    Stale,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JournalVerdict {
    Fresh,
    Ahead,
    Stale,
    /// The card carries no journal identity.
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Classified {
    pub lines: Vec<(String, LineVerdict)>,
    /// `*.jsonl` ledger files in the folder that the card does not list (appeared since).
    pub unlisted_lines: Vec<String>,
    pub journal: JournalVerdict,
}

/// The overall trust level of a card.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Freshness {
    Fresh,
    Ahead,
    /// Treat the card as absent for this thread.
    Cold,
}

impl Classified {
    #[must_use]
    pub fn freshness(&self) -> Freshness {
        if self.lines.iter().any(|(_, v)| *v == LineVerdict::Stale) {
            Freshness::Cold
        } else if self.lines.iter().all(|(_, v)| *v == LineVerdict::Fresh)
            && self.unlisted_lines.is_empty()
        {
            Freshness::Fresh
        } else {
            Freshness::Ahead
        }
    }
}

fn open_nofollow(path: &Path) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;
    std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
}

fn verify_prefix(
    file: &File,
    valid_bytes: u64,
    genesis_len: u64,
    genesis_sha: &str,
    tail_len: u64,
    tail_sha: &str,
    samples: &[Sample],
) -> io::Result<LineVerdict> {
    let len = file.metadata()?.len();
    if len < valid_bytes || valid_bytes == 0 || tail_len > valid_bytes || genesis_len > valid_bytes
    {
        return Ok(LineVerdict::Stale);
    }
    if sha256_hex(&pread(file, 0, genesis_len)?) != genesis_sha
        || sha256_hex(&pread(file, valid_bytes - tail_len, tail_len)?) != tail_sha
    {
        return Ok(LineVerdict::Stale);
    }
    for sample in samples {
        if sample.off.saturating_add(sample.len) > valid_bytes
            || sha256_hex(&pread(file, sample.off, sample.len)?) != sample.sha256
        {
            return Ok(LineVerdict::Stale);
        }
    }
    Ok(if len == valid_bytes {
        LineVerdict::Fresh
    } else {
        LineVerdict::Ahead
    })
}

/// Classifies `card` against the files in `folder` without decoding any event. The card must
/// belong to the folder (`card.thread == folder name`), otherwise every line is Stale.
pub fn classify(folder: &Path, card: &ThreadCard) -> io::Result<Classified> {
    let belongs = folder
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == card.thread);
    let mut lines = Vec::with_capacity(card.lines.len());
    for line in &card.lines {
        let verdict = if !belongs || line.file.contains('/') || line.file == JOURNAL_FILE {
            LineVerdict::Stale
        } else {
            match open_nofollow(&folder.join(&line.file)) {
                Ok(file) => verify_prefix(
                    &file,
                    line.valid_bytes,
                    line.genesis_len,
                    &line.genesis_sha256,
                    line.tail_len,
                    &line.tail_sha256,
                    &line.samples,
                )?,
                Err(error) if error.kind() == io::ErrorKind::NotFound => LineVerdict::Stale,
                Err(error) => return Err(error),
            }
        };
        lines.push((line.file.clone(), verdict));
    }
    let mut unlisted_lines = Vec::new();
    for entry in std::fs::read_dir(folder)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".jsonl")
            && name != JOURNAL_FILE
            && entry.file_type()?.is_file()
            && !card.lines.iter().any(|line| line.file == name)
        {
            unlisted_lines.push(name);
        }
    }
    unlisted_lines.sort();
    let journal = match &card.journal {
        None => JournalVerdict::Unknown,
        Some(journal) if journal.len == 0 => match std::fs::metadata(folder.join(JOURNAL_FILE)) {
            Ok(metadata) if metadata.len() == 0 => JournalVerdict::Fresh,
            Ok(_) => JournalVerdict::Ahead,
            Err(error) if error.kind() == io::ErrorKind::NotFound => JournalVerdict::Fresh,
            Err(error) => return Err(error),
        },
        Some(journal) => match open_nofollow(&folder.join(JOURNAL_FILE)) {
            Ok(file) => match verify_prefix(
                &file,
                journal.len,
                journal.head_len(),
                &journal.head_sha256,
                journal.tail_len,
                &journal.tail_sha256,
                &journal.samples,
            )? {
                LineVerdict::Fresh => JournalVerdict::Fresh,
                LineVerdict::Ahead => JournalVerdict::Ahead,
                LineVerdict::Stale => JournalVerdict::Stale,
            },
            Err(error) if error.kind() == io::ErrorKind::NotFound && journal.len == 0 => {
                JournalVerdict::Fresh
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => JournalVerdict::Stale,
            Err(error) => return Err(error),
        },
    };
    Ok(Classified {
        lines,
        unlisted_lines,
        journal,
    })
}

impl JournalCard {
    /// Length of the hashed head window: journal records can be large, so the head hash covers
    /// the first `min(256, len)` bytes rather than a whole record.
    fn head_len(&self) -> u64 {
        SAMPLE_LEN.min(self.len)
    }
}

// ---------------------------------------------------------------------------------------------
// writer

/// What the caller derived from a fold of one ledger file.
pub struct LineInput<'a> {
    pub file: &'a str,
    /// The exact bytes `[0, valid_bytes)` that were folded (must end with LF).
    pub verified_prefix: &'a [u8],
    pub last_seq: u64,
    pub line: String,
    pub parent_file: Option<String>,
    pub facts: LifecycleFacts,
    pub holds: Vec<Hold>,
}

/// Journal shape at card time, supplied by the journal's owner; its identity hashes are taken
/// from the file here.
pub struct JournalInput {
    pub len: u64,
    pub rows: u64,
    pub last_endpoint_seq: i64,
    pub last_kernel_seq: u64,
}

pub struct CardInput<'a> {
    pub thread: &'a str,
    pub kernel: &'a str,
    pub written: String,
    pub lines: Vec<LineInput<'a>>,
    pub summary: CardSummary,
    pub journal: Option<JournalInput>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WriteOutcome {
    Written,
    /// The folder or a file vanished (archive, discard) or no longer holds the folded bytes.
    /// Nothing was written; the previous card, if any, stays.
    Skipped(SkipReason),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkipReason {
    FolderGone,
    FileGone,
    FileChanged,
}

fn errno_is_gone(error: &io::Error) -> bool {
    matches!(
        error.raw_os_error(),
        Some(libc::ENOENT | libc::ENOTDIR | libc::ESTALE)
    )
}

fn openat(dir: &File, name: &str, flags: libc::c_int, mode: libc::mode_t) -> io::Result<File> {
    let name = CString::new(name).map_err(io::Error::other)?;
    loop {
        // SAFETY: `dir` is a live descriptor, `name` is NUL-terminated, and the returned
        // descriptor (if any) is immediately owned by a `File`.
        let fd = unsafe {
            libc::openat(
                dir.as_raw_fd(),
                name.as_ptr(),
                flags | libc::O_CLOEXEC | libc::O_NOFOLLOW,
                libc::c_uint::from(mode),
            )
        };
        if fd >= 0 {
            // SAFETY: `fd` was just returned by `openat` and is not owned elsewhere.
            return Ok(unsafe { File::from_raw_fd(fd) });
        }
        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error);
        }
    }
}

fn renameat(dir: &File, from: &str, to: &str) -> io::Result<()> {
    let from = CString::new(from).map_err(io::Error::other)?;
    let to = CString::new(to).map_err(io::Error::other)?;
    // SAFETY: `dir` is a live descriptor and both names are NUL-terminated.
    let result =
        unsafe { libc::renameat(dir.as_raw_fd(), from.as_ptr(), dir.as_raw_fd(), to.as_ptr()) };
    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

fn unlinkat(dir: &File, name: &str) {
    if let Ok(name) = CString::new(name) {
        // SAFETY: `dir` is a live descriptor and `name` is NUL-terminated. Best effort.
        let _ = unsafe { libc::unlinkat(dir.as_raw_fd(), name.as_ptr(), 0) };
    }
}

/// Writes `<folder>/thread-card.json` for the folded bytes in `input`.
///
/// Per ledger file: open once (no lock, no follow), check it still holds exactly the folded
/// bytes at the first line, last line and every sampled window, `F_FULLFSYNC` **that
/// descriptor** (C1), then publish the card with a temp file and `renameat` relative to a
/// directory descriptor opened once. The card file itself is not fsynced: losing it only
/// loses the cache. A vanished folder or file yields `Skipped`, never an error.
pub fn write_card(folder: &Path, input: &CardInput<'_>) -> Result<WriteOutcome, StoreError> {
    let dir = match File::open(folder) {
        Ok(dir) => dir,
        Err(error) if errno_is_gone(&error) => {
            return Ok(WriteOutcome::Skipped(SkipReason::FolderGone));
        }
        Err(error) => return Err(error.into()),
    };
    let mut lines = Vec::with_capacity(input.lines.len());
    for line in &input.lines {
        let Some(identity) = Identity::of(line.verified_prefix) else {
            return Ok(WriteOutcome::Skipped(SkipReason::FileChanged));
        };
        let file = match openat(&dir, line.file, libc::O_RDONLY, 0) {
            Ok(file) => file,
            Err(error) if errno_is_gone(&error) => {
                return Ok(WriteOutcome::Skipped(SkipReason::FileGone));
            }
            Err(error) => return Err(error.into()),
        };
        let valid_bytes = line.verified_prefix.len() as u64;
        // The pinned file must still hold what was folded, at the points the card hashes.
        match verify_prefix(
            &file,
            valid_bytes,
            identity.genesis_len,
            &identity.genesis_sha256,
            identity.tail_len,
            &identity.tail_sha256,
            &identity.samples,
        ) {
            Ok(LineVerdict::Fresh | LineVerdict::Ahead) => {}
            Ok(LineVerdict::Stale) => return Ok(WriteOutcome::Skipped(SkipReason::FileChanged)),
            Err(error) if errno_is_gone(&error) => {
                return Ok(WriteOutcome::Skipped(SkipReason::FileGone));
            }
            Err(error) => return Err(error.into()),
        }
        FullSync::full_sync(&file)?;
        lines.push(LineCard {
            file: line.file.to_owned(),
            valid_bytes,
            last_seq: line.last_seq,
            genesis_len: identity.genesis_len,
            genesis_sha256: identity.genesis_sha256,
            tail_len: identity.tail_len,
            tail_sha256: identity.tail_sha256,
            samples: identity.samples,
            line: line.line.clone(),
            parent_file: line.parent_file.clone(),
            facts: line.facts.clone(),
            holds: line.holds.clone(),
        });
    }
    let journal = match &input.journal {
        None => None,
        Some(journal) => match openat(&dir, JOURNAL_FILE, libc::O_RDONLY, 0) {
            Ok(file) => {
                if file.metadata()?.len() < journal.len {
                    return Ok(WriteOutcome::Skipped(SkipReason::FileChanged));
                }
                let head_len = SAMPLE_LEN.min(journal.len);
                let tail = journal_tail_len(&file, journal.len)?;
                Some(JournalCard {
                    len: journal.len,
                    rows: journal.rows,
                    last_endpoint_seq: journal.last_endpoint_seq,
                    last_kernel_seq: journal.last_kernel_seq,
                    head_sha256: sha256_hex(&pread(&file, 0, head_len)?),
                    tail_sha256: sha256_hex(&pread(&file, journal.len - tail, tail)?),
                    tail_len: tail,
                    samples: journal_samples(&file, journal.len)?,
                })
            }
            Err(error) if errno_is_gone(&error) && journal.len == 0 => None,
            Err(error) if errno_is_gone(&error) => {
                return Ok(WriteOutcome::Skipped(SkipReason::FileGone));
            }
            Err(error) => return Err(error.into()),
        },
    };
    let card = ThreadCard {
        v: CARD_VERSION,
        kernel: input.kernel.to_owned(),
        thread: input.thread.to_owned(),
        written: input.written.clone(),
        lines,
        summary: input.summary.clone(),
        journal,
        body_sha256: String::new(),
    };
    let bytes = card.encode()?;
    let temp = format!(
        ".{CARD_FILE}.{}.{}.tmp",
        std::process::id(),
        TEMP_ORDINAL.fetch_add(1, Ordering::Relaxed)
    );
    let published = (|| -> io::Result<()> {
        let file = openat(
            &dir,
            &temp,
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL,
            0o600,
        )?;
        file.write_all_at(&bytes, 0)?;
        drop(file);
        renameat(&dir, &temp, CARD_FILE)
    })();
    match published {
        Ok(()) => Ok(WriteOutcome::Written),
        Err(error) => {
            unlinkat(&dir, &temp);
            if errno_is_gone(&error) {
                Ok(WriteOutcome::Skipped(SkipReason::FolderGone))
            } else {
                Err(error.into())
            }
        }
    }
}

/// Journal rows are canonical lines; the final row's length is found by scanning back from
/// `len` for the previous LF within a bounded window.
fn journal_tail_len(file: &File, len: u64) -> io::Result<u64> {
    if len == 0 {
        return Ok(0);
    }
    let window = len.min(1 << 20);
    let bytes = pread(file, len - window, window)?;
    let body = bytes.strip_suffix(b"\n").unwrap_or(&bytes);
    Ok(match body.iter().rposition(|b| *b == b'\n') {
        Some(position) => (bytes.len() - (position + 1)) as u64,
        None => window,
    })
}

fn journal_samples(file: &File, len: u64) -> io::Result<Vec<Sample>> {
    sample_offsets(len)
        .into_iter()
        .map(|(off, width)| {
            Ok(Sample {
                off,
                len: width,
                sha256: sha256_hex(&pread(file, off, width)?),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests;
