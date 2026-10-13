# Sandbox oracle

- `policy.canonical.json` is the policy source for all golden profiles.
- `darwin-seatbelt.sb`, `linux-plan.canonical.json` and
  `linux-bubblewrap-plan.canonical.json` are exact compiler
  output.
- `approval.canonical.json` matches only run `run1`, call `c1`, and the
  policy digest named in the fixture.
- `unsorted.invalid.json` and `write-outside.invalid.json` are rejected.
