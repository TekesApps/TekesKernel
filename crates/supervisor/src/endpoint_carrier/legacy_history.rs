//! Read-only legacy history, isolated from catalog trust and execution recovery.
use super::*;
use sha2::{Digest, Sha256};

pub(super) struct FrozenLegacyHistory {
    digest: String,
    rows: Vec<endpoint::JournalRecord>,
}

impl FrozenLegacyHistory {
    fn through(&self) -> i64 {
        self.rows.last().map_or(-1, |row| row.event.seq as i64)
    }

    fn page(
        &self,
        before: Option<i64>,
        limit: usize,
    ) -> Result<endpoint::HistoryPage, ProductionCarrierError> {
        if !(1..=endpoint::MAX_JOURNAL_WINDOW_MESSAGES).contains(&limit)
            || before.is_some_and(|v| v < -1 || v > self.through())
        {
            return Err(ProductionCarrierError::Lifecycle(
                "invalid historical window".into(),
            ));
        }
        endpoint::history_page(&self.rows, before.map(|v| v.max(0) as u64), Some(limit))
            .map_err(|e| ProductionCarrierError::Lifecycle(e.to_string()))
    }
}

fn digest(bytes: &[u8]) -> String {
    format!("sha256-{:x}", Sha256::digest(bytes))
}

