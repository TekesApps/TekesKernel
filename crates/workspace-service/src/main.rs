use serde::Deserialize;
use serde_json::{Value, json};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    method: String,
    request: Value,
}

fn main() {
    // This executable is an internal child protocol, not a public root-authority API.
    let arguments = std::env::args_os().skip(1).collect::<Vec<_>>();
    if ![2, 4].contains(&arguments.len())
        || arguments[0] != "--root"
        || (arguments.len() == 4 && arguments[2] != "--authority-file")
    {
        eprintln!("usage: tekes-workspace-service --root REGISTERED_ROOT");
        std::process::exit(64);
    }
    let root = PathBuf::from(&arguments[1]);
    let mut input = Vec::new();
    let result: Result<Value, String> = (|| {
        std::io::stdin()
            .take(8 * 1024 * 1024 + 1)
            .read_to_end(&mut input)
            .map_err(|e| e.to_string())?;
        if input.len() > 8 * 1024 * 1024 {
            return Err("Request too large".into());
        }
        let envelope: Request = serde_json::from_slice(&input).map_err(|e| e.to_string())?;
        let authority: Option<workspace_service::git::MutationAuthority> = if arguments.len() == 4 {
            Some(
                serde_json::from_slice(&std::fs::read(&arguments[3]).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?,
            )
        } else {
            None
        };
        if envelope.method != "filesWrite" && input.len() > 1024 * 1024 {
            return Err("Request too large".into());
        }
        let result = if envelope.method == "filesWrite" {
            let request = serde_json::from_value(envelope.request).map_err(|e| e.to_string())?;
            workspace_service::write::execute(&root, request, authority.as_ref())
        } else if envelope.method.starts_with("git") {
            let request = serde_json::from_value(envelope.request).map_err(|e| e.to_string())?;
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| e.to_string())?
                .block_on(async {
                    if ["gitCommit", "gitPush", "gitChangeBranch"]
                        .contains(&envelope.method.as_str())
                    {
                        workspace_service::git::mutate(
                            &root,
                            &envelope.method,
                            request,
                            authority.as_ref(),
                        )
                        .await
                    } else {
                        workspace_service::git::query(&root, &envelope.method, request).await
                    }
                })
        } else if [
            "turnChanges",
            "recordTurnEdit",
            "prepareTurnEdit",
            "abortTurnEdit",
        ]
        .contains(&envelope.method.as_str())
        {
            let request = serde_json::from_value(envelope.request).map_err(|e| e.to_string())?;
            match authority.as_ref() {
                Some(authority) => workspace_service::turn::execute_with_roots(
                    &authority.state_root,
                    &root,
                    &authority.workspace_roots,
                    &envelope.method,
                    request,
                ),
                None => Err(workspace_service::Failure {
                    code: "unavailable",
                    message: "Missing parent service state authority".into(),
                }),
            }
        } else {
            let request = serde_json::from_value(envelope.request).map_err(|e| e.to_string())?;
            workspace_service::files(&root, &envelope.method, request)
        };
        Ok(match result {
            Ok(value) => json!({"result":value}),
            Err(error) => json!({"error":error}),
        })
    })();
    let value = result
        .unwrap_or_else(|message| json!({"error":{"code":"invalid-request","message":message}}));
    let output = serde_json::to_vec(&value).expect("JSON response");
    if std::io::stdout().write_all(&output).is_err() {
        std::process::exit(74);
    }
}
