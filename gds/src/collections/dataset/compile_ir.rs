//! Compilation IR for Dataset DSL generation.

use std::collections::{BTreeMap, BTreeSet};

use crate::form::{ProgramFeatureKind, ProgramFeatures};

/// High-level kind classification for LM-first compilation graph nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DatasetNodeKind {
    Image,
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

    pub fn merge(&mut self, other: DatasetCompilation) {
        self.nodes.extend(other.nodes);
        self.entrypoints.extend(other.entrypoints);
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

    /// Build a Dataset compilation graph from Program Features as an executable image.
    ///
    /// This models Unix-style image formation: features are linked under a single image
    /// root entrypoint rather than treated as standalone linked structs.
    pub fn from_program_features(features: &ProgramFeatures) -> Self {
        let mut compilation = Self::new();
        let image_id = format!("image:{}", sanitize_id_segment(&features.program_name));

        let mut image_node = DatasetNode::new(
            &image_id,
            format!("{} image", features.program_name),
            DatasetNodeKind::Image,
        )
        .with_meta("image.kind", "program-feature-image")
        .with_meta("program.name", features.program_name.clone());

        let mut previous_pattern_node: Option<String> = None;

        for (index, feature) in features.features.iter().enumerate() {
            let (prefix, kind) = match feature.kind {
                ProgramFeatureKind::ApplicationForm => ("pf.feature", DatasetNodeKind::Feature),
                ProgramFeatureKind::OperatorPattern => ("pf.expr", DatasetNodeKind::Expr),
                ProgramFeatureKind::Dependency => ("pf.model", DatasetNodeKind::Model),
                ProgramFeatureKind::Condition => ("pf.macro", DatasetNodeKind::Macro),
                ProgramFeatureKind::SpecificationBinding => {
                    ("pf.function", DatasetNodeKind::Function)
                }
            };

            let node_id = format!(
                "{}:{}:{}",
                prefix,
                index,
                sanitize_id_segment(&feature.value)
            );

            let mut node = DatasetNode::new(&node_id, feature.value.clone(), kind)
                .with_meta("program.feature.kind", format!("{:?}", feature.kind))
                .with_meta("program.feature.source", feature.source.clone())
                .with_dep(&image_id);

            if matches!(feature.kind, ProgramFeatureKind::OperatorPattern) {
                if let Some(previous) = &previous_pattern_node {
                    node = node.with_dep(previous);
                }
                previous_pattern_node = Some(node_id.clone());
            }

            image_node = image_node.with_dep(&node_id);
            compilation.add_node(node);
        }

        compilation.add_node(image_node);
        compilation.add_entrypoint(image_id);
        compilation
    }
}

fn sanitize_id_segment(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }

    if out.is_empty() {
        "unnamed".to_string()
    } else {
        out
    }
}
