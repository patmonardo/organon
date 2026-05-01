//! Dataset DSL functions.
//!
//! This module is the small, typed authoring surface for the Dataset DSL. The
//! functions here intentionally mirror the DataFrame DSL's thin factory style:
//! they construct existing semantic objects rather than introducing another
//! wrapper layer.

use std::fs::read_dir;
use std::path::Path;

use serde_json::Value as JsonValue;

use crate::collections::dataframe::GDSFrameError;
use crate::collections::dataset::corpus::{Corpus, CorpusError};
use crate::collections::dataset::expressions::dataop::DatasetDataOpExpr;
use crate::collections::dataset::expressions::io::DatasetIoExpr;
use crate::collections::dataset::expressions::metadata::DatasetMetadataExpr;
use crate::collections::dataset::expressions::projection::DatasetProjectionExpr;
use crate::collections::dataset::expressions::registry::DatasetRegistryExpr;
use crate::collections::dataset::expressions::reporting::DatasetReportExpr;
use crate::collections::dataset::toolchain::DatasetPipeline;

pub mod feature;
pub mod model;
pub mod tree;

/// Start an empty semantic Dataset pipeline.
pub fn pipeline() -> DatasetPipeline {
    DatasetPipeline::new()
}

/// Build the canonical text-domain Input -> Encode -> Transform -> Decode -> Output lifecycle.
pub fn text_lifecycle(base_name: impl AsRef<str>) -> DatasetPipeline {
    DatasetPipeline::text_lifecycle(base_name)
}

/// Generic Dataset data-op constructors.
pub fn dataop_input(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::input(name)
}

pub fn dataop_encode(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::encode(name)
}

pub fn dataop_transform(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::transform(name)
}

pub fn dataop_decode(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::decode(name)
}

pub fn dataop_output(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::output(name)
}

/// Text-domain Dataset data-op constructors.
pub fn text_input(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::text_input(name)
}

pub fn text_encode(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::text_encode(name)
}

pub fn text_transform(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::text_transform(name)
}

pub fn text_decode(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::text_decode(name)
}

pub fn text_output(name: impl Into<String>) -> DatasetDataOpExpr {
    DatasetDataOpExpr::text_output(name)
}

/// Declarative Dataset source descriptors.
pub fn io_path(path: impl Into<String>) -> DatasetIoExpr {
    DatasetIoExpr::from_path(path)
}

pub fn io_url(url: impl Into<String>) -> DatasetIoExpr {
    DatasetIoExpr::from_url(url)
}

pub fn registry(name: impl Into<String>) -> DatasetRegistryExpr {
    DatasetRegistryExpr::new(name)
}

pub fn registry_versioned(
    name: impl Into<String>,
    version: impl Into<String>,
) -> DatasetRegistryExpr {
    DatasetRegistryExpr::versioned(name, version)
}

pub fn metadata(key: impl Into<String>, value: impl Into<JsonValue>) -> DatasetMetadataExpr {
    DatasetMetadataExpr::new(key, value)
}

pub fn project_text<I, S>(columns: I) -> DatasetProjectionExpr
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    DatasetProjectionExpr::text(collect_columns(columns))
}

pub fn project_corpus<I, S>(columns: I) -> DatasetProjectionExpr
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    DatasetProjectionExpr::corpus(collect_columns(columns))
}

pub fn project_graph<I, S>(columns: I) -> DatasetProjectionExpr
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    DatasetProjectionExpr::graph(collect_columns(columns))
}

pub fn report_summary() -> DatasetReportExpr {
    DatasetReportExpr::summary()
}

pub fn report_profile() -> DatasetReportExpr {
    DatasetReportExpr::profile()
}

fn collect_columns<I, S>(columns: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    columns.into_iter().map(Into::into).collect()
}

/// Scan a directory for text files (non-recursive) and build a `Corpus`.
/// Only regular files are considered; file contents are read as UTF-8.
pub fn scan_text_dir(path: impl AsRef<Path>) -> Result<Corpus, CorpusError> {
    let mut paths: Vec<std::path::PathBuf> = Vec::new();
    for entry in read_dir(path.as_ref()).map_err(GDSFrameError::from)? {
        let entry = entry.map_err(GDSFrameError::from)?;
        if entry.file_type().map_err(GDSFrameError::from)?.is_file() {
            paths.push(entry.path());
        }
    }
    Corpus::from_text_files(&paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, remove_dir_all, write};

    #[test]
    fn test_scan_text_dir() {
        let tmp = std::env::temp_dir().join("gds_test_scan_text_dir");
        let _ = remove_dir_all(&tmp);
        create_dir_all(&tmp).unwrap();
        write(tmp.join("a.txt"), "hello world\n").unwrap();
        write(tmp.join("b.txt"), "one two three").unwrap();

        let corpus = scan_text_dir(&tmp).unwrap();
        assert_eq!(corpus.len(), 2);

        // cleanup
        let _ = remove_dir_all(&tmp);
    }

    #[test]
    fn test_text_lifecycle_function_surface() {
        let pipeline = text_lifecycle("article");

        assert_eq!(pipeline.ops.len(), 5);
        assert_eq!(pipeline.ops[0].stage(), "input");
        assert_eq!(pipeline.ops[0].name(), "article.input");
        assert_eq!(pipeline.ops[2].stage(), "transform");
        assert_eq!(pipeline.ops[4].name(), "article.output");
    }

    #[test]
    fn test_projection_and_report_functions() {
        let projection = project_text(["doc_id", "text", "token_count"]);
        assert_eq!(projection.columns(), &["doc_id", "text", "token_count"]);

        let report = report_summary().with_title("Dataset DSL summary");
        assert_eq!(report.title(), Some("Dataset DSL summary"));
    }
}
