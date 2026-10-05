use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use deployment_tests::{
    HELPER_BIN_ENV, RELEASE_VERSION_ENV, SELECTOR_BIN_ENV, SUPERVISOR_BIN_ENV, WORKER_BIN_ENV,
    configured_binary, workspace_root,
};
use tekes_selector::{
    InstallRequest, InstallerEffects, InstallerOperationType, InstallerStateMachine, SelectorError,
};

const IDENTITY_SHA: &str = "ba8d64d3d850bcbba7b7c5a40abe643f583604c586717be0cf77edfc594a5d13";

struct FilesystemEffects {
    kernel: PathBuf,
    threads: PathBuf,
    installer: PathBuf,
    launch_agent: PathBuf,
    credential: PathBuf,
    service: PathBuf,
    selector_source: PathBuf,
    supervisor_source: PathBuf,
    worker_source: PathBuf,
    helper_source: PathBuf,
    identity_source: PathBuf,
    authority_registry: PathBuf,
    fail_once_at: Option<&'static str>,
}

impl FilesystemEffects {
    fn effect(
        &mut self,
        name: &'static str,
        action: impl FnOnce(&Self) -> std::io::Result<()>,
    ) -> Result<(), SelectorError> {
        if self.fail_once_at == Some(name) {
            self.fail_once_at = None;
            return Err(SelectorError::invalid_state(format!("injected-{name}")));
        }
        action(self).map_err(|error| SelectorError::io(name, error))
    }

    fn write(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, bytes)
    }

    fn write_mode(path: &Path, bytes: &[u8], mode: u32) -> std::io::Result<()> {
        Self::write(path, bytes)?;
        fs::set_permissions(path, fs::Permissions::from_mode(mode))
    }

    fn private_directory(path: &Path) -> std::io::Result<()> {
        fs::create_dir_all(path)?;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
    }

    fn install_executable(source: &Path, destination: &Path) -> std::io::Result<()> {
        if let Some(parent) = destination.parent() {
            Self::private_directory(parent)?;
        }
        fs::copy(source, destination)?;
        fs::set_permissions(destination, fs::Permissions::from_mode(0o755))
    }

    fn regular_mode(path: &Path, mode: u32) -> bool {
        fs::symlink_metadata(path).is_ok_and(|metadata| {
            metadata.file_type().is_file() && metadata.permissions().mode() & 0o7777 == mode
        })
    }
}

impl InstallerEffects for FilesystemEffects {
    fn publish_identity(&mut self) -> Result<(), SelectorError> {
        self.effect("identity-published", |this| {
            let destination = this.installer.join("install-identity.json");
            fs::copy(&this.identity_source, &destination)?;
            fs::set_permissions(destination, fs::Permissions::from_mode(0o600))?;
            Self::private_directory(&this.threads)?;
            let root_lock = this.threads.join(".root-lock");
            if !root_lock.exists() {
                Self::write_mode(&root_lock, b"", 0o600)?;
            }
            Ok(())
        })
    }

    fn ensure_credential(&mut self) -> Result<(), SelectorError> {
        self.effect("credential-published", |this| {
            if this.credential.exists() {
                return Ok(());
            }
            Self::write_mode(&this.credential, &[0x5a; 32], 0o600)
        })
    }

