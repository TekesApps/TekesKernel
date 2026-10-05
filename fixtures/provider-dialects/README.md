# Provider dialect profile fixtures

These files are the language-neutral oracle for
`spec/provider-dialect-profiles.md`.

- `profiles.canonical.json` contains fourteen exact target profiles: thirteen
  advertised tuples and one test-only generic codec fixture. For each it holds
  a data-owned endpoint rule and request URL, exact request/multi-turn/SSE
  bytes, normalized terminal, recovery, negative, mismatch and
  session-control-to-digest evidence. Each row's explicit
  `serializer_revision` is repeated in its immutable epoch profile and proof
  digest; DeepSeek Responses also has native reasoning/tool sealed-carrier
  roundtrips.
- `model-capabilities.canonical.json` is the reviewed, SHA-pinned production
  catalog for model-specific reasoning levels and defaults. Array order is
  semantic UI order and must be preserved end to end. Evidence URLs make
  provider-document refreshes explicit without putting credentials or user
  configuration in the fixture.
- `invalid.canonical.json` is the closed fail-closed inventory for route,
  control, framing, structural, and lifecycle invariants; provider vocabulary
  growth is not an invalid case.
- `forward-compatible*.canonical.json` covers structurally valid provider
  vocabulary growth, semantic extraction of known fields, and verbatim sealed
  replay of provider-native fragments that the Kernel does not interpret.

The proof and invalid JSON files are RFC 8785 canonical JSON followed by one LF. The checker
validates bytes and proof completeness without importing production provider
code. Production tests must reproduce these bytes; neither the checker nor a
runtime implementation may rewrite the oracle.

An advertised tuple needs every proof arm in the registry. A family-level
fixture in `provider-runtime/` does not satisfy this corpus.
