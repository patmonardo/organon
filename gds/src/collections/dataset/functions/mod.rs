//! Dataset DSL functions.
//!
//! Core dataset builders and operations (ingest, scan, splits).

use std::fs::read_dir;
use std::path::Path;

use crate::collections::dataframe::GDSFrameError;
use crate::collections::dataset::corpus::Corpus;

pub mod feature;
pub mod model;
pub mod tree;

/// Scan a directory for text files (non-recursive) and build a `Corpus`.
/// Only regular files are considered; file contents are read as UTF-8.
pub fn scan_text_dir(path: impl AsRef<Path>) -> Result<Corpus, GDSFrameError> {
    let mut paths: Vec<std::path::PathBuf> = Vec::new();
    for entry in read_dir(path.as_ref())? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
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
}
