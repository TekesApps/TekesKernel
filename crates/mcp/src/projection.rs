use std::collections::BTreeSet;

use profile::{
    DynamicTool, DynamicToolCatalog, DynamicToolEffect, DynamicToolSource, DynamicToolSourceKind,
    ExternalEffectBinding, ExternalEffectProtocol,
};
use schema::IJsonValue;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

use crate::McpTool;

pub fn project_name(server: &str, tool: &str) -> Result<String, McpProjectionError> {
    if server.is_empty() || tool.is_empty() {
        return Err(McpProjectionError::EmptyName);
    }
    Ok(format!(
        "mcp__{}__{}",
        escape_component(server),
        escape_component(tool)
    ))
}

pub fn project_catalog(
    server: &str,
    tools: &[McpTool],
    always_on: bool,
) -> Result<DynamicToolCatalog, McpProjectionError> {
    let mut seen_source = BTreeSet::new();
    let mut projected = Vec::with_capacity(tools.len());
    for tool in tools {
        if !seen_source.insert(tool.name.clone()) {
            return Err(McpProjectionError::Duplicate(tool.name.clone()));
        }
        let name = project_name(server, &tool.name)?;
        let schema = IJsonValue::parse(
            &serde_json::to_vec(&json!({
                "name": name,
                "description": tool.description,
                "parameters": serde_json::from_slice::<serde_json::Value>(
                    &tool.input_schema.canonical_bytes().map_err(|error| McpProjectionError::Schema(error.to_string()))?
                ).map_err(|error| McpProjectionError::Schema(error.to_string()))?
            }))
            .map_err(|error| McpProjectionError::Schema(error.to_string()))?,
        )
        .map_err(|error| McpProjectionError::Schema(error.to_string()))?;
        let effect = if tool.annotations.destructive_hint {
            DynamicToolEffect::Destructive
        } else if tool.annotations.read_only_hint {
            DynamicToolEffect::ReadOnly
        } else {
            DynamicToolEffect::ExternalProcess
        };
        let external_effect = external_effect_binding(tool, tools)?;
        let mut dynamic = DynamicTool::declared(
            name,
            DynamicToolSource {
                kind: DynamicToolSourceKind::Mcp,
                id: server.to_owned(),
            },
            effect,
            always_on,
            Vec::new(),
            schema,
        )
        .map_err(|error| McpProjectionError::Schema(format!("tool {}: {error}", tool.name)))?;
        dynamic.external_effect = external_effect;
        projected.push(dynamic);
    }
    DynamicToolCatalog::resolve(projected)
        .map_err(|error| McpProjectionError::Schema(error.to_string()))
}

const EXTERNAL_EFFECT_METADATA_KEY: &str = "io.tekes/externalEffect";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExternalEffectMetadata {
    version: u64,
    #[serde(rename = "reconcileTool")]
    reconcile_tool: String,
}

fn external_effect_binding(
    tool: &McpTool,
    catalog: &[McpTool],
) -> Result<Option<ExternalEffectBinding>, McpProjectionError> {
    let Some(metadata) = &tool.metadata else {
        return Ok(None);
    };
    let raw: serde_json::Value = serde_json::from_slice(
        &metadata
            .canonical_bytes()
            .map_err(|error| McpProjectionError::Schema(error.to_string()))?,
    )
    .map_err(|error| McpProjectionError::Schema(error.to_string()))?;
    let Some(contract) = raw.get(EXTERNAL_EFFECT_METADATA_KEY) else {
        return Ok(None);
    };
    let contract: ExternalEffectMetadata = serde_json::from_value(contract.clone())
        .map_err(|error| McpProjectionError::ExternalEffect(error.to_string()))?;
    if contract.version != 1 || contract.reconcile_tool.is_empty() {
        return Err(McpProjectionError::ExternalEffect(
            "external-effect version must be 1 and reconcileTool must be nonempty".to_owned(),
        ));
    }
    let reconcile = catalog
        .iter()
        .find(|candidate| candidate.name == contract.reconcile_tool)
        .ok_or_else(|| {
            McpProjectionError::ExternalEffect(format!(
                "reconcile tool {} is absent from the same MCP catalog",
                contract.reconcile_tool
            ))
        })?;
    if !reconcile.annotations.read_only_hint || reconcile.annotations.destructive_hint {
        return Err(McpProjectionError::ExternalEffect(format!(
            "reconcile tool {} must be explicitly read-only",
            contract.reconcile_tool
        )));
    }
    validate_reconcile_input_schema(reconcile)?;
    Ok(Some(ExternalEffectBinding {
        protocol: ExternalEffectProtocol::IdempotencyReconcileV1,
        reconcile_tool: contract.reconcile_tool,
    }))
}

fn validate_reconcile_input_schema(tool: &McpTool) -> Result<(), McpProjectionError> {
    let schema: serde_json::Value = serde_json::from_slice(
        &tool
            .input_schema
            .canonical_bytes()
            .map_err(|error| McpProjectionError::Schema(error.to_string()))?,
    )
    .map_err(|error| McpProjectionError::Schema(error.to_string()))?;
    let valid_property = schema
        .get("properties")
        .and_then(|value| value.get("idempotencyKey"))
        .and_then(|value| value.get("type"))
        .and_then(serde_json::Value::as_str)
        == Some("string");
    let required = schema
        .get("required")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|values| {
            values
                .iter()
                .any(|value| value.as_str() == Some("idempotencyKey"))
        });
    if !valid_property || !required {
        return Err(McpProjectionError::ExternalEffect(format!(
            "reconcile tool {} must require string idempotencyKey",
            tool.name
        )));
    }
    Ok(())
}

fn escape_component(value: &str) -> String {
    let mut output = String::new();
    for byte in value.as_bytes() {
        if byte.is_ascii_alphanumeric() || *byte == b'_' {
            output.push(char::from(*byte));
        } else {
            use std::fmt::Write as _;
            let _ = write!(output, "_{byte:02x}");
        }
    }
    output
}

#[derive(Debug, Error)]
pub enum McpProjectionError {
    #[error("MCP server and tool names must be nonempty")]
    EmptyName,
    #[error("MCP source contains duplicate tool {0}")]
    Duplicate(String),
    #[error("MCP schema projection failed: {0}")]
    Schema(String),
    #[error("MCP external-effect contract failed: {0}")]
    ExternalEffect(String),
}
