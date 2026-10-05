# Chinese short-drama live fixture

`prompt.txt` and `legacy-skill.md` preserve the original input from TekesRuntime
`Tests/TekesRuntimeTests/Live/LiveTests.swift`, pinned revision
`995d8ffeece88c84502e721c5cf9c30d286a17bd`.

`kernel-skill.md` replaces the retired category/name write API with Kernel
apply_patch paths. All task names, dependencies, required outputs and limits
remain unchanged. Run the `drama` scenario of `scripts/run-live-process.py`.
The separate drama and frozen-artifact audits are required for a passing result.