    fn publish_bundles(&mut self) -> Result<(), SelectorError> {
        self.effect("bundles-published", |this| {
            for directory in [
                this.kernel.clone(),
                this.kernel.join("bundles"),
                this.kernel.join(format!("bundles/{}", release_version())),
                this.kernel
                    .join(format!("bundles/{}/apps", release_version())),
                this.kernel.join(format!(
                    "bundles/{}/apps/TekesKernelSupervisor.app",
                    release_version()
                )),
                this.kernel.join(format!(
                    "bundles/{}/apps/TekesKernelSupervisor.app/Contents",
                    release_version()
                )),
                this.kernel.join(format!(
                    "bundles/{}/apps/TekesKernelSupervisor.app/Contents/MacOS",
                    release_version()
                )),
                this.kernel.join(format!(
                    "bundles/{}/apps/TekesKernelSupervisor.app/Contents/_CodeSignature",
                    release_version()
                )),
                this.kernel
                    .join(format!("bundles/{}/bin", release_version())),
                this.kernel.join("selector"),
                this.kernel.join("selector/bin"),
                this.kernel.join("selector/operations"),
                this.kernel.join("selector/observations"),
                this.kernel.join(format!(
                    "bundles/{}/apps/TekesKernelSupervisor.app/Contents/Resources",
                    release_version()
                )),
            ] {
                Self::private_directory(&directory)?;
            }
            Self::install_executable(
                &this.helper_source,
                &this
                    .kernel
                    .join(format!("bundles/{}/bin/tekes-helper", release_version())),
            )?;
            Self::install_executable(
                &this.helper_source.with_file_name("tekes-workspace-service"),
                &this.kernel.join(format!(
                    "bundles/{}/bin/tekes-workspace-service",
                    release_version()
                )),
            )?;
            Self::install_executable(
                &this.supervisor_source,
                &this.kernel.join(format!(
                    "bundles/{}/apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor",
                    release_version()
                )),
            )?;
            for (relative, bytes) in [
                ("Info.plist", b"fixture-info".as_slice()),
                (
                    "Resources/WebClientManifest.canonical.json",
                    b"fixture-web-client-manifest".as_slice(),
                ),
                ("embedded.provisionprofile", b"fixture-profile".as_slice()),
                (
                    "_CodeSignature/CodeResources",
                    b"fixture-code-resources".as_slice(),
                ),
            ] {
                let source_contents = this.supervisor_source.parent().and_then(Path::parent);
                let source_resource = source_contents.map(|contents| contents.join(relative));
                let resource = if let Some(source) = source_resource.filter(|path| path.is_file()) {
                    fs::read(source)?
                } else {
                    bytes.to_vec()
                };
                Self::write_mode(
                    &this
                        .kernel
                        .join(format!(
                            "bundles/{}/apps/TekesKernelSupervisor.app/Contents",
                            release_version()
                        ))
                        .join(relative),
                    &resource,
                    0o644,
                )?;
            }
            Self::install_executable(
                &this.worker_source,
                &this
                    .kernel
                    .join(format!("bundles/{}/bin/tekes-worker", release_version())),
            )?;
            Self::install_executable(
                &this.selector_source,
                &this.kernel.join("selector/bin/tekes-selector"),
            )?;
            let status = Command::new("python3")
                .arg(workspace_root().join("packaging/macos/assemble-bundle.py"))
                .args(["--bundle-root"])
                .arg(this.kernel.join(format!("bundles/{}", release_version())))
                .args([
                    "--version",
                    &release_version(),
                    "--team-id",
                    "TEKESAPP01",
                    "--requirement",
                    "anchor apple generic and identifier com.tekes.kernel.supervisor",
                    "--authority-registry",
                ])
                .arg(&this.authority_registry)
                .status()?;
            if !status.success() {
                return Err(std::io::Error::other("bundle assembler failed"));
            }
            Ok(())
        })
    }

    fn publish_selection(&mut self) -> Result<(), SelectorError> {
        self.effect("selection-published", |this| {
            let manifest = this
                .kernel
                .join(format!("bundles/{}/manifest.canonical.json", release_version()));
            let output = Command::new("/usr/bin/shasum")
                .args(["-a", "256"])
                .arg(&manifest)
                .output()?;
            if !output.status.success() {
                return Err(std::io::Error::other("manifest digest failed"));
            }
            let text = String::from_utf8(output.stdout).map_err(std::io::Error::other)?;
            let digest = text
                .split_whitespace()
                .next()
                .ok_or_else(|| std::io::Error::other("manifest digest missing"))?;
            let selector = this.kernel.join("selector");
            Self::write_mode(
                &selector.join("current.json"),
                format!("{{\"format\":1,\"generation\":1,\"selection\":{{\"manifest_sha256\":\"{digest}\",\"version\":\"{}\"}}}}\n", release_version()).as_bytes(),
                0o600,
            )?;
            Self::write_mode(
                &selector.join("previous.json"),
                b"{\"format\":1,\"generation\":1}\n",
                0o600,
            )?;
            symlink(format!("../bundles/{}", release_version()), selector.join("active"))
        })
    }

