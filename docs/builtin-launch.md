# Application-owned Kernel launch

The application launches `tekes-supervisor --built-in /absolute/launch.json`.
This entry point uses the native SessionEndpoint HTTP/WebSocket implementation;
there is no provider-specific client translation layer and no installer or
Keychain access on this launch path.

Build the four sibling executables with:

```sh
cargo build -p tekes-supervisor -p tekes-worker -p tools -p workspace-service --bin tekes-supervisor --bin tekes-worker --bin tekes-helper --bin tekes-workspace-service
```

Then run the checks:

```sh
python3 scripts/test-builtin-launch.py
python3 scripts/test-builtin-client-contract.py
```

`test-builtin-client-contract.py` also needs `swiftc` and a TekesClientKit
checkout (`TEKES_CLIENT_KIT_ROOT`, default `../TekesClientKit`).

`tekes-supervisor --models-available` returns the dialects this build supports
as `[{"dialect_id", "protocol_family"}]`. The kernel has no route allow-list:
the launch document's provider routes are used as given, and a route the
upstream rejects fails at request time.

The launch document is non-secret JSON:

```json
{
  "format": 1,
  "bootstrap_version": 1,
  "root": "/absolute/application-owned/kernel-state",
  "worker": "/absolute/bin/tekes-worker",
  "listen": "127.0.0.1:0",
  "configuration_mode": "provided",
  "providers": {"format": 1, "revision": 1, "providers": []},
  "credential_bindings": {}
}
```

`format` and `bootstrap_version` must be 1, `root` and `worker` must be absolute
paths, and `listen` must be a loopback address. A non-null `web_listen` is rejected
before startup: the optional library Web Client currently checks only Host/Origin,
which cannot authenticate a local session tool. An administrative host must not
expose that surface. Unknown fields are rejected. `tekes-helper` and
`tekes-workspace-service` must be in the same directory as `worker`. `HOME`
must be set; shared user skills are read from `$HOME/.agents`.

`configuration_mode` is optional. Omission means `provided` for compatibility.
In `provided` mode, `providers` is required and is the Kernel execution
configuration generated from the application's selected providers and their
resolved direct/router connections. It is replaced on startup; its persisted
revision is assigned by Kernel. It contains credential IDs, never credential
material.

Use `"configuration_mode": "existing"` to use the provider and settings
configuration already stored under `root/config/providers.json` and
`root/config/settings.json`. In this mode omit `providers`. Both files must
already exist and validate; startup fails before creating thread storage if
either is missing or invalid. Kernel reads them without publishing or changing
their bytes, revisions, provider roster, or selected default. Existing
configuration is not migrated. `credential_bindings` may contain only the
configured credential IDs that the launcher can supply; other configured
providers remain visible and configured, but requests needing an unbound
credential fail closed. Unknown binding IDs are rejected. The private bootstrap
frame's credential keys must exactly match `credential_bindings`.

In both modes, `credential_bindings` retains each lookup ID's former
environment-source name for compatibility diagnostics; no value is read from
that environment. Source names must remain syntactically valid. In `provided`
mode, binding IDs must cover all provider and web-search credential references.
The private frame must contain exactly those IDs. Missing, extra, duplicate or
empty selected credentials fail startup. There is no Keychain fallback. The
application owns selection and restarts after changes.

Before waiting for readiness, write one frame to the child's private stdin pipe:

```text
ASCII "TEKES-KERNEL-BOOTSTRAP/1\n"
4-byte unsigned big-endian payload length (1 ... 1048576)
UTF-8 JSON {"endpointToken":"<64 hex characters>","credentials":{"<lookup ID>":"<material>"}}
```

The object is closed and rejects duplicate keys. The whole frame has a 10-second
deadline, including short reads. Stdin must be a pipe or socket, not a regular
file or TTY. No config is changed, listener bound or worker started until the
complete frame and credential set validate. Exact bounded reads consume one
frame without buffered prefetch. Explicit raw/parsed secret and token buffers
are zeroized when released; this does not claim that every allocator/compiler
copy is observable or erasable.

