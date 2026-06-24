use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::core::error::DatasetIoError;
use polars::prelude::{DataFrame, NamedFrom, Series};

use super::{DatasetLayout, DatasetWorkspace};

impl DatasetWorkspace {
    pub fn write_catalog_frame(
        &self,
        dataset: &str,
        layout: &DatasetLayout,
    ) -> Result<std::path::PathBuf, DatasetIoError> {
        let catalog_path = layout.semantic_dir.join("catalog.csv");
        let catalog_path_string = catalog_path.to_string_lossy().into_owned();
        let frame = self.build_catalog_frame(dataset, layout)?;
        frame
            .write_csv(&catalog_path_string)
            .map_err(|e| DatasetIoError::Io(e.to_string()))?;
        Ok(catalog_path)
    }

    fn build_catalog_frame(
        &self,
        dataset: &str,
        layout: &DatasetLayout,
    ) -> Result<GDSDataFrame, DatasetIoError> {
        let entries = vec![
            (
                "semantic_frame",
                "semantic",
                "frame.json",
                layout.semantic_dir.join("frame.json"),
                "principle-law seed frame",
            ),
            (
                "catalog_frame",
                "semantic",
                "catalog.csv",
                layout.semantic_dir.join("catalog.csv"),
                "schema catalog frame",
            ),
            (
                "model_dir",
                "model",
                "folder",
                layout.model_dir.clone(),
                "model persistence root",
            ),
            (
                "feature_dir",
                "feature",
                "folder",
                layout.feature_dir.clone(),
                "feature persistence root",
            ),
            (
                "plan_dir",
                "plan",
                "folder",
                layout.plan_dir.clone(),
                "plan persistence root",
            ),
            (
                "corpus_dir",
                "corpus",
                "folder",
                layout.corpus_dir.clone(),
                "corpus persistence root",
            ),
            (
                "language_dir",
                "language",
                "folder",
                layout.language_dir.clone(),
                "language persistence root",
            ),
            (
                "logic_dir",
                "logic",
                "folder",
                layout.logic_dir.clone(),
                "logic persistence root",
            ),
            (
                "raw_dir",
                "raw",
                "folder",
                layout.raw_dir.clone(),
                "raw archive extraction root",
            ),
            (
                "artifacts_dir",
                "artifacts",
                "folder",
                layout.artifacts_dir.clone(),
                "generated artifacts root",
            ),
        ];

        let dataset_column: Vec<String> = std::iter::repeat(dataset.to_string())
            .take(entries.len())
            .collect();
        let entry_id: Vec<String> = entries
            .iter()
            .map(|(id, _, _, _, _)| (*id).to_string())
            .collect();
        let kind: Vec<String> = entries
            .iter()
            .map(|(_, kind, _, _, _)| (*kind).to_string())
            .collect();
        let name: Vec<String> = entries
            .iter()
            .map(|(_, _, name, _, _)| (*name).to_string())
            .collect();
        let path: Vec<String> = entries
            .iter()
            .map(|(_, _, _, path, _)| path.to_string_lossy().into_owned())
            .collect();
        let status: Vec<String> = std::iter::repeat("created".to_string())
            .take(entries.len())
            .collect();
        let description: Vec<String> = entries
            .iter()
            .map(|(_, _, _, _, desc)| (*desc).to_string())
            .collect();

        let dataframe = DataFrame::new_infer_height(vec![
            Series::new("dataset".into(), dataset_column).into(),
            Series::new("entry_id".into(), entry_id).into(),
            Series::new("kind".into(), kind).into(),
            Series::new("name".into(), name).into(),
            Series::new("path".into(), path).into(),
            Series::new("status".into(), status).into(),
            Series::new("description".into(), description).into(),
        ])
        .map_err(|e| DatasetIoError::Io(e.to_string()))?;

        Ok(GDSDataFrame::new(dataframe))
    }
}
