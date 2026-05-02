//! `Valuation` — partial valuation against a Feature schema (R4 access object).
//!
//! See `gds/doc/SEMANTIC-DATASET.md` (Five-Fold Synthesis), Root
//! Object **R4** ("`Model` — a partial valuation: a row, a region, or a
//! sub-frame considered as satisfying a Feature schema"). NLTK Ch9 / Ch10
//! are the reference texts; the DRS, the FeatStruct, and the Valuation are
//! all R4-Models.
//!
//! ## Why this module exists alongside [`crate::collections::dataset::model`]
//!
//! Two different senses of "Model" live in this crate:
//!
//! 1. **Compute Model** — the existing [`crate::collections::dataset::model`]
//!    `Model` trait (Tagger / Parser / Segmenter / …). A *transform* that
//!    consumes a `Dataset` and produces a `ModelDelta`.
//! 2. **R4 Model = Valuation** — *this* module. A *partial map* from
//!    Feature names to values, considered as satisfying a Feature schema.
//!    Has no `apply` method; carries data, not behavior.
//!
//! A compute Model produces and consumes Valuations. They are not in
//! conflict; they name different things. Future passes may rename the
//! compute trait to disambiguate, but the Valuation type sits at the R4
//! address regardless.
//!
//! ## Position in the four-fold
//!
//! Valuation is the **intensional** counterpart of a Document. A Document
//! lives on the extensional/evidentiary side and carries a distinguished
//! `source : Source` Feature; a Valuation lives on the intensional side and
//! carries any Feature schema you point it at. They meet through one
//! Feature read two ways: the Document side calls it `source : Source`, the
//! Valuation side calls it `evidence : Document`. See doctrine §"The
//! Document / Model meeting-point".
//!
//! Status: **Phase 4**. Row + frame access objects, schema pairing, and
//! the meeting-point bridge `DocumentFrame ↔ ValuationFrame`. Lattice
//! operations (`unify`, `subsumes`, `extend`) are deliberately deferred —
//! their signatures will be the next architectural conversation.

use std::collections::BTreeMap;

use polars::prelude::PolarsError;

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::document::{DocumentFrame, Span};
use crate::collections::dataset::feature::role::{
    FeatureDType, FeatureDescriptor, FeatureName, FeatureRole,
};
use crate::collections::dataset::source::ContentHash;

/// Errors raised when constructing or pairing Valuations with schemas.
#[derive(Debug)]
pub enum ValuationError {
    /// The paired `GDSDataFrame` is missing a column declared by the schema.
    SchemaColumnMissing(FeatureName),
    /// A `ValuationFrame` was asked for `evidence()` but its schema does not
    /// declare a `source : Source` Feature.
    NotEvidentiary,
    /// A schema-construction or polars-shape problem.
    Polars(PolarsError),
}

impl From<PolarsError> for ValuationError {
    fn from(e: PolarsError) -> Self {
        Self::Polars(e)
    }
}

impl std::fmt::Display for ValuationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaColumnMissing(name) => {
                write!(f, "schema column missing in frame: {}", name.as_str())
            }
            Self::NotEvidentiary => write!(
                f,
                "valuation is not evidentiary (no `source : Source` feature)"
            ),
            Self::Polars(e) => write!(f, "polars error: {e}"),
        }
    }
}

impl std::error::Error for ValuationError {}

/// One cell of a Valuation. Mirrors [`FeatureDType`] one-for-one (modulo
/// the `Custom` escape hatch). Carries values, not Polars dtype mappings.
#[derive(Debug, Clone, PartialEq)]
pub enum ValuationCell {
    Null,
    String(String),
    Int64(i64),
    Float64(f64),
    Bool(bool),
    Span(Span),
    /// Reference to a `Source` by content hash.
    Source(ContentHash),
    /// Reference to a `Document` by its source content hash.
    Document(ContentHash),
    /// A nested Valuation.
    Nested(Box<Valuation>),
    /// Opaque value carried as a string for now.
    Custom(String),
}

