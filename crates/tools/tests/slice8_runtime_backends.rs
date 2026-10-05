use std::collections::BTreeMap;
use std::os::unix::fs::PermissionsExt as _;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tools::{
    BackendFailure, BackendGate, BackendHold, BackendOutcome, BackendUnavailable, BoundedHelper,
    BoundedHttpClient, CancellationToken, HttpLimits, JobBroker, JobLaunchPolicy, JobSpec,
    JobState, SandboxedJobLauncher, SearchHit, SearchProvider, SearchRequest, SearchTopic,
};
use tools::{
    HelperClient, HelperClientError, HelperInvoker, HelperOperation, HelperRequest, NetworkPolicy,
    SandboxApproval, SandboxPolicy, policy_digest,
};

struct MissingHelper;

impl HelperInvoker for MissingHelper {
    fn invoke(
        &self,
        _request: &HelperRequest,
        _cancellation: &CancellationToken,
    ) -> Result<tools::HelperResponse, BackendFailure> {
        Err(BackendFailure::Unavailable("missing".to_owned()))
    }
}

#[test]
fn helper_client_cancellation_kills_the_process_group() {
    let root = tempfile::tempdir().unwrap();
    let pid_path = root.path().join("helper.pid");
    let pid_staging_path = root.path().join("helper.pid.tmp");
    let script = root.path().join("blocking-helper");
    std::fs::write(
        &script,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$$\" > '{}'\nmv '{}' '{}'\nsleep 30\n",
            pid_staging_path.display(),
            pid_staging_path.display(),
            pid_path.display()
        ),
    )
    .unwrap();
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o700)).unwrap();
    let canonical_root = std::fs::canonicalize(root.path()).unwrap();
    let policy = SandboxPolicy {
        format: 1,
        read_roots: vec![canonical_root.to_string_lossy().into_owned()],
        write_roots: vec![canonical_root.to_string_lossy().into_owned()],
        network: NetworkPolicy::Deny,
        allow_process: true,
        scratch: None,
    };
    let digest = policy_digest(&policy).unwrap();
    let approval = SandboxApproval {
        run: "run-1".to_owned(),
        call: "call-1".to_owned(),
        policy_digest: digest,
        grant: true,
    };
    let client = HelperClient::approved_unsandboxed(
        &script,
        Vec::new(),
        &policy,
        &approval,
        "run-1",
        "call-1",
    )
    .unwrap();
    let request = HelperRequest {
        id: "cancel-1".to_owned(),
        operation: HelperOperation::Read {
            root: "workspace".to_owned(),
            path: "ignored".to_owned(),
            max_bytes: 1,
        },
    };
    let cancelled = Arc::new(AtomicBool::new(false));
    let trigger = Arc::clone(&cancelled);
    let trigger_pid_path = pid_path.clone();
    thread::spawn(move || {
        for _ in 0..2_000 {
            if trigger_pid_path.is_file() {
                break;
            }
            thread::sleep(Duration::from_millis(5));
        }
        trigger.store(true, Ordering::Release);
    });
    assert!(matches!(
        client.execute_cancellable(&request, Some(Duration::from_secs(15)), || {
            cancelled.load(Ordering::Acquire)
        }),
        Err(HelperClientError::Cancelled)
    ));
    let pid: i32 = std::fs::read_to_string(pid_path)
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    // SAFETY: signal 0 only probes the numeric pid and retains no pointer.
    let result = unsafe { libc::kill(pid, 0) };
    assert_eq!(result, -1);
    assert_eq!(
        std::io::Error::last_os_error().raw_os_error(),
        Some(libc::ESRCH)
    );
}

#[test]
fn slice8_gate_63_job_and_helper_lifetime() {
    // Gate 63 is an independently runnable CI entrypoint. Reuse the same
    // destructive assertions so filtering to this exact name still proves
    // foreground group reap and detached runner survival across launcher exit.
    helper_client_cancellation_kills_the_process_group();
    job_launch_policy_is_per_call_bounded_and_defaults_to_primary_workspace();
    job_survives_launching_supervisor_process_exit();
}

#[test]
fn helper_adapter_preserves_gate_and_limits() {
    let helper = BoundedHelper::new(Some(MissingHelper));
    let request = HelperRequest {
        id: "read-1".to_owned(),
        operation: HelperOperation::Read {
            root: "workspace".to_owned(),
            path: "a.txt".to_owned(),
            max_bytes: 0,
        },
    };
    assert!(matches!(
        helper.invoke(&request, BackendGate::Ready, &CancellationToken::default()),
        BackendOutcome::Completed(Err(BackendFailure::Limit(_)))
    ));
    let hold = BackendHold {
        scope: "execute".to_owned(),
        reason: "approval required".to_owned(),
    };
    assert_eq!(
        helper.invoke(
            &request,
            BackendGate::Hold(hold.clone()),
            &CancellationToken::default()
        ),
        BackendOutcome::Hold(hold)
    );
}

