//! Dataset Object Request Broker.
//!
//! Dataset is a semantic artifact world, not a synonym for one DataFrame. It
//! owns named frame artifacts and language plugins while preserving a primary
//! frame compatibility surface for existing Dataset DSL clients.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;
use std::sync::Arc;

use crate::collections::dataframe::GDSExpr as Expr;
use crate::collections::dataframe::PolarsSortMultipleOptions as SortMultipleOptions;

use crate::collections::backends::polars::PolarsFrameBackend;
use crate::collections::dataframe::selectors::Selector;
use crate::collections::dataframe::table::TableBuilder;
use crate::collections::dataframe::GDSFrameError;
use crate::collections::dataframe::{GDSDataFrame, GDSLazyFrame};
use crate::collections::dataset::core::artifact::{DatasetArtifactKind, DatasetArtifactProfile};
use crate::collections::dataset::core::plugin::{
    DatasetLanguagePlugin, DatasetPluginError, DatasetPluginErrorClass, DatasetPluginRegistry,
    DatasetPluginRequest, DatasetPluginResponse, DatasetPluginValidationReport,
    DatasetPluginValidationRequest,
};
use crate::collections::dataset::dsl::namespaces::is_dataset_namespace_registered;
use crate::collections::dataset::dsl::namespaces::register_corpus_namespace as register_corpus_ns;
use crate::collections::dataset::dsl::namespaces::register_dataset_namespace;
use crate::collections::dataset::dsl::namespaces::NameSpaceError;
use crate::collections::dataset::frame::DatasetDataFrameNs;
use crate::collections::io::{csv, ipc, json, parquet};

const PRIMARY_ARTIFACT_ID: &str = "primary";

#[derive(Debug, Clone)]
pub struct DatasetFrameArtifact {
    artifact_id: String,
    table: GDSDataFrame,
    profile: DatasetArtifactProfile,
}

impl DatasetFrameArtifact {
    pub fn new(
        artifact_id: impl Into<String>,
        table: GDSDataFrame,
        profile: DatasetArtifactProfile,
    ) -> Self {
        Self {
            artifact_id: artifact_id.into(),
            table,
            profile,
        }
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    pub fn table(&self) -> &GDSDataFrame {
        &self.table
    }

    pub fn profile(&self) -> &DatasetArtifactProfile {
        &self.profile
    }
}

/// Oculus-ready Dataset broker over named semantic frame artifacts.
#[derive(Clone)]
pub struct DatasetOrb {
    name: Option<String>,
    primary_artifact_id: String,
    artifacts: BTreeMap<String, DatasetFrameArtifact>,
    plugins: DatasetPluginRegistry,
}

/// Stable public name retained while callers migrate from the obsolete
/// single-table representation to the Dataset ORB.
pub type Dataset = DatasetOrb;

impl fmt::Debug for DatasetOrb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatasetOrb")
            .field("name", &self.name)
            .field("primary_artifact_id", &self.primary_artifact_id)
            .field("artifacts", &self.artifacts)
            .field("plugin_count", &self.plugins.len())
            .finish()
    }
}

impl DatasetOrb {
    pub fn new(table: GDSDataFrame) -> Self {
        let artifact = DatasetFrameArtifact::new(
            PRIMARY_ARTIFACT_ID,
            table,
            DatasetArtifactProfile::default(),
        );
        Self {
            name: None,
            primary_artifact_id: PRIMARY_ARTIFACT_ID.to_string(),
            artifacts: BTreeMap::from([(PRIMARY_ARTIFACT_ID.to_string(), artifact)]),
            plugins: DatasetPluginRegistry::new(),
        }
    }

