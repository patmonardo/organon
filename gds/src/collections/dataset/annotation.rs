//! `Annotation` — the evidentiary `Feature` (R3 in the doctrine).
//!
//! See `gds/doc/SEMANTIC-DATASET-FIVE-FOLD.md` (Five-Fold Synthesis), Root
//! Object **R3**, and the §"Four roles of a Feature" table. NLTK Ch11 is
//! the reference text for *why* annotation must carry provenance as part of
//! its identity rather than as metadata.
//!
//! An `Annotation` is **not a parallel concept to `Feature`**. It is a
//! `Feature` (R5) playing its **Annotation** role — the fourth of the four
//! Feature roles in the doctrine — with one additional commitment:
//!
//! - A *mandatory* provenance tuple
//!   `(layer, annotator, guideline_version, derivation)`.
//!
//! Provenance is part of the Annotation's identity. Two Annotations with
//! different annotators (or different guideline versions) are *different
//! Features*, even if they address the same Document path with the same
//! codomain. This is the contract that makes inter-annotator agreement
//! (κ, windowdiff) a first-class measurement on the `Corpus` (R7) rather
//! than on a downstream model.
//!
//! Position in the four-fold:
//!
//! Annotation lives on the **intensional** side as a Feature, but it
//! addresses a `Document` (extensional/evidentiary), and its presence on a
//! Document is what makes that Document's interpretation *witnessed*. It is
//! the bridge across the Document/Model meeting-point.
//!
//! Contract this module owes the kernel:
//!
//! - Provenance is *required*, not optional. An Annotation without
//!   provenance is a `Feature` in its Projection role, not an Annotation.
//! - Annotation equality reflects provenance. The kernel must not silently
//!   merge Annotations from different layers/annotators/guideline-versions.
//! - The `Corpus` (R7) groups Annotations by `(layer, guideline_version)`
//!   for agreement scoring; the layer dimension is part of the public API,
//!   not an implementation detail.
//!
//! Status: **Phase 2**. The `Annotation` wrapper is a `FeatureDescriptor`
//! constrained to its Annotation role. The `AnnotationFrame` access object
//! holds annotation *records* — the rows that a `Corpus` groups by
//! `(layer, annotator, guideline_version)` for agreement scoring.

use polars::prelude::{DataFrame, NamedFrom, PolarsError, Series};

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::document::SpanUnit;
use crate::collections::dataset::feature_role::{
    FeatureDType, FeatureDescriptor, FeatureName, FeatureRole, Provenance,
};

/// A `FeatureDescriptor` constrained to its Annotation role.
///
/// `Annotation` is **not a parallel concept to `Feature`** — it is exactly
/// "Feature in its Annotation role" with the mandatory provenance tuple.
/// This wrapper exists for ergonomics and to make role-checking infallible.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annotation {
    descriptor: FeatureDescriptor,
}

impl Annotation {
    pub fn new(name: impl Into<FeatureName>, dtype: FeatureDType, provenance: Provenance) -> Self {
        Self {
            descriptor: FeatureDescriptor::annotation(name, dtype, provenance),
        }
    }

    /// Try to view a generic `FeatureDescriptor` as an `Annotation`. Returns
    /// `None` if the descriptor's role is not `Annotation`.
    pub fn from_descriptor(descriptor: FeatureDescriptor) -> Option<Self> {
        if matches!(descriptor.role, FeatureRole::Annotation { .. }) {
            Some(Self { descriptor })
        } else {
            None
        }
    }

    pub fn descriptor(&self) -> &FeatureDescriptor {
        &self.descriptor
    }

    pub fn into_descriptor(self) -> FeatureDescriptor {
        self.descriptor
    }

    /// Provenance is part of the Annotation's identity (see doctrine §R3).
    pub fn provenance(&self) -> &Provenance {
        match &self.descriptor.role {
            FeatureRole::Annotation { provenance } => provenance,
            // Construction guarantees Annotation role.
            _ => unreachable!("Annotation invariant violated"),
        }
    }
}

/// One row of an annotation record: a `Span` over a `Document` (identified
/// by its source `ContentHash` string), carrying a value and the provenance
/// tuple that makes the record evidentiary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationRecord {
    pub document: String,
    pub span_start: u64,
    pub span_end: u64,
    pub span_unit: SpanUnit,
    pub value: String,
    pub layer: String,
    pub annotator: String,
    pub guideline_version: String,
    pub derivation: String,
}

impl AnnotationRecord {
    pub fn new(
        document: impl Into<String>,
        span_start: u64,
        span_end: u64,
        span_unit: SpanUnit,
        value: impl Into<String>,
        provenance: &Provenance,
    ) -> Self {
        Self {
            document: document.into(),
            span_start,
            span_end,
            span_unit,
            value: value.into(),
            layer: provenance.layer.clone(),
            annotator: provenance.annotator.clone(),
            guideline_version: provenance.guideline_version.clone(),
            derivation: provenance.derivation.clone(),
        }
    }
}

fn span_unit_str(unit: SpanUnit) -> &'static str {
    match unit {
        SpanUnit::Char => "char",
        SpanUnit::Byte => "byte",
        SpanUnit::Token => "token",
        SpanUnit::Time => "time",
        SpanUnit::Page => "page",
    }
}

