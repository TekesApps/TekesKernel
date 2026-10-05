//! MCP task callbacks for the durable continuation journal. Worker activation
//! and initial asynchronous-result publication remain separate integration work.
use crate::continuation_journal::{ContinuationJournal, ContinuationJournalError};
use mcp::{McpBrokerHandle, McpCancellationToken, McpPoolKey, McpTask};
use schema::IJsonValue;
use serde_json::json;
use worker_control::continuation::{
    ContinuationOperation, ToolContinuationOutcome, ToolContinuationRequest,
    ToolContinuationResponse,
};
use worker_control::{ToolControlError, ToolControlErrorCode};

type Result<T> = std::result::Result<T, ContinuationJournalError>;
fn error(message: impl Into<String>) -> ContinuationJournalError {
    ContinuationJournalError::Conflict(message.into())
}
fn value(value: serde_json::Value) -> Result<IJsonValue> {
    IJsonValue::parse_str(&value.to_string()).map_err(|e| error(e.to_string()))
}

pub fn task_authority(key: &McpPoolKey) -> Result<IJsonValue> {
    value(
        json!({"format":1,"workspace":key.workspace,"scope":key.scope,"server":key.server,"configDigest":key.config_digest,"authorizationIdentity":key.authorization_identity,"protocolMode":key.protocol_mode,"pluginGeneration":key.plugin_generation}),
    )
}

pub fn resolve_task(
    journal: &ContinuationJournal,
    broker: &McpBrokerHandle,
    key: &McpPoolKey,
    request: &ToolContinuationRequest,
    cancellation: McpCancellationToken,
) -> Result<ToolContinuationResponse> {
    let binding = journal.binding(request)?;
    if binding.authority != task_authority(key)? {
        return Err(error("MCP continuation authority changed"));
    }
    let initial = serde_json::to_value(&binding.initial_state)?;
    let task_id = initial["taskId"]
        .as_str()
        .filter(|id| !id.is_empty())
        .ok_or_else(|| error("missing bound MCP task identity"))?;
    let initial_task = McpTask::validate_result(&binding.initial_state, task_id)
        .map_err(|e| error(e.to_string()))?;
    if !matches!(initial_task.status.as_str(), "working" | "input_required") {
        return Err(error("initial MCP continuation state is already terminal"));
    }
    let query = || -> Result<IJsonValue> {
        broker
            .task_operation_cancellable(
                key.clone(),
                "tasks/get",
                value(json!({"taskId":task_id}))?,
                cancellation.clone(),
            )
            .map_err(|e| error(e.to_string()))
    };
    journal.resolve(
        request,
        || {
            match &request.action {
                ContinuationOperation::Query => {}
                ContinuationOperation::Update { input_responses } => {
                    broker
                        .task_operation_cancellable(
                            key.clone(),
                            "tasks/update",
                            value(json!({"taskId":task_id,"inputResponses":input_responses}))?,
                            cancellation.clone(),
                        )
                        .map_err(|e| error(e.to_string()))?;
                }
                ContinuationOperation::Cancel => {
                    broker
                        .task_operation_cancellable(
                            key.clone(),
                            "tasks/cancel",
                            value(json!({"taskId":task_id}))?,
                            cancellation.clone(),
                        )
                        .map_err(|e| error(e.to_string()))?;
                }
            }
            task_response(request, task_id, query()?, false)
        },
        || {
            task_response(
                request,
                task_id,
                query()?,
                !matches!(request.action, ContinuationOperation::Query),
            )
        },
    )
}

