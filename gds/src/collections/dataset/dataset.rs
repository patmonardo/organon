//! High-level Dataset wrapper over Polars-backed tables.
//!
//! This is intentionally lightweight and ergonomic for “dataset-first” workflows
//! without forcing callers to pull in Polars types directly.

use std::path::Path;

use polars::prelude::{Expr, SortMultipleOptions};

use crate::collections::dataframe::selectors::Selector;
use crate::collections::dataframe::table::TableBuilder;
use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataframe::GDSFrameError;
use crate::collections::dataset::artifact::{DatasetArtifactKind, DatasetArtifactProfile};
use crate::collections::io::{csv, ipc, json, parquet};

/// Minimal dataset wrapper (Polars-backed).
#[derive(Debug, Clone)]
pub struct Dataset {
    name: Option<String>,
    table: GDSDataFrame,
    artifact_profile: DatasetArtifactProfile,
}

impl Dataset {
    pub fn new(table: GDSDataFrame) -> Self {
        Self {
            name: None,
            table,
            artifact_profile: DatasetArtifactProfile::default(),
        }
    }

    pub fn named(name: impl Into<String>, table: GDSDataFrame) -> Self {
        Self {
            name: Some(name.into()),
            table,
            artifact_profile: DatasetArtifactProfile::default(),
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

    pub fn artifact_profile(&self) -> &DatasetArtifactProfile {
        &self.artifact_profile
    }

    pub fn artifact_kind(&self) -> &DatasetArtifactKind {
        self.artifact_profile.primary_kind()
    }

    pub fn with_artifact_profile(mut self, artifact_profile: DatasetArtifactProfile) -> Self {
        self.artifact_profile = artifact_profile;
        self
    }

    pub fn with_artifact_kind(mut self, artifact_kind: DatasetArtifactKind) -> Self {
        self.artifact_profile = self
            .artifact_profile
            .clone()
            .with_primary_kind(artifact_kind);
        self
    }

    pub fn with_artifact_facet(mut self, facet: impl Into<String>) -> Self {
        self.artifact_profile = self.artifact_profile.clone().with_facet(facet);
        self
    }

    pub fn has_artifact_kind(&self, artifact_kind: &DatasetArtifactKind) -> bool {
        self.artifact_profile.has_kind(artifact_kind)
    }

    pub fn has_artifact_facet(&self, facet: &str) -> bool {
        self.artifact_profile.has_facet(facet)
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
            artifact_profile: self.artifact_profile.clone(),
        }
    }

    pub fn tail(&self, n: usize) -> Self {
        let table = self.table.tail(n);
        Self {
            name: self.name.clone(),
            table,
            artifact_profile: self.artifact_profile.clone(),
        }
    }

    pub fn slice(&self, offset: i64, length: usize) -> Self {
        let table = self.table.slice(offset, length);
        Self {
            name: self.name.clone(),
            table,
            artifact_profile: self.artifact_profile.clone(),
        }
    }

    pub fn select_columns(&self, columns: &[&str]) -> Result<Self, GDSFrameError> {
        let table = self.table.select_columns(columns)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            artifact_profile: self.artifact_profile.clone(),
        })
    }

    pub fn select(&self, exprs: &[Expr]) -> Result<Self, GDSFrameError> {
        let table = self.table.select(exprs)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            artifact_profile: self.artifact_profile.clone(),
        })
    }

    pub fn select_selector(&self, selector: &Selector) -> Result<Self, GDSFrameError> {
        let table = self.table.select_selector(selector)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            artifact_profile: self.artifact_profile.clone(),
        })
    }

    pub fn filter_expr(&self, predicate: Expr) -> Result<Self, GDSFrameError> {
        let table = self.table.filter_expr(predicate)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            artifact_profile: self.artifact_profile.clone(),
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
            artifact_profile: self.artifact_profile.clone(),
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
            artifact_profile: self.artifact_profile.clone(),
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
            artifact_profile: self.artifact_profile.clone(),
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
            artifact_profile: self.artifact_profile.clone(),
        })
    }

    pub fn group_by_columns(&self, keys: &[&str], aggs: &[Expr]) -> Result<Self, GDSFrameError> {
        let table = self.table.group_by_columns(keys, aggs)?;
        Ok(Self {
            name: self.name.clone(),
            table,
            artifact_profile: self.artifact_profile.clone(),
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
