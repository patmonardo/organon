//! Logic persistence frame scaffolds.
//!
//! These are row-oriented storage shapes for logic artifacts so database design
//! can proceed before full inferential semantics are finalized.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicFormRow {
    pub logic_form_id: String,
    pub source_id: String,
    pub kind: String,
    pub text: String,
    pub parse_status: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LogicFormFrame {
    rows: Vec<LogicFormRow>,
}

impl LogicFormFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<LogicFormRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: LogicFormRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[LogicFormRow] {
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
pub struct ProofTraceRow {
    pub trace_id: String,
    pub step_index: usize,
    pub rule_id: Option<String>,
    pub premise: String,
    pub conclusion: String,
    pub status: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProofTraceFrame {
    rows: Vec<ProofTraceRow>,
}

impl ProofTraceFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<ProofTraceRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: ProofTraceRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[ProofTraceRow] {
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
pub struct RuleRow {
    pub rule_id: String,
    pub namespace: String,
    pub antecedent: String,
    pub consequent: String,
    pub priority: Option<i32>,
    pub active: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RuleFrame {
    rows: Vec<RuleRow>,
}

impl RuleFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<RuleRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: RuleRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[RuleRow] {
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
    fn logic_form_frame_tracks_rows() {
        let frame = LogicFormFrame::from_rows(vec![LogicFormRow {
            logic_form_id: "lf:1".to_string(),
            source_id: "src:1".to_string(),
            kind: "query".to_string(),
            text: "forall x".to_string(),
            parse_status: "parsed".to_string(),
            error: None,
        }]);
        assert_eq!(frame.len(), 1);
    }

    #[test]
    fn rule_frame_tracks_rows() {
        let mut frame = RuleFrame::new();
        frame.push(RuleRow {
            rule_id: "rule:1".to_string(),
            namespace: "core".to_string(),
            antecedent: "a".to_string(),
            consequent: "b".to_string(),
            priority: Some(1),
            active: true,
        });
        assert!(!frame.is_empty());
    }
}