fn task_response(
    request: &ToolContinuationRequest,
    expected_id: &str,
    state: IJsonValue,
    require_terminal: bool,
) -> Result<ToolContinuationResponse> {
    let task = McpTask::validate_result(&state, expected_id).map_err(|e| error(e.to_string()))?;
    let raw = serde_json::to_value(&state)?;
    let result = match task.status.as_str() {
        "working" | "input_required" if require_terminal => {
            return Err(error(
                "remote task still pending; prior mutation remains unconfirmed",
            ));
        }
        "working" | "input_required" => ToolContinuationOutcome::Pending {
            continuation_id: request.continuation_id.clone(),
            next_step: request.step + 1,
            state,
        },
        "completed" if raw["result"]["isError"] == true => ToolContinuationOutcome::Failed {
            error: ToolControlError {
                code: ToolControlErrorCode::Unavailable,
                message: format!("MCP task completed with tool error: {}", raw["result"]),
                retryable: false,
            },
        },
        "completed" => ToolContinuationOutcome::Completed {
            value: value(raw["result"].clone())?,
        },
        "failed" | "cancelled" => ToolContinuationOutcome::Failed {
            error: ToolControlError {
                code: ToolControlErrorCode::Unavailable,
                message: format!(
                    "MCP task {}: {}",
                    task.status,
                    raw.get("error")
                        .map(ToString::to_string)
                        .unwrap_or_default()
                ),
                retryable: false,
            },
        },
        _ => return Err(error("unsupported MCP task status")),
    };
    let response = ToolContinuationResponse {
        request_id: request.request_id.clone(),
        call_id: request.original.call_id.clone(),
        result,
    };
    response.validate_for(request).map_err(error)?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::continuation_journal::ContinuationBinding;
    use std::collections::BTreeMap;
    use worker_control::ToolControl;
    #[test]
    fn completed_remote_task_with_tool_error_is_a_failed_receipt() {
        let temp = tempfile::tempdir().unwrap();
        let original = ToolControl::new(
            "018f0000-0000-7000-8000-000000000003",
            "018f0000-0000-7000-8000-000000000003",
            1,
            "call-error",
            "mcp__tasks__run",
            value(json!({})).unwrap(),
        )
        .unwrap();
        let request = ToolContinuationRequest::new(
            original.clone(),
            "d".repeat(64),
            1,
            ContinuationOperation::Query,
        )
        .unwrap();
        let state = json!({"taskId":"remote-error", "status":"completed", "createdAt":"2026-09-04T00:00:00Z",
            "lastUpdatedAt":"2026-09-04T00:00:01Z", "result":{"content":[{"type":"text","text":"cannot write output"}],"isError":true}});
        let journal = ContinuationJournal::open(temp.path()).unwrap();
        journal
            .bind(&ContinuationBinding {
                original,
                continuation_id: request.continuation_id.clone(),
                authority: value(json!({})).unwrap(),
                initial_state: value(json!({})).unwrap(),
            })
            .unwrap();
        let response = journal
            .resolve(
                &request,
                || task_response(&request, "remote-error", value(state.clone())?, false),
                || panic!("first query must execute"),
            )
            .unwrap();
        assert!(
            matches!(&response.result, ToolContinuationOutcome::Failed { error }
            if !error.retryable && error.message.contains("cannot write output"))
        );
        drop(journal);
        let reopened = ContinuationJournal::open(temp.path()).unwrap();
        assert_eq!(
            reopened
                .resolve(
                    &request,
                    || panic!("must not execute again"),
                    || panic!("must not reconcile receipt")
                )
                .unwrap(),
            response
        );
        for flag in [json!("true"), json!(null), json!(1)] {
            let mut malformed = state.clone();
            malformed["result"]["isError"] = flag;
            assert!(
                task_response(&request, "remote-error", value(malformed).unwrap(), false).is_err()
            );
        }
        for flag in [Some(json!(false)), None] {
            let mut success = state.clone();
            if let Some(flag) = flag {
                success["result"]["isError"] = flag;
            } else {
                success["result"].as_object_mut().unwrap().remove("isError");
            }
            assert!(matches!(
                task_response(&request, "remote-error", value(success).unwrap(), false)
                    .unwrap()
                    .result,
                ToolContinuationOutcome::Completed { .. }
            ));
        }
    }

    #[test]
    fn journal_drives_bound_task_query_update_and_terminal_replay() {
        let temp = tempfile::tempdir().unwrap();
        let log = temp.path().join("calls.jsonl");
        let script = r#"
import sys,json
count=0
for line in sys.stdin:
 r=json.loads(line)
 if 'id' not in r: continue
 method=r['method']
 with open(sys.argv[1],'a') as f: f.write(json.dumps(r)+'\n')
 if method=='initialize': result={'protocolVersion':'2025-11-25','capabilities':{'tasks':{}},'serverInfo':{'name':'tasks','version':'1'}}
 elif method=='tasks/update':
  assert r['params']['taskId']=='remote-1' and r['params']['inputResponses']=={'answer':'yes'}
  result={}
 elif method=='tasks/get':
  assert r['params']['taskId']=='remote-1'
  count+=1
  result={'taskId':'remote-1','status':'working' if count==1 else 'completed','createdAt':'2026-09-04T00:00:00Z','lastUpdatedAt':'2026-09-04T00:00:01Z'}
  if count>1: result['result']={'content':[{'type':'text','text':'task:public'}],'isError':False}
 else: raise AssertionError(method)
 print(json.dumps({'jsonrpc':'2.0','id':r['id'],'result':result}),flush=True)
"#;
        let transport = mcp::StdioTransport::spawn(
            &[
                "/usr/bin/python3".into(),
                "-u".into(),
                "-c".into(),
                script.into(),
                log.to_string_lossy().into_owned(),
            ],
            None,
            &BTreeMap::new(),
        )
        .unwrap();
        let mut client = mcp::McpClient::new("tasks", transport);
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime
            .block_on(client.connect(mcp::ProtocolMode::Legacy))
            .unwrap();
        let broker = mcp::McpBroker::start().unwrap();
        let handle = broker.handle();
        let key = McpPoolKey {
            workspace: "ws".into(),
            scope: "user".into(),
            server: "tasks".into(),
            config_digest: "1".into(),
            authorization_identity: "anonymous".into(),
            protocol_mode: "legacy".into(),
            plugin_generation: None,
        };
        handle
            .register(key.clone(), Box::new(client), false)
            .unwrap();
        let original = ToolControl::new(
            "018f0000-0000-7000-8000-000000000003",
            "018f0000-0000-7000-8000-000000000003",
            1,
            "call-1",
            "mcp__tasks__run",
            value(json!({})).unwrap(),
        )
        .unwrap();
        let binding=ContinuationBinding {original:original.clone(),continuation_id:"c".repeat(64),authority:task_authority(&key).unwrap(),initial_state:value(json!({"taskId":"remote-1","status":"working","createdAt":"2026-09-04T00:00:00Z","lastUpdatedAt":"2026-09-04T00:00:00Z"})).unwrap()};
        let journal = ContinuationJournal::open(temp.path().join("journal")).unwrap();
        journal.bind(&binding).unwrap();
        let first = ToolContinuationRequest::new(
            original.clone(),
            binding.continuation_id.clone(),
            1,
            ContinuationOperation::Query,
        )
        .unwrap();
        assert!(matches!(
            resolve_task(
                &journal,
                &handle,
                &key,
                &first,
                McpCancellationToken::default()
            )
            .unwrap()
            .result,
            ToolContinuationOutcome::Pending { next_step: 2, .. }
        ));
        let second = ToolContinuationRequest::new(
            original,
            binding.continuation_id,
            2,
            ContinuationOperation::Update {
                input_responses: value(json!({"answer":"yes"})).unwrap(),
            },
        )
        .unwrap();
        let final_response = resolve_task(
            &journal,
            &handle,
            &key,
            &second,
            McpCancellationToken::default(),
        )
        .unwrap();
        assert!(
            matches!(&final_response.result,ToolContinuationOutcome::Completed{value} if serde_json::to_value(value).unwrap()["content"][0]["text"]=="task:public")
        );
        let before = std::fs::read(&log).unwrap();
        drop(journal);
        let journal = ContinuationJournal::open(temp.path().join("journal")).unwrap();
        assert_eq!(
            resolve_task(
                &journal,
                &handle,
                &key,
                &second,
                McpCancellationToken::default()
            )
            .unwrap(),
            final_response
        );
        assert_eq!(std::fs::read(&log).unwrap(), before);
        let mut wrong = key.clone();
        wrong.authorization_identity = "rotated".into();
        assert!(
            resolve_task(
                &journal,
                &handle,
                &wrong,
                &second,
                McpCancellationToken::default()
            )
            .is_err()
        );
        assert_eq!(std::fs::read(&log).unwrap(), before);
        handle.remove(key).unwrap();
    }
}
