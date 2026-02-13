//! Compilation IR for Dataset DSL generation.

use std::collections::{BTreeMap, BTreeSet};

/// High-level kind classification for LM-first compilation graph nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DatasetNodeKind {
    Model,
    Feature,
    Frame,
    Series,
    Expr,
    Function,
    Macro,
}

/// A single node in the dataset compilation graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetNode {
    pub id: String,
    pub name: String,
    pub kind: DatasetNodeKind,
    pub depends_on: BTreeSet<String>,
    pub metadata: BTreeMap<String, String>,
}

impl DatasetNode {
    pub fn new(id: impl Into<String>, name: impl Into<String>, kind: DatasetNodeKind) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
            depends_on: BTreeSet::new(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_dep(mut self, dependency_id: impl Into<String>) -> Self {
        self.depends_on.insert(dependency_id.into());
        self
    }

    pub fn with_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Full compilation payload to drive indexing/codegen.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DatasetCompilation {
    pub nodes: BTreeMap<String, DatasetNode>,
    pub entrypoints: BTreeSet<String>,
}

impl DatasetCompilation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: DatasetNode) -> Option<DatasetNode> {
        self.nodes.insert(node.id.clone(), node)
    }

    pub fn add_entrypoint(&mut self, node_id: impl Into<String>) {
        self.entrypoints.insert(node_id.into());
    }

    pub fn add_dependency(
        &mut self,
        node_id: impl AsRef<str>,
        dependency_id: impl Into<String>,
    ) -> bool {
        if let Some(node) = self.nodes.get_mut(node_id.as_ref()) {
            node.depends_on.insert(dependency_id.into());
            true
        } else {
            false
        }
    }

    pub fn node(&self, node_id: impl AsRef<str>) -> Option<&DatasetNode> {
        self.nodes.get(node_id.as_ref())
    }

    pub fn validate(&self) -> Result<(), String> {
        for (node_id, node) in &self.nodes {
            for dependency_id in &node.depends_on {
                if !self.nodes.contains_key(dependency_id) {
                    return Err(format!(
                        "node '{node_id}' depends on missing node '{dependency_id}'"
                    ));
                }
                if dependency_id == node_id {
                    return Err(format!("node '{node_id}' depends on itself"));
                }
            }
        }

        for entrypoint in &self.entrypoints {
            if !self.nodes.contains_key(entrypoint) {
                return Err(format!(
                    "entrypoint '{entrypoint}' is missing from compilation"
                ));
            }
        }

        Ok(())
    }
}
