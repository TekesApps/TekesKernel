use serde_json::{Value, json};
use std::io::Write;
use std::process::{Command, Stdio};

mod support;
use support::warm_first_exec;

#[tokio::test]
async fn parent_invocation_returns_child_result() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("parent.txt"), "proof").unwrap();
    let result = workspace_service::process::invoke(
        std::path::Path::new(env!("CARGO_BIN_EXE_tekes-workspace-service")),
        root.path(),
        "filesList",
        json!({"workspaceId":"w"}),
        std::time::Duration::from_secs(5),
    )
    .await
    .unwrap();
    assert_eq!(result["result"]["entries"][0]["name"], "parent.txt");
}

#[cfg(unix)]
#[tokio::test]
async fn parent_timeout_terminates_and_reaps_child() {
    use std::os::unix::fs::PermissionsExt;
    let root = tempfile::tempdir().unwrap();
    let script = root.path().join("hung-helper");
    // A fixture executable, never a user-supplied shell command in production.
    std::fs::write(
        &script,
        "#!/bin/sh\n[ \"$1\" = --warm ] && exit 0\necho $$ > \"$2/child.pid\"\nexec /bin/sleep 30\n",
    )
    .unwrap();
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o700)).unwrap();
    warm_first_exec(&script);
    let before = std::time::Instant::now();
    let failure = workspace_service::process::invoke(
        &script,
        root.path(),
        "filesList",
        json!({"workspaceId":"w"}),
        std::time::Duration::from_secs(2),
    )
    .await
    .unwrap_err();
    assert_eq!(failure.code, "timeout");
    // The bound proves the timeout fired, not that the machine was idle: a
    // loaded CI host has stretched the 2 s deadline past 5 s before.
    assert!(before.elapsed() < std::time::Duration::from_secs(15));
    let pid = std::fs::read_to_string(root.path().join("child.pid")).unwrap();
    // Reaping is asynchronous to the error return; poll instead of sampling once.
    let reaped_by = std::time::Instant::now() + std::time::Duration::from_secs(5);
    let alive = || {
        Command::new("/bin/kill")
            .args(["-0", pid.trim()])
            .stderr(Stdio::null())
            .status()
            .unwrap()
            .success()
    };
    while alive() && std::time::Instant::now() < reaped_by {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    assert!(
        !alive(),
        "hung child {} survived the parent timeout",
        pid.trim()
    );
}