#[derive(Clone)]
struct TestLauncher;

impl SandboxedJobLauncher for TestLauncher {
    fn runner_command(
        &self,
        _spec: &JobSpec,
        _effective_policy: &SandboxPolicy,
    ) -> Result<Command, BackendFailure> {
        Ok(Command::new(env!("CARGO_BIN_EXE_tekes-helper")))
    }
}

#[derive(Clone, Default)]
struct CapturingLauncher {
    launches: Arc<Mutex<Vec<(JobSpec, SandboxPolicy)>>>,
}

impl SandboxedJobLauncher for CapturingLauncher {
    fn runner_command(
        &self,
        spec: &JobSpec,
        effective_policy: &SandboxPolicy,
    ) -> Result<Command, BackendFailure> {
        self.launches
            .lock()
            .unwrap()
            .push((spec.clone(), effective_policy.clone()));
        Ok(Command::new(env!("CARGO_BIN_EXE_tekes-helper")))
    }
}

fn test_job_policy(primary_workspace: &std::path::Path) -> JobLaunchPolicy {
    let primary_workspace = std::fs::canonicalize(primary_workspace).unwrap();
    let primary_workspace = primary_workspace.to_string_lossy().into_owned();
    JobLaunchPolicy::new(
        primary_workspace.clone(),
        SandboxPolicy {
            format: 1,
            read_roots: vec![primary_workspace.clone()],
            write_roots: Vec::new(),
            network: NetworkPolicy::Deny,
            allow_process: true,
            scratch: None,
        },
        vec![primary_workspace],
    )
    .unwrap()
}

#[test]
fn job_launch_policy_is_per_call_bounded_and_defaults_to_primary_workspace() {
    let root = tempfile::tempdir().unwrap();
    let jobs = root.path().join("jobs");
    let workspace = root.path().join("workspace");
    let requested = workspace.join("requested");
    let outside = root.path().join("outside");
    std::fs::create_dir_all(&jobs).unwrap();
    std::fs::create_dir_all(&requested).unwrap();
    std::fs::create_dir_all(&outside).unwrap();

    let broker = JobBroker::new(&jobs, 4096).unwrap();
    let policy = test_job_policy(&workspace);
    let launcher = Arc::new(CapturingLauncher::default());
    let accepted = JobSpec {
        program: "/usr/bin/true".to_owned(),
        args: Vec::new(),
        working_directory: None,
        writable_paths: vec!["requested".to_owned()],
        environment: BTreeMap::new(),
    };
    let started = match broker.start(
        "bounded",
        accepted,
        &policy,
        Arc::clone(&launcher),
        BackendGate::Ready,
        &CancellationToken::default(),
    ) {
        BackendOutcome::Completed(Ok(record)) => record,
        other => panic!("bounded job did not start: {other:?}"),
    };
    let workspace = std::fs::canonicalize(workspace)
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let requested = std::fs::canonicalize(requested)
        .unwrap()
        .to_string_lossy()
        .into_owned();
    assert_eq!(
        started.spec.working_directory.as_deref(),
        Some(workspace.as_str())
    );
    assert_eq!(started.spec.writable_paths, [requested.clone()]);

    let launches = launcher.launches.lock().unwrap();
    assert_eq!(launches.len(), 1);
    assert_eq!(launches[0].0, started.spec);
    assert!(launches[0].1.write_roots.contains(&requested));
    assert!(!launches[0].1.write_roots.contains(&workspace));
    assert!(
        launches[0]
            .1
            .write_roots
            .iter()
            .any(|path| path.ends_with("/jobs/bounded"))
    );
    drop(launches);

    let denied = JobSpec {
        program: "/usr/bin/true".to_owned(),
        args: Vec::new(),
        working_directory: None,
        writable_paths: vec![outside.to_string_lossy().into_owned()],
        environment: BTreeMap::new(),
    };
    assert!(matches!(
        broker.start(
            "denied",
            denied,
            &policy,
            Arc::clone(&launcher),
            BackendGate::Ready,
            &CancellationToken::default(),
        ),
        BackendOutcome::Completed(Err(BackendFailure::Denied(_)))
    ));
    assert_eq!(launcher.launches.lock().unwrap().len(), 1);
    assert!(!jobs.join("denied").exists());
}

