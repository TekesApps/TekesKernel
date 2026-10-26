# Legacy historical reads (Kernel #93)

Compatibility contract for Kernel 0.9.8. Release and installed-runtime status are
tracked in the single current-status entry of AgentKit Epic #14.

## Authority and scope

The 2026-10-26 user authorization permits evidence-based read-only compatibility,
not ledger repair, invented tool catalogs, or execution relaxation. The archived
Kernel repository preserves the dev writer sources. Commit
`9b3d024ebc1d087e73394272a9c19eecd83ef82d` (2026-09-06) explicitly replaced
`epoch.tools_digest` with `epoch.tools:{asset,digest}` because a digest cannot
reconstruct a live MCP catalog. All 13 source prefixes recorded by the 12 affected
histories resolve in that archive to writers using the digest-only schema.

A frozen read-only auditor using archived schema at
`cfa7a6be31c9` parsed every complete record and finished the full ledger fold for
all 12 affected originals successfully. This is old-contract validation only,
not current projection, public protocol, UI, or execution acceptance. Original
private manifests, hashes and tool logs are retained under the existing
`web-delivery-20261026` evidence directory. The first auditor compilation failure
is retained separately from its corrected successful run.

The original writers also predate durable `attempt.request` (archive commit
`875edd3c`) and inline terminal usage. Their `tools_digest` values use the
`sha256-` prefix, as established by writer source, not an inferred modern format.
Both unavailable request/catalog facts remain unavailable; no execution request
is reconstructed.

## Boundary

- Ordinary Event decoding, store verification, repair, recovery, request
  reconstruction and every mutating operation retain current strict validation.
- A separate historical decoder recognizes only the evidenced dev lineage and
  genesis format/minimum reader/minimum writer 1. It preserves raw records.
- Historical events carry a marker rejected by the ordinary ledger validator;
  the historical result is not an executable ledger projection.
- The historical path must validate the complete ledger, including old usage
  records and references. Unknown format differences remain failures.
- Only in-memory history projection may consume this result. No canonical
  ledger, endpoint journal, tools asset, provider configuration or cache is
  rewritten to manufacture current-format facts.
- Public reads must report read-only access explicitly; the shared AgentKit
  presentation disables submission for such histories. Execution remains
  blocked server-side even for clients ignoring that indication.
- Pagination must stay bound to the frozen original bytes. Source changes
  invalidate the historical snapshot rather than mixing two observations.

## Delivery gates

1. Positive complete old-format fixtures, raw-byte preservation, malformed and
   unknown-writer negative cases; normal decoder and writer rejection unchanged.
2. Original 12 complete histories validated and projected privately without
   exposing contents or modifying bytes; failure samples retained.
3. Public history and pagination pilot, explicit read-only presentation and
   rejected mutation/recovery; healthy current-format session unaffected.
4. Exact source/program checks, independent consumer and CI before merge/tag.
   Live service replacement only after a fresh active-work check.

No Native UI, physical IME, AX, Swift or SDK work is included. Do not replace a running
service until the replacement passes its applicable gates and active work is checked.
