# host-files — host browsing and references

[Crate map](../../../docs/architecture/crates.md) · [Generated source index](generated/index.md)

This library implements four Client extension methods: `directory.list`,
`directory.create`, `session.references.files`, and
`session.references.sessions`. It validates requests, browses directories and
workspace files, and shapes results. The supervisor supplies the caller's home
directory, workspace root and session inventory; this crate does not infer
those authorities from environment variables or endpoint payloads.

The method list and limits are in [source](../src/lib.rs). The
[Client extension contract](../../../spec/client-extensions.md) defines the
wire behavior. The supervisor maps crate failures to public error envelopes.

Use the [generated index](generated/index.md) to locate declarations and call
sites. Generated call graphs are static navigation aids, not runtime evidence.
