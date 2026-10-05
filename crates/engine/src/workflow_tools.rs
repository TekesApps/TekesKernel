use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

use schema::IJsonValue;
use serde_json::{Value, json};
use store::{NamedLock, SyncPolicy, SystemSync};
use tools::{BackendTerminal, DurableApprovalResponse, ToolExecution};

use crate::ToolBackend;

struct LockedWorkflowLog {
    _lock: NamedLock,
    file: File,
    records: Vec<Value>,
    parent: PathBuf,
    path_was_new: bool,
}

impl LockedWorkflowLog {
    fn append(&mut self, value: &Value) -> Result<(), String> {
        let mut bytes =
            serde_json_canonicalizer::to_vec(value).map_err(|error| error.to_string())?;
        bytes.push(b'\n');
        self.file
            .write_all(&bytes)
            .map_err(|error| error.to_string())?;
        SystemSync
            .full_sync(&self.file)
            .map_err(|error| error.to_string())?;
        if self.path_was_new {
            SystemSync
                .full_sync(&File::open(&self.parent).map_err(|error| error.to_string())?)
                .map_err(|error| error.to_string())?;
            self.path_was_new = false;
        }
        self.records.push(value.clone());
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CatalogEntry {
    pub name: String,
    pub summary: String,
    pub aliases: Vec<String>,
    pub schema_digest: String,
    pub content: IJsonValue,
}

#[derive(Clone, Debug)]
pub struct WorkflowBackend {
    root: PathBuf,
    goal_id: Option<String>,
    skills: Vec<CatalogEntry>,
    deferred_tools: Vec<CatalogEntry>,
}

impl WorkflowBackend {
    pub fn new(
        root: impl Into<PathBuf>,
        workspace: impl Into<String>,
        goal_id: Option<String>,
        mut skills: Vec<CatalogEntry>,
        mut deferred_tools: Vec<CatalogEntry>,
    ) -> Result<Self, String> {
        let root = root.into();
        let workspace = workspace.into();
        if workspace.is_empty() {
            return Err("workflow workspace identity is empty".to_owned());
        }
        fs::create_dir_all(&root).map_err(|error| error.to_string())?;
        if fs::symlink_metadata(&root)
            .map_err(|error| error.to_string())?
            .file_type()
            .is_symlink()
        {
            return Err("workflow state root is a symlink".to_owned());
        }
        sort_catalog(&mut skills)?;
        sort_catalog(&mut deferred_tools)?;
        Ok(Self {
            root,
            goal_id,
            skills,
            deferred_tools,
        })
    }

    fn execute_inner(
        &mut self,
        execution: &ToolExecution,
        invocation: &IJsonValue,
    ) -> Result<BackendTerminal, String> {
        let arguments = serde_json::to_value(invocation).map_err(|error| error.to_string())?;
        match execution.name.as_str() {
            "ask_user_questions" => Ok(BackendTerminal::Hold {
                scope: "answer".to_owned(),
                question: invocation.clone(),
            }),
            "plan" => Ok(BackendTerminal::Hold {
                scope: "plan".to_owned(),
                question: invocation.clone(),
            }),
            "think" => Ok(completed(json!({"recorded": true}))),
            "summary_artifact" => Ok(completed(json!({
                "continuation": required(&arguments, "continuation")?,
                "evidence_refs": required(&arguments, "evidence_refs")?,
            }))),
            "verify" => Ok(completed(json!({
                "verdict": required(&arguments, "verdict")?,
                "covered_set": required(&arguments, "covered_set")?,
                "failures": required(&arguments, "failures")?,
            }))),
            "new_goal" => self.new_goal(execution, &arguments),
            "set_goal_state" => self.set_goal_state(execution, &arguments),
            "skill_explorer" => self.offer(execution, &arguments, &self.skills),
            "tool_search" => self.offer(execution, &arguments, &self.deferred_tools),
            "skill" => self.load_skill(&arguments),
            _ => Ok(BackendTerminal::Unavailable {
                code: "unsupported".to_owned(),
                message: format!("{} is not a workflow backend tool", execution.name),
                retryable: false,
            }),
        }
    }

    fn new_goal(
        &self,
        execution: &ToolExecution,
        arguments: &Value,
    ) -> Result<BackendTerminal, String> {
        let goal_id = self
            .goal_id
            .as_deref()
            .ok_or("host supplied goal id is unavailable")?;
        let record = json!({
            "format": 1,
            "op": "new_goal",
            "goal_id": goal_id,
            "thread": execution.thread,
            "turn": execution.turn,
            "call": execution.call,
            "ts": execution.timestamp,
            "goal": required(arguments, "goal")?,
            "completion_criteria": required(arguments, "completion_criteria")?,
            "reason": required(arguments, "reason")?,
        });
        self.append_log("goals/log.jsonl", &record)?;
        Ok(completed(json!({"goal_id":goal_id,"state":"active"})))
    }

    fn set_goal_state(
        &self,
        execution: &ToolExecution,
        arguments: &Value,
    ) -> Result<BackendTerminal, String> {
        let goal_id = self
            .goal_id
            .as_deref()
            .ok_or("host supplied goal id is unavailable")?;
        if !self.goal_exists(goal_id, &execution.thread)? {
            return Ok(unavailable(
                "not_found",
                "host-bound goal does not exist",
                false,
            ));
        }
        let record = json!({
            "format": 1,
            "op": "set_goal_state",
            "goal_id": goal_id,
            "thread": execution.thread,
            "turn": execution.turn,
            "call": execution.call,
            "ts": execution.timestamp,
            "state": required(arguments, "state")?,
            "progress": required(arguments, "progress")?,
            "reason": arguments.get("reason").cloned().unwrap_or(Value::Null),
            "user_action": arguments.get("user_action").cloned().unwrap_or(Value::Null),
        });
        self.append_log("goals/log.jsonl", &record)?;
        Ok(completed(
            json!({"goal_id":goal_id,"state":arguments["state"]}),
        ))
    }

    fn offer(
        &self,
        execution: &ToolExecution,
        arguments: &Value,
        catalog: &[CatalogEntry],
    ) -> Result<BackendTerminal, String> {
        let query = required_str(arguments, "query")?;
        let limit = arguments.get("limit").and_then(Value::as_u64).unwrap_or(5) as usize;
        let query_terms = terms(query);
        let mut matches = catalog
            .iter()
            .filter_map(|entry| {
                let mut haystack = format!("{} {}", entry.name, entry.summary);
                for alias in &entry.aliases {
                    haystack.push(' ');
                    haystack.push_str(alias);
                }
                let score = query_terms
                    .iter()
                    .filter(|term| haystack.to_ascii_lowercase().contains(term.as_str()))
                    .count();
                (score > 0).then_some((score, entry))
            })
            .collect::<Vec<_>>();
        matches.sort_by(|(left_score, left), (right_score, right)| {
            right_score
                .cmp(left_score)
                .then_with(|| left.name.as_bytes().cmp(right.name.as_bytes()))
        });
        matches.truncate(limit);
        Ok(completed(json!({
            "offer":execution.call,
            "items":matches.into_iter().map(|(_, entry)| json!({
                "name":entry.name,"summary":entry.summary,"aliases":entry.aliases,
                "schema_digest":entry.schema_digest
            })).collect::<Vec<_>>()
        })))
    }

    fn load_skill(&self, arguments: &Value) -> Result<BackendTerminal, String> {
        let name = required_str(arguments, "skill")?;
        let entry = self
            .skills
            .iter()
            .find(|entry| entry.name == name)
            .ok_or("skill catalog entry disappeared")?;
        Ok(BackendTerminal::Completed(entry.content.clone()))
    }

    /// A goal exists for `set_goal_state` when the model authored it with
    /// `new_goal` in this log, or when the host bound it from the session's
    /// durable `goals.v1` record (`goals/sessions/<thread>.json`), which is
    /// how a goal created through the endpoint reaches the worker.
    fn goal_exists(&self, goal_id: &str, thread: &str) -> Result<bool, String> {
        if self.read_log("goals/log.jsonl")?.iter().any(|record| {
            record.get("goal_id").and_then(Value::as_str) == Some(goal_id)
                && record.get("op").and_then(Value::as_str) == Some("new_goal")
        }) {
            return Ok(true);
        }
        let record_path = self
            .root
            .join("goals")
            .join("sessions")
            .join(format!("{thread}.json"));
        match fs::read(&record_path) {
            Ok(bytes) => {
                let record: Value = serde_json::from_slice(&bytes)
                    .map_err(|error| format!("session goal record unreadable: {error}"))?;
                Ok(record.get("id").and_then(Value::as_str) == Some(goal_id))
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error.to_string()),
        }
    }

    fn append_log(&self, name: &str, value: &Value) -> Result<(), String> {
        let mut log = self.open_log_exclusive(name)?;
        if let Some(call) = value.get("call").and_then(Value::as_str) {
            let thread = value.get("thread").and_then(Value::as_str);
            for record in &log.records {
                if record.get("call").and_then(Value::as_str) == Some(call)
                    && record.get("thread").and_then(Value::as_str) == thread
                {
                    return if *record == *value {
                        Ok(())
                    } else {
                        Err(format!(
                            "workflow call {call} has conflicting durable bytes"
                        ))
                    };
                }
            }
        }
        log.append(value)
    }

    fn read_log(&self, name: &str) -> Result<Vec<Value>, String> {
        Ok(self.open_log_exclusive(name)?.records)
    }

    fn open_log_exclusive(&self, name: &str) -> Result<LockedWorkflowLog, String> {
        let path = self.root.join(name);
        let parent = path.parent().ok_or("workflow log has no parent")?;
        let parent_existed = parent.exists();
        let path_was_new = !path.exists();
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        if !parent_existed {
            SystemSync
                .full_sync(&File::open(&self.root).map_err(|error| error.to_string())?)
                .map_err(|error| error.to_string())?;
        }
        let lock =
            NamedLock::exclusive(path.with_extension("lock")).map_err(|error| error.to_string())?;
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&path)
            .map_err(|error| error.to_string())?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|error| error.to_string())?;
        let valid_end = bytes
            .iter()
            .rposition(|byte| *byte == b'\n')
            .map_or(0, |index| index + 1);
        if valid_end != bytes.len() {
            file.set_len(valid_end as u64)
                .map_err(|error| error.to_string())?;
            SystemSync
                .full_sync(&file)
                .map_err(|error| error.to_string())?;
            bytes.truncate(valid_end);
        }
        let mut records = Vec::new();
        for line in bytes
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
        {
            let value: Value = serde_json::from_slice(line).map_err(|error| error.to_string())?;
            if serde_json_canonicalizer::to_vec(&value).map_err(|error| error.to_string())? != line
            {
                return Err("workflow authority contains non-canonical JSONL".to_owned());
            }
            records.push(value);
        }
        Ok(LockedWorkflowLog {
            _lock: lock,
            file,
            records,
            parent: parent.to_path_buf(),
            path_was_new,
        })
    }
}