impl ValuationCell {
    /// Stable string tag for the cell kind, parallel to [`FeatureDType::name`].
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::String(_) => "string",
            Self::Int64(_) => "int64",
            Self::Float64(_) => "float64",
            Self::Bool(_) => "bool",
            Self::Span(_) => "span",
            Self::Source(_) => "source",
            Self::Document(_) => "document",
            Self::Nested(_) => "model",
            Self::Custom(_) => "custom",
        }
    }
}

/// A single-row Valuation: a partial map from `FeatureName` to
/// [`ValuationCell`]. "Partial" is essential — features may be unbound
/// (Binder role) or simply not yet observed, and that absence is itself
/// information for the lattice ops that come later.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Valuation {
    cells: BTreeMap<FeatureName, ValuationCell>,
}

impl Valuation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, name: impl Into<FeatureName>, cell: ValuationCell) -> Self {
        self.cells.insert(name.into(), cell);
        self
    }

    pub fn set(&mut self, name: impl Into<FeatureName>, cell: ValuationCell) {
        self.cells.insert(name.into(), cell);
    }

    pub fn get(&self, name: &FeatureName) -> Option<&ValuationCell> {
        self.cells.get(name)
    }

    pub fn contains(&self, name: &FeatureName) -> bool {
        self.cells.contains_key(name)
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&FeatureName, &ValuationCell)> {
        self.cells.iter()
    }
}

/// Frame-level access object: a `GDSDataFrame` paired with a Feature
/// schema. Each row of the frame is read as a Valuation against the
/// schema. The frame's columns must include every schema feature; extra
/// columns are tolerated and ignored.
///
/// The schema is held as `Vec<FeatureDescriptor>` so iteration is cheap;
/// if you have a [`crate::collections::dataset::feature::role::FeatureFrame`]
/// you can hand its descriptors in via [`Self::with_schema_descriptors`].
#[derive(Debug, Clone)]
pub struct ValuationFrame {
    df: GDSDataFrame,
    schema: Vec<FeatureDescriptor>,
}

impl ValuationFrame {
    /// Build from a `GDSDataFrame` and an explicit schema, validating that
    /// every schema feature has a column in the frame.
    pub fn with_schema_descriptors(
        df: GDSDataFrame,
        schema: Vec<FeatureDescriptor>,
    ) -> Result<Self, ValuationError> {
        let names: std::collections::HashSet<String> = df.column_names().into_iter().collect();
        for d in &schema {
            if !names.contains(d.name.as_str()) {
                return Err(ValuationError::SchemaColumnMissing(d.name.clone()));
            }
        }
        Ok(Self { df, schema })
    }

    pub fn dataframe(&self) -> &GDSDataFrame {
        &self.df
    }

    pub fn into_dataframe(self) -> GDSDataFrame {
        self.df
    }

    pub fn schema(&self) -> &[FeatureDescriptor] {
        &self.schema
    }

    pub fn len(&self) -> usize {
        self.df.height()
    }

    pub fn is_empty(&self) -> bool {
        self.df.height() == 0
    }

    /// True iff the schema declares a Feature whose `dtype` is
    /// [`FeatureDType::Source`] and whose name matches the
    /// [`crate::collections::dataset::document::columns::SOURCE`] convention.
    /// This is the doctrinal test for "is this Valuation evidentiary."
    pub fn is_evidentiary(&self) -> bool {
        self.evidentiary_feature().is_some()
    }

    fn evidentiary_feature(&self) -> Option<&FeatureDescriptor> {
        use crate::collections::dataset::document::columns as doccols;
        self.schema.iter().find(|d| {
            d.dtype == FeatureDType::Source
                && d.name.as_str() == doccols::SOURCE
                && matches!(d.role, FeatureRole::Projection)
        })
    }

    /// Read this Valuation as the Document side of the meeting-point.
    /// Returns the `DocumentFrame` view of the same underlying frame.
    /// `Err(NotEvidentiary)` if the schema does not declare the
    /// distinguished `source : Source` feature.
    pub fn evidence(&self) -> Result<DocumentFrame, ValuationError> {
        if !self.is_evidentiary() {
            return Err(ValuationError::NotEvidentiary);
        }
        DocumentFrame::from_dataframe(self.df.clone()).map_err(ValuationError::Polars)
    }
}