    fn publish_plist(&mut self) -> Result<(), SelectorError> {
        self.effect("plist-published", |this| {
            let workspace = workspace_root();
            let logs = this
                .kernel
                .parent()
                .and_then(Path::parent)
                .and_then(Path::parent)
                .expect("test home")
                .join("Logs/Tekes/Kernel");
            fs::create_dir_all(&logs)?;
            fs::set_permissions(&logs, fs::Permissions::from_mode(0o700))?;
            let caches = logs
                .parent()
                .and_then(Path::parent)
                .and_then(Path::parent)
                .expect("test Library")
                .join("Caches/Tekes/Kernel");
            fs::create_dir_all(&caches)?;
            fs::set_permissions(caches, fs::Permissions::from_mode(0o700))?;
            let selector = this.kernel.join("selector/bin/tekes-selector");
            let status = Command::new("python3")
                .arg(workspace.join("packaging/macos/render-launch-agent.py"))
                .args(["--template"])
                .arg(workspace.join("fixtures/deployment/com.tekes.kernel.supervisor.plist"))
                .args(["--output"])
                .arg(&this.launch_agent)
                .args(["--install-root"])
                .arg(&this.kernel)
                .args(["--storage-root"])
                .arg(&this.threads)
                .args(["--selector"])
                .arg(&selector)
                .args(["--stdout"])
                .arg(logs.join("stdout.log"))
                .args(["--stderr"])
                .arg(logs.join("stderr.log"))
                .status()?;
            if !status.success() {
                return Err(std::io::Error::other("LaunchAgent renderer failed"));
            }
            Ok(())
        })
    }

    fn start_service(&mut self) -> Result<(), SelectorError> {
        self.effect("service-started", |this| {
            Self::write(&this.service, b"running")
        })
    }

    fn stop_service(&mut self) -> Result<(), SelectorError> {
        self.effect("service-stopped", |this| {
            if this.service.exists() {
                fs::remove_file(&this.service)?;
            }
            Ok(())
        })
    }

    fn replace_credential(&mut self) -> Result<(), SelectorError> {
        self.effect("credential-replaced", |this| {
            Self::write_mode(&this.credential, &[0xa5; 32], 0o600)
        })
    }

    fn delete_credential(&mut self) -> Result<(), SelectorError> {
        self.effect("credential-deleted", |this| {
            if this.credential.exists() {
                fs::remove_file(&this.credential)?;
            }
            Ok(())
        })
    }

    fn remove_plist(&mut self) -> Result<(), SelectorError> {
        self.effect("plist-removed", |this| {
            if this.launch_agent.exists() {
                fs::remove_file(&this.launch_agent)?;
            }
            Ok(())
        })
    }

    fn remove_binaries(&mut self) -> Result<(), SelectorError> {
        self.effect("binaries-removed", |this| {
            if this.kernel.exists() {
                fs::remove_dir_all(&this.kernel)?;
            }
            Ok(())
        })
    }

