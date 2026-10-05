use std::io::Write;
use std::os::unix::process::CommandExt;
use std::process::Command as ProcessCommand;

use tekes_selector::{
    Command as SelectorCommand, MacOsCodeSignatureVerifier, Selector, describe_conformance,
    parse_args, reply_bytes, run_command,
};

fn main() {
    let argv = std::env::args().skip(1).collect::<Vec<_>>();
    if argv == ["describe-conformance"] {
        match describe_conformance().and_then(|reply| reply_bytes(&reply)) {
            Ok(bytes) => {
                if std::io::stdout().write_all(&bytes).is_err() {
                    std::process::exit(74);
                }
                return;
            }
            Err(error) => {
                let _ = std::io::stderr().write_all(
                    &error.envelope_bytes().unwrap_or_else(|_| {
                        b"{\"error\":{\"code\":\"io\",\"details\":{\"operation\":\"error-format\"},\"message\":\"Selector I/O failed\"}}\n".to_vec()
                    }),
                );
                std::process::exit(error.code.exit_code());
            }
        }
    }
    let parsed = parse_args(&argv);
    if matches!(&parsed, Ok((_, SelectorCommand::Serve { .. }))) {
        sanitize_service_environment(&argv);
    }
    let result = parsed.and_then(|(install_root, command)| {
        let selector = Selector::new(install_root, MacOsCodeSignatureVerifier);
        run_command(&selector, &command, &argv)
    });
    match result {
        Ok(Some(bytes)) => {
            if std::io::stdout().write_all(&bytes).is_err() {
                std::process::exit(74);
            }
        }
        Ok(None) => {}
        Err(error) => {
            let bytes = error.envelope_bytes().unwrap_or_else(|_| {
                b"{\"error\":{\"code\":\"io\",\"details\":{\"operation\":\"error-format\"},\"message\":\"Selector I/O failed\"}}\n".to_vec()
            });
            let _ = std::io::stderr().write_all(&bytes);
            std::process::exit(error.code.exit_code());
        }
    }
}

fn sanitize_service_environment(argv: &[String]) {
    if std::env::vars_os().next().is_none() {
        return;
    }

    let executable = match std::env::current_exe() {
        Ok(executable) => executable,
        Err(_) => {
            eprintln!("tekes-selector: cannot resolve executable for environment isolation");
            std::process::exit(74);
        }
    };
    let error = ProcessCommand::new(executable)
        .args(argv)
        .env_clear()
        .exec();
    eprintln!("tekes-selector: cannot isolate inherited environment: {error}");
    std::process::exit(74);
}
