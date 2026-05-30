use std::fs;
use std::path::Path;

use crate::collections::dataset::core::error::DatasetIoError;
use crate::collections::dataset::core::utils::download::{copy_local, download_url};
use crate::collections::dataset::core::utils::extract::extract_archive;

use super::{
    DatasetCreateOptions, DatasetCreateReport, DatasetIngestReport, DatasetStageReport,
    DatasetWorkspace,
};

impl DatasetWorkspace {
    /// Builder-style semantic dataset creation.
    ///
    /// This creates the dataset layout, writes a canonical semantic frame, and
    /// emits a schema catalog frame over the dataset folder layout.
    pub fn create_dataset(
        &self,
        dataset: &str,
        options: DatasetCreateOptions,
    ) -> Result<DatasetCreateReport, DatasetIoError> {
        let layout = self.ensure_dataset_layout(dataset)?;
        let semantic_frame_path =
            self.write_semantic_frame(dataset, &options.principle_triad, &options.law_view)?;
        let catalog_frame_path = self.write_catalog_frame(dataset, &layout)?;
        Ok(DatasetCreateReport {
            dataset: dataset.to_string(),
            layout,
            semantic_frame_path,
            catalog_frame_path,
        })
    }

    pub fn stage_archive(
        &self,
        dataset: &str,
        source: &str,
    ) -> Result<DatasetStageReport, DatasetIoError> {
        let stage_dir = self.staging_root().join(dataset);
        fs::create_dir_all(&stage_dir).map_err(|e| DatasetIoError::Io(e.to_string()))?;

        let source_path = Path::new(source);
        let archive_name = source_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("archive.bin");
        let staged_path = stage_dir.join(archive_name);

        let transferred = if source.starts_with("http://") || source.starts_with("https://") {
            download_url(source, &staged_path)?
        } else {
            copy_local(source_path, &staged_path)?
        };

        Ok(DatasetStageReport {
            dataset: dataset.to_string(),
            source: source.to_string(),
            staged_path,
            transferred,
        })
    }

    pub fn ingest_archive(
        &self,
        dataset: &str,
        source: &str,
    ) -> Result<DatasetIngestReport, DatasetIoError> {
        let layout = self.ensure_dataset_layout(dataset)?;
        let cache_dir = self.cache_root().join(dataset);
        fs::create_dir_all(&cache_dir).map_err(|e| DatasetIoError::Io(e.to_string()))?;

        let staged = self.stage_archive(dataset, source)?;
        let archive_name = staged
            .staged_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("archive.bin");
        let archive_path = cache_dir.join(archive_name);

        let downloaded = Some(copy_local(&staged.staged_path, &archive_path)?);

        let extracted = extract_archive(&archive_path, &layout.raw_dir)?;
        let raw_inventory_path = self.write_raw_inventory_frame(dataset, &layout)?;
        Ok(DatasetIngestReport {
            dataset: dataset.to_string(),
            staged_path: staged.staged_path,
            archive_path,
            extracted_to: layout.raw_dir,
            raw_inventory_path,
            downloaded,
            extracted,
        })
    }
}
