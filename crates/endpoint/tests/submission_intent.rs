use endpoint::{NativeEndpoint, SessionEvent, SessionHistoryEntry};
use profile::{ConfigRepository, SessionSettings};
use schema::{IJsonValue, OriginTuple, ResumePolicy};
use serde_json::{Value, json};
use std::fs;

#[test]
fn submission_is_durable_before_run_and_retry_keeps_the_original_selection() {
    let root = tempfile::tempdir().unwrap();
    let repo = ConfigRepository::open(root.path()).unwrap();
    let fixtures = test_support::FixtureRoot::discover().unwrap();
    let mut workspace = profile::WorkspaceConfig::decode(
        &fs::read(fixtures.join("config/workspace.canonical.json")).unwrap(),
    )
    .unwrap();
    workspace.revision = 1;
    workspace.policy = None;
    workspace.cwd = vec![root.path().display().to_string()];
    repo.publish_workspace(0, &workspace).unwrap();
    let mut providers = profile::ProvidersConfig::decode(
        &fs::read(fixtures.join("config/providers.canonical.json")).unwrap(),
    )
    .unwrap();
    providers.revision = 1;
    repo.publish_providers(0, &providers).unwrap();
    let endpoint = NativeEndpoint::open(root.path()).unwrap();
    let id = "018f0000-0000-7000-8000-000000000009";
    let mut origin = OriginTuple {
        principal: "user".into(),
        client: "submission-test".into(),
        target: id.into(),
        op: "create".into(),
        key: "create-1".into(),
    };
    endpoint
        .create_session(
            id,
            &workspace.id,
            "creation-config",
            ResumePolicy::Never,
            "2026-09-04T00:00:00.000Z",
            &origin,
        )
        .unwrap();
    let folder = root.path().join("threads").join(id);
    let mut settings = SessionSettings {
        format: 1,
        revision: 1,
        provider: "openai".into(),
        model: "gpt-5".into(),
        reasoning_effort: Some("high".into()),
    };
    repo.publish_session_settings(&folder, 0, &settings)
        .unwrap();
    origin.op = "submit".into();
    origin.key = "send-1".into();
    let content =
        IJsonValue::parse_str(r#"[{"type":"text","text":"remember this submission"}]"#).unwrap();
    let receipt = endpoint
        .prompt(
            id,
            "2026-09-04T00:00:01.000Z",
            &origin,
            content.clone(),
            false,
        )
        .unwrap();
    let before = fs::read(folder.join("main.jsonl")).unwrap();
    let events: Vec<Value> = String::from_utf8(before.clone())
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(
        events.len(),
        2,
        "no run or response is required to save intent"
    );
    let submission = &events[1]["submission"];
    assert_eq!(submission["provider"], "openai");
    assert_eq!(submission["model"], "gpt-5");
    assert_eq!(submission["reasoningEffort"], "high");
    let digest = submission["configDigest"].as_str().unwrap();
    let captured = profile::ConfigSnapshot::decode(
        &fs::read(folder.join("assets").join(format!("sha256-{digest}"))).unwrap(),
    )
    .unwrap();
    assert_eq!(captured.session_settings.as_ref(), Some(&settings));
    settings.revision = 2;
    settings.reasoning_effort = Some("low".into());
    repo.publish_session_settings(&folder, 1, &settings)
        .unwrap();
    let retry = endpoint
        .prompt(id, "2026-09-04T00:00:02.000Z", &origin, content, false)
        .unwrap();
    assert!(retry.deduplicated);
    assert_eq!(receipt.seq, retry.seq);
    assert_eq!(fs::read(folder.join("main.jsonl")).unwrap(), before);
}

#[test]
#[ignore = "requires an explicitly selected existing session folder; operates only on a copy"]
fn existing_history_uses_its_recorded_config_without_mutating_the_journal() {
    let source = std::path::PathBuf::from(std::env::var("TEKES_HISTORY_SESSION_FOLDER").unwrap());
    let root = tempfile::tempdir().unwrap();
    let id = source.file_name().unwrap().to_str().unwrap();
    let folder = root.path().join("threads").join(id);
    fs::create_dir_all(folder.join("assets")).unwrap();
    fs::copy(source.join("main.jsonl"), folder.join("main.jsonl")).unwrap();
    for file in fs::read_dir(source.join("assets")).unwrap().flatten() {
        if file.file_type().unwrap().is_file() {
            fs::copy(file.path(), folder.join("assets").join(file.file_name())).unwrap();
        }
    }
    let before = fs::read(folder.join("main.jsonl")).unwrap();
    let mut entries = vec![SessionHistoryEntry {
        event: SessionEvent {
            event_type: "user/message".into(),
            seq: 1,
            time: 1.0,
            data: IJsonValue::parse(
                &serde_json::to_vec(&json!({"id":"input-2","content":[]})).unwrap(),
            )
            .unwrap(),
            ignorable: None,
            source_event_seqs: None,
            surface_op: None,
        },
        view: None,
    }];
    NativeEndpoint::open(root.path())
        .unwrap()
        .hydrate_historical_submissions(id, &mut entries);
    let data = serde_json::to_value(&entries[0].event.data).unwrap();
    assert_eq!(data["submission"]["model"], "deepseek-v4-flash");
    assert_eq!(
        data["submission"]["configDigest"],
        "59bf64b87a24f95ea46dde7fce3be639291ff5bc9888f41b6660ce883a282ca2"
    );
    assert_eq!(fs::read(folder.join("main.jsonl")).unwrap(), before);
}
