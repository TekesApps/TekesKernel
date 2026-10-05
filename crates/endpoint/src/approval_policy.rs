//! Approval choices the Kernel offers to a Client.
//!
//! Every session runs under one durable per-session permission mode
//! (`threads/<sessionId>/permission-mode.json`, owned by the `engine`
//! crate's `PermissionMode`). The wire policy publishes the three modes as
//! thread-level options in menu order; there is no Server-level value, so
//! `serverLevel` carries no options and a null current value. The ids are
//! the ones the Tekes composer already maps for other hosts.

use serde::Serialize;

pub const PERMISSION_MODE_READ_ONLY: &str = "read-only";
pub const PERMISSION_MODE_WORKSPACE_WRITE: &str = "workspace-write";
pub const PERMISSION_MODE_DANGER_FULL_ACCESS: &str = "danger-full-access";

/// The thread-level ids in menu order. `engine::PermissionMode::ALL` is the
/// authority; the supervisor test pins the two lists to each other.
pub const PERMISSION_MODE_IDS: [&str; 3] = [
    PERMISSION_MODE_READ_ONLY,
    PERMISSION_MODE_WORKSPACE_WRITE,
    PERMISSION_MODE_DANGER_FULL_ACCESS,
];

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalOption {
    pub value: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerApprovalState {
    pub options: Vec<ApprovalOption>,
    pub current_value: Option<&'static str>,
}

/// Wire policy: the three per-session modes at thread level, nothing at
/// Server level.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalPolicy {
    pub thread_level: Vec<ApprovalOption>,
    pub server_level: ServerApprovalState,
}

impl Default for ApprovalPolicy {
    fn default() -> Self {
        Self {
            thread_level: vec![
                ApprovalOption {
                    value: PERMISSION_MODE_READ_ONLY,
                    title: "Read Only",
                    description: "Read-only tools and questions run directly. Every file edit, \
                                  command, and other effect is refused.",
                },
                ApprovalOption {
                    value: PERMISSION_MODE_WORKSPACE_WRITE,
                    title: "Workspace Write",
                    description: "Workspace edits and sandboxed commands run directly. \
                                  Destructive actions wait for your approval before they run.",
                },
                ApprovalOption {
                    value: PERMISSION_MODE_DANGER_FULL_ACCESS,
                    title: "Full Access",
                    description: "Every tool call runs without asking, including destructive \
                                  actions.",
                },
            ],
            server_level: ServerApprovalState {
                options: Vec::new(),
                current_value: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_policy_declares_three_thread_modes_and_no_server_value() {
        let value = serde_json::to_value(ApprovalPolicy::default()).expect("serializable");
        let thread = value["threadLevel"].as_array().expect("thread options");
        assert_eq!(
            thread
                .iter()
                .map(|option| option["value"].as_str().expect("value"))
                .collect::<Vec<_>>(),
            PERMISSION_MODE_IDS
        );
        assert_eq!(
            thread
                .iter()
                .map(|option| option["title"].as_str().expect("title"))
                .collect::<Vec<_>>(),
            ["Read Only", "Workspace Write", "Full Access"]
        );
        assert!(thread.iter().all(|option| {
            option["description"]
                .as_str()
                .is_some_and(|text| !text.is_empty())
        }));
        assert_eq!(value["serverLevel"]["options"], serde_json::json!([]));
        assert_eq!(
            value["serverLevel"]["currentValue"],
            serde_json::Value::Null
        );
    }
}
