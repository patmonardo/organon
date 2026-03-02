//! NLP Semantic Artifacts for a Dataset Language Model Focus.
//!
//! This module introduces explicit NLP structures that map Polars
//! Arrow layouts back into heterogeneous semantic graphs using the
//! `FeatStruct` DSL as an escape hatch for dynamic properties.

use crate::collections::dataset::featstruct::{parse_featstruct, FeatStruct};

/// A node within an SDL Subgraph (e.g., a token, entity, or span).
#[derive(Debug, Clone, PartialEq)]
pub struct SdlNode {
    pub id: u32,
    pub span_start: u32,
    pub span_end: u32,
    pub text: String,
    /// Parsed from the dynamic Arrow string fallback.
    /// Represents the heterogeneous linguistic/semantic attributes.
    pub features: Option<FeatStruct>,
}

/// A directed edge within an SDL Subgraph (e.g., dependency relation, coref).
#[derive(Debug, Clone, PartialEq)]
pub struct SdlEdge {
    pub source: u32, // Points to SdlNode::id
    pub target: u32, // Points to SdlNode::id
    pub relation: String,
    /// Parsed from the dynamic Arrow string fallback.
    /// Represents edge weights, attention scores, or alignment info.
    pub features: Option<FeatStruct>,
}

/// An SDL Subgraph represents a single row in the Dataset.
/// Typically, this corresponds to a full Sentence or Document,
/// enabling Arrow-native adjacency while supporting rich syntax trees.
#[derive(Debug, Clone, PartialEq)]
pub struct SdlSubgraph {
    pub doc_id: u64,
    pub text: String,
    pub nodes: Vec<SdlNode>,
    pub edges: Vec<SdlEdge>,
}

impl SdlSubgraph {
    pub fn new(doc_id: u64, text: impl Into<String>) -> Self {
        Self {
            doc_id,
            text: text.into(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn with_node(
        mut self,
        id: u32,
        span_start: u32,
        span_end: u32,
        text: impl Into<String>,
        features_dsl: Option<&str>,
    ) -> Self {
        let features = features_dsl.and_then(|dsl| parse_featstruct(dsl).ok());
        self.nodes.push(SdlNode {
            id,
            span_start,
            span_end,
            text: text.into(),
            features,
        });
        self
    }

    pub fn with_edge(
        mut self,
        source: u32,
        target: u32,
        relation: impl Into<String>,
        features_dsl: Option<&str>,
    ) -> Self {
        let features = features_dsl.and_then(|dsl| parse_featstruct(dsl).ok());
        self.edges.push(SdlEdge {
            source,
            target,
            relation: relation.into(),
            features,
        });
        self
    }
}

/// Language Model Focus representing the dataset's intention to be treated
/// primarily as an NLP Resource (Corpus / LM dataset).
#[derive(Debug, Clone, Default)]
pub struct LanguageModelFocus {
    /// Indicates whether the underlying DataFrame conforms to the SDL Schema
    /// (doc_id, text, tokens, edges)
    pub is_sdl_compliant: bool,
}

impl LanguageModelFocus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark_sdl_compliant(mut self) -> Self {
        self.is_sdl_compliant = true;
        self
    }
}
