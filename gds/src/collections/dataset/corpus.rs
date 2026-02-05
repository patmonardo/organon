//! Corpus dataset: lightweight text corpus wrapper over `Dataset`.
//!
//! Provides convenience builders for text-based datasets (files, directories).

use std::fs::read_to_string;
use std::path::Path;

use polars::prelude::{NamedFrom, PlSmallStr, Series};

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataframe::GDSFrameError;
use crate::collections::dataset::dataset::Dataset;

/// Text Corpus: a single-column dataset with `text` column.
#[derive(Debug, Clone)]
pub struct Corpus {
    dataset: Dataset,
}

impl Corpus {
    /// Construct a corpus from in-memory strings.
    pub fn from_texts<T: AsRef<str>>(texts: &[T]) -> Result<Self, GDSFrameError> {
        let strs: Vec<String> = texts.iter().map(|s| s.as_ref().to_string()).collect();
        let s = Series::new(PlSmallStr::from_static("text"), strs);
        let df = GDSDataFrame::from_series(vec![s])?;
        Ok(Self {
            dataset: Dataset::new(df),
        })
    }

    /// Read a single text file and create a corpus with one row.
    pub fn from_text_file(path: impl AsRef<Path>) -> Result<Self, GDSFrameError> {
        let content = read_to_string(path.as_ref())?;
        Self::from_texts(&[content])
    }

    /// Read multiple files and create a corpus with one row per file.
    pub fn from_text_files(paths: &[impl AsRef<Path>]) -> Result<Self, GDSFrameError> {
        let mut rows = Vec::with_capacity(paths.len());
        for p in paths {
            let content = read_to_string(p.as_ref())?;
            rows.push(content);
        }
        Self::from_texts(&rows)
    }

    /// Access the inner dataset.
    pub fn dataset(&self) -> &Dataset {
        &self.dataset
    }

    /// Consume into the inner dataset.
    pub fn into_dataset(self) -> Dataset {
        self.dataset
    }

    /// Number of documents in the corpus.
    pub fn len(&self) -> usize {
        self.dataset.row_count()
    }

    /// Quick tokenizer that splits on whitespace and returns a `Series` of token counts.
    /// This is intentionally tiny; real tokenization would be pluggable.
    pub fn token_counts(&self) -> Result<polars::prelude::Series, GDSFrameError> {
        // build an expression that counts tokens via splitting on whitespace
        // We materialize eagerly for simplicity here.
        let df = self.dataset.table().dataframe().clone();
        let series = df
            .column("text")?
            .as_series()
            .ok_or_else(|| GDSFrameError::from("column is not a Series"))?;

        let mut counts: Vec<i64> = Vec::with_capacity(series.len());
        for i in 0..series.len() {
            let av = series.get(i)?;
            // Fall back to textual rendering if the underlying AnyValue API differs by feature
            let s = av.to_string();
            if s == "null" {
                counts.push(0)
            } else {
                counts.push(s.split_whitespace().count() as i64)
            }
        }

        Ok(Series::new(PlSmallStr::from_static("token_count"), counts))
    }
}
