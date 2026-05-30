//! Logic manifest artifacts produced from Plan mediators.
//!
//! A logical Plan is directed inferential movement before it is fixed as validity,
//! rule, entailment, proof trace, or transformation. A `LogicElement` is that
//! movement made manifest as a logic artifact.

use polars::prelude::{DataFrame, NamedFrom, PolarsError, Series};

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::feature::mediator::ArtifactKind;
use crate::collections::dataset::feature::mediator::MediatorId;
use crate::collections::dataset::feature::mediator::MediatorProvenance;
use crate::collections::dataset::plan::AppearanceObservation;
use crate::collections::dataset::plan::PlanPrinciple;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LogicElementId(pub String);

impl LogicElementId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for LogicElementId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for LogicElementId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogicElementKind {
    Rule,
    Entailment,
    ProofTrace,
    Validity,
    Transformation,
}

impl LogicElementKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rule => "rule",
            Self::Entailment => "entailment",
            Self::ProofTrace => "proof-trace",
            Self::Validity => "validity",
            Self::Transformation => "transformation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicElement {
    id: LogicElementId,
    kind: LogicElementKind,
    form: String,
    provenance: Vec<MediatorProvenance>,
}

impl LogicElement {
    pub fn new(
        id: impl Into<LogicElementId>,
        kind: LogicElementKind,
        form: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            form: form.into(),
            provenance: Vec::new(),
        }
    }

    pub fn from_plan(
        id: impl Into<LogicElementId>,
        kind: LogicElementKind,
        form: impl Into<String>,
        plan_id: impl Into<String>,
        source: impl Into<String>,
        derivation: impl Into<String>,
    ) -> Self {
        let provenance =
            MediatorProvenance::new(MediatorId::plan(plan_id), "logic", source, "v1", derivation)
                .with_artifact_kind(ArtifactKind::Logic);
        Self::new(id, kind, form).with_provenance(provenance)
    }

    pub fn from_principle_law(
        id: impl Into<LogicElementId>,
        kind: LogicElementKind,
        principle: &PlanPrinciple,
        observation: &AppearanceObservation,
        plan_id: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        let form = format!(
            "{} -> {} [{}]",
            principle.triad.as_str(),
            observation.feature_anchor,
            observation.evidence,
        );
        let derivation = format!(
            "principle:{};mode:{};law:{}={}",
            principle.triad.as_str(),
            principle.mode.as_str(),
            observation.feature_anchor,
            observation.evidence,
        );
        let provenance = MediatorProvenance::new(
            MediatorId::plan(plan_id),
            "logic/law",
            source,
            "v1",
            derivation,
        )
        .with_binding_id(observation.feature_anchor.clone())
        .with_artifact_kind(ArtifactKind::Logic);
        Self::new(id, kind, form).with_provenance(provenance)
    }

    pub fn with_provenance(mut self, provenance: MediatorProvenance) -> Self {
        self.provenance
            .push(provenance.with_artifact_kind(ArtifactKind::Logic));
        self
    }

    pub fn id(&self) -> &LogicElementId {
        &self.id
    }

    pub fn kind(&self) -> LogicElementKind {
        self.kind
    }

    pub fn form(&self) -> &str {
        &self.form
    }

    pub fn provenance(&self) -> &[MediatorProvenance] {
        &self.provenance
    }
}

pub const LOGIC_ARTIFACT_COL_ID: &str = "artifact_id";
pub const LOGIC_ARTIFACT_COL_KIND: &str = "artifact_kind";
pub const LOGIC_ARTIFACT_COL_FORM: &str = "form";
pub const LOGIC_ARTIFACT_COL_PROVENANCE: &str = "provenance";
pub const LOGIC_ARTIFACT_COL_PROVENANCE_COUNT: &str = "provenance_count";

#[derive(Debug, Clone)]
pub struct LogicArtifactFrame {
    df: GDSDataFrame,
}

impl LogicArtifactFrame {
    pub fn from_dataframe(df: GDSDataFrame) -> Result<Self, PolarsError> {
        let names: std::collections::HashSet<String> = df.column_names().into_iter().collect();
        for required in [
            LOGIC_ARTIFACT_COL_ID,
            LOGIC_ARTIFACT_COL_KIND,
            LOGIC_ARTIFACT_COL_FORM,
            LOGIC_ARTIFACT_COL_PROVENANCE,
            LOGIC_ARTIFACT_COL_PROVENANCE_COUNT,
        ] {
            if !names.contains(required) {
                return Err(PolarsError::ColumnNotFound(required.into()));
            }
        }
        Ok(Self { df })
    }

    pub fn from_elements<I>(elements: I) -> Result<Self, PolarsError>
    where
        I: IntoIterator<Item = LogicElement>,
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
            Series::new(LOGIC_ARTIFACT_COL_ID.into(), ids).into(),
            Series::new(LOGIC_ARTIFACT_COL_KIND.into(), kinds).into(),
            Series::new(LOGIC_ARTIFACT_COL_FORM.into(), forms).into(),
            Series::new(LOGIC_ARTIFACT_COL_PROVENANCE.into(), provenance).into(),
            Series::new(LOGIC_ARTIFACT_COL_PROVENANCE_COUNT.into(), provenance_count).into(),
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
    use crate::collections::dataset::plan::ConceptTriad;
    use crate::collections::dataset::plan::LawOfAppearance;
    use crate::collections::dataset::plan::PlanPrinciple;

    #[test]
    fn logic_element_targets_plan_as_logic_artifact() {
        let element = LogicElement::from_plan(
            "logic:rule:1",
            LogicElementKind::Rule,
            "if feature then language",
            "plan:derive-language",
            "test",
            "fixture",
        );

        assert_eq!(element.kind(), LogicElementKind::Rule);
        assert_eq!(element.form(), "if feature then language");
        let provenance = &element.provenance()[0];
        assert_eq!(provenance.mediator_id().value(), "plan:derive-language");
        assert_eq!(provenance.artifact_kind(), Some(ArtifactKind::Logic));
    }

    #[test]
    fn logic_artifact_frame_builds_from_elements() {
        let element = LogicElement::from_plan(
            "logic:rule:1",
            LogicElementKind::Rule,
            "if feature then language",
            "plan:derive-language",
            "test",
            "fixture",
        );

        let frame = LogicArtifactFrame::from_elements(vec![element]).expect("frame");
        assert_eq!(frame.len(), 1);
        assert!(frame
            .dataframe()
            .column_names()
            .iter()
            .any(|name| name == LOGIC_ARTIFACT_COL_FORM));
    }

    #[test]
    fn logic_element_manifests_principle_as_law() {
        let law = LawOfAppearance::mock_from_feature_anchors(["feature:cursor-move"]);
        let principle = PlanPrinciple::rational(ConceptTriad::ModelFeaturePlan)
            .with_law_of_appearance(law.clone());
        let observation = &law.observations[0];

        let element = LogicElement::from_principle_law(
            "logic:law:cursor-move",
            LogicElementKind::Rule,
            &principle,
            observation,
            "plan:mouse-action",
            "test",
        );

        assert_eq!(element.kind(), LogicElementKind::Rule);
        assert!(element.form().contains("model-feature-plan"));
        assert!(element.form().contains("feature:cursor-move"));
        let provenance = &element.provenance()[0];
        assert_eq!(provenance.mediator_id().value(), "plan:mouse-action");
        assert_eq!(provenance.layer(), "logic/law");
        assert_eq!(provenance.artifact_kind(), Some(ArtifactKind::Logic));
        assert!(provenance
            .derivation()
            .contains("principle:model-feature-plan"));
    }
}
