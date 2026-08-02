//! Stable identities and invocation forms for Shell Components.

mod builtins;
mod pathfinding;
mod plan;

use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt;

pub use builtins::ALGORITHM_BUILTINS;
pub use builtins::PIPELINE_BUILTINS;
pub use pathfinding::ShellBfsCallBuilder;
pub use pathfinding::ShellDijkstraCallBuilder;
pub use plan::ShellComponentCallBuilder;
pub use plan::ShellComponentPlan;
pub use plan::ShellComponentPlanError;

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
    use crate::shell::ShellAddress;
    use crate::shell::ShellAlgebra;
    use crate::shell::ShellPipeline;
    use crate::shell::ShellRegister;

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
    fn typed_pathfinding_calls_preserve_origin_and_order() {
        let origin = ShellAddress::new(
            ShellRegister::Unified,
            ShellPipeline::ModelFeaturePlan,
            ShellAlgebra::ProgramFeature,
        );
        let plan = ShellComponentPlan::new(origin)
            .bfs(7)
            .max_depth(3)
            .track_paths(true)
            .estimate()
            .dijkstra(7)
            .target(8)
            .weight_property("cost")
            .stream();

        assert_eq!(plan.origin(), origin);
        assert_eq!(plan.len(), 2);
        assert_eq!(
            plan.calls()[0].component.as_str(),
            "gds.algorithms.pathfinding.bfs"
        );
        assert_eq!(plan.calls()[0].mode, ShellComponentMode::Estimate);
        assert_eq!(
            plan.calls()[0].inputs.get("maxDepth"),
            Some(&Value::from(3))
        );
        assert_eq!(
            plan.calls()[1].component.as_str(),
            "gds.algorithms.pathfinding.dijkstra"
        );
        assert_eq!(plan.calls()[1].mode, ShellComponentMode::Stream);
        assert_eq!(
            plan.calls()[1].inputs.get("weightProperty"),
            Some(&Value::from("cost"))
        );
    }

    #[test]
    fn generic_component_builder_resolves_and_validates_builtin_calls() {
        let origin = ShellAddress::new(
            ShellRegister::Unified,
            ShellPipeline::ModelFeaturePlan,
            ShellAlgebra::ProgramFeature,
        );
        let plan = ShellComponentPlan::new(origin)
            .component("pagerank", ShellComponentMode::Estimate)
            .unwrap()
            .with_input("maxIterations", 20_u64)
            .with_input("dampingFactor", 0.85)
            .finish();

        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan.calls()[0].component.as_str(),
            "gds.algorithms.centrality.pagerank"
        );
        assert_eq!(plan.calls()[0].mode, ShellComponentMode::Estimate);
        assert_eq!(
            plan.calls()[0].inputs.get("dampingFactor"),
            Some(&Value::from(0.85))
        );

        assert!(matches!(
            ShellComponentPlan::new(origin)
                .component("missing", ShellComponentMode::Estimate),
            Err(ShellComponentPlanError::UnknownComponent(name)) if name == "missing"
        ));
        assert!(matches!(
            ShellComponentPlan::new(origin).component("bfs", ShellComponentMode::Invoke),
            Err(ShellComponentPlanError::UnsupportedMode { .. })
        ));
    }

    #[test]
    fn embedding_components_advertise_only_executable_modes() {
        for alias in ["fast_rp", "node2vec"] {
            let descriptor = builtin_component(alias).unwrap().descriptor();
            assert_eq!(
                descriptor.modes,
                &[ShellComponentMode::Stream, ShellComponentMode::Stats]
            );
        }

        for alias in ["graphsage", "hash_gnn"] {
            let descriptor = builtin_component(alias).unwrap().descriptor();
            assert_eq!(descriptor.modes, &[ShellComponentMode::Stats]);
        }
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
