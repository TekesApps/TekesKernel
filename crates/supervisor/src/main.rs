use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

const WEB_CLIENT_URL: &str = "http://127.0.0.1:7357/";

fn main() -> ExitCode {
    if env::args_os().len() == 2
        && env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--models-available"))
    {
        return match provider::advertised_dialect_proofs().and_then(|proofs| {
            serde_json::to_string(&proofs)
                .map_err(|_| provider::DialectError::UnprovedProfile("catalog".into()))
        }) {
            Ok(catalog) => {
                println!("{catalog}");
                ExitCode::SUCCESS
            }
            Err(_) => {
                eprintln!("tekes-supervisor: model catalog unavailable");
                ExitCode::from(74)
            }
        };
    }
    if env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--built-in")) {
        let args = env::args_os().collect::<Vec<_>>();
        if args.len() != 3 {
            return ExitCode::from(64);
        }
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();
        let result = match runtime {
            Ok(runtime) => {
                runtime.block_on(tekes_supervisor::builtin::run(PathBuf::from(&args[2])))
            }
            Err(error) => Err(Box::new(error) as Box<dyn std::error::Error + Send + Sync>),
        };
        return match result {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("tekes-supervisor: {error}");
                ExitCode::from(74)
            }
        };
    }

    if env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--web-launch-url")) {
        return run_web_launch_url();
    }
    if env::args_os().len() == 2
        && env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--describe-web-client"))
    {
        println!(
            "{{\"format\":1,\"sha256\":\"{}\"}}",
            transport::WEB_CLIENT_SHA256,
        );
        return ExitCode::SUCCESS;
    }
    if env::args_os().len() == 2
        && env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--describe-build"))
    {
        let embedded_build =
            option_env!("TEKES_SELECTED_BUILD").unwrap_or(env!("CARGO_PKG_VERSION"));
        let source_revision = tekes_supervisor::daemon::SOURCE_REVISION
            .map(|revision| format!("\"source_revision\":\"{revision}\","))
            .unwrap_or_default();
        println!(
            "{{\"authority_registry_sha256\":\"{}\",\"format\":1,\"identifier\":\"com.tekes.kernel.supervisor\",{}\"version\":\"{}\"}}",
            tekes_supervisor::daemon::AUTHORITY_REGISTRY_SHA256,
            source_revision,
            embedded_build,
        );
        return ExitCode::SUCCESS;
    }
    if env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("support-bundle")) {
        return run_support_bundle();
    }
    if env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--install-root")) {
        return run_production();
    }
    eprintln!(
        "tekes-supervisor: usage: --install-root ... | support-bundle ... | --describe-build | --describe-web-client | --web-launch-url ..."
    );
    ExitCode::from(64)
}

fn run_web_launch_url() -> ExitCode {
    let arguments = env::args_os().skip(1).collect::<Vec<_>>();
    let usage = || {
        eprintln!(
            "tekes-supervisor: usage: --web-launch-url --access-group TEAM.com.tekes.shared.endpoint"
        );
        ExitCode::from(64)
    };
    if arguments.len() != 3 || arguments[1] != "--access-group" {
        return usage();
    }
    let Some(access_group) = arguments[2].to_str() else {
        return usage();
    };
    let Some(team) = access_group.strip_suffix(".com.tekes.shared.endpoint") else {
        return usage();
    };
    if team.len() != 10
        || !team
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
    {
        return usage();
    }
    println!("{WEB_CLIENT_URL}");
    ExitCode::SUCCESS
}

fn run_support_bundle() -> ExitCode {
    let arguments = env::args_os().skip(1).collect::<Vec<_>>();
    let usage = || {
        eprintln!(
            "tekes-supervisor: usage: support-bundle --destination ABSOLUTE --log-root ABSOLUTE --build ID --capability-digest HEX --config-digests HEX[,HEX...]"
        );
        ExitCode::from(64)
    };
    if arguments.len() != 11
        || arguments[0] != "support-bundle"
        || arguments[1] != "--destination"
        || arguments[3] != "--log-root"
        || arguments[5] != "--build"
        || arguments[7] != "--capability-digest"
        || arguments[9] != "--config-digests"
    {
        return usage();
    }
    let destination = PathBuf::from(&arguments[2]);
    let log_root = PathBuf::from(&arguments[4]);
    let Some(build) = arguments[6].to_str() else {
        return usage();
    };
    let Some(capability_digest) = arguments[8].to_str() else {
        return usage();
    };
    let Some(config_digests) = arguments[10].to_str() else {
        return usage();
    };
    if !destination.is_absolute()
        || !log_root.is_absolute()
        || destination.exists()
        || build.is_empty()
        || config_digests.is_empty()
    {
        return usage();
    }
    let config_digests = config_digests
        .split(',')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let created_at = match tekes_supervisor::daemon::system_timestamp() {
        Ok(value) => value,
        Err(error) => {
            eprintln!("tekes-supervisor: {error}");
            return ExitCode::from(74);
        }
    };
    match tekes_supervisor::observability::publish_support_bundle_from_files(
        &destination,
        &created_at,
        build,
        capability_digest,
        &config_digests,
        &log_root,
    ) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("tekes-supervisor: {error}");
            ExitCode::from(74)
        }
    }
}

fn run_production() -> ExitCode {
    let arguments = env::args_os().skip(1).collect::<Vec<_>>();
    let args = match tekes_supervisor::daemon::DaemonArgs::parse(arguments) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("tekes-supervisor: {error}");
            return ExitCode::from(error.exit_code());
        }
    };
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            eprintln!("tekes-supervisor: {error}");
            return ExitCode::from(74);
        }
    };
    match runtime.block_on(tekes_supervisor::daemon::run_daemon(args)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("tekes-supervisor: {error}");
            ExitCode::from(error.exit_code())
        }
    }
}
