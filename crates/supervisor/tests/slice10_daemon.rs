use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;

use endpoint::{EndpointCarrierHost, SessionHostDescription};
use serde_json::Value;
use tekes_supervisor::daemon::{
    AUTHORITY_REGISTRY_SHA256, BootstrapReporter, DaemonArgs, ProductionRootLock, Selection,
    preflight_storage, wait_for_launcher_shutdown,
};
use tekes_supervisor::endpoint_carrier::{LiveRespondAuthority, ProductionCarrierAssembly};
use tekes_supervisor::endpoint_host::{
    ProductionEndpointHost, ProviderReadinessAuthority, QueueTransactionAuthority,
    SessionDeliveryAuthority,
};
use tekes_supervisor::observability::{
    Correlation, FrozenAttribution, LogRecord, OperationalMetrics, ProductionAccessLog,
    RotatingJsonlLog, Severity, publish_metric_snapshot,
};
use tekes_supervisor::process_host::ProductionProcessHost;
use transport::{AccessLogRecord, AccessLogSink, ROUTE_REGISTRY};

const DIGEST: &str = "ff39963a3cb039294471cf58ab3c23baad7c411dc447bcf446fd7c7d57ce42fa";

#[test]
fn slice10_gate_71_deployment_filesystem_and_ownership() {
    let directory = tempfile::tempdir().expect("storage root");
    fs::set_permissions(directory.path(), fs::Permissions::from_mode(0o700)).expect("root mode");
    let kernel = directory.path().join("Kernel");
    fs::create_dir(&kernel).expect("install root");
    fs::set_permissions(&kernel, fs::Permissions::from_mode(0o700)).expect("install root mode");
    let threads = directory.path().join("threads");
    fs::create_dir(&threads).expect("threads root");
    fs::set_permissions(&threads, fs::Permissions::from_mode(0o700)).expect("threads mode");
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(threads.join(".root-lock"))
        .expect("installer-created root lock");
    let installer = directory.path().join("Installer");
    fs::create_dir(&installer).expect("installer root");
    fs::set_permissions(&installer, fs::Permissions::from_mode(0o700)).expect("installer mode");
    let identity = b"{\"access_group\":\"TEKESAPP01.com.tekes.shared.endpoint\",\"client_requirement\":\"client-designated\",\"format\":1,\"installer_requirement\":\"installer-designated\",\"selector_requirement\":\"selector-designated\",\"supervisor_requirement\":\"supervisor-designated\",\"team_id\":\"TEKESAPP01\"}\n";
    let mut identity_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(installer.join("install-identity.json"))
        .expect("install identity");
    std::io::Write::write_all(&mut identity_file, identity).expect("identity bytes");

    let first = ProductionRootLock::acquire(&threads).expect("first owner");
    assert_eq!(first.path(), threads.join(".root-lock"));
    let second = match ProductionRootLock::acquire(&threads) {
        Ok(_) => panic!("duplicate owner acquired production root lock"),
        Err(error) => error,
    };
    assert_eq!(second.bootstrap_code(), "already-running");
    assert_eq!(second.exit_code(), 66);

    let (status_read, status_write) = pipe();
    let (lifetime_read, lifetime_write) = pipe();
    let status_fd = status_write.as_raw_fd();
    let lifetime_fd = lifetime_read.as_raw_fd();
    let mut command = Command::new(env!("CARGO_BIN_EXE_tekes-supervisor"));
    command
        .args(production_arguments(&threads))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    // SAFETY: dup2 is async-signal-safe and only installs the two exact
    // selector handoff descriptors before exec.
    unsafe {
        command.pre_exec(move || {
            if libc::dup2(status_fd, 3) == -1 || libc::dup2(lifetime_fd, 4) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let child = command.spawn().expect("real supervisor child");
    drop(status_write);
    drop(lifetime_read);
    drop(lifetime_write);
    let output = child.wait_with_output().expect("real supervisor exit");
    assert_eq!(
        output.status.code(),
        Some(66),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut bootstrap = Vec::new();
    File::from(status_read)
        .read_to_end(&mut bootstrap)
        .expect("real bootstrap status");
    let bootstrap: Value = serde_json::from_slice(&bootstrap).expect("bootstrap JSONL");
    assert_eq!(bootstrap["state"], "failed");
    assert_eq!(bootstrap["code"], "already-running");

    drop(first);
    ProductionRootLock::acquire(&threads).expect("released owner");
    preflight_storage(&threads).expect("production storage preflight");
    assert_eq!(
        fs::symlink_metadata(directory.path().join("jobs"))
            .expect("jobs authority")
            .permissions()
            .mode()
            & 0o777,
        0o700
    );

    let args = DaemonArgs::parse(production_arguments(&threads)).expect("exact handoff");
    assert_eq!(args.listen.to_string(), "127.0.0.1:7347");
    assert_eq!(args.selector_generation, 2);
    assert_eq!(args.launch_id, "2-1-0123456789abcdef0123456789abcdef");
    assert_eq!(args.bootstrap_status_fd, 3);
    assert_eq!(args.launcher_lifetime_fd, 4);
    assert_eq!(args.authority_registry_sha256, AUTHORITY_REGISTRY_SHA256);
    assert_eq!(args.web_listen, None, "Web Client is disabled by default");
    let mut web_arguments = production_arguments(&threads);
    web_arguments.extend([
        OsString::from("--web-listen"),
        OsString::from("127.0.0.1:7357"),
    ]);
    assert_eq!(
        DaemonArgs::parse(web_arguments)
            .expect("explicit Web Client listen")
            .web_listen
            .expect("enabled Web Client")
            .to_string(),
        "127.0.0.1:7357"
    );
    assert!(
        !threads.join("threads").exists(),
        "storage root must never be interpreted as its own data root"
    );

    let mut reordered = production_arguments(&threads);
    reordered.swap(2, 4);
    assert_eq!(
        DaemonArgs::parse(reordered)
            .expect_err("argument order is frozen")
            .bootstrap_code(),
        "protocol-mismatch"
    );
    let mut mismatched_launch = production_arguments(&threads);
    mismatched_launch[9] = OsString::from("3-1-0123456789abcdef0123456789abcdef");
    assert_eq!(
        DaemonArgs::parse(mismatched_launch)
            .expect_err("launch generation must bind argv generation")
            .bootstrap_code(),
        "protocol-mismatch"
    );

    let process_host = ProductionProcessHost::open(
        directory.path(),
        std::env::current_exe().expect("test binary"),
        env!("CARGO_PKG_VERSION"),
        directory.path().join(".agent"),
    )
    .expect("process host");
    let host = ProductionEndpointHost::open_with_full_authorities(
        directory.path(),
        SessionHostDescription {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            cwd: "/".to_owned(),
            provider: None,
            model: None,
            attached_sessions: 0,
            home: directory.path().to_string_lossy().into_owned(),
            can_open_path: false,
        },
        Arc::new(|| Ok("2026-08-28T00:00:00.000000000Z".to_owned())),
        None,
        Some(Arc::clone(&process_host) as Arc<dyn ProviderReadinessAuthority>),
        Some(Arc::clone(&process_host) as Arc<dyn SessionDeliveryAuthority>),
        Some(Arc::clone(&process_host) as Arc<dyn QueueTransactionAuthority>),
    )
    .expect("production endpoint host");
    let live: Arc<dyn LiveRespondAuthority> = Arc::clone(&process_host) as Arc<_>;
    let assembly = ProductionCarrierAssembly::assemble(
        directory.path(),
        host,
        live,
        transport::TransportConfig::loopback(
            "127.0.0.1:0".parse().expect("loopback"),
            transport::BearerToken::new([7; 32]),
        ),
    )
    .expect("complete carrier");
    assert_eq!(
        assembly.host().registered_methods(),
        ROUTE_REGISTRY
            .iter()
            .map(|method| (*method).to_owned())
            .collect()
    );
}

#[test]
fn slice10_gate_72_launchd_crash_recovery() {
    let selection = Selection {
        version: env!("CARGO_PKG_VERSION").to_owned(),
        manifest_sha256: DIGEST.to_owned(),
    };
    let (status_read, status_write) = pipe();
    let mut reporter = BootstrapReporter::from_fd(status_write.as_raw_fd()).expect("reporter");
    reporter
        .listener_bound("2-1-0123456789abcdef0123456789abcdef", &selection)
        .expect("single bootstrap row");
    drop(reporter);
    drop(status_write);
    let mut bytes = Vec::new();
    File::from(status_read)
        .read_to_end(&mut bytes)
        .expect("bootstrap bytes");
    assert_eq!(
        bytes,
        format!(
            "{{\"format\":1,\"launch_id\":\"2-1-0123456789abcdef0123456789abcdef\",\"selection\":{{\"manifest_sha256\":\"{DIGEST}\",\"version\":\"{}\"}},\"state\":\"listener-bound\"}}\n",
            env!("CARGO_PKG_VERSION")
        )
        .as_bytes()
    );

    let (lifetime_read, lifetime_write) = pipe();
    let descriptor = lifetime_read.as_raw_fd();
    let watcher = std::thread::spawn(move || wait_for_launcher_shutdown(descriptor));
    std::thread::sleep(std::time::Duration::from_millis(20));
    drop(lifetime_write);
    watcher
        .join()
        .expect("watcher thread")
        .expect("launcher EOF triggers shutdown");
    drop(lifetime_read);

    let periodic_root = tempfile::tempdir().expect("periodic sweep root");
    fs::create_dir(periodic_root.path().join("threads")).expect("threads root");
    let process_host = ProductionProcessHost::open(
        periodic_root.path(),
        std::env::current_exe().expect("test executable"),
        "test-build",
        periodic_root.path().join("missing-agent-home"),
    )
    .expect("process host");
    process_host.start_periodic_sweep();
    process_host.start_periodic_sweep();
    assert!(
        process_host
            .periodic_sweep_once()
            .expect("uncontended periodic sweep"),
        "the production recovery authority must run a periodic pass"
    );
    process_host.shutdown();
    assert!(
        !process_host
            .periodic_sweep_once()
            .expect("draining periodic sweep"),
        "launcher shutdown must stop periodic recovery before new work"
    );
}

#[test]
fn slice10_support_bundle_command_and_schema() {
    let directory = tempfile::tempdir().expect("support root");
    let log_root = directory.path().join("logs");
    fs::create_dir(&log_root).expect("log root");
    let destination = directory.path().join("support-bundle");
    let attribution = FrozenAttribution {
        build: "2.0.0".to_owned(),
        launch_id: "2-2-0123456789abcdef0123456789abcdef".to_owned(),
        attempt: 2,
        generation: 2,
        manifest_sha256: DIGEST.to_owned(),
    };
    let record = LogRecord {
        v: 1,
        ts: "2026-08-28T00:00:00.000000000Z".to_owned(),
        severity: Severity::Error,
        component: "supervisor".to_owned(),
        build: attribution.build.clone(),
        code: "provider-terminal".to_owned(),
        message: "Provider request failed terminally".to_owned(),
        correlation: Correlation::from(&attribution),
        fields: BTreeMap::from([(
            "classification".to_owned(),
            Value::String("http-terminal".to_owned()),
        )]),
    };
    let log = Arc::new(RotatingJsonlLog::new(log_root.join("supervisor.jsonl")));
    let production_metrics = Arc::new(OperationalMetrics::default());
    let production = ProductionAccessLog::new(
        Arc::clone(&production_metrics),
        Arc::clone(&log),
        attribution,
    );
    production.record(&AccessLogRecord {
        v: 1,
        request_id: "rpc-invalid".to_owned(),
        operation: "session.prompt".to_owned(),
        path: "/api/session.prompt".to_owned(),
        status: 400,
        elapsed_ms: 1,
        error_code: Some("invalid-request".to_owned()),
    });
    let metrics = production_metrics.snapshot("2026-08-28T00:00:00.000000000Z", true);
    publish_metric_snapshot(&log_root.join("metrics.canonical.json"), &metrics)
        .expect("production metric snapshot");
    let output = Command::new(env!("CARGO_BIN_EXE_tekes-supervisor"))
        .args([
            "support-bundle",
            "--destination",
            destination.to_str().expect("destination UTF-8"),
            "--log-root",
            log_root.to_str().expect("log root UTF-8"),
            "--build",
            "2.0.0",
            "--capability-digest",
            DIGEST,
            "--config-digests",
            AUTHORITY_REGISTRY_SHA256,
        ])
        .output()
        .expect("run production support-bundle command");
    assert!(
        output.status.success(),
        "support-bundle failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    assert_eq!(
        fs::read_dir(&destination)
            .expect("bundle")
            .map(|entry| entry.expect("entry").file_name())
            .collect::<std::collections::BTreeSet<_>>(),
        [
            "logs.jsonl".into(),
            "manifest.canonical.json".into(),
            "metrics.canonical.json".into(),
        ]
        .into_iter()
        .collect()
    );
    for entry in fs::read_dir(&destination).expect("bundle") {
        let entry = entry.expect("entry");
        assert_eq!(
            entry.metadata().expect("metadata").permissions().mode() & 0o777,
            0o600
        );
        let bytes = fs::read(entry.path()).expect("bundle bytes");
        assert!(!bytes.windows(13).any(|window| window == b"Authorization"));
    }

    let mut forbidden = record;
    forbidden.fields.insert(
        "classification".to_owned(),
        Value::String("endpoint-token-canary".to_owned()),
    );
    assert!(forbidden.canonical_line().is_err());

    production_metrics.increment("endpoint_requests_total");
    let snapshot = production_metrics.snapshot("2026-08-28T00:00:00.000000000Z", true);
    assert_eq!(snapshot.metrics.len(), 17);
    assert!(
        snapshot
            .metrics
            .windows(2)
            .all(|metrics| metrics[0].name < metrics[1].name)
    );
    assert_eq!(
        snapshot
            .metrics
            .iter()
            .find(|metric| metric.name == "readiness")
            .expect("readiness metric")
            .value,
        1.0
    );
    let mut incomplete = snapshot.clone();
    incomplete.metrics.pop();
    assert!(incomplete.canonical_line().is_err());
    let mut duplicate = snapshot.clone();
    duplicate.metrics[1] = duplicate.metrics[0].clone();
    assert!(duplicate.canonical_line().is_err());
}

fn production_arguments(root: &Path) -> Vec<OsString> {
    let install_root = root.parent().expect("authority root").join("Kernel");
    [
        OsString::from("--install-root"),
        install_root.into_os_string(),
        OsString::from("--storage-root"),
        root.as_os_str().to_owned(),
        OsString::from("--listen"),
        OsString::from("127.0.0.1:7347"),
        OsString::from("--selected-version"),
        OsString::from(env!("CARGO_PKG_VERSION")),
        OsString::from("--selector-generation"),
        OsString::from("2"),
        OsString::from("--launch-id"),
        OsString::from("2-1-0123456789abcdef0123456789abcdef"),
        OsString::from("--manifest-sha256"),
        OsString::from(DIGEST),
        OsString::from("--bootstrap-status-fd"),
        OsString::from("3"),
        OsString::from("--authority-registry-sha256"),
        OsString::from(AUTHORITY_REGISTRY_SHA256),
        OsString::from("--launcher-lifetime-fd"),
        OsString::from("4"),
    ]
    .into_iter()
    .collect()
}

fn pipe() -> (std::os::fd::OwnedFd, std::os::fd::OwnedFd) {
    let mut descriptors = [-1; 2];
    // SAFETY: descriptors points to two writable integers; pipe initializes both.
    assert_eq!(unsafe { libc::pipe(descriptors.as_mut_ptr()) }, 0);
    // SAFETY: successful pipe returned two new owned descriptors.
    unsafe {
        (
            std::os::fd::OwnedFd::from_raw_fd(descriptors[0]),
            std::os::fd::OwnedFd::from_raw_fd(descriptors[1]),
        )
    }
}
