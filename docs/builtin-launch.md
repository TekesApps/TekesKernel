# Application-owned Kernel launch

The application launches `tekes-supervisor --built-in /absolute/launch.json`.
This entry point uses the native SessionEndpoint HTTP/WebSocket implementation;
there is no provider-specific client translation layer and no installer or
Keychain access on this launch path.

Build the four sibling executables with:

```sh
cargo build -p tekes-supervisor -p tekes-worker -p tools -p workspace-service --bin tekes-supervisor --bin tekes-worker --bin tekes-helper --bin tekes-workspace-service
python3 scripts/test-builtin-launch.py
python3 scripts/test-builtin-client-contract.py
```

`tekes-supervisor --models-available` returns the non-secret executable dialect
proof catalog before startup, for application-side launch configuration.

The launch document is non-secret JSON:

```json
{
  "format": 1,
  "root": "/absolute/application-owned/kernel-state",
  "worker": "/absolute/bin/tekes-worker",
  "listen": "127.0.0.1:0",
  "providers": {"format": 1, "revision": 1, "providers": []},
  "credential_bindings": {}
}
```

`providers` is the Kernel execution configuration generated from the application's
selected providers and their resolved direct/router connections. It is replaced
on startup; its persisted revision is assigned by Kernel. It contains credential
IDs, never credential material. `credential_bindings` maps every credential ID
referenced by that configuration to the exact environment variable containing
its value. Extra or missing bindings fail startup. A missing or empty selected
secret fails startup; there is no Keychain fallback. The application owns key
editing, selection, and process restart after credential changes.

Set `TEKES_KERNEL_ENDPOINT_TOKEN` to 32 random bytes encoded as 64 hexadecimal
characters. HTTP authentication uses the same bytes encoded as unpadded URL-safe
base64 in `Authorization: Bearer ...`. The token is process-scoped and is not
persisted. Keep the child's stdin pipe open for its lifetime. Closing it requests
drain and shutdown; SIGTERM also requests shutdown.

The first stdout line is a JSON readiness record containing `type: ready`,
`protocolVersion: 3`, `url`, and `pid`. Use the reported URL, especially when
requesting an ephemeral port. Health is `/health/ready`; unary requests are
`POST /api/{method}` and streams use `/api/remote.mux`. The authoritative protocol
handshake is the WebSocket `ready` frame, not the stdout launch notification.

The smoke test uses a temporary state directory and synthetic provider key. It
verifies readiness, workspace/session creation, model selection and routability,
WebSocket handshake and all five stream baselines, and parent-pipe shutdown.
The Client contract check compiles the sibling Tekes Swift DTO sources and
decodes/validates actual Kernel frames without a Client translation shim. It
also compiles the generic `NativeSessionEndpoint` from Tekes and runs its
connection, workspace/session creation, model selection, journal and history
paging against the real child process. It does
not prove a real provider response or Tekes application UI integration.

The sibling Tekes application bundles the four executables in
`Resources/BuiltInKernel`. Its `KernelManagedServer` owns launch configuration,
environment injection and process lifetime; `NativeSessionEndpoint` connects
directly using the shared protocol. The Kernel Server settings permit a binary
directory override and provider selection. `KernelManagedServerTests` covers
packaged launch, connection, process reuse, stop and restart, in addition to
credential binding and unsupported-model rejection.

History cursors follow the current Client contract: `beforeSequence` is optional,
accepts `-1` for an exhausted page, and cannot exceed `throughSequence`.
Application tests and contract tests do not constitute visual UI acceptance.