/// Canonical column names for an [`AnnotationFrame`].
pub mod columns {
    pub const DOCUMENT: &str = "document";
    pub const SPAN_START: &str = "span_start";
    pub const SPAN_END: &str = "span_end";
    pub const SPAN_UNIT: &str = "span_unit";
    pub const VALUE: &str = "value";
    pub const LAYER: &str = "layer";
    pub const ANNOTATOR: &str = "annotator";
    pub const GUIDELINE_VERSION: &str = "guideline_version";
    pub const DERIVATION: &str = "derivation";
}

/// Frame-level access object for annotation records.
///
/// One row per record. Grouping by
/// `(layer, annotator, guideline_version)` is what the `Corpus` uses to
/// score agreement (κ, windowdiff). That grouping is deferred — Phase 5.
#[derive(Debug, Clone)]
pub struct AnnotationFrame {
    df: GDSDataFrame,
}

impl AnnotationFrame {
    pub fn from_dataframe(df: GDSDataFrame) -> Result<Self, PolarsError> {
        let names: std::collections::HashSet<String> = df.column_names().into_iter().collect();
        for required in [
            columns::DOCUMENT,
            columns::SPAN_START,
            columns::SPAN_END,
            columns::SPAN_UNIT,
            columns::VALUE,
            columns::LAYER,
            columns::ANNOTATOR,
            columns::GUIDELINE_VERSION,
            columns::DERIVATION,
        ] {
            if !names.contains(required) {
                return Err(PolarsError::ColumnNotFound(required.into()));
            }
        }
        Ok(Self { df })
    }

    pub fn from_records<I>(records: I) -> Result<Self, PolarsError>
    where
        I: IntoIterator<Item = AnnotationRecord>,
    {
        let mut document = Vec::<String>::new();
        let mut span_start = Vec::<u64>::new();
        let mut span_end = Vec::<u64>::new();
        let mut span_unit = Vec::<String>::new();
        let mut value = Vec::<String>::new();
        let mut layer = Vec::<String>::new();
        let mut annotator = Vec::<String>::new();
        let mut guideline_version = Vec::<String>::new();
        let mut derivation = Vec::<String>::new();

        for r in records {
            document.push(r.document);
            span_start.push(r.span_start);
            span_end.push(r.span_end);
            span_unit.push(span_unit_str(r.span_unit).to_owned());
            value.push(r.value);
            layer.push(r.layer);
            annotator.push(r.annotator);
            guideline_version.push(r.guideline_version);
            derivation.push(r.derivation);
        }

        let df = DataFrame::new(vec![
            Series::new(columns::DOCUMENT.into(), document).into(),
            Series::new(columns::SPAN_START.into(), span_start).into(),
            Series::new(columns::SPAN_END.into(), span_end).into(),
            Series::new(columns::SPAN_UNIT.into(), span_unit).into(),
            Series::new(columns::VALUE.into(), value).into(),
            Series::new(columns::LAYER.into(), layer).into(),
            Series::new(columns::ANNOTATOR.into(), annotator).into(),
            Series::new(columns::GUIDELINE_VERSION.into(), guideline_version).into(),
            Series::new(columns::DERIVATION.into(), derivation).into(),
        ])?;

        Ok(Self {
            df: GDSDataFrame::new(df),
        })
    }

    pub fn dataframe(&self) -> &GDSDataFrame {
        &self.df
    }

    pub fn into_dataframe(self) -> GDSDataFrame {
        self.df
    }

    pub fn len(&self) -> usize {
        self.df.height()
    }

    pub fn is_empty(&self) -> bool {
        self.df.height() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p() -> Provenance {
        Provenance::new("ner", "alice", "v1.2", "manual")
    }

    #[test]
    fn annotation_carries_provenance_in_identity() {
        let a = Annotation::new("ner_tag", FeatureDType::String, p());
        assert_eq!(a.provenance().annotator, "alice");
        assert!(matches!(
            a.descriptor().role,
            FeatureRole::Annotation { .. }
        ));
    }

    #[test]
    fn from_descriptor_rejects_non_annotation_roles() {
        let proj = FeatureDescriptor::projection("token", FeatureDType::String);
        assert!(Annotation::from_descriptor(proj).is_none());
        let ann = FeatureDescriptor::annotation("x", FeatureDType::String, p());
        assert!(Annotation::from_descriptor(ann).is_some());
    }

    #[test]
    fn annotation_frame_round_trips_records() {
        let prov = p();
        let frame = AnnotationFrame::from_records(vec![
            AnnotationRecord::new("sha256:doc1", 0, 5, SpanUnit::Char, "PER", &prov),
            AnnotationRecord::new("sha256:doc1", 10, 18, SpanUnit::Char, "ORG", &prov),
            AnnotationRecord::new("sha256:doc2", 0, 3, SpanUnit::Token, "LOC", &prov),
        ])
        .expect("frame construction");

        assert_eq!(frame.len(), 3);
        let cols: std::collections::HashSet<String> =
            frame.dataframe().column_names().into_iter().collect();
        for c in [
            columns::DOCUMENT,
            columns::LAYER,
            columns::ANNOTATOR,
            columns::GUIDELINE_VERSION,
            columns::SPAN_UNIT,
        ] {
            assert!(cols.contains(c), "missing column {c}");
        }
    }

    #[test]
    fn annotation_frame_rejects_missing_columns() {
        let df = GDSDataFrame::new(
            DataFrame::new(vec![Series::new("document".into(), vec!["x"]).into()]).unwrap(),
        );
        let err = AnnotationFrame::from_dataframe(df).unwrap_err();
        assert!(matches!(err, PolarsError::ColumnNotFound(_)));
    }
}
