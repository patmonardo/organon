//! Dataset projection expressions.
//!
//! Describe how dataset-native structures are projected into tabular views.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatasetProjectionKind {
    TextToFrame,
    CorpusToFrame,
    GraphToFrame,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetProjectionExpr {
    kind: DatasetProjectionKind,
    columns: Vec<String>,
}

impl DatasetProjectionExpr {
    pub fn text(columns: Vec<String>) -> Self {
        Self {
            kind: DatasetProjectionKind::TextToFrame,
            columns,
        }
    }

    pub fn corpus(columns: Vec<String>) -> Self {
        Self {
            kind: DatasetProjectionKind::CorpusToFrame,
            columns,
        }
    }

    pub fn graph(columns: Vec<String>) -> Self {
        Self {
            kind: DatasetProjectionKind::GraphToFrame,
            columns,
        }
    }

    pub fn kind(&self) -> &DatasetProjectionKind {
        &self.kind
    }

    pub fn columns(&self) -> &[String] {
        &self.columns
    }
}
