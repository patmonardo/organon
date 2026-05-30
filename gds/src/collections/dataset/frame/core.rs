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

use std::fmt;

use crate::collections::dataframe::datatypes::register_extension_type;
use crate::collections::dataframe::datatypes::ExtensionRegistryError;
use crate::collections::dataframe::namespaces::register_dataframe_namespace;
use crate::collections::dataframe::namespaces::register_expr_namespace;
use crate::collections::dataframe::namespaces::register_lazyframe_namespace;
use crate::collections::dataframe::namespaces::register_series_namespace;
use crate::collections::dataframe::namespaces::NameSpaceError as DataFrameNameSpaceError;
use crate::collections::dataframe::{GDSDataFrame, GDSLazyFrame};
use crate::collections::dataset::core::artifact::{DatasetArtifactKind, DatasetArtifactProfile};
use crate::collections::dataset::core::dataset::Dataset;
use crate::collections::dataset::dsl::namespaces::register_dataset_namespace;
use crate::collections::dataset::dsl::namespaces::NameSpaceError as DatasetNameSpaceError;
use crate::collections::dataset::frame::lazy::DatasetLazyFrameNs;
use crate::collections::dataset::lab::protocol::io::DatasetIoExpr;
use crate::collections::dataset::lab::toolchain::DatasetPipeline;
use crate::form::ProgramFeatures;
use crate::shell::{GdsShell, ShellProgram};

pub const DATASET_FRAMING_NAMESPACE: &str = "framing";
pub const DATAFRAME_FRAMING_NAMESPACE: &str = "frame";
pub const DATASET_FRAME_EXTENSION: &str = "gds.dataset.frame";
pub const DATASET_FRAME_STORAGE_EXTENSION: &str = "gds.dataset.frame.storage";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatasetFramingError {
    DataFrameNamespace(DataFrameNameSpaceError),
    DatasetNamespace(DatasetNameSpaceError),
    Extension(ExtensionRegistryError),
}

impl fmt::Display for DatasetFramingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DataFrameNamespace(error) => write!(f, "dataframe namespace error: {error:?}"),
            Self::DatasetNamespace(error) => write!(f, "dataset namespace error: {error:?}"),
            Self::Extension(error) => write!(f, "dataset framing extension error: {error}"),
        }
    }
}

impl std::error::Error for DatasetFramingError {}

impl From<DataFrameNameSpaceError> for DatasetFramingError {
    fn from(error: DataFrameNameSpaceError) -> Self {
        Self::DataFrameNamespace(error)
    }
}

impl From<DatasetNameSpaceError> for DatasetFramingError {
    fn from(error: DatasetNameSpaceError) -> Self {
        Self::DatasetNamespace(error)
    }
}

