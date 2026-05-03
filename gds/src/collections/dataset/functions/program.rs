//! Program Feature helpers for the Dataset internal DSL.
//!
//! These functions are the DirectCompute authoring surface for program images:
//! callers construct `ProgramFeatures` through small conceptual functions rather
//! than spelling raw `ProgramFeature::new` records or embedding a text grammar.

use crate::form::{ProgramFeature, ProgramFeatureKind, ProgramFeatures};

pub fn program_features<N, S, F>(program_name: N, selected_forms: S, features: F) -> ProgramFeatures
where
    N: Into<String>,
    S: IntoIterator,
    S::Item: Into<String>,
    F: IntoIterator<Item = ProgramFeature>,
{
    let program_name = program_name.into();
    let mut lowered = Vec::new();
    lowered.push(ProgramFeature::new(
        ProgramFeatureKind::SpecificationBinding,
        program_name.clone(),
        format!("specification::{program_name}"),
    ));
    lowered.extend(features);

    ProgramFeatures::new(
        program_name,
        selected_forms.into_iter().map(Into::into).collect(),
        lowered,
    )
}

pub fn program_feature(
    kind: ProgramFeatureKind,
    value: impl Into<String>,
    source: impl Into<String>,
) -> ProgramFeature {
    ProgramFeature::new(kind, value.into(), source.into())
}

pub fn program_import(path: impl AsRef<str>) -> ProgramFeature {
    let path = path.as_ref();
    declaration(ProgramFeatureKind::Import, "use", path)
}

pub fn program_source(name: impl AsRef<str>, descriptor: impl AsRef<str>) -> ProgramFeature {
    phrase(
        ProgramFeatureKind::Source,
        "source",
        name.as_ref(),
        descriptor.as_ref(),
    )
}

pub fn program_observation(name: impl AsRef<str>) -> ProgramFeature {
    declaration(
        ProgramFeatureKind::Observation,
        "observation",
        name.as_ref(),
    )
}

pub fn program_appearance(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Observation, "appearance", name.as_ref())
}

pub fn program_reflection(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Reflection, "reflection", name.as_ref())
}

pub fn program_logogenesis(name: impl AsRef<str>) -> ProgramFeature {
    declaration(
        ProgramFeatureKind::Logogenesis,
        "logogenesis",
        name.as_ref(),
    )
}

pub fn program_subfeature(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Subfeature, "subfeature", name.as_ref())
}

pub fn program_mark(name: impl AsRef<str>, expression: impl AsRef<str>) -> ProgramFeature {
    phrase(
        ProgramFeatureKind::Mark,
        "mark",
        name.as_ref(),
        expression.as_ref(),
    )
}

pub fn program_derive(name: impl AsRef<str>, expression: impl AsRef<str>) -> ProgramFeature {
    phrase(
        ProgramFeatureKind::Mark,
        "derive",
        name.as_ref(),
        expression.as_ref(),
    )
}

pub fn program_concept(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Concept, "concept", name.as_ref())
}

pub fn program_identity(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Concept, "identity", name.as_ref())
}

pub fn program_judgment(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Judgment, "judgment", name.as_ref())
}

pub fn program_syllogism(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Syllogism, "syllogism", name.as_ref())
}

pub fn program_principle(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Principle, "principle", name.as_ref())
}

pub fn program_query(name: impl AsRef<str>, expression: impl AsRef<str>) -> ProgramFeature {
    phrase(
        ProgramFeatureKind::Query,
        "query",
        name.as_ref(),
        expression.as_ref(),
    )
}

pub fn program_procedure(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Procedure, "procedure", name.as_ref())
}

pub fn program_from(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Source, "from", name.as_ref())
}

pub fn program_key(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Observation, "key", name.as_ref())
}

pub fn program_retain(fields: impl AsRef<str>) -> ProgramFeature {
    phrase(
        ProgramFeatureKind::Observation,
        "retain",
        "fields",
        fields.as_ref(),
    )
}

pub fn program_stage(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Reflection, "stage", name.as_ref())
}

pub fn program_preserve(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Reflection, "preserve", name.as_ref())
}

pub fn program_culminate(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Reflection, "culminate", name.as_ref())
}

pub fn program_unfold(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Logogenesis, "unfold", name.as_ref())
}

pub fn program_infer(name: impl AsRef<str>, condition: impl AsRef<str>) -> ProgramFeature {
    phrase(
        ProgramFeatureKind::Inference,
        "infer",
        name.as_ref(),
        condition.as_ref(),
    )
}

pub fn program_conclude(name: impl AsRef<str>, condition: impl AsRef<str>) -> ProgramFeature {
    phrase(
        ProgramFeatureKind::Inference,
        "conclude",
        name.as_ref(),
        condition.as_ref(),
    )
}

pub fn program_middle(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Syllogism, "middle", name.as_ref())
}

pub fn program_require(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Condition, "require", name.as_ref())
}

pub fn program_unify(names: impl AsRef<str>) -> ProgramFeature {
    phrase(
        ProgramFeatureKind::Principle,
        "unify",
        "forms",
        names.as_ref(),
    )
}

pub fn program_emit(name: impl AsRef<str>) -> ProgramFeature {
    declaration(ProgramFeatureKind::Procedure, "emit", name.as_ref())
}

fn declaration(kind: ProgramFeatureKind, prefix: &str, name: &str) -> ProgramFeature {
    let normalized_name = normalize_phrase(name);
    ProgramFeature::new(
        kind,
        format!("{prefix}::{normalized_name}"),
        format!("{prefix}::{normalized_name}"),
    )
}

fn phrase(kind: ProgramFeatureKind, prefix: &str, name: &str, expression: &str) -> ProgramFeature {
    let normalized_name = normalize_phrase(name);
    let normalized_expression = normalize_phrase(expression);
    ProgramFeature::new(
        kind,
        format!("{prefix}::{normalized_name}"),
        format!("{prefix}::{normalized_name}::{normalized_expression}"),
    )
}

fn normalize_phrase(value: &str) -> String {
    value
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, ':' | '_' | '-' | '.') {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_features_adds_specification_binding() {
        let features = program_features(
            "org.test.program",
            ["ConceptA"],
            [program_concept("ConceptA")],
        );

        assert_eq!(features.program_name, "org.test.program");
        assert_eq!(features.selected_forms, vec!["ConceptA"]);
        assert_eq!(
            features.features[0].kind,
            ProgramFeatureKind::SpecificationBinding
        );
        assert_eq!(features.features[1].value, "concept::concepta");
    }
}
