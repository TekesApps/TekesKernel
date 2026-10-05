use std::fs::{self, OpenOptions};
use std::io::Write as _;

use chrono::{DateTime, Utc};
use schedule::{
    CronSchedule, MissedPolicy, OriginTuple, RunStatus, ScheduleAuthority, ScheduleDefinition,
    ScheduleError,
};
use serde_json::Value;
use tempfile::TempDir;

const TASK: &str = "018f0000-0000-7000-8000-0000000000c1";
const TASK_2: &str = "018f0000-0000-7000-8000-0000000000c2";
const SESSION: &str = "018f0000-0000-7000-8000-0000000000d1";

#[test]
fn slice14c_gate_101_contract_oracle_cron_and_timezone() {
    let fixture = fixture("cases.canonical.json");
    for case in fixture["cron_cases"].as_array().expect("cron cases") {
        let cron = CronSchedule::parse(case["cron"].as_str().expect("cron")).expect("valid cron");
        let actual = cron
            .next_after(
                time(case["after"].as_str().expect("after")),
                case["time_zone"].as_str().expect("zone"),
            )
            .expect("next occurrence");
        assert_eq!(
            actual.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            case["next"].as_str().expect("next")
        );
    }
    assert!(CronSchedule::parse("0 0 * *").is_err());
    assert!(CronSchedule::parse("60 * * * *").is_err());
    assert!(CronSchedule::parse("*/0 * * * *").is_err());
    assert!(CronSchedule::parse("0\t0 * * *").is_err());
    let star = CronSchedule::parse("0 0 * * 1").expect("star DOM");
    let stepped_star = CronSchedule::parse("0 0 */1 * 1").expect("full-domain DOM");
    assert_eq!(
        star.next_after(time("2026-06-02T00:00:00Z"), "UTC")
            .expect("next Monday"),
        stepped_star
            .next_after(time("2026-06-02T00:00:00Z"), "UTC")
            .expect("same next Monday")
    );

    let expected: ScheduleDefinition =
        serde_json::from_value(fixture["definition"].clone()).expect("definition fixture");
    assert_eq!(expected, definition(TASK));
}

#[test]
fn slice14c_gate_102_keyed_management_durability_and_dedup() {
    let root = TempDir::new().expect("temp root");
    let authority = ScheduleAuthority::open(root.path()).expect("authority");
    let saved = authority
        .save(
            origin("save-1"),
            definition(TASK),
            time("2026-08-29T04:01:00Z"),
        )
        .expect("save");
    assert_eq!(saved.next_run_at.as_deref(), Some("2026-08-29T04:15:00Z"));
    assert_eq!(saved.last_status, RunStatus::Idle);
    assert_eq!(authority.list(None).expect("list"), vec![saved.clone()]);

    let bytes_after_save = fs::read(authority.log_path()).expect("durable log");
    assert!(bytes_after_save.ends_with(b"\n"));
    let reopened = ScheduleAuthority::open(root.path()).expect("reopen");
    assert_eq!(
        reopened
            .save(
                origin("save-1"),
                definition(TASK),
                time("2027-01-01T00:00:00Z")
            )
            .expect("deduplicated save"),
        saved
    );
    assert_eq!(
        fs::read(authority.log_path()).expect("same log"),
        bytes_after_save
    );
    let mut changed = definition(TASK);
    changed.prompt = "different".to_owned();
    assert!(matches!(
        reopened.save(origin("save-1"), changed, time("2026-08-29T04:02:00Z")),
        Err(ScheduleError::OriginCollision)
    ));

    let claim = reopened
        .run_now(origin("run-1"), TASK, time("2026-08-29T04:02:00Z"))
        .expect("run now claim");
    assert!(claim.claim_id.starts_with("manual-"));
    let retry = reopened
        .run_now(origin("run-1"), TASK, time("2026-08-29T05:02:00Z"))
        .expect("run-now reack");
    assert_eq!(retry, claim);
    assert!(matches!(
        reopened.delete(origin("delete-active"), TASK, time("2026-08-29T04:03:00Z")),
        Err(ScheduleError::Active(_))
    ));
    let running = reopened
        .bind_launch(
            TASK,
            &claim.claim_id,
            SESSION,
            2,
            time("2026-08-29T04:03:00Z"),
        )
        .expect("bind");
    assert_eq!(running.last_status, RunStatus::Running);
    let parked = reopened
        .record_status(
            TASK,
            &claim.claim_id,
            RunStatus::Parked,
            None,
            time("2026-08-29T04:04:00Z"),
        )
        .expect("park");
    assert_eq!(parked.last_status, RunStatus::Parked);
    let completed = reopened
        .record_status(
            TASK,
            &claim.claim_id,
            RunStatus::Completed,
            None,
            time("2026-08-29T04:05:00Z"),
        )
        .expect("complete");
    assert_eq!(completed.last_status, RunStatus::Completed);
    assert!(completed.active_claim_id.is_none());
    assert!(
        reopened
            .delete(origin("delete-1"), TASK, time("2026-08-29T04:06:00Z"))
            .expect("delete")
    );
    assert!(reopened.list(None).expect("empty").is_empty());
    assert_eq!(
        reopened
            .run_now(origin("run-1"), TASK, time("2026-08-29T06:00:00Z"))
            .expect("historical reack after delete"),
        claim
    );
}

