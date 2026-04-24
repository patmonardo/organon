//! `Corpus` — the universal evidentiary product of the dataset layer.
//!
//! A `Corpus` is the user-facing crystallization of evidence. It owns
//! three frames:
//!
//! - a **Source pool** — content-addressed immutable byte references
//!   ([`SourceFrame`]);
//! - a **Document frame** — one row per document, each pinned to a
//!   `Source` by content hash ([`DocumentFrame`]);
//! - an **Annotation index** — annotation records grouped by
//!   `(layer, annotator, guideline_version)` ([`AnnotationFrame`]).
//!
//! It also exposes a working **text dataset** ([`Dataset`]) — a Polars
//! frame with a `text` column aligned 1:1 with the document frame — so
//! the standard semantic-web/NLP convenience methods (`tokenize`,
//! `stem`, `parse`, `tag`) read directly off the corpus without an
//! extra unwrapping step.
//!
//! Construction:
//!
//! - `Corpus::from_texts` / `from_text_file` / `from_text_files` are the
//!   common entry points. They hash each text into a `Source`, seed the
//!   `DocumentFrame` from those hashes, and start with an empty
//!   `AnnotationFrame`.
//! - `Corpus::from_frames` lets callers assemble a corpus from
//!   pre-built frames (e.g. when annotations already exist).
//!
//! The `LanguageModel` (in [`crate::collections::dataset::lm`]) is the
//! intensional counterpart to a `Corpus`. A semantic dataset in use is a
//! `Corpus` paired with a `LanguageModel`: evidence on one side,
//! meaning on the other.

use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::path::Path;

use polars::prelude::{NamedFrom, PlSmallStr, PolarsError, Series};
use sha2::{Digest, Sha256};

use crate::collections::dataframe::{GDSDataFrame, GDSFrameError};
use crate::collections::dataset::annotation::{columns as anncols, AnnotationFrame};
use crate::collections::dataset::artifact::DatasetArtifactKind;
use crate::collections::dataset::dataset::Dataset;
use crate::collections::dataset::document::DocumentFrame;
use crate::collections::dataset::parse::ParseForest;
use crate::collections::dataset::parser::Parser;
use crate::collections::dataset::source::{ContentHash, Source, SourceFrame};
use crate::collections::dataset::stem::Stem;
use crate::collections::dataset::stemmer::Stemmer;
use crate::collections::dataset::tag::Tag;
use crate::collections::dataset::tagger::Tagger;
use crate::collections::dataset::token::Token;
use crate::collections::dataset::tokenizer::Tokenizer;

/// Errors raised when constructing or querying a `Corpus`.
#[derive(Debug)]
pub enum CorpusError {
    /// A Polars-level error during a query (e.g. distinct-tuple read).
    Polars(PolarsError),
    /// A frame-level error during construction.
    Frame(GDSFrameError),
    /// An I/O error while reading source text from disk.
    Io(std::io::Error),
}

impl From<PolarsError> for CorpusError {
    fn from(e: PolarsError) -> Self {
        Self::Polars(e)
    }
}

impl From<GDSFrameError> for CorpusError {
    fn from(e: GDSFrameError) -> Self {
        Self::Frame(e)
    }
}

impl From<std::io::Error> for CorpusError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::fmt::Display for CorpusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Polars(e) => write!(f, "polars error: {e}"),
            Self::Frame(e) => write!(f, "frame error: {e}"),
            Self::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for CorpusError {}

/// One annotator-effort identity within a `Corpus`. Rows that share the
/// `(layer, annotator, guideline_version)` tuple are observations of the
/// same annotation effort and form the natural grouping for inter-annotator
/// agreement scoring.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnnotatorEffort {
    pub layer: String,
    pub annotator: String,
    pub guideline_version: String,
}

/// The unified `Corpus` access object.
///
/// Holds the three evidentiary frames plus a working text dataset. The
/// text dataset has one row per document (column `text`), aligned with
/// the `DocumentFrame` row order.
#[derive(Debug, Clone)]
pub struct Corpus {
    sources: SourceFrame,
    documents: DocumentFrame,
    annotations: AnnotationFrame,
    dataset: Dataset,
}

impl Corpus {
    // ---- Constructors -----------------------------------------------------

    /// Construct a corpus from in-memory strings. Each string is hashed,
    /// stored as a `Source` (deduplicated by hash), and seeded as a
    /// `Document` row. The annotation index is empty.
    pub fn from_texts<T: AsRef<str>>(texts: &[T]) -> Result<Self, CorpusError> {
        let raw: Vec<String> = texts.iter().map(|s| s.as_ref().to_string()).collect();
        Self::from_text_payload(raw, |idx| format!("mem:///doc/{idx}"))
    }

