use std::path::PathBuf;

use serde::Serialize;

use crate::error::SelectorError;
use crate::fs::{canonical_line, validate_absolute_lexical, validate_id};
use crate::selector::{Selector, cli_command_sha256};
use crate::signature::CodeSignatureVerifier;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Stage {
        bundle: PathBuf,
        version: String,
    },
    Activate {
        version: String,
    },
    Rollback {
        reason: String,
    },
    Status,
    Recover,
    Prune,
    AttestCanary {
        version: String,
        session: String,
        run: String,
        storage_root: PathBuf,
    },
    AttestInstallHealth {
        version: String,
    },
    Serve {
        storage_root: PathBuf,
        listen: String,
        web_listen: Option<String>,
    },
    UpdateSelector {
        artifact: PathBuf,
        manifest: PathBuf,
    },
}

pub fn parse_args(args: &[String]) -> Result<(PathBuf, Command), SelectorError> {
    if args.len() < 3 || args[0] != "--install-root" {
        return Err(SelectorError::usage(
            args.first().cloned().unwrap_or_default(),
        ));
    }
    let install_root = PathBuf::from(&args[1]);
    validate_absolute_lexical(&install_root)?;
    let command = match (args[2].as_str(), &args[3..]) {
        ("stage", [bundle_option, bundle, version_option, version])
            if bundle_option == "--bundle" && version_option == "--version" =>
        {
            let bundle = PathBuf::from(bundle);
            validate_absolute_lexical(&bundle)?;
            validate_id(version)?;
            Command::Stage {
                bundle,
                version: version.clone(),
            }
        }
        ("activate", [option, version]) if option == "--version" => {
            validate_id(version)?;
            Command::Activate {
                version: version.clone(),
            }
        }
        ("rollback", [option, reason]) if option == "--reason" => {
            validate_id(reason)?;
            Command::Rollback {
                reason: reason.clone(),
            }
        }
        ("status", []) => Command::Status,
        ("recover", []) => Command::Recover,
        ("prune", []) => Command::Prune,
        ("attest-canary", [v, version, s, session, r, run, root, storage])
            if v == "--version" && s == "--session" && r == "--run" && root == "--storage-root" =>
        {
            let storage_root = PathBuf::from(storage);
            validate_absolute_lexical(&storage_root)?;
            validate_id(version)?;
            validate_id(run)?;
            Command::AttestCanary {
                version: version.clone(),
                session: session.clone(),
                run: run.clone(),
                storage_root,
            }
        }
        ("attest-install-health", [option, version]) if option == "--version" => {
            validate_id(version)?;
            Command::AttestInstallHealth {
                version: version.clone(),
            }
        }
        ("serve", [root, storage, listen_option, listen])
            if root == "--storage-root"
                && listen_option == "--listen"
                && listen == "127.0.0.1:7347" =>
        {
            let storage_root = PathBuf::from(storage);
            validate_absolute_lexical(&storage_root)?;
            Command::Serve {
                storage_root,
                listen: listen.clone(),
                web_listen: None,
            }
        }
        ("serve", [root, storage, listen_option, listen, web_option, web_listen])
            if root == "--storage-root"
                && listen_option == "--listen"
                && listen == "127.0.0.1:7347"
                && web_option == "--web-listen"
                && web_listen == "127.0.0.1:7357" =>
        {
            let storage_root = PathBuf::from(storage);
            validate_absolute_lexical(&storage_root)?;
            Command::Serve {
                storage_root,
                listen: listen.clone(),
                web_listen: Some(web_listen.clone()),
            }
        }
        ("update-selector", [artifact_option, artifact, manifest_option, manifest])
            if artifact_option == "--artifact" && manifest_option == "--manifest" =>
        {
            let artifact = PathBuf::from(artifact);
            let manifest = PathBuf::from(manifest);
            validate_absolute_lexical(&artifact)?;
            validate_absolute_lexical(&manifest)?;
            Command::UpdateSelector { artifact, manifest }
        }
        _ => {
            return Err(SelectorError::usage(
                args.get(2).cloned().unwrap_or_default(),
            ));
        }
    };
    Ok((install_root, command))
}

pub fn run_command<V: CodeSignatureVerifier>(
    selector: &Selector<V>,
    command: &Command,
    exact_argv: &[String],
) -> Result<Option<Vec<u8>>, SelectorError> {
    let command_hash = cli_command_sha256(exact_argv)?;
    match command {
        Command::Stage { bundle, version } => {
            reply(selector.stage(bundle, version, command_hash)?).map(Some)
        }
        Command::Activate { version } => reply(selector.activate(version, command_hash)?).map(Some),
        Command::Rollback { reason } => reply(selector.rollback(reason, command_hash)?).map(Some),
        Command::Status => reply(selector.status()?).map(Some),
        Command::Recover => reply(selector.recover()?).map(Some),
        Command::Prune => reply(selector.prune()?).map(Some),
        Command::AttestCanary {
            version,
            session,
            run,
            storage_root,
        } => {
            let accepted = rfc3339_now()?;
            let window = rfc3339_after(300)?;
            reply(selector.attest_canary(
                version,
                session,
                run,
                storage_root,
                &accepted,
                &window,
            )?)
            .map(Some)
        }
        Command::AttestInstallHealth { version } => {
            let accepted = rfc3339_now()?;
            let window = rfc3339_after(300)?;
            reply(selector.attest_install_health(version, "127.0.0.1:7347", &accepted, &window)?)
                .map(Some)
        }
        Command::UpdateSelector { artifact, manifest } => {
            reply(selector.update_selector(artifact, manifest)?).map(Some)
        }
        Command::Serve {
            storage_root,
            listen,
            web_listen,
        } => {
            selector.serve_with_web(storage_root, listen, web_listen.as_deref())?;
            Ok(None)
        }
    }
}

