//! Folder authorization has no RPC grant route. Local approval writes the
//! sandbox-protected authority store; a terminal is consent, not authentication.
use crate::endpoint_host::{ProductionEndpointRoutes, ProductionRouteFailure};
use endpoint::{EndpointHostCall, MethodClass};
use profile::FolderAuthorityStore;
use schema::IJsonValue;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::BTreeSet,
    io::{self, BufRead, Write},
    path::PathBuf,
};

pub struct FolderTrustRoutes {
    root: PathBuf,
}
impl FolderTrustRoutes {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Status {
    path: PathBuf,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Request {
    path: PathBuf,
    expected_revision: u64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Revoke {
    folder_id: String,
    expected_revision: u64,
}
fn failure(error: impl std::fmt::Display) -> ProductionRouteFailure {
    ProductionRouteFailure {
        code: "folder-trust-refused".into(),
        message: error.to_string(),
        details: IJsonValue::parse(b"{}").expect("empty object"),
    }
}
impl ProductionEndpointRoutes for FolderTrustRoutes {
    fn capabilities(&self) -> BTreeSet<String> {
        [
            "folders.trust.status",
            "folders.trust.request",
            "folders.trust.revoke",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    }
    fn extension_method_class(&self, method: &str) -> Option<MethodClass> {
        match method {
            "folders.trust.status" => Some(MethodClass::ReadOnly),
            "folders.trust.request" | "folders.trust.revoke" => Some(MethodClass::Mutation),
            _ => None,
        }
    }
    fn validate_extension_payload(
        &self,
        method: &str,
        payload: &Value,
    ) -> Result<(), ProductionRouteFailure> {
        match method {
            "folders.trust.status" => serde_json::from_value::<Status>(payload.clone())
                .map(|_| ())
                .map_err(failure),
            "folders.trust.request" => serde_json::from_value::<Request>(payload.clone())
                .map(|_| ())
                .map_err(failure),
            "folders.trust.revoke" => serde_json::from_value::<Revoke>(payload.clone())
                .map(|_| ())
                .map_err(failure),
            _ => Err(failure("unsupported-folder-trust-operation")),
        }
    }
    fn extension_failure_is_exact(
        &self,
        _operation: &str,
        failure: &ProductionRouteFailure,
    ) -> bool {
        failure.code == "folder-trust-refused"
    }
    fn execute(
        &self,
        call: &EndpointHostCall,
        payload: &Value,
        _principal: &str,
    ) -> Result<IJsonValue, ProductionRouteFailure> {
        let store = FolderAuthorityStore::new(&self.root);
        let value = match call.operation.as_str() {
            "folders.trust.status" => {
                let input: Status = serde_json::from_value(payload.clone()).map_err(failure)?;
                json!({"revision":store.registry_revision().map_err(failure)?, "folder":store.status(&input.path).map_err(failure)?})
            }
            "folders.trust.request" => {
                let input: Request = serde_json::from_value(payload.clone()).map_err(failure)?;
                json!({"requestId":call.rpc_id, "receipt":store.request(&call.rpc_id,input.expected_revision,&input.path).map_err(failure)?})
            }
            "folders.trust.revoke" => {
                let input: Revoke = serde_json::from_value(payload.clone()).map_err(failure)?;
                let receipt = store
                    .revoke(&call.rpc_id, input.expected_revision, &input.folder_id)
                    .map_err(failure)?;
                json!({"receipt":receipt,"effectsDrained":store.effects_drained(&input.folder_id).map_err(failure)?,"poolCleanup":"pending"})
            }
            _ => return Err(failure("unsupported-folder-trust-operation")),
        };
        if call.operation != "folders.trust.status" {
            call.handoff
                .mark_handed_off(endpoint::DurableHandoffProof {
                    delivery: "folder-authority-barrier".into(),
                    durable_identity: Some(endpoint::RpcDurableIdentity {
                        kind: "folder-trust".into(),
                        id: call.rpc_id.clone(),
                        seq: None,
                    }),
                })
                .map_err(failure)?;
        }
        IJsonValue::parse(&serde_json::to_vec(&value).map_err(failure)?).map_err(failure)
    }
}

/// `--folder-trust status ROOT PATH` / `request ROOT PATH OP REV` /
/// `grant ROOT REQUEST OP REV` / `revoke ROOT FOLDER OP REV`.
pub fn cli(args: &[std::ffi::OsString]) -> io::Result<()> {
    let strings = args
        .iter()
        .map(|arg| {
            arg.to_str()
                .ok_or_else(|| io::Error::other("non-utf8-argument"))
        })
        .collect::<io::Result<Vec<_>>>()?;
    if strings.len() < 5 {
        return Err(io::Error::other("folder-trust-usage"));
    }
    let store = FolderAuthorityStore::new(PathBuf::from(strings[3]));
    let value = match strings[2] {
        "status" if strings.len() == 5 => {
            json!({"revision":store.registry_revision()?,"folder":store.status(std::path::Path::new(strings[4]))?})
        }
        command if strings.len() == 7 => {
            let expected = strings[6].parse::<u64>().map_err(io::Error::other)?;
            let receipt = match command {
                "request" => {
                    store.request(strings[5], expected, std::path::Path::new(strings[4]))?
                }
                "revoke" => store.revoke(strings[5], expected, strings[4])?,
                "grant" => {
                    if let Some(receipt) = store.prior_grant(strings[5], expected, strings[4])? {
                        println!(
                            "{}",
                            serde_json::to_string(&json!({"receipt":receipt}))
                                .map_err(io::Error::other)?
                        );
                        return Ok(());
                    }
                    let folder = store.pending(strings[4])?;
                    let mut terminal = std::fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .open("/dev/tty")?;
                    writeln!(
                        terminal,
                        "Approve folder trust: {}",
                        serde_json::to_string(&folder).map_err(io::Error::other)?
                    )?;
                    writeln!(
                        terminal,
                        "Registry revision {expected}. Type approve {} to continue:",
                        serde_json::to_string(strings[4]).map_err(io::Error::other)?
                    )?;
                    terminal.flush()?;
                    let mut answer = String::new();
                    io::BufReader::new(terminal).read_line(&mut answer)?;
                    if answer.trim_end()
                        != format!(
                            "approve {}",
                            serde_json::to_string(strings[4]).map_err(io::Error::other)?
                        )
                    {
                        return Err(io::Error::other("folder-consent-refused"));
                    }
                    store.grant_from_host(strings[5], expected, strings[4])?
                }
                _ => return Err(io::Error::other("folder-trust-usage")),
            };
            json!({"receipt":receipt})
        }
        _ => return Err(io::Error::other("folder-trust-usage")),
    };
    println!(
        "{}",
        serde_json::to_string(&value).map_err(io::Error::other)?
    );
    Ok(())
}
