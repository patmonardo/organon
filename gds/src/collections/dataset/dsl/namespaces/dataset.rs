//! Dataset orchestration namespace.
//!
//! `DatasetNs` is the top-level orchestration façade for declarative dataset
//! expressions: **what dataset is this, where does it come from, how is it
//! described, what is projected out of it, and how is it reported on**.
//!
//! It deliberately does **not** duplicate the specialist builder namespaces:
//!
//! - **Data-op authoring** lives in [`super::dataop::DataOpNs`] (and its
//!   text-domain alias [`super::text::TextNs`]).
//! - **Essence authoring** lives in [`super::model::ModelNs`],
//!   [`super::feature::FeatureNs`], and [`super::plan::PlanNs`].
//! - **Tree algebra** lives in [`super::treens::TreeNs`].
//! - **Per-column text expressions** (tokenize / lowercase / token-count)
//!   live on the dataset DSL shell at
//!   [`crate::collections::dataset::frame::expr::DatasetExprNs::text`].
//!
//! Keep this struct narrowly scoped to dataset-level orchestration concerns.

use crate::collections::dataset::dsl::functions;
use crate::collections::dataset::lab::protocol::io::DatasetIoExpr;
use crate::collections::dataset::lab::protocol::metadata::DatasetMetadataExpr;
use crate::collections::dataset::lab::protocol::projection::DatasetProjectionExpr;
use crate::collections::dataset::lab::protocol::registry::DatasetRegistryExpr;
use crate::collections::dataset::lab::protocol::reporting::DatasetReportExpr;

#[derive(Debug, Clone, Default)]
pub struct DatasetNs;

impl DatasetNs {
    // ---- Registry ----------------------------------------------------------

    pub fn registry(name: impl Into<String>) -> DatasetRegistryExpr {
        functions::registry(name)
    }

    pub fn registry_versioned(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> DatasetRegistryExpr {
        functions::registry_versioned(name, version)
    }

    // ---- IO ----------------------------------------------------------------

    pub fn io_path(path: impl Into<String>) -> DatasetIoExpr {
        functions::io_path(path)
    }

    pub fn io_url(url: impl Into<String>) -> DatasetIoExpr {
        functions::io_url(url)
    }

    // ---- Metadata ----------------------------------------------------------

    pub fn metadata(
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> DatasetMetadataExpr {
        functions::metadata(key, value)
    }

    // ---- Projection --------------------------------------------------------

    pub fn project_text(columns: Vec<String>) -> DatasetProjectionExpr {
        functions::project_text(columns)
    }

    pub fn project_corpus(columns: Vec<String>) -> DatasetProjectionExpr {
        functions::project_corpus(columns)
    }

    pub fn project_graph(columns: Vec<String>) -> DatasetProjectionExpr {
        functions::project_graph(columns)
    }

    // ---- Reporting ---------------------------------------------------------

    pub fn report_summary() -> DatasetReportExpr {
        functions::report_summary()
    }

    pub fn report_profile() -> DatasetReportExpr {
        functions::report_profile()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::lab::protocol::io::DatasetSource;
    use crate::collections::dataset::lab::protocol::projection::DatasetProjectionKind;
    use crate::collections::dataset::lab::protocol::reporting::DatasetReportKind;

    #[test]
    fn test_dataset_ns_builders_match_expected_shapes() {
        let reg = DatasetNs::registry_versioned("main", "v1");
        assert_eq!(reg.name(), "main");
        assert_eq!(reg.version(), Some("v1"));

        let io = DatasetNs::io_path("/tmp/corpus.txt");
        assert!(matches!(io.source(), DatasetSource::Path(path) if path == "/tmp/corpus.txt"));

        let meta = DatasetNs::metadata("k", serde_json::json!("v"));
        assert_eq!(meta.key(), "k");

        let proj = DatasetNs::project_text(vec!["text".to_string(), "tokens".to_string()]);
        assert!(matches!(proj.kind(), DatasetProjectionKind::TextToFrame));
        assert_eq!(proj.columns().len(), 2);

        let report = DatasetNs::report_summary();
        assert!(matches!(report.kind(), DatasetReportKind::Summary));
    }
}
