//! Dataset metadata expressions.
//!
//! Declarative metadata updates or requirements that can be applied to datasets.

use serde_json::Value as JsonValue;

#[derive(Debug, Clone, PartialEq)]
pub struct DatasetMetadataExpr {
    key: String,
    value: JsonValue,
}

impl DatasetMetadataExpr {
    pub fn new(key: impl Into<String>, value: impl Into<JsonValue>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &JsonValue {
        &self.value
    }
}
