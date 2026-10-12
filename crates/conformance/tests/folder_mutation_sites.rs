//! T6c (ADDENDUM-04 K1, E831 E5): every place in non-test Kernel code that creates, renames,
//! links, copies or removes a filesystem entry is enumerated and classified. A new site, or a
//! stale allow-list entry, fails the build, so nobody can add a way for a thread folder to
//! appear, move or vanish without passing the `store` chokepoints that announce it.
//!
//! The scan parses every `crates/*/src/**/*.rs` file with `syn` and skips exactly the items
//! carrying `#[cfg(test)]` (and `#[test]` functions) -- not "everything after the first
//! `mod tests`". `UPDATE_FOLDER_MUTATION_SITES=1` prints a fresh list instead of failing.
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use syn::visit::Visit;

const CALLEES: &[&str] = &[
    "rename",
    "renameat",
    "renameatx_np",
    "create_dir",
    "create_dir_all",
    "hard_link",
    "link",
    "linkat",
    "symlink",
    "symlinkat",
    "copy",
    "clonefile",
    "clonefileat",
    "copy_file_range",
    "remove_dir_all",
    "remove_dir",
    "remove_file",
    "unlink",
    "unlinkat",
    "rmdir",
    "DirBuilder::create",
];

#[derive(Default)]
struct Scan {
    file: String,
    functions: Vec<String>,
    /// `(file, fn, callee)` -> number of calls
    sites: BTreeMap<(String, String, String), usize>,
    /// callees seen per function (for the "must announce" rule)
    seen: BTreeMap<(String, String), BTreeSet<String>>,
}

fn is_test_attr(attr: &syn::Attribute) -> bool {
    if attr.path().is_ident("test") {
        return true;
    }
    if attr.path().is_ident("cfg") {
        let text = attr.meta.require_list().map(|list| list.tokens.to_string());
        return text.is_ok_and(|tokens| {
            let words = tokens
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .collect::<Vec<_>>();
            words.contains(&"test") && !words.contains(&"not")
        });
    }
    false
}

impl Scan {
    fn record(&mut self, callee: &str) {
        let function = self
            .functions
            .last()
            .cloned()
            .unwrap_or_else(|| "<item>".into());
        *self
            .sites
            .entry((self.file.clone(), function.clone(), callee.to_owned()))
            .or_default() += 1;
        self.seen
            .entry((self.file.clone(), function))
            .or_default()
            .insert(callee.to_owned());
    }

    fn note(&mut self, callee: &str) {
        let function = self
            .functions
            .last()
            .cloned()
            .unwrap_or_else(|| "<item>".into());
        self.seen
            .entry((self.file.clone(), function))
            .or_default()
            .insert(callee.to_owned());
    }
}

