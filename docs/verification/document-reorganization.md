# Documentation reorganization verification

[System overview](../README.md) · [Document migration map](../history/document-map.md) · [Crate map](../architecture/crates.md)

## Changes made

This change physically split, combined and moved the document bodies and files. The 22 numbered documents at the documentation root were removed;
all 226 top-level content blocks, including introductions, have explicit destinations recorded in
the [section map](../history/document-map.json). The old navigation was replaced without keeping parallel copies of the bodies.

| Previously combined content | Current location |
|---|---|
| Thread definition and complete disk layout | [Domain concepts](../concepts/thread.md) / [Data storage](../data/storage.md) |
| Event fields and reliability details | [Event format](../data/events.md) / [Durability](../data/durability.md) |
| Root execution flow, process startup details and child tasks | [Turn flow](../flows/turn.md) / [Worker mechanics](../runtime/worker.md) / [Delegation](../flows/delegation.md) |
| Projection overview and detailed context rules | Combined in [Model context](../data/context.md); child contexts moved to delegation |
| UI projection and current Client protocol | Combined in [Client and delivery](../interfaces/client.md) |
| Current Client and detailed legacy V2 design | Current interfaces separated from [Historical V2 mappings](../history/client-v2.md) |
| Tool execution and permission details | [Execution mechanics](../runtime/tools.md) / [Permissions and approvals](../runtime/tool-permissions.md) |
| Thousand-line conformance checklist and stage order | [Gates by topic](gates/README.md) / [Historical stage order](../history/gate-rollout.md) |
| Rust constraints, test commands and implementation plan | [Rust constraints](../architecture/rust.md) / [Running tests](running-tests.md) / [Historical rollout](../history/rust-rollout.md) |
| Centrally stored crate documentation | 21 `crates/<name>/docs/README.md` files and adjacent `docs/generated/` directories |

The project overview retains entry points for cross-crate concepts, flows, data and protocols.
Each crate's explanation, public interfaces, module diagrams and function call graphs are maintained beside its source;
the overall dependency graph and global machine inventory remain in `docs/architecture/generated/`.
New files use semantic names instead of old chapter numbers. Protocol versions and gate/decision identities retain their names.

## References and compatibility

- Markdown references are redirected to section anchors in the split documents; whole-file links lead to the corresponding topic entry point.
- The architecture generator now writes both the global directory and each crate's `docs/generated/`;
  `--check` checks every output, and `--links` covers project docs, specs and crate docs.
- The byte digest of `authority-registry.canonical.json` is a runtime compatibility identity.
  It retains historical document paths; the deployment fixture checker uses the section migration map to verify that every new destination exists,
  without changing registry bytes, protocol identities or Rust constants for a documentation move.
- The two continuously maintained live-audit JSON files retain their original paths and are linked from the verification entry point.
  This preserves neither old numbered document bodies nor any new interpretation of their runtime evidence.
- Historical reviews retain the meaning of their stage-specific findings; old chapter shorthand can be traced through the migration map.

## Checks for this change

| Check | Result |
|---|---|
| Original chapter bodies | No numbered documents remain at the documentation root; all 226 content blocks from 22 sources have destinations and valid target section anchors |
| Crate-local documentation | All 21 crates have `docs/README.md` and adjacent generated references |
| File links and Markdown anchors | 66,815 checks passed; original evidence paths in historical reviews are not treated as current navigation contracts |
| Global and crate indexes | 186 generated artifacts; `scripts/code-architecture.py --check` passed after regeneration |
| File references | `scripts/code-architecture.py --links` passed across project docs, specs and crate docs |
| Mermaid | Mermaid 11.12.0 parsed 265 diagrams with no syntax errors |
| Deployment fixtures | `python3 scripts/check-deployment-fixtures.py` passed: 22 cases and 66 case artifacts; registry bytes match the pre-migration version |
| Python / patch | `py_compile` passed for both modified scripts, and task-related `git diff --check` passed |

Index snapshot fingerprint: `b1141c92ee4e98bddda25159bad69c6875d327e63e84eed9b8941424d0301e98`.
The source worktree still contains changes from other tasks. This records the generated snapshot; subsequent source changes require regeneration.
This change adjusted documentation and generator layout. It changed no Rust behavior and neither reran nor expanded the scope of the previous 106
tests; their results remain in the [Thread alignment record](thread-alignment.md).
