use endpoint::{
    AUTOMATIC_TITLE_REFINE_OPERATION, AUTOMATIC_TITLE_SEED_OPERATION, EndpointJournal,
    NativeEndpoint,
};
use schema::{OriginTuple, ResumePolicy};

const SESSION: &str = "018f0000-0000-7000-8000-000000000120";

fn origin(operation: &str, key: &str) -> OriginTuple {
    OriginTuple {
        principal: "test".to_owned(),
        client: "endpoint-test".to_owned(),
        target: SESSION.to_owned(),
        op: operation.to_owned(),
        key: key.to_owned(),
    }
}

fn create(endpoint: &NativeEndpoint) {
    endpoint
        .create_session(
            SESSION,
            "workspace-1",
            "config-digest",
            ResumePolicy::Never,
            "2026-09-03T08:00:00.000Z",
            &origin("create", "create-1"),
        )
        .expect("create session");
}

#[test]
fn automatic_title_is_a_durable_one_shot_and_projects_each_visible_write() {
    let root = tempfile::tempdir().expect("root");
    let endpoint = NativeEndpoint::open(root.path()).expect("endpoint");
    create(&endpoint);
    let seed = origin(AUTOMATIC_TITLE_SEED_OPERATION, "automatic-seed");
    let first = endpoint
        .seed_automatic_title_if_missing(
            SESSION,
            "2026-09-03T08:00:01.000Z",
            &seed,
            "请修复登录问题",
        )
        .expect("seed")
        .expect("seed written");
    assert!(!first.deduplicated);
    let retry = endpoint
        .seed_automatic_title_if_missing(
            SESSION,
            "2026-09-03T08:00:01.000Z",
            &seed,
            "different fallback",
        )
        .expect("seed retry")
        .expect("idempotent seed receipt");
    assert_eq!(retry.seq, first.seq);
    assert!(retry.deduplicated);
    assert!(
        endpoint
            .seed_automatic_title_if_missing(
                SESSION,
                "2026-09-03T08:00:01.500Z",
                &origin(AUTOMATIC_TITLE_SEED_OPERATION, "competing-seed"),
                "competing fallback",
            )
            .expect("competing seed")
            .is_none(),
        "the durable title, not only the origin key, owns the one-shot claim"
    );

    let refine = origin(AUTOMATIC_TITLE_REFINE_OPERATION, "automatic-refine");
    let refined = endpoint
        .refine_automatic_title(SESSION, "2026-09-03T08:00:02.000Z", &refine, "修复登录故障")
        .expect("refine")
        .expect("refinement written");
    assert!(!refined.deduplicated);
    assert!(
        endpoint
            .refine_automatic_title(
                SESSION,
                "2026-09-03T08:00:02.500Z",
                &origin(AUTOMATIC_TITLE_REFINE_OPERATION, "competing-refine"),
                "second model title",
            )
            .expect("competing refine")
            .is_none(),
        "only the seed can be refined, so distinct dispatchers cannot append twice"
    );
    assert_eq!(
        endpoint
            .list_sessions(&Default::default())
            .expect("inventory")[0]
            .title
            .as_deref(),
        Some("修复登录故障")
    );

    endpoint
        .reconcile_projection(SESSION)
        .expect("project titles");
    let records = EndpointJournal::open(root.path().join("threads").join(SESSION))
        .expect("journal")
        .records()
        .expect("records");
    assert_eq!(
        records
            .iter()
            .filter(|record| record.event.event_type == "session/title")
            .count(),
        2
    );
}

#[test]
fn a_manual_rename_wins_the_refinement_race() {
    let root = tempfile::tempdir().expect("root");
    let endpoint = NativeEndpoint::open(root.path()).expect("endpoint");
    create(&endpoint);
    endpoint
        .seed_automatic_title_if_missing(
            SESSION,
            "2026-09-03T08:00:01.000Z",
            &origin(AUTOMATIC_TITLE_SEED_OPERATION, "automatic-seed"),
            "fallback",
        )
        .expect("seed")
        .expect("seed written");
    endpoint
        .rename_session_for_endpoint(
            SESSION,
            "2026-09-03T08:00:02.000Z",
            &origin("session.rename", "manual-rename"),
            "用户命名",
        )
        .expect("manual rename");

    assert!(
        endpoint
            .refine_automatic_title(
                SESSION,
                "2026-09-03T08:00:03.000Z",
                &origin(AUTOMATIC_TITLE_REFINE_OPERATION, "automatic-refine"),
                "模型命名",
            )
            .expect("conditional refine")
            .is_none()
    );
    assert_eq!(
        endpoint
            .list_sessions(&Default::default())
            .expect("inventory")[0]
            .title
            .as_deref(),
        Some("用户命名")
    );
}