impl<'ast> Visit<'ast> for Scan {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        if !node.attrs.iter().any(is_test_attr) {
            syn::visit::visit_item_mod(self, node);
        }
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        if node.attrs.iter().any(is_test_attr) {
            return;
        }
        self.functions.push(node.sig.ident.to_string());
        syn::visit::visit_item_fn(self, node);
        self.functions.pop();
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        if node.attrs.iter().any(is_test_attr) {
            return;
        }
        self.functions.push(node.sig.ident.to_string());
        syn::visit::visit_impl_item_fn(self, node);
        self.functions.pop();
    }

    fn visit_trait_item_fn(&mut self, node: &'ast syn::TraitItemFn) {
        if node.attrs.iter().any(is_test_attr) {
            return;
        }
        self.functions.push(node.sig.ident.to_string());
        syn::visit::visit_trait_item_fn(self, node);
        self.functions.pop();
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        if !node.attrs.iter().any(is_test_attr) {
            syn::visit::visit_item_impl(self, node);
        }
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = &*node.func {
            let segments = path
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>();
            if let Some(last) = segments.last() {
                if CALLEES.contains(&last.as_str()) {
                    self.record(last);
                }
                if matches!(last.as_str(), "emit" | "move_thread_folder" | "retire_to") {
                    self.note(last);
                }
            }
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let method = node.method.to_string();
        // `fs::DirBuilder::new().recursive(true).create(path)`: a `create` whose receiver chain
        // mentions DirBuilder. `std::fs::File::create` is a path call and is not matched.
        if method == "create" {
            let receiver = quote_tokens(&node.receiver);
            if receiver.contains("DirBuilder") {
                self.record("DirBuilder::create");
            }
        }
        if matches!(
            method.as_str(),
            "remove_dir_all" | "remove_file" | "remove_dir"
        ) {
            self.record(&method);
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

fn quote_tokens(expr: &syn::Expr) -> String {
    use syn::__private::ToTokens;
    expr.to_token_stream().to_string()
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn rust_files(directory: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(directory).expect("read_dir") {
        let path = entry.expect("entry").path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

fn scan_workspace() -> Scan {
    let root = workspace_root();
    let mut files = Vec::new();
    for krate in std::fs::read_dir(root.join("crates")).expect("crates") {
        let src = krate.expect("crate").path().join("src");
        if src.is_dir() {
            rust_files(&src, &mut files);
        }
    }
    files.sort();
    let mut scan = Scan::default();
    for path in files {
        let relative = path
            .strip_prefix(&root)
            .expect("under root")
            .to_string_lossy()
            .into_owned();
        // `#[cfg(test)] mod tests;` pulls in a file that is test code by construction.
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        if relative.ends_with("/tests.rs")
            || relative.contains("/tests/")
            || stem == "tests"
            || stem.ends_with("_tests")
        {
            continue;
        }
        let source = std::fs::read_to_string(&path).expect("source");
        let Ok(file) = syn::parse_file(&source) else {
            panic!("{relative} does not parse");
        };
        scan.file = relative;
        scan.visit_file(&file);
    }
    scan
}

fn allow_list() -> BTreeMap<(String, String, String), (usize, String)> {
    let text = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/folder_mutation_sites.txt"),
    )
    .expect("allow-list");
    let mut map = BTreeMap::new();
    for line in text
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let parts = line.split('|').collect::<Vec<_>>();
        assert_eq!(parts.len(), 5, "bad allow-list line: {line}");
        map.insert(
            (
                parts[0].to_owned(),
                parts[1].to_owned(),
                parts[2].to_owned(),
            ),
            (parts[3].parse().expect("count"), parts[4].to_owned()),
        );
    }
    map
}

#[test]
fn every_filesystem_mutation_site_is_enumerated_and_classified() {
    let scan = scan_workspace();
    if std::env::var_os("UPDATE_FOLDER_MUTATION_SITES").is_some() {
        for ((file, function, callee), count) in &scan.sites {
            println!("{file}|{function}|{callee}|{count}|TODO");
        }
        return;
    }
    let allowed = allow_list();
    let mut problems = Vec::new();
    for (key, count) in &scan.sites {
        match allowed.get(key) {
            None => problems.push(format!("new site: {}|{}|{} x{count}", key.0, key.1, key.2)),
            Some((expected, _)) if expected != count => problems.push(format!(
                "count changed: {}|{}|{} is {count}, allow-list says {expected}",
                key.0, key.1, key.2
            )),
            Some(_) => {}
        }
    }
    for key in allowed.keys() {
        if !scan.sites.contains_key(key) {
            problems.push(format!(
                "stale allow-list entry: {}|{}|{}",
                key.0, key.1, key.2
            ));
        }
    }
    // `publish` and `retire` sites must announce (emit / chokepoint) in the same function.
    for ((file, function, callee), (_, class)) in &allowed {
        if matches!(class.as_str(), "publish" | "retire") {
            let seen = scan.seen.get(&(file.clone(), function.clone()));
            let announces = seen.is_some_and(|seen| {
                seen.contains("emit")
                    || seen.contains("move_thread_folder")
                    || seen.contains("retire_to")
            });
            if !announces {
                problems.push(format!(
                    "{file}|{function}|{callee} is classified {class} but does not announce a FolderEvent"
                ));
            }
        }
    }
    assert!(
        problems.is_empty(),
        "folder mutation sites changed (classify new sites in tests/folder_mutation_sites.txt):\n{}",
        problems.join("\n")
    );
}

/// Shell-outs can move folders without any `rename` call: there must be none.
#[test]
fn no_kernel_code_shells_out_to_move_or_copy_files() {
    let root = workspace_root();
    for krate in ["store", "endpoint", "supervisor", "profile"] {
        let mut files = Vec::new();
        rust_files(&root.join("crates").join(krate).join("src"), &mut files);
        for path in files {
            let source = std::fs::read_to_string(&path).expect("source");
            for forbidden in [
                "Command::new(\"mv\")",
                "Command::new(\"cp\")",
                "Command::new(\"rm\")",
                "Command::new(\"rsync\")",
            ] {
                assert!(
                    !source.contains(forbidden),
                    "{}: {forbidden}",
                    path.display()
                );
            }
        }
    }
}

/// The scanner itself: production code after a test module is still scanned, `#[cfg(test)]`
/// and `#[test]` items are not, and the extended callee list is recognised.
#[test]
fn the_scanner_skips_test_items_by_parsing_and_sees_all_callees() {
    let source = r#"
        #[cfg(test)]
        mod tests { fn a() { std::fs::rename("a", "b"); } }
        fn production_after_tests() {
            std::fs::rename("a", "b");
            libc::renameat(1, p, 1, q);
            libc::renameatx_np(1, p, 1, q, 0);
            std::fs::DirBuilder::new().recursive(true).create("d");
            std::fs::remove_dir_all("d");
            x.remove_file();
            File::create("not-a-folder-mutation");
        }
        #[test]
        fn a_test_fn() { std::fs::rename("a", "b"); }
        #[cfg(not(test))]
        fn not_test() { std::fs::create_dir("d"); }
        #[cfg(all(test, target_os = "macos"))]
        fn only_in_tests() { std::fs::create_dir("d"); }
        impl S { fn method(&self) { std::fs::hard_link("a", "b"); } }
    "#;
    let mut scan = Scan {
        file: "synthetic.rs".into(),
        ..Scan::default()
    };
    scan.visit_file(&syn::parse_file(source).unwrap());
    let found = scan
        .sites
        .iter()
        .map(|((_, function, callee), count)| format!("{function}:{callee}:{count}"))
        .collect::<Vec<_>>();
    assert_eq!(
        found,
        vec![
            "method:hard_link:1",
            "not_test:create_dir:1",
            "production_after_tests:DirBuilder::create:1",
            "production_after_tests:remove_dir_all:1",
            "production_after_tests:remove_file:1",
            "production_after_tests:rename:1",
            "production_after_tests:renameat:1",
            "production_after_tests:renameatx_np:1",
        ]
    );
}
