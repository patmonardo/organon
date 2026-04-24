//! `Source` — content-addressed immutable bytes (R1 in the doctrine).
//!
//! See `gds/doc/SEMANTIC-DATASET-FIVE-FOLD.md` (Five-Fold Synthesis), Root
//! Object **R1**, and NLTK Ch11 *Managing Linguistic Data* for the motivating
//! account. Source is the ground-truth artifact: a `.wav`, a scanned page, a
//! raw text file. It carries bytes and a MIME/structure declaration. **It has
//! no schema.**
//!
//! Contract this module owes the kernel:
//!
//! - A `Source` is **immutable**. The kernel must guarantee that two
//!   `Source`s with equal `hash` are interchangeable.
//! - A `Source` carries `(uri, hash, media_type, len)` and nothing semantic.
//!   Schema, offsets, and annotations live on `Document` (R2) and on the
//!   `Annotation` Features (R3) that point back into the Source.
//! - `Source` is the *evidentiary substrate*. Every `Document` that claims
//!   to be grounded must pin itself to one via the distinguished Feature
//!   `source : Source`.
//!
//! Position in the four-fold:
//!
//! Source is *pre-extensional*. It is the byte ground beneath the
//! `Frame:Series` cell — addressable but not yet projected into a Frame.
//!
//! Status: **Phase 1**. Minimal access-object types only — a `Source` row
//! struct, a `SourceFrame` wrapper over `GDSDataFrame` with the canonical
//! schema `(uri, hash, media_type, len)`, and the `ContentHash` /
//! `MediaType` newtypes. No bytes loader, no fetcher, no validation beyond
//! schema-shape. See `SEMANTIC-DATASET-FIVE-FOLD.md` §"What this note
//! commits to" for what is and is not decided.

use polars::prelude::{DataFrame, DataType, Field, NamedFrom, PolarsError, Schema, Series};

use crate::collections::dataframe::GDSDataFrame;

/// Content-addressed identity for a `Source`. The kernel guarantees that two
/// `Source`s with equal `ContentHash` are interchangeable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentHash(pub String);

impl ContentHash {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// MIME / structure declaration carried by a `Source`. Free-form for now;
/// future passes may constrain to a registered enum.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MediaType(pub String);

impl MediaType {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A single `Source` row: the immutable, content-addressed evidentiary
/// substrate for a `Document`. Carries no schema and no bytes — only the
/// addressing tuple `(uri, hash, media_type, len)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub uri: String,
    pub hash: ContentHash,
    pub media_type: MediaType,
    pub len: u64,
}

impl Source {
    pub fn new(
        uri: impl Into<String>,
        hash: impl Into<String>,
        media_type: impl Into<String>,
        len: u64,
    ) -> Self {
        Self {
            uri: uri.into(),
            hash: ContentHash(hash.into()),
            media_type: MediaType(media_type.into()),
            len,
        }
    }
}

/// Canonical column names for a `SourceFrame`.
pub mod columns {
    pub const URI: &str = "uri";
    pub const HASH: &str = "hash";
    pub const MEDIA_TYPE: &str = "media_type";
    pub const LEN: &str = "len";
}

/// The canonical Polars schema for a `SourceFrame`.
pub fn source_schema() -> Schema {
    Schema::from_iter(vec![
        Field::new(columns::URI.into(), DataType::String),
        Field::new(columns::HASH.into(), DataType::String),
        Field::new(columns::MEDIA_TYPE.into(), DataType::String),
        Field::new(columns::LEN.into(), DataType::UInt64),
    ])
}

/// Frame-level access object over a table of `Source` rows.
///
/// Thin wrapper around `GDSDataFrame` with the schema declared by
/// [`source_schema`]. Construction validates column presence; per-row
/// reads decode into [`Source`] values.
#[derive(Debug, Clone)]
pub struct SourceFrame {
    df: GDSDataFrame,
}

impl SourceFrame {
    /// Wrap an existing `GDSDataFrame`, validating that the canonical
    /// columns are present. Extra columns are tolerated; missing columns
    /// are an error.
    pub fn from_dataframe(df: GDSDataFrame) -> Result<Self, PolarsError> {
        let names: std::collections::HashSet<String> = df.column_names().into_iter().collect();
        for required in [
            columns::URI,
            columns::HASH,
            columns::MEDIA_TYPE,
            columns::LEN,
        ] {
            if !names.contains(required) {
                return Err(PolarsError::ColumnNotFound(required.into()));
            }
        }
        Ok(Self { df })
    }

    /// Build a `SourceFrame` from an iterator of [`Source`] rows.
    pub fn from_rows<I>(rows: I) -> Result<Self, PolarsError>
    where
        I: IntoIterator<Item = Source>,
    {
        let mut uris = Vec::new();
        let mut hashes = Vec::new();
        let mut mimes = Vec::new();
        let mut lens = Vec::<u64>::new();
        for row in rows {
            uris.push(row.uri);
            hashes.push(row.hash.0);
            mimes.push(row.media_type.0);
            lens.push(row.len);
        }
        let df = DataFrame::new(vec![
            Series::new(columns::URI.into(), uris).into(),
            Series::new(columns::HASH.into(), hashes).into(),
            Series::new(columns::MEDIA_TYPE.into(), mimes).into(),
            Series::new(columns::LEN.into(), lens).into(),
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

    fn sample_source() -> Source {
        Source::new("file:///tmp/example.txt", "sha256:abc123", "text/plain", 42)
    }

    #[test]
    fn source_round_trips_through_frame() {
        let frame = SourceFrame::from_rows(vec![sample_source(), sample_source()])
            .expect("frame construction");
        assert_eq!(frame.len(), 2);
        assert!(!frame.is_empty());

        let cols: std::collections::HashSet<String> =
            frame.dataframe().column_names().into_iter().collect();
        for c in [
            columns::URI,
            columns::HASH,
            columns::MEDIA_TYPE,
            columns::LEN,
        ] {
            assert!(cols.contains(c), "missing column {c}");
        }
    }

    #[test]
    fn source_frame_rejects_missing_columns() {
        let df = GDSDataFrame::new(
            DataFrame::new(vec![Series::new("uri".into(), vec!["x"]).into()]).unwrap(),
        );
        let err = SourceFrame::from_dataframe(df).unwrap_err();
        assert!(matches!(err, PolarsError::ColumnNotFound(_)));
    }

    #[test]
    fn schema_declares_canonical_columns() {
        let schema = source_schema();
        assert_eq!(schema.len(), 4);
        assert_eq!(schema.get(columns::LEN), Some(&DataType::UInt64));
    }

    #[test]
    fn content_hash_and_media_type_are_thin_newtypes() {
        let h = ContentHash("sha256:zzz".into());
        let m = MediaType("application/json".into());
        assert_eq!(h.as_str(), "sha256:zzz");
        assert_eq!(m.as_str(), "application/json");
    }
}
