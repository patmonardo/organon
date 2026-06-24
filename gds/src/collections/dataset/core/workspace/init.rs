use std::fs;
use std::path::{Path, PathBuf};

use crate::collections::dataset::core::error::DatasetIoError;

use super::{DatasetLayout, DatasetSemanticSupportFold, DatasetWorkspace};

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
            model_dir: base.join("model"),
            feature_dir: base.join("feature"),
            plan_dir: base.join("plan"),
            corpus_dir: base.join("corpus"),
            language_dir: base.join("language"),
            logic_dir: base.join("logic"),
            raw_dir: base.join("raw"),
            artifacts_dir: base.join("artifacts"),
        };
        fs::create_dir_all(&layout.semantic_dir).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        fs::create_dir_all(&layout.model_dir).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        fs::create_dir_all(&layout.feature_dir).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        fs::create_dir_all(&layout.plan_dir).map_err(|e| DatasetIoError::Io(e.to_string()))?;
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
        let seven_fold_support = DatasetSemanticSupportFold::ALL
            .iter()
            .map(|fold| format!("\"{}\"", fold.as_str()))
            .collect::<Vec<_>>()
            .join(", ");
        let body = format!(
            "{{\n  \"dataset\": \"{}\",\n  \"principle\": \"{}\",\n  \"law\": \"{}\",\n  \"pairs\": {{\n    \"rational\": \"Model-Feature-Plan\",\n    \"empirical\": \"Corpus-Language-Logic\"\n  }},\n  \"seven_fold_support\": [{}],\n  \"support_directories\": {{\n    \"semantic\": \"semantic\",\n    \"model\": \"model\",\n    \"feature\": \"feature\",\n    \"plan\": \"plan\",\n    \"corpus\": \"corpus\",\n    \"language\": \"language\",\n    \"logic\": \"logic\"\n  }}\n}}\n",
            dataset, principle_triad, law_view, seven_fold_support
        );
        fs::write(&frame_path, body).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        Ok(frame_path)
    }
}