    /// Read a single text file and create a corpus with one row.
    pub fn from_text_file(path: impl AsRef<Path>) -> Result<Self, CorpusError> {
        let path = path.as_ref();
        let content = read_to_string(path)?;
        let uri = format!("file://{}", path.display());
        Self::from_text_payload(vec![content], move |_| uri.clone())
    }

    /// Read multiple files and create a corpus with one row per file.
    pub fn from_text_files(paths: &[impl AsRef<Path>]) -> Result<Self, CorpusError> {
        let mut texts = Vec::with_capacity(paths.len());
        let mut uris = Vec::with_capacity(paths.len());
        for p in paths {
            let p = p.as_ref();
            texts.push(read_to_string(p)?);
            uris.push(format!("file://{}", p.display()));
        }
        Self::from_text_payload(texts, move |idx| uris[idx].clone())
    }

    /// Assemble a `Corpus` directly from pre-built frames. The caller is
    /// responsible for cross-frame consistency (e.g. every document's
    /// `source` hash appears in `sources`).
    pub fn from_frames(
        sources: SourceFrame,
        documents: DocumentFrame,
        annotations: AnnotationFrame,
        dataset: Dataset,
    ) -> Self {
        Self {
            sources,
            documents,
            annotations,
            dataset,
        }
    }

    fn from_text_payload<F>(texts: Vec<String>, mut uri_for: F) -> Result<Self, CorpusError>
    where
        F: FnMut(usize) -> String,
    {
        // Hash, deduplicate sources, build document hash list, and the
        // text-frame in one pass.
        let mut seen: std::collections::HashMap<String, ()> = std::collections::HashMap::new();
        let mut source_rows = Vec::with_capacity(texts.len());
        let mut doc_hashes = Vec::with_capacity(texts.len());

        for (idx, content) in texts.iter().enumerate() {
            let hash = sha256_hex(content.as_bytes());
            doc_hashes.push(ContentHash(hash.clone()));
            if seen.insert(hash.clone(), ()).is_none() {
                source_rows.push(Source::new(
                    uri_for(idx),
                    hash,
                    "text/plain",
                    content.len() as u64,
                ));
            }
        }

        let sources = SourceFrame::from_rows(source_rows)?;
        let documents = DocumentFrame::from_source_hashes(doc_hashes)?;
        let annotations = AnnotationFrame::from_records(Vec::new())?;

        let s = Series::new(PlSmallStr::from_static("text"), texts);
        let df = GDSDataFrame::from_series(vec![s])?;
        let dataset = Dataset::new(df).with_artifact_kind(DatasetArtifactKind::Corpus);

        Ok(Self {
            sources,
            documents,
            annotations,
            dataset,
        })
    }

    // ---- Frame accessors --------------------------------------------------

    pub fn sources(&self) -> &SourceFrame {
        &self.sources
    }

    pub fn documents(&self) -> &DocumentFrame {
        &self.documents
    }

    pub fn annotations(&self) -> &AnnotationFrame {
        &self.annotations
    }

    pub fn dataset(&self) -> &Dataset {
        &self.dataset
    }

    pub fn dataset_mut(&mut self) -> &mut Dataset {
        &mut self.dataset
    }

    pub fn into_dataset(self) -> Dataset {
        self.dataset
    }

    pub fn into_parts(self) -> (SourceFrame, DocumentFrame, AnnotationFrame, Dataset) {
        (self.sources, self.documents, self.annotations, self.dataset)
    }

    // ---- Counts -----------------------------------------------------------

    /// Number of documents in the corpus.
    pub fn len(&self) -> usize {
        self.documents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }

    pub fn source_count(&self) -> usize {
        self.sources.len()
    }

    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    pub fn annotation_count(&self) -> usize {
        self.annotations.len()
    }

    // ---- Annotation grouping ---------------------------------------------

    /// Distinct annotator-effort tuples present in the annotation frame,
    /// returned as a sorted set so the result is order-stable.
    pub fn efforts(&self) -> Result<BTreeSet<AnnotatorEffort>, CorpusError> {
        let df = self.annotations.dataframe().dataframe();
        if df.height() == 0 {
            return Ok(BTreeSet::new());
        }

        let layer = df.column(anncols::LAYER)?.str()?.clone();
        let annotator = df.column(anncols::ANNOTATOR)?.str()?.clone();
        let guideline = df.column(anncols::GUIDELINE_VERSION)?.str()?.clone();

        let mut out = BTreeSet::new();
        for i in 0..df.height() {
            out.insert(AnnotatorEffort {
                layer: layer.get(i).unwrap_or("").to_owned(),
                annotator: annotator.get(i).unwrap_or("").to_owned(),
                guideline_version: guideline.get(i).unwrap_or("").to_owned(),
            });
        }
        Ok(out)
    }

