use crate::{McpError, McpTool};
use base64::Engine as _;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
struct Field {
    path: Vec<String>,
    name: String,
    kind: String,
}
#[derive(Clone, Default)]
pub(crate) struct ParameterHeaders(Vec<Field>);

fn invalid() -> McpError {
    McpError::Protocol("invalid MCP parameter header schema or argument".into())
}

impl ParameterHeaders {
    pub(crate) fn catalog(tools: &[McpTool]) -> Result<BTreeMap<String, Self>, McpError> {
        tools
            .iter()
            .map(|tool| {
                let schema = serde_json::to_value(&tool.input_schema).map_err(|_| invalid())?;
                let mut projection = Self::default();
                projection.scan(&schema, Some(Vec::new()), &mut BTreeSet::new(), 0)?;
                Ok((tool.name.clone(), projection))
            })
            .collect()
    }
    fn scan(
        &mut self,
        node: &Value,
        path: Option<Vec<String>>,
        names: &mut BTreeSet<String>,
        depth: usize,
    ) -> Result<(), McpError> {
        if depth > 128 {
            return Err(invalid());
        }
        let Some(object) = node.as_object() else {
            return Ok(());
        };
        if let Some(annotation) = object.get("x-mcp-header") {
            let path = path
                .as_ref()
                .filter(|p| !p.is_empty())
                .ok_or_else(invalid)?;
            let name = annotation
                .as_str()
                .filter(|s| {
                    !s.is_empty()
                        && s.bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&b))
                })
                .ok_or_else(invalid)?;
            if !names.insert(name.to_ascii_lowercase()) {
                return Err(invalid());
            }
            let kind = object
                .get("type")
                .and_then(Value::as_str)
                .filter(|t| matches!(*t, "string" | "integer" | "boolean"))
                .ok_or_else(invalid)?;
            self.0.push(Field {
                path: path.clone(),
                name: format!("mcp-param-{}", name.to_ascii_lowercase()),
                kind: kind.into(),
            });
        }
        if let Some(properties) = object.get("properties").and_then(Value::as_object) {
            for (key, child) in properties {
                let next = path.clone().map(|mut p| {
                    p.push(key.clone());
                    p
                });
                self.scan(child, next, names, depth + 1)?;
            }
        }
        for key in [
            "$defs",
            "additionalProperties",
            "allOf",
            "anyOf",
            "contains",
            "definitions",
            "dependentSchemas",
            "else",
            "if",
            "items",
            "not",
            "oneOf",
            "prefixItems",
            "propertyNames",
            "then",
            "unevaluatedItems",
            "unevaluatedProperties",
        ] {
            if let Some(child) = object.get(key) {
                Self::reject_unreachable(child, depth + 1)?;
            }
        }
        Ok(())
    }
    fn reject_unreachable(node: &Value, depth: usize) -> Result<(), McpError> {
        if depth > 128 {
            return Err(invalid());
        }
        match node {
            Value::Object(object) => {
                if object.contains_key("x-mcp-header") {
                    return Err(invalid());
                }
                for value in object.values() {
                    Self::reject_unreachable(value, depth + 1)?;
                }
            }
            Value::Array(array) => {
                for value in array {
                    Self::reject_unreachable(value, depth + 1)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub(crate) fn project(&self, arguments: &Value) -> Result<Vec<(String, String)>, McpError> {
        self.0
            .iter()
            .filter_map(|field| {
                let value = field
                    .path
                    .iter()
                    .try_fold(arguments, |value, key| value.get(key));
                let value = match value {
                    Some(value) if !value.is_null() => value,
                    _ => return None,
                };
                Some((|| {
                    let text = match field.kind.as_str() {
                        "string" => value.as_str().ok_or_else(invalid)?.to_owned(),
                        "boolean" => value.as_bool().ok_or_else(invalid)?.to_string(),
                        "integer" => {
                            let number = value.as_f64().ok_or_else(invalid)?;
                            if number.fract() != 0.0 || number.abs() > 9_007_199_254_740_991.0 {
                                return Err(invalid());
                            }
                            (number as i64).to_string()
                        }
                        _ => return Err(invalid()),
                    };
                    let encoded = if !text.is_empty()
                        && text.bytes().all(|b| (0x20..=0x7e).contains(&b))
                        && text.trim() == text
                        && !(text.starts_with("=?base64?") && text.ends_with("?="))
                    {
                        text
                    } else {
                        format!(
                            "=?base64?{}?=",
                            base64::engine::general_purpose::STANDARD.encode(text)
                        )
                    };
                    Ok((field.name.clone(), encoded))
                })())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn compile(schema: Value) -> Result<ParameterHeaders, McpError> {
        let mut projection = ParameterHeaders::default();
        projection.scan(&schema, Some(Vec::new()), &mut BTreeSet::new(), 0)?;
        Ok(projection)
    }
    #[test]
    fn schema_rejects_ambiguous_or_unreachable_headers() {
        for schema in [
            json!({"type":"string","x-mcp-header":"root"}),
            json!({"properties":{"a":{"type":"string","x-mcp-header":"X"},"b":{"type":"string","x-mcp-header":"x"}}}),
            json!({"properties":{"a":{"type":"string","x-mcp-header":"bad\r\nname"}}}),
            json!({"properties":{"a":{"type":"number","x-mcp-header":"a"}}}),
            json!({"anyOf":[{"properties":{"a":{"type":"string","x-mcp-header":"a"}}}]}),
        ] {
            assert!(compile(schema).is_err());
        }
    }
    #[test]
    fn header_values_preserve_types_and_encode_unsafe_text() {
        let p = compile(json!({"properties":{"nested":{"properties":{
            "s":{"type":"string","x-mcp-header":"s"},
            "n":{"type":"integer","x-mcp-header":"n"},
            "b":{"type":"boolean","x-mcp-header":"b"}
        }}}}))
        .unwrap();
        let headers: BTreeMap<_, _> = p
            .project(&json!({"nested":{"s":"\r\n","n":7,"b":false}}))
            .unwrap()
            .into_iter()
            .collect();
        assert_eq!(headers["mcp-param-s"], "=?base64?DQo=?=");
        assert_eq!(headers["mcp-param-n"], "7");
        assert_eq!(headers["mcp-param-b"], "false");
        assert_eq!(
            p.project(&json!({"nested":{"s":"发布"}})).unwrap()[0].1,
            "=?base64?5Y+R5biD?="
        );
        for args in [
            json!({"nested":{"n":1.5}}),
            json!({"nested":{"n":9007199254740992u64}}),
            json!({"nested":{"b":"false"}}),
        ] {
            assert!(p.project(&args).is_err());
        }
        assert!(p.project(&json!({"nested":{"s":null}})).unwrap().is_empty());
    }
}