    fn verify_completed_phase(
        &mut self,
        operation: &tekes_selector::InstallerOperation,
        phase: &str,
    ) -> Result<bool, SelectorError> {
        let verified = match (operation.operation_type, phase) {
            (InstallerOperationType::Install, "identity-published") => {
                Self::regular_mode(&self.installer.join("install-identity.json"), 0o600)
                    && Self::regular_mode(&self.threads.join(".root-lock"), 0o600)
                    && !self.threads.join("threads").exists()
            }
            (InstallerOperationType::Install, "credential-published") => self
                .credential
                .metadata()
                .is_ok_and(|metadata| metadata.is_file() && metadata.len() == 32),
            (InstallerOperationType::Install, "bundles-published") => {
                [
                    "apps/TekesKernelSupervisor.app/Contents/Info.plist",
                    "apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor",
                    "apps/TekesKernelSupervisor.app/Contents/Resources/WebClientManifest.canonical.json",
                    "apps/TekesKernelSupervisor.app/Contents/_CodeSignature/CodeResources",
                    "apps/TekesKernelSupervisor.app/Contents/embedded.provisionprofile",
                    "bin/tekes-helper",
                    "bin/tekes-worker",
                    "bin/tekes-workspace-service",
                ]
                .iter()
                .all(|relative| {
                    let mode =
                        if relative.ends_with("tekes-supervisor") || relative.starts_with("bin/") {
                            0o755
                        } else {
                            0o644
                        };
                    Self::regular_mode(&self.kernel.join(format!("bundles/{}", release_version())).join(relative), mode)
                }) && Self::regular_mode(
                    &self.kernel.join(format!("bundles/{}/manifest.canonical.json", release_version())),
                    0o600,
                )
            }
            (InstallerOperationType::Install, "selection-published") => {
                Self::regular_mode(&self.kernel.join("selector/current.json"), 0o600)
                    && Self::regular_mode(&self.kernel.join("selector/previous.json"), 0o600)
                    && fs::symlink_metadata(self.kernel.join("selector/active"))
                        .is_ok_and(|metadata| metadata.file_type().is_symlink())
            }
            (InstallerOperationType::Install, "plist-published") => {
                Self::regular_mode(&self.launch_agent, 0o644)
            }
            (
                InstallerOperationType::Install | InstallerOperationType::RotateCredential,
                "service-started",
            ) => self.service.is_file(),
            (
                InstallerOperationType::RotateCredential | InstallerOperationType::Uninstall,
                "service-stopped",
            ) => !self.service.exists(),
            (InstallerOperationType::RotateCredential, "credential-replaced") => {
                fs::read(&self.credential).is_ok_and(|bytes| bytes == [0xa5; 32])
            }
            (InstallerOperationType::Uninstall, "credential-deleted") => !self.credential.exists(),
            (InstallerOperationType::Uninstall, "plist-removed") => !self.launch_agent.exists(),
            (InstallerOperationType::Uninstall, "binaries-removed") => !self.kernel.exists(),
            _ => false,
        };
        Ok(verified)
    }
}

fn scratch() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "tekes-slice10-gate76-{}-{stamp}",
        std::process::id()
    ));
    fs::create_dir(&path).expect("create UAT root");
    path
}

fn request(kind: InstallerOperationType, id: &str, digest_byte: char) -> InstallRequest {
    InstallRequest {
        install_identity_sha256: IDENTITY_SHA.to_owned(),
        op_id: id.to_owned(),
        request_sha256: digest_byte.to_string().repeat(64),
        operation_type: kind,
    }
}

fn required_binary(variable: &str) -> PathBuf {
    let path = configured_binary(variable)
        .unwrap_or_else(|| panic!("{variable} must name the built Slice-10 executable"));
    let metadata = fs::metadata(&path)
        .unwrap_or_else(|error| panic!("{variable}={} is unavailable: {error}", path.display()));
    assert!(metadata.is_file(), "{variable} is not a regular file");
    assert_ne!(
        metadata.permissions().mode() & 0o111,
        0,
        "{variable} is not executable"
    );
    path
}

fn release_version() -> String {
    let version = std::env::var(RELEASE_VERSION_ENV)
        .unwrap_or_else(|_| panic!("{RELEASE_VERSION_ENV} must name the immutable build version"));
    assert!(
        !version.is_empty()
            && version.len() <= 128
            && version
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(&byte))
            && version.as_bytes()[0].is_ascii_alphanumeric(),
        "invalid fixture release version"
    );
    version
}

