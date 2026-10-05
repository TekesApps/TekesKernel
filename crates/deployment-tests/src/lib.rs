use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub const SELECTOR_BIN_ENV: &str = "TEKES_SLICE10_SELECTOR_BIN";
pub const SUPERVISOR_BIN_ENV: &str = "TEKES_SLICE10_SUPERVISOR_BIN";
pub const WORKER_BIN_ENV: &str = "TEKES_SLICE10_WORKER_BIN";
pub const HELPER_BIN_ENV: &str = "TEKES_SLICE10_HELPER_BIN";
pub const RELEASE_VERSION_ENV: &str = "TEKES_SLICE10_RELEASE_VERSION";

pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("deployment-tests lives under crates/")
        .to_path_buf()
}

pub fn run_fixture_checker() -> Output {
    Command::new("python3")
        .arg(workspace_root().join("scripts/check-deployment-fixtures.py"))
        .current_dir(workspace_root())
        .output()
        .expect("run Slice-10 fixture checker")
}

pub fn configured_binary(variable: &str) -> Option<PathBuf> {
    std::env::var_os(variable).map(PathBuf::from)
}