    pub fn named(name: impl Into<String>, table: GDSDataFrame) -> Self {
        let mut dataset = Self::new(table);
        dataset.name = Some(name.into());
        dataset
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

    pub fn primary_artifact_id(&self) -> &str {
        &self.primary_artifact_id
    }

    pub fn artifacts(&self) -> &BTreeMap<String, DatasetFrameArtifact> {
        &self.artifacts
    }

    pub fn artifact(&self, artifact_id: &str) -> Option<&DatasetFrameArtifact> {
        self.artifacts.get(artifact_id)
    }

    pub fn insert_artifact(
        &mut self,
        artifact_id: impl Into<String>,
        table: GDSDataFrame,
        profile: DatasetArtifactProfile,
    ) -> Option<DatasetFrameArtifact> {
        let artifact_id = artifact_id.into();
        self.artifacts.insert(
            artifact_id.clone(),
            DatasetFrameArtifact::new(artifact_id, table, profile),
        )
    }

    pub fn with_artifact(
        mut self,
        artifact_id: impl Into<String>,
        table: GDSDataFrame,
        profile: DatasetArtifactProfile,
    ) -> Self {
        self.insert_artifact(artifact_id, table, profile);
        self
    }

    pub fn plugins(&self) -> &DatasetPluginRegistry {
        &self.plugins
    }

    pub(crate) fn register_plugin(
        &mut self,
        plugin: Arc<dyn DatasetLanguagePlugin>,
    ) -> Result<(), DatasetPluginError> {
        self.plugins.register(plugin)
    }

    pub fn validate_plugin(
        &self,
        plugin_id: &str,
        request: &DatasetPluginValidationRequest,
    ) -> Result<DatasetPluginValidationReport, DatasetPluginError> {
        self.plugins.validate(plugin_id, request)
    }

    /// Dispatch background plugin work and atomically admit successful outputs.
    pub(crate) fn dispatch(
        &mut self,
        request: &DatasetPluginRequest,
    ) -> Result<DatasetPluginResponse, DatasetPluginError> {
        for artifact_id in &request.input_artifact_ids {
            if !self.artifacts.contains_key(artifact_id) {
                return Err(DatasetPluginError::new(
                    DatasetPluginErrorClass::MissingArtifact,
                    format!("input artifact {artifact_id} is not present"),
                ));
            }
        }

        let response = self.plugins.execute(request)?;
        if !response.report.passed {
            return Ok(response);
        }

        for artifact in &response.artifacts {
            if self.artifacts.contains_key(&artifact.artifact_id) {
                return Err(DatasetPluginError::new(
                    DatasetPluginErrorClass::ArtifactConflict,
                    format!("output artifact {} already exists", artifact.artifact_id),
                ));
            }
        }

        for artifact in &response.artifacts {
            self.insert_artifact(
                artifact.artifact_id.clone(),
                artifact.table.clone(),
                artifact.profile.clone(),
            );
        }

        Ok(response)
    }

    pub fn polars_backend(&self) -> PolarsFrameBackend {
        PolarsFrameBackend::from_dataframe(self.table().clone())
    }

    fn primary_artifact(&self) -> &DatasetFrameArtifact {
        self.artifacts
            .get(&self.primary_artifact_id)
            .expect("DatasetOrb invariant: primary artifact must exist")
    }

    fn primary_artifact_mut(&mut self) -> &mut DatasetFrameArtifact {
        self.artifacts
            .get_mut(&self.primary_artifact_id)
            .expect("DatasetOrb invariant: primary artifact must exist")
    }

    fn with_primary_table(&self, table: GDSDataFrame) -> Self {
        let mut next = self.clone();
        next.primary_artifact_mut().table = table;
        next
    }

    /// Register a custom dataset namespace from the root Dataset API.
    pub fn register_namespace(name: &str) -> Result<(), NameSpaceError> {
        register_dataset_namespace(name)
    }

    /// Register the canonical dataset `corpus` namespace from the root API.
    pub fn register_corpus_namespace() -> Result<(), NameSpaceError> {
        register_corpus_ns()
    }

    /// Check dataset namespace registration via the root API.
    pub fn is_namespace_registered(name: &str) -> bool {
        is_dataset_namespace_registered(name)
    }

    pub fn artifact_profile(&self) -> &DatasetArtifactProfile {
        self.primary_artifact().profile()
    }

    pub fn artifact_kind(&self) -> &DatasetArtifactKind {
        self.artifact_profile().primary_kind()
    }

    pub fn with_artifact_profile(mut self, artifact_profile: DatasetArtifactProfile) -> Self {
        self.primary_artifact_mut().profile = artifact_profile;
        self
    }

    pub fn with_artifact_kind(mut self, artifact_kind: DatasetArtifactKind) -> Self {
        let profile = self
            .artifact_profile()
            .clone()
            .with_primary_kind(artifact_kind);
        self.primary_artifact_mut().profile = profile;
        self
    }

    pub fn with_artifact_facet(mut self, facet: impl Into<String>) -> Self {
        let profile = self.artifact_profile().clone().with_facet(facet);
        self.primary_artifact_mut().profile = profile;
        self
    }

    pub fn has_artifact_kind(&self, artifact_kind: &DatasetArtifactKind) -> bool {
        self.artifact_profile().has_kind(artifact_kind)
    }

    pub fn has_artifact_facet(&self, facet: &str) -> bool {
        self.artifact_profile().has_facet(facet)
    }

    pub fn table(&self) -> &GDSDataFrame {
        self.primary_artifact().table()
    }

    pub fn table_mut(&mut self) -> &mut GDSDataFrame {
        &mut self.primary_artifact_mut().table
    }

    pub fn into_table(mut self) -> GDSDataFrame {
        self.artifacts
            .remove(&self.primary_artifact_id)
            .expect("DatasetOrb invariant: primary artifact must exist")
            .table
    }

    /// Enter the focused DataFrame control namespace for this Dataset body.
    pub fn frame(&self) -> DatasetDataFrameNs {
        let mut frame = DatasetDataFrameNs::new(self.table().clone())
            .artifact_kind(self.artifact_kind().clone());
        if let Some(name) = self.name() {
            frame = frame.named(name);
        }
        for facet in self.artifact_profile().facets() {
            frame = frame.facet(facet.clone());
        }
        frame
    }

    /// Build a lazy query graph from this dataset's semantic table.
    pub fn lazy(&self) -> GDSLazyFrame {
        self.table().lazy()
    }

    pub fn row_count(&self) -> usize {
        self.table().row_count()
    }

    pub fn column_count(&self) -> usize {
        self.table().column_count()
    }

    pub fn column_names(&self) -> Vec<String> {
        self.table().column_names()
    }

    pub fn dtypes(&self) -> Vec<polars::prelude::DataType> {
        self.table().dtypes()
    }

    pub fn is_empty(&self) -> bool {
        self.table().is_empty()
    }

    pub fn head(&self, n: usize) -> Self {
        self.with_primary_table(self.table().head(n))
    }

    pub fn tail(&self, n: usize) -> Self {
        self.with_primary_table(self.table().tail(n))
    }

    pub fn slice(&self, offset: i64, length: usize) -> Self {
        self.with_primary_table(self.table().slice(offset, length))
    }

    pub fn select_columns(&self, columns: &[&str]) -> Result<Self, GDSFrameError> {
        Ok(self.with_primary_table(self.table().select_columns(columns)?))
    }

    pub fn select(&self, exprs: &[Expr]) -> Result<Self, GDSFrameError> {
        Ok(self.with_primary_table(self.table().select(exprs)?))
    }

    pub fn select_selector(&self, selector: &Selector) -> Result<Self, GDSFrameError> {
        Ok(self.with_primary_table(self.table().select_selector(selector)?))
    }

    pub fn filter_expr(&self, predicate: Expr) -> Result<Self, GDSFrameError> {
        Ok(self.with_primary_table(self.table().filter_expr(predicate)?))
    }

    /// Python-Polars alias for filter expression.
    pub fn filter(&self, predicate: Expr) -> Result<Self, GDSFrameError> {
        self.filter_expr(predicate)
    }

    pub fn with_columns_exprs(&self, exprs: &[Expr]) -> Result<Self, GDSFrameError> {
        Ok(self.with_primary_table(self.table().with_columns_exprs(exprs)?))
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
        Ok(self.with_primary_table(self.table().order_by_columns(columns, options)?))
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
        Ok(self.with_primary_table(self.table().order_by_exprs(exprs, options)?))
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
        Ok(self.with_primary_table(self.table().group_by_exprs(keys, aggs)?))
    }

    pub fn group_by_columns(&self, keys: &[&str], aggs: &[Expr]) -> Result<Self, GDSFrameError> {
        Ok(self.with_primary_table(self.table().group_by_columns(keys, aggs)?))
    }

    /// Python-Polars alias for group_by columns.
    pub fn group_by(&self, keys: &[&str], aggs: &[Expr]) -> Result<Self, GDSFrameError> {
        self.group_by_columns(keys, aggs)
    }

    pub fn to_csv(&self, path: impl AsRef<Path>) -> Result<(), GDSFrameError> {
        csv::write_table(path.as_ref(), self.table(), csv::CsvWriteConfig::default())?;
        Ok(())
    }

    pub fn to_parquet(&self, path: impl AsRef<Path>) -> Result<(), GDSFrameError> {
        parquet::write_table(path.as_ref(), self.table())?;
        Ok(())
    }

    pub fn to_ipc(&self, path: impl AsRef<Path>) -> Result<(), GDSFrameError> {
        ipc::write_table(path.as_ref(), self.table())?;
        Ok(())
    }

    pub fn to_json(&self, path: impl AsRef<Path>) -> Result<(), GDSFrameError> {
        json::write_table(
            path.as_ref(),
            self.table(),
            json::JsonWriteConfig::default(),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use polars::df;

    use super::*;
    use crate::collections::dataset::oculus::OculusHandle;

    struct OculusProbePlugin;

    impl DatasetLanguagePlugin for OculusProbePlugin {
        fn plugin_id(&self) -> &'static str {
            "oculus.probe"
        }

        fn language_id(&self) -> &'static str {
            "oculus"
        }

        fn validate(
            &self,
            request: &DatasetPluginValidationRequest,
        ) -> Result<DatasetPluginValidationReport, DatasetPluginError> {
            Ok(DatasetPluginValidationReport::success(
                self.plugin_id(),
                format!("accepted {}", request.workflow_id),
            ))
        }
    }

    #[test]
    fn test_dataset_root_namespace_registration_bridge() {
        let ns = "dataset-root-test-ns";

        Dataset::register_namespace(ns).expect("custom namespace registration should succeed");

        assert!(Dataset::is_namespace_registered(ns));
    }

    #[test]
    fn test_dataset_root_reserved_namespace_is_rejected() {
        let err = Dataset::register_corpus_namespace()
            .expect_err("reserved corpus namespace should be rejected by dataset registry");

        assert!(matches!(err, NameSpaceError::Reserved { .. }));
    }

    #[test]
    fn dataset_orb_holds_multiple_named_artifacts() {
        let primary = GDSDataFrame::new(df!("token" => ["sat", "cit"]).unwrap());
        let features = GDSDataFrame::new(df!("feature" => ["lemma", "sense"]).unwrap());
        let dataset = DatasetOrb::named("oculus", primary).with_artifact(
            "features",
            features,
            DatasetArtifactProfile::new(DatasetArtifactKind::FeatureMap),
        );

        assert_eq!(dataset.artifacts().len(), 2);
        assert_eq!(dataset.primary_artifact_id(), PRIMARY_ARTIFACT_ID);
        assert_eq!(
            dataset
                .artifact("features")
                .unwrap()
                .profile()
                .primary_kind(),
            &DatasetArtifactKind::FeatureMap
        );
    }

    #[test]
    fn dataset_orb_brokers_plugins_and_exposes_polars_backend() {
        let frame = GDSDataFrame::new(df!("text" => ["meaning emerges"]).unwrap());
        let mut dataset = DatasetOrb::named("oculus", frame);
        OculusHandle::new()
            .register_plugin(&mut dataset, Arc::new(OculusProbePlugin))
            .unwrap();

        let request = DatasetPluginValidationRequest::new(
            "oculus",
            "oculus.showtime",
            crate::collections::dataset::core::plugin::DatasetPluginPayload::typed(()),
        );
        let report = dataset.validate_plugin("oculus.probe", &request).unwrap();

        assert!(report.passed);
        assert_eq!(
            dataset.polars_backend().backend(),
            crate::config::CollectionsBackend::Polars
        );
    }
}
