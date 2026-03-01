//! NLP Semantic Artifacts for a Dataset Language Model Focus.
//!
//! This module introduces explicit NLP structures (XML, NLP Trees, Word Networks)
//! that allow a generic DataFrame-backed Dataset to act as an LM Language Model dataset.

use crate::collections::dataset::tree::TreeCollection;
use std::collections::BTreeMap;

/// Defines a localized Word Network (e.g. synonym rings, semantic dependency graph).
/// This serves as a placeholder for graph/link network algorithms relating words to each other.
#[derive(Debug, Clone, PartialEq)]
pub struct WordNetwork {
    pub name: String,
    // Nodes could eventually be token indices or vocabulary IDs
    pub nodes: Vec<String>,
    // Edges defined as (source_node_idx, target_node_idx, relationship_type)
    pub edges: Vec<(usize, usize, String)>,
}

impl WordNetwork {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

/// A collection of Semantic Artifacts that support the LM Focus of a Dataset.
#[derive(Debug, Clone, Default)]
pub struct SemanticArtifacts {
    pub xml_content: Option<String>,
    pub nlp_trees: Option<TreeCollection>,
    pub word_networks: BTreeMap<String, WordNetwork>,
}

impl SemanticArtifacts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_xml(mut self, xml: impl Into<String>) -> Self {
        self.xml_content = Some(xml.into());
        self
    }

    pub fn with_trees(mut self, trees: TreeCollection) -> Self {
        self.nlp_trees = Some(trees);
        self
    }

    pub fn add_word_network(mut self, network: WordNetwork) -> Self {
        self.word_networks.insert(network.name.clone(), network);
        self
    }
}

/// Language Model Focus representing the dataset's intention to be treated
/// primarily as an NLP Resource (Corpus / LM dataset), enriching the tabular
/// DataFrame backing with semantic structural artifacts.
#[derive(Debug, Clone, Default)]
pub struct LanguageModelFocus {
    pub artifacts: SemanticArtifacts,
}

impl LanguageModelFocus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_artifacts(mut self, artifacts: SemanticArtifacts) -> Self {
        self.artifacts = artifacts;
        self
    }

    pub fn artifacts(&self) -> &SemanticArtifacts {
        &self.artifacts
    }

    pub fn artifacts_mut(&mut self) -> &mut SemanticArtifacts {
        &mut self.artifacts
    }
}