impl ToolBackend for WorkflowBackend {
    fn supports(&self, name: &str) -> bool {
        match name {
            "new_goal" | "set_goal_state" => self.goal_id.is_some(),
            "skill" => !self.skills.is_empty(),
            "tool_search" => !self.deferred_tools.is_empty(),
            "ask_user_questions" | "plan" | "think" | "summary_artifact" | "verify"
            | "skill_explorer" => true,
            _ => false,
        }
    }

    fn execute(&mut self, execution: &ToolExecution, invocation: &IJsonValue) -> BackendTerminal {
        self.execute_inner(execution, invocation)
            .unwrap_or_else(|message| unavailable("internal", &message, false))
    }

    fn resume_after_approval(
        &mut self,
        execution: &ToolExecution,
        _invocation: &IJsonValue,
        approval: &DurableApprovalResponse,
    ) -> BackendTerminal {
        match execution.name.as_str() {
            "ask_user_questions" => approval.answer.clone().map_or_else(
                || unavailable("missing_answer", "approved question has no answer", false),
                BackendTerminal::Completed,
            ),
            "plan" => BackendTerminal::Completed(
                approval
                    .answer
                    .clone()
                    .unwrap_or_else(|| IJsonValue::from(true)),
            ),
            _ => self.execute(execution, _invocation),
        }
    }
}

