use std::fmt;

use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Map;
use serde_json::Value;

use crate::SchemaError;

/// Kernel-owned I-JSON value. The third-party representation is private so it
/// cannot become part of a durable or wire API by accident.
#[derive(Clone, PartialEq)]
pub struct IJsonValue(pub(crate) Value);

impl IJsonValue {
    pub fn parse(bytes: &[u8]) -> Result<Self, SchemaError> {
        let mut deserializer = serde_json::Deserializer::from_slice(bytes);
        let value = Self::deserialize(&mut deserializer)?;
        deserializer.end()?;
        Ok(value)
    }

    pub fn parse_str(text: &str) -> Result<Self, SchemaError> {
        Self::parse(text.as_bytes())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, SchemaError> {
        serde_json_canonicalizer::to_vec(&self.0)
            .map_err(|error| SchemaError::Canonical(error.to_string()))
    }

    pub fn canonical_string(&self) -> Result<String, SchemaError> {
        let bytes = self.canonical_bytes()?;
        String::from_utf8(bytes).map_err(|error| SchemaError::Canonical(error.to_string()))
    }

    #[must_use]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub(crate) const fn value(&self) -> &Value {
        &self.0
    }
}

impl fmt::Debug for IJsonValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Serialize for IJsonValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for IJsonValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(IJsonVisitor).map(Self)
    }
}

struct IJsonVisitor;

impl<'de> Visitor<'de> for IJsonVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an I-JSON value with unique object member names")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Number(value.into()))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::Number(value.into()))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        serde_json::Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("I-JSON numbers must be finite"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(Value::String(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element::<IJsonValue>()? {
            values.push(value.0);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if values.contains_key(&key) {
                return Err(serde::de::Error::custom(format!(
                    "duplicate object member {key:?}"
                )));
            }
            let value = map.next_value::<IJsonValue>()?;
            values.insert(key, value.0);
        }
        Ok(Value::Object(values))
    }
}

impl From<bool> for IJsonValue {
    fn from(value: bool) -> Self {
        Self(Value::Bool(value))
    }
}

impl From<String> for IJsonValue {
    fn from(value: String) -> Self {
        Self(Value::String(value))
    }
}

impl From<&str> for IJsonValue {
    fn from(value: &str) -> Self {
        Self(Value::String(value.to_owned()))
    }
}
