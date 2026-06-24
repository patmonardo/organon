use std::path::PathBuf;

use crate::collections::dataset::core::utils::download::DownloadReport;
use crate::collections::dataset::core::utils::extract::ExtractReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DatasetSemanticSupportFold {
    Semantic,
    Model,
    Feature,
    Plan,
    Corpus,
    Language,
    Logic,
}

impl DatasetSemanticSupportFold {
    pub const ALL: [Self; 7] = [
        Self::Semantic,
        Self::Model,
        Self::Feature,
        Self::Plan,
        Self::Corpus,
        Self::Language,
        Self::Logic,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Semantic => "semantic",
            Self::Model => "model",
            Self::Feature => "feature",
            Self::Plan => "plan",
            Self::Corpus => "corpus",
            Self::Language => "language",
            Self::Logic => "logic",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DatasetWorkspace {
    pub(super) root: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DatasetLayout {
    pub root: PathBuf,
    pub semantic_dir: PathBuf,
    pub model_dir: PathBuf,
    pub feature_dir: PathBuf,
    pub plan_dir: PathBuf,
    pub corpus_dir: PathBuf,
    pub language_dir: PathBuf,
    pub logic_dir: PathBuf,
    pub raw_dir: PathBuf,
    pub artifacts_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DatasetIngestReport {
    pub dataset: String,
    pub staged_path: PathBuf,
    pub archive_path: PathBuf,
    pub extracted_to: PathBuf,
    pub raw_inventory_path: PathBuf,
    pub downloaded: Option<DownloadReport>,
    pub extracted: ExtractReport,
}

#[derive(Debug, Clone)]
pub struct DatasetStageReport {
    pub dataset: String,
    pub source: String,
    pub staged_path: PathBuf,
    pub transferred: DownloadReport,
}

#[derive(Debug, Clone)]
pub struct DatasetCreateOptions {
    pub principle_triad: String,
    pub law_view: String,
}

impl Default for DatasetCreateOptions {
    fn default() -> Self {
        Self {
            principle_triad: "model-feature-plan".to_string(),
            law_view: "corpus-language-logic".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DatasetCreateReport {
    pub dataset: String,
    pub layout: DatasetLayout,
    pub semantic_frame_path: PathBuf,
    pub catalog_frame_path: PathBuf,
}