impl ProductionCarrierStreams {
    /// A display-only compatibility read must not open an append/management
    /// loophole. Recognize it before first open as well as while cached. Other
    /// formats retain their existing admission behavior.
    pub(super) fn legacy_mutation_failure(
        &self,
        request: &EndpointHostCall,
    ) -> Option<endpoint::RpcResult> {
        let session = payload_session_id(&request.payload)?;
        endpoint::validate_session_id(&session).ok()?;
        let known = self
            .inner
            .legacy_histories
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .contains_key(&session);
        let recognized = known
            || matches!(
                self.verify_session_carrier(&session),
                Err(ProductionCarrierError::Quarantined(_))
            ) && fs::read(
                self.inner
                    .root
                    .join("threads")
                    .join(&session)
                    .join("main.jsonl"),
            )
            .ok()
            .is_some_and(|bytes| {
                schema::validate_historical_ledger(&bytes, schema::READER_VERSION).is_ok()
            });
        recognized.then(|| endpoint::RpcResult {
            ok: false, value: None, error: Some(endpoint::RpcError {
                code: "internal".into(),
                message: "This legacy session is available as read-only history; execution facts are unavailable".into(),
                details: IJsonValue::parse_str(r#"{"reason":"legacy-history-read-only"}"#).expect("constant I-JSON"),
            }),
        })
    }

    pub(super) fn open_legacy_journal(
        &self,
        generation: u64,
        address: SessionAddress,
        max_messages: usize,
        lease: JournalLease,
    ) -> Result<SessionStreamReceiver, ProductionCarrierError> {
        endpoint::validate_session_id(&address.session_id)
            .map_err(|e| ProductionCarrierError::Lifecycle(e.to_string()))?;
        if self.live_sessions().contains(&address.session_id) {
            return Err(ProductionCarrierError::Lifecycle(
                "historical session has a live worker".into(),
            ));
        }
        let folder = self.inner.root.join("threads").join(&address.session_id);
        let bytes = fs::read(folder.join("main.jsonl"))?;
        let ledger = schema::validate_historical_ledger(&bytes, schema::READER_VERSION)?;
        if ledger
            .events()
            .first()
            .and_then(|e| e.string_field("thread"))
            != Some(address.session_id.as_str())
        {
            return Err(ProductionCarrierError::Lifecycle(
                "historical ledger address mismatch".into(),
            ));
        }
        let rows = endpoint::Projector::read_only_legacy_history(&ledger, &folder)
            .map_err(|e| ProductionCarrierError::Lifecycle(e.to_string()))?;
        let frozen = Arc::new(FrozenLegacyHistory {
            digest: digest(&bytes),
            rows,
        });
        if digest(&fs::read(folder.join("main.jsonl"))?) != frozen.digest {
            return Err(ProductionCarrierError::Lifecycle(
                "historical source changed during projection".into(),
            ));
        }
        let page = frozen.page(None, max_messages)?;
        let through = frozen.through();
        let projections = IJsonValue::parse(&serde_json::to_vec(&json!({
            "asOfSeq": through,
            "values": { "historyAccess": {
                "mode": "read-only", "reason": "legacy-tool-catalog-unavailable",
                "sourceDigest": frozen.digest,
            }}
        }))?)?;
        {
            let mut histories = self.inner.legacy_histories.lock().map_err(|_| poisoned())?;
            if histories
                .get(&address.session_id)
                .is_some_and(|old| old.digest != frozen.digest)
            {
                return Err(ProductionCarrierError::Lifecycle(
                    "historical source changed; close the previous history first".into(),
                ));
            }
            histories.insert(address.session_id.clone(), frozen);
        }
        Ok(Box::new(ProductionStream {
            _lease: Some(lease),
            owner: self.clone(),
            generation,
            kind: ProductionStreamKind::Historical,
            pending: [SessionSyncFrame::JournalSnapshot {
                generation,
                snapshot: SessionJournalSnapshot {
                    window_limit: max_messages,
                    address,
                    through_sequence: through,
                    entries: page.events,
                    has_more_before: page.has_more,
                    projections,
                },
            }]
            .into_iter()
            .collect(),
            source: None,
        }))
    }

    pub(super) fn legacy_journal_page(
        &self,
        address: &SessionAddress,
        through: i64,
        before: Option<i64>,
        limit: usize,
    ) -> Result<Option<SessionJournalPage>, ProductionCarrierError> {
        let frozen = self
            .inner
            .legacy_histories
            .lock()
            .map_err(|_| poisoned())?
            .get(&address.session_id)
            .cloned();
        let Some(frozen) = frozen else {
            return Ok(None);
        };
        if through != frozen.through() {
            return Err(ProductionCarrierError::Lifecycle(
                "historical cut mismatch".into(),
            ));
        }
        let current = fs::read(
            self.inner
                .root
                .join("threads")
                .join(&address.session_id)
                .join("main.jsonl"),
        )?;
        if digest(&current) != frozen.digest {
            return Err(ProductionCarrierError::Lifecycle(
                "historical source changed; reopen required".into(),
            ));
        }
        let page = frozen.page(before, limit)?;
        Ok(Some(SessionJournalPage {
            address: address.clone(),
            through_sequence: through,
            entries: page.events,
            has_more_before: page.has_more,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Quarantined;
    impl SupervisorSessionAuthority for Quarantined {
        fn live_sessions(&self) -> HashSet<String> {
            HashSet::new()
        }
        fn reconcile_projection(&self, _: &str) -> Result<(), String> {
            panic!("historical reads must not reconcile a durable journal")
        }
        fn ensure_verified(&self, _: &str) -> Result<(), SessionGateError> {
            Err(SessionGateError::Quarantined("missing tool catalog".into()))
        }
    }

    fn fixture() -> (String, Vec<u8>) {
        let mut bytes = Vec::new();
        let mut id = String::new();
        for line in include_str!("../../../../fixtures/invalid/_valid-baseline.jsonl").lines() {
            let mut value: Value = serde_json::from_str(line).unwrap();
            match value["kind"].as_str().unwrap() {
                "genesis" => id = value["thread"].as_str().unwrap().into(),
                "run_start" => value["binary"] = json!("dev-cfa7a6be31c9-123456789abc"),
                "epoch" => {
                    value.as_object_mut().unwrap().remove("tools");
                    value["tools_digest"] = json!(format!("sha256-{}", "a".repeat(64)));
                }
                "output" => {
                    value.as_object_mut().unwrap().remove("usage");
                }
                _ => {}
            }
            bytes.extend(
                IJsonValue::parse(&serde_json::to_vec(&value).unwrap())
                    .unwrap()
                    .canonical_bytes()
                    .unwrap(),
            );
            bytes.push(b'\n');
        }
        (id, bytes)
    }

    #[tokio::test]
    async fn quarantined_history_is_readable_but_never_promoted_to_execution_trust() {
        let root = tempfile::tempdir().unwrap();
        let (id, bytes) = fixture();
        let folder = root.path().join("threads").join(&id);
        fs::create_dir_all(&folder).unwrap();
        fs::write(folder.join("main.jsonl"), &bytes).unwrap();
        let carrier = ProductionCarrierStreams::new(root.path());
        let authority: Arc<dyn SupervisorSessionAuthority> = Arc::new(Quarantined);
        carrier.attach_session_authority(Arc::downgrade(&authority));
        let address = SessionAddress {
            session_id: id.clone(),
        };
        let mut stream = carrier
            .open_mux(
                1,
                SessionStreamTarget::SessionJournal {
                    address: address.clone(),
                    max_messages: 1,
                },
            )
            .unwrap();
        let SessionSyncFrame::JournalSnapshot { snapshot, .. } =
            stream.recv().await.unwrap().unwrap()
        else {
            panic!("snapshot expected")
        };
        assert!(!snapshot.entries.is_empty());
        assert!(snapshot.has_more_before);
        let projections = serde_json::to_value(&snapshot.projections).unwrap();
        assert_eq!(projections["values"]["historyAccess"]["mode"], "read-only");
        let frozen = carrier
            .inner
            .legacy_histories
            .lock()
            .unwrap()
            .get(&id)
            .unwrap()
            .clone();
        let whole = frozen.page(None, 500).unwrap();
        let first = snapshot.entries.first().unwrap().event.seq as i64;
        let earlier = carrier
            .legacy_journal_page(&address, snapshot.through_sequence, Some(first), 500)
            .unwrap()
            .unwrap();
        assert!(!earlier.entries.is_empty());
        assert_eq!(
            earlier.entries.len() + snapshot.entries.len(),
            whole.events.len()
        );
        assert!(
            carrier.verify_session(&id).is_err(),
            "read-only fallback never changes the strict gate"
        );
        assert!(schema::validate_ledger(&bytes, schema::READER_VERSION).is_err());
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(5), stream.recv())
                .await
                .is_err(),
            "static history stays attached without an EOF/retry loop"
        );
        assert_eq!(fs::read(folder.join("main.jsonl")).unwrap(), bytes);
        assert_eq!(
            fs::read_dir(&folder).unwrap().count(),
            1,
            "no derived files written"
        );
        fs::write(
            folder.join("main.jsonl"),
            [bytes.as_slice(), b"\n"].concat(),
        )
        .unwrap();
        assert!(
            carrier
                .legacy_journal_page(&address, snapshot.through_sequence, None, 50)
                .is_err()
        );
        drop(stream);
        assert!(carrier.inner.legacy_histories.lock().unwrap().is_empty());
    }
}