fn call(root: &std::path::Path, method: &str, request: Value) -> Value {
    let mut child = Command::new(env!("CARGO_BIN_EXE_tekes-workspace-service"))
        .arg("--root")
        .arg(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(&json!({"method":method,"request":request})).unwrap())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn real_child_lists_searches_and_reads_bounded_content() {
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir(root.path().join("folder")).unwrap();
    std::fs::write(root.path().join("folder/proof.txt"), b"alpha\nbeta\n").unwrap();
    let listed = call(root.path(), "filesList", json!({"workspaceId":"w"}));
    assert_eq!(listed["result"]["entries"][0]["kind"], "directory");
    let searched = call(
        root.path(),
        "filesSearch",
        json!({"workspaceId":"w","query":"PROOF"}),
    );
    assert_eq!(searched["result"]["entries"][0]["path"], "folder/proof.txt");
    let read = call(
        root.path(),
        "filesRead",
        json!({"workspaceId":"w","path":"folder/proof.txt"}),
    );
    assert_eq!(read["result"]["content"], "YWxwaGEKYmV0YQo=");
    assert_eq!(read["result"]["revision"].as_str().unwrap().len(), 64);
    let prefix = call(
        root.path(),
        "filesRead",
        json!({"workspaceId":"w","path":"folder/proof.txt","maxBytes":2}),
    );
    assert_eq!(prefix["result"]["truncated"], true);
    assert!(prefix["result"]["revision"].is_null());
    assert_eq!(
        call(
            root.path(),
            "filesRead",
            json!({"workspaceId":"w","path":"folder"})
        )["error"]["code"],
        "not-file"
    );
}

#[test]
fn real_child_rejects_escape_and_bad_requests() {
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    std::fs::write(outside.path().join("secret"), "outside").unwrap();
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(outside.path(), root.path().join("link")).unwrap();
        assert_eq!(
            call(
                root.path(),
                "filesRead",
                json!({"workspaceId":"w","path":"link/secret"})
            )["error"]["code"],
            "path-outside-workspace"
        );
        assert!(
            call(
                root.path(),
                "filesSearch",
                json!({"workspaceId":"w","query":"secret"})
            )["result"]["entries"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }
    for path in ["../secret", "/etc/passwd"] {
        assert_eq!(
            call(
                root.path(),
                "filesRead",
                json!({"workspaceId":"w","path":path})
            )["error"]["code"],
            "path-outside-workspace"
        );
    }
    assert_eq!(
        call(
            root.path(),
            "filesList",
            json!({"workspaceId":"w","limit":0})
        )["error"]["code"],
        "invalid-request"
    );
    assert_eq!(
        call(
            root.path(),
            "filesList",
            json!({"workspaceId":"w","shell":"pwd"})
        )["error"]["code"],
        "invalid-request"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn cancelling_parent_request_terminates_descendant_process() {
    use std::os::unix::fs::PermissionsExt;
    let root = tempfile::tempdir().unwrap();
    let script = root.path().join("descendant-helper");
    std::fs::write(
        &script,
        "#!/bin/sh\n[ \"$1\" = --warm ] && exit 0\n/bin/sleep 30 &\necho $! > \"$2/descendant.pid\"\nwait\n",
    )
    .unwrap();
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o700)).unwrap();
    warm_first_exec(&script);
    let path = root.path().to_path_buf();
    let task = tokio::spawn(async move {
        workspace_service::process::invoke(
            &script,
            &path,
            "filesList",
            json!({"workspaceId":"w"}),
            std::time::Duration::from_secs(20),
        )
        .await
    });
    let pid_file = root.path().join("descendant.pid");
    let start = std::time::Instant::now();
    while !pid_file.exists() {
        assert!(start.elapsed() < std::time::Duration::from_secs(5));
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
    let pid = std::fs::read_to_string(pid_file).unwrap();
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    let start = std::time::Instant::now();
    loop {
        let output = Command::new("ps")
            .args(["-o", "stat=", "-p", pid.trim()])
            .output()
            .unwrap();
        let state = String::from_utf8_lossy(&output.stdout);
        if state.trim().is_empty() || state.trim().starts_with('Z') {
            break;
        }
        assert!(
            start.elapsed() < std::time::Duration::from_secs(5),
            "descendant still executing: {state}"
        );
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
}

#[tokio::test]
async fn complete_file_crosses_preview_and_helper_output_limits() {
    use base64::Engine;
    use sha2::{Digest, Sha256};
    let root = tempfile::tempdir().unwrap();
    let bytes = vec![b'x'; 7 * 1024 * 1024];
    std::fs::write(root.path().join("large.dxf"), &bytes).unwrap();
    // Exercises the parent pipe too: base64 for this file exceeds the old 8 MiB stdout cap.
    let envelope = workspace_service::process::invoke(
        std::path::Path::new(env!("CARGO_BIN_EXE_tekes-workspace-service")),
        root.path(),
        "filesRead",
        json!({"workspaceId":"w", "path":"large.dxf", "maxBytes":0}),
        std::time::Duration::from_secs(20),
    )
    .await
    .unwrap();
    let result = &envelope["result"];
    assert_eq!(result["truncated"], false);
    assert_eq!(result["size"], bytes.len());
    assert_eq!(
        base64::engine::general_purpose::STANDARD
            .decode(result["content"].as_str().unwrap())
            .unwrap(),
        bytes
    );
    assert_eq!(result["revision"], format!("{:x}", Sha256::digest(&bytes)));
}
