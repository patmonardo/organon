use crate::form::ProgramFeature;
use crate::form::ProgramSpec;
use serde::Serialize;
use std::collections::BTreeMap;

pub trait FormCapabilitySource {
    fn snapshot(&self, program: &ProgramSpec) -> Result<FormCapabilitySnapshot, String>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryDatasetCapabilitySource;

impl InMemoryDatasetCapabilitySource {
    pub fn new() -> Self {
        Self
    }
}

impl FormCapabilitySource for InMemoryDatasetCapabilitySource {
    fn snapshot(&self, program: &ProgramSpec) -> Result<FormCapabilitySnapshot, String> {
        let program_features = program
            .define_features()
            .map_err(|error| format!("Form capability projection failed: {error}"))?;

        let application_forms = program
            .application_forms
            .iter()
            .map(|form| {
                let selected = program_features.selected_forms.contains(&form.name);
                let resolved_program_features = program_features
                    .features
                    .iter()
                    .filter(|feature| {
                        feature.value == form.name || form.patterns.contains(&feature.value)
                    })
                    .map(ProgramFeatureCapability::from)
                    .collect();
                let specifications = form
                    .specifications
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect();

                ApplicationFormCapability {
                    name: form.name.clone(),
                    domain: form.domain.clone(),
                    selected,
                    declared_features: form.features.clone(),
                    operator_patterns: form.patterns.clone(),
                    specifications,
                    resolved_program_features,
                }
            })
            .collect();

        Ok(FormCapabilitySnapshot {
            source: FormCapabilitySourceMetadata::dataset_mock(),
            program_name: program_features.program_name,
            application_forms,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormCapabilitySourceMetadata {
    pub kind: String,
    pub persistent: bool,
    pub semantics: String,
}

impl FormCapabilitySourceMetadata {
    fn dataset_mock() -> Self {
        Self {
            kind: "dataset_mock".to_string(),
            persistent: false,
            semantics: "in_memory_program_snapshot".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormCapabilitySnapshot {
    pub source: FormCapabilitySourceMetadata,
    pub program_name: String,
    pub application_forms: Vec<ApplicationFormCapability>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationFormCapability {
    pub name: String,
    pub domain: String,
    pub selected: bool,
    pub declared_features: Vec<String>,
    pub operator_patterns: Vec<String>,
    pub specifications: BTreeMap<String, String>,
    pub resolved_program_features: Vec<ProgramFeatureCapability>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramFeatureCapability {
    pub kind: String,
    pub value: String,
    pub source: String,
}

impl From<&ProgramFeature> for ProgramFeatureCapability {
    fn from(feature: &ProgramFeature) -> Self {
        Self {
            kind: feature.kind.as_str().to_string(),
            value: feature.value.clone(),
            source: feature.source.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form::ApplicationForm;
    use crate::form::Context;
    use crate::form::FormShape;
    use crate::form::Morph;
    use crate::form::Shape;
    use crate::form::Specification;
    use std::collections::HashMap;

    fn program() -> ProgramSpec {
        ProgramSpec::new(
            FormShape::new(
                Shape::default(),
                Context::default(),
                Morph::new(vec!["base.normalize".to_string()]),
            ),
            Specification::new("form.capabilities".to_string(), None, HashMap::new()),
            Vec::new(),
            vec![
                ApplicationForm::new(
                    "centrality".to_string(),
                    "graph-ml".to_string(),
                    vec!["feature.centrality.pagerank".to_string()],
                    vec!["algo.pagerank".to_string()],
                    HashMap::from([("binding".to_string(), "spec.pagerank".to_string())]),
                ),
                ApplicationForm::new(
                    "community".to_string(),
                    "graph-ml".to_string(),
                    vec!["feature.community.leiden".to_string()],
                    vec!["algo.leiden".to_string()],
                    HashMap::new(),
                ),
            ],
            vec!["centrality".to_string()],
        )
    }

    #[test]
    fn snapshots_all_application_forms_and_marks_selection() {
        let snapshot = InMemoryDatasetCapabilitySource::new()
            .snapshot(&program())
            .expect("capability snapshot should resolve");

        assert_eq!(snapshot.application_forms.len(), 2);
        assert!(snapshot.application_forms[0].selected);
        assert!(!snapshot.application_forms[1].selected);
        assert_eq!(snapshot.source.kind, "dataset_mock");
        assert!(!snapshot.source.persistent);
        assert_eq!(snapshot.source.semantics, "in_memory_program_snapshot");
    }

    #[test]
    fn preserves_declarations_and_associates_program_features() {
        let snapshot = InMemoryDatasetCapabilitySource::new()
            .snapshot(&program())
            .expect("capability snapshot should resolve");
        let centrality = &snapshot.application_forms[0];

        assert_eq!(
            centrality.declared_features,
            vec!["feature.centrality.pagerank"]
        );
        assert_eq!(centrality.operator_patterns, vec!["algo.pagerank"]);
        assert_eq!(
            centrality.specifications.get("binding").map(String::as_str),
            Some("spec.pagerank")
        );
        assert!(centrality
            .resolved_program_features
            .iter()
            .any(|feature| feature.kind == "application-form" && feature.value == "centrality"));
        assert!(centrality
            .resolved_program_features
            .iter()
            .any(|feature| feature.kind == "operator-pattern"
                && feature.value == "algo.pagerank"
                && feature.source == "operator_pattern::algo.pagerank"));
    }
}
