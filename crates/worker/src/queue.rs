use super::*;

#[derive(Clone)]
struct QueueTarget {
    content: Vec<schema::Block>,
    steer: bool,
    assets: Option<Vec<worker_control::AssetRef>>,
}

pub(crate) fn execute_queue_transaction(
    ledger: &mut LockedLedger,
    timestamp: &str,
    transaction: &QueueTransaction,
) -> Result<QueueTransactionResult, Box<dyn std::error::Error>> {
    transaction.validate()?;
    let projection = ledger
        .projection()
        .ok_or("queue transaction requires a nonempty ledger")?;
    let retract_seq = projection
        .origin_tuples
        .get(&transaction.retract_origin)
        .copied();
    let replacement_seq = transaction
        .replacement_origin()
        .and_then(|origin| projection.origin_tuples.get(origin))
        .copied();
    if replacement_seq.is_some() && retract_seq.is_none() {
        return Err(StoreError::Corruption(
            "queue transaction replacement exists without its retraction".to_owned(),
        )
        .into());
    }

    let target = match queue_target(ledger, transaction, retract_seq.is_some())? {
        Some(target) => target,
        None if retract_seq.is_none() => {
            return Ok(queue_rejected(
                transaction,
                QueueTransactionRejectCode::QueueItemNotFound,
                None,
            ));
        }
        None => {
            return Err(StoreError::Corruption(
                "durable queue retraction no longer has a validator-checkable target".to_owned(),
            )
            .into());
        }
    };

    if matches!(transaction.action, QueueTransactionAction::Steer { .. })
        && (target.steer || !queue_steer_window_open(ledger))
    {
        if retract_seq.is_some() {
            return Err(StoreError::Corruption(
                "durable queue retraction belongs to an invalid steer transaction".to_owned(),
            )
            .into());
        }
        return Ok(queue_rejected(
            transaction,
            QueueTransactionRejectCode::SteerUnavailable,
            None,
        ));
    }

    let replacement = match &transaction.action {
        QueueTransactionAction::Edit {
            content,
            steer,
            assets,
            ..
        } => {
            if *steer != target.steer {
                return Err(StoreError::Corruption(
                    "queue edit steer bit does not match its target".to_owned(),
                )
                .into());
            }
            Some((content.clone(), *steer, assets.clone()))
        }
        QueueTransactionAction::Remove => None,
        QueueTransactionAction::Steer { .. } => {
            Some((target.content.clone(), true, target.assets.clone()))
        }
    };

    if let Some((_, _, assets)) = &replacement {
        if let Err(error) = verify_queue_assets(ledger, assets.as_deref().unwrap_or_default()) {
            if retract_seq.is_some() {
                return Err(StoreError::Corruption(format!(
                    "queue replacement asset became invalid after retraction: {error}"
                ))
                .into());
            }
            return Ok(queue_rejected(
                transaction,
                QueueTransactionRejectCode::AttachmentError,
                Some("CORRUPT".to_owned()),
            ));
        }
    }

    if let Some(seq) = retract_seq {
        validate_existing_retraction(ledger, transaction, seq)?;
    }
    if let (Some(seq), Some(replacement)) = (replacement_seq, replacement.as_ref()) {
        validate_existing_replacement(ledger, transaction, seq, replacement)?;
    }

    let deduplicated = retract_seq.is_some();
    let first_seq = if let Some(seq) = retract_seq {
        seq
    } else {
        let seq = ledger.next_seq();
        let mut object = origin_event(ledger, timestamp, "queue_edit", &transaction.retract_origin);
        object.insert(
            "supersedes".to_owned(),
            json!([{"from": transaction.target_seq, "to": transaction.target_seq}]),
        );
        ledger.append_contract(
            make_event(Value::Object(object))?,
            BarrierContext::default(),
        )?;
        seq
    };

    let last_seq = match (replacement, replacement_seq) {
        (None, None) => first_seq,
        (Some(_), Some(seq)) => seq,
        (Some((content, steer, assets)), None) => {
            let origin = transaction
                .replacement_origin()
                .expect("replacement action has origin");
            let seq = ledger.next_seq();
            let mut object = origin_event(ledger, timestamp, "input", origin);
            object.insert("content".to_owned(), serde_json::to_value(content)?);
            object.insert("steer".to_owned(), Value::Bool(steer));
            if let Some(assets) = assets {
                object.insert("assets".to_owned(), serde_json::to_value(assets)?);
            }
            ledger.append_contract(
                make_event(Value::Object(object))?,
                BarrierContext::default(),
            )?;
            seq
        }
        (None, Some(_)) => unreachable!("remove has no replacement origin"),
    };

    Ok(QueueTransactionResult {
        delivery: transaction.delivery.clone(),
        outcome: QueueTransactionOutcome::Committed {
            first_seq,
            last_seq,
            deduplicated,
        },
    })
}

fn queue_rejected(
    transaction: &QueueTransaction,
    code: QueueTransactionRejectCode,
    reason: Option<String>,
) -> QueueTransactionResult {
    QueueTransactionResult {
        delivery: transaction.delivery.clone(),
        outcome: QueueTransactionOutcome::Rejected { code, reason },
    }
}