    /// Distinct annotation layers present in the corpus.
    pub fn layers(&self) -> Result<BTreeSet<String>, CorpusError> {
        Ok(self.efforts()?.into_iter().map(|e| e.layer).collect())
    }

    /// Token counts per document, computed by whitespace splitting.
    pub fn token_counts(&self) -> Result<Series, GDSFrameError> {
        let df = self.dataset.table().dataframe().clone();
        let series = df
            .column("text")?
            .as_series()
            .ok_or_else(|| GDSFrameError::from("column is not a Series"))?;

        let mut counts: Vec<i64> = Vec::with_capacity(series.len());
        for i in 0..series.len() {
            let av = series.get(i)?;
            let s = av.to_string();
            if s == "null" {
                counts.push(0);
            } else {
                counts.push(s.split_whitespace().count() as i64);
            }
        }

        Ok(Series::new(PlSmallStr::from_static("token_count"), counts))
    }

    /// Tokenize each document with a pluggable tokenizer.
    pub fn tokenize<T: Tokenizer>(&self, tokenizer: &T) -> Result<Vec<Vec<Token>>, GDSFrameError> {
        let texts = self.read_text_column()?;
        Ok(texts.into_iter().map(|t| tokenizer.tokenize(&t)).collect())
    }

    /// Stem each document using the provided tokenizer and stemmer.
    pub fn stem<T: Tokenizer, S: Stemmer>(
        &self,
        tokenizer: &T,
        stemmer: &S,
    ) -> Result<Vec<Vec<Stem>>, GDSFrameError> {
        let tokens = self.tokenize(tokenizer)?;
        Ok(tokens.iter().map(|doc| stemmer.stem_tokens(doc)).collect())
    }

    /// Parse each document using the provided tokenizer and parser.
    pub fn parse<T: Tokenizer, P: Parser>(
        &self,
        tokenizer: &T,
        parser: &P,
    ) -> Result<Vec<ParseForest>, GDSFrameError> {
        let tokens = self.tokenize(tokenizer)?;
        Ok(tokens.iter().map(|doc| parser.parse_tokens(doc)).collect())
    }

    /// Tag each document using the provided tokenizer and tagger.
    pub fn tag<T: Tokenizer, G: Tagger>(
        &self,
        tokenizer: &T,
        tagger: &G,
    ) -> Result<Vec<Vec<Tag>>, GDSFrameError> {
        let tokens = self.tokenize(tokenizer)?;
        Ok(tokens.iter().map(|doc| tagger.tag_tokens(doc)).collect())
    }

    fn read_text_column(&self) -> Result<Vec<String>, GDSFrameError> {
        let df = self.dataset.table().dataframe().clone();
        let series = df
            .column("text")?
            .as_series()
            .ok_or_else(|| GDSFrameError::from("column is not a Series"))?;
        let mut out = Vec::with_capacity(series.len());
        for i in 0..series.len() {
            let av = series.get(i)?;
            let s = av.to_string();
            out.push(if s == "null" { String::new() } else { s });
        }
        Ok(out)
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(7 + digest.len() * 2);
    hex.push_str("sha256:");
    for byte in digest {
        use std::fmt::Write;
        let _ = write!(hex, "{byte:02x}");
    }
    hex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_texts_builds_all_three_frames() {
        let c = Corpus::from_texts(&["alpha beta", "gamma"]).expect("corpus");
        assert_eq!(c.document_count(), 2);
        assert_eq!(c.source_count(), 2);
        assert_eq!(c.annotation_count(), 0);
        assert_eq!(c.len(), 2);
        assert!(c.efforts().unwrap().is_empty());
    }

    #[test]
    fn duplicate_texts_dedupe_in_source_pool() {
        let c = Corpus::from_texts(&["same", "same", "different"]).expect("corpus");
        assert_eq!(c.document_count(), 3);
        assert_eq!(c.source_count(), 2);
    }

    #[test]
    fn token_counts_match_whitespace_split() {
        let c = Corpus::from_texts(&["a b c", "x y"]).expect("corpus");
        let counts = c.token_counts().expect("token counts");
        assert_eq!(counts.len(), 2);
    }
}
