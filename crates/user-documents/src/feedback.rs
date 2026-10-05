//! Message feedback: `root/feedback/<sessionId>.json`, `{format:1, counter, items:[...]}`.
//! `counter` is the per-session version source; versions are never reused, so a
//! delete followed by a recreate still yields a strictly larger version.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{Failure, closed_object, required_str};

const MAX_MESSAGE_ID_BYTES: usize = 200;
const MAX_NOTE_BYTES: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Feedback {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub rating: String,
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "updatedAtMilliseconds")]
    pub updated_at_milliseconds: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Document {
    format: u32,
    #[serde(default)]
    counter: u64,
    #[serde(default)]
    items: Vec<Feedback>,
}

pub fn path(root: &Path, session_id: &str) -> PathBuf {
    root.join("feedback").join(format!("{session_id}.json"))
}

fn validate_session(value: &str) -> Result<(), String> {
    endpoint::validate_session_id(value)
        .map_err(|_| "sessionId must be a canonical lowercase UUID".to_owned())
}

fn validate_message_id(value: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > MAX_MESSAGE_ID_BYTES {
        return Err(format!(
            "messageId must be 1..={MAX_MESSAGE_ID_BYTES} bytes"
        ));
    }
    Ok(())
}

fn validate_rating(value: &Value) -> Result<(), String> {
    match value.as_str() {
        Some("positive" | "negative") => Ok(()),
        _ => Err("rating must be \"positive\" or \"negative\"".to_owned()),
    }
}

pub fn validate(method: &str, payload: &Value) -> Result<(), String> {
    match method {
        "feedback.list" => {
            let object = closed_object(payload, &["sessionId"])?;
            validate_session(required_str(object, "sessionId")?)
        }
        "feedback.put" => {
            let object = closed_object(
                payload,
                &["sessionId", "messageId", "rating", "ifVersion", "note"],
            )?;
            validate_session(required_str(object, "sessionId")?)?;
            validate_message_id(required_str(object, "messageId")?)?;
            validate_rating(object.get("rating").ok_or("rating is required")?)?;
            match object.get("ifVersion") {
                Some(Value::Null) | Some(Value::String(_)) => {}
                _ => return Err("ifVersion must be a string or null".to_owned()),
            }
            match object.get("note") {
                None | Some(Value::Null) => {}
                Some(Value::String(note)) if note.len() <= MAX_NOTE_BYTES => {}
                Some(Value::String(_)) => {
                    return Err(format!("note must be at most {MAX_NOTE_BYTES} bytes"));
                }
                Some(_) => return Err("note must be a string or null".to_owned()),
            }
            Ok(())
        }
        "feedback.delete" => {
            let object = closed_object(payload, &["sessionId", "messageId", "ifVersion"])?;
            validate_session(required_str(object, "sessionId")?)?;
            validate_message_id(required_str(object, "messageId")?)?;
            required_str(object, "ifVersion")?;
            Ok(())
        }
        other => Err(format!("unknown method {other}")),
    }
}

fn load(root: &Path, session_id: &str) -> Result<Document, Failure> {
    let path = path(root, session_id);
    match std::fs::read(&path) {
        Ok(bytes) => {
            let document: Document = serde_json::from_slice(&bytes).map_err(|error| {
                Failure::new("io", format!("feedback document is corrupt: {error}"))
            })?;
            if document.format != 1 {
                return Err(Failure::new(
                    "io",
                    format!("unsupported feedback format {}", document.format),
                ));
            }
            Ok(document)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Document {
            format: 1,
            ..Default::default()
        }),
        Err(error) => Err(Failure::io("read feedback", &error)),
    }
}

fn save(root: &Path, session_id: &str, document: &Document) -> Result<(), Failure> {
    let value =
        serde_json::to_value(document).map_err(|error| Failure::new("io", error.to_string()))?;
    crate::write_atomic(&path(root, session_id), &crate::canonical_bytes(&value)?)
}

fn conflict(current: Option<&Feedback>) -> Failure {
    Failure::new("version-conflict", "feedback version does not match").with_details(
        json!({ "current": current.map(|item| serde_json::to_value(item).unwrap_or(Value::Null)) }),
    )
}

pub fn list(root: &Path, payload: &Value) -> Result<Value, Failure> {
    let session_id = payload["sessionId"].as_str().unwrap_or_default();
    let document = load(root, session_id)?;
    Ok(json!({ "items": document.items }))
}

pub fn put(root: &Path, payload: &Value) -> Result<Value, Failure> {
    let session_id = payload["sessionId"].as_str().unwrap_or_default();
    let message_id = payload["messageId"].as_str().unwrap_or_default();
    let rating = payload["rating"].as_str().unwrap_or_default().to_owned();
    let mut document = load(root, session_id)?;
    let existing = document
        .items
        .iter()
        .position(|item| item.message_id == message_id);
    let current = existing.map(|index| &document.items[index]);
    match (&payload["ifVersion"], current) {
        (Value::Null, None) => {}
        (Value::String(expected), Some(item)) if *expected == item.version => {}
        _ => return Err(conflict(current)),
    }
    let note = match payload.get("note") {
        None => current.and_then(|item| item.note.clone()),
        Some(Value::String(note)) => Some(note.clone()),
        Some(_) => None,
    };
    document.counter += 1;
    let item = Feedback {
        message_id: message_id.to_owned(),
        rating,
        version: document.counter.to_string(),
        note,
        updated_at_milliseconds: crate::now_milliseconds(),
    };
    match existing {
        Some(index) => document.items[index] = item.clone(),
        None => document.items.push(item.clone()),
    }
    document
        .items
        .sort_by(|a, b| a.message_id.cmp(&b.message_id));
    save(root, session_id, &document)?;
    serde_json::to_value(item).map_err(|error| Failure::new("io", error.to_string()))
}

pub fn delete(root: &Path, payload: &Value) -> Result<Value, Failure> {
    let session_id = payload["sessionId"].as_str().unwrap_or_default();
    let message_id = payload["messageId"].as_str().unwrap_or_default();
    let expected = payload["ifVersion"].as_str().unwrap_or_default();
    let mut document = load(root, session_id)?;
    let Some(index) = document
        .items
        .iter()
        .position(|item| item.message_id == message_id)
    else {
        return Err(conflict(None));
    };
    if document.items[index].version != expected {
        return Err(conflict(Some(&document.items[index])));
    }
    document.items.remove(index);
    save(root, session_id, &document)?;
    Ok(json!({ "deleted": true }))
}
