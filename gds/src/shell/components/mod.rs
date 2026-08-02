//! Stable identities and invocation forms for Shell Components.

mod builtins;

use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt;

pub use builtins::ALGORITHM_BUILTINS;
pub use builtins::PIPELINE_BUILTINS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct ShellComponentId(&'static str);

impl ShellComponentId {
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for ShellComponentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellComponentCategory {
    Pathfinding,
    Centrality,
    Community,
    Similarity,
    Embeddings,
    Miscellaneous,
    Pipeline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellComponentMode {
    Invoke,
    Stream,
    Stats,
    Estimate,
    Mutate,
    Write,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellComponentDescriptor {
    pub id: ShellComponentId,
    pub alias: &'static str,
    pub category: ShellComponentCategory,
    pub modes: &'static [ShellComponentMode],
}

impl ShellComponentDescriptor {
    pub const fn new(
        id: &'static str,
        alias: &'static str,
        category: ShellComponentCategory,
        modes: &'static [ShellComponentMode],
    ) -> Self {
        Self {
            id: ShellComponentId::new(id),
            alias,
            category,
            modes,
        }
    }

    pub fn supports(self, mode: ShellComponentMode) -> bool {
        self.modes.contains(&mode)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinComponentRef(&'static ShellComponentDescriptor);

impl BuiltinComponentRef {
    pub const fn descriptor(self) -> &'static ShellComponentDescriptor {
        self.0
    }

    pub fn call(self, mode: ShellComponentMode) -> ShellComponentCall {
        ShellComponentCall {
            component: self.0.id,
            mode,
            inputs: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellComponentCall {
    pub component: ShellComponentId,
    pub mode: ShellComponentMode,
    pub inputs: BTreeMap<String, Value>,
}

impl ShellComponentCall {
    pub fn with_input(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.inputs.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BuiltinComponentSuite {
    entries: &'static [ShellComponentDescriptor],
}

impl BuiltinComponentSuite {
    pub const fn algorithms() -> Self {
        Self {
            entries: ALGORITHM_BUILTINS,
        }
    }

    pub const fn pipelines() -> Self {
        Self {
            entries: PIPELINE_BUILTINS,
        }
    }

    pub const fn all(self) -> &'static [ShellComponentDescriptor] {
        self.entries
    }

    pub fn find(self, name: &str) -> Option<BuiltinComponentRef> {
        self.entries
            .iter()
            .find(|entry| entry.id.as_str() == name || entry.alias == name)
            .map(BuiltinComponentRef)
    }

    pub fn for_category(
        self,
        category: ShellComponentCategory,
    ) -> impl Iterator<Item = &'static ShellComponentDescriptor> {
        self.entries
            .iter()
            .filter(move |entry| entry.category == category)
    }

    pub fn validate(self) -> Result<(), ShellComponentSuiteError> {
        let mut ids = HashSet::new();
        let mut aliases = HashSet::new();

        for entry in self.entries {
            if entry.modes.is_empty() {
                return Err(ShellComponentSuiteError::NoModes(entry.id));
            }
            if !ids.insert(entry.id) {
                return Err(ShellComponentSuiteError::DuplicateId(entry.id));
            }
            if !aliases.insert(entry.alias) {
                return Err(ShellComponentSuiteError::DuplicateAlias(entry.alias));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ShellComponentSuiteError {
    #[error("duplicate Shell Component ID: {0}")]
    DuplicateId(ShellComponentId),
    #[error("duplicate Shell Component alias: {0}")]
    DuplicateAlias(&'static str),
    #[error("Shell Component has no supported modes: {0}")]
    NoModes(ShellComponentId),
}

pub fn builtin_component(name: &str) -> Option<BuiltinComponentRef> {
    BuiltinComponentSuite::algorithms()
        .find(name)
        .or_else(|| BuiltinComponentSuite::pipelines().find(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_suite_has_unique_valid_entries() {
        let suite = BuiltinComponentSuite::algorithms();
        assert_eq!(suite.all().len(), 47);
        assert_eq!(suite.validate(), Ok(()));
    }

    #[test]
    fn canonical_and_legacy_names_resolve_to_one_component() {
        let canonical = builtin_component("gds.algorithms.pathfinding.dijkstra").unwrap();
        let legacy = builtin_component("dijkstra").unwrap();

        assert_eq!(canonical, legacy);
        assert!(canonical
            .descriptor()
            .supports(ShellComponentMode::Estimate));
    }

    #[test]
    fn component_calls_are_inspectable_plan_values() {
        let call = builtin_component("bfs")
            .unwrap()
            .call(ShellComponentMode::Stream)
            .with_input("source", 7_u64);

        assert_eq!(call.component.as_str(), "gds.algorithms.pathfinding.bfs");
        assert_eq!(call.inputs.get("source"), Some(&Value::from(7_u64)));
    }

    #[test]
    fn pipeline_suite_has_unique_valid_entries() {
        let suite = BuiltinComponentSuite::pipelines();
        assert_eq!(suite.all().len(), 6);
        assert_eq!(suite.validate(), Ok(()));

        let create = builtin_component("gds.pipelines.node_classification.create").unwrap();
        assert_eq!(
            create.descriptor().alias,
            "create_node_classification_pipeline"
        );
        assert!(create.descriptor().supports(ShellComponentMode::Invoke));
    }
}
