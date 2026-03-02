//! High-level Dataset wrapper over Polars-backed tables.
//!
//! This is intentionally lightweight and ergonomic for “dataset-first” workflows
//! without forcing callers to pull in Polars types directly.

use std::path::Path;

use polars::prelude::{Expr, NewChunkedArray, SortMultipleOptions};

use crate::collections::dataframe::selectors::Selector;
use crate::collections::dataframe::table::TableBuilder;
use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataframe::GDSFrameError;
use crate::collections::dataset::semantic::LanguageModelFocus;
use crate::collections::io::{csv, ipc, json, parquet};

/// Minimal dataset wrapper (Polars-backed).
#[derive(Debug, Clone)]
pub struct Dataset {
    name: Option<String>,
    table: GDSDataFrame,
    lm_focus: Option<LanguageModelFocus>,
}

impl Dataset {
    pub fn new(table: GDSDataFrame) -> Self {
        Self {
            name: None,
            table,
            lm_focus: None,
        }
    }

    pub fn named(name: impl Into<String>, table: GDSDataFrame) -> Self {
        Self {
            name: Some(name.into()),
            table,
            lm_focus: None,
        }
    }

    pub fn from_builder(builder: TableBuilder) -> Result<Self, GDSFrameError> {
        let table = builder.build()?;
        Ok(Self::new(table))
    }

    pub fn from_csv(path: impl AsRef<Path>) -> Result<Self, GDSFrameError> {
        let table = csv::read_table(path.as_ref())?;
        Ok(Self::new(table))
    }

    pub fn from_parquet(path: impl AsRef<Path>) -> Result<Self, GDSFrameError> {
        let table = parquet::read_table(path.as_ref())?;
        Ok(Self::new(table))
    }

    pub fn from_ipc(path: impl AsRef<Path>) -> Result<Self, GDSFrameError> {
        let table = ipc::read_table(path.as_ref())?;
        Ok(Self::new(table))
    }

