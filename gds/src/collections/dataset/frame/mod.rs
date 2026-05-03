//! Dataset shell — `DataFrame` namespace.
//!
//! Part of the dataset-side 2×2 matrix that mirrors the DataFrame DSL shell:
//!
//! |        | lazy        | eager        |
//! |--------|-------------|--------------|
//! | scalar | `Expr`      | `Series`     |
//! | frame  | `LazyFrame` | `DataFrame`  |
//!
//! This module provides [`DatasetDataFrameNameSpace`], the eager-frame entry
//! point for the GDS Shell protocol, plus the [`DataFrameDatasetExt`] trait
//! that attaches `.ds()` onto `GDSDataFrame`. The eager frame is the first
//! semantic shell over the DataFrame body: it carries dataset identity/profile
//! context, can realize a [`crate::collections::dataset::dataset::Dataset`],
//! and seeds central pipeline architecture.

use crate::collections::dataframe::{GDSDataFrame, GDSLazyFrame};
use crate::collections::dataset::artifact::{DatasetArtifactKind, DatasetArtifactProfile};
use crate::collections::dataset::dataset::Dataset;
use crate::collections::dataset::expressions::io::DatasetIoExpr;
use crate::collections::dataset::lazy::DatasetLazyFrameNameSpace;
use crate::collections::dataset::toolchain::DatasetPipeline;
use crate::form::ProgramFeatures;
use crate::shell::{GdsShell, ShellProgram};

#[derive(Debug, Clone)]
pub struct DatasetDataFrameNameSpace {
    df: GDSDataFrame,
    name: Option<String>,
    artifact_profile: DatasetArtifactProfile,
    source: Option<DatasetIoExpr>,
}

impl DatasetDataFrameNameSpace {
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
    pub fn lazy(&self) -> DatasetLazyFrameNameSpace {
        let lf = GDSLazyFrame::from(self.df.dataframe().clone());
        DatasetLazyFrameNameSpace::new(lf)
    }
}

pub trait DataFrameDatasetExt {
    fn ds(self) -> DatasetDataFrameNameSpace;
}

impl DataFrameDatasetExt for GDSDataFrame {
    fn ds(self) -> DatasetDataFrameNameSpace {
        DatasetDataFrameNameSpace::new(self)
    }
}
