//! Compilation IR for Dataset DSL generation.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::form::{ProgramFeatureKind, ProgramFeatures};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OntologyRuntimeMode {
    TranscendentalLogic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyImageModelRow {
    pub model_id: String,
    pub label: String,
    pub kind: String,
    pub ontology_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyImageFeatureRow {
    pub feature_id: String,
    pub model_id: Option<String>,
    pub label: String,
    pub kind: String,
    pub ontology_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyImageConstraintRow {
    pub ontology_id: String,
    pub constraint_id: String,
    pub language: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyImageQueryRow {
    pub ontology_id: String,
    pub query_id: String,
    pub language: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyImageProvenanceRow {
    pub source: String,
    pub specification_id: String,
    pub runtime_mode: OntologyRuntimeMode,
    pub substrate: String,
    pub generated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyDataFrameImageTables {
    pub models: Vec<OntologyImageModelRow>,
    pub features: Vec<OntologyImageFeatureRow>,
    pub constraints: Vec<OntologyImageConstraintRow>,
    pub queries: Vec<OntologyImageQueryRow>,
    pub provenance: Vec<OntologyImageProvenanceRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyDataFrameImage {
    pub image_id: String,
    pub engine: String,
    pub tables: OntologyDataFrameImageTables,
}

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

    /// Build a Dataset compilation graph from an Ontology DataFrame Image manifest.
    pub fn from_ontology_image(image: &OntologyDataFrameImage) -> Self {
        let mut compilation = Self::new();
        let root_id = format!("image:{}", sanitize_id_segment(&image.image_id));
        let mut model_node_ids = BTreeMap::new();

        let mut root = DatasetNode::new(
            &root_id,
            format!("{} image", image.image_id),
            DatasetNodeKind::Image,
        )
        .with_meta("image.kind", "ontology-dataframe-image")
        .with_meta("image.engine", image.engine.clone());

        for (index, model) in image.tables.models.iter().enumerate() {
            let node_id = format!(
                "img.model:{}:{}",
                index,
                sanitize_id_segment(&model.model_id)
            );
            let node = DatasetNode::new(&node_id, model.label.clone(), DatasetNodeKind::Model)
                .with_dep(&root_id)
                .with_meta("model.id", model.model_id.clone())
                .with_meta("model.kind", model.kind.clone())
                .with_meta("model.ontology_ids", model.ontology_ids.join(","));

            root = root.with_dep(&node_id);
            compilation.add_node(node);
            model_node_ids.insert(model.model_id.clone(), node_id);
        }

        for (index, feature) in image.tables.features.iter().enumerate() {
            let node_id = format!(
                "img.feature:{}:{}",
                index,
                sanitize_id_segment(&feature.feature_id)
            );
            let mut node =
                DatasetNode::new(&node_id, feature.label.clone(), DatasetNodeKind::Feature)
                    .with_dep(&root_id)
                    .with_meta("feature.id", feature.feature_id.clone())
                    .with_meta("feature.kind", feature.kind.clone())
                    .with_meta("feature.ontology_ids", feature.ontology_ids.join(","));

            if let Some(model_id) = &feature.model_id {
                if let Some(model_node_id) = model_node_ids.get(model_id) {
                    node = node.with_dep(model_node_id.clone());
                }
                node = node.with_meta("feature.model_id", model_id.clone());
            }

            root = root.with_dep(&node_id);
            compilation.add_node(node);
        }

        for (index, constraint) in image.tables.constraints.iter().enumerate() {
            let node_id = format!(
                "img.constraint:{}:{}",
                index,
                sanitize_id_segment(&constraint.constraint_id)
            );
            let node = DatasetNode::new(
                &node_id,
                constraint.constraint_id.clone(),
                DatasetNodeKind::Macro,
            )
            .with_dep(&root_id)
            .with_meta("constraint.ontology_id", constraint.ontology_id.clone())
            .with_meta("constraint.language", constraint.language.clone())
            .with_meta("constraint.text", constraint.text.clone());

            root = root.with_dep(&node_id);
            compilation.add_node(node);
        }

        for (index, query) in image.tables.queries.iter().enumerate() {
            let node_id = format!(
                "img.query:{}:{}",
                index,
                sanitize_id_segment(&query.query_id)
            );
            let node =
                DatasetNode::new(&node_id, query.query_id.clone(), DatasetNodeKind::Function)
                    .with_dep(&root_id)
                    .with_meta("query.ontology_id", query.ontology_id.clone())
                    .with_meta("query.language", query.language.clone())
                    .with_meta("query.text", query.text.clone());

            root = root.with_dep(&node_id);
            compilation.add_node(node);
        }

        for (index, provenance) in image.tables.provenance.iter().enumerate() {
            let node_id = format!(
                "img.provenance:{}:{}",
                index,
                sanitize_id_segment(&provenance.specification_id)
            );
            let node = DatasetNode::new(
                &node_id,
                provenance.specification_id.clone(),
                DatasetNodeKind::Frame,
            )
            .with_dep(&root_id)
            .with_meta("provenance.source", provenance.source.clone())
            .with_meta(
                "provenance.runtime_mode",
                match provenance.runtime_mode {
                    OntologyRuntimeMode::TranscendentalLogic => "transcendental-logic",
                },
            )
            .with_meta("provenance.substrate", provenance.substrate.clone())
            .with_meta(
                "provenance.generated_at_unix_ms",
                provenance.generated_at_unix_ms.to_string(),
            );

            root = root.with_dep(&node_id);
            compilation.add_node(node);
        }

        compilation.add_node(root);
        compilation.add_entrypoint(root_id);
        compilation
    }
}

pub fn ontology_image_from_program_features(features: &ProgramFeatures) -> OntologyDataFrameImage {
    let ontology_ids = vec![features.program_name.clone()];
    let runtime_mode = OntologyRuntimeMode::TranscendentalLogic;

    let models = features
        .selected_forms
        .iter()
        .map(|name| OntologyImageModelRow {
            model_id: name.clone(),
            label: name.clone(),
            kind: "model".to_string(),
            ontology_ids: ontology_ids.clone(),
        })
        .collect::<Vec<_>>();

    let feature_rows = features
        .features
        .iter()
        .enumerate()
        .filter_map(|(index, feature)| {
            let kind = match feature.kind {
                ProgramFeatureKind::ApplicationForm => "application-form",
                ProgramFeatureKind::OperatorPattern => "operator-pattern",
                ProgramFeatureKind::Dependency => "dependency",
                ProgramFeatureKind::Condition => "condition",
                ProgramFeatureKind::SpecificationBinding => "specification-binding",
            };

            Some(OntologyImageFeatureRow {
                feature_id: format!("pf:{}:{}", index, sanitize_id_segment(&feature.value)),
                model_id: None,
                label: feature.value.clone(),
                kind: kind.to_string(),
                ontology_ids: ontology_ids.clone(),
            })
        })
        .collect::<Vec<_>>();

    OntologyDataFrameImage {
        image_id: format!("ontology-image:{}", features.program_name),
        engine: "polars".to_string(),
        tables: OntologyDataFrameImageTables {
            models,
            features: feature_rows,
            constraints: Vec::new(),
            queries: Vec::new(),
            provenance: vec![OntologyImageProvenanceRow {
                source: "gdsl/sdsl".to_string(),
                specification_id: features.program_name.clone(),
                runtime_mode,
                substrate: "dataframe/dataset".to_string(),
                generated_at_unix_ms: 0,
            }],
        },
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::form::{ProgramFeature, ProgramFeatureKind};

    #[test]
    fn ontology_image_converts_to_valid_compilation() {
        let image = OntologyDataFrameImage {
            image_id: "ontology-image:science".to_string(),
            engine: "polars".to_string(),
            tables: OntologyDataFrameImageTables {
                models: vec![OntologyImageModelRow {
                    model_id: "kernel-model".to_string(),
                    label: "Kernel Model".to_string(),
                    kind: "domain-model".to_string(),
                    ontology_ids: vec!["science-ontology".to_string()],
                }],
                features: vec![OntologyImageFeatureRow {
                    feature_id: "signal-feature".to_string(),
                    model_id: Some("kernel-model".to_string()),
                    label: "Signal Feature".to_string(),
                    kind: "analytic-feature".to_string(),
                    ontology_ids: vec!["science-ontology".to_string()],
                }],
                constraints: vec![OntologyImageConstraintRow {
                    ontology_id: "science-ontology".to_string(),
                    constraint_id: "c-signal-shape".to_string(),
                    language: "shacl".to_string(),
                    text: "shape".to_string(),
                }],
                queries: vec![OntologyImageQueryRow {
                    ontology_id: "science-ontology".to_string(),
                    query_id: "q-signal-evidence".to_string(),
                    language: "sparql".to_string(),
                    text: "select".to_string(),
                }],
                provenance: vec![OntologyImageProvenanceRow {
                    source: "gdsl/sdsl".to_string(),
                    specification_id: "spec-1".to_string(),
                    runtime_mode: OntologyRuntimeMode::TranscendentalLogic,
                    substrate: "dataframe/dataset".to_string(),
                    generated_at_unix_ms: 1,
                }],
            },
        };

        let compilation = DatasetCompilation::from_ontology_image(&image);
        assert!(compilation
            .entrypoints
            .iter()
            .any(|entry| entry.starts_with("image:")));
        assert!(compilation.validate().is_ok());
    }

    #[test]
    fn program_features_mirror_to_ontology_image() {
        let features = ProgramFeatures::new(
            "gdsl.analytics".to_string(),
            vec!["centrality".to_string()],
            vec![
                ProgramFeature::new(
                    ProgramFeatureKind::SpecificationBinding,
                    "gdsl.analytics".to_string(),
                    "specification::gdsl.analytics".to_string(),
                ),
                ProgramFeature::new(
                    ProgramFeatureKind::OperatorPattern,
                    "algo.pagerank".to_string(),
                    "operator_pattern::algo.pagerank".to_string(),
                ),
            ],
        );

        let image = ontology_image_from_program_features(&features);
        assert_eq!(image.engine, "polars");
        assert_eq!(image.tables.models.len(), 1);
        assert_eq!(image.tables.features.len(), 2);
        assert_eq!(
            image.tables.provenance[0].runtime_mode,
            OntologyRuntimeMode::TranscendentalLogic
        );
    }
}
