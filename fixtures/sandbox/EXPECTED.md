# Sandbox oracle

- `policy.canonical.json` is the policy source for both golden profiles.
- `darwin-seatbelt.sb` and `linux-plan.canonical.json` are exact compiler
  output.
- `approval.canonical.json` matches only run `run1`, call `c1`, and the
  policy digest named in the fixture.
- `unsorted.invalid.json` and `write-outside.invalid.json` are rejected.
