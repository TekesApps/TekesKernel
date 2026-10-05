//! Immutable continuation bindings, write-ahead intents and exact receipts.
//! Not wired to worker execution until the negotiated pending protocol lands.
use schema::IJsonValue;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use store::{AtomicPublisher, NamedLock};
use worker_control::ToolControl;
use worker_control::continuation::{
    ContinuationOperation, ToolContinuationOutcome, ToolContinuationRequest,
    ToolContinuationResponse,
};

#[derive(Debug, thiserror::Error)]
pub enum ContinuationJournalError {
    #[error("continuation conflict: {0}")]
    Conflict(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Store(#[from] store::StoreError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
type Result<T> = std::result::Result<T, ContinuationJournalError>;
fn conflict(message: impl Into<String>) -> ContinuationJournalError {
    ContinuationJournalError::Conflict(message.into())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationBinding {
    pub original: ToolControl,
    pub continuation_id: String,
    pub authority: IJsonValue,
    pub initial_state: IJsonValue,
}

pub enum ContinuationPreparation {
    Execute,
    Reconcile,
    Completed(ToolContinuationResponse),
}

pub struct ContinuationJournal {
    root: PathBuf,
}
impl ContinuationJournal {
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        if !root.as_ref().exists() {
            fs::create_dir(root.as_ref())?;
            fs::File::open(
                root.as_ref()
                    .parent()
                    .ok_or_else(|| conflict("journal root needs an existing parent"))?,
            )?
            .sync_all()?;
        }
        Ok(Self {
            root: root.as_ref().to_owned(),
        })
    }
    pub fn bind(&self, binding: &ContinuationBinding) -> Result<()> {
        ToolContinuationRequest::new(
            binding.original.clone(),
            binding.continuation_id.clone(),
            1,
            ContinuationOperation::Query,
        )
        .map_err(conflict)?;
        let _lock = NamedLock::exclusive(self.root.join(".lock"))?;
        let folder = self.root.join(&binding.continuation_id);
        fs::create_dir_all(&folder)?;
        fs::File::open(&self.root)?.sync_all()?;
        let path = folder.join("binding.json");
        if path.exists() {
            if self.read::<ContinuationBinding>(&path)? != *binding {
                return Err(conflict("binding bytes changed"));
            }
            return Ok(());
        }
        self.publish(&path, binding)
    }
    pub fn binding(&self, request: &ToolContinuationRequest) -> Result<ContinuationBinding> {
        request.validate().map_err(conflict)?;
        let folder = self.folder(request)?;
        self.read(&folder.join("binding.json"))
    }

    /// Serializes execution/reconciliation for this continuation across processes.
    /// A lost response leaves its intent unresolved; retries never call execute.
    pub fn resolve<E, R>(
        &self,
        request: &ToolContinuationRequest,
        execute: E,
        reconcile: R,
    ) -> Result<ToolContinuationResponse>
    where
        E: FnOnce() -> Result<ToolContinuationResponse>,
        R: FnOnce() -> Result<ToolContinuationResponse>,
    {
        request.validate().map_err(conflict)?;
        let folder = self.folder(request)?;
        let _execution = NamedLock::exclusive(folder.join(".execution.lock"))?;
        let response = match self.prepare(request)? {
            ContinuationPreparation::Completed(response) => return Ok(response),
            ContinuationPreparation::Execute => execute()?,
            ContinuationPreparation::Reconcile => reconcile()?,
        };
        self.commit(request, &response)?;
        Ok(response)
    }

    fn prepare(&self, request: &ToolContinuationRequest) -> Result<ContinuationPreparation> {
        request.validate().map_err(conflict)?;
        let _lock = NamedLock::exclusive(self.root.join(".lock"))?;
        let folder = self.folder(request)?;
        let intent = folder.join(format!("{}.intent.json", request.step));
        let receipt = folder.join(format!("{}.receipt.json", request.step));
        if intent.exists() {
            let stored: ToolContinuationRequest = self.read(&intent)?;
            if !request.matches_receipt(&stored).map_err(conflict)? {
                return Err(conflict("step request changed"));
            }
            if receipt.exists() {
                let response: ToolContinuationResponse = self.read(&receipt)?;
                response.validate_for(request).map_err(conflict)?;
                return Ok(ContinuationPreparation::Completed(response));
            }
            return Ok(ContinuationPreparation::Reconcile);
        }
        if request.step > 1 {
            let previous: ToolContinuationResponse =
                self.read(&folder.join(format!("{}.receipt.json", request.step - 1)))?;
            let prior_request: ToolContinuationRequest =
                self.read(&folder.join(format!("{}.intent.json", request.step - 1)))?;
            previous.validate_for(&prior_request).map_err(conflict)?;
            if prior_request.original != request.original
                || prior_request.continuation_id != request.continuation_id
            {
                return Err(conflict("previous step belongs to another binding"));
            }
            match previous.result {
                ToolContinuationOutcome::Pending {
                    next_step,
                    continuation_id,
                    ..
                } if next_step == request.step && continuation_id == request.continuation_id => {}
                _ => {
                    return Err(conflict(
                        "previous step is terminal or does not authorize this step",
                    ));
                }
            }
        }
        self.publish(&intent, request)?;
        Ok(ContinuationPreparation::Execute)
    }
    fn commit(
        &self,
        request: &ToolContinuationRequest,
        response: &ToolContinuationResponse,
    ) -> Result<()> {
        response.validate_for(request).map_err(conflict)?;
        let _lock = NamedLock::exclusive(self.root.join(".lock"))?;
        let folder = self.folder(request)?;
        let stored: ToolContinuationRequest =
            self.read(&folder.join(format!("{}.intent.json", request.step)))?;
        if !request.matches_receipt(&stored).map_err(conflict)? {
            return Err(conflict("commit does not match intent"));
        }
        let path = folder.join(format!("{}.receipt.json", request.step));
        if path.exists() {
            if self.read::<ToolContinuationResponse>(&path)? != *response {
                return Err(conflict("receipt bytes changed"));
            }
            return Ok(());
        }
        self.publish(&path, response)
    }
    fn folder(&self, request: &ToolContinuationRequest) -> Result<PathBuf> {
        let folder = self.root.join(&request.continuation_id);
        let binding: ContinuationBinding = self.read(&folder.join("binding.json"))?;
        if binding.original != request.original
            || binding.continuation_id != request.continuation_id
        {
            return Err(conflict("request does not belong to binding"));
        }
        Ok(folder)
    }
    fn read<T: serde::de::DeserializeOwned>(&self, path: &Path) -> Result<T> {
        Ok(serde_json::from_slice(&fs::read(path)?)?)
    }
    fn publish<T: Serialize>(&self, path: &Path, value: &T) -> Result<()> {
        let bytes = serde_json_canonicalizer::to_vec(value)?;
        AtomicPublisher::replace(path, &bytes)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reopen_distinguishes_unresolved_intent_from_immutable_receipt() {
        let dir = tempfile::tempdir().unwrap();
        let original = ToolControl::new(
            "018f0000-0000-7000-8000-000000000003",
            "018f0000-0000-7000-8000-000000000003",
            1,
            "call-1",
            "mcp__test__task",
            IJsonValue::parse_str("{}").unwrap(),
        )
        .unwrap();
        let binding = ContinuationBinding {
            original: original.clone(),
            continuation_id: "a".repeat(64),
            authority: IJsonValue::parse_str(r#"{"server":"test","generation":1}"#).unwrap(),
            initial_state: IJsonValue::parse_str(r#"{"taskId":"remote-1","status":"working"}"#)
                .unwrap(),
        };
        let request = ToolContinuationRequest::new(
            original.clone(),
            binding.continuation_id.clone(),
            1,
            ContinuationOperation::Query,
        )
        .unwrap();
        let journal = ContinuationJournal::open(dir.path()).unwrap();
        journal.bind(&binding).unwrap();
        let binding_bytes = fs::read(
            dir.path()
                .join(&binding.continuation_id)
                .join("binding.json"),
        )
        .unwrap();
        assert!(matches!(
            journal.prepare(&request).unwrap(),
            ContinuationPreparation::Execute
        ));
        drop(journal);
        let journal = ContinuationJournal::open(dir.path()).unwrap();
        assert!(matches!(
            journal.prepare(&request).unwrap(),
            ContinuationPreparation::Reconcile
        ));
        let response = ToolContinuationResponse {
            request_id: request.request_id.clone(),
            call_id: original.call_id.clone(),
            result: ToolContinuationOutcome::Pending {
                continuation_id: binding.continuation_id.clone(),
                next_step: 2,
                state: binding.initial_state.clone(),
            },
        };
        journal.commit(&request, &response).unwrap();
        journal.commit(&request, &response).unwrap();
        assert!(
            matches!(journal.prepare(&request).unwrap(),ContinuationPreparation::Completed(value) if value==response)
        );
        let changed = ToolContinuationRequest::new(
            original.clone(),
            binding.continuation_id.clone(),
            1,
            ContinuationOperation::Cancel,
        )
        .unwrap();
        assert!(journal.prepare(&changed).is_err());
        let second = ToolContinuationRequest::new(
            original.clone(),
            binding.continuation_id.clone(),
            2,
            ContinuationOperation::Query,
        )
        .unwrap();
        assert!(matches!(
            journal.prepare(&second).unwrap(),
            ContinuationPreparation::Execute
        ));
        let terminal = ToolContinuationResponse {
            request_id: second.request_id.clone(),
            call_id: original.call_id.clone(),
            result: ToolContinuationOutcome::Completed {
                value: IJsonValue::parse_str(r#"{"content":[]}"#).unwrap(),
            },
        };
        journal.commit(&second, &terminal).unwrap();
        let third = ToolContinuationRequest::new(
            original,
            binding.continuation_id.clone(),
            3,
            ContinuationOperation::Query,
        )
        .unwrap();
        assert!(journal.prepare(&third).is_err());
        assert_eq!(
            fs::read(
                dir.path()
                    .join(&binding.continuation_id)
                    .join("binding.json")
            )
            .unwrap(),
            binding_bytes
        );
        let mut changed_binding = binding.clone();
        changed_binding.authority = IJsonValue::parse_str(r#"{"generation":2}"#).unwrap();
        assert!(journal.bind(&changed_binding).is_err());
    }
}

#[cfg(test)]
mod execution_tests {
    use super::*;
    fn concurrent_request() -> ToolContinuationRequest {
        let original = ToolControl::new(
            "018f0000-0000-7000-8000-000000000003",
            "018f0000-0000-7000-8000-000000000003",
            1,
            "call-concurrent",
            "mcp__test__task",
            IJsonValue::parse_str("{}").unwrap(),
        )
        .unwrap();
        ToolContinuationRequest::new(original, "c".repeat(64), 1, ContinuationOperation::Cancel)
            .unwrap()
    }

    #[test]
    fn continuation_process_participant() {
        use std::io::Write;
        let Some(root) = std::env::var_os("TEKES_CONTINUATION_TEST_ROOT") else {
            return;
        };
        let root = PathBuf::from(root);
        let participant = std::env::var("TEKES_CONTINUATION_TEST_PARTICIPANT").unwrap();
        let request = concurrent_request();
        let journal = ContinuationJournal::open(&root).unwrap();
        fs::write(root.join(format!("ready-{participant}")), b"ready").unwrap();
        let result = journal
            .resolve(
                &request,
                || {
                    let mut calls = fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(root.join("remote-calls"))?;
                    writeln!(calls, "cancel")?;
                    calls.sync_all()?;
                    Ok(ToolContinuationResponse {
                        request_id: request.request_id.clone(),
                        call_id: request.original.call_id.clone(),
                        result: ToolContinuationOutcome::Completed {
                            value: IJsonValue::parse_str(r#"{"cancelled":true}"#).unwrap(),
                        },
                    })
                },
                || panic!("concurrent request must wait for the receipt, not reconcile in flight"),
            )
            .unwrap();
        fs::write(
            root.join(format!("result-{participant}")),
            serde_json_canonicalizer::to_vec(&result).unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn concurrent_processes_execute_once_and_replay_identical_receipts() {
        let dir = tempfile::tempdir().unwrap();
        let request = concurrent_request();
        let journal = ContinuationJournal::open(dir.path()).unwrap();
        journal
            .bind(&ContinuationBinding {
                original: request.original.clone(),
                continuation_id: request.continuation_id.clone(),
                authority: IJsonValue::parse_str("{}").unwrap(),
                initial_state: IJsonValue::parse_str("{}").unwrap(),
            })
            .unwrap();
        let lock = NamedLock::exclusive(
            dir.path()
                .join(&request.continuation_id)
                .join(".execution.lock"),
        )
        .unwrap();
        let mut children: Vec<_> = (0..4)
            .map(|index| {
                std::process::Command::new(std::env::current_exe().unwrap())
                    .args([
                        "--exact",
                        "continuation_journal::execution_tests::continuation_process_participant",
                    ])
                    .env("TEKES_CONTINUATION_TEST_ROOT", dir.path())
                    .env("TEKES_CONTINUATION_TEST_PARTICIPANT", index.to_string())
                    .spawn()
                    .unwrap()
            })
            .collect();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !(0..4).all(|index| dir.path().join(format!("ready-{index}")).exists()) {
            if std::time::Instant::now() >= deadline {
                for child in &mut children {
                    let _ = child.kill();
                    let _ = child.wait();
                }
                panic!("continuation participants did not reach the execution lock");
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert!(!dir.path().join("remote-calls").exists());
        drop(lock);
        for child in &mut children {
            loop {
                if let Some(result) = child.try_wait().unwrap() {
                    assert!(result.success());
                    break;
                }
                if std::time::Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    panic!("continuation participant did not finish");
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
        assert_eq!(
            fs::read_to_string(dir.path().join("remote-calls")).unwrap(),
            "cancel\n"
        );
        let receipt = fs::read(
            dir.path()
                .join(&request.continuation_id)
                .join("1.receipt.json"),
        )
        .unwrap();
        for index in 0..4 {
            assert_eq!(
                fs::read(dir.path().join(format!("result-{index}"))).unwrap(),
                receipt
            );
        }
    }

    #[test]
    fn lost_response_uses_reconciliation_and_receipt_replay_never_reexecutes() {
        let dir = tempfile::tempdir().unwrap();
        let original = ToolControl::new(
            "018f0000-0000-7000-8000-000000000003",
            "018f0000-0000-7000-8000-000000000003",
            1,
            "call-1",
            "mcp__test__task",
            IJsonValue::parse_str("{}").unwrap(),
        )
        .unwrap();
        let binding = ContinuationBinding {
            original: original.clone(),
            continuation_id: "b".repeat(64),
            authority: IJsonValue::parse_str("{}").unwrap(),
            initial_state: IJsonValue::parse_str("{}").unwrap(),
        };
        let request = ToolContinuationRequest::new(
            original.clone(),
            binding.continuation_id.clone(),
            1,
            ContinuationOperation::Cancel,
        )
        .unwrap();
        let journal = ContinuationJournal::open(dir.path()).unwrap();
        journal.bind(&binding).unwrap();
        let calls = std::sync::atomic::AtomicUsize::new(0);
        assert!(
            journal
                .resolve(
                    &request,
                    || {
                        calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        Err(conflict("simulated lost response after remote operation"))
                    },
                    || panic!("fresh request cannot reconcile")
                )
                .is_err()
        );
        drop(journal);
        let journal = ContinuationJournal::open(dir.path()).unwrap();
        let response = ToolContinuationResponse {
            request_id: request.request_id.clone(),
            call_id: original.call_id,
            result: ToolContinuationOutcome::Completed {
                value: IJsonValue::parse_str(r#"{"cancelled":true}"#).unwrap(),
            },
        };
        assert_eq!(
            journal
                .resolve(
                    &request,
                    || panic!("must not repeat operation"),
                    || Ok(response.clone())
                )
                .unwrap(),
            response
        );
        assert_eq!(
            journal
                .resolve(
                    &request,
                    || panic!("must not execute receipt"),
                    || panic!("must not reconcile receipt")
                )
                .unwrap(),
            response
        );
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}
