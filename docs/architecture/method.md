# Architecture evidence and maintenance

[Architecture entry point](README.md) · [Generated index](generated/index.md)

## Evidence levels

- **Source-confirmed**: the relationship is present in manifests, types, function bodies or wiring, with links to the relevant source.
- **Statically resolved direct call**: an AST call expression identifies a unique function definition through an explicit path, import, re-export, or the current type's `Self` / `self`.
  This is a syntactic resolution result, not proof from compiler type checking.
- **Unresolved**: the index cannot determine the receiver type, trait implementation, callback or macro expansion. The original expression is retained without inventing an edge.
- **Runtime-verified**: requires separate test or runtime evidence. These static diagrams do not supply that proof.

## Scope and limits

`cargo metadata --no-deps --offline` obtains packages, compilation targets and direct dependencies.
Tree-sitter parses each package's `src`, `tests`, `examples`, `benches` and `build.rs`.
Declarations of every visibility enter the inventory. A pub type is not a function call, and pub(crate) is not the only entry point for cross-module calls.

Each `src` file has declarations, imports/exports, module declarations, a call-site table and function graphs grouped by 20 callers.
Test sources, examples and build targets are marked separately in the package table and machine inventory. Function graphs do not expand test functions;
test calls remain in the call-site data. `cfg` is not evaluated, so platform branches may appear together.

Module paths inferred from files are navigation aids. Inline modules are listed separately; `#[path]` redirection, reuse of one file across targets,
and macro-generated modules may prevent reconstruction of the compiler's final module tree. The Cargo target list defines compilation targets.

The resolver performs neither Rust type inference nor monomorphization and does not force globally identical method names onto one implementation. External dependency internals are not expanded;
constructors, function pointers, trait/dyn dispatch and methods with unknown receiver types remain unresolved call sites.
Macro invocations are counted separately; Rust code inside macro token trees is not treated as expanded.
Calls inside closures are attributed to the enclosing function; this does not imply immediate closure execution. Actual ordering across `.await`, spawn and channels requires the flow pages.
A function with no direct references is not necessarily dead code: route registrations, trait implementations and callbacks may invoke it indirectly.

Thus, "complete" means coverage of input files, declarations and parseable call expressions; it **does not claim to reconstruct every runtime call target**.
Cross-process edges must be confirmed manually through `Command`, descriptors, protocol codecs and wiring; Cargo dependencies cannot establish them.

## Reproduce

From the repository root, install the documentation dependencies in an isolated environment:

```sh
python3 -m venv /tmp/tekes-architecture-venv
/tmp/tekes-architecture-venv/bin/pip install -r scripts/architecture-requirements.txt
/tmp/tekes-architecture-venv/bin/python scripts/code-architecture.py
/tmp/tekes-architecture-venv/bin/python scripts/code-architecture.py --check
/tmp/tekes-architecture-venv/bin/python scripts/code-architecture.py --links
```

Requires Python 3.11+, Cargo and the pinned Rust toolchain. Generation calls no model, starts no product process and runs no tests.
Source files, Cargo configuration, the generator and its dependency manifest have SHA-256 fingerprints; `--check` compares all generated artifacts.
Generation also checks extraction against declaration/call-expression counts from the entire AST and rejects duplicate symbol IDs or parse failures;
complete counts do not imply complete dynamic-call resolution. Literal `include!` in test files propagates test marking without executing macros.
`--links` checks local links and source line ranges in the project README, project docs, specs and crate docs; historical review records are not treated as current path contracts.

## Change checklist

| Code change | Documentation to update |
|---|---|
| Package / target / dependency / pub interface changes | Regenerate the atlas and review the corresponding crate guide |
| Process lifecycle, descriptor or communication changes | Processes page and related flow pages |
| Core control flow, waiting or error-handling changes | Scenario sequence diagrams and their source references |
| Public protocol changes | Establish spec version and compatibility first, then update interfaces/, architecture entry points, README and diagrams |
| New tests or acceptance results | Relevant conformance / audit records; static graphs do not replace acceptance evidence |

Ordinary source path changes can be discovered automatically; responsibilities and business ordering still require review. New architecture documentation does not change the specification: if an implementation violates the current spec,
record and resolve the discrepancy separately instead of changing the specification to match incorrect behavior.

## Documentation and generated artifact locations

Write explanatory prose, navigation labels and diagram labels in English across
the project, specs and crate guides. Preserve protocol identifiers, source symbols,
commands and quoted test inputs exactly; a non-English input literal in a source
excerpt or audit is evidence, not untranslated explanatory prose. Generated index
templates already use English and retain source expressions verbatim.

Project concepts, flows, data and interface documentation live under the corresponding topics in `docs/`. Crate-specific content lives in
`crates/<name>/docs/README.md`, with generated module/API/function references in the adjacent
`docs/generated/`. Cross-crate dependency graphs, the overall index and the complete inventory remain in
`docs/architecture/generated/`. One generator invocation updates both locations; `--check` validates both.

When splitting document bodies, update section anchors, project/cross-crate references and document references in fixtures together;
a documentation move does not change Rust behavior. Path migrations are recorded in the [historical map](../history/document-map.md).
