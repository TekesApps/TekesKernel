# Worker startup and recovery

On macOS, a newly installed ad hoc signed executable can wait inside AMFI / Apple
System Policy assessment before it reaches `main`. The old ten-second handshake
budget killed and retried the child while that assessment was pending. A recorded
incident had five consecutive pre-hello timeouts before the sixth launch worked.
The system policy log named each affected worker PID; the normal sandbox probe
completed within the first second. This was independent of tool access mode.

The accepted prompt is persisted and acknowledged before worker startup. A
background scheduler owns the launch, with a per-process reservation counted
against capacity. It releases the process table while awaiting hello, so other
sessions and metadata/cancellation requests remain responsive. Exact RPC retries
retain their original input receipt without launching another worker. Shutdown
cancels pre-adoption handshakes and waits for their guards to reap children.

The supervisor now reports slow startup at ten seconds and retains the same child
for a bounded 120-second macOS cold-start budget. Other platforms retain ten
seconds. No signing checks, security policy, permissions or protocol validation
are bypassed. A hard timeout, malformed hello, EOF or incompatible protocol still
reaps the child and closes its credential broker.

`worker_startup` JSON records include UTC time, session/run identity, child PID,
stage, elapsed/budget milliseconds, exit status and a bounded stderr byte count.
The worker emits constant `main_entered` and `hello_sent` witnesses on stderr;
only these allowlisted witnesses enter startup telemetry. Arbitrary stderr,
arguments, environment, prompts and credentials are not copied into these logs.
A missing witness means the child has not been observed entering application
startup; correlate the PID/time with macOS AMFI / AppleSystemPolicy logs rather
than calling it a sandbox denial. Stderr is drained before hello negotiation, so
verbose startup errors cannot fill the pipe and deadlock hello. Full startup
stderr remains bounded to the existing 4 KiB private failure tail.

The shared Rust presentation preserves launch-error text and identity and marks
it recovered after a later durable `turn/start` in the same session. An accepted
input, reconnection or idle status alone is insufficient evidence of recovery.
Historical replay uses the same rule, including errors from older Kernels.

Regression coverage includes delayed hello past the slow threshold without a
restart, hard timeout and child cleanup, early EOF, malformed negotiation, and
stderr larger than pipe capacity without handshake deadlock or telemetry leaks.

To reproduce cold startup using real Release binaries and the local mock provider,
place the built binaries under `target/debug/` and run:

```sh
TEKES_TEST_CLI_HOST=1 TEKES_TEST_WORKER_STARTUP_DELAY=11 python3 scripts/test-builtin-launch.py
```

The fixture checks sub-ten-second prompt acknowledgements, slow-start telemetry,
hello recovery with unchanged PIDs, execution, MCP, restart and child cleanup. It
does not contact a model provider or change system security policy.
