use std::collections::BTreeSet;

use profile::{
    DynamicTool, DynamicToolCatalog, DynamicToolEffect, DynamicToolSource, DynamicToolSourceKind,
    ExternalEffectBinding, ExternalEffectProtocol, SchemaRefusal, is_dynamic_tool_name,
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

/// Strict projection: the first tool the dynamic launch validator refuses
/// fails the whole catalog. Callers that can launch with a partial catalog use
/// [`project_catalog_partial`].
pub fn project_catalog(
    server: &str,
    tools: &[McpTool],
    always_on: bool,
) -> Result<DynamicToolCatalog, McpProjectionError> {
    let partial = project_catalog_partial(server, tools, always_on)?;
    match partial.refused.into_iter().next() {
        None => Ok(partial.catalog),
        Some(refusal) => Err(refusal.into_error()),
    }
}

/// Longest `reason` kept in a [`McpToolRefusal`]; a diagnostic is a bounded
/// summary, never a copy of the refused schema.
pub const MAX_TOOL_REFUSAL_REASON_BYTES: usize = 512;
/// Longest remote tool name echoed in a [`McpToolRefusal`].
pub const MAX_TOOL_REFUSAL_NAME_BYTES: usize = 256;

/// Stable refusal codes of [`McpToolRefusal::code`].
pub mod refusal_code {
    /// The tool's parameters schema (or description, or name) is outside the
    /// dynamic-tool contract; `pointer` and `keyword` locate it when known.
    pub const SCHEMA_REFUSED: &str = "schema-refused";
    /// The tool's external-effect metadata or its reconcile tool is invalid.
    pub const EXTERNAL_EFFECT_REFUSED: &str = "external-effect-refused";
    /// The tool is projectable itself, but a tool it depends on was refused.
    pub const DEPENDENCY_REFUSED: &str = "dependency-refused";
}

/// One tool that did not project. The other tools of the same server are
/// unaffected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpToolRefusal {
    /// The tool name as the server listed it (bounded).
    pub tool: String,
    /// The projected `mcp__<server>__<tool>` name, at most 128 bytes. Absent
    /// when the tool cannot be projected to a name the launch binding accepts
    /// (empty, longer than 128 bytes after escaping, ASCII control): such a
    /// name could never be an executable tool, so a policy naming it is an
    /// unknown tool, not an unavailable one.
    pub name: Option<String>,
    pub code: &'static str,
    /// JSON pointer of the refused schema node (`/parameters/...`).
    pub pointer: Option<String>,
    /// The offending schema keyword, when the refusal names one.
    pub keyword: Option<String>,
    /// Bounded human sentence without the pointer.
    pub reason: String,
}

impl McpToolRefusal {
    fn into_error(self) -> McpProjectionError {
        let detail = format!(
            "tool {}: {}{}",
            self.tool,
            self.pointer
                .as_deref()
                .map(|pointer| format!("{pointer}: "))
                .unwrap_or_default(),
            self.reason
        );
        if self.code == refusal_code::SCHEMA_REFUSED {
            McpProjectionError::Schema(detail)
        } else {
            McpProjectionError::ExternalEffect(detail)
        }
    }
}

/// Result of [`project_catalog_partial`]: the tools that projected and the
/// tools that did not, in listing order.
#[derive(Clone, Debug)]
pub struct PartialProjection {
    pub catalog: DynamicToolCatalog,
    pub refused: Vec<McpToolRefusal>,
}

/// Projects every tool that satisfies the dynamic launch validator and reports
/// the others.
///
/// Per-tool causes (the tool alone is refused): a parameters schema, description
/// or projected name outside the dynamic-tool contract, a malformed
/// external-effect declaration or reconcile tool, and an effectful tool whose
/// reconcile tool was itself refused. Server-level causes (`Err`): an empty
/// server name, duplicate tool names in one listing (the server's identity for
/// its own tools is ambiguous), and a resolved catalog that still fails
/// validation.
pub fn project_catalog_partial(
    server: &str,
    tools: &[McpTool],
    always_on: bool,
) -> Result<PartialProjection, McpProjectionError> {
    if server.is_empty() {
        return Err(McpProjectionError::EmptyName);
    }
    let mut seen_source = BTreeSet::new();
    for tool in tools {
        if !seen_source.insert(tool.name.clone()) {
            return Err(McpProjectionError::Duplicate(tool.name.clone()));
        }
    }
    let mut projected: Vec<(String, DynamicTool)> = Vec::with_capacity(tools.len());
    // Refusals keyed by the unbounded source name, which the dependency pass
    // below compares against reconcile-tool names.
    let mut refused: Vec<(String, McpToolRefusal)> = Vec::new();
    for tool in tools {
        match project_tool(server, tool, tools, always_on) {
            Ok(dynamic) => projected.push((tool.name.clone(), dynamic)),
            Err(refusal) => refused.push((tool.name.clone(), *refusal)),
        }
    }
    // An effectful tool is only as good as its reconcile tool: when that one
    // was refused, refuse the dependent one too, until nothing changes.
    loop {
        let refused_sources = refused
            .iter()
            .map(|(source, _)| source.clone())
            .collect::<BTreeSet<_>>();
        let Some(position) = projected.iter().position(|(_, dynamic)| {
            dynamic
                .external_effect
                .as_ref()
                .is_some_and(|binding| refused_sources.contains(&binding.reconcile_tool))
        }) else {
            break;
        };
        let (source, dynamic) = projected.remove(position);
        let reconcile = dynamic
            .external_effect
            .as_ref()
            .map(|binding| binding.reconcile_tool.clone())
            .unwrap_or_default();
        refused.push((
            source.clone(),
            McpToolRefusal {
                tool: bounded(&source, MAX_TOOL_REFUSAL_NAME_BYTES),
                name: Some(dynamic.name),
                code: refusal_code::DEPENDENCY_REFUSED,
                pointer: None,
                keyword: None,
                reason: bounded(
                    &format!("reconcile tool {reconcile} was refused"),
                    MAX_TOOL_REFUSAL_REASON_BYTES,
                ),
            },
        ));
    }
    let catalog =
        DynamicToolCatalog::resolve(projected.into_iter().map(|(_, tool)| tool).collect())
            .map_err(|error| McpProjectionError::Schema(error.to_string()))?;
    // Listing order, so a diagnostic list is stable across runs.
    let order = tools
        .iter()
        .enumerate()
        .map(|(index, tool)| (tool.name.as_str(), index))
        .collect::<std::collections::BTreeMap<_, _>>();
    refused.sort_by_key(|(source, _)| order.get(source.as_str()).copied().unwrap_or(usize::MAX));
    let refused = refused.into_iter().map(|(_, refusal)| refusal).collect();
    Ok(PartialProjection { catalog, refused })
}

