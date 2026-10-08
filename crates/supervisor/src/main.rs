use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    if env::args_os().len() == 2
        && env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--models-available"))
    {
        return match serde_json::to_string(&provider::supported_dialects()) {
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

    if env::args_os().len() == 2
        && env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--describe-build"))
    {
        let embedded_build =
            option_env!("TEKES_SELECTED_BUILD").unwrap_or(env!("CARGO_PKG_VERSION"));
        let source_revision = tekes_supervisor::host_runtime::SOURCE_REVISION
            .map(|revision| format!("\"source_revision\":\"{revision}\","))
            .unwrap_or_default();
        println!(
            "{{\"format\":1,\"identifier\":\"com.tekes.kernel.supervisor\",{}\"version\":\"{}\"}}",
            source_revision, embedded_build,
        );
        return ExitCode::SUCCESS;
    }
    eprintln!(
        "tekes-supervisor: usage: --built-in LAUNCH.json | --models-available | --describe-build"
    );
    ExitCode::from(64)
}
