# Kernel workspace service Client integration

Status: response compatibility verified; Client endpoint adaptation and UI acceptance
remain open. This document belongs to Kernel. It does not authorize changes to Tekes.

## Native carrier

Use the authenticated Kernel host address and its existing bearer credential.
Do not obtain a credential from model output or put it in logs.

POST `/api/tekesWorkspace.filesList` with this JSON shape:

```json
{
  "type": "client-request",
  "rpcId": "a-new-uuid-for-this-logical-operation",
  "method": "tekesWorkspace.filesList",
  "payload": {"workspaceId": "registered-workspace-id", "path": ""}
}
```

Success values are under `result.value`, with `result.ok` true. Errors have
`result.ok` false and `result.error`; HTTP authentication errors also need handling.
A transport retry of a mutation must retain its original RPC ID and exact payload.
An `outcome-unknown` error requires refreshing repository state, not issuing a new
mutation automatically. Allow at least 135 seconds for a Git push response.

The current DSH adapter uses slash method names and an `args.request` wrapper.
Changing only its base URL will not connect it to this native carrier.

## Endpoint protocol mapping

The Client runtime dynamically requires `SessionWorkspaceServiceEndpoint` and
`SessionGitServiceEndpoint`. Its current DSH implementation supplies both. A Kernel
endpoint must supply those same protocols using the native host/session identities.
An additional server alias alone cannot make an unrelated Client endpoint conform.

All methods below are prefixed `tekesWorkspace.`. Payloads include `workspaceId`
except `capabilities`, whose payload is `{}`. Unknown fields are rejected.

| Method | Other request fields |
| --- | --- |
| capabilities | none |
| filesList | path, limit |
| filesSearch | query, limit |
| filesRead | path, maxBytes |
| gitRepository / gitStatus / gitBranches | repositoryPath |
| gitLog | repositoryPath, limit, offset, head |
| gitDiff | repositoryPath, scope, paths |
| gitChangesNavigator | includeClean |
| gitCommit | repositoryPath, message, paths |
| gitPush | repositoryPath, remote |
| gitChangeBranch | repositoryPath, operation, name |
| turnChanges | sessionId, turnId |

Paths are relative to the registered server workspace. Optional repositoryPath
selects a repository within it. Without that field, a sole child repository can be
selected automatically; multiple candidates return `repository-selection-required`.
Git mutations are denied unless the server operator grants the named operation and
exact repository root in `workspace-service/git-authority.json`. Capabilities return
`allowedGitOperations`; method presence alone is not mutation authorization.

Files covers browse/search/preview. `filesWrite` is not advertised by this delivery;
the Client protocol's unsupported-write default must remain visible. Turn changes
uses recorded edit facts, not a Git working-tree baseline. Large or uncertain edits
return unknown counts. Display only complete net counts; do not turn null into zero.
The internal prepare/record/abort methods are not public Client operations.

## Existing verification and remaining acceptance

Run `scripts/verify-workspace-carrier.py --server PATH --helper PATH --report PATH`
with the Kernel production carrier harness and current helper. It writes both the
check report and an adjacent `.responses.json` file. Then run:

```text
python3 scripts/verify-workspace-client-values.py --client ABSOLUTE_TEKES_CHECKOUT --responses ABSOLUTE_RESPONSES_JSON
```

The second script reads actual Client DTO declarations and compiles them in a
private temporary directory. It requires all fourteen response methods and checks
Swift decoding. Neither script changes the Client checkout or verifies UI routing.

Once the Kernel Client endpoint exists, reuse the same disposable-repository
scenarios in Files, repository management and Changes UI. Verify thread switching
shows only the selected session's latest turn and suppresses unknown counts.
Exercise denied operations and interrupted mutation recovery as well as successes.
For local acceptance, launch the Kernel the way the application does
([Application-owned launch](builtin-launch.md)); the signed-installer path
previously used here is retired.
