#![cfg(target_os = "macos")]
use std::{
    fs,
    process::{Command, Stdio},
};

#[test]
fn model_sandbox_cannot_use_the_local_cli_or_direct_writes_to_change_host_trust() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().canonicalize().unwrap();
    let state = home.join("state");
    let project = home.join("project");
    fs::create_dir(&project).unwrap();
    let authority = profile::FolderAuthorityStore::new(&state);
    authority.request("request", 0, &project).unwrap();
    authority.grant_from_host("grant", 1, "request").unwrap();
    let path = state.join("config/folder-authority.json");
    let before = fs::read(&path).unwrap();
    let policy = tools::SandboxPolicy {
        format: 1,
        read_roots: vec![home.display().to_string()],
        write_roots: vec![home.display().to_string()],
        network: tools::NetworkPolicy::All,
        allow_process: true,
        scratch: None,
        protected_roots: tools::exposed_protected_roots(
            &tools::kernel_protected_roots(&state),
            &[home.display().to_string()],
        ),
    };
    let rules = String::from_utf8(tools::compile_darwin_profile(&policy).unwrap()).unwrap();
    let run = |args: &[&str]| {
        Command::new("/usr/bin/sandbox-exec")
            .args(["-p", &rules])
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap()
            .success()
    };
    assert!(run(&[
        "/bin/sh",
        "-c",
        "printf allowed > \"$1\"",
        "fixture",
        home.join("sibling").to_str().unwrap()
    ]));
    assert!(!run(&[
        env!("CARGO_BIN_EXE_tekes-supervisor"),
        "--folder-trust",
        "request",
        state.to_str().unwrap(),
        project.to_str().unwrap(),
        "nested-client",
        "2"
    ]));
    assert!(!run(&[
        "/bin/sh",
        "-c",
        "printf corrupt > \"$1\"",
        "fixture",
        path.to_str().unwrap()
    ]));
    assert_eq!(fs::read(&path).unwrap(), before);
    assert!(authority.status(&project).unwrap().trusted);
}
