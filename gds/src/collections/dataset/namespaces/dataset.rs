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
//! - **Feature algebra** lives in [`super::feature::FeatureNs`] and
//!   [`super::feature::FeatureExprNameSpace`].
//! - **Tree algebra** lives in [`super::tree::TreeNs`].
//! - **Per-column text expressions** (tokenize / lowercase / token-count)
//!   live on the dataset DSL shell at
//!   [`crate::collections::dataset::expr::DatasetExprNameSpace::text`].
//!
//! Keep this struct narrowly scoped to dataset-level orchestration concerns.

use crate::collections::dataset::expressions::io::DatasetIoExpr;
use crate::collections::dataset::expressions::metadata::DatasetMetadataExpr;
use crate::collections::dataset::expressions::projection::DatasetProjectionExpr;
use crate::collections::dataset::expressions::registry::DatasetRegistryExpr;
use crate::collections::dataset::expressions::reporting::DatasetReportExpr;

#[derive(Debug, Clone, Default)]
pub struct DatasetNs;

impl DatasetNs {
    // ---- Registry ----------------------------------------------------------

    pub fn registry(name: impl Into<String>) -> DatasetRegistryExpr {
        DatasetRegistryExpr::new(name)
    }

    pub fn registry_versioned(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> DatasetRegistryExpr {
        DatasetRegistryExpr::versioned(name, version)
    }

    // ---- IO ----------------------------------------------------------------

    pub fn io_path(path: impl Into<String>) -> DatasetIoExpr {
        DatasetIoExpr::from_path(path)
    }

    pub fn io_url(url: impl Into<String>) -> DatasetIoExpr {
        DatasetIoExpr::from_url(url)
    }

    // ---- Metadata ----------------------------------------------------------

    pub fn metadata(
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> DatasetMetadataExpr {
        DatasetMetadataExpr::new(key, value)
    }

    // ---- Projection --------------------------------------------------------

    pub fn project_text(columns: Vec<String>) -> DatasetProjectionExpr {
        DatasetProjectionExpr::text(columns)
    }

    pub fn project_corpus(columns: Vec<String>) -> DatasetProjectionExpr {
        DatasetProjectionExpr::corpus(columns)
    }

    pub fn project_graph(columns: Vec<String>) -> DatasetProjectionExpr {
        DatasetProjectionExpr::graph(columns)
    }

    // ---- Reporting ---------------------------------------------------------

    pub fn report_summary() -> DatasetReportExpr {
        DatasetReportExpr::summary()
    }

    pub fn report_profile() -> DatasetReportExpr {
        DatasetReportExpr::profile()
    }
}
