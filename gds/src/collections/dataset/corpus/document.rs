//! `Document` — the addressable meaning-bearer (R2 in the doctrine).
//!
//! See `gds/doc/SEMANTIC-DATASET-FIVE-FOLD.md` (Five-Fold Synthesis), Root
//! Object **R2** and the §"Document / Model meeting-point" section. NLTK Ch11
//! is the reference text.
//!
//! A `Document` is a `Model` (R4) with two distinguishing commitments:
//!
//! 1. A distinguished Feature `source : Source` pinning it to an immutable
//!    `Source` (R1).
//! 2. *All* of its other Features are offset-indexed back into that Source —
//!    via a typed `Span` (char range, byte range, time range, page-box).
//!    Never a raw integer pair.
//!
//! Documents are addressable; their Features are *evidence*. A Document can
//! exist uninterpreted (raw spans over a Source with no further annotation)
//! and still be a well-formed kernel object.
//!
//! Position in the four-fold:
//!
//! Document is the **extensional/evidentiary** counterpart of `Model`. They
//! sit on opposite axes and meet at exactly one place: the Document side
//! reads its distinguished Feature as `source : Source`; the Model side
//! reads the same Feature as `evidence : Document`. Same Feature, two
//! readings. This is what lets `Token`, `Span`, `Tree`, `FeatStruct` cells
//! be both *a region of a Document* and *a Model in their own right*.
//!
//! Contract this module owes the kernel:
//!
//! - Every Document instance must carry a resolved `source : Source`
//!   reference; an "ungrounded Document" is a contradiction (use a `Model`).
//! - All Features on a Document must be expressible as offsets into its
//!   Source via a typed `Span` codomain.
//! - The Annotation Features (R3) attached to a Document carry the
//!   provenance tuple required by R3.
//!
//! Status: **Phase 1**. Minimal access-object types only — `SpanUnit`,
//! `Span`, and `DocumentFrame` (a `GDSDataFrame` wrapper that requires the
//! distinguished `source` column carrying a `ContentHash` reference).
//! Offset columns and per-document Feature decoding come in later phases.
//! See `SEMANTIC-DATASET-FIVE-FOLD.md` §"Document / Model meeting-point".

use polars::prelude::{DataFrame, NamedFrom, PolarsError, Series};

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::source::ContentHash;

/// The unit a `Span` is measured in. Documents declare which unit applies
/// to a given offset column; mixing units in one column is an error a later
/// phase will enforce.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpanUnit {
    Char,
    Byte,
    Token,
    Time,
    Page,
}

/// A typed offset range into a `Source`. Always carried with its `unit`;
/// raw integer pairs are deliberately not allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: u64,
    pub end: u64,
    pub unit: SpanUnit,
}

impl Span {
    pub fn new(start: u64, end: u64, unit: SpanUnit) -> Self {
        Self { start, end, unit }
    }

    pub fn len(&self) -> u64 {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.end <= self.start
    }
}

/// Canonical column names for a `DocumentFrame`.
pub mod columns {
    /// The distinguished Feature `source : Source` — required on every
    /// `DocumentFrame`. Stored as the source's `ContentHash` string.
    pub const SOURCE: &str = "source";
}

/// Frame-level access object for a table of `Document` rows. Each row is
/// one Document; the distinguished `source` column pins it to a `Source`
/// (by `ContentHash`). Other columns are Document Features and may be
/// offset-indexed via `Span`-shaped data in later phases.
#[derive(Debug, Clone)]
pub struct DocumentFrame {
    df: GDSDataFrame,
}

impl DocumentFrame {
    /// Wrap an existing `GDSDataFrame`, validating that the distinguished
    /// `source` column is present.
    pub fn from_dataframe(df: GDSDataFrame) -> Result<Self, PolarsError> {
        let names: std::collections::HashSet<String> = df.column_names().into_iter().collect();
        if !names.contains(columns::SOURCE) {
            return Err(PolarsError::ColumnNotFound(columns::SOURCE.into()));
        }
        Ok(Self { df })
    }

    /// Build a minimal `DocumentFrame` from an iterator of source-hash
    /// references. Useful for tests and as the smallest well-formed shape:
    /// one row per Document, only the distinguished column populated.
    pub fn from_source_hashes<I>(hashes: I) -> Result<Self, PolarsError>
    where
        I: IntoIterator<Item = ContentHash>,
    {
        let strs: Vec<String> = hashes.into_iter().map(|h| h.0).collect();
        let df =
            DataFrame::new_infer_height(vec![Series::new(columns::SOURCE.into(), strs).into()])?;
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

    #[test]
    fn span_arithmetic_is_saturating() {
        let s = Span::new(10, 25, SpanUnit::Char);
        assert_eq!(s.len(), 15);
        assert!(!s.is_empty());

        let degenerate = Span::new(40, 30, SpanUnit::Byte);
        assert_eq!(degenerate.len(), 0);
        assert!(degenerate.is_empty());
    }

    #[test]
    fn document_frame_requires_source_column() {
        let df = GDSDataFrame::new(
            DataFrame::new_infer_height(vec![Series::new("not_source".into(), vec!["x"]).into()])
                .unwrap(),
        );
        let err = DocumentFrame::from_dataframe(df).unwrap_err();
        assert!(matches!(err, PolarsError::ColumnNotFound(_)));
    }

    #[test]
    fn document_frame_round_trips_source_hashes() {
        let frame = DocumentFrame::from_source_hashes(vec![
            ContentHash("sha256:a".into()),
            ContentHash("sha256:b".into()),
            ContentHash("sha256:c".into()),
        ])
        .expect("frame construction");
        assert_eq!(frame.len(), 3);
        assert!(!frame.is_empty());
        assert!(frame
            .dataframe()
            .column_names()
            .iter()
            .any(|c| c == columns::SOURCE));
    }
}
