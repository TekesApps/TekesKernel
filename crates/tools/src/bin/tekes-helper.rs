//! `tekes-helper`: the sole sandboxed filesystem, exec, and job-runner helper.

use std::path::PathBuf;

use tools::{HelperServer, RootBinding};

fn main() {
    match run() {
        Ok(()) => {}
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(tools::EX_PROTOCOL);
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    if arguments.first().is_some_and(|flag| flag == "--job-runner") {
        let request = arguments
            .get(1)
            .ok_or("--job-runner requires an absolute request path")?;
        if arguments.len() != 2 {
            return Err("--job-runner accepts exactly one request path".into());
        }
        tools::run_job_runner(PathBuf::from(request))?;
        return Ok(());
    }
    let mut arguments = arguments.into_iter();
    let mut roots = Vec::new();
    while let Some(flag) = arguments.next() {
        if flag != "--root" {
            return Err(format!("unknown argument {flag:?}").into());
        }
        let binding = arguments
            .next()
            .ok_or("--root requires name=absolute-path")?;
        let binding = binding.to_string_lossy();
        let (name, path) = binding
            .split_once('=')
            .ok_or("--root requires name=absolute-path")?;
        roots.push(RootBinding::open(name, PathBuf::from(path))?);
    }
    let server = HelperServer::new(roots)?;
    tools::serve_stdio(&server)?;
    Ok(())
}
