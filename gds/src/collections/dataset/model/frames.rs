//! Rational dataset frame scaffolds for Model persistence.
//!
//! These are database-oriented row shapes for the Model side of the dataset.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSpecRow {
    pub model_id: String,
    pub kind: String,
    pub input_view: String,
    pub output_view: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModelSpecFrame {
    rows: Vec<ModelSpecRow>,
}

impl ModelSpecFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<ModelSpecRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: ModelSpecRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[ModelSpecRow] {
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
pub struct ModelContextRow {
    pub context_id: String,
    pub model_id: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModelContextFrame {
    rows: Vec<ModelContextRow>,
}

impl ModelContextFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<ModelContextRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: ModelContextRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[ModelContextRow] {
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
pub struct ModelStateRow {
    pub state_id: String,
    pub model_id: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModelStateFrame {
    rows: Vec<ModelStateRow>,
}

impl ModelStateFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<ModelStateRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: ModelStateRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[ModelStateRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_spec_frame_tracks_rows() {
        let frame = ModelSpecFrame::from_rows(vec![ModelSpecRow {
            model_id: "model:parser".to_string(),
            kind: "parser".to_string(),
            input_view: "tokens".to_string(),
            output_view: "parses".to_string(),
            description: Some("test".to_string()),
        }]);
        assert_eq!(frame.len(), 1);
    }
}