#[test]
fn slice14c_gate_103_claim_redrive_restart_and_missed_policy() {
    let root = TempDir::new().expect("temp root");
    let authority = ScheduleAuthority::open(root.path()).expect("authority");
    let mut hourly = definition(TASK);
    hourly.cron = "0 * * * *".to_owned();
    hourly.time_zone = "UTC".to_owned();
    authority
        .save(origin("save-hourly"), hourly, time("2026-08-29T00:00:00Z"))
        .expect("save hourly");
    let claims = authority
        .poll_due(time("2026-08-29T01:00:00Z"))
        .expect("claim due");
    assert_eq!(claims.len(), 1);
    let claim = claims[0].clone();
    let bytes_after_claim = fs::read(authority.log_path()).expect("claim durable");
    drop(authority);

    let restarted = ScheduleAuthority::open(root.path()).expect("restart");
    let recovered = restarted
        .recover(time("2026-08-29T01:01:00Z"))
        .expect("recover claim");
    assert_eq!(recovered, vec![claim.clone()]);
    assert_eq!(
        fs::read(restarted.log_path()).expect("unchanged"),
        bytes_after_claim
    );
    restarted
        .bind_launch(
            TASK,
            &claim.claim_id,
            SESSION,
            2,
            time("2026-08-29T01:02:00Z"),
        )
        .expect("bind after keyed redrive");
    assert!(
        restarted
            .recover(time("2026-08-29T01:03:00Z"))
            .expect("bound recovery")
            .is_empty()
    );

    assert!(
        restarted
            .poll_due(time("2026-08-29T02:00:00Z"))
            .expect("active occurrence is missed")
            .is_empty()
    );
    let active = restarted.list(None).expect("active view").remove(0);
    assert_eq!(active.missed_at.as_deref(), Some("2026-08-29T02:00:00Z"));
    assert_eq!(active.next_run_at.as_deref(), Some("2026-08-29T03:00:00Z"));
    restarted
        .record_status(
            TASK,
            &claim.claim_id,
            RunStatus::Completed,
            None,
            time("2026-08-29T02:01:00Z"),
        )
        .expect("terminal observation");

    let mut second = definition(TASK_2);
    second.cron = "0 * * * *".to_owned();
    second.time_zone = "UTC".to_owned();
    restarted
        .save(origin("save-second"), second, time("2026-08-29T00:00:00Z"))
        .expect("second schedule");
    assert!(
        restarted
            .recover(time("2026-08-29T05:30:00Z"))
            .expect("startup skip")
            .is_empty()
    );
    let second_view = restarted
        .list(None)
        .expect("views")
        .into_iter()
        .find(|view| view.definition.id == TASK_2)
        .expect("second view");
    assert_eq!(
        second_view.missed_at.as_deref(),
        Some("2026-08-29T01:00:00Z")
    );
    assert_eq!(
        second_view.next_run_at.as_deref(),
        Some("2026-08-29T06:00:00Z")
    );
}

