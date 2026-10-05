//! Signed product installation orchestration. Platform authority lives in `platform`;
//! durable journals, publication and recovery are independent of service-manager APIs.
mod artifact;
mod fs;
mod migration;
mod platform;
mod process;
mod transaction;

use serde_json::{Value, json};
use std::path::{Path, PathBuf};

pub const PROTOCOL: &str = "tekes-kernel-product-installer";
/// The handshake string a product assembled before the rename still sends.
pub const LEGACY_PROTOCOL: &str = "tekes-kernel-product-installer-v1";
pub const ORIGIN: &str = "http://127.0.0.1:7347";
pub const IDENTIFIER: &str = "com.tekes.kernel.installer";
pub const CONTRACT: &[u8] =
    include_bytes!("../../../packaging/macos/product-installer/contract.canonical.json");
pub type Result<T> = std::result::Result<T, Failure>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Failure(pub &'static str);
impl From<std::io::Error> for Failure {
    fn from(_: std::io::Error) -> Self {
        Self("io")
    }
}
impl From<serde_json::Error> for Failure {
    fn from(_: serde_json::Error) -> Self {
        Self("invalid-json")
    }
}
fn require(ok: bool, code: &'static str) -> Result<()> {
    if ok { Ok(()) } else { Err(Failure(code)) }
}
fn keys(v: &Value, names: &[&str]) -> bool {
    v.as_object()
        .is_some_and(|o| o.len() == names.len() && names.iter().all(|n| o.contains_key(*n)))
}
fn string<'a>(v: &'a Value, key: &str, code: &'static str) -> Result<&'a str> {
    v[key].as_str().ok_or(Failure(code))
}
fn canonical(v: &Value) -> Result<Vec<u8>> {
    let mut b = serde_json::to_vec(v)?;
    b.push(b'\n');
    Ok(b)
}
fn identifier(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-._".contains(&b))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Status,
    Install,
    Ensure,
    Enable,
    Disable,
}
impl Operation {
    pub fn name(self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Install => "install-or-upgrade",
            Self::Ensure => "ensure-running",
            Self::Enable => "enable",
            Self::Disable => "disable",
        }
    }
}
#[derive(Debug)]
pub struct Arguments {
    pub operation: Operation,
    pub root: PathBuf,
}
impl Arguments {
    pub fn parse(args: &[String]) -> Result<Option<Self>> {
        if args == ["--describe-contract"] {
            return Ok(None);
        }
        let err = "invalid-arguments";
        require(
            args.len() >= 8
                && args[0] == "--protocol"
                && (args[1] == PROTOCOL || args[1] == LEGACY_PROTOCOL)
                && args[2] == "--operation"
                && args[4] == "--artifact-root"
                && args[6] == "--origin"
                && args[7] == ORIGIN,
            err,
        )?;
        let operation = match args[3].as_str() {
            "status" => Operation::Status,
            "install-or-upgrade" => Operation::Install,
            "ensure-running" => Operation::Ensure,
            "enable" => Operation::Enable,
            "disable" => Operation::Disable,
            _ => return Err(Failure(err)),
        };
        require(args.len() == 8, err)?;
        Ok(Some(Self {
            operation,
            root: fs::absolute(&args[5])?,
        }))
    }
}
#[derive(Debug, Clone)]
struct Layout {
    home: PathBuf,
    base: PathBuf,
    data: PathBuf,
    kernel: PathBuf,
    installer: PathBuf,
    threads: PathBuf,
    logs: PathBuf,
    service_file: PathBuf,
}
impl Layout {
    // This layout is the existing macOS on-disk contract, not a portable path default.
    #[cfg(any(target_os = "macos", test))]
    fn macos(home: &Path) -> Self {
        let base = home.join("Library/Application Support/Tekes");
        let data = home.join(".agents");
        Self {
            home: home.into(),
            kernel: base.join("Kernel"),
            installer: base.join("Installer"),
            threads: data.join("threads"),
            logs: data.join("logs/kernel"),
            service_file: home.join("Library/LaunchAgents/com.tekes.kernel.supervisor.plist"),
            base,
            data,
        }
    }
}

pub fn execute(args: Arguments) -> Result<Value> {
    platform::supported()?;
    let artifact = artifact::Artifact::load(&args.root)?;
    platform::verify_caller(&artifact)?;
    let layout = platform::layout()?;
    transaction::execute(args.operation, &layout, &artifact)
}
pub fn error_reply(error: Failure) -> Vec<u8> {
    canonical(&json!({"error":{"code":error.0}})).expect("literal error object")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn command_contract_and_argument_errors() {
        let contract: Value = serde_json::from_slice(CONTRACT).unwrap();
        let mut operations = vec![];
        for op in [
            Operation::Status,
            Operation::Install,
            Operation::Ensure,
            Operation::Enable,
            Operation::Disable,
        ] {
            operations.push(op.name());
            let mut args = vec![
                "--protocol",
                PROTOCOL,
                "--operation",
                op.name(),
                "--artifact-root",
                "/valid/product",
                "--origin",
                ORIGIN,
            ]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
            assert_eq!(Arguments::parse(&args).unwrap().unwrap().operation, op);
            args.push("extra".into());
            assert_eq!(
                Arguments::parse(&args).unwrap_err(),
                Failure("invalid-arguments")
            );
        }
        operations.sort();
        assert_eq!(contract["operations"], json!(operations));
        assert!(
            Arguments::parse(&["--describe-contract".into()])
                .unwrap()
                .is_none()
        );
        for path in ["relative", "/a/../b", "/a/./b", "/a//b", "/a/"] {
            assert_eq!(fs::absolute(path), Err(Failure("unsafe-path")));
        }
    }
}
