//! Rational dataset frame scaffolds for Plan persistence.
//!
//! These rows capture plan topology and principle metadata as stable storage
//! shapes for schema-first database design.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanRow {
    pub plan_id: String,
    pub name: Option<String>,
    pub source_kind: String,
    pub source_ref: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlanFrame {
    rows: Vec<PlanRow>,
}

impl PlanFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<PlanRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: PlanRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[PlanRow] {
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
pub struct PlanStepRow {
    pub plan_id: String,
    pub step_index: usize,
    pub step_kind: String,
    pub payload: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlanStepFrame {
    rows: Vec<PlanStepRow>,
}

impl PlanStepFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<PlanStepRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: PlanStepRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[PlanStepRow] {
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
pub struct PlanPrincipleRow {
    pub plan_id: String,
    pub triad: String,
    pub mode: String,
    pub observation_count: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlanPrincipleFrame {
    rows: Vec<PlanPrincipleRow>,
}

impl PlanPrincipleFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<PlanPrincipleRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: PlanPrincipleRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[PlanPrincipleRow] {
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
pub struct PlanAnchorRow {
    pub plan_id: String,
    pub anchor_kind: String,
    pub anchor_value: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlanAnchorFrame {
    rows: Vec<PlanAnchorRow>,
}

impl PlanAnchorFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<PlanAnchorRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: PlanAnchorRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[PlanAnchorRow] {
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
    fn plan_frame_tracks_rows() {
        let frame = PlanFrame::from_rows(vec![PlanRow {
            plan_id: "plan:1".to_string(),
            name: Some("derive-tags".to_string()),
            source_kind: "dataset-var".to_string(),
            source_ref: "ds:input".to_string(),
        }]);
        assert_eq!(frame.len(), 1);
    }
}
