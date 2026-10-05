# schedule — public/restricted declarations and direct callers

[Package atlas](index.md)

Includes pub, pub(crate), pub(super), and other restricted declarations; a pub method in a binary is not an importable library API. A missing direct caller is not evidence of dead code. Trait implementation methods are indexed in source pages even without a pub keyword.

| Declaration | Visibility | Direct caller functions (all indexed configurations) |
|---|---|---|
| [schedule::OriginTuple](../../src/lib.rs#L34) | `pub` | not a function |
| [schedule::MissedPolicy](../../src/lib.rs#L41) | `pub` | not a function |
| [schedule::ScheduleDefinition](../../src/lib.rs#L47) | `pub` | not a function |
| [schedule::RunStatus](../../src/lib.rs#L63) | `pub` | not a function |
| [schedule::RunStatus::is_active](../../src/lib.rs#L75) | `pub` | no resolved direct caller |
| [schedule::ClaimReason](../../src/lib.rs#L82) | `pub` | not a function |
| [schedule::LaunchClaim](../../src/lib.rs#L89) | `pub` | not a function |
| [schedule::ScheduleView](../../src/lib.rs#L99) | `pub` | not a function |
| [schedule::ScheduleAuthority](../../src/lib.rs#L211) | `pub` | not a function |
| [schedule::ScheduleAuthority::open](../../src/lib.rs#L216) | `pub` | [schedule::tests::slice14c_gates::slice14c_gate_103_claim_redrive_restart_and_missed_policy](../../tests/slice14c_gates.rs#L148); [schedule::tests::slice14c_gates::slice14c_gate_104_fail_closed_nonmutation_and_internal_boundary](../../tests/slice14c_gates.rs#L238); [schedule::tests::slice14c_gates::slice14c_gate_102_keyed_management_durability_and_dedup](../../tests/slice14c_gates.rs#L52); [tekes-supervisor::process_host::ProductionProcessHost::open_with_secret_authorities](../../../supervisor/src/process_host.rs#L782) |
| [schedule::ScheduleAuthority::list](../../src/lib.rs#L227) | `pub` | no resolved direct caller |
| [schedule::ScheduleAuthority::save](../../src/lib.rs#L240) | `pub` | no resolved direct caller |
| [schedule::ScheduleAuthority::delete](../../src/lib.rs#L295) | `pub` | no resolved direct caller |
| [schedule::ScheduleAuthority::run_now](../../src/lib.rs#L340) | `pub` | no resolved direct caller |
| [schedule::ScheduleAuthority::poll_due](../../src/lib.rs#L390) | `pub` | no resolved direct caller |
| [schedule::ScheduleAuthority::recover](../../src/lib.rs#L447) | `pub` | no resolved direct caller |
| [schedule::ScheduleAuthority::bind_launch](../../src/lib.rs#L480) | `pub` | no resolved direct caller |
| [schedule::ScheduleAuthority::record_status](../../src/lib.rs#L526) | `pub` | no resolved direct caller |
| [schedule::ScheduleAuthority::log_path](../../src/lib.rs#L579) | `pub` | no resolved direct caller |
| [schedule::CronSchedule](../../src/lib.rs#L1088) | `pub` | not a function |
| [schedule::CronSchedule::parse](../../src/lib.rs#L1103) | `pub` | [schedule::ScheduleAuthority::save](../../src/lib.rs#L240); [schedule::ScheduleAuthority::poll_due](../../src/lib.rs#L390); [schedule::ScheduleAuthority::recover](../../src/lib.rs#L447); [schedule::validate_definition](../../src/lib.rs#L584); [schedule::tests::slice14c_gates::slice14c_gate_101_contract_oracle_cron_and_timezone](../../tests/slice14c_gates.rs#L17) |
| [schedule::CronSchedule::next_after](../../src/lib.rs#L1124) | `pub` | no resolved direct caller |
| [schedule::ScheduleError](../../src/lib.rs#L1230) | `pub` | not a function |
