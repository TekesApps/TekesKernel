use std::fmt;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use crate::IJsonValue;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginTuple {
    pub principal: String,
    pub client: String,
    pub target: String,
    pub op: String,
    pub key: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SeqRange {
    pub from: u64,
    pub to: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Block {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "reasoning")]
    Reasoning { text: String },
    #[serde(rename = "image")]
    Image {
        asset: String,
        mime: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "file")]
    File {
        asset: String,
        mime: String,
        name: String,
        bytes: u64,
    },
    #[serde(rename = "tool-call")]
    ToolCall {
        call: String,
        name: String,
        args: IJsonValue,
    },
    #[serde(rename = "tool-result")]
    ToolResult {
        call: String,
        content: Vec<Block>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        error: Option<bool>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResumePolicy {
    Never,
    Bounded(u64),
}

impl ResumePolicy {
    #[must_use]
    pub const fn permits(self, recovery_ordinal: u64) -> bool {
        match self {
            Self::Never => false,
            Self::Bounded(limit) => recovery_ordinal < limit,
        }
    }
}

impl Serialize for ResumePolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Never => serializer.serialize_str("never"),
            Self::Bounded(limit) => {
                #[derive(Serialize)]
                struct Bounded {
                    bounded: u64,
                }
                Bounded { bounded: *limit }.serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for ResumePolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::String(value) if value == "never" => Ok(Self::Never),
            Value::Object(value) if value.len() == 1 => value
                .get("bounded")
                .and_then(Value::as_u64)
                .map(Self::Bounded)
                .ok_or_else(|| D::Error::custom("bounded resume must contain an unsigned integer")),
            _ => Err(D::Error::custom(
                "resume must be \"never\" or {\"bounded\":n}",
            )),
        }
    }
}

impl fmt::Display for ResumePolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Never => formatter.write_str("never"),
            Self::Bounded(limit) => write!(formatter, "bounded({limit})"),
        }
    }
}