fn reply<T: Serialize>(value: T) -> Result<Vec<u8>, SelectorError> {
    canonical_line(&value)
}

pub(crate) fn rfc3339_now() -> Result<String, SelectorError> {
    rfc3339_from(std::time::SystemTime::now())
}

pub(crate) fn rfc3339_after(seconds: u64) -> Result<String, SelectorError> {
    rfc3339_from(std::time::SystemTime::now() + std::time::Duration::from_secs(seconds))
}

fn rfc3339_from(time: std::time::SystemTime) -> Result<String, SelectorError> {
    use std::time::UNIX_EPOCH;
    let duration = time
        .duration_since(UNIX_EPOCH)
        .map_err(|_| SelectorError::invalid_state("clock-before-epoch"))?;
    let seconds = duration.as_secs() as libc::time_t;
    let mut broken_down = std::mem::MaybeUninit::<libc::tm>::uninit();
    // SAFETY: pointers refer to live storage and gmtime_r initializes `tm` on success.
    let result = unsafe { libc::gmtime_r(&seconds, broken_down.as_mut_ptr()) };
    if result.is_null() {
        return Err(SelectorError::invalid_state("clock-unavailable"));
    }
    // SAFETY: gmtime_r succeeded above.
    let tm = unsafe { broken_down.assume_init() };
    Ok(format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:09}Z",
        tm.tm_year + 1900,
        tm.tm_mon + 1,
        tm.tm_mday,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec,
        duration.subsec_nanos()
    ))
}

#[cfg(test)]
mod tests {
    use super::{Command, parse_args};

    #[test]
    fn parser_accepts_only_the_closed_ordered_grammar() {
        let args = [
            "--install-root",
            "/tmp/Kernel",
            "stage",
            "--bundle",
            "/tmp/Bundle",
            "--version",
            "1.0.0",
        ]
        .map(str::to_owned);
        let (_, command) = parse_args(&args).expect("closed stage grammar");
        assert!(matches!(command, Command::Stage { .. }));

        let health_admission = [
            "--install-root",
            "/tmp/Kernel",
            "attest-install-health",
            "--version",
            "1.0.0",
        ]
        .map(str::to_owned);
        assert!(matches!(
            parse_args(&health_admission)
                .expect("health admission grammar")
                .1,
            Command::AttestInstallHealth { .. }
        ));

        let reordered = [
            "--install-root",
            "/tmp/Kernel",
            "stage",
            "--version",
            "1.0.0",
            "--bundle",
            "/tmp/Bundle",
        ]
        .map(str::to_owned);
        assert!(parse_args(&reordered).is_err());

        let prune = ["--install-root", "/tmp/Kernel", "prune"].map(str::to_owned);
        assert_eq!(parse_args(&prune).expect("prune grammar").1, Command::Prune);
        let prune_with_option =
            ["--install-root", "/tmp/Kernel", "prune", "--all"].map(str::to_owned);
        assert!(parse_args(&prune_with_option).is_err());

        let duplicated = [
            "--install-root",
            "/tmp/Kernel",
            "status",
            "--install-root",
            "/tmp/Other",
        ]
        .map(str::to_owned);
        assert!(parse_args(&duplicated).is_err());
    }

    #[test]
    fn parser_rejects_noncanonical_paths_and_non_loopback_serve() {
        let repeated_separator = ["--install-root", "/tmp//Kernel", "status"].map(str::to_owned);
        assert!(parse_args(&repeated_separator).is_err());

        let non_loopback = [
            "--install-root",
            "/tmp/Kernel",
            "serve",
            "--storage-root",
            "/tmp/threads",
            "--listen",
            "0.0.0.0:7347",
        ]
        .map(str::to_owned);
        assert!(parse_args(&non_loopback).is_err());

        let web_enabled = [
            "--install-root",
            "/tmp/Kernel",
            "serve",
            "--storage-root",
            "/tmp/threads",
            "--listen",
            "127.0.0.1:7347",
            "--web-listen",
            "127.0.0.1:7357",
        ]
        .map(str::to_owned);
        assert!(matches!(
            parse_args(&web_enabled)
                .expect("explicit Web Client service")
                .1,
            Command::Serve {
                web_listen: Some(_),
                ..
            }
        ));

        let public_web = [
            "--install-root",
            "/tmp/Kernel",
            "serve",
            "--storage-root",
            "/tmp/threads",
            "--listen",
            "127.0.0.1:7347",
            "--web-listen",
            "0.0.0.0:7357",
        ]
        .map(str::to_owned);
        assert!(parse_args(&public_web).is_err());
    }
}
