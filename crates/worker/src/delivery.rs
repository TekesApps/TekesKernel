use super::*;

/// Appends and receipts every delivery that has already reached this process:
/// parked deliveries, deferred stops, and whatever the channel still holds.
/// Called at each yield point with ledger access (before an attempt's lease,
/// after tool execution, after the turn) so a delivery never waits for the
/// worker's exit. A stop that arrived during the turn was acted on through the
/// cancellation flag; echoing it here makes the generation durable before any
/// settle that answers it.
pub(crate) fn drain_ready_deliveries(
    ledger: &mut LockedLedger,
    options: &Options,
    selected: &Selected,
    lines: &mut ControlLines,
    stdout: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    for line in lines.drain_ready() {
        let Ok(message) = decode_supervisor(line.as_bytes()) else {
            return Err(Box::new(ProtocolFailure(
                "undecodable supervisor line at a yield point".to_owned(),
            )));
        };
        handle_delivery(ledger, options, selected, stdout, message)?;
    }
    Ok(())
}

/// The provider loop's yield point: appends and receipts the deliveries the
/// reader thread parked while the loop was busy. Stops stay on the channel and
/// the cancellation flag; the loop's own waits consume them.
pub(crate) fn drain_parked_deliveries(
    ledger: &mut LockedLedger,
    options: &Options,
    selected: &Selected,
    cancellation: &RuntimeCancellation,
    stdout: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    for line in cancellation.take_parked_deliveries() {
        let Ok(message) = decode_supervisor(line.as_bytes()) else {
            return Err(Box::new(ProtocolFailure(
                "undecodable supervisor delivery at a yield point".to_owned(),
            )));
        };
        handle_delivery(ledger, options, selected, stdout, message)?;
    }
    Ok(())
}

/// One delivery, appended under this worker's lock and receipted (D-48). Non-
/// delivery messages that reached a yield point (`ping`, `lease`, `pong` echoes,
/// `launch_result`, unknown keys) are ignored: their waits have already ended.
pub(crate) fn handle_delivery(
    ledger: &mut LockedLedger,
    options: &Options,
    selected: &Selected,
    stdout: &mut impl Write,
    message: SupervisorMessage,
) -> Result<(), Box<dyn std::error::Error>> {
    require_version(selected)?;
    match message {
        SupervisorMessage::Input(input) => {
            let mut object =
                origin_event(ledger, &options.event_timestamp(), "input", &input.origin);
            object.insert("content".to_owned(), serde_json::to_value(input.content)?);
            if let Some(submission) = input.submission {
                object.insert("submission".to_owned(), serde_json::to_value(submission)?);
            }
            if let Some(steer) = input.steer {
                object.insert("steer".to_owned(), Value::Bool(steer));
            }
            if let Some(assets) = input.assets {
                object.insert("assets".to_owned(), serde_json::to_value(assets)?);
            }
            append_and_receipt(ledger, stdout, input.delivery, &input.origin, object)
        }
        SupervisorMessage::ApprovalResponse(answer) => {
            let mut object = origin_event(
                ledger,
                &options.event_timestamp(),
                "approval_response",
                &answer.origin,
            );
            object.insert(
                "turn".to_owned(),
                Value::from(
                    ledger
                        .projection()
                        .and_then(|projection| projection.latest_turn)
                        .ok_or("approval response requires an open turn")?,
                ),
            );
            object.insert("call".to_owned(), Value::String(answer.call));
            object.insert("grant".to_owned(), Value::Bool(answer.grant));
            if let Some(value) = answer.answer {
                object.insert("answer".to_owned(), serde_json::to_value(value)?);
            }
            append_and_receipt(ledger, stdout, answer.delivery, &answer.origin, object)
        }
        SupervisorMessage::QueueEdit(edit) => {
            let mut object = origin_event(
                ledger,
                &options.event_timestamp(),
                "queue_edit",
                &edit.origin,
            );
            object.insert(
                "supersedes".to_owned(),
                serde_json::to_value(edit.supersedes)?,
            );
            append_and_receipt(ledger, stdout, edit.delivery, &edit.origin, object)
        }
        SupervisorMessage::Stop(stop) => {
            let mut object = origin_event(
                ledger,
                &options.event_timestamp(),
                "stop_requested",
                &stop.origin,
            );
            object.insert("generation".to_owned(), Value::from(stop.generation));
            append_and_receipt(ledger, stdout, stop.delivery, &stop.origin, object)
        }
        SupervisorMessage::Meta(meta) => {
            let mut object = origin_event(ledger, &options.event_timestamp(), "meta", &meta.origin);
            if let Some(title) = meta.title {
                object.insert("title".to_owned(), Value::String(title));
            }
            if let Some(labels) = meta.labels {
                object.insert("labels".to_owned(), serde_json::to_value(labels)?);
            }
            append_and_receipt(ledger, stdout, meta.delivery, &meta.origin, object)
        }
        SupervisorMessage::Compact(request) => {
            if ledger
                .projection()
                .is_some_and(|p| p.origin_tuples.contains_key(&request.origin))
            {
                return append_and_receipt(
                    ledger,
                    stdout,
                    request.delivery,
                    &request.origin,
                    Map::new(),
                );
            }
            let projection = ledger
                .projection()
                .ok_or("manual compact requires projection")?;
            let turn = projection
                .latest_turn
                .unwrap_or(0)
                .saturating_add(u64::from(
                    projection.terminal_tail || projection.latest_turn.is_none(),
                ));
            let mut object = build_compaction_event(ledger, options, turn, true, None)?
                .ok_or("manual compaction did not produce a receipt-bearing event")?;
            object.insert("origin_key".into(), json!(request.origin.key));
            object.insert(
                "origin_tuple".into(),
                serde_json::to_value(&request.origin)?,
            );
            append_and_receipt(ledger, stdout, request.delivery, &request.origin, object)
        }
        SupervisorMessage::QueueTransaction(transaction) => {
            let result =
                execute_queue_transaction(ledger, &options.event_timestamp(), &transaction)?;
            stdout.write_all(&encode_queue_transaction_result(&result)?)?;
            stdout.flush()?;
            announce_appended(ledger, stdout)
        }
        SupervisorMessage::Ping(ping) => {
            stdout.write_all(&encode_line("pong", &worker_control::Pong { id: ping.id })?)?;
            stdout.flush()?;
            Ok(())
        }
        SupervisorMessage::Unknown { .. }
        | SupervisorMessage::Lease(_)
        | SupervisorMessage::LaunchResult(_) => Ok(()),
    }
}

