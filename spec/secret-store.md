# spec: application-owned launch credentials

Tekes owns provider/router configuration and Keychain. Kernel never reads,
writes, migrates, enumerates, or falls back to Keychain items. The built-in
launcher supplies an explicit non-secret credential selection with effective
model configuration, and the selected values in a private pipe frame. See
[the built-in launch contract](../docs/builtin-launch.md).

## Startup

Every credential reference in the launch provider configuration (including
web search when configured) has exactly one binding. Unreferenced bindings,
missing frame values, duplicate credential IDs, empty secrets and malformed
former environment-source names reject startup. Kernel captures selected frame
values before runtime assembly. The old endpoint-token and selected credential
environment entries are rejected when present, without persisting or logging
values. Kernel never scans for plausible API keys.

The captured store is immutable and process-scoped. Rotation, deletion and
revocation belong to Tekes and take effect through a new process launch.
No provider secret, OAuth grant, or secret generation floor is persisted by
Kernel startup. The built-in assembly installs no secret mutation authority.
The application must not reuse a live process when its selected credentials
change.

## Runtime interface

`SecretStore.resolve(credential_id)` returns an active in-memory record or
`not_found`; errors remain typed and never include material. The existing
credential broker resolves these records into per-request, origin-scoped
worker capabilities. Workers and tools continue to start with cleared ambient
environments and do not receive the supervisor's full credential environment.
A secret value is nonempty UTF-8 without C0 controls or DEL. In-memory records
zero their material on drop and redact Debug output.

The record format retains a positive generation field for broker compatibility.
Private launch snapshots use generation 1 within their new process lifetime;
there is no cross-launch persisted generation comparison. Memory-store mutation
and OAuth exchange helpers remain available to isolated tests and explicit
embedders, but neither production nor built-in assembly installs them as a
credential write authority.

## Connection authentication

The endpoint token is separate from provider/tool credentials. Tekes generates
32 random bytes for the child process and supplies the hexadecimal encoding in
the private stdin bootstrap frame; HTTP uses the unpadded base64url encoding of those
bytes as its bearer. Kernel does not persist this token or read a previous one
from Keychain. Missing/malformed tokens reject startup. Initial argv/environment
are not secret carriers; the old environment entry point is rejected. The
built-in host also rejects the unauthenticated optional library Web Client.

## Verification

- Provider tests cover explicit-only capture, unknown references, malformed
  names, missing/invalid values, and secret redaction.
- `scripts/test-builtin-launch.py` runs real supervisor/worker binaries and
  verifies missing credentials/token rejection, duplicate-process exclusion,
  provider-edit rejection, model readiness, all stream baselines and shutdown.
- `scripts/test-builtin-client-contract.py` decodes emitted frames using the
  current Tekes Swift contract.
- A real provider call remains a separate runtime gate; synthetic keys and
  model readiness alone do not prove successful model execution.