#[test]
fn job_survives_launching_supervisor_process_exit() {
    let root = tempfile::tempdir().unwrap();
    let status = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("job_supervisor_child")
        .arg("--nocapture")
        .env("TEKES_JOB_TEST_ROOT", root.path())
        .status()
        .unwrap();
    assert!(status.success());
    let broker = JobBroker::new(root.path(), 4096).unwrap();
    let mut terminal = None;
    for _ in 0..300 {
        if let BackendOutcome::Completed(Ok(record)) = broker.status("survivor") {
            if matches!(record.state, JobState::Exited { .. }) {
                terminal = Some(record);
                break;
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    assert!(
        terminal.is_some(),
        "detached runner never recorded completion"
    );
    let tail = std::fs::read_to_string(root.path().join("survivor/stdout.tail")).unwrap();
    assert!(tail.contains("before-exit"));
    assert!(tail.contains("after-exit"));
}

#[test]
fn job_supervisor_child() {
    let Ok(root) = std::env::var("TEKES_JOB_TEST_ROOT") else {
        return;
    };
    let broker = JobBroker::new(&root, 4096).unwrap();
    let spec = JobSpec {
        program: "/bin/sh".to_owned(),
        args: vec![
            "-c".to_owned(),
            "printf before-exit; sleep 0.2; printf after-exit".to_owned(),
        ],
        working_directory: Some(root.clone()),
        writable_paths: Vec::new(),
        environment: BTreeMap::new(),
    };
    let launch_policy = test_job_policy(std::path::Path::new(&root));
    assert!(matches!(
        broker.start(
            "survivor",
            spec,
            &launch_policy,
            Arc::new(TestLauncher),
            BackendGate::Ready,
            &CancellationToken::default(),
        ),
        BackendOutcome::Completed(Ok(_))
    ));
}

#[test]
fn ownerless_job_is_durable_queryable_and_stoppable() {
    let root = tempfile::tempdir().unwrap();
    let broker = JobBroker::new(root.path(), 4096).unwrap();
    let spec = JobSpec {
        program: "/bin/sh".to_owned(),
        args: vec!["-c".to_owned(), "printf started; sleep 30".to_owned()],
        working_directory: Some(root.path().to_string_lossy().into_owned()),
        writable_paths: Vec::new(),
        environment: BTreeMap::new(),
    };
    let launch_policy = test_job_policy(root.path());
    let started = match broker.start(
        "job-1",
        spec,
        &launch_policy,
        Arc::new(TestLauncher),
        BackendGate::Ready,
        &CancellationToken::default(),
    ) {
        BackendOutcome::Completed(Ok(record)) => record,
        other => panic!("unexpected start: {other:?}"),
    };
    assert_eq!(started.state, JobState::Running);
    let reopened = JobBroker::new(root.path(), 4096).unwrap();
    assert!(matches!(
        reopened.status("job-1"),
        BackendOutcome::Completed(Ok(record)) if record.pid == started.pid
    ));
    assert!(matches!(
        reopened.stop("job-1", BackendGate::Ready, &CancellationToken::default()),
        BackendOutcome::Completed(Ok(record)) if record.state == JobState::Stopped
    ));
    thread::sleep(Duration::from_millis(30));
    assert!(matches!(
        reopened.stop("job-1", BackendGate::Ready, &CancellationToken::default()),
        BackendOutcome::Completed(Ok(record)) if record.state == JobState::Stopped
    ));
}

#[test]
fn job_output_tail_is_bounded() {
    let root = tempfile::tempdir().unwrap();
    let broker = JobBroker::new(root.path(), 64).unwrap();
    let spec = JobSpec {
        program: "/usr/bin/printf".to_owned(),
        args: vec!["abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789++++".to_owned()],
        working_directory: Some(root.path().to_string_lossy().into_owned()),
        writable_paths: Vec::new(),
        environment: BTreeMap::new(),
    };
    let launch_policy = test_job_policy(root.path());
    assert!(matches!(
        broker.start(
            "job-tail",
            spec,
            &launch_policy,
            Arc::new(TestLauncher),
            BackendGate::Ready,
            &CancellationToken::default(),
        ),
        BackendOutcome::Completed(Ok(_))
    ));
    for _ in 0..100 {
        if matches!(
            broker.status("job-tail"),
            BackendOutcome::Completed(Ok(ref record)) if matches!(record.state, JobState::Exited { .. })
        ) {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    assert!(
        std::fs::metadata(root.path().join("job-tail/stdout.tail"))
            .unwrap()
            .len()
            <= 64
    );
}

struct SearchFixture;

impl SearchProvider for SearchFixture {
    fn search(
        &self,
        request: &SearchRequest,
        _limits: &HttpLimits,
        cancellation: &CancellationToken,
    ) -> Result<Vec<SearchHit>, BackendFailure> {
        if cancellation.is_cancelled() {
            return Err(BackendFailure::Cancelled);
        }
        Ok(vec![SearchHit {
            title: request.query.clone(),
            url: "https://example.com/".to_owned(),
            snippet: "fixture".to_owned(),
        }])
    }
}

#[test]
fn search_is_credential_conditional_and_bounded() {
    let client = BoundedHttpClient::new(HttpLimits::default()).unwrap();
    let request = SearchRequest {
        call_id: "fixture-call".to_owned(),
        query: "kernel".to_owned(),
        max_results: 5,
        topic: SearchTopic::General,
    };
    assert!(matches!(
        client.search::<SearchFixture>(
            &request,
            None,
            BackendGate::Ready,
            &CancellationToken::default()
        ),
        BackendOutcome::Unavailable(BackendUnavailable { .. })
    ));
    assert!(matches!(
        client.search(
            &request,
            Some(&SearchFixture),
            BackendGate::Ready,
            &CancellationToken::default()
        ),
        BackendOutcome::Completed(Ok(results)) if results.len() == 1
    ));
}

#[test]
fn cancellation_is_terminal_and_does_not_become_success() {
    let token = CancellationToken::default();
    token.cancel();
    let client = BoundedHttpClient::new(HttpLimits::default()).unwrap();
    assert!(matches!(
        client.fetch("https://example.com", BackendGate::Ready, &token),
        BackendOutcome::Completed(Err(BackendFailure::Cancelled))
    ));
}

#[cfg(target_os = "macos")]
#[test]
#[ignore = "requires TEKES_TEST_TOOLCHAIN_ROOT containing bin/python3.14"]
fn declared_toolchain_python_runs_inside_sandbox() {
    let toolchain =
        std::fs::canonicalize(std::env::var("TEKES_TEST_TOOLCHAIN_ROOT").unwrap()).unwrap();
    let workspace = tempfile::tempdir().unwrap();
    let cwd = std::fs::canonicalize(workspace.path()).unwrap();
    let scratch = tempfile::tempdir().unwrap();
    let scratch_path = std::fs::canonicalize(scratch.path()).unwrap();
    let policy = SandboxPolicy {
        format: 1,
        read_roots: vec![
            toolchain.to_string_lossy().into_owned(),
            cwd.to_string_lossy().into_owned(),
        ],
        write_roots: vec![cwd.to_string_lossy().into_owned()],
        network: NetworkPolicy::Deny,
        allow_process: true,
        scratch: Some(scratch_path.to_string_lossy().into_owned()),
    };
    let profile = String::from_utf8(tools::compile_darwin_profile(&policy).unwrap()).unwrap();
    let output = Command::new("/usr/bin/sandbox-exec")
        .args(["-p", &profile])
        .arg(toolchain.join("bin/python3.14"))
        .args([
            "-c",
            "import unittest, threading; print('PYTHON_TOOLCHAIN_OK')",
        ])
        .current_dir(&cwd)
        .env_clear()
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"PYTHON_TOOLCHAIN_OK\n");
    let payload = "large heredoc line\n".repeat(2000);
    let command = format!("cat <<'TEKES_END'\n{payload}TEKES_END\n");
    let heredoc = Command::new("/usr/bin/sandbox-exec")
        .args(["-p", &profile, "/bin/sh", "-c", &command])
        .current_dir(&cwd)
        .env_clear()
        .env("TMPDIR", &scratch_path)
        .output()
        .unwrap();
    assert!(
        heredoc.status.success(),
        "{}",
        String::from_utf8_lossy(&heredoc.stderr)
    );
    assert_eq!(heredoc.stdout, payload.as_bytes());

    let outside = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(outside.path(), b"undeclared sibling contents").unwrap();
    let denied = Command::new("/usr/bin/sandbox-exec")
        .args(["-p", &profile, "/bin/cat"])
        .arg(outside.path())
        .env_clear()
        .output()
        .unwrap();
    assert!(
        !denied.status.success(),
        "ancestor metadata must not permit unrelated file reads"
    );
}

#[test]
fn managed_helper_scratch_survives_clones_and_is_removed_on_last_drop() {
    let policy = SandboxPolicy {
        format: 1,
        read_roots: vec![],
        write_roots: vec![],
        network: NetworkPolicy::Deny,
        allow_process: true,
        scratch: None,
    };
    let client = HelperClient::sandboxed_with_scratch(
        "/unused-helper",
        vec![],
        policy,
        tools::ProbeStatus::Available {
            backend: "test".into(),
            version: "test".into(),
        },
    )
    .unwrap();
    let path = std::path::PathBuf::from(client.scratch_path().unwrap());
    assert!(path.is_dir());
    assert_eq!(
        std::fs::metadata(&path).unwrap().permissions().mode() & 0o077,
        0
    );
    std::fs::write(path.join("temporary"), b"owned scratch").unwrap();
    let pending = client.clone();
    drop(client);
    assert!(path.join("temporary").is_file());
    drop(pending);
    assert!(!path.exists());
}
