//! Crate-local test fixtures (`crates/*/tests/fixtures/**`) are shipped in a public repository, so
//! they must not carry a machine-specific path or a deployment's product tool names (CONTRIBUTING,
//! pull request checklist item 6). The root `fixtures/` oracle corpus predates this guard and is
//! not scanned; no crate-local fixture directory existed when the guard was added.
use std::path::{Path, PathBuf};

const FORBIDDEN: &[&str] = &["/private/tmp/", "/Users/", "mcp__tekes_"];

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn files(directory: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(directory).expect("read_dir") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            files(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// Files under `root` whose bytes contain a forbidden marker, as paths relative to `root`.
fn offenders(root: &Path, fixture_roots: &[PathBuf]) -> Vec<String> {
    let mut found = Vec::new();
    for fixture_root in fixture_roots {
        let mut all = Vec::new();
        files(fixture_root, &mut all);
        for path in all {
            let bytes = std::fs::read(&path).expect("fixture bytes");
            let text = String::from_utf8_lossy(&bytes);
            if FORBIDDEN.iter().any(|marker| text.contains(marker)) {
                found.push(
                    path.strip_prefix(root)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
    }
    found.sort();
    found
}

fn crate_fixture_roots(root: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    for krate in std::fs::read_dir(root.join("crates")).expect("crates") {
        let fixtures = krate.expect("crate").path().join("tests/fixtures");
        if fixtures.is_dir() {
            roots.push(fixtures);
        }
    }
    roots
}

#[test]
fn crate_test_fixtures_hold_no_machine_paths_or_product_tool_names() {
    let root = workspace_root();
    let found = offenders(&root, &crate_fixture_roots(&root));
    assert!(
        found.is_empty(),
        "fixtures with a machine path or product tool name:\n{}",
        found.join("\n")
    );
}

#[test]
fn the_guard_detects_each_forbidden_marker() {
    let directory = tempfile::tempdir().expect("directory");
    for (index, marker) in FORBIDDEN.iter().enumerate() {
        std::fs::write(
            directory.path().join(format!("bad-{index}.json")),
            format!("{{\"x\":\"{marker}y\"}}\n"),
        )
        .expect("bad fixture");
    }
    std::fs::write(directory.path().join("good.json"), b"{\"x\":\"ok\"}\n").expect("good");
    let found = offenders(directory.path(), &[directory.path().to_path_buf()]);
    assert_eq!(found.len(), FORBIDDEN.len());
    assert!(found.iter().all(|name| name.starts_with("bad-")));
}