fn sort_catalog(entries: &mut [CatalogEntry]) -> Result<(), String> {
    entries.sort_by(|left, right| left.name.as_bytes().cmp(right.name.as_bytes()));
    if entries.iter().any(|entry| entry.name.is_empty())
        || entries.windows(2).any(|pair| pair[0].name == pair[1].name)
    {
        return Err("catalog entries require unique nonempty names".to_owned());
    }
    Ok(())
}

fn required<'a>(value: &'a Value, name: &str) -> Result<&'a Value, String> {
    value.get(name).ok_or_else(|| format!("missing {name}"))
}

fn required_str<'a>(value: &'a Value, name: &str) -> Result<&'a str, String> {
    required(value, name)?
        .as_str()
        .ok_or_else(|| format!("{name} must be a string"))
}

fn completed(value: Value) -> BackendTerminal {
    BackendTerminal::Completed(to_ijson(value))
}

fn unavailable(code: &str, message: &str, retryable: bool) -> BackendTerminal {
    BackendTerminal::Unavailable {
        code: code.to_owned(),
        message: message.to_owned(),
        retryable,
    }
}

fn to_ijson(value: Value) -> IJsonValue {
    IJsonValue::parse(
        &serde_json_canonicalizer::to_vec(&value).expect("workflow result is serializable JSON"),
    )
    .expect("workflow result is I-JSON")
}