fn assert_embedded_release_bootstrap(
    supervisor: &Path,
    install_root: &Path,
    storage_root: &Path,
    manifest_sha256: &str,
) {
    let release_version = release_version();
    let output = Command::new("python3")
        .arg(workspace_root().join("packaging/macos/probe-supervisor-bootstrap.py"))
        .args(["--supervisor"])
        .arg(supervisor)
        .args(["--install-root"])
        .arg(install_root)
        .args(["--storage-root"])
        .arg(storage_root)
        .args(["--selected-version", &release_version])
        .args(["--manifest-sha256", manifest_sha256])
        .output()
        .expect("run installed supervisor bootstrap probe");
    assert!(
        output.status.success(),
        "installed supervisor embedded build differs from bundle selection: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
#[ignore = "requires the four explicitly selected Slice-10 build artifacts; ci-slice10 invokes it"]
fn slice10_installer_recovery_built_artifact_support() {
    let root = scratch();
    let app_support = root.join("Library/Application Support/Tekes");
    let threads = app_support.join("threads");
    let archive = app_support.join("archive");
    let installer = app_support.join("Installer");
    fs::create_dir_all(threads.join("active-a")).expect("active thread");
    fs::create_dir_all(archive.join("archived-a")).expect("archived thread");
    fs::write(threads.join("active-a/main.jsonl"), b"active-ledger\n").expect("active ledger");
    fs::write(archive.join("archived-a/main.jsonl"), b"archive-ledger\n").expect("archive ledger");

    let machine = InstallerStateMachine::new(&installer);
    machine
        .initialize()
        .expect("initialize installer authority");
    let mut effects = FilesystemEffects {
        kernel: app_support.join("Kernel"),
        threads: threads.clone(),
        installer: installer.clone(),
        launch_agent: root.join("Library/LaunchAgents/com.tekes.kernel.supervisor.plist"),
        credential: installer.join("endpoint-credential.test"),
        service: installer.join("service-running.test"),
        selector_source: required_binary(SELECTOR_BIN_ENV),
        supervisor_source: required_binary(SUPERVISOR_BIN_ENV),
        worker_source: required_binary(WORKER_BIN_ENV),
        helper_source: required_binary(HELPER_BIN_ENV),
        identity_source: workspace_root()
            .join("fixtures/deployment/install-identity.canonical.json"),
        authority_registry: workspace_root()
            .join("fixtures/deployment/authority-registry.canonical.json"),
        fail_once_at: Some("credential-published"),
    };

    let install = request(InstallerOperationType::Install, "install-0001", 'a');
    assert!(
        machine
            .begin(install.clone())
            .expect("begin install")
            .is_none()
    );
    assert!(
        machine.recover(&mut effects).is_err(),
        "injected install crash must surface"
    );
    assert!(
        machine
            .begin(install)
            .expect("same install retry")
            .is_none()
    );
    let reply = machine.recover(&mut effects).expect("recover install");
    assert_eq!(reply["operation"], "install");
    assert_eq!(reply["service"], "running");
    assert_eq!(fs::read(&effects.credential).expect("credential").len(), 32);
    assert!(effects.kernel.join("selector/bin/tekes-selector").is_file());
    // Exercise the installed copy, not just the build input or its file mode.
    {
        use std::io::Write;
        use std::process::Stdio;
        let mut child = Command::new(effects.kernel.join(format!(
            "bundles/{}/bin/tekes-workspace-service",
            release_version()
        )))
        .arg("--root")
        .arg(&threads)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(br#"{"method":"filesList","request":{"workspaceId":"installed","path":""}}"#)
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success());
        let reply: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(
            reply.get("result").is_some(),
            "installed workspace helper failed: {reply}"
        );
    }
    assert!(effects.launch_agent.is_file());
    let launch_agent = fs::read_to_string(&effects.launch_agent).expect("LaunchAgent text");
    assert!(launch_agent.contains(&format!(
        "<string>{}</string><string>--listen</string>",
        effects.threads.display()
    )));
    assert!(
        !launch_agent.contains("threads/threads"),
        "LaunchAgent must pass the threads root exactly once"
    );
    assert_eq!(
        fs::metadata(&effects.launch_agent)
            .expect("LaunchAgent metadata")
            .permissions()
            .mode()
            & 0o777,
        0o644
    );
    // The unsigned supporting lane cannot truthfully exercise the production
    // selector's macOS code-signature verifier. Gate 73 covers selector state
    // transactions with its injectable verifier; the production lane below
    // must run `serve` against distribution-signed bytes.
    for directory in [
        effects.kernel.clone(),
        effects.kernel.join("bundles"),
        effects
            .kernel
            .join(format!("bundles/{}", release_version())),
        effects
            .kernel
            .join(format!("bundles/{}/bin", release_version())),
        effects.kernel.join("selector"),
        effects.kernel.join("selector/bin"),
        effects.kernel.join("selector/operations"),
        effects.kernel.join("selector/observations"),
        effects.threads.clone(),
    ] {
        assert_eq!(
            fs::metadata(&directory)
                .unwrap_or_else(|error| panic!("{}: {error}", directory.display()))
                .permissions()
                .mode()
                & 0o777,
            0o700,
            "{}",
            directory.display()
        );
    }
    for executable in ["tekes-helper", "tekes-worker", "tekes-workspace-service"] {
        assert_eq!(
            fs::metadata(
                effects
                    .kernel
                    .join(format!("bundles/{}/bin", release_version()))
                    .join(executable)
            )
            .expect("installed executable")
            .permissions()
            .mode()
                & 0o777,
            0o755
        );
    }
    assert_eq!(
        fs::metadata(effects.kernel.join(format!(
            "bundles/{}/apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor",
            release_version()
        )),)
        .expect("installed supervisor")
        .permissions()
        .mode()
            & 0o777,
        0o755
    );
    let manifest = effects.kernel.join(format!(
        "bundles/{}/manifest.canonical.json",
        release_version()
    ));
    let digest_output = Command::new("/usr/bin/shasum")
        .args(["-a", "256"])
        .arg(&manifest)
        .output()
        .expect("manifest digest");
    assert!(digest_output.status.success());
    let digest_text = String::from_utf8(digest_output.stdout).expect("digest text");
    let digest = digest_text.split_whitespace().next().expect("digest");
    assert_embedded_release_bootstrap(
        &effects.kernel.join(format!(
            "bundles/{}/apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor",
            release_version()
        )),
        &effects.kernel,
        &effects.threads,
        digest,
    );
    let verified = Command::new("python3")
        .arg(workspace_root().join("packaging/macos/verify-release.py"))
        .arg("bundle")
        .args(["--root"])
        .arg(
            effects
                .kernel
                .join(format!("bundles/{}", release_version())),
        )
        .args(["--manifest"])
        .arg(&manifest)
        .args(["--install-identity"])
        .arg(&effects.identity_source)
        .args(["--authority-registry"])
        .arg(&effects.authority_registry)
        .args(["--expected-manifest-sha256", digest])
        .status()
        .expect("release verifier");
    assert!(
        verified.success(),
        "installed release bundle failed parity verification"
    );

    let rotate = request(InstallerOperationType::RotateCredential, "rotate-0002", 'b');
    machine.begin(rotate).expect("begin rotation");
    let rotated = machine.recover(&mut effects).expect("rotate credential");
    assert_eq!(rotated["operation"], "rotate-credential");
    assert_eq!(
        fs::read(&effects.credential).expect("rotated credential"),
        [0xa5; 32]
    );

    let uninstall = request(InstallerOperationType::Uninstall, "uninstall-0003", 'd');
    machine.begin(uninstall.clone()).expect("begin uninstall");
    effects.fail_once_at = Some("plist-removed");
    assert!(
        machine.recover(&mut effects).is_err(),
        "injected uninstall crash must surface"
    );
    assert!(
        machine
            .begin(uninstall)
            .expect("same uninstall retry")
            .is_none()
    );
    let removed = machine.recover(&mut effects).expect("recover uninstall");
    assert_eq!(removed["operation"], "uninstall");
    assert_eq!(removed["service"], "absent");
    assert!(!effects.kernel.exists());
    assert!(!effects.launch_agent.exists());
    assert!(!effects.credential.exists());
    assert_eq!(
        fs::read(effects.threads.join("active-a/main.jsonl")).expect("retained active ledger"),
        b"active-ledger\n"
    );
    assert_eq!(
        fs::read(archive.join("archived-a/main.jsonl")).expect("retained archive ledger"),
        b"archive-ledger\n"
    );
    assert!(
        !effects.threads.join("threads").exists(),
        "installer must not create threads/threads"
    );
    assert!(
        installer.join("operation.json").is_file(),
        "closed retry authority retained"
    );

    fs::remove_dir_all(root).expect("remove UAT root");
}
