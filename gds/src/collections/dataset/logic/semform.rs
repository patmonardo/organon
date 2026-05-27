//! Semantic forms received by the SemDataset fold.

use crate::form::program::{ProgramFeature, ProgramFeatureKind};
use crate::ml::nlp::sem::logic::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemForm {
    pub kind: ProgramFeatureKind,
    pub text: String,
    pub source: String,
    pub expr: Option<Expression>,
    pub error: Option<String>,
}

impl SemForm {
    pub fn new(
        kind: ProgramFeatureKind,
        text: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            text: text.into(),
            source: source.into(),
            expr: None,
            error: None,
        }
    }

    pub fn parsed(&self) -> bool {
        self.expr.is_some() && self.error.is_none()
    }
}

impl From<ProgramFeature> for SemForm {
    fn from(feature: ProgramFeature) -> Self {
        Self::new(feature.kind, feature.value, feature.source)
    }
}
