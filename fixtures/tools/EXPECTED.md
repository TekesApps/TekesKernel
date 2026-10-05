# builtin-tools fixture expectations

- `builtin-tools.canonical.json` is one RFC-8785 canonical JSON object plus
  one final LF.
- `format` is exactly `1`; objects and classification enums are closed by
  `spec/builtin-tools.md`.
- The runtime manifest has only `format` and `tools`; migration provenance is
  deliberately split into `builtin-tool-migration.canonical.json`.
- `tools` is strictly UTF-8 byte-sorted by unique `name` and contains the
  25 callable fixed names. Entries have no owner/source, `exposure`, or
  `schema` field; visibility is derived from `availability`.
- `arguments` preserves the model schema property order and contains every
  top-level property exactly once. Full nested schemas are materialized by the
  typed implementation and checked by canonical digest; this inventory does
  not carry a second, potentially divergent copy of every nested schema node.
- `builtin-tools.digest.canonical.json` locks `tools_digest` to
  `sha256-3065d0e7dbfdc01b0c564c388cc1e4c28bc19d9a362e23daf215674780946e08`:
  SHA-256 over the 3,752 RFC-8785 canonical bytes of the runtime manifest's
  exact `tools` array, with no trailing LF. All five entry fields participate;
  mutating any one must change the digest.
- `builtin-tool-migration.canonical.json` pins the two legacy repository
  baselines and explicitly replaces ten old names: six Git wrappers by
  `shell`, `compact` by host compaction, `goal_completed` by `set_goal_state`,
  and `note`/`recall` by external memory. `write` and the new literal `edit` are direct builtins.
  Its evidence inventory includes every pinned
  source file the parity parser reads, including the Runtime context-name and
  skill-explorer-name declarations; the check never fills an omitted evidence
  path from the live checkout.
- Conditional and role-only names remain in the manifest even when absent from
  a particular model request; compatibility-only aliases and tombstones do not.
- Local-manifest, plugin, and MCP tool instances are dynamic and therefore do
  not appear in this fixed-name fixture.
- The private migration parity check (it needs the legacy repositories)
  independently extracts the
  two legacy inventories, applies every explicit migration disposition, then
  checks the remaining names and every top-level argument order against this
  one catalog; gate 43 additionally owns
  closed nested-shape and schema-digest conformance.
- `first-party-tools.canonical.json` separately inventories the 12 tools
  shipped by the optional `com.tekes.computer-use` plugin at the migration
  baseline. They remain dynamic/plugin-owned and never enter the fixed-name
  manifest merely because their source is shipped in the AppServer tree.
