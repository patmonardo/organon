use std::fs;
use std::path::{Path, PathBuf};

use polars::prelude::{DataFrame, NamedFrom, Series};

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::core::error::DatasetIoError;

use super::{DatasetLayout, DatasetWorkspace};

impl DatasetWorkspace {
    pub fn index_raw_contents(&self, dataset: &str) -> Result<PathBuf, DatasetIoError> {
        let layout = self.ensure_dataset_layout(dataset)?;
        self.write_raw_inventory_frame(dataset, &layout)
    }

    pub fn write_raw_inventory_frame(
        &self,
        dataset: &str,
        layout: &DatasetLayout,
    ) -> Result<PathBuf, DatasetIoError> {
        let inventory_path = layout.semantic_dir.join("raw_inventory.csv");
        let inventory_path_string = inventory_path.to_string_lossy().into_owned();
        let frame = self.build_raw_inventory_frame(dataset, &layout.raw_dir)?;
        frame
            .write_csv(&inventory_path_string)
            .map_err(|e| DatasetIoError::Io(e.to_string()))?;
        Ok(inventory_path)
    }

    fn build_raw_inventory_frame(
        &self,
        dataset: &str,
        raw_dir: &Path,
    ) -> Result<GDSDataFrame, DatasetIoError> {
        let files = collect_files(raw_dir)?;

        let dataset_column: Vec<String> = std::iter::repeat(dataset.to_string())
            .take(files.len())
            .collect();
        let relative_path: Vec<String> = files
            .iter()
            .map(|path| {
                path.strip_prefix(raw_dir)
                    .ok()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.to_string_lossy().into_owned())
            })
            .collect();
        let extension: Vec<String> = files
            .iter()
            .map(|path| {
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("")
                    .to_string()
            })
            .collect();
        let bytes: Vec<i64> = files
            .iter()
            .map(|path| {
                fs::metadata(path)
                    .map(|meta| meta.len() as i64)
                    .map_err(|e| DatasetIoError::Io(e.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let dataframe = DataFrame::new_infer_height(vec![
            Series::new("dataset".into(), dataset_column).into(),
            Series::new("relative_path".into(), relative_path).into(),
            Series::new("extension".into(), extension).into(),
            Series::new("bytes".into(), bytes).into(),
        ])
        .map_err(|e| DatasetIoError::Io(e.to_string()))?;

        Ok(GDSDataFrame::new(dataframe))
    }
}

fn collect_files(root: &Path) -> Result<Vec<PathBuf>, DatasetIoError> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).map_err(|e| DatasetIoError::Io(e.to_string()))? {
            let entry = entry.map_err(|e| DatasetIoError::Io(e.to_string()))?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.is_file() {
                out.push(path);
            }
        }
    }

    out.sort();
    Ok(out)
}