fn queue_target(
    ledger: &LockedLedger,
    transaction: &QueueTransaction,
    own_retraction_exists: bool,
) -> Result<Option<QueueTarget>, Box<dyn std::error::Error>> {
    let projection = ledger
        .projection()
        .ok_or("queue target lookup requires a projection")?;
    let Some(event) = projection
        .events
        .iter()
        .find(|event| event.seq() == transaction.target_seq)
    else {
        return Ok(None);
    };
    if event.string_field("kind") != Some("input") {
        return Ok(None);
    }

    let consumed = projection.events.iter().any(|candidate| {
        let Ok(value) = event_json(candidate) else {
            return false;
        };
        if candidate.string_field("kind") == Some("turn_open") {
            return value
                .get("trigger")
                .and_then(Value::as_object)
                .and_then(|trigger| trigger.get("inputs"))
                .and_then(Value::as_array)
                .is_some_and(|inputs| {
                    inputs
                        .iter()
                        .any(|seq| seq.as_u64() == Some(transaction.target_seq))
                });
        }
        candidate.string_field("kind") == Some("attempt")
            && value
                .get("admits")
                .and_then(Value::as_array)
                .is_some_and(|ranges| {
                    ranges.iter().any(|range| {
                        range.get("from").and_then(Value::as_u64) <= Some(transaction.target_seq)
                            && range.get("to").and_then(Value::as_u64)
                                >= Some(transaction.target_seq)
                    })
                })
    });
    if consumed {
        return Ok(None);
    }

    let superseded_by_other = projection.events.iter().any(|candidate| {
        if candidate.string_field("kind") != Some("queue_edit") {
            return false;
        }
        if own_retraction_exists
            && candidate.origin_tuple().ok().flatten().as_ref() == Some(&transaction.retract_origin)
        {
            return false;
        }
        event_json(candidate)
            .ok()
            .and_then(|value| value.get("supersedes").cloned())
            .and_then(|value| value.as_array().cloned())
            .is_some_and(|ranges| {
                ranges.iter().any(|range| {
                    range.get("from").and_then(Value::as_u64) <= Some(transaction.target_seq)
                        && range.get("to").and_then(Value::as_u64) >= Some(transaction.target_seq)
                })
            })
    });
    if superseded_by_other {
        return Ok(None);
    }

    let value = event_json(event)?;
    let content = serde_json::from_value(
        value
            .get("content")
            .cloned()
            .ok_or("input target is missing content")?,
    )?;
    let assets = value
        .get("assets")
        .cloned()
        .map(serde_json::from_value)
        .transpose()?;
    Ok(Some(QueueTarget {
        content,
        steer: value.get("steer").and_then(Value::as_bool).unwrap_or(false),
        assets,
    }))
}

fn queue_steer_window_open(ledger: &LockedLedger) -> bool {
    ledger.projection().is_some_and(|projection| {
        projection.latest_turn.is_some()
            && !projection.terminal_tail
            && !projection.lifecycle.stop_active
            && !projection.lifecycle.open_hold
    })
}

fn verify_queue_assets(
    ledger: &LockedLedger,
    assets: &[worker_control::AssetRef],
) -> Result<(), StoreError> {
    let root = ledger
        .path()
        .parent()
        .ok_or_else(|| StoreError::Corruption("ledger has no thread folder".to_owned()))?
        .join("assets");
    let store = AssetStore::new(root)?;
    for asset in assets {
        store.verify_named(&asset.asset)?;
    }
    Ok(())
}

fn validate_existing_retraction(
    ledger: &LockedLedger,
    transaction: &QueueTransaction,
    seq: u64,
) -> Result<(), StoreError> {
    let event = event_at(ledger, seq)?;
    let value = event_json(event).map_err(|error| StoreError::Corruption(error.to_string()))?;
    let expected = json!([{"from": transaction.target_seq, "to": transaction.target_seq}]);
    if event.string_field("kind") != Some("queue_edit")
        || event.origin_tuple()? != Some(transaction.retract_origin.clone())
        || value.get("supersedes") != Some(&expected)
    {
        return Err(StoreError::Corruption(
            "queue transaction retraction origin resolves to mismatched bytes".to_owned(),
        ));
    }
    Ok(())
}

fn validate_existing_replacement(
    ledger: &LockedLedger,
    transaction: &QueueTransaction,
    seq: u64,
    replacement: &(
        Vec<schema::Block>,
        bool,
        Option<Vec<worker_control::AssetRef>>,
    ),
) -> Result<(), StoreError> {
    let event = event_at(ledger, seq)?;
    let value = event_json(event).map_err(|error| StoreError::Corruption(error.to_string()))?;
    let content = serde_json::to_value(&replacement.0)
        .map_err(|error| StoreError::Corruption(error.to_string()))?;
    let assets = replacement
        .2
        .as_ref()
        .map(serde_json::to_value)
        .transpose()
        .map_err(|error| StoreError::Corruption(error.to_string()))?;
    if event.string_field("kind") != Some("input")
        || event.origin_tuple()? != transaction.replacement_origin().cloned()
        || value.get("content") != Some(&content)
        || value.get("steer").and_then(Value::as_bool) != Some(replacement.1)
        || value.get("assets") != assets.as_ref()
    {
        return Err(StoreError::Corruption(
            "queue transaction replacement origin resolves to mismatched bytes".to_owned(),
        ));
    }
    Ok(())
}