impl DocumentFrame {
    /// Read this Document as the Valuation side of the meeting-point.
    /// The schema must include a `source : Source` feature; the rest of the
    /// schema describes whichever Document Features are projected.
    pub fn as_valuation(
        &self,
        schema: Vec<FeatureDescriptor>,
    ) -> Result<ValuationFrame, ValuationError> {
        ValuationFrame::with_schema_descriptors(self.dataframe().clone(), schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::document::SpanUnit;

    fn source_feature() -> FeatureDescriptor {
        use crate::collections::dataset::document::columns as doccols;
        FeatureDescriptor::projection(doccols::SOURCE, FeatureDType::Source)
    }

    #[test]
    fn valuation_is_a_partial_map() {
        let v = Valuation::new()
            .with("token", ValuationCell::String("hello".into()))
            .with("span", ValuationCell::Span(Span::new(0, 5, SpanUnit::Char)));
        assert_eq!(v.len(), 2);
        assert!(v.contains(&FeatureName::new("token")));
        assert!(!v.contains(&FeatureName::new("absent")));
        assert_eq!(
            v.get(&FeatureName::new("token")).map(|c| c.kind()),
            Some("string")
        );
    }

    #[test]
    fn valuation_cell_kinds_are_stable() {
        assert_eq!(ValuationCell::Null.kind(), "null");
        assert_eq!(ValuationCell::Int64(1).kind(), "int64");
        assert_eq!(
            ValuationCell::Source(ContentHash("h".into())).kind(),
            "source"
        );
        assert_eq!(
            ValuationCell::Nested(Box::new(Valuation::new())).kind(),
            "model"
        );
    }

    #[test]
    fn valuation_frame_validates_schema_against_columns() {
        let docs = DocumentFrame::from_source_hashes(vec![
            ContentHash("sha256:a".into()),
            ContentHash("sha256:b".into()),
        ])
        .unwrap();

        // Schema with only `source` — present in the frame, should succeed.
        let v = ValuationFrame::with_schema_descriptors(
            docs.dataframe().clone(),
            vec![source_feature()],
        )
        .expect("valid schema");
        assert_eq!(v.len(), 2);
        assert_eq!(v.schema().len(), 1);

        // Schema asks for a column the frame does not have.
        let err = ValuationFrame::with_schema_descriptors(
            docs.dataframe().clone(),
            vec![FeatureDescriptor::projection(
                "missing",
                FeatureDType::String,
            )],
        )
        .unwrap_err();
        assert!(matches!(err, ValuationError::SchemaColumnMissing(_)));
    }

    #[test]
    fn meeting_point_bridges_document_and_valuation() {
        let docs = DocumentFrame::from_source_hashes(vec![ContentHash("sha256:x".into())]).unwrap();

        // Document → Valuation (intensional reading).
        let v = docs
            .as_valuation(vec![source_feature()])
            .expect("schema valid");
        assert!(v.is_evidentiary());

        // Valuation → Document (extensional reading).
        let back = v.evidence().expect("is evidentiary");
        assert_eq!(back.len(), 1);
    }

    #[test]
    fn non_evidentiary_valuation_refuses_evidence() {
        // A Valuation whose schema does not include `source : Source`.
        // Build a frame that *happens* to have a `source` column (because
        // DocumentFrame requires it), but pair it with a schema that does
        // not declare that column — the schema is what determines the
        // intensional reading, so this is *not* evidentiary.
        let docs = DocumentFrame::from_source_hashes(vec![ContentHash("sha256:y".into())]).unwrap();
        // Pair with an empty schema — frame has columns, schema does not
        // claim the distinguished feature.
        let v = ValuationFrame::with_schema_descriptors(docs.dataframe().clone(), vec![])
            .expect("empty schema is trivially valid");
        assert!(!v.is_evidentiary());
        assert!(matches!(v.evidence(), Err(ValuationError::NotEvidentiary)));
    }
}
