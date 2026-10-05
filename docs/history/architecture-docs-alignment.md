# Documentation alignment record

[History entry point](README.md) · [Current system overview](../README.md)

This document preserves its original design or stage scope; see the [document migration map](document-map.md) for current locations of old chapter numbers.

[Architecture entry point](../architecture/README.md) · [Documentation navigation](../README.md)

This record explains how the four-layer architecture documentation task handled existing documents. Paths are stable and Git preserves history.
Its scope is architecture responsibilities, current entry points, inventories, links and version semantics; it does not claim to rerun all normative or release gates.

For the subsequent unification of Thread semantics A, five-layer reading reorganization, code review and tests, see
the [Thread alignment record](../verification/thread-alignment.md). This page preserves the scope of the earlier architecture navigation task;
its completion is not the conclusion of the subsequent semantic audit.

## Per-document disposition

| Document | Main responsibility | Treatment in this task |
|---|---|---|
| README | Project entry point | Added documentation index / chapter 21; declared current V3; limited Slice status to historical milestone descriptions |
| 00 | Conceptual model | Distinguished the resident execution core from the selector; linked the logical diagram to the full process view |
| 01 | Storage layout | Retained V2 filenames; clarified their comments as internal carrier identities |
| 02 | Event model | Retained domain rules; added architecture navigation; centralized concrete Rust declarations in the atlas |
| 03 | Worker lifecycle | Pointed external Client references to V3; added nonfinal/validation stages; retained separate worker-control v1/v2 |
| 04 | Supervisor | Distinguished deployment selector; updated current Client interface and protocol inheritance explanation |
| 05 | Projections | Distinguished V3 transient events from the durable journal; retained carrier data explanations |
| 06 | Tools | Retained ABI, approval and catalog rules; limited Slice-8 "not yet wired" wording to that historical stage |
| 07 | Topology | Clarified "root position" as a logical task tree to avoid conflating it with OS process parentage |
| 08 | Portability | Updated Client protocol references; platform seam design unchanged |
| 09 | Migration | Marked the historical migration baseline; corrected MCP host ownership and later safepoint integration wording |
| 10 | Decisions | Retained D numbers and original decisions; noted that V3 supersedes old public entry points without rewriting historical rationale |
| 11 | Context | Retained model context rules; linked implementation inventories and calls to chapter 21 |
| 12 | Idempotency | Retained domain rules; implementation and IPC evidence live in the corresponding scenario pages |
| 13 | Evolution | Retained external evolution boundaries and plans without inferring that planned capabilities had run |
| 14 | Conformance | Retained gate identities; added V3 spec/test entry points and clarified that this is not a current run report |
| 15 | Client endpoint | Added the main current V3 explanation; explicitly scoped remaining V2 mappings as historical/reused semantics |
| 16 | Implementation plan | Retained the phased plan; distinguished logical component dependencies from the current Cargo inventory |
| 17 | Rust profile | Removed a duplicate directory list missing schedule/installer; linked the generated atlas; corrected the source of lint settings |
| 18 | Parity | Clarified the comparison baseline; old 20-route findings do not define the current V3 interface |
| 19 | Provider parity | Clarified that profile counts belong to the original registry baseline; current inventories must come from source/fixtures |
| 20 | Web Client | Updated /web/remote.mux, logical streams, journal-page and actionable-respond |
| review-history | Historical reviews | Retained original text without requiring historical paths to exist today |
| validation/live audit | Existing dedicated evidence | Retained original text and linked it from navigation without changing completion/acceptance status |

## Concrete drift addressed

1. `spec/session-endpoint-v3.md` already declared public V3 while multiple explanations still called V2 the current entry point.
   New navigation and current descriptions use V3; old storage names and historical gates are not mechanically renamed.
2. The Rust profile's manual inventory omitted current packages; detailed inventories now come from Cargo metadata.
3. "The only resident component" originally described the logical execution core and did not cover the actual selector; the process view makes that layer explicit.
4. Semantic parent/spawn relationships are not OS PPIDs; the two topologies are explained separately.
5. The claim that all production source files contain a forbid attribute did not match the actual Cargo lint configuration; it now references the real manifest.
   Also corrected the nonexistent `platform::darwin` path and the overly broad claim that all public Rust types exclude serde_json::Value.

## Remaining evidence boundaries

- Existing uncommitted validation, provider and MCP changes were included in the source index; their runtime status is established only by the original audits and subsequent tests.
- This documentation task did not rewrite the V2 storage/management semantics explicitly retained in `spec/`.
- Current source diagrams do not certify production installation, Keychain, launchd restart or Client UI behavior.
- Unresolved edges remain in the static call inventory; possible trait targets are not presented as proven direct calls.
