use crate::{artifact::Artifact, fs, process::run, *};
use std::ffi::CStr;
const SERVICE: &str = "com.tekes.kernel.supervisor";
pub fn supported() -> Result<()> {
    Ok(())
}
pub fn layout() -> Result<Layout> {
    // SAFETY: getpwuid's record is copied before another libc account lookup.
    let home = unsafe {
        let p = libc::getpwuid(libc::geteuid());
        if p.is_null() || (*p).pw_dir.is_null() {
            return Err(Failure("platform-unavailable"));
        }
        CStr::from_ptr((*p).pw_dir)
            .to_str()
            .map_err(|_| Failure("platform-unavailable"))?
            .to_owned()
    };
    Ok(Layout::macos(&fs::absolute(&home)?))
}
fn path(p: &Path) -> Result<&str> {
    p.to_str().ok_or(Failure("unsafe-path"))
}
fn verify(p: &Path, team: &str, requirement: &str, groups: &[&str]) -> Result<()> {
    let p = path(p)?;
    require(
        run(
            "/usr/bin/codesign",
            &["--verify", "--strict", "-R", &format!("={requirement}"), p],
            30,
        )?
        .status
            == 0,
        "invalid-signature",
    )?;
    let details = run("/usr/bin/codesign", &["-d", "--verbose=4", p], 30)?;
    require(
        details.status == 0
            && String::from_utf8_lossy(&details.stderr)
                .lines()
                .any(|l| l == format!("TeamIdentifier={team}")),
        "invalid-signature",
    )?;
    if !groups.is_empty() {
        let ent = run("/usr/bin/codesign", &["-d", "--entitlements", ":-", p], 30)?;
        require(ent.status == 0, "invalid-entitlement")?;
        let text = String::from_utf8_lossy(&ent.stdout).into_owned()
            + &String::from_utf8_lossy(&ent.stderr);
        for g in groups {
            require(text.contains(g), "invalid-entitlement")?;
        }
    }
    Ok(())
}
pub fn verify_artifact(a: &Artifact) -> Result<()> {
    let exe = std::env::current_exe()?;
    require(
        fs::same_file(
            &exe,
            &a.root
                .join("installer/TekesKernelInstaller.app/Contents/MacOS/tekes-kernel-installer"),
        ),
        "invalid-artifact-executable",
    )?;
    let provider = format!("{}.com.tekes.kernel.provider-secrets", a.team);
    let groups = [a.group.as_str(), provider.as_str()];
    verify(
        &a.root.join("installer/TekesKernelInstaller.app"),
        &a.team,
        &format!("anchor apple generic and identifier {IDENTIFIER}"),
        &groups,
    )?;
    verify(
        &a.bundle().join("apps/TekesKernelSupervisor.app"),
        &a.team,
        &a.supervisor_requirement,
        &groups,
    )?;
    verify(&a.selector(), &a.team, &a.selector_requirement, &[])?;
    for name in ["tekes-worker", "tekes-helper", "tekes-workspace-service"] {
        verify(
            &a.bundle().join("bin").join(name),
            &a.team,
            &format!(
                "anchor apple generic and certificate leaf[subject.OU] = \"{}\"",
                a.team
            ),
            &[],
        )?;
    }
    Ok(())
}
pub fn verify_caller(a: &Artifact) -> Result<()> {
    let mut buffer = [0u8; 4096];
    let n = unsafe {
        libc::proc_pidpath(
            libc::getppid(),
            buffer.as_mut_ptr().cast(),
            buffer.len() as u32,
        )
    };
    require(n > 0, "invalid-client")?;
    let exe = CStr::from_bytes_until_nul(&buffer)
        .map_err(|_| Failure("invalid-client"))?
        .to_str()
        .map_err(|_| Failure("invalid-client"))?;
    let (app, _) = exe
        .rsplit_once("/Contents/MacOS/")
        .ok_or(Failure("invalid-client"))?;
    verify(Path::new(app), &a.team, &a.client_requirement, &[&a.group])
}
fn domain() -> String {
    format!("gui/{}", fs::uid())
}
fn target() -> String {
    format!("{}/{SERVICE}", domain())
}
pub fn loaded() -> bool {
    run("/bin/launchctl", &["print", &target()], 30).is_ok_and(|o| o.status == 0)
}
pub fn bootstrap(l: &Layout) -> Result<()> {
    if !loaded() {
        require(
            run(
                "/bin/launchctl",
                &["bootstrap", &domain(), path(&l.service_file)?],
                30,
            )?
            .status
                == 0,
            "launch-agent-start-failed",
        )?;
    }
    // Never use -k: an already-ready selector must keep its process identity.
    require(
        run("/bin/launchctl", &["kickstart", &target()], 30)?.status == 0,
        "launch-agent-start-failed",
    )
}
pub fn stop(l: &Layout) -> Result<()> {
    if run("/bin/launchctl", &["bootout", &target()], 30)?.status != 0 {
        let fallback = run(
            "/bin/launchctl",
            &["bootout", &domain(), path(&l.service_file)?],
            30,
        )?;
        if fallback.status != 0 && fs::exists(&l.service_file)? {
            require(!loaded(), "launch-agent-stop-failed")?;
        }
    }
    Ok(())
}
pub fn render(l: &Layout) -> Result<Vec<u8>> {
    fn esc(p: &Path) -> Result<String> {
        Ok(path(p)?
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;"))
    }
    let selector = esc(&l.kernel.join("selector/bin/tekes-selector"))?;
    let root = esc(&l.kernel)?;
    let storage = esc(&l.threads)?;
    let stdout = esc(&l.logs.join("stdout.log"))?;
    let stderr = esc(&l.logs.join("stderr.log"))?;
    Ok(format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\"><dict>\n<key>Label</key><string>{SERVICE}</string>\n<key>ProgramArguments</key><array><string>{selector}</string><string>--install-root</string><string>{root}</string><string>serve</string><string>--storage-root</string><string>{storage}</string><string>--listen</string><string>127.0.0.1:7347</string></array>\n<key>KeepAlive</key><true/><key>RunAtLoad</key><true/><key>ProcessType</key><string>Background</string>\n<key>StandardOutPath</key><string>{stdout}</string><key>StandardErrorPath</key><string>{stderr}</string>\n</dict></plist>").into_bytes())
}
/// The owning launcher supplies the endpoint token; the installer never persists it.
pub fn bearer_read(_artifact: &Artifact) -> Result<Option<Vec<u8>>> {
    let value = match std::env::var("TEKES_KERNEL_ENDPOINT_TOKEN") {
        Ok(value) => zeroize::Zeroizing::new(value),
        Err(std::env::VarError::NotPresent) => return Ok(None),
        Err(_) => return Err(Failure("endpoint-credential-unavailable")),
    };
    require(
        value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit()),
        "endpoint-credential-unavailable",
    )?;
    let mut token = Vec::with_capacity(32);
    for pair in value.as_bytes().chunks_exact(2) {
        token.push(
            u8::from_str_radix(
                std::str::from_utf8(pair)
                    .map_err(|_| Failure("endpoint-credential-unavailable"))?,
                16,
            )
            .map_err(|_| Failure("endpoint-credential-unavailable"))?,
        );
    }
    Ok(Some(token))
}

pub fn bearer_ensure(artifact: &Artifact) -> Result<()> {
    require(
        bearer_read(artifact)?.is_some(),
        "endpoint-credential-unavailable",
    )
}

pub fn bearer_rotate(_artifact: &Artifact) -> Result<()> {
    Err(Failure("credential-owned-by-application"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn launch_agent_preserves_existing_bytes() {
        let l = Layout::macos(Path::new("/Users/test"));
        let rendered = String::from_utf8(render(&l).unwrap()).unwrap();
        assert_eq!(rendered, include_str!("../../fixtures/launch-agent.plist"));
    }
}
