use profile::ConfigRepository;

fn write_pretty(path: &std::path::Path, canonical: &[u8]) -> Vec<u8> {
    let value: serde_json::Value = serde_json::from_slice(canonical).unwrap();
    let bytes = serde_json::to_vec_pretty(&value).unwrap();
    std::fs::write(path, &bytes).unwrap();
    bytes
}

#[test]
fn global_config_reads_pretty_legacy_json_without_rewriting_it() {
    let root = tempfile::tempdir().unwrap();
    let config = root.path().join("config");
    std::fs::create_dir_all(&config).unwrap();
    let providers = config.join("providers.json");
    let settings = config.join("settings.json");
    let provider_bytes = write_pretty(
        &providers,
        include_bytes!("../../../fixtures/config/providers.canonical.json"),
    );
    let settings_bytes = write_pretty(
        &settings,
        include_bytes!("../../../fixtures/config/settings.canonical.json"),
    );

    let repository = ConfigRepository::open(root.path()).unwrap();
    assert!(repository.providers().unwrap().revision > 0);
    assert!(repository.settings().unwrap().revision > 0);
    assert_eq!(std::fs::read(providers).unwrap(), provider_bytes);
    assert_eq!(std::fs::read(settings).unwrap(), settings_bytes);
}

#[test]
fn global_config_read_still_rejects_duplicate_members_and_null() {
    let root = tempfile::tempdir().unwrap();
    let config = root.path().join("config");
    std::fs::create_dir_all(&config).unwrap();
    let providers = config.join("providers.json");
    std::fs::write(
        &providers,
        br#"{ "format": 1, "format": 1, "revision": 1, "providers": [] }"#,
    )
    .unwrap();
    let repository = ConfigRepository::open(root.path()).unwrap();
    assert!(repository.providers().is_err());

    let valid = include_bytes!("../../../fixtures/config/providers.canonical.json");
    let value: serde_json::Value = serde_json::from_slice(valid).unwrap();
    let mut value = value;
    value["providers"] = serde_json::Value::Null;
    std::fs::write(&providers, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    assert!(repository.providers().is_err());
}

#[test]
fn global_config_publication_accepts_legacy_read_and_writes_canonical_bytes() {
    let root = tempfile::tempdir().unwrap();
    let config = root.path().join("config");
    std::fs::create_dir_all(&config).unwrap();
    let providers_path = config.join("providers.json");
    let settings_path = config.join("settings.json");
    write_pretty(
        &providers_path,
        include_bytes!("../../../fixtures/config/providers.canonical.json"),
    );
    write_pretty(
        &settings_path,
        include_bytes!("../../../fixtures/config/settings.canonical.json"),
    );
    let repository = ConfigRepository::open(root.path()).unwrap();
    let mut settings = repository.settings().unwrap();
    let expected = settings.revision;
    settings.revision += 1;
    repository.publish_settings(expected, &settings).unwrap();
    let bytes = std::fs::read(settings_path).unwrap();
    assert!(bytes.ends_with(b"\n"));
    assert!(!bytes.windows(2).any(|pair| pair == b"\n "));
    assert_eq!(repository.settings().unwrap(), settings);
    assert_eq!(repository.providers().unwrap().revision, 4);
}
