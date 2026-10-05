//! Production resolution of immutable dynamic launch bindings: the MCP
//! runtime's projected tools joined to the fixed catalog and validated
//! against the frozen configuration and instruction snapshots. Local tools
//! are MCP stdio servers; there is no other executable dynamic source.

use profile::{ConfigSnapshot, DynamicToolCatalog, InstructionSnapshot};

use crate::WorkerLaunchBindings;

/// Resolve the launch bindings of one worker from the frozen snapshots.
/// `tools` are the catalog tools bound by the MCP runtime; they are part of
/// the effective catalog the workspace policy is validated against, so a
/// policy may allow-list an MCP tool.
pub fn resolve_worker_launch_bindings(
    config: &ConfigSnapshot,
    instruction: &InstructionSnapshot,
    goal_id: Option<String>,
    tools: Vec<profile::DynamicTool>,
) -> Result<WorkerLaunchBindings, DynamicBindingError> {
    let dynamic_catalog = DynamicToolCatalog::resolve(tools)
        .map_err(|error| DynamicBindingError::Invalid(error.to_string()))?;
    // Touch the policy meet here so launch assembly is explicitly derived from
    // both frozen snapshots. The worker repeats the same meet when projecting.
    let _effective_policy = instruction.meet_workspace_policy(&config.workspace.policy);
    let bindings = WorkerLaunchBindings {
        goal_id,
        dynamic_catalog,
    };
    profile::LaunchBindings::bind(
        config,
        bindings.goal_id.clone(),
        bindings.dynamic_catalog.clone(),
    )
    .map_err(|error| DynamicBindingError::Invalid(error.to_string()))?;
    Ok(bindings)
}

#[derive(Debug, thiserror::Error)]
pub enum DynamicBindingError {
    #[error("invalid dynamic binding: {0}")]
    Invalid(String),
}

#[cfg(test)]
mod tests {
    use profile::{
        ConfigSnapshot, DynamicTool, DynamicToolEffect, DynamicToolSource, DynamicToolSourceKind,
        EffectiveInstructions, InstructionSnapshot, ProvidersConfig, ResolvedWorkspace,
        RevisionVector, SettingsConfig, WorkspacePolicy,
    };
    use schema::IJsonValue;

    use super::*;

    fn config(allowed_tools: Vec<String>) -> ConfigSnapshot {
        ConfigSnapshot {
            format: 1,
            workspace: ResolvedWorkspace {
                format: 1,
                revision: 1,
                id: "ws".to_owned(),
                name: "ws".to_owned(),
                folder_binding: None,
                selected_cwd: None,
                cwd: vec![
                    std::env::temp_dir()
                        .canonicalize()
                        .expect("canonical temp dir")
                        .to_string_lossy()
                        .into_owned(),
                ],
                policy: WorkspacePolicy {
                    allowed_tools,
                    ..WorkspacePolicy::default()
                },
            },
            providers: ProvidersConfig {
                format: 1,
                revision: 1,
                providers: Vec::new(),
                web_search: None,
            },
            legacy_integrations: (),
            settings: SettingsConfig::default(),
            session_settings: None,
            revisions: RevisionVector {
                workspace: 1,
                providers: 1,
                legacy_integrations: (),
                settings: 0,
                session_settings: None,
            },
        }
    }

    fn instruction() -> InstructionSnapshot {
        InstructionSnapshot {
            format: 1,
            sources: Vec::new(),
            effective: EffectiveInstructions::default(),
        }
    }

    fn mcp_tool(name: &str) -> DynamicTool {
        let schema = IJsonValue::parse(
            &serde_json_canonicalizer::to_vec(&serde_json::json!({
                "name": name,
                "description": "Echo one value",
                "parameters": {"type":"object","properties":{"value":{"type":"string"}},"required":["value"],"additionalProperties":false}
            }))
            .unwrap(),
        )
        .unwrap();
        DynamicTool::declared(
            name,
            DynamicToolSource {
                kind: DynamicToolSourceKind::Mcp,
                id: "fixture".to_owned(),
            },
            DynamicToolEffect::ReadOnly,
            false,
            Vec::new(),
            schema,
        )
        .expect("mcp tool")
    }

    #[test]
    fn mcp_tools_bind_into_one_immutable_catalog_and_allowed_references_fail_closed() {
        let bindings = resolve_worker_launch_bindings(
            &config(vec!["mcp__fixture__echo".to_owned(), "read".to_owned()]),
            &instruction(),
            None,
            vec![mcp_tool("mcp__fixture__echo")],
        )
        .expect("bindings");
        assert_eq!(bindings.dynamic_catalog.tools.len(), 1);
        assert_eq!(bindings.dynamic_catalog.tools[0].name, "mcp__fixture__echo");
        let missing = resolve_worker_launch_bindings(
            &config(vec!["mcp__fixture__missing".to_owned()]),
            &instruction(),
            None,
            vec![mcp_tool("mcp__fixture__echo")],
        );
        assert!(
            missing.is_err(),
            "an allowed reference to an unbound tool fails the launch"
        );
    }

    #[test]
    fn fixed_and_dynamic_name_collisions_fail_launch() {
        let collision = resolve_worker_launch_bindings(
            &config(vec!["read".to_owned()]),
            &instruction(),
            None,
            vec![mcp_tool("read")],
        );
        assert!(collision.is_err());
    }
}
