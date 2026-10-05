use crate::{artifact::Artifact, fs, process::run, *};
use std::{
    thread,
    time::{Duration, Instant},
};
fn p(p: &Path) -> Result<&str> {
    p.to_str().ok_or(Failure("unsafe-path"))
}
const PHASES: &[&str] = &[
    "prepared",
    "identity-published",
    "credential-published",
    "bundles-published",
    "selection-published",
    "plist-published",
    "service-started",
    "closed",
];
fn request(a: &Artifact) -> Result<(String, String)> {
    let b = canonical(
        &json!({"artifact_root":a.root,"format":1,"operation":"install-or-upgrade","origin":ORIGIN,"product_manifest_sha256":a.product_sha,"protocol":PROTOCOL,"version":a.version}),
    )?;
    let digest = fs::sha(&b);
    let id = format!("product-install-{}", &digest[..16]);
    Ok((digest, id))
}
fn write_operation(l: &Layout, a: &Artifact, phase: &str) -> Result<()> {
    let (digest, id) = request(a)?;
    let mut v = json!({"format":1,"install_identity_sha256":a.identity_sha,"op_id":id,"phase":phase,"request_sha256":digest,"type":"install"});
    if phase == "closed" {
        v["response"] = json!({"format":1,"operation":"install","service":"running"});
    }
    fs::durable(&l.installer.join("operation.json"), &canonical(&v)?, 0o600)
}
fn installed(l: &Layout) -> Option<String> {
    fs::object(&l.kernel.join("selector/current.json")).ok()?.0["selection"]["version"]
        .as_str()
        .map(str::to_owned)
}
fn health() -> Option<Value> {
    let out = run(
        "/usr/bin/curl",
        &[
            "--silent",
            "--show-error",
            "--max-time",
            "2",
            &format!("{ORIGIN}/health/ready"),
        ],
        4,
    )
    .ok()?;
    if !out.success() {
        return None;
    }
    let v: Value = serde_json::from_slice(&out.stdout).ok()?;
    if keys(&v, &["build", "generation", "ready"])
        && v["ready"].as_bool() == Some(true)
        && v["build"].is_string()
        && v["generation"].as_u64().is_some_and(|g| g >= 1)
    {
        Some(v)
    } else {
        None
    }
}
fn snapshot(op: Operation, l: &Layout, a: &Artifact) -> Value {
    let installed = installed(l);
    let loaded = platform::loaded();
    let ready = if loaded { health() } else { None };
    let service = if installed.is_none() {
        "not-installed"
    } else if ready.is_some() {
        "ready"
    } else if loaded {
        "running"
    } else {
        "stopped"
    };
    json!({"bundled_version":a.version,"format":1,"installed_version":installed,"operation":op.name(),"readiness":ready,"service":service})
}
fn shutdown_complete(loaded: bool, ready: bool, service_lock: bool, child_lock: bool) -> bool {
    !loaded && !ready && service_lock && child_lock
}
fn bootout(l: &Layout) -> Result<()> {
    platform::stop(l)?;
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        if shutdown_complete(
            platform::loaded(),
            health().is_some(),
            fs::lock_available(&l.kernel.join("selector/.service-lock"))?,
            fs::lock_available(&l.threads.join(".root-lock"))?,
        ) {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(200));
    }
    Err(Failure("launch-agent-stop-failed"))
}
fn begin(l: &Layout, a: &Artifact) -> Result<String> {
    let path = l.installer.join("operation.json");
    let (digest, id) = request(a)?;
    if fs::exists(&path)? {
        let (v, bytes) = fs::object(&path)?;
        let e = "invalid-install";
        require(
            (keys(
                &v,
                &[
                    "format",
                    "install_identity_sha256",
                    "op_id",
                    "phase",
                    "request_sha256",
                    "type",
                ],
            ) || keys(
                &v,
                &[
                    "format",
                    "install_identity_sha256",
                    "op_id",
                    "phase",
                    "request_sha256",
                    "response",
                    "type",
                ],
            )) && v["format"].as_u64() == Some(1)
                && v["install_identity_sha256"] == a.identity_sha
                && v["type"] == "install",
            e,
        )?;
        let phase = string(&v, "phase", e)?;
        require(PHASES.contains(&phase), e)?;
        let prior_digest = string(&v, "request_sha256", e)?;
        let prior_id = string(&v, "op_id", e)?;
        let closed = phase == "closed";
        require(closed == v.get("response").is_some(), e)?;
        let matches = prior_digest == digest && prior_id == id;
        if !closed && !matches {
            bootout(l)?;
            fs::durable(
                &l.installer.join(format!(
                    "operation.superseded-{}.json",
                    &fs::sha(&bytes)[..16]
                )),
                &bytes,
                0o600,
            )?;
            write_operation(l, a, "prepared")?;
            return Ok("prepared".into());
        }
        if matches {
            if closed {
                let r = &v["response"];
                require(
                    keys(r, &["format", "operation", "service"])
                        && r["format"].as_u64() == Some(1)
                        && r["operation"] == "install"
                        && r["service"] == "running",
                    e,
                )?;
                if installed(l).as_deref() != Some(&a.version) {
                    write_operation(l, a, "prepared")?;
                    return Ok("prepared".into());
                }
            }
            return Ok(phase.into());
        }
    }
    write_operation(l, a, "prepared")?;
    Ok("prepared".into())
}
fn initialize(l: &Layout, a: &Artifact) -> Result<()> {
    crate::migration::migrate(l)?;
    for path in [
        &l.base,
        &l.kernel,
        &l.data,
        &l.threads,
        &l.logs,
        &l.installer,
    ] {
        fs::directory(path)?;
    }
    for sub in [
        "bundles",
        "selector",
        "selector/bin",
        "selector/operations",
        "selector/observations",
    ] {
        fs::directory(&l.kernel.join(sub))?;
    }
    for sub in [
        "settings",
        "workspaces",
        "jobs",
        "config",
        "cache",
        "runtime",
        "runtime/composer",
        "skills",
        "client",
        "not-in-project",
    ] {
        fs::directory(&l.data.join(sub))?;
    }
    for f in [
        l.kernel.join("selector/.lock"),
        l.kernel.join("selector/.service-lock"),
        l.threads.join(".root-lock"),
    ] {
        if !fs::exists(&f)? {
            fs::durable(&f, &[], 0o600)?;
        }
    }
    let path = l.installer.join("install-identity.json");
    if fs::exists(&path)? {
        require(fs::regular(&path)? == a.identity, "invalid-install")?;
    } else {
        fs::durable(&path, &a.identity, 0o600)?;
    }
    Ok(())
}
fn publish_selector(l: &Layout, a: &Artifact) -> Result<()> {
    let installed = l.kernel.join("selector/bin/tekes-selector");
    let selector = a.selector();
    let manifest = a.selector_manifest();
    if fs::exists(&installed)? {
        if fs::regular(&installed)? != fs::regular(&selector)? {
            let args = [
                "--install-root",
                p(&l.kernel)?,
                "update-selector",
                "--artifact",
                p(&selector)?,
                "--manifest",
                p(&manifest)?,
            ];
            if !run(&installed, &args, 45)?.success() {
                require(
                    run(&selector, &args, 45)?.success(),
                    "selector-update-failed",
                )?;
            }
        }
    } else {
        fs::durable(&installed, &fs::regular(&selector)?, 0o755)?;
    }
    Ok(())
}
fn install(l: &Layout, a: &Artifact) -> Result<Value> {
    let _lock = fs::lock(&l.installer.join(".lock"))?;
    let mut phase = begin(l, a)?;
    while phase != "closed" {
        phase = match phase.as_str() {
            "prepared" => {
                bootout(l)?;
                initialize(l, a)?;
                "identity-published"
            }
            "identity-published" => {
                require(
                    fs::regular(&l.installer.join("install-identity.json"))? == a.identity,
                    "invalid-install",
                )?;
                platform::bearer_ensure(a)?;
                "credential-published"
            }
            "credential-published" => {
                require(
                    platform::bearer_read(a)?.is_some(),
                    "endpoint-credential-unavailable",
                )?;
                bootout(l)?;
                publish_selector(l, a)?;
                let selector = l.kernel.join("selector/bin/tekes-selector");
                require(
                    run(
                        &selector,
                        &[
                            "--install-root",
                            p(&l.kernel)?,
                            "stage",
                            "--bundle",
                            p(&a.bundle())?,
                            "--version",
                            &a.version,
                        ],
                        45,
                    )?
                    .success(),
                    "bundle-stage-failed",
                )?;
                "bundles-published"
            }
            "bundles-published" => {
                require(
                    fs::regular(
                        &l.kernel
                            .join("bundles")
                            .join(&a.version)
                            .join("manifest.canonical.json"),
                    )? == fs::regular(&a.bundle().join("manifest.canonical.json"))?,
                    "invalid-install",
                )?;
                bootout(l)?;
                require(
                    run(
                        l.kernel.join("selector/bin/tekes-selector"),
                        &[
                            "--install-root",
                            p(&l.kernel)?,
                            "activate",
                            "--version",
                            &a.version,
                        ],
                        45,
                    )?
                    .success(),
                    "bundle-activate-failed",
                )?;
                "selection-published"
            }
            "selection-published" => {
                require(
                    installed(l).as_deref() == Some(&a.version),
                    "invalid-install",
                )?;
                fs::durable(&l.service_file, &platform::render(l)?, 0o644)?;
                "plist-published"
            }
            "plist-published" => {
                require(
                    fs::regular(&l.service_file)? == platform::render(l)?
                        && fs::mode(&l.service_file)? == 0o644,
                    "invalid-install",
                )?;
                platform::bootstrap(l)?;
                "service-started"
            }
            "service-started" => {
                if !platform::loaded() {
                    platform::bootstrap(l)?;
                }
                "closed"
            }
            _ => return Err(Failure("invalid-install")),
        }
        .into();
        // Close the durable transaction before readiness: selector intentionally waits for closure.
        write_operation(l, a, &phase)?;
    }
    require(
        installed(l).as_deref() == Some(&a.version),
        "invalid-install",
    )?;
    Ok(snapshot(Operation::Install, l, a))
}
fn attest(l: &Layout, a: &Artifact) -> Result<()> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if run(
            l.kernel.join("selector/bin/tekes-selector"),
            &[
                "--install-root",
                p(&l.kernel)?,
                "attest-install-health",
                "--version",
                &a.version,
            ],
            10,
        )?
        .success()
        {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(Failure("health-admission-failed"));
        }
        thread::sleep(Duration::from_millis(100));
    }
}
fn ready_reply(op: Operation, l: &Layout, a: &Artifact, ready: Value) -> Result<Value> {
    attest(l, a)?;
    Ok(
        json!({"bundled_version":a.version,"format":1,"installed_version":a.version,"operation":op.name(),"readiness":ready,"service":"ready"}),
    )
}
fn failure_code(l: &Layout) -> Option<String> {
    let out = run(
        l.kernel.join("selector/bin/tekes-selector"),
        &["--install-root", p(&l.kernel).ok()?, "status"],
        10,
    )
    .ok()?;
    if !out.success() {
        return None;
    }
    let v: Value = serde_json::from_slice(&out.stdout).ok()?;
    ["last_failure", "prelaunch_failure"]
        .into_iter()
        .find_map(|k| v[k]["code"].as_str().map(str::to_owned))
}
fn wait_ready(op: Operation, l: &Layout, a: &Artifact, rotate: bool) -> Result<Value> {
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline {
        if let Some(h) = health() {
            if h["build"] == a.version {
                return ready_reply(op, l, a, h);
            }
        }
        thread::sleep(Duration::from_millis(200));
    }
    if rotate
        && failure_code(l).as_deref() == Some("endpoint-credential-unavailable")
        && platform::bearer_read(a).ok().flatten().is_some()
    {
        platform::bearer_rotate(a)?;
        bootout(l)?;
        platform::bootstrap(l)?;
        return wait_ready(op, l, a, false);
    }
    Err(Failure("readiness-timeout"))
}
pub fn execute(op: Operation, l: &Layout, a: &Artifact) -> Result<Value> {
    match op {
        Operation::Status => Ok(snapshot(op, l, a)),
        Operation::Install => install(l, a),
        Operation::Ensure | Operation::Enable => {
            if installed(l).as_deref() != Some(&a.version) {
                install(l, a)?;
            }
            let h = health();
            if installed(l).as_deref() == Some(&a.version) && platform::loaded() {
                if let Some(h) = h {
                    if h["build"] == a.version {
                        return ready_reply(op, l, a, h);
                    }
                }
            }
            platform::bootstrap(l)?;
            wait_ready(op, l, a, true)
        }
        Operation::Disable => {
            let _lock = fs::lock(&l.installer.join(".lock"))?;
            bootout(l)?;
            Ok(
                json!({"bundled_version":a.version,"format":1,"installed_version":installed(l),"operation":op.name(),"readiness":null,"service":"stopped"}),
            )
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shutdown_requires_all_four_authorities() {
        assert!(shutdown_complete(false, false, true, true));
        for flags in [
            (true, false, true, true),
            (false, true, true, true),
            (false, false, false, true),
            (false, false, true, false),
        ] {
            assert!(!shutdown_complete(flags.0, flags.1, flags.2, flags.3));
        }
    }
}

#[cfg(test)]
mod receipt_tests {
    use super::*;
    fn setup() -> (tempfile::TempDir, Layout, Artifact) {
        let d = tempfile::tempdir().unwrap();
        let home = std::fs::canonicalize(d.path()).unwrap();
        let l = Layout::macos(&home);
        let a = Artifact {
            root: home.join("product"),
            release: home.join("product/release"),
            version: "v2".into(),
            team: "TEKESAPP01".into(),
            group: "group".into(),
            client_requirement: "client".into(),
            selector_requirement: "selector".into(),
            supervisor_requirement: "supervisor".into(),
            identity: b"{}\n".to_vec(),
            identity_sha: fs::sha(b"{}\n"),
            product_sha: "f".repeat(64),
        };
        (d, l, a)
    }
    #[test]
    fn closed_receipt_reopens_after_selection_rollback() {
        let (_d, l, a) = setup();
        write_operation(&l, &a, "closed").unwrap();
        fs::durable(
            &l.kernel.join("selector/current.json"),
            &canonical(&json!({"selection":{"version":"v1"}})).unwrap(),
            0o600,
        )
        .unwrap();
        assert_eq!(begin(&l, &a).unwrap(), "prepared");
        assert_eq!(
            fs::object(&l.installer.join("operation.json")).unwrap().0["phase"],
            "prepared"
        );
    }
    #[test]
    fn matching_closed_receipt_stays_closed() {
        let (_d, l, a) = setup();
        write_operation(&l, &a, "closed").unwrap();
        fs::durable(
            &l.kernel.join("selector/current.json"),
            &canonical(&json!({"selection":{"version":"v2"}})).unwrap(),
            0o600,
        )
        .unwrap();
        assert_eq!(begin(&l, &a).unwrap(), "closed");
    }
    #[test]
    fn incomplete_receipt_requires_absent_response() {
        let (_d, l, a) = setup();
        write_operation(&l, &a, "prepared").unwrap();
        let path = l.installer.join("operation.json");
        let (mut v, _) = fs::object(&path).unwrap();
        v["response"] = json!({});
        fs::durable(&path, &canonical(&v).unwrap(), 0o600).unwrap();
        assert_eq!(begin(&l, &a), Err(Failure("invalid-install")));
    }
    #[test]
    fn installer_lock_excludes_second_actor() {
        let (_d, l, _a) = setup();
        let _first = fs::lock(&l.installer.join(".lock")).unwrap();
        assert_eq!(
            fs::lock(&l.installer.join(".lock")).unwrap_err(),
            Failure("operation-busy")
        );
    }
}