Generate 32 random endpoint bytes and encode them as 64 hex characters in that
frame only. HTTP uses those bytes as unpadded URL-safe base64 in `Authorization:
Bearer ...`. Never put the token or selected provider material in initial argv
or environment: macOS process-argument queries can expose a same-user process's
initial environment even from the current tool sandbox. Presence of
`TEKES_KERNEL_ENDPOINT_TOKEN` or any selected former source variable rejects
startup, including an empty value. Clearing them after exec is insufficient.

This is a required 0.8 launcher adaptation. Launchers using the former environment
protocol fail closed; ClientKit must adapt before consuming 0.8. Consumer changes
are a separate integration step.

Keep the child's stdin pipe open after the frame for its lifetime. Closing it
requests drain and shutdown; SIGTERM also requests shutdown. Exit code 64 means
a usage error and 74 a startup or runtime failure.

The first stdout line is a JSON readiness record containing `type: ready`,
`protocolVersion: 3`, `bootstrapVersion: 1`, `url`, and `pid`.
Use the reported URLs, especially when requesting an ephemeral port. Health is `/health/ready`; unary requests are
`POST /api/{method}`, the multiplexed streams use `/api/remote.mux`, and
file-change notifications use `GET /api/session.files.changes`. The
authoritative protocol handshake is the WebSocket `ready` frame, not the stdout
launch notification.

The smoke test uses a temporary state directory and synthetic provider key. It
verifies private bootstrap, legacy-environment and Web Client rejection,
readiness, workspace/session creation, model selection and routability,
WebSocket handshake and all five stream baselines, and parent-pipe shutdown. The
macOS check reads `KERN_PROCARGS2` with a synthetic positive-control marker and
asserts that the synthetic endpoint token and selected provider key are absent;
the queried process bytes are never printed.

The Client contract check (it needs TekesClientKit, see above) compiles the
TekesClientKit Swift contract sources and decodes/validates actual Kernel frames
without a Client translation shim. It also compiles the generic
`NativeSessionEndpoint` from TekesClientKit and runs its connection,
workspace/session creation, model selection, journal and history paging against
the real child process. It does not prove a real provider response or Tekes
application UI integration.

This paragraph applies to the sibling Tekes client repository; none of these
names exist in this repository. The sibling Tekes application bundles the four
executables in `Resources/BuiltInKernel`. Its `KernelManagedServer` owns launch
configuration, private bootstrap and process lifetime;
`NativeSessionEndpoint` connects directly using the shared protocol. The Kernel
Server settings permit a binary directory override and provider selection.
`KernelManagedServerTests` covers packaged launch, connection, process reuse,
stop and restart, in addition to credential binding and unsupported-model
rejection.

History cursors follow the current Client contract: `beforeSequence` is optional,
accepts `-1` for an exhausted page, and cannot exceed `throughSequence`.
Application tests and contract tests do not constitute visual UI acceptance.

Global `providers.json` and `settings.json` may retain existing formatted JSON.
Reads validate I-JSON members/numbers, null and the typed schema after in-memory
canonicalization; they do not rewrite either file. Explicit config publication
continues to emit canonical JSON. Session configuration and ledger byte contracts
remain strict.

Recovery distinguishes semantic ledgers from the exact retired endpoint projection
names `endpoint-v2.jsonl` and `endpoint-requests-v2.jsonl`. These sidecars are not
semantic input; their existing endpoint-journal lifecycle owns retirement/rebuild.
Empty or corrupt genuine semantic ledgers still fail closed. The isolated public
process regression is `scripts/test-legacy-endpoint-recovery.py --binary-dir DIR
--output NEW_EVIDENCE_DIR`; it verifies history and a full restart without model
calls. `--expect-quarantine` records the old binary negative control separately.
