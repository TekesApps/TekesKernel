//! OS adapters. No Linux installation claim until its service, credential and trust
//! authorities are implemented together. Shared recovery remains testable on Unix.
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub(crate) use macos::*;
#[cfg(not(target_os = "macos"))]
mod unsupported {
    use crate::{artifact::Artifact, *};
    pub fn supported() -> Result<()> {
        Err(Failure("platform-unavailable"))
    }
    pub fn layout() -> Result<Layout> {
        Err(Failure("platform-unavailable"))
    }
    pub fn verify_artifact(_: &Artifact) -> Result<()> {
        supported()
    }
    pub fn verify_caller(_: &Artifact) -> Result<()> {
        supported()
    }
    pub fn loaded() -> bool {
        false
    }
    pub fn bootstrap(_: &Layout) -> Result<()> {
        supported()
    }
    pub fn stop(_: &Layout) -> Result<()> {
        supported()
    }
    pub fn render(_: &Layout) -> Result<Vec<u8>> {
        Err(Failure("platform-unavailable"))
    }
    pub fn bearer_read(_: &Artifact) -> Result<Option<Vec<u8>>> {
        Err(Failure("platform-unavailable"))
    }
    pub fn bearer_ensure(_: &Artifact) -> Result<()> {
        supported()
    }
    pub fn bearer_rotate(_: &Artifact) -> Result<()> {
        supported()
    }
}
#[cfg(not(target_os = "macos"))]
pub(crate) use unsupported::*;
