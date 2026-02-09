//! Dataset registry expressions.
//!
//! These are lightweight, declarative structs that represent dataset registry
//! references without executing any IO.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetRegistryExpr {
    name: String,
    version: Option<String>,
}

impl DatasetRegistryExpr {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
        }
    }

    pub fn versioned(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: Some(version.into()),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}
