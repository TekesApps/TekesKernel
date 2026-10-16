# plugins — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [plugins::archive::PluginSource](../../src/archive.rs#L22) | `pub` | not a function |
| [plugins::archive::PluginSource::from_path](../../src/archive.rs#L28) | `pub` | [plugins::tests::slice12_gates::slice12_gate_81_hermetic_package_carrier_compatibility](../../tests/slice12_gates.rs#L144); [plugins::tests::slice12_gates::slice12_reference_qualification_external_archive_without_launch](../../tests/slice12_gates.rs#L826) |
| [plugins::archive::PluginSource::stage](../../src/archive.rs#L47) | `pub(crate)` | no resolved direct caller |
| [plugins::archive::PluginSource::description](../../src/archive.rs#L54) | `pub(crate)` | no resolved direct caller |
| [plugins::archive::PluginArchive](../../src/archive.rs#L61) | `pub` | not a function |
| [plugins::archive::PluginArchive::inspect](../../src/archive.rs#L88) | `pub` | [plugins::tests::slice12_gates::slice12_gate_81_hermetic_package_carrier_compatibility](../../tests/slice12_gates.rs#L144) |
| [plugins::archive::PluginArchive::expand](../../src/archive.rs#L98) | `pub` | [plugins::archive::PluginSource::stage](../../src/archive.rs#L47) |
| [plugins::PLUGIN_MANIFEST_FILE](../../src/lib.rs#L26) | `pub` | not a function |
| [plugins::PLUGIN_ARCHIVE_EXTENSION](../../src/lib.rs#L27) | `pub` | not a function |
| [plugins::model::PluginVersion](../../src/model.rs#L11) | `pub` | not a function |
| [plugins::model::PluginVersion::precedence_cmp](../../src/model.rs#L99) | `pub` | no resolved direct caller |
| [plugins::model::Manifest](../../src/model.rs#L171) | `pub` | not a function |
| [plugins::model::CapabilityRequest](../../src/model.rs#L187) | `pub` | not a function |
| [plugins::model::Component](../../src/model.rs#L194) | `pub` | not a function |
| [plugins::model::ComponentKind](../../src/model.rs#L205) | `pub` | not a function |
| [plugins::model::ComponentKind::expects_directory](../../src/model.rs#L216) | `pub(crate)` | no resolved direct caller |
| [plugins::model::PlatformRequirement](../../src/model.rs#L223) | `pub` | not a function |
| [plugins::model::OperatingSystem](../../src/model.rs#L233) | `pub` | not a function |
| [plugins::model::HostEnvironment](../../src/model.rs#L241) | `pub` | not a function |
| [plugins::model::Manifest::validate](../../src/model.rs#L249) | `pub` | no resolved direct caller |
| [plugins::model::ComponentProjection](../../src/model.rs#L384) | `pub` | not a function |
| [plugins::model::PluginComponentReference](../../src/model.rs#L404) | `pub` | not a function |
| [plugins::model::ResolvedPluginExecutable](../../src/model.rs#L415) | `pub` | not a function |
| [plugins::model::safe_relative](../../src/model.rs#L424) | `pub(crate)` | [plugins::archive::validate_entries](../../src/archive.rs#L271); [plugins::model::Manifest::validate](../../src/model.rs#L249); [plugins::store::validate_receipt](../../src/store.rs#L1095) |
| [plugins::model::qualified_id](../../src/model.rs#L435) | `pub(crate)` | [plugins::model::Manifest::validate](../../src/model.rs#L249); [plugins::store::validate_operation](../../src/store.rs#L1043); [plugins::store::validate_receipt](../../src/store.rs#L1095) |
| [plugins::signature::PUBLISHER_ATTESTATION_PATH](../../src/signature.rs#L13) | `pub` | not a function |
| [plugins::signature::NativeHelperIdentity](../../src/signature.rs#L17) | `pub` | not a function |
| [plugins::signature::PublisherIdentity](../../src/signature.rs#L28) | `pub` | not a function |
| [plugins::signature::SignaturePolicy](../../src/signature.rs#L35) | `pub` | not a function |
| [plugins::signature::NativeHelperVerifier](../../src/signature.rs#L41) | `pub` | not a function |
| [plugins::signature::MacOsNativeHelperVerifier](../../src/signature.rs#L51) | `pub` | not a function |
| [plugins::signature::verify_publisher](../../src/signature.rs#L131) | `pub(crate)` | [plugins::store::PluginStore::install](../../src/store.rs#L258); [plugins::store::PluginStore::inspect_staged](../../src/store.rs#L437); [plugins::store::PluginStore::verify_stored_receipt](../../src/store.rs#L740) |
| [plugins::signature::hex](../../src/signature.rs#L181) | `pub(crate)` | [plugins::signature::verify_publisher](../../src/signature.rs#L131); [plugins::store::package_digest](../../src/store.rs#L1198) |
| [plugins::store::PluginError](../../src/store.rs#L29) | `pub` | not a function |
| [plugins::store::FaultPoint](../../src/store.rs#L71) | `pub` | not a function |
| [plugins::store::InstallOptions](../../src/store.rs#L78) | `pub` | not a function |
| [plugins::store::IntegrityStatus](../../src/store.rs#L87) | `pub` | not a function |
| [plugins::store::PluginReceipt](../../src/store.rs#L94) | `pub` | not a function |
| [plugins::store::PluginInspection](../../src/store.rs#L112) | `pub` | not a function |
| [plugins::store::ManagementRequest](../../src/store.rs#L129) | `pub` | not a function |
| [plugins::store::ManagementResult](../../src/store.rs#L151) | `pub` | not a function |
| [plugins::store::PluginStore](../../src/store.rs#L202) | `pub` | not a function |
| [plugins::store::PluginStore::open](../../src/store.rs#L211) | `pub` | no resolved direct caller |
| [plugins::store::PluginStore::injecting_fault](../../src/store.rs#L230) | `pub` | no resolved direct caller |
| [plugins::store::PluginStore::manage](../../src/store.rs#L235) | `pub` | no resolved direct caller |
| [plugins::store::PluginStore::install](../../src/store.rs#L258) | `pub` | [plugins::store::PluginStore::manage](../../src/store.rs#L235) |
| [plugins::store::PluginStore::inspect](../../src/store.rs#L391) | `pub` | no resolved direct caller |
| [plugins::store::PluginStore::freeze_source](../../src/store.rs#L411) | `pub` | no resolved direct caller |
| [plugins::store::PluginStore::list](../../src/store.rs#L468) | `pub` | [plugins::store::PluginStore::manage](../../src/store.rs#L235) |
| [plugins::store::PluginStore::set_enabled](../../src/store.rs#L474) | `pub` | [plugins::store::PluginStore::manage](../../src/store.rs#L235) |
| [plugins::store::PluginStore::set_grants](../../src/store.rs#L495) | `pub` | [plugins::store::PluginStore::manage](../../src/store.rs#L235) |
| [plugins::store::PluginStore::remove](../../src/store.rs#L513) | `pub` | [plugins::store::PluginStore::manage](../../src/store.rs#L235) |
| [plugins::store::PluginStore::components](../../src/store.rs#L549) | `pub` | [plugins::store::PluginStore::manage](../../src/store.rs#L235) |
| [plugins::store::PluginStore::resolve_executable_component](../../src/store.rs#L597) | `pub` | no resolved direct caller |
| [plugins::store::PluginStore::recover](../../src/store.rs#L647) | `pub` | [plugins::store::PluginStore::manage](../../src/store.rs#L235) |
