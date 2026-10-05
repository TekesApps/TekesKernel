# workspace-service — isolated workspace operations

[Crate map](../../../docs/architecture/crates.md) · [Generated source index](generated/index.md)

The `tekes-workspace-service` child process handles workspace file, Git,
observation, write and process operations. Its parent resolves the workspace
root from the registry and passes that authority to the child. The service
checks that requested paths stay within the resolved root.

[Library source](../src/lib.rs) defines request and path handling;
[binary source](../src/main.rs) owns process entry. See the
[workspace service guide](../../../docs/workspace-service-wse.md) and
[Client extension contract](../../../spec/client-extensions.md) for the
cross-process boundary.
