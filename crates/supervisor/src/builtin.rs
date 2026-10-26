//! Application-owned child process entry point. stdin EOF ends the process;
//! stdout emits one readiness record. The launch file contains no credentials.
use crate::{
    endpoint_carrier::ProductionCarrierAssembly, host_runtime, process_host::ProductionProcessHost,
};
use serde::{Deserialize, Deserializer};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::{
    collections::BTreeMap,
    io::{self, Write},
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuiltInLaunch {
    pub format: u64,
    pub bootstrap_version: u64,
    pub root: PathBuf,
    pub worker: PathBuf,
    pub listen: SocketAddr,
    /// `provided` is the legacy launch contract. `existing` consumes the
    /// already-published configuration under `root` without replacing it.
    #[serde(default)]
    pub configuration_mode: ConfigurationMode,
    #[serde(default)]
    pub providers: LaunchProviders,
    pub credential_bindings: BTreeMap<String, String>,
    #[serde(default)]
    pub stdio_channels: BTreeMap<String, PathBuf>,
    /// Loopback address for the optional browser Web Client; absent means off.
    #[serde(default)]
    pub web_listen: Option<SocketAddr>,
}

#[derive(Debug, Default)]
pub enum LaunchProviders {
    #[default]
    Omitted,
    Supplied(profile::ProvidersConfig),
}

impl<'de> Deserialize<'de> for LaunchProviders {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        profile::ProvidersConfig::deserialize(deserializer).map(Self::Supplied)
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConfigurationMode {
    #[default]
    Provided,
    Existing,
}

fn configured_providers(launch: &BuiltInLaunch) -> Result<profile::ProvidersConfig> {
    match (launch.configuration_mode, &launch.providers) {
        (ConfigurationMode::Provided, LaunchProviders::Supplied(providers)) => {
            Ok(providers.clone())
        }
        (ConfigurationMode::Provided, LaunchProviders::Omitted) => {
            Err(io::Error::other("provided configuration mode requires providers").into())
        }
        (ConfigurationMode::Existing, LaunchProviders::Supplied(_)) => {
            Err(io::Error::other("existing configuration mode forbids launch providers").into())
        }
        (ConfigurationMode::Existing, LaunchProviders::Omitted) => {
            read_existing_configuration(&launch.root)
        }
    }
}

fn read_existing_configuration(root: &std::path::Path) -> Result<profile::ProvidersConfig> {
    // Check both durable authorities before opening ConfigRepository: its open
    // path is allowed to prepare directories for new installations. Existing
    // mode must never bootstrap or synthesize a missing config file.
    for name in ["providers.json", "settings.json"] {
        let path = root.join("config").join(name);
        if !path.is_file() {
            return Err(io::Error::other(format!(
                "existing configuration requires {}",
                path.display()
            ))
            .into());
        }
    }
    // Reject malformed JSON before the repository acquires its config lock.
    // Repository reads below retain the authoritative schema validation.
    let _: profile::ProvidersConfig =
        serde_json::from_slice(&std::fs::read(root.join("config/providers.json"))?)?;
    let _: profile::SettingsConfig =
        serde_json::from_slice(&std::fs::read(root.join("config/settings.json"))?)?;
    let repository = profile::ConfigRepository::open(root)?;
    let providers = repository.providers()?;
    // Reading settings validates the selected default and ensures the existing
    // configuration is complete before any session storage is touched.
    let settings = repository.settings()?;
    match settings.default_provider.as_deref() {
        None if settings.default_model.is_some() => {
            return Err(
                io::Error::other("existing settings select a model without a provider").into(),
            );
        }
        None => {}
        Some(provider_id) => {
            let provider = providers
                .providers
                .iter()
                .find(|provider| provider.id == provider_id)
                .ok_or_else(|| io::Error::other("existing settings select an unknown provider"))?;
            if settings.default_model.as_deref().is_some_and(|model_id| {
                !provider
                    .models
                    .iter()
                    .any(|model| model.enabled && model.id == model_id)
            }) {
                return Err(
                    io::Error::other("existing settings select an unavailable model").into(),
                );
            }
        }
    }
    Ok(providers)
}

fn validate_credential_bindings(
    launch: &BuiltInLaunch,
    providers: &profile::ProvidersConfig,
) -> Result<()> {
    let configured: std::collections::BTreeSet<&str> = providers
        .providers
        .iter()
        .filter_map(|provider| provider.credential_key.as_deref())
        .chain(
            providers
                .web_search
                .iter()
                .map(|search| search.credential_key.as_str()),
        )
        .collect();
    let supplied: std::collections::BTreeSet<&str> = launch
        .credential_bindings
        .keys()
        .map(String::as_str)
        .collect();
    match launch.configuration_mode {
        ConfigurationMode::Provided if !configured.is_subset(&supplied) => Err(io::Error::other(
            "launch credentials must include every configured provider binding",
        )
        .into()),
        ConfigurationMode::Existing if !supplied.is_subset(&configured) => {
            Err(io::Error::other("existing configuration has unknown credential bindings").into())
        }
        _ => Ok(()),
    }
}

pub async fn run(path: PathBuf) -> Result<()> {
    let launch: BuiltInLaunch = serde_json::from_slice(&std::fs::read(path)?)?;
    if launch.format != 1
        || launch.bootstrap_version != crate::bootstrap::VERSION
        || !launch.root.is_absolute()
        || !launch.worker.is_absolute()
        || !launch.listen.ip().is_loopback()
        || launch
            .web_listen
            .is_some_and(|address| !address.ip().is_loopback())
    {
        return Err(io::Error::other("invalid built-in launch configuration").into());
    }
    // This optional web surface currently has no authentication beyond
    // Host/Origin. It cannot share an administrative host with session tools.
    if launch.web_listen.is_some() {
        return Err(io::Error::other("unauthenticated-web-client-disabled").into());
    }
    // Load and validate existing files before creating thread storage or any
    // other root state. A malformed/missing config must fail without writes.
    let providers = configured_providers(&launch)?;
    if launch.credential_bindings.values().any(|name| {
        name.is_empty()
            || !name.bytes().enumerate().all(|(index, byte)| {
                byte == b'_' || byte.is_ascii_alphabetic() || (index > 0 && byte.is_ascii_digit())
            })
    }) {
        return Err(io::Error::other("invalid-launch-credential-bindings").into());
    }
    if crate::bootstrap::rejects_legacy_environment(&launch.credential_bindings) {
        return Err(io::Error::other("legacy-secret-environment-rejected").into());
    }
    validate_credential_bindings(&launch, &providers)?;
    let bootstrap =
        crate::bootstrap::read(0, &launch.credential_bindings, crate::bootstrap::DEADLINE)?;
    let bearer = transport::BearerToken::new(*bootstrap.token);
    drop(bootstrap.token);
    let secrets = Arc::new(bootstrap.store);
    crate::boot_trace::start();
    let storage = launch.root.join("threads");
    std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&storage)?;
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(storage.join(".root-lock"))?;
    let _lock = host_runtime::ProductionRootLock::acquire(&storage)?;
    crate::boot_trace::phase("root_lock");
    host_runtime::preflight_storage(&storage)?;
    crate::boot_trace::phase("preflight_storage");
    if launch.configuration_mode == ConfigurationMode::Provided {
        let repository = profile::ConfigRepository::open(&launch.root)?;
        let current = repository.providers()?;
        let mut provided = providers.clone();
        provided.revision = current
            .revision
            .checked_add(1)
            .ok_or_else(|| io::Error::other("provider revision overflow"))?;
        repository.publish_providers(current.revision, &provided)?;
        // Keep a still-selected default; otherwise new sessions use the first enabled
        // model from the application's ordered launch configuration.
        let mut settings = repository.settings()?;
        let default_is_available = provided.providers.iter().any(|provider| {
            settings.default_provider.as_deref() == Some(provider.id.as_str())
                && provider.models.iter().any(|model| {
                    model.enabled && settings.default_model.as_deref() == Some(model.id.as_str())
                })
        });
        if !default_is_available {
            let selected = provided.providers.iter().find_map(|provider| {
                provider
                    .models
                    .iter()
                    .find(|model| model.enabled)
                    .map(|model| (provider.id.clone(), model.id.clone()))
            });
            let (provider, model) = selected.map_or((None, None), |(p, m)| (Some(p), Some(m)));
            if settings.default_provider != provider || settings.default_model != model {
                let revision = settings.revision;
                settings.revision = revision
                    .checked_add(1)
                    .ok_or_else(|| io::Error::other("settings revision overflow"))?;
                settings.default_provider = provider;
                settings.default_model = model;
                repository.publish_settings(revision, &settings)?;
            }
        }
    }
    crate::boot_trace::phase("config");
    let user_agent_dir = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::other("HOME is required for shared user skills"))?
        .join(".agents");
    let build = option_env!("TEKES_SELECTED_BUILD").unwrap_or(env!("CARGO_PKG_VERSION"));
    let process = ProductionProcessHost::open_with_secret_store(
        &launch.root,
        launch.worker,
        build,
        &user_agent_dir,
        secrets,
    )?;
    process
        .mcp_runtime()
        .set_stdio_connector(Arc::new(crate::stdio_channels::UnixStdioChannels(
            launch.stdio_channels,
        )));
    crate::boot_trace::phase("process_host");
    process.preflight_mandatory_authorities()?;
    crate::boot_trace::phase("preflight_authorities");
    let unary = host_runtime::assemble_application_endpoint_host(
        &launch.root,
        endpoint::SessionHostDescription {
            version: build.into(),
            cwd: launch.root.to_string_lossy().into(),
            provider: None,
            model: None,
            attached_sessions: 0,
            home: launch.root.to_string_lossy().into(),
            can_open_path: false,
        },
        Arc::new(|| {
            host_runtime::system_timestamp()
                .map_err(crate::endpoint_host::EndpointAssemblyError::Clock)
        }),
        &user_agent_dir,
        Arc::clone(&process),
    )?;
    crate::boot_trace::phase("endpoint_host");
    let listener = tokio::net::TcpListener::bind(launch.listen).await?;
    let address = listener.local_addr()?;
    let web_listener = match launch.web_listen {
        Some(web_listen) => Some(tokio::net::TcpListener::bind(web_listen).await?),
        None => None,
    };
    let config = transport::TransportConfig::loopback(address, bearer);
    let assembly =
        ProductionCarrierAssembly::assemble(&launch.root, unary, process.clone(), config)?
            .with_file_changes(process.clone());
    crate::boot_trace::phase("carrier_assembled");
    process.attach_streams(assembly.streams().clone());
    // Readiness reads no thread (ADDENDUM-01 R1): sessions created, forked or unarchived from
    // here on are admitted; every other session is deferred until `sessions.recover`.
    process.arm_admission();
    assembly.mark_ready();
    process.start_periodic_sweep();
    crate::boot_trace::phase("ready_marked");
    host_runtime::install_termination_handler()?;
    let server = assembly.into_server();
    let handle = server.handle();
    // The Web Client checks Host and Origin against its configured address,
    // so configure it with the bound address rather than a requested port 0.
    let web = match web_listener {
        Some(listener) => {
            let bound = listener.local_addr()?;
            let service = server.web_client(transport::WebClientConfig::loopback(bound))?;
            Some((service, listener, bound))
        }
        None => None,
    };
    let mut ready = serde_json::json!({"type":"ready","protocolVersion":endpoint::SESSION_ENDPOINT_PROTOCOL_VERSION,"bootstrapVersion":crate::bootstrap::VERSION,"url":format!("http://{address}"),"pid":std::process::id()});
    if let Some((_, _, bound)) = &web {
        ready["webUrl"] = format!("http://{bound}/").into();
    }
    crate::boot_trace::phase("ready");
    println!("{ready}");
    io::stdout().flush()?;
    // Everything that touches threads starts only now, in the background.
    process.run_post_ready();
    let serve = server.serve(listener);
    tokio::pin!(serve);
    let web_serve = async move {
        match web {
            Some((service, listener, _)) => service.serve(listener).await,
            None => std::future::pending().await,
        }
    };
    tokio::pin!(web_serve);
    // A server failure must be able to exit even while the parent keeps stdin
    // open. Tokio waits for blocking tasks on runtime drop, so the lifetime
    // watcher must not occupy its blocking pool.
    let (lifetime_sender, lifetime) = tokio::sync::oneshot::channel();
    std::thread::Builder::new()
        .name("launcher-lifetime".into())
        .spawn(move || {
            let _ = lifetime_sender.send(host_runtime::wait_for_launcher_shutdown(0));
        })?;
    tokio::select! {
        result = &mut serve => { process.shutdown(); result?; }
        result = &mut web_serve => {
            // Either listener failing ends the shared lifetime.
            handle.begin_drain();
            process.shutdown();
            serve.await?;
            result?;
        }
        result = lifetime => {
            result??;
            handle.begin_drain();
            process.shutdown();
            serve.await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use profile::{Model, Provider, ProvidersConfig, SettingsConfig};
    use std::fs;

    fn provider(id: &str) -> Provider {
        Provider {
            id: id.into(),
            name: None,
            adapter: "openai_compatible".into(),
            dialect: "openai_chat_completions".into(),
            endpoint_owner: "user".into(),
            gateway_translation: "none".into(),
            evidence_revision: "v1".into(),
            endpoint: format!("https://{id}.example.com"),
            credential_key: Some(format!("{id}-credential")),
            models: vec![Model {
                id: format!("{id}-model"),
                profile: "test_profile".into(),
                enabled: true,
                context_window_tokens: 32_000,
                compact_trigger_tokens: 28_000,
            }],
        }
    }

    fn eight_provider_config() -> ProvidersConfig {
        ProvidersConfig {
            format: 1,
            revision: 7,
            providers: [
                "openai",
                "anthropic",
                "google",
                "deepseek",
                "glm",
                "qwen",
                "moonshot",
                "minimax",
            ]
            .into_iter()
            .map(provider)
            .collect(),
            web_search: None,
        }
    }

    fn base_launch(root: PathBuf) -> BuiltInLaunch {
        BuiltInLaunch {
            format: 1,
            bootstrap_version: crate::bootstrap::VERSION,
            root,
            worker: PathBuf::from("/tmp/worker"),
            listen: "127.0.0.1:0".parse().unwrap(),
            configuration_mode: ConfigurationMode::Existing,
            providers: LaunchProviders::Omitted,
            credential_bindings: BTreeMap::new(),
            stdio_channels: BTreeMap::new(),
            web_listen: None,
        }
    }

    #[test]
    fn existing_mode_reads_eight_providers_without_changing_config_bytes_or_revisions() {
        let root = tempfile::tempdir().unwrap();
        let repository = profile::ConfigRepository::open(root.path()).unwrap();
        let providers = eight_provider_config();
        fs::write(
            root.path().join("config/providers.json"),
            providers.canonical_bytes().unwrap(),
        )
        .unwrap();
        let settings = SettingsConfig {
            format: 1,
            revision: 11,
            default_provider: Some("glm".into()),
            default_model: Some("glm-model".into()),
            limits: None,
        };
        fs::write(
            root.path().join("config/settings.json"),
            settings.canonical_bytes().unwrap(),
        )
        .unwrap();
        let providers_path = root.path().join("config/providers.json");
        let settings_path = root.path().join("config/settings.json");
        let provider_bytes = fs::read(&providers_path).unwrap();
        let settings_bytes = fs::read(&settings_path).unwrap();

        let launch = base_launch(root.path().to_path_buf());
        let loaded = configured_providers(&launch).unwrap();
        assert_eq!(loaded, providers);
        assert_eq!(loaded.providers.len(), 8);
        assert_eq!(
            repository.settings().unwrap().default_provider.as_deref(),
            Some("glm")
        );
        assert_eq!(fs::read(providers_path).unwrap(), provider_bytes);
        assert_eq!(fs::read(settings_path).unwrap(), settings_bytes);
    }

    #[test]
    fn existing_mode_allows_missing_secret_bindings_but_rejects_unknown_ones() {
        let root = tempfile::tempdir().unwrap();
        let launch = base_launch(root.path().to_path_buf());
        let providers = eight_provider_config();
        // The endpoint still has all configured provider authorities; absence
        // from the bootstrap frame is a credential-store miss at invocation.
        assert!(validate_credential_bindings(&launch, &providers).is_ok());
        assert_eq!(providers.providers.len(), 8);
        let mut launch = launch;
        launch
            .credential_bindings
            .insert("not-configured".into(), "TEKES_UNUSED".into());
        assert!(validate_credential_bindings(&launch, &providers).is_err());
    }

    #[test]
    fn existing_mode_fails_missing_or_malformed_config_before_writing_root_state() {
        let root = tempfile::tempdir().unwrap();
        let launch = base_launch(root.path().to_path_buf());
        assert!(configured_providers(&launch).is_err());
        assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);

        fs::create_dir(root.path().join("config")).unwrap();
        fs::write(root.path().join("config/providers.json"), b"not-json").unwrap();
        fs::write(root.path().join("config/settings.json"), b"{}\n").unwrap();
        let before: Vec<_> = fs::read_dir(root.path().join("config"))
            .unwrap()
            .map(|entry| {
                (
                    entry.as_ref().unwrap().file_name(),
                    fs::read(entry.unwrap().path()).unwrap(),
                )
            })
            .collect();
        assert!(configured_providers(&launch).is_err());
        let after: Vec<_> = fs::read_dir(root.path().join("config"))
            .unwrap()
            .map(|entry| {
                (
                    entry.as_ref().unwrap().file_name(),
                    fs::read(entry.unwrap().path()).unwrap(),
                )
            })
            .collect();
        assert_eq!(before, after);
        assert!(!root.path().join("threads").exists());
    }

    #[test]
    fn legacy_launch_defaults_to_provided_and_existing_mode_rejects_mixed_input() {
        let providers = serde_json::to_value(eight_provider_config()).unwrap();
        let legacy = serde_json::json!({
            "format": 1,
            "bootstrap_version": crate::bootstrap::VERSION,
            "root": "/tmp/root",
            "worker": "/tmp/worker",
            "listen": "127.0.0.1:0",
            "providers": providers,
            "credential_bindings": {}
        });
        let launch: BuiltInLaunch = serde_json::from_value(legacy.clone()).unwrap();
        assert_eq!(launch.configuration_mode, ConfigurationMode::Provided);
        assert!(matches!(launch.providers, LaunchProviders::Supplied(_)));
        let mut mixed = legacy;
        mixed["configuration_mode"] = "existing".into();
        let mixed: BuiltInLaunch = serde_json::from_value(mixed).unwrap();
        assert!(configured_providers(&mixed).is_err());
    }
}