fn terms(value: &str) -> Vec<String> {
    value
        .split(|character: char| !character.is_alphanumeric())
        .filter(|term| !term.is_empty())
        .map(str::to_ascii_lowercase)
        .collect()
}

#[cfg(test)]
mod host_goal_tests {
    use super::*;

    fn execution(root_thread: &str, name: &str, arguments: Value) -> ToolExecution {
        ToolExecution {
            thread: root_thread.to_owned(),
            call: format!("call-{name}"),
            name: name.to_owned(),
            attempt: "attempt-1".to_owned(),
            invocation: IJsonValue::parse(&serde_json::to_vec(&arguments).unwrap()).unwrap(),
            side_effectful: true,
            turn: 1,
            timestamp: "2026-09-14T00:00:00.000Z".to_owned(),
        }
    }

    #[test]
    fn set_goal_state_accepts_a_host_bound_session_goal_record() {
        let root = tempfile::tempdir().unwrap();
        let thread = "018f0000-0000-7000-8000-0000000000c1";
        let sessions = root.path().join("goals").join("sessions");
        fs::create_dir_all(&sessions).unwrap();
        fs::write(
            sessions.join(format!("{thread}.json")),
            br#"{"format":1,"id":"goal-host-1","revision":1,"objective":"ship","phase":"active","maxGoalRounds":1,"roundsStarted":0,"createdAt":"2026-09-14T00:00:00.000Z","updatedAt":"2026-09-14T00:00:00.000Z"}"#,
        )
        .unwrap();
        let mut backend = WorkflowBackend::new(
            root.path(),
            "ws",
            Some("goal-host-1".to_owned()),
            vec![],
            vec![],
        )
        .unwrap();
        assert!(backend.supports("set_goal_state") && backend.supports("new_goal"));
        let arguments = json!({"state":"active","progress":"10","reason":"checking"});
        let terminal = backend.execute(
            &execution(thread, "set_goal_state", arguments.clone()),
            &execution(thread, "set_goal_state", arguments).invocation,
        );
        let BackendTerminal::Completed(value) = terminal else {
            panic!("host-bound goal must be updatable: {terminal:?}")
        };
        let value: Value = serde_json::from_slice(&value.canonical_bytes().unwrap()).unwrap();
        assert_eq!(value["goal_id"], "goal-host-1");

        // A different bound id (record belongs to another goal) still reports not_found.
        let mut other = WorkflowBackend::new(
            root.path(),
            "ws",
            Some("goal-other".to_owned()),
            vec![],
            vec![],
        )
        .unwrap();
        let arguments = json!({"state":"active","progress":"1"});
        let terminal = other.execute(
            &execution(thread, "set_goal_state", arguments.clone()),
            &execution(thread, "set_goal_state", arguments).invocation,
        );
        assert!(
            matches!(terminal, BackendTerminal::Unavailable { ref code, .. } if code == "not_found"),
            "{terminal:?}"
        );
    }
}
