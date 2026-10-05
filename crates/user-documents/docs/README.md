# user-documents — feedback and settings

[Crate map](../../../docs/architecture/crates.md) · [Generated source index](generated/index.md)

This library owns Client-facing durable documents that are separate from the
thread ledger. `feedback.*` methods manage per-session message feedback;
`settings.*` methods manage user instruction settings and their revision. Both
families validate closed request shapes and write canonical JSON atomically.

[Source](../src/lib.rs) lists the methods and dispatches to
[feedback](../src/feedback.rs) and [settings](../src/settings.rs). The
[Client extension contract](../../../spec/client-extensions.md) defines public
behavior; [instruction snapshots](../../../spec/instruction-snapshot.md)
define settings use at launch.
