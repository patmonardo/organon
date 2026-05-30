//! Rational dataset frame scaffolds for Feature persistence.
//!
//! These rows model feature definitions and grammar/binding relationships in a
//! database-oriented shape.

use crate::collections::dataset::feature::featstruct::{FeatStruct, FeatValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureSpecRow {
    pub feature_id: String,
    pub role: String,
    pub dtype: String,
    pub property: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FeatureSpecFrame {
    rows: Vec<FeatureSpecRow>,
}

impl FeatureSpecFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<FeatureSpecRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: FeatureSpecRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[FeatureSpecRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureGrammarRow {
    pub grammar_id: String,
    pub feature_id: String,
    pub production: String,
    pub constraint: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FeatureGrammarFrame {
    rows: Vec<FeatureGrammarRow>,
}

impl FeatureGrammarFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<FeatureGrammarRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: FeatureGrammarRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[FeatureGrammarRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelFeatureBindingRow {
    pub model_id: String,
    pub feature_id: String,
    pub relation: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModelFeatureBindingFrame {
    rows: Vec<ModelFeatureBindingRow>,
}

impl ModelFeatureBindingFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<ModelFeatureBindingRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: ModelFeatureBindingRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[ModelFeatureBindingRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatStructNodeRow {
    pub struct_id: String,
    pub node_id: u64,
    pub kind: String,
    pub text_value: Option<String>,
    pub number_value: Option<i64>,
    pub bool_value: Option<bool>,
    pub range_start: Option<usize>,
    pub range_end: Option<usize>,
    pub variable: Option<String>,
    pub reentrance_id: Option<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FeatStructNodeFrame {
    rows: Vec<FeatStructNodeRow>,
}

impl FeatStructNodeFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<FeatStructNodeRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: FeatStructNodeRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[FeatStructNodeRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatStructEdgeRow {
    pub struct_id: String,
    pub parent_node_id: u64,
    pub edge_kind: String,
    pub edge_label: Option<String>,
    pub edge_index: Option<usize>,
    pub child_node_id: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FeatStructEdgeFrame {
    rows: Vec<FeatStructEdgeRow>,
}

impl FeatStructEdgeFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<FeatStructEdgeRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: FeatStructEdgeRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[FeatStructEdgeRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatStructDecomposition {
    pub root_node_id: u64,
    pub nodes: FeatStructNodeFrame,
    pub edges: FeatStructEdgeFrame,
}

impl FeatStructDecomposition {
    pub fn from_featstruct(struct_id: impl Into<String>, value: &FeatStruct) -> Self {
        let struct_id = struct_id.into();
        let mut builder = FeatStructBuilder {
            struct_id,
            next_node_id: 1,
            nodes: Vec::new(),
            edges: Vec::new(),
        };
        let root_node_id = builder.push_struct(value);
        Self {
            root_node_id,
            nodes: FeatStructNodeFrame::from_rows(builder.nodes),
            edges: FeatStructEdgeFrame::from_rows(builder.edges),
        }
    }
}

struct FeatStructBuilder {
    struct_id: String,
    next_node_id: u64,
    nodes: Vec<FeatStructNodeRow>,
    edges: Vec<FeatStructEdgeRow>,
}

impl FeatStructBuilder {
    fn next_id(&mut self) -> u64 {
        let id = self.next_node_id;
        self.next_node_id += 1;
        id
    }

    fn push_struct(&mut self, value: &FeatStruct) -> u64 {
        match value {
            FeatStruct::Dict(dict) => {
                let parent_id = self.push_auto_node(FeatStructNodeRow {
                    struct_id: self.struct_id.clone(),
                    node_id: 0,
                    kind: "dict".to_string(),
                    text_value: None,
                    number_value: None,
                    bool_value: None,
                    range_start: None,
                    range_end: None,
                    variable: None,
                    reentrance_id: None,
                });
                for (key, child_value) in dict {
                    let child_id = self.push_value(child_value);
                    self.edges.push(FeatStructEdgeRow {
                        struct_id: self.struct_id.clone(),
                        parent_node_id: parent_id,
                        edge_kind: "key".to_string(),
                        edge_label: Some(key.clone()),
                        edge_index: None,
                        child_node_id: child_id,
                    });
                }
                parent_id
            }
            FeatStruct::List(values) => {
                let parent_id = self.push_auto_node(FeatStructNodeRow {
                    struct_id: self.struct_id.clone(),
                    node_id: 0,
                    kind: "list".to_string(),
                    text_value: None,
                    number_value: None,
                    bool_value: None,
                    range_start: None,
                    range_end: None,
                    variable: None,
                    reentrance_id: None,
                });
                for (index, child_value) in values.iter().enumerate() {
                    let child_id = self.push_value(child_value);
                    self.edges.push(FeatStructEdgeRow {
                        struct_id: self.struct_id.clone(),
                        parent_node_id: parent_id,
                        edge_kind: "index".to_string(),
                        edge_label: None,
                        edge_index: Some(index),
                        child_node_id: child_id,
                    });
                }
                parent_id
            }
        }
    }

    fn push_value(&mut self, value: &FeatValue) -> u64 {
        match value {
            FeatValue::Text(v) => self.push_auto_node(FeatStructNodeRow {
                struct_id: self.struct_id.clone(),
                node_id: 0,
                kind: "text".to_string(),
                text_value: Some(v.clone()),
                number_value: None,
                bool_value: None,
                range_start: None,
                range_end: None,
                variable: None,
                reentrance_id: None,
            }),
            FeatValue::Number(v) => self.push_auto_node(FeatStructNodeRow {
                struct_id: self.struct_id.clone(),
                node_id: 0,
                kind: "number".to_string(),
                text_value: None,
                number_value: Some(*v),
                bool_value: None,
                range_start: None,
                range_end: None,
                variable: None,
                reentrance_id: None,
            }),
            FeatValue::Bool(v) => self.push_auto_node(FeatStructNodeRow {
                struct_id: self.struct_id.clone(),
                node_id: 0,
                kind: "bool".to_string(),
                text_value: None,
                number_value: None,
                bool_value: Some(*v),
                range_start: None,
                range_end: None,
                variable: None,
                reentrance_id: None,
            }),
            FeatValue::Null => self.push_auto_node(FeatStructNodeRow {
                struct_id: self.struct_id.clone(),
                node_id: 0,
                kind: "null".to_string(),
                text_value: None,
                number_value: None,
                bool_value: None,
                range_start: None,
                range_end: None,
                variable: None,
                reentrance_id: None,
            }),
            FeatValue::Empty => self.push_auto_node(FeatStructNodeRow {
                struct_id: self.struct_id.clone(),
                node_id: 0,
                kind: "empty".to_string(),
                text_value: None,
                number_value: None,
                bool_value: None,
                range_start: None,
                range_end: None,
                variable: None,
                reentrance_id: None,
            }),
            FeatValue::Variable(name) => self.push_auto_node(FeatStructNodeRow {
                struct_id: self.struct_id.clone(),
                node_id: 0,
                kind: "variable".to_string(),
                text_value: None,
                number_value: None,
                bool_value: None,
                range_start: None,
                range_end: None,
                variable: Some(name.clone()),
                reentrance_id: None,
            }),
            FeatValue::BytesRange { start, end } => self.push_auto_node(FeatStructNodeRow {
                struct_id: self.struct_id.clone(),
                node_id: 0,
                kind: "bytes-range".to_string(),
                text_value: None,
                number_value: None,
                bool_value: None,
                range_start: Some(*start),
                range_end: Some(*end),
                variable: None,
                reentrance_id: None,
            }),
            FeatValue::Reentrant(id) => self.push_auto_node(FeatStructNodeRow {
                struct_id: self.struct_id.clone(),
                node_id: 0,
                kind: "reentrant".to_string(),
                text_value: None,
                number_value: None,
                bool_value: None,
                range_start: None,
                range_end: None,
                variable: None,
                reentrance_id: Some(id.0),
            }),
            FeatValue::ReentranceDef { id, value } => {
                let parent_id = self.push_auto_node(FeatStructNodeRow {
                    struct_id: self.struct_id.clone(),
                    node_id: 0,
                    kind: "reentrance-def".to_string(),
                    text_value: None,
                    number_value: None,
                    bool_value: None,
                    range_start: None,
                    range_end: None,
                    variable: None,
                    reentrance_id: Some(id.0),
                });
                let child_id = self.push_value(value);
                self.edges.push(FeatStructEdgeRow {
                    struct_id: self.struct_id.clone(),
                    parent_node_id: parent_id,
                    edge_kind: "value".to_string(),
                    edge_label: None,
                    edge_index: None,
                    child_node_id: child_id,
                });
                parent_id
            }
            FeatValue::Struct(inner) => self.push_struct(inner),
            FeatValue::List(values) => {
                let parent_id = self.push_auto_node(FeatStructNodeRow {
                    struct_id: self.struct_id.clone(),
                    node_id: 0,
                    kind: "value-list".to_string(),
                    text_value: None,
                    number_value: None,
                    bool_value: None,
                    range_start: None,
                    range_end: None,
                    variable: None,
                    reentrance_id: None,
                });
                for (index, child_value) in values.iter().enumerate() {
                    let child_id = self.push_value(child_value);
                    self.edges.push(FeatStructEdgeRow {
                        struct_id: self.struct_id.clone(),
                        parent_node_id: parent_id,
                        edge_kind: "index".to_string(),
                        edge_label: None,
                        edge_index: Some(index),
                        child_node_id: child_id,
                    });
                }
                parent_id
            }
        }
    }

    fn push_auto_node(&mut self, mut row: FeatStructNodeRow) -> u64 {
        row.node_id = self.next_id();
        self.push_node(row)
    }

    fn push_node(&mut self, row: FeatStructNodeRow) -> u64 {
        let node_id = row.node_id;
        self.nodes.push(row);
        node_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::feature::featstruct::{
        FeatDict, FeatReentranceId, FeatStruct, FeatValue,
    };

    #[test]
    fn feature_spec_frame_tracks_rows() {
        let mut frame = FeatureSpecFrame::new();
        frame.push(FeatureSpecRow {
            feature_id: "feature:pos".to_string(),
            role: "projection".to_string(),
            dtype: "string".to_string(),
            property: Some("tag".to_string()),
            path: Some("offsets:[0]".to_string()),
        });
        assert_eq!(frame.len(), 1);
    }

    #[test]
    fn featstruct_decomposition_keeps_structure() {
        let mut agreement = FeatDict::new();
        agreement.insert("num".to_string(), FeatValue::Text("sg".to_string()));

        let mut root = FeatDict::new();
        root.insert("pos".to_string(), FeatValue::Text("noun".to_string()));
        root.insert(
            "agreement".to_string(),
            FeatValue::Struct(FeatStruct::Dict(agreement)),
        );
        root.insert(
            "coref".to_string(),
            FeatValue::ReentranceDef {
                id: FeatReentranceId(1),
                value: Box::new(FeatValue::Text("x".to_string())),
            },
        );

        let decomposition =
            FeatStructDecomposition::from_featstruct("featstruct:1", &FeatStruct::Dict(root));

        assert!(!decomposition.nodes.is_empty());
        assert!(!decomposition.edges.is_empty());
        assert!(decomposition
            .nodes
            .rows()
            .iter()
            .any(|node| node.kind == "reentrance-def"));
    }
}
