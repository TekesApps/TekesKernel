# plugins

[Package atlas](index.md) · [Source](../../src/lib.rs)

## Declarations

Visibility is the declaration spelling; trait members and reexports require their enclosing interface. `cfg` is not evaluated.

| Symbol | Kind | Visibility | Test / cfg |
|---|---|---|---|
| [plugins::PLUGIN_MANIFEST_FILE](../../src/lib.rs#L26) | const_item | `pub` |  |
| [plugins::PLUGIN_ARCHIVE_EXTENSION](../../src/lib.rs#L27) | const_item | `pub` |  |

## Imports / reexports

| Local name | Source path | Visibility |
|---|---|---|
| `PluginArchive` | `archive::PluginArchive` | `pub` |
| `PluginSource` | `archive::PluginSource` | `pub` |
| `CapabilityRequest` | `model::CapabilityRequest` | `pub` |
| `Component` | `model::Component` | `pub` |
| `ComponentKind` | `model::ComponentKind` | `pub` |
| `ComponentProjection` | `model::ComponentProjection` | `pub` |
| `HostEnvironment` | `model::HostEnvironment` | `pub` |
| `Manifest` | `model::Manifest` | `pub` |
| `OperatingSystem` | `model::OperatingSystem` | `pub` |
| `PlatformRequirement` | `model::PlatformRequirement` | `pub` |
| `PluginComponentReference` | `model::PluginComponentReference` | `pub` |
| `PluginVersion` | `model::PluginVersion` | `pub` |
| `ResolvedPluginExecutable` | `model::ResolvedPluginExecutable` | `pub` |
| `MacOsNativeHelperVerifier` | `signature::MacOsNativeHelperVerifier` | `pub` |
| `NativeHelperIdentity` | `signature::NativeHelperIdentity` | `pub` |
| `NativeHelperVerifier` | `signature::NativeHelperVerifier` | `pub` |
| `PublisherIdentity` | `signature::PublisherIdentity` | `pub` |
| `SignaturePolicy` | `signature::SignaturePolicy` | `pub` |
| `FaultPoint` | `store::FaultPoint` | `pub` |
| `InstallOptions` | `store::InstallOptions` | `pub` |
| `IntegrityStatus` | `store::IntegrityStatus` | `pub` |
| `ManagementRequest` | `store::ManagementRequest` | `pub` |
| `ManagementResult` | `store::ManagementResult` | `pub` |
| `PluginError` | `store::PluginError` | `pub` |
| `PluginInspection` | `store::PluginInspection` | `pub` |
| `PluginReceipt` | `store::PluginReceipt` | `pub` |
| `PluginStore` | `store::PluginStore` | `pub` |

## Module declarations

| Module | Visibility | Attributes |
|---|---|---|
| `plugins::archive` | `private` |  |
| `plugins::model` | `private` |  |
| `plugins::signature` | `private` |  |
| `plugins::store` | `private` |  |

## Function call graphs

Edges below are syntactically resolved calls only, including private functions. Graphs partition callers into groups of 20; they are not execution order. All unresolved sites are listed below and in the JSON inventory.

## Call sites

Includes test functions (marked in declarations). Receiver-type-required sites need type analysis/manual tracing. Calls in closures are attributed to their enclosing function; their occurrence here does not mean the closure executes immediately.

| Caller | Callee expression | Source lines | Target / classification |
|---|---|---|---|
