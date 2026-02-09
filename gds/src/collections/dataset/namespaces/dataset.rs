//! Dataset namespace for dataset-level expression builders.
//!
//! These helpers create declarative dataset expressions that can later be
//! interpreted by higher-level Dataset planners.

use crate::collections::dataset::expressions::etl::DatasetEtlExpr;
use crate::collections::dataset::expressions::io::DatasetIoExpr;
use crate::collections::dataset::expressions::metadata::DatasetMetadataExpr;
use crate::collections::dataset::expressions::projection::DatasetProjectionExpr;
use crate::collections::dataset::expressions::registry::DatasetRegistryExpr;
use crate::collections::dataset::expressions::reporting::DatasetReportExpr;

#[derive(Debug, Clone, Default)]
pub struct DatasetNs;

impl DatasetNs {
    pub fn registry(name: impl Into<String>) -> DatasetRegistryExpr {
        DatasetRegistryExpr::new(name)
    }

    pub fn registry_versioned(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> DatasetRegistryExpr {
        DatasetRegistryExpr::versioned(name, version)
    }

    pub fn io_path(path: impl Into<String>) -> DatasetIoExpr {
        DatasetIoExpr::from_path(path)
    }

    pub fn io_url(url: impl Into<String>) -> DatasetIoExpr {
        DatasetIoExpr::from_url(url)
    }

    pub fn metadata(key: impl Into<String>, value: impl Into<serde_json::Value>) -> DatasetMetadataExpr {
        DatasetMetadataExpr::new(key, value)
    }

    pub fn project_text(columns: Vec<String>) -> DatasetProjectionExpr {
        DatasetProjectionExpr::text(columns)
    }

    pub fn project_corpus(columns: Vec<String>) -> DatasetProjectionExpr {
        DatasetProjectionExpr::corpus(columns)
    }

    pub fn project_graph(columns: Vec<String>) -> DatasetProjectionExpr {
        DatasetProjectionExpr::graph(columns)
    }

    pub fn etl_rename(from: impl Into<String>, to: impl Into<String>) -> DatasetEtlExpr {
        DatasetEtlExpr::rename(from, to)
    }

    pub fn etl_cast(column: impl Into<String>, dtype: impl Into<String>) -> DatasetEtlExpr {
        DatasetEtlExpr::cast(column, dtype)
    }

    pub fn etl_drop(columns: Vec<String>) -> DatasetEtlExpr {
        DatasetEtlExpr::drop(columns)
    }

    pub fn etl_dedupe(columns: Option<Vec<String>>) -> DatasetEtlExpr {
        DatasetEtlExpr::dedupe(columns)
    }

    pub fn etl_null_cleanup(columns: Option<Vec<String>>) -> DatasetEtlExpr {
        DatasetEtlExpr::null_cleanup(columns)
    }

    pub fn report_summary() -> DatasetReportExpr {
        DatasetReportExpr::summary()
    }

    pub fn report_profile() -> DatasetReportExpr {
        DatasetReportExpr::profile()
    }
}
