use base64::Engine as _;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn request(path: &str, before: &[u8], after: &[u8]) -> Value {
    json!({"workspaceId":"w","path":path,"content":base64::engine::general_purpose::STANDARD.encode(after),
        "expectedRevision":format!("{:x}",Sha256::digest(before))})
}
fn start(root: &Path, authority: &Path, request: Value) -> std::process::Child {
    let mut child = Command::new(env!("CARGO_BIN_EXE_tekes-workspace-service"))
        .arg("--root")
        .arg(root)
        .arg("--authority-file")
        .arg(authority)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(&json!({"method":"filesWrite","request":request})).unwrap())
        .unwrap();
    child
}
fn result(child: std::process::Child) -> Value {
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    serde_json::from_slice(&output.stdout).unwrap()
}
fn authority(directory: &Path, allowed: bool) -> std::path::PathBuf {
    let path = directory.join("authority.json");
    std::fs::write(
        &path,
        serde_json::to_vec(&json!({"stateRoot":directory.join("state"),
        "allowedOperations":[],"allowedRepositoryRoots":[],"allowFileWrites":allowed}))
        .unwrap(),
    )
    .unwrap();
    path
}

#[test]
fn file_saves_require_grant_and_current_revision_and_preserve_mode() {
    use std::os::unix::fs::PermissionsExt;
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let path = root.path().join("proof");
    std::fs::write(&path, b"before\n").unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o640)).unwrap();
    let payload = request("proof", b"before\n", "saved 中文\r\n".as_bytes());
    let denied = result(start(
        root.path(),
        &authority(state.path(), false),
        payload.clone(),
    ));
    assert_eq!(denied["error"]["code"], "permission-denied");
    let grant = authority(state.path(), true);
    let saved = result(start(root.path(), &grant, payload.clone()));
    assert_eq!(saved["result"]["size"], "saved 中文\r\n".len());
    assert_eq!(
        std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o640
    );
    let stale = result(start(root.path(), &grant, payload));
    assert_eq!(stale["error"]["code"], "revision-conflict");
    assert_eq!(std::fs::read(&path).unwrap(), "saved 中文\r\n".as_bytes());
    let outside = result(start(
        root.path(),
        &grant,
        request("../escape", b"", b"bad"),
    ));
    assert_eq!(outside["error"]["code"], "path-outside-workspace");
}

#[test]
fn concurrent_helpers_and_aliases_have_one_revision_winner() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("proof"), b"before").unwrap();
    std::os::unix::fs::symlink("proof", root.path().join("alias")).unwrap();
    let grant = authority(state.path(), true);
    let a = start(root.path(), &grant, request("proof", b"before", b"first"));
    let b = start(root.path(), &grant, request("alias", b"before", b"second"));
    let results = [result(a), result(b)];
    assert_eq!(
        results.iter().filter(|r| r.get("result").is_some()).count(),
        1
    );
    assert_eq!(
        results
            .iter()
            .filter(|r| r["error"]["code"] == "revision-conflict")
            .count(),
        1
    );
}

#[tokio::test]
async fn parent_transports_four_megabyte_save_without_truncation() {
    let root = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("proof"), b"before").unwrap();
    let grant = authority(state.path(), true);
    let content = vec![0x81; 4 * 1024 * 1024];
    let result = workspace_service::process::invoke_with_authority(
        Path::new(env!("CARGO_BIN_EXE_tekes-workspace-service")),
        root.path(),
        Some(&grant),
        "filesWrite",
        request("proof", b"before", &content),
        std::time::Duration::from_secs(10),
    )
    .await
    .unwrap();
    assert_eq!(result["result"]["size"], content.len());
    assert_eq!(std::fs::read(root.path().join("proof")).unwrap(), content);
}