impl From<ExtensionRegistryError> for DatasetFramingError {
    fn from(error: ExtensionRegistryError) -> Self {
        Self::Extension(error)
    }
}

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

    pub fn register_framing_surface() -> Result<(), DatasetFramingError> {
        register_dataset_namespace(DATASET_FRAMING_NAMESPACE)?;
        register_dataframe_namespace(DATAFRAME_FRAMING_NAMESPACE)?;
        register_lazyframe_namespace(DATAFRAME_FRAMING_NAMESPACE)?;
        register_series_namespace(DATAFRAME_FRAMING_NAMESPACE)?;
        register_expr_namespace(DATAFRAME_FRAMING_NAMESPACE)?;
        register_extension_type_once(DATASET_FRAME_EXTENSION, Some("DatasetFrame"), false)?;
        register_extension_type_once(DATASET_FRAME_STORAGE_EXTENSION, None, true)?;
        Ok(())
    }

    pub fn ensure_framing_registered(&self) -> Result<(), DatasetFramingError> {
        Self::register_framing_surface()
    }

    pub fn with_registered_framing(self) -> Result<Self, DatasetFramingError> {
        Self::register_framing_surface()?;
        Ok(self)
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

fn register_extension_type_once(
    ext_name: &str,
    ext_class: Option<&str>,
    as_storage: bool,
) -> Result<(), DatasetFramingError> {
    match register_extension_type(ext_name, ext_class, as_storage) {
        Ok(()) | Err(ExtensionRegistryError::AlreadyRegistered(_)) => Ok(()),
        Err(error) => Err(error.into()),
    }
}

pub trait DataFrameDatasetExt {
    /// Enter the Dataset DataFrame namespace using a short alias.
    ///
    /// `ds` stands for "dataset shell".
    fn ds(self) -> DatasetDataFrameNs;

    /// Enter the Dataset DataFrame namespace using the explicit long form.
    fn dataset(self) -> DatasetDataFrameNs;

    /// Enter the Dataset DataFrame namespace with an explicit frame-oriented
    /// name for autocomplete-first workflows.
    fn dataset_frame(self) -> DatasetDataFrameNs;

    /// Alias for [`DataFrameDatasetExt::dataset_frame`].
    fn frame_ns(self) -> DatasetDataFrameNs;
}

impl DataFrameDatasetExt for GDSDataFrame {
    fn ds(self) -> DatasetDataFrameNs {
        DatasetDataFrameNs::new(self)
    }

    fn dataset(self) -> DatasetDataFrameNs {
        DatasetDataFrameNs::new(self)
    }

    fn dataset_frame(self) -> DatasetDataFrameNs {
        DatasetDataFrameNs::new(self)
    }

    fn frame_ns(self) -> DatasetDataFrameNs {
        DatasetDataFrameNs::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataframe::datatypes::get_extension_type;
    use crate::collections::dataframe::datatypes::ExtensionType;
    use crate::collections::dataframe::namespaces::is_dataframe_namespace_registered;
    use crate::collections::dataframe::namespaces::is_expr_namespace_registered;
    use crate::collections::dataframe::namespaces::is_lazyframe_namespace_registered;
    use crate::collections::dataframe::namespaces::is_series_namespace_registered;
    use crate::collections::dataset::dsl::namespaces::is_dataset_namespace_registered;
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

    #[test]
    fn test_dataframe_extension_aliases_match_short_form() -> Result<(), Box<dyn Error>> {
        let df_short = tbl!((x: i64 => [1, 2, 3]))?;
        let df_explicit = tbl!((x: i64 => [1, 2, 3]))?;
        let df_ns = tbl!((x: i64 => [1, 2, 3]))?;

        let short = df_short.ds().into_dataset();
        let explicit = df_explicit.dataset_frame().into_dataset();
        let ns_alias = df_ns.frame_ns().into_dataset();

        assert_eq!(short.row_count(), explicit.row_count());
        assert_eq!(short.column_names(), explicit.column_names());
        assert_eq!(short.row_count(), ns_alias.row_count());
        assert_eq!(short.column_names(), ns_alias.column_names());
        Ok(())
    }

    #[test]
    fn test_dataset_framing_registers_proxy_surface() -> Result<(), Box<dyn Error>> {
        let frame = tbl!((x: i64 => [1, 2, 3]))?
            .ds()
            .with_registered_framing()?;

        assert_eq!(frame.dataframe().shape(), (3, 1));
        assert!(is_dataset_namespace_registered(DATASET_FRAMING_NAMESPACE));
        assert!(is_dataframe_namespace_registered(
            DATAFRAME_FRAMING_NAMESPACE
        ));
        assert!(is_lazyframe_namespace_registered(
            DATAFRAME_FRAMING_NAMESPACE
        ));
        assert!(is_series_namespace_registered(DATAFRAME_FRAMING_NAMESPACE));
        assert!(is_expr_namespace_registered(DATAFRAME_FRAMING_NAMESPACE));
        assert_eq!(
            get_extension_type(DATASET_FRAME_EXTENSION),
            Some(ExtensionType::Class("DatasetFrame".to_string()))
        );
        assert_eq!(
            get_extension_type(DATASET_FRAME_STORAGE_EXTENSION),
            Some(ExtensionType::Storage)
        );
        Ok(())
    }

    #[test]
    fn test_dataset_frame_preserves_dataset_profile() -> Result<(), Box<dyn Error>> {
        let dataset = Dataset::named("curated-fixture", tbl!((x: i64 => [1, 2, 3]))?)
            .with_artifact_kind(DatasetArtifactKind::FeatureMap)
            .with_artifact_facet("framing");

        let frame = dataset.frame();

        assert_eq!(frame.name(), Some("curated-fixture"));
        assert_eq!(
            frame.artifact_profile().primary_kind(),
            &DatasetArtifactKind::FeatureMap
        );
        assert!(frame.artifact_profile().has_facet("framing"));
        Ok(())
    }
}
