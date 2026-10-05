# Rust: toolchain and implementation constraints

[Implementation · documentation home](../README.md) · [Architecture overview](README.md) · [Next: running tests](../verification/running-tests.md)

This is the language-specific companion to the neutral
[implementation plan](../history/implementation-plan.md). It fixes how the first
TekesKernel implementation maps the logical components and conformance gates
to Rust (D-63). It is not an event, wire, or lifecycle authority; `spec/` wins.

## Toolchain and platform

- Rust edition 2024 with Cargo resolver 2.
- `rust-toolchain.toml` pins the repository's verified Slice-1
  toolchain, `1.85.1`, with `rustfmt` and `clippy`; upgrades are deliberate
  commits, never floating CI behavior.
- `Cargo.lock` is committed for binaries and CI reproducibility.
- The authoritative Slice-1–10 lane is `aarch64-apple-darwin` on macOS 15 or
  newer. `x86_64-apple-darwin` runs when available; Linux is a portability
  lane, not a substitute for Darwin durability tests.
- `unsafe` is denied by the workspace defaults; packages with platform bindings
  declare explicit Cargo lint overrides. The generated target/dependency inventory
  links each manifest for review.

The current kernel integrates with the Swift Client/AppServer through process
and versioned wire boundaries. Slices 1–10 do not add UniFFI, a C ABI, or shared
in-process model types.

## Cargo workspace

The current package/target/dependency inventory is maintained once in the
[generated atlas](generated/index.md), with
[per-crate explanations and executable mapping](crates.md).
It includes schedule, product-installer, test targets and the helper binary;
do not infer the product process tree from package directories.

Dependency intent remains in the [implementation plan](../history/implementation-plan.md). Actual normal/dev/build
edges are separate in the atlas. Lower libraries do not acquire a supervisor
implementation dependency merely because the supervisor supplies a trait implementation.
The selector remains independent of worker/supervisor production source.

Cargo workspace lints currently deny `unsafe_code` and `unsafe_op_in_unsafe_fn`;
packages owning platform bindings explicitly opt into their declared exceptions.
See the actual Cargo manifests rather than assuming every source has a crate-level
`forbid` attribute. Async scheduling is never a durable ordering authority.

The current public client protocol is [V3](../../spec/session-endpoint.md#routes-and-streams).
The Slice mappings below preserve their original gate identity and V2 baseline;
they are not proof that those old routes are still exposed.

## Dependency policy

The initial dependency set stays narrow:

- `serde`/`serde_json` for syntax and typed payloads. Durable event parsing uses
  the kernel-owned `IJsonValue`; some exported runtime DTOs also contain
  `serde_json::Value`, so this is not a blanket ban on third-party public Rust types;
- an RFC-8785/ES6-number implementation or a small owned canonical writer,
  accepted only by all 86 byte fixtures and the rejection corpus;
- `sha2` for content-addressed assets;
- `thiserror` for closed typed errors;
- `libc` or an equally thin syscall binding for Darwin operations;
- one workspace-pinned, feature-pruned async runtime plus HTTP/WebSocket client,
  selected and added in Slice 7 (not assumed to exist beforehand) and reused by
  provider/transport and their hosting binaries.

Crate versions are workspace-pinned and locked. A dependency cannot become an
authority for semantics: fixture bytes, error classes, barrier membership,
and validation acceptance remain kernel-owned. No dependency may generate or
rewrite the oracle.

## Darwin durability boundary

`store` owns [platform.rs](../../crates/store/src/platform.rs), with conditional
Darwin handling (including `FullSync::full_sync`), alongside its file/tail writers for:

- `open`/`openat` flags including `O_APPEND`, `O_CLOEXEC`, `O_NONBLOCK`, and
  no-follow behavior;
- nonblocking `flock(LOCK_EX | LOCK_NB)` and lifecycle locks;
- checked `write` loops with `EINTR` and partial-write handling;
- `fcntl(fd, F_FULLFSYNC)` for D-45 barriers;
- directory sync, inode revalidation, rename/clone capability probes, and
  explicit errno mapping.

`std::fs::File::sync_all()` is not a replacement for a D-45 barrier on
Darwin. Safe wrappers own file descriptors, validate return values, and expose
typed failures. Unit tests exercise errno mapping; macOS integration gates
exercise real locks, power-boundary fault points, and tail repair.