#[test]
fn slice14c_gate_104_fail_closed_nonmutation_and_internal_boundary() {
    let root = TempDir::new().expect("temp root");
    let authority = ScheduleAuthority::open(root.path()).expect("authority");
    let before = fs::read(authority.log_path()).unwrap_or_default();
    let mut invalid = definition(TASK);
    invalid.cron = "60 * * * *".to_owned();
    assert!(matches!(
        authority.save(origin("invalid"), invalid, time("2026-08-29T00:00:00Z")),
        Err(ScheduleError::InvalidCron)
    ));
    assert_eq!(fs::read(authority.log_path()).unwrap_or_default(), before);

    authority
        .save(
            origin("save"),
            definition(TASK),
            time("2026-08-29T04:01:00Z"),
        )
        .expect("save");
    let claim = authority
        .run_now(origin("run"), TASK, time("2026-08-29T04:02:00Z"))
        .expect("claim");
    let before_mismatch = fs::read(authority.log_path()).expect("before mismatch");
    assert!(matches!(
        authority.bind_launch(
            TASK,
            "manual-wrong",
            SESSION,
            2,
            time("2026-08-29T04:03:00Z")
        ),
        Err(ScheduleError::ClaimMismatch)
    ));
    assert_eq!(
        fs::read(authority.log_path()).expect("no mutation"),
        before_mismatch
    );

    let mut file = OpenOptions::new()
        .append(true)
        .open(authority.log_path())
        .expect("append torn tail");
    file.write_all(b"{\"op\":\"save\"").expect("partial write");
    drop(file);
    let second = definition(TASK_2);
    authority
        .save(origin("tail-repair"), second, time("2026-08-29T04:04:00Z"))
        .expect("mutation repairs final partial line");
    assert!(
        fs::read(authority.log_path())
            .expect("repaired")
            .ends_with(b"\n")
    );

    let completed_corrupt = root.path().join("corrupt");
    fs::create_dir_all(&completed_corrupt).expect("corrupt root");
    fs::write(
        completed_corrupt.join("schedules.jsonl"),
        b"{\"format\":1,\"op\":\"unknown\",\"seq\":1}\n",
    )
    .expect("complete corrupt line");
    let corrupt = ScheduleAuthority::open(&completed_corrupt).expect("corrupt authority");
    assert!(matches!(corrupt.list(None), Err(ScheduleError::Corrupt(_))));

    let bound = authority
        .bind_launch(
            TASK,
            &claim.claim_id,
            SESSION,
            2,
            time("2026-08-29T04:05:00Z"),
        )
        .expect("original claim still bindable");
    assert_eq!(bound.last_status, RunStatus::Running);
}

fn fixture(name: &str) -> Value {
    serde_json::from_slice(
        &fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../fixtures/schedule")
                .join(name),
        )
        .expect("fixture bytes"),
    )
    .expect("fixture JSON")
}

fn time(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .expect("RFC3339")
        .with_timezone(&Utc)
}

fn origin(key: &str) -> OriginTuple {
    OriginTuple {
        client_id: "test-client".to_owned(),
        key: key.to_owned(),
    }
}

fn definition(id: &str) -> ScheduleDefinition {
    ScheduleDefinition {
        id: id.to_owned(),
        name: "Quarter-hour review".to_owned(),
        workspace_id: "ws-main".to_owned(),
        cron: "*/15 * * * *".to_owned(),
        time_zone: "Asia/Shanghai".to_owned(),
        prompt: "Review the workspace and report changes.".to_owned(),
        permission_mode: "workspace-write".to_owned(),
        model_id: Some("openai:gpt-5".to_owned()),
        enabled: true,
        missed_policy: MissedPolicy::SkipAndRecord,
    }
}
