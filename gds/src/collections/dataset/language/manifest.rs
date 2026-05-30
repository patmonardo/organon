//! Language manifest artifacts produced from Feature mediators.
//!
//! Features are meta-linguistic capacities. A `LanguageElement` is one such
//! capacity made local and public as a token, construction, relation, rule, or
//! usage form.

use polars::prelude::{DataFrame, NamedFrom, PolarsError, Series};

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::feature::mediator::ArtifactKind;
use crate::collections::dataset::feature::mediator::MediatorId;
use crate::collections::dataset::feature::mediator::MediatorProvenance;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LanguageElementId(pub String);

impl LanguageElementId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for LanguageElementId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for LanguageElementId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LanguageElementKind {
    Token,
    Construction,
    FeatureRealization,
    Relation,
    Rule,
    Usage,
}

impl LanguageElementKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Token => "token",
            Self::Construction => "construction",
            Self::FeatureRealization => "feature-realization",
            Self::Relation => "relation",
            Self::Rule => "rule",
            Self::Usage => "usage",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageElement {
    id: LanguageElementId,
    kind: LanguageElementKind,
    form: String,
    provenance: Vec<MediatorProvenance>,
}

impl LanguageElement {
    pub fn new(
        id: impl Into<LanguageElementId>,
        kind: LanguageElementKind,
        form: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            form: form.into(),
            provenance: Vec::new(),
        }
    }

    pub fn from_feature(
        id: impl Into<LanguageElementId>,
        kind: LanguageElementKind,
        form: impl Into<String>,
        feature_id: impl Into<String>,
        source: impl Into<String>,
        derivation: impl Into<String>,
    ) -> Self {
        let provenance = MediatorProvenance::new(
            MediatorId::feature(feature_id),
            "language",
            source,
            "v1",
            derivation,
        )
        .with_artifact_kind(ArtifactKind::Language);
        Self::new(id, kind, form).with_provenance(provenance)
    }

    pub fn with_provenance(mut self, provenance: MediatorProvenance) -> Self {
        self.provenance
            .push(provenance.with_artifact_kind(ArtifactKind::Language));
        self
    }

    pub fn id(&self) -> &LanguageElementId {
        &self.id
    }

    pub fn kind(&self) -> LanguageElementKind {
        self.kind
    }

    pub fn form(&self) -> &str {
        &self.form
    }

    pub fn provenance(&self) -> &[MediatorProvenance] {
        &self.provenance
    }
}

pub const LANGUAGE_ARTIFACT_COL_ID: &str = "artifact_id";
pub const LANGUAGE_ARTIFACT_COL_KIND: &str = "artifact_kind";
pub const LANGUAGE_ARTIFACT_COL_FORM: &str = "form";
pub const LANGUAGE_ARTIFACT_COL_PROVENANCE: &str = "provenance";
pub const LANGUAGE_ARTIFACT_COL_PROVENANCE_COUNT: &str = "provenance_count";

#[derive(Debug, Clone)]
pub struct LanguageArtifactFrame {
    df: GDSDataFrame,
}

impl LanguageArtifactFrame {
    pub fn from_dataframe(df: GDSDataFrame) -> Result<Self, PolarsError> {
        let names: std::collections::HashSet<String> = df.column_names().into_iter().collect();
        for required in [
            LANGUAGE_ARTIFACT_COL_ID,
            LANGUAGE_ARTIFACT_COL_KIND,
            LANGUAGE_ARTIFACT_COL_FORM,
            LANGUAGE_ARTIFACT_COL_PROVENANCE,
            LANGUAGE_ARTIFACT_COL_PROVENANCE_COUNT,
        ] {
            if !names.contains(required) {
                return Err(PolarsError::ColumnNotFound(required.into()));
            }
        }
        Ok(Self { df })
    }

    pub fn from_elements<I>(elements: I) -> Result<Self, PolarsError>
    where
        I: IntoIterator<Item = LanguageElement>,
    {
        let mut ids = Vec::<String>::new();
        let mut kinds = Vec::<String>::new();
        let mut forms = Vec::<String>::new();
        let mut provenance = Vec::<String>::new();
        let mut provenance_count = Vec::<u32>::new();

        for element in elements {
            ids.push(element.id.0);
            kinds.push(element.kind.as_str().to_string());
            forms.push(element.form);
            provenance.push(encode_provenance(&element.provenance));
            provenance_count.push(element.provenance.len() as u32);
        }

        let df = DataFrame::new_infer_height(vec![
            Series::new(LANGUAGE_ARTIFACT_COL_ID.into(), ids).into(),
            Series::new(LANGUAGE_ARTIFACT_COL_KIND.into(), kinds).into(),
            Series::new(LANGUAGE_ARTIFACT_COL_FORM.into(), forms).into(),
            Series::new(LANGUAGE_ARTIFACT_COL_PROVENANCE.into(), provenance).into(),
            Series::new(
                LANGUAGE_ARTIFACT_COL_PROVENANCE_COUNT.into(),
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
    fn language_element_targets_feature_as_language_artifact() {
        let element = LanguageElement::from_feature(
            "language:pos:noun",
            LanguageElementKind::FeatureRealization,
            "NOUN",
            "feature:part-of-speech",
            "test",
            "fixture",
        );

        assert_eq!(element.kind(), LanguageElementKind::FeatureRealization);
        assert_eq!(element.form(), "NOUN");
        let provenance = &element.provenance()[0];
        assert_eq!(provenance.mediator_id().value(), "feature:part-of-speech");
        assert_eq!(provenance.artifact_kind(), Some(ArtifactKind::Language));
    }

    #[test]
    fn language_artifact_frame_builds_from_elements() {
        let element = LanguageElement::from_feature(
            "language:pos:noun",
            LanguageElementKind::FeatureRealization,
            "NOUN",
            "feature:part-of-speech",
            "test",
            "fixture",
        );

        let frame = LanguageArtifactFrame::from_elements(vec![element]).expect("frame");
        assert_eq!(frame.len(), 1);
        assert!(frame
            .dataframe()
            .column_names()
            .iter()
            .any(|name| name == LANGUAGE_ARTIFACT_COL_FORM));
    }
}