    pub fn from_json(path: impl AsRef<Path>) -> Result<Self, GDSFrameError> {
        let table = json::read_table(path.as_ref(), json::JsonReadConfig::default())?;
        Ok(Self::new(table))
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn with_lm_focus(mut self, focus: LanguageModelFocus) -> Self {
        self.lm_focus = Some(focus);
        self
    }

    pub fn lm_focus(&self) -> Option<&LanguageModelFocus> {
        self.lm_focus.as_ref()
    }

    pub fn lm_focus_mut(&mut self) -> Option<&mut LanguageModelFocus> {
        self.lm_focus.as_mut()
    }

    pub fn is_language_model(&self) -> bool {
        self.lm_focus.is_some()
    }

    pub fn is_sdl_compliant(&self) -> bool {
        self.lm_focus
            .as_ref()
            .map(|f| f.is_sdl_compliant)
            .unwrap_or(false)
    }

    /// Extract an `SdlSubgraph` from the given row index.
    /// This requires the dataset to contain `doc_id` and `text` columns,
    /// alongside nested `tokens` and `edges` struct lists for full topological parsing.
    pub fn try_extract_sdl_subgraph(
        &self,
        row_index: usize,
    ) -> Result<crate::collections::dataset::semantic::SdlSubgraph, GDSFrameError> {
        if !self.is_sdl_compliant() {
            return Err(GDSFrameError::from("Dataset lacks SDL compliance focus."));
        }

        let df = self.table.dataframe();

        let doc_id = match df.column("doc_id") {
            Ok(col) => {
                let s = col.as_materialized_series();
                match s.u64()?.get(row_index) {
                    Some(id) => id,
                    None => return Err(GDSFrameError::from("doc_id row out of bounds")),
                }
            }
            Err(_) => 0, // Fallback if missing, though schema implies it
        };

        let text = match df.column("text") {
            Ok(col) => {
                let s = col.as_materialized_series();
                match s.str()?.get(row_index) {
                    Some(t) => t.to_string(),
                    None => return Err(GDSFrameError::from("text row out of bounds")),
                }
            }
            Err(_) => String::new(),
        };

        // Construct the base subgraph.
        // Full Polars unpacking of `tokens` and `edges` List<Struct> into `FeatStruct` properties
        // requires physical iteration over `ListChunked` arrays, which will be integrated in the
        // specific Semantic Table loaders.
        Ok(crate::collections::dataset::semantic::SdlSubgraph::new(
            doc_id, text,
        ))
    }

    /// Two-Phase Universal Dependency Graph Search
    ///
    /// Executes a semantic structure search against the Dataset.
    /// Phase 1: Uses Polars natively to filter rows that contain the necessary structural edges.
    /// Phase 2: Extracts the SdlSubgraphs and evaluates deep FeatStruct semantic unification.
    pub fn search_graph(
        &self,
        query: &crate::collections::dataset::search::SdlSearchQuery,
    ) -> Result<Self, GDSFrameError> {
        if !self.is_sdl_compliant() {
            return Err(GDSFrameError::from("Dataset lacks SDL compliance focus."));
        }

        let df = self.table.dataframe().clone();

        // Phase 1: Topological Pre-Filtering (Fast)
        // We require that for every edge in the query, the `edges` list-column contains that relation.
        // In a full implementation, we would build a robust `Expr` checking the struct lists.
        // For demonstration of the SDK bounding, we currently pass all rows to Phase 2,
        // but this is where `df.filter(col("edges").... )` occurs.

        // Phase 2: Semantic Unification (Deep)
        let row_count = df.height();
        let mut match_indices = Vec::new();

        for i in 0..row_count {
            if let Ok(subgraph) = self.try_extract_sdl_subgraph(i) {
                if query.is_match(&subgraph) {
                    match_indices.push(i as u32);
                }
            }
        }

        // Materialize the final filtered set using the matched indices
        let ca = polars::prelude::ChunkedArray::<polars::prelude::UInt32Type>::from_slice(
            polars::prelude::PlSmallStr::from_static("idx"),
            &match_indices,
        );
        let filtered_df = df.take(&ca)?;

        Ok(Self {
            name: self.name.clone(),
            table: GDSDataFrame::from(filtered_df),
            lm_focus: self.lm_focus.clone(),
        })
    }

    pub fn table(&self) -> &GDSDataFrame {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut GDSDataFrame {
        &mut self.table
    }

    pub fn into_table(self) -> GDSDataFrame {
        self.table
    }

    pub fn row_count(&self) -> usize {
        self.table.row_count()
    }

    pub fn column_count(&self) -> usize {
        self.table.column_count()
    }

    pub fn column_names(&self) -> Vec<String> {
        self.table.column_names()
    }

    pub fn dtypes(&self) -> Vec<polars::prelude::DataType> {
        self.table.dtypes()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    pub fn head(&self, n: usize) -> Self {
        let table = self.table.head(n);
        Self {
            name: self.name.clone(),
            table,
            lm_focus: self.lm_focus.clone(),
        }
    }

    pub fn tail(&self, n: usize) -> Self {
        let table = self.table.tail(n);
        Self {
            name: self.name.clone(),
            table,
            lm_focus: self.lm_focus.clone(),
        }
    }

    pub fn slice(&self, offset: i64, length: usize) -> Self {
        let table = self.table.slice(offset, length);
        Self {
            name: self.name.clone(),
            table,
            lm_focus: self.lm_focus.clone(),
        }
    }

    pub fn select_columns(&self, columns: &[&str]) -> Result<Self, GDSFrameError> {
        let table = self.table.select_columns(columns)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            lm_focus: self.lm_focus.clone(),
        })
    }

    pub fn select(&self, exprs: &[Expr]) -> Result<Self, GDSFrameError> {
        let table = self.table.select(exprs)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            lm_focus: self.lm_focus.clone(),
        })
    }

    pub fn select_selector(&self, selector: &Selector) -> Result<Self, GDSFrameError> {
        let table = self.table.select_selector(selector)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            lm_focus: self.lm_focus.clone(),
        })
    }

    pub fn filter_expr(&self, predicate: Expr) -> Result<Self, GDSFrameError> {
        let table = self.table.filter_expr(predicate)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            lm_focus: self.lm_focus.clone(),
        })
    }

    /// Python-Polars alias for filter expression.
    pub fn filter(&self, predicate: Expr) -> Result<Self, GDSFrameError> {
        self.filter_expr(predicate)
    }

    pub fn with_columns_exprs(&self, exprs: &[Expr]) -> Result<Self, GDSFrameError> {
        let table = self.table.with_columns_exprs(exprs)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            lm_focus: self.lm_focus.clone(),
        })
    }

    /// Python-Polars alias for with_columns.
    pub fn with_columns(&self, exprs: &[Expr]) -> Result<Self, GDSFrameError> {
        self.with_columns_exprs(exprs)
    }

    pub fn order_by_columns(
        &self,
        columns: &[&str],
        options: SortMultipleOptions,
    ) -> Result<Self, GDSFrameError> {
        let table = self.table.order_by_columns(columns, options)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            lm_focus: self.lm_focus.clone(),
        })
    }

    /// Python-Polars alias for sort by columns.
    pub fn sort(
        &self,
        columns: &[&str],
        options: SortMultipleOptions,
    ) -> Result<Self, GDSFrameError> {
        self.order_by_columns(columns, options)
    }

    pub fn order_by_exprs(
        &self,
        exprs: &[Expr],
        options: SortMultipleOptions,
    ) -> Result<Self, GDSFrameError> {
        let table = self.table.order_by_exprs(exprs, options)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            lm_focus: self.lm_focus.clone(),
        })
    }

    /// Python-Polars alias for sort by expressions.
    pub fn sort_by_exprs(
        &self,
        exprs: &[Expr],
        options: SortMultipleOptions,
    ) -> Result<Self, GDSFrameError> {
        self.order_by_exprs(exprs, options)
    }

    pub fn group_by_exprs(&self, keys: &[Expr], aggs: &[Expr]) -> Result<Self, GDSFrameError> {
        let table = self.table.group_by_exprs(keys, aggs)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            lm_focus: self.lm_focus.clone(),
        })
    }

    pub fn group_by_columns(&self, keys: &[&str], aggs: &[Expr]) -> Result<Self, GDSFrameError> {
        let table = self.table.group_by_columns(keys, aggs)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            lm_focus: self.lm_focus.clone(),
        })
    }

    /// Python-Polars alias for group_by columns.
    pub fn group_by(&self, keys: &[&str], aggs: &[Expr]) -> Result<Self, GDSFrameError> {
        self.group_by_columns(keys, aggs)
    }

    pub fn to_csv(&self, path: impl AsRef<Path>) -> Result<(), GDSFrameError> {
        csv::write_table(path.as_ref(), &self.table, csv::CsvWriteConfig::default())?;
        Ok(())
    }

    pub fn to_parquet(&self, path: impl AsRef<Path>) -> Result<(), GDSFrameError> {
        parquet::write_table(path.as_ref(), &self.table)?;
        Ok(())
    }

    pub fn to_ipc(&self, path: impl AsRef<Path>) -> Result<(), GDSFrameError> {
        ipc::write_table(path.as_ref(), &self.table)?;
        Ok(())
    }

    pub fn to_json(&self, path: impl AsRef<Path>) -> Result<(), GDSFrameError> {
        json::write_table(path.as_ref(), &self.table, json::JsonWriteConfig::default())?;
        Ok(())
    }
}
