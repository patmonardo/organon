//! Corpus manifest artifacts produced from Model mediators.
//!
//! A Corpus artifact is an evidentiary manifestation: it records what a Model
//! mediator has made available as source, document, annotation, or selection
//! evidence. DataFrame rows can represent it, but the artifact is not reducible
//! to the table representation.

use polars::prelude::{DataFrame, NamedFrom, PolarsError, Series};

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::feature::mediator::ArtifactKind;
use crate::collections::dataset::feature::mediator::MediatorId;
use crate::collections::dataset::feature::mediator::MediatorProvenance;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CorpusArtifactId(pub String);

impl CorpusArtifactId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for CorpusArtifactId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for CorpusArtifactId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CorpusArtifactKind {
    SourceEvidence,
    DocumentEvidence,
    AnnotationEvidence,
    Selection,
}

impl CorpusArtifactKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SourceEvidence => "source-evidence",
            Self::DocumentEvidence => "document-evidence",
            Self::AnnotationEvidence => "annotation-evidence",
            Self::Selection => "selection",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusArtifact {
    id: CorpusArtifactId,
    kind: CorpusArtifactKind,
    label: String,
    provenance: Vec<MediatorProvenance>,
}

impl CorpusArtifact {
    pub fn new(
        id: impl Into<CorpusArtifactId>,
        kind: CorpusArtifactKind,
        label: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            label: label.into(),
            provenance: Vec::new(),
        }
    }

    pub fn from_model(
        id: impl Into<CorpusArtifactId>,
        kind: CorpusArtifactKind,
        label: impl Into<String>,
        model_id: impl Into<String>,
        source: impl Into<String>,
        derivation: impl Into<String>,
    ) -> Self {
        let provenance = MediatorProvenance::new(
            MediatorId::model(model_id),
            "corpus",
            source,
            "v1",
            derivation,
        )
        .with_artifact_kind(ArtifactKind::Corpus);
        Self::new(id, kind, label).with_provenance(provenance)
    }

    pub fn with_provenance(mut self, provenance: MediatorProvenance) -> Self {
        self.provenance
            .push(provenance.with_artifact_kind(ArtifactKind::Corpus));
        self
    }

    pub fn id(&self) -> &CorpusArtifactId {
        &self.id
    }

    pub fn kind(&self) -> CorpusArtifactKind {
        self.kind
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn provenance(&self) -> &[MediatorProvenance] {
        &self.provenance
    }
}

pub const CORPUS_ARTIFACT_COL_ID: &str = "artifact_id";
pub const CORPUS_ARTIFACT_COL_KIND: &str = "artifact_kind";
pub const CORPUS_ARTIFACT_COL_LABEL: &str = "label";
pub const CORPUS_ARTIFACT_COL_PROVENANCE: &str = "provenance";
pub const CORPUS_ARTIFACT_COL_PROVENANCE_COUNT: &str = "provenance_count";

#[derive(Debug, Clone)]
pub struct CorpusArtifactFrame {
    df: GDSDataFrame,
}

impl CorpusArtifactFrame {
    pub fn from_dataframe(df: GDSDataFrame) -> Result<Self, PolarsError> {
        let names: std::collections::HashSet<String> = df.column_names().into_iter().collect();
        for required in [
            CORPUS_ARTIFACT_COL_ID,
            CORPUS_ARTIFACT_COL_KIND,
            CORPUS_ARTIFACT_COL_LABEL,
            CORPUS_ARTIFACT_COL_PROVENANCE,
            CORPUS_ARTIFACT_COL_PROVENANCE_COUNT,
        ] {
            if !names.contains(required) {
                return Err(PolarsError::ColumnNotFound(required.into()));
            }
        }
        Ok(Self { df })
    }

    pub fn from_artifacts<I>(artifacts: I) -> Result<Self, PolarsError>
    where
        I: IntoIterator<Item = CorpusArtifact>,
    {
        let mut ids = Vec::<String>::new();
        let mut kinds = Vec::<String>::new();
        let mut labels = Vec::<String>::new();
        let mut provenance = Vec::<String>::new();
        let mut provenance_count = Vec::<u32>::new();

        for artifact in artifacts {
            ids.push(artifact.id.0);
            kinds.push(artifact.kind.as_str().to_string());
            labels.push(artifact.label);
            provenance.push(encode_provenance(&artifact.provenance));
            provenance_count.push(artifact.provenance.len() as u32);
        }

        let df = DataFrame::new_infer_height(vec![
            Series::new(CORPUS_ARTIFACT_COL_ID.into(), ids).into(),
            Series::new(CORPUS_ARTIFACT_COL_KIND.into(), kinds).into(),
            Series::new(CORPUS_ARTIFACT_COL_LABEL.into(), labels).into(),
            Series::new(CORPUS_ARTIFACT_COL_PROVENANCE.into(), provenance).into(),
            Series::new(
                CORPUS_ARTIFACT_COL_PROVENANCE_COUNT.into(),
                provenance_count,
            )
            .into(),
        ])?;

        Ok(Self {
            df: GDSDataFrame::new(df),
        })
    }

    pub fn dataframe(&self) -> &GDSDataFrame {
        &self.df
    }

    pub fn into_dataframe(self) -> GDSDataFrame {
        self.df
    }

    pub fn len(&self) -> usize {
        self.df.height()
    }

    pub fn is_empty(&self) -> bool {
        self.df.height() == 0
    }
}

fn encode_provenance(values: &[MediatorProvenance]) -> String {
    values
        .iter()
        .map(|value| {
            format!(
                "{}:{}:{}:{}:{}:{}",
                value.mediator_id().kind().as_str(),
                value.mediator_id().value(),
                value
                    .artifact_kind()
                    .map(|k| k.as_str())
                    .unwrap_or("unspecified"),
                value.layer(),
                value.version(),
                value.derivation(),
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_artifact_targets_model_as_corpus_artifact() {
        let artifact = CorpusArtifact::from_model(
            "corpus:selection:1",
            CorpusArtifactKind::Selection,
            "selected examples",
            "model:reader",
            "test",
            "fixture",
        );

        assert_eq!(artifact.kind(), CorpusArtifactKind::Selection);
        assert_eq!(artifact.provenance().len(), 1);
        let provenance = &artifact.provenance()[0];
        assert_eq!(provenance.mediator_id().value(), "model:reader");
        assert_eq!(provenance.artifact_kind(), Some(ArtifactKind::Corpus));
    }

    #[test]
    fn corpus_artifact_frame_builds_from_artifacts() {
        let artifact = CorpusArtifact::from_model(
            "corpus:selection:1",
            CorpusArtifactKind::Selection,
            "selected examples",
            "model:reader",
            "test",
            "fixture",
        );

        let frame = CorpusArtifactFrame::from_artifacts(vec![artifact]).expect("frame");
        assert_eq!(frame.len(), 1);
        assert!(frame
            .dataframe()
            .column_names()
            .iter()
            .any(|name| name == CORPUS_ARTIFACT_COL_PROVENANCE));
    }
}
