# Shared Workspace Service Endpoint

Kernel and the DSH Workspace plugin implement the same `tekesWorkspace.*` HTTP envelope and Client DTOs. Client has one stateless `HTTPWorkspaceServiceEndpoint`; it does not maintain business projections or edit aggregates. Client runtime/Mirror own observation and caching, while the service owns files, Git execution and durable turn edit facts.

The shared Client contract and executable acceptance probe live in the sibling Tekes repository at `docs/workspace-service-endpoint.md` and `scripts/workspace-service/`. Pass that compiled probe to `scripts/verify-workspace-carrier.py --swift-wse-probe <executable>` to verify the actual Kernel carrier and helper with exactly the Swift implementation used against DSH.

## File saves

`tekesWorkspace.filesWrite` accepts `workspaceId`, workspace-relative `path`, canonical base64 `content`, and `expectedRevision` (SHA-256 of the complete previous content). It modifies an existing regular file only. The maximum content is 4 MiB. The helper's input allowance is 8 MiB for this method to accommodate JSON/base64; other helper methods retain the 1 MiB input limit.

The operator-managed `workspace-service/git-authority.json` supports `allowFileWrites: true` in addition to its existing Git grants. Missing/false denies all file saves. The server registry selects the workspace root; request JSON cannot override it. `capabilities.methods` includes filesWrite only while the grant is enabled, and `mutations` reflects either file or Git grants.

Saves use a cross-process lock keyed by the resolved path, a pinned parent directory descriptor, revision rechecks, a sibling temporary file, mode preservation, file sync, atomic rename and directory sync. A stale revision returns `revision-conflict`. Recovered mutations without a recorded outcome return `outcome-unknown` and are not automatically executed again. UI file saves do not create agent turn edit facts.

`process_write` tests default denial, stale revisions, mode preservation, path escape rejection, concurrent saves through aliases and the full 4 MiB parent/helper boundary. The shared Swift HTTP probe additionally saves and reads 4 MiB through both server implementations.

## Complete file reads

`tekesWorkspace.filesRead` accepts `maxBytes: 0` for a complete snapshot with no
fixed file-size limit. Positive values preserve bounded preview semantics and
omitted `maxBytes` still defaults to 1 MiB. Complete reads preserve scoped path
resolution, regular-file checks, snapshot validation and full-content SHA-256.
The parent helper pipe removes its stdout byte cap only for this explicit mode;
request/stderr limits, process cancellation and timeout ownership remain intact.
Clients must reject truncated responses instead of using a prefix as a full file.
Older Host builds reject zero, so this mode requires the updated service binary.