/// One post-turn transition, derived from the ledger and the cancellation
/// cause; the worker never waits on stdin after its turn (03-worker R3-5).
///
/// | tail after the turn | exit |
/// |---|---|
/// | settled or open hold | 0, nothing appended |
/// | stop requested, no unresolved work | `settle {interrupted, user_stop}`, 0 |
/// | stop requested, unresolved work | nonzero, no settle: the recovery run terminalizes |
/// | open turn, no stop (lease denied, provider unavailable, no provider) | 0 (nonzero with unresolved work), no settle: `recovery_needed` |
pub(crate) fn post_turn_exit(
    ledger: &mut LockedLedger,
    options: &Options,
    cancellation: &RuntimeCancellation,
    stdout: &mut impl Write,
) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let facts = ledger
        .projection()
        .ok_or("post-turn ledger has no projection")?
        .lifecycle
        .clone();
    if facts.terminal_tail || facts.open_hold {
        announce_appended(ledger, stdout)?;
        return Ok(ExitCode::SUCCESS);
    }
    if facts.continuation_wait_until.is_some()
        && !cancellation.stop_requested()
        && !facts.stop_active
    {
        // Parked remote continuation: the turn stays open by design and the
        // supervisor resumes the line when the durable poll instant passes.
        announce_appended(ledger, stdout)?;
        return Ok(ExitCode::SUCCESS);
    }
    let Some(turn) = facts.latest_turn else {
        announce_appended(ledger, stdout)?;
        return Ok(ExitCode::SUCCESS);
    };
    if cancellation.stop_requested() || facts.stop_active {
        if facts.unresolved_work {
            announce_appended(ledger, stdout)?;
            eprintln!(
                "tekes-worker: stop left turn {turn} with unresolved work; exiting for tail recovery"
            );
            return Ok(ExitCode::FAILURE);
        }
        append_settle(
            ledger,
            &options.event_timestamp(),
            turn,
            "interrupted",
            Some("user_stop"),
        )?;
        announce_appended(ledger, stdout)?;
        return Ok(ExitCode::SUCCESS);
    }
    announce_appended(ledger, stdout)?;
    eprintln!(
        "tekes-worker: turn {turn} ended without a settle or a hold (cause={}); exiting for tail recovery",
        if cancellation.supervisor_lost() {
            "supervisor_lost"
        } else {
            "no_provider_outcome"
        }
    );
    Ok(if facts.unresolved_work {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}
