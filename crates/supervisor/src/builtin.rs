//! Application-owned child process entry point. stdin EOF ends the process;
//! stdout emits one readiness record. The launch file contains no credentials.
use crate::{
    endpoint_carrier::ProductionCarrierAssembly, host_runtime, process_host::ProductionProcessHost,
};
use serde::Deserialize;
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
    pub root: PathBuf,
    pub worker: PathBuf,
    pub listen: SocketAddr,
    pub providers: profile::ProvidersConfig,
    pub credential_bindings: BTreeMap<String, String>,
    /// Loopback address for the optional browser Web Client; absent means off.
    #[serde(default)]
    pub web_listen: Option<SocketAddr>,
}

pub async fn run(path: PathBuf) -> Result<()> {
    let launch: BuiltInLaunch = serde_json::from_slice(&std::fs::read(path)?)?;
    if launch.format != 1
        || !launch.root.is_absolute()
        || !launch.worker.is_absolute()
        || !launch.listen.ip().is_loopback()
        || launch
            .web_listen
            .is_some_and(|address| !address.ip().is_loopback())
    {
        return Err(io::Error::other("invalid built-in launch configuration").into());
    }
    let required: std::collections::BTreeSet<&str> = launch
        .providers
        .providers
        .iter()
        .filter_map(|provider| provider.credential_key.as_deref())
        .chain(
            launch
                .providers
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
    if required != supplied {
        return Err(io::Error::other("credential bindings must match launch configuration").into());
    }
    // Capture only explicitly selected credentials before creating any runtime.
    let secrets = Arc::new(provider::EnvironmentSecretStore::capture(
        &launch.credential_bindings,
    )?);
    let bearer = host_runtime::endpoint_token_from_environment()?;
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
    host_runtime::preflight_storage(&storage)?;
    let repository = profile::ConfigRepository::open(&launch.root)?;
    let current = repository.providers()?;
    let mut providers = launch.providers;
    providers.revision = current
        .revision
        .checked_add(1)
        .ok_or_else(|| io::Error::other("provider revision overflow"))?;
    repository.publish_providers(current.revision, &providers)?;
    // Keep a still-selected default; otherwise new sessions use the first enabled
    // model from the application's ordered launch configuration.
    let mut settings = repository.settings()?;
    let default_is_available = providers.providers.iter().any(|provider| {
        settings.default_provider.as_deref() == Some(provider.id.as_str())
            && provider.models.iter().any(|model| {
                model.enabled && settings.default_model.as_deref() == Some(model.id.as_str())
            })
    });
    if !default_is_available {
        let selected = providers.providers.iter().find_map(|provider| {
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
    let user_agent_dir = std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::other("HOME is required for shared user skills"))?
        .join(".agents");
    let build = option_env!("TEKES_SELECTED_BUILD").unwrap_or(env!("CARGO_PKG_VERSION"));
    let process = ProductionProcessHost::open_with_secret_authorities(
        &launch.root,
        launch.worker,
        build,
        &user_agent_dir,
        secrets,
        None,
    )?;
    process.preflight_mandatory_authorities()?;
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
    let listener = tokio::net::TcpListener::bind(launch.listen).await?;
    let address = listener.local_addr()?;
    let web_listener = match launch.web_listen {
        Some(web_listen) => Some(tokio::net::TcpListener::bind(web_listen).await?),
        None => None,
    };
    let config = transport::TransportConfig::loopback(address, transport::BearerToken::new(bearer));
    let assembly =
        ProductionCarrierAssembly::assemble(&launch.root, unary, process.clone(), config)?
            .with_file_changes(process.clone());
    process.attach_streams(assembly.streams().clone());
    process.defer_existing_session_recovery()?;
    process.boot_sweep()?;
    assembly.finish_recovery()?;
    process.start_periodic_sweep();
    process.start_schedule_timer()?;
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
    let mut ready = serde_json::json!({"type":"ready","protocolVersion":3,"url":format!("http://{address}"),"pid":std::process::id()});
    if let Some((_, _, bound)) = &web {
        ready["webUrl"] = format!("http://{bound}/").into();
    }
    println!("{ready}");
    io::stdout().flush()?;
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
