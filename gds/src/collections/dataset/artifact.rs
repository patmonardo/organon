//! Generic artifact classification for Dataset-facing tables.
//!
//! This keeps the Dataset/DataFrame interface generic while still letting the
//! caller declare what sort of semantic artifact a table represents.
//! Program/plan artifacts are one important case, but not the only one.

use std::fmt;

use crate::collections::dataframe::table::TableBuilder;
use crate::collections::dataframe::{GDSDataFrame, GDSFrameError};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum DatasetArtifactKind {
    #[default]
    Table,
    Corpus,
    FeatureMap,
    ModelView,
    ModelState,
    SemanticSubgraph,
    ProgramPlan,
    ProgramImage,
    Custom(String),
}

impl DatasetArtifactKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Table => "table",
            Self::Corpus => "corpus",
            Self::FeatureMap => "feature-map",
            Self::ModelView => "model-view",
            Self::ModelState => "model-state",
            Self::SemanticSubgraph => "semantic-subgraph",
            Self::ProgramPlan => "program-plan",
            Self::ProgramImage => "program-image",
            Self::Custom(value) => value.as_str(),
        }
    }
}

impl fmt::Display for DatasetArtifactKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DatasetArtifactProfile {
    primary_kind: DatasetArtifactKind,
    facets: Vec<String>,
}

impl DatasetArtifactProfile {
    pub fn new(primary_kind: DatasetArtifactKind) -> Self {
        Self {
            primary_kind,
            facets: Vec::new(),
        }
    }

    pub fn primary_kind(&self) -> &DatasetArtifactKind {
        &self.primary_kind
    }

    pub fn facets(&self) -> &[String] {
        &self.facets
    }

    pub fn facets_csv(&self) -> String {
        self.facets.join("|")
    }

    pub fn with_primary_kind(mut self, primary_kind: DatasetArtifactKind) -> Self {
        self.primary_kind = primary_kind;
        self
    }

    pub fn with_facet(mut self, facet: impl Into<String>) -> Self {
        let facet = facet.into();
        if !self.facets.iter().any(|existing| existing == &facet) {
            self.facets.push(facet);
        }
        self
    }

    pub fn has_kind(&self, kind: &DatasetArtifactKind) -> bool {
        &self.primary_kind == kind
    }

    pub fn has_facet(&self, facet: &str) -> bool {
        self.facets.iter().any(|existing| existing == facet)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetArtifactRecord {
    pub artifact_id: String,
    pub label: String,
    pub profile: DatasetArtifactProfile,
}

impl DatasetArtifactRecord {
    pub fn new(
        artifact_id: impl Into<String>,
        label: impl Into<String>,
        profile: DatasetArtifactProfile,
    ) -> Self {
        Self {
            artifact_id: artifact_id.into(),
            label: label.into(),
            profile,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetArtifactRelationRecord {
    pub artifact_id: String,
    pub relation: String,
    pub target_id: String,
}

impl DatasetArtifactRelationRecord {
    pub fn new(
        artifact_id: impl Into<String>,
        relation: impl Into<String>,
        target_id: impl Into<String>,
    ) -> Self {
        Self {
            artifact_id: artifact_id.into(),
            relation: relation.into(),
            target_id: target_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetArtifactPropertyRecord {
    pub artifact_id: String,
    pub key: String,
    pub value: String,
}

impl DatasetArtifactPropertyRecord {
    pub fn new(
        artifact_id: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            artifact_id: artifact_id.into(),
            key: key.into(),
            value: value.into(),
        }
    }
}

pub fn build_artifact_records_table(
    rows: &[DatasetArtifactRecord],
) -> Result<GDSDataFrame, GDSFrameError> {
    let artifact_ids = rows
        .iter()
        .map(|row| row.artifact_id.clone())
        .collect::<Vec<_>>();
    let labels = rows.iter().map(|row| row.label.clone()).collect::<Vec<_>>();
    let primary_kinds = rows
        .iter()
        .map(|row| row.profile.primary_kind().to_string())
        .collect::<Vec<_>>();
    let facets = rows
        .iter()
        .map(|row| row.profile.facets_csv())
        .collect::<Vec<_>>();

    TableBuilder::new()
        .with_string_column("artifact_id", &artifact_ids)
        .with_string_column("label", &labels)
        .with_string_column("primary_kind", &primary_kinds)
        .with_string_column("facets", &facets)
        .build()
        .map_err(GDSFrameError::from)
}

pub fn build_artifact_relations_table(
    rows: &[DatasetArtifactRelationRecord],
) -> Result<GDSDataFrame, GDSFrameError> {
    let artifact_ids = rows
        .iter()
        .map(|row| row.artifact_id.clone())
        .collect::<Vec<_>>();
    let relations = rows
        .iter()
        .map(|row| row.relation.clone())
        .collect::<Vec<_>>();
    let target_ids = rows
        .iter()
        .map(|row| row.target_id.clone())
        .collect::<Vec<_>>();

    TableBuilder::new()
        .with_string_column("artifact_id", &artifact_ids)
        .with_string_column("relation", &relations)
        .with_string_column("target_id", &target_ids)
        .build()
        .map_err(GDSFrameError::from)
}

pub fn build_artifact_properties_table(
    rows: &[DatasetArtifactPropertyRecord],
) -> Result<GDSDataFrame, GDSFrameError> {
    let artifact_ids = rows
        .iter()
        .map(|row| row.artifact_id.clone())
        .collect::<Vec<_>>();
    let keys = rows.iter().map(|row| row.key.clone()).collect::<Vec<_>>();
    let values = rows.iter().map(|row| row.value.clone()).collect::<Vec<_>>();

    TableBuilder::new()
        .with_string_column("artifact_id", &artifact_ids)
        .with_string_column("key", &keys)
        .with_string_column("value", &values)
        .build()
        .map_err(GDSFrameError::from)
}
