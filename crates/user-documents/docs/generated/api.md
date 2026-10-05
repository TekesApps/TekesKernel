# user-documents — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [user-documents::feedback::Feedback](../../src/feedback.rs#L16) | `pub` | not a function |
| [user-documents::feedback::path](../../src/feedback.rs#L36) | `pub` | [user-documents::feedback::save](../../src/feedback.rs#L123) |
| [user-documents::feedback::validate](../../src/feedback.rs#L61) | `pub` | [user-documents::validate](../../src/lib.rs#L64) |
| [user-documents::feedback::list](../../src/feedback.rs#L135) | `pub` | [user-documents::execute](../../src/lib.rs#L75) |
| [user-documents::feedback::put](../../src/feedback.rs#L141) | `pub` | [user-documents::execute](../../src/lib.rs#L75) |
| [user-documents::feedback::delete](../../src/feedback.rs#L180) | `pub` | [user-documents::execute](../../src/lib.rs#L75) |
| [user-documents::METHODS](../../src/lib.rs#L21) | `pub` | not a function |
| [user-documents::Failure](../../src/lib.rs#L34) | `pub` | not a function |
| [user-documents::Failure::new](../../src/lib.rs#L41) | `pub` | [user-documents::feedback::load](../../src/feedback.rs#L100); [user-documents::feedback::save](../../src/feedback.rs#L123); [user-documents::feedback::conflict](../../src/feedback.rs#L129); [user-documents::feedback::put](../../src/feedback.rs#L141); [user-documents::write_atomic](../../src/lib.rs#L101); [user-documents::Failure::bad_request](../../src/lib.rs#L54); [user-documents::Failure::io](../../src/lib.rs#L58); [user-documents::canonical_bytes](../../src/lib.rs#L93); [user-documents::settings::invalid](../../src/settings.rs#L116); [user-documents::settings::check_namespace](../../src/settings.rs#L297); [user-documents::settings::check_revision](../../src/settings.rs#L309) |
| [user-documents::Failure::with_details](../../src/lib.rs#L49) | `pub` | no resolved direct caller |
| [user-documents::Failure::bad_request](../../src/lib.rs#L54) | `pub` | [user-documents::execute](../../src/lib.rs#L75); [user-documents::settings::field_of](../../src/settings.rs#L319) |
| [user-documents::Failure::io](../../src/lib.rs#L58) | `pub` | [user-documents::feedback::load](../../src/feedback.rs#L100); [user-documents::write_atomic](../../src/lib.rs#L101); [user-documents::settings::read_document](../../src/settings.rs#L189); [user-documents::settings::read_sidecar](../../src/settings.rs#L211) |
| [user-documents::validate](../../src/lib.rs#L64) | `pub` | [tekes-supervisor::client_extensions::validate_payload](../../../supervisor/src/client_extensions.rs#L2834); [user-documents::execute](../../src/lib.rs#L75) |
| [user-documents::execute](../../src/lib.rs#L75) | `pub` | [tekes-supervisor::client_extensions::ProductionClientExtensions::execute](../../../supervisor/src/client_extensions.rs#L1739); [user-documents::tests::documents::run](../../tests/documents.rs#L8) |
| [user-documents::canonical_bytes](../../src/lib.rs#L93) | `pub(crate)` | [user-documents::feedback::save](../../src/feedback.rs#L123); [user-documents::settings::read_document](../../src/settings.rs#L189); [user-documents::settings::store](../../src/settings.rs#L253) |
| [user-documents::write_atomic](../../src/lib.rs#L101) | `pub(crate)` | [user-documents::feedback::save](../../src/feedback.rs#L123); [user-documents::settings::write_sidecar](../../src/settings.rs#L224); [user-documents::settings::store](../../src/settings.rs#L253); [user-documents::settings::document](../../src/settings.rs#L380) |
| [user-documents::closed_object](../../src/lib.rs#L123) | `pub(crate)` | [user-documents::feedback::validate](../../src/feedback.rs#L61); [user-documents::settings::validate](../../src/settings.rs#L70) |
| [user-documents::required_str](../../src/lib.rs#L136) | `pub(crate)` | [user-documents::feedback::validate](../../src/feedback.rs#L61) |
| [user-documents::now_milliseconds](../../src/lib.rs#L147) | `pub(crate)` | [user-documents::feedback::put](../../src/feedback.rs#L141) |
| [user-documents::settings::NAMESPACE](../../src/settings.rs#L14) | `pub` | not a function |
| [user-documents::settings::document_path](../../src/settings.rs#L22) | `pub` | [user-documents::settings::read_document](../../src/settings.rs#L189); [user-documents::settings::store](../../src/settings.rs#L253); [user-documents::settings::document](../../src/settings.rs#L380) |
| [user-documents::settings::revision_path](../../src/settings.rs#L26) | `pub` | [user-documents::settings::read_sidecar](../../src/settings.rs#L211); [user-documents::settings::write_sidecar](../../src/settings.rs#L224) |
| [user-documents::settings::schema](../../src/settings.rs#L30) | `pub` | no resolved direct caller |
| [user-documents::settings::validate](../../src/settings.rs#L70) | `pub` | [user-documents::validate](../../src/lib.rs#L64) |
| [user-documents::settings::describe](../../src/settings.rs#L332) | `pub` | [user-documents::execute](../../src/lib.rs#L75) |
| [user-documents::settings::mutate](../../src/settings.rs#L337) | `pub` | [user-documents::execute](../../src/lib.rs#L75) |
| [user-documents::settings::update](../../src/settings.rs#L360) | `pub` | [user-documents::execute](../../src/lib.rs#L75) |
| [user-documents::settings::document](../../src/settings.rs#L380) | `pub` | [user-documents::execute](../../src/lib.rs#L75) |
