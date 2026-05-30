//! Mode-oriented Dataset DSL functions.
//!
//! These are the composed workflows and convenience entry points that make the
//! core constructors pleasant to use.

use std::fs::read_dir;
use std::path::Path;

use crate::collections::dataframe::GDSFrameError;
use crate::collections::dataset::corpus::{Corpus, CorpusError};
use crate::collections::dataset::lab::toolchain::DatasetPipeline;

/// Start an empty semantic Dataset pipeline.
pub fn pipeline() -> DatasetPipeline {
    DatasetPipeline::new()
}

/// Build the canonical text-domain Input -> Encode -> Transform -> Decode -> Output lifecycle.
pub fn text_lifecycle(base_name: impl AsRef<str>) -> DatasetPipeline {
    DatasetPipeline::text_lifecycle(base_name)
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
    use crate::collections::dataset::dsl::functions::core::{project_text, report_summary};
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
