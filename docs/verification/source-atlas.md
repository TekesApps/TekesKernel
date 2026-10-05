# Architecture documentation verification

[Verification entry point](README.md) · [Subsequent documentation reorganization and index refresh](document-reorganization.md)

[Architecture entry point](../architecture/README.md) · [Maintenance method](../architecture/method.md)

This record covers the documentation work on 2026-09-04, not runtime acceptance of the Kernel product.
Generation input fingerprint: `270ec6c0073c73585b54b80c2b665935875151300df05e73a12ad232c9456b2b`.
For subsequent source changes, regenerate the inventory locally with `python3 scripts/code-architecture.py` (`docs/architecture/generated/inventory.json`, not tracked in Git); this record preserves the results of that validation run.

For subsequent Thread A documentation reorganization and code tests, see the [Thread alignment record](thread-alignment.md).
References to "this change" below mean the initial architecture index task, not that no Rust tests were run later.

| Check | Result |
|---|---|
| Cargo package coverage | 21/21; distinguishes lib/bin/test/example/build targets and normal/dev/build dependencies |
| Rust file coverage | 202/202; matches the actual set of `.rs` files under crates |
| Declaration coverage | 6,034 symbols, including 4,381 function implementations; matches the complete AST counts by category |
| Call-site coverage | 63,851 call expressions; extracted counts match the complete AST |
| Direct call resolution | 9,954; remaining calls retain explicit classifications without invented targets |
| Generated artifact consistency | 186 files; `--check` passed after regeneration |
| Call-resolution spot checks | Verified schema's cross-module re-exports, private method calls and store → schema calls |
| Mermaid | Mermaid 11.12.0 parsed 263 diagrams with no syntax errors |
| Local links | File links and source line ranges passed for the README and current docs; historical review records retain historical-path treatment |
| Documentation diff | `git diff --check -- README.md docs` passed |
| Python generator | `py_compile` passed |

Unresolved categories include 38,136 method calls requiring receiver types, 15,748 external/constructor/callback calls,
and 13 calls ambiguous under cfg or overloads. They remain in the call-site tables. Complete coverage does not mean complete compiler-level target resolution.

This task ran no Rust builds/tests, installed no product, called no real provider, and performed no UI or restart acceptance checks.
The Mermaid result establishes syntax validity, not visual acceptance in the target application.
Existing Rust, spec, fixture and dedicated audit changes remain maintained by their original tasks; this task edited only documentation and architecture index scripts.
