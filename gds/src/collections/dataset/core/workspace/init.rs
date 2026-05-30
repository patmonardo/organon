use std::fs;
use std::path::{Path, PathBuf};

use crate::collections::dataset::core::error::DatasetIoError;

use super::{DatasetLayout, DatasetWorkspace};

impl DatasetWorkspace {
    pub fn new(root: impl AsRef<Path>) -> Result<Self, DatasetIoError> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(&root).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        fs::create_dir_all(root.join("datasets")).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        fs::create_dir_all(root.join("cache")).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        fs::create_dir_all(root.join("staging")).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn datasets_root(&self) -> PathBuf {
        self.root.join("datasets")
    }

    pub fn cache_root(&self) -> PathBuf {
        self.root.join("cache")
    }

    pub fn staging_root(&self) -> PathBuf {
        self.root.join("staging")
    }

    pub fn ensure_dataset_layout(&self, dataset: &str) -> Result<DatasetLayout, DatasetIoError> {
        let base = self.datasets_root().join(dataset);
        let layout = DatasetLayout {
            root: base.clone(),
            semantic_dir: base.join("semantic"),
            corpus_dir: base.join("corpus"),
            language_dir: base.join("language"),
            logic_dir: base.join("logic"),
            raw_dir: base.join("raw"),
            artifacts_dir: base.join("artifacts"),
        };
        fs::create_dir_all(&layout.semantic_dir).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        fs::create_dir_all(&layout.corpus_dir).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        fs::create_dir_all(&layout.language_dir).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        fs::create_dir_all(&layout.logic_dir).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        fs::create_dir_all(&layout.raw_dir).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        fs::create_dir_all(&layout.artifacts_dir).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        Ok(layout)
    }

    pub fn write_semantic_frame(
        &self,
        dataset: &str,
        principle_triad: &str,
        law_view: &str,
    ) -> Result<PathBuf, DatasetIoError> {
        let layout = self.ensure_dataset_layout(dataset)?;
        let frame_path = layout.semantic_dir.join("frame.json");
        let body = format!(
            "{{\n  \"dataset\": \"{}\",\n  \"principle\": \"{}\",\n  \"law\": \"{}\",\n  \"pairs\": {{\n    \"rational\": \"Model-Feature-Plan\",\n    \"empirical\": \"Corpus-Language-Logic\"\n  }}\n}}\n",
            dataset, principle_triad, law_view
        );
        fs::write(&frame_path, body).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        Ok(frame_path)
    }
}
