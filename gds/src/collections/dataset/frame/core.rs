//! Dataset shell — `DataFrame` namespace.
//!
//! Part of the dataset-side 2×2 matrix that mirrors the DataFrame DSL shell:
//!
//! |        | lazy        | eager        |
//! |--------|-------------|--------------|
//! | scalar | `Expr`      | `Series`     |
//! | frame  | `LazyFrame` | `DataFrame`  |
//!
//! This module provides [`DatasetDataFrameNs`], the eager-frame entry
//! point for the GDS Shell protocol, plus the [`DataFrameDatasetExt`] trait
//! that attaches `.ds()` and `.dataset()` onto `GDSDataFrame`. The eager frame is the first
//! semantic shell over the DataFrame body: it carries dataset identity/profile
//! context, can realize a [`crate::collections::dataset::core::dataset::Dataset`],
//! and seeds central pipeline architecture.

use crate::collections::dataframe::{GDSDataFrame, GDSLazyFrame};
use crate::collections::dataset::core::artifact::{DatasetArtifactKind, DatasetArtifactProfile};
use crate::collections::dataset::core::dataset::Dataset;
use crate::collections::dataset::frame::lazy::DatasetLazyFrameNs;
use crate::collections::dataset::lab::protocol::io::DatasetIoExpr;
use crate::collections::dataset::lab::toolchain::DatasetPipeline;
use crate::form::ProgramFeatures;
use crate::shell::{GdsShell, ShellProgram};

#[derive(Debug, Clone)]
pub struct DatasetDataFrameNs {
    df: GDSDataFrame,
    name: Option<String>,
    artifact_profile: DatasetArtifactProfile,
    source: Option<DatasetIoExpr>,
}

impl DatasetDataFrameNs {
    pub fn new(df: GDSDataFrame) -> Self {
        Self {
            df,
            name: None,
            artifact_profile: DatasetArtifactProfile::default(),
            source: None,
        }
    }

    pub fn dataframe(&self) -> &GDSDataFrame {
        &self.df
    }

    pub fn into_dataframe(self) -> GDSDataFrame {
        self.df
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn artifact_profile(&self) -> &DatasetArtifactProfile {
        &self.artifact_profile
    }

    pub fn source(&self) -> Option<&DatasetIoExpr> {
        self.source.as_ref()
    }

    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn artifact_kind(mut self, kind: DatasetArtifactKind) -> Self {
        self.artifact_profile = self.artifact_profile.with_primary_kind(kind);
        self
    }

    pub fn facet(mut self, facet: impl Into<String>) -> Self {
        self.artifact_profile = self.artifact_profile.with_facet(facet);
        self
    }

    pub fn source_io(mut self, source: DatasetIoExpr) -> Self {
        self.source = Some(source);
        self
    }

    pub fn into_dataset(self) -> Dataset {
        let dataset = if let Some(name) = self.name {
            Dataset::named(name, self.df)
        } else {
            Dataset::new(self.df)
        };

        dataset.with_artifact_profile(self.artifact_profile)
    }

    pub fn into_shell(self, program: ShellProgram) -> GdsShell {
        GdsShell::from_dataset(self.into_dataset()).with_program(program)
    }

    pub fn into_shell_with_program_features(self, program_features: ProgramFeatures) -> GdsShell {
        GdsShell::from_dataset(self.into_dataset()).with_program_features(program_features)
    }

    pub fn pipeline(&self) -> DatasetPipeline {
        let mut pipeline = DatasetPipeline::new();
        if let Some(source) = self.source.clone() {
            pipeline = pipeline.with_io(source);
        }
        if let Some(name) = &self.name {
            pipeline = pipeline.with_metadata(
                crate::collections::dataset::DatasetMetadataExpr::new("dataset.name", name.clone()),
            );
        }
        pipeline
    }

    /// Convert this DataFrame into a LazyFrame and enter the dataset lazy namespace.
    pub fn lazy(&self) -> DatasetLazyFrameNs {
        let lf = GDSLazyFrame::from(self.df.dataframe().clone());
        DatasetLazyFrameNs::new(lf)
    }
}

pub trait DataFrameDatasetExt {
    fn ds(self) -> DatasetDataFrameNs;
    fn dataset(self) -> DatasetDataFrameNs;
}

impl DataFrameDatasetExt for GDSDataFrame {
    fn ds(self) -> DatasetDataFrameNs {
        DatasetDataFrameNs::new(self)
    }

    fn dataset(self) -> DatasetDataFrameNs {
        DatasetDataFrameNs::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tbl;
    use std::error::Error;

    #[test]
    fn test_dataframe_extension_alias_matches_short_form() -> Result<(), Box<dyn Error>> {
        let df_left = tbl!((x: i64 => [1, 2, 3]))?;
        let df_right = tbl!((x: i64 => [1, 2, 3]))?;

        let left = df_left.ds().into_dataset();
        let right = df_right.dataset().into_dataset();

        assert_eq!(left.row_count(), right.row_count());
        assert_eq!(left.column_names(), right.column_names());
        Ok(())
    }
}