fn project_tool(
    server: &str,
    tool: &McpTool,
    tools: &[McpTool],
    always_on: bool,
) -> Result<DynamicTool, Box<McpToolRefusal>> {
    let refuse_schema = |reason: String| {
        Box::new(McpToolRefusal {
            tool: bounded(&tool.name, MAX_TOOL_REFUSAL_NAME_BYTES),
            name: launch_name(project_name(server, &tool.name).ok()),
            code: refusal_code::SCHEMA_REFUSED,
            pointer: None,
            keyword: None,
            reason: bounded(&reason, MAX_TOOL_REFUSAL_REASON_BYTES),
        })
    };
    let name =
        project_name(server, &tool.name).map_err(|error| refuse_schema(error.to_string()))?;
    let parameters = tool
        .input_schema
        .canonical_bytes()
        .map_err(|error| error.to_string())
        .and_then(|bytes| {
            serde_json::from_slice::<serde_json::Value>(&bytes).map_err(|error| error.to_string())
        })
        .map_err(refuse_schema)?;
    let schema = serde_json::to_vec(&json!({
        "name": name,
        "description": tool.description,
        "parameters": parameters
    }))
    .map_err(|error| error.to_string())
    .and_then(|bytes| IJsonValue::parse(&bytes).map_err(|error| error.to_string()))
    .map_err(refuse_schema)?;
    let effect = if tool.annotations.destructive_hint {
        DynamicToolEffect::Destructive
    } else if tool.annotations.read_only_hint {
        DynamicToolEffect::ReadOnly
    } else {
        DynamicToolEffect::ExternalProcess
    };
    let external_effect = external_effect_binding(tool, tools).map_err(|error| {
        Box::new(McpToolRefusal {
            tool: bounded(&tool.name, MAX_TOOL_REFUSAL_NAME_BYTES),
            name: launch_name(Some(name.clone())),
            code: refusal_code::EXTERNAL_EFFECT_REFUSED,
            pointer: None,
            keyword: None,
            reason: bounded(&error.to_string(), MAX_TOOL_REFUSAL_REASON_BYTES),
        })
    })?;
    let mut dynamic = DynamicTool::declared(
        name.clone(),
        DynamicToolSource {
            kind: DynamicToolSourceKind::Mcp,
            id: server.to_owned(),
        },
        effect,
        always_on,
        Vec::new(),
        schema,
    )
    .map_err(|error| {
        let parts = SchemaRefusal::from_error(&error);
        Box::new(McpToolRefusal {
            tool: bounded(&tool.name, MAX_TOOL_REFUSAL_NAME_BYTES),
            name: launch_name(Some(name)),
            code: refusal_code::SCHEMA_REFUSED,
            pointer: parts
                .pointer
                .map(|pointer| bounded(&pointer, MAX_TOOL_REFUSAL_REASON_BYTES)),
            keyword: parts
                .keyword
                .map(|keyword| bounded(&keyword, MAX_TOOL_REFUSAL_NAME_BYTES)),
            reason: bounded(&parts.message, MAX_TOOL_REFUSAL_REASON_BYTES),
        })
    })?;
    dynamic.external_effect = external_effect;
    Ok(dynamic)
}

/// A projected name worth reporting: only one the launch rule accepts.
fn launch_name(name: Option<String>) -> Option<String> {
    name.filter(|name| is_dynamic_tool_name(name))
}

/// Truncates at a character boundary to at most `limit` bytes.
fn bounded(value: &str, limit: usize) -> String {
    if value.len() <= limit {
        return value.to_owned();
    }
    let mut end = limit.saturating_sub(3);
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &value[..end])
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
