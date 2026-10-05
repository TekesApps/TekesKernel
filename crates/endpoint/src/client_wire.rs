//! Public frames matching the Client-owned SessionEndpoint DTOs.
//! Ledger/multiplexer records stay internal; no Client translation is required.
use crate::{SessionControlItem, SessionMuxServerFrame, SessionSyncFrame, WorkspaceSummary};
use serde_json::{Value, json};

fn workspace(value: &WorkspaceSummary) -> Value {
    json!({"workspaceId":value.id,"path":value.path,"title":value.title,
        "sessionIds":value.session_ids,"createdAt":value.created_at,"updatedAt":value.updated_at})
}
fn queue(value: &SessionControlItem) -> Value {
    json!({"sessionId":value.session_id,"items":value.queue})
}
fn jobs(value: &SessionControlItem) -> Value {
    json!({"sessionId":value.session_id,"jobs":value.jobs})
}
fn projection(value: &SessionControlItem) -> Value {
    json!({"sessionId":value.session_id,"projections":value.projections})
}

pub fn client_sync_frames(frame: &SessionSyncFrame) -> Vec<Value> {
    use SessionSyncFrame::*;
    let generation = frame.generation().to_string();
    let frames = match frame {
        WorkspaceBaseline { baseline, .. } => vec![json!({"type":"baseline","snapshot":{
            "items":baseline.items.iter().map(workspace).collect::<Vec<_>>(),
            "archivedSessionIds":baseline.archived_session_ids}})],
        WorkspaceUpsert {
            workspace: value, ..
        } => vec![json!({"type":"upsert","workspace":workspace(value)})],
        WorkspaceRemove { workspace_id, .. } => {
            vec![json!({"type":"remove","workspaceId":workspace_id})]
        }
        WorkspaceOrder { workspace_ids, .. } => {
            vec![json!({"type":"reorder","workspaceIds":workspace_ids})]
        }
        ArchivedSessions { session_ids, .. } => {
            vec![json!({"type":"archived","sessionIds":session_ids})]
        }
        InventoryBaseline { items, .. } => {
            vec![json!({"type":"baseline","snapshot":{"items":items}})]
        }
        InventoryUpsert { session, .. } => vec![json!({"type":"upsert","session":session})],
        InventoryRemove { session_id, .. } => vec![json!({"type":"remove","sessionId":session_id})],
        JournalSnapshot { snapshot, .. } => {
            return vec![json!({"type":"snapshot","snapshot":{
            "generation":generation,"sessionId":snapshot.address.session_id,
            "throughSequence":snapshot.through_sequence,"windowLimit":snapshot.window_limit,
            "entries":snapshot.entries,"hasMoreBefore":snapshot.has_more_before,"projections":snapshot.projections}})];
        }
        JournalEvent {
            address,
            event,
            view,
            ..
        } => {
            let mut entry = json!({"event":event});
            if let Some(view) = view {
                entry["view"] = json!(view);
            }
            vec![json!({"type":"event","sessionId":address.session_id,"entry":entry})]
        }
        JournalTransient { address, event, .. } => {
            vec![json!({"type":"transient","sessionId":address.session_id,"event":event})]
        }
        JournalProjection {
            address,
            key,
            value,
            sequence,
            ..
        } => vec![json!({"type":"projection","frame":{
            "type":"session/projection","sessionId":address.session_id,"key":key,"value":value,"seq":sequence}})],
        ControlBaseline { items, .. } => vec![json!({"type":"baseline","snapshot":{
            "queues":items.iter().map(queue).collect::<Vec<_>>(),
            "jobs":items.iter().map(jobs).collect::<Vec<_>>(),
            "projections":items.iter().map(projection).collect::<Vec<_>>()}})],
        ControlUpsert { control, .. } => vec![
            json!({"type":"queue","state":queue(control)}),
            json!({"type":"jobs","state":jobs(control)}),
            json!({"type":"projection","state":projection(control)}),
        ],
        ActionableBaseline { items, .. } => {
            let mut values = vec![json!({"type":"reset"})];
            values.extend(
                items
                    .iter()
                    .map(|item| json!({"type":"upsert","actionable":item})),
            );
            values
        }
        ActionableUpsert { actionable, .. } => {
            vec![json!({"type":"upsert","actionable":actionable})]
        }
        ActionableResolved { id, revision, .. } => {
            vec![json!({"type":"resolved","actionableId":id,"revision":revision})]
        }
    };
    frames
        .into_iter()
        .map(|mut frame| {
            frame["generation"] = json!(generation);
            frame
        })
        .collect()
}

pub fn client_mux_frames(frame: &SessionMuxServerFrame) -> Result<Vec<Value>, serde_json::Error> {
    match frame {
        SessionMuxServerFrame::Stream { stream_id, frame } => Ok(client_sync_frames(frame)
            .into_iter()
            .map(|frame| json!({"type":"stream","streamId":stream_id,"frame":frame}))
            .collect()),
        SessionMuxServerFrame::JournalPageResult { request_id, page } => Ok(vec![json!({
            "type":"journal-page-result","requestId":request_id,"page":{
                "sessionId":page.address.session_id,"throughSequence":page.through_sequence,
                "entries":page.entries,"hasMoreBefore":page.has_more_before}})]),
        SessionMuxServerFrame::Ready { generation, host } => Ok(vec![
            json!({"type":"ready","generation":generation.to_string(),"host":host}),
        ]),
        _ => Ok(vec![serde_json::to_value(frame)?]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use schema::IJsonValue;

    #[test]
    fn control_replacements_keep_empty_arrays_to_clear_client_state() {
        let frame = SessionSyncFrame::ControlUpsert {
            generation: 7,
            control: SessionControlItem {
                session_id: "session".into(),
                queue: vec![],
                jobs: vec![],
                projections: IJsonValue::parse(br#"{}"#).unwrap(),
            },
        };
        let frames = client_sync_frames(&frame);
        assert_eq!(frames.len(), 3);
        assert_eq!(
            frames[0],
            json!({"type":"queue","generation":"7","state":{"sessionId":"session","items":[]}})
        );
        assert_eq!(
            frames[1],
            json!({"type":"jobs","generation":"7","state":{"sessionId":"session","jobs":[]}})
        );
        assert_eq!(
            frames[2],
            json!({"type":"projection","generation":"7","state":{"sessionId":"session","projections":{}}})
        );
    }

    #[test]
    fn actionable_resolution_preserves_zero_revision_and_public_identity() {
        let frames = client_sync_frames(&SessionSyncFrame::ActionableResolved {
            generation: 2,
            id: "approval".into(),
            revision: 0,
        });
        assert_eq!(
            frames,
            vec![
                json!({"type":"resolved","generation":"2","actionableId":"approval","revision":0})
            ]
        );
    }
}
