# tekes-kernel-installer — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [tekes-kernel-installer::artifact::Artifact](../../src/artifact.rs#L4) | `pub(crate)` | not a function |
| [tekes-kernel-installer::artifact::Artifact::selector](../../src/artifact.rs#L24) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::artifact::Artifact::selector_manifest](../../src/artifact.rs#L27) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::artifact::Artifact::bundle](../../src/artifact.rs#L30) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::artifact::Artifact::load](../../src/artifact.rs#L33) | `pub` | [tekes-kernel-installer::execute](../../src/lib.rs#L140) |
| [tekes-kernel-installer::fs::absolute](../../src/fs.rs#L14) | `pub` | [tekes-kernel-installer::fs::no_symlink_ancestors](../../src/fs.rs#L35); [tekes-kernel-installer::Arguments::parse](../../src/lib.rs#L80); [tekes-kernel-installer::platform::macos::layout](../../src/platform/macos.rs#L7) |
| [tekes-kernel-installer::fs::uid](../../src/fs.rs#L25) | `pub` | [tekes-kernel-installer::fs::directory](../../src/fs.rs#L88); [tekes-kernel-installer::migration::migrate](../../src/migration.rs#L163) |
| [tekes-kernel-installer::fs::exists](../../src/fs.rs#L28) | `pub` | [tekes-kernel-installer::migration::migrate_with](../../src/migration.rs#L166); [tekes-kernel-installer::migration::validate_state](../../src/migration.rs#L79); [tekes-kernel-installer::platform::macos::stop](../../src/platform/macos.rs#L140); [tekes-kernel-installer::transaction::begin](../../src/transaction.rs#L101); [tekes-kernel-installer::transaction::initialize](../../src/transaction.rs#L175); [tekes-kernel-installer::transaction::publish_selector](../../src/transaction.rs#L227) |
| [tekes-kernel-installer::fs::no_symlink_ancestors](../../src/fs.rs#L35) | `pub` | [tekes-kernel-installer::fs::no_symlink_tree](../../src/fs.rs#L44); [tekes-kernel-installer::fs::directory](../../src/fs.rs#L88); [tekes-kernel-installer::migration::migrate_with](../../src/migration.rs#L166); [tekes-kernel-installer::migration::validate_identity](../../src/migration.rs#L66) |
| [tekes-kernel-installer::fs::no_symlink_tree](../../src/fs.rs#L44) | `pub` | [tekes-kernel-installer::artifact::Artifact::manifest](../../src/artifact.rs#L40) |
| [tekes-kernel-installer::fs::regular](../../src/fs.rs#L58) | `pub` | [tekes-kernel-installer::artifact::Artifact::manifest](../../src/artifact.rs#L40); [tekes-kernel-installer::fs::object](../../src/fs.rs#L73); [tekes-kernel-installer::fs::digest](../../src/fs.rs#L82); [tekes-kernel-installer::transaction::initialize](../../src/transaction.rs#L175); [tekes-kernel-installer::transaction::publish_selector](../../src/transaction.rs#L227); [tekes-kernel-installer::transaction::install](../../src/transaction.rs#L254) |
| [tekes-kernel-installer::fs::mode](../../src/fs.rs#L70) | `pub` | [tekes-kernel-installer::artifact::Artifact::manifest](../../src/artifact.rs#L40); [tekes-kernel-installer::transaction::install](../../src/transaction.rs#L254) |
| [tekes-kernel-installer::fs::object](../../src/fs.rs#L73) | `pub` | [tekes-kernel-installer::artifact::Artifact::manifest](../../src/artifact.rs#L40); [tekes-kernel-installer::migration::read](../../src/migration.rs#L127); [tekes-kernel-installer::transaction::begin](../../src/transaction.rs#L101); [tekes-kernel-installer::transaction::installed](../../src/transaction.rs#L35) |
| [tekes-kernel-installer::fs::sha](../../src/fs.rs#L79) | `pub` | [tekes-kernel-installer::artifact::Artifact::manifest](../../src/artifact.rs#L40); [tekes-kernel-installer::fs::digest](../../src/fs.rs#L82); [tekes-kernel-installer::transaction::request](../../src/transaction.rs#L19) |
| [tekes-kernel-installer::fs::digest](../../src/fs.rs#L82) | `pub` | [tekes-kernel-installer::artifact::Artifact::manifest](../../src/artifact.rs#L40) |
| [tekes-kernel-installer::fs::directory](../../src/fs.rs#L88) | `pub` | [tekes-kernel-installer::fs::durable](../../src/fs.rs#L118); [tekes-kernel-installer::fs::lock](../../src/fs.rs#L131); [tekes-kernel-installer::migration::migrate_with](../../src/migration.rs#L166); [tekes-kernel-installer::transaction::initialize](../../src/transaction.rs#L175) |
| [tekes-kernel-installer::fs::sync_dir](../../src/fs.rs#L111) | `pub` | [tekes-kernel-installer::fs::durable](../../src/fs.rs#L118); [tekes-kernel-installer::migration::migrate_with](../../src/migration.rs#L166) |
| [tekes-kernel-installer::fs::durable](../../src/fs.rs#L118) | `pub` | [tekes-kernel-installer::migration::write](../../src/migration.rs#L118); [tekes-kernel-installer::transaction::begin](../../src/transaction.rs#L101); [tekes-kernel-installer::transaction::initialize](../../src/transaction.rs#L175); [tekes-kernel-installer::transaction::publish_selector](../../src/transaction.rs#L227); [tekes-kernel-installer::transaction::install](../../src/transaction.rs#L254); [tekes-kernel-installer::transaction::write_operation](../../src/transaction.rs#L27) |
| [tekes-kernel-installer::fs::lock](../../src/fs.rs#L131) | `pub` | [tekes-kernel-installer::transaction::install](../../src/transaction.rs#L254); [tekes-kernel-installer::transaction::execute](../../src/transaction.rs#L428) |
| [tekes-kernel-installer::fs::lock_available](../../src/fs.rs#L147) | `pub` | [tekes-kernel-installer::transaction::bootout](../../src/transaction.rs#L85) |
| [tekes-kernel-installer::fs::same_file](../../src/fs.rs#L168) | `pub` | [tekes-kernel-installer::platform::macos::verify_artifact](../../src/platform/macos.rs#L55) |
| [tekes-kernel-installer::PROTOCOL](../../src/lib.rs#L13) | `pub` | not a function |
| [tekes-kernel-installer::LEGACY_PROTOCOL](../../src/lib.rs#L15) | `pub` | not a function |
| [tekes-kernel-installer::ORIGIN](../../src/lib.rs#L16) | `pub` | not a function |
| [tekes-kernel-installer::IDENTIFIER](../../src/lib.rs#L17) | `pub` | not a function |
| [tekes-kernel-installer::CONTRACT](../../src/lib.rs#L18) | `pub` | not a function |
| [tekes-kernel-installer::Result](../../src/lib.rs#L20) | `pub` | not a function |
| [tekes-kernel-installer::Failure](../../src/lib.rs#L22) | `pub` | not a function |
| [tekes-kernel-installer::Operation](../../src/lib.rs#L56) | `pub` | not a function |
| [tekes-kernel-installer::Operation::name](../../src/lib.rs#L64) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::Arguments](../../src/lib.rs#L75) | `pub` | not a function |
| [tekes-kernel-installer::Arguments::parse](../../src/lib.rs#L80) | `pub` | [tekes-kernel-installer::main::main](../../src/main.rs#L3) |
| [tekes-kernel-installer::execute](../../src/lib.rs#L140) | `pub` | [tekes-kernel-installer::main::main](../../src/main.rs#L3) |
| [tekes-kernel-installer::error_reply](../../src/lib.rs#L147) | `pub` | [tekes-kernel-installer::main::main](../../src/main.rs#L3) |
| [tekes-kernel-installer::migration::migrate](../../src/migration.rs#L163) | `pub` | [tekes-kernel-installer::transaction::initialize](../../src/transaction.rs#L175) |
| [tekes-kernel-installer::platform::macos::supported](../../src/platform/macos.rs#L4) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::macos::layout](../../src/platform/macos.rs#L7) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::macos::verify_artifact](../../src/platform/macos.rs#L55) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::macos::verify_caller](../../src/platform/macos.rs#L93) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::macos::loaded](../../src/platform/macos.rs#L118) | `pub` | [tekes-kernel-installer::platform::macos::bootstrap](../../src/platform/macos.rs#L121); [tekes-kernel-installer::platform::macos::stop](../../src/platform/macos.rs#L140) |
| [tekes-kernel-installer::platform::macos::bootstrap](../../src/platform/macos.rs#L121) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::macos::stop](../../src/platform/macos.rs#L140) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::macos::render](../../src/platform/macos.rs#L153) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::macos::bearer_read](../../src/platform/macos.rs#L170) | `pub` | [tekes-kernel-installer::platform::macos::bearer_ensure](../../src/platform/macos.rs#L194) |
| [tekes-kernel-installer::platform::macos::bearer_ensure](../../src/platform/macos.rs#L194) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::macos::bearer_rotate](../../src/platform/macos.rs#L201) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::unsupported::supported](../../src/platform/mod.rs#L10) | `pub` | [tekes-kernel-installer::platform::unsupported::verify_artifact](../../src/platform/mod.rs#L16); [tekes-kernel-installer::platform::unsupported::verify_caller](../../src/platform/mod.rs#L19); [tekes-kernel-installer::platform::unsupported::bootstrap](../../src/platform/mod.rs#L25); [tekes-kernel-installer::platform::unsupported::stop](../../src/platform/mod.rs#L28); [tekes-kernel-installer::platform::unsupported::bearer_ensure](../../src/platform/mod.rs#L37); [tekes-kernel-installer::platform::unsupported::bearer_rotate](../../src/platform/mod.rs#L40) |
| [tekes-kernel-installer::platform::unsupported::layout](../../src/platform/mod.rs#L13) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::unsupported::verify_artifact](../../src/platform/mod.rs#L16) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::unsupported::verify_caller](../../src/platform/mod.rs#L19) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::unsupported::loaded](../../src/platform/mod.rs#L22) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::unsupported::bootstrap](../../src/platform/mod.rs#L25) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::unsupported::stop](../../src/platform/mod.rs#L28) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::unsupported::render](../../src/platform/mod.rs#L31) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::unsupported::bearer_read](../../src/platform/mod.rs#L34) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::unsupported::bearer_ensure](../../src/platform/mod.rs#L37) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::platform::unsupported::bearer_rotate](../../src/platform/mod.rs#L40) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::process::Output](../../src/process.rs#L10) | `pub` | not a function |
| [tekes-kernel-installer::process::Output::success](../../src/process.rs#L16) | `pub` | no resolved direct caller |
| [tekes-kernel-installer::process::run](../../src/process.rs#L20) | `pub` | [tekes-kernel-installer::platform::macos::loaded](../../src/platform/macos.rs#L118); [tekes-kernel-installer::platform::macos::bootstrap](../../src/platform/macos.rs#L121); [tekes-kernel-installer::platform::macos::stop](../../src/platform/macos.rs#L140); [tekes-kernel-installer::platform::macos::verify](../../src/platform/macos.rs#L24); [tekes-kernel-installer::transaction::publish_selector](../../src/transaction.rs#L227); [tekes-kernel-installer::transaction::install](../../src/transaction.rs#L254); [tekes-kernel-installer::transaction::attest](../../src/transaction.rs#L362); [tekes-kernel-installer::transaction::failure_code](../../src/transaction.rs#L392); [tekes-kernel-installer::transaction::health](../../src/transaction.rs#L40) |
| [tekes-kernel-installer::transaction::execute](../../src/transaction.rs#L428) | `pub` | [tekes-kernel-installer::execute](../../src/lib.rs#L140) |
