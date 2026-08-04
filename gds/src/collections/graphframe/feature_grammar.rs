//! Graph feature grammar checker.
//!
//! Graph feature language ownership lives in GraphFrame. Dataset may host
//! plugin contracts, but Graph semantics are defined here.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphFeatureStratum {
    Graph,
    Node,
    Edge,
}

impl GraphFeatureStratum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Graph => "Graph",
            Self::Node => "Node",
            Self::Edge => "Edge",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphFeatureValueType {
    Scalar,
    Vector,
    Symbolic,
    Distribution,
}

impl GraphFeatureValueType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Scalar => "Scalar",
            Self::Vector => "Vector",
            Self::Symbolic => "Symbolic",
            Self::Distribution => "Distribution",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphFeatureCardinality {
    One,
    Many,
}

impl GraphFeatureCardinality {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::One => "One",
            Self::Many => "Many",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphFeatureDerivationKind {
    Aggregate,
    Propagate,
    Compose,
    Infer,
}

impl GraphFeatureDerivationKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Aggregate => "Aggregate",
            Self::Propagate => "Propagate",
            Self::Compose => "Compose",
            Self::Infer => "Infer",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphFeatureGrammarErrorClass {
    ScopeCollapse,
    TypeCollapse,
    ProvenanceBreak,
    InvalidGrammar,
}

impl GraphFeatureGrammarErrorClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ScopeCollapse => "ScopeCollapse",
            Self::TypeCollapse => "TypeCollapse",
            Self::ProvenanceBreak => "ProvenanceBreak",
            Self::InvalidGrammar => "InvalidGrammar",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphFeatureAddress {
    pub stratum: GraphFeatureStratum,
    pub feature_name: String,
}

impl GraphFeatureAddress {
    pub fn new(stratum: GraphFeatureStratum, feature_name: impl Into<String>) -> Self {
        Self {
            stratum,
            feature_name: feature_name.into(),
        }
    }
}

impl fmt::Display for GraphFeatureAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.stratum.as_str(), self.feature_name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphFeatureRule {
    pub address: GraphFeatureAddress,
    pub value_type: GraphFeatureValueType,
    pub required: bool,
    pub cardinality: GraphFeatureCardinality,
}

impl GraphFeatureRule {
    pub fn new(
        stratum: GraphFeatureStratum,
        feature_name: impl Into<String>,
        value_type: GraphFeatureValueType,
        required: bool,
        cardinality: GraphFeatureCardinality,
    ) -> Self {
        Self {
            address: GraphFeatureAddress::new(stratum, feature_name),
            value_type,
            required,
            cardinality,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphFeatureDerivationRule {
    pub sources: Vec<GraphFeatureAddress>,
    pub target: GraphFeatureAddress,
    pub kind: GraphFeatureDerivationKind,
    pub trace_required: bool,
}

impl GraphFeatureDerivationRule {
    pub fn new(
        sources: Vec<GraphFeatureAddress>,
        target: GraphFeatureAddress,
        kind: GraphFeatureDerivationKind,
        trace_required: bool,
    ) -> Self {
        Self {
            sources,
            target,
            kind,
            trace_required,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphFeatureGrammarForm {
    pub name: String,
    pub version: String,
    pub feature_rules: Vec<GraphFeatureRule>,
    pub derivations: Vec<GraphFeatureDerivationRule>,
    pub norms: Vec<String>,
}

impl GraphFeatureGrammarForm {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            feature_rules: Vec::new(),
            derivations: Vec::new(),
            norms: Vec::new(),
        }
    }

    pub fn with_feature_rule(mut self, rule: GraphFeatureRule) -> Self {
        self.feature_rules.push(rule);
        self
    }

    pub fn with_derivation(mut self, rule: GraphFeatureDerivationRule) -> Self {
        self.derivations.push(rule);
        self
    }

    pub fn with_norm(mut self, norm: impl Into<String>) -> Self {
        self.norms.push(norm.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphFeatureGrammarChecked {
    form: GraphFeatureGrammarForm,
}

impl GraphFeatureGrammarChecked {
    pub fn form(&self) -> &GraphFeatureGrammarForm {
        &self.form
    }

    pub fn rule_graph_digest(&self) -> u64 {
        let mut feature_lines: Vec<String> = self
            .form
            .feature_rules
            .iter()
            .map(|rule| {
                format!(
                    "FEATURE|{}|{}|{}|{}|{}",
                    rule.address.stratum.as_str(),
                    rule.address.feature_name,
                    rule.value_type.as_str(),
                    rule.required,
                    rule.cardinality.as_str()
                )
            })
            .collect();
        feature_lines.sort();

        let mut derivation_lines: Vec<String> = self
            .form
            .derivations
            .iter()
            .map(|rule| {
                let mut srcs: Vec<String> = rule.sources.iter().map(ToString::to_string).collect();
                srcs.sort();
                format!(
                    "DERIVE|{}|{}|{}|{}",
                    rule.kind.as_str(),
                    srcs.join(","),
                    rule.target,
                    rule.trace_required
                )
            })
            .collect();
        derivation_lines.sort();

        let mut norm_lines = self.form.norms.clone();
        norm_lines.sort();

        let mut canonical = String::new();
        canonical.push_str("GRAPH-FEATURE-GRAMMAR\n");
        canonical.push_str(&self.form.name);
        canonical.push('\n');
        canonical.push_str(&self.form.version);
        canonical.push('\n');
        for line in feature_lines {
            canonical.push_str(&line);
            canonical.push('\n');
        }
        for line in derivation_lines {
            canonical.push_str(&line);
            canonical.push('\n');
        }
        for line in norm_lines {
            canonical.push_str("NORM|");
            canonical.push_str(&line);
            canonical.push('\n');
        }

        stable_fnv1a_64(canonical.as_bytes())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphFeatureGrammarError {
    pub class: GraphFeatureGrammarErrorClass,
    pub message: String,
}

impl GraphFeatureGrammarError {
    pub fn new(class: GraphFeatureGrammarErrorClass, message: impl Into<String>) -> Self {
        Self {
            class,
            message: message.into(),
        }
    }
}

impl fmt::Display for GraphFeatureGrammarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.class.as_str(), self.message)
    }
}

impl std::error::Error for GraphFeatureGrammarError {}

pub fn validate_graph_feature_grammar(
    form: GraphFeatureGrammarForm,
) -> Result<GraphFeatureGrammarChecked, GraphFeatureGrammarError> {
    if form.name.trim().is_empty() {
        return Err(GraphFeatureGrammarError::new(
            GraphFeatureGrammarErrorClass::InvalidGrammar,
            "grammar name must not be empty",
        ));
    }
    if form.version.trim().is_empty() {
        return Err(GraphFeatureGrammarError::new(
            GraphFeatureGrammarErrorClass::InvalidGrammar,
            "grammar version must not be empty",
        ));
    }

    let mut seen_features = BTreeSet::<(GraphFeatureStratum, String)>::new();
    let mut type_index = BTreeMap::<(GraphFeatureStratum, String), GraphFeatureValueType>::new();

    for rule in &form.feature_rules {
        let key = (rule.address.stratum, rule.address.feature_name.clone());
        if !seen_features.insert(key.clone()) {
            return Err(GraphFeatureGrammarError::new(
                GraphFeatureGrammarErrorClass::TypeCollapse,
                format!("duplicate feature declaration for {}", rule.address),
            ));
        }
        type_index.insert(key, rule.value_type);
    }

    for derivation in &form.derivations {
        if derivation.sources.is_empty() {
            return Err(GraphFeatureGrammarError::new(
                GraphFeatureGrammarErrorClass::InvalidGrammar,
                format!("derivation {} has no sources", derivation.target),
            ));
        }

        for source in &derivation.sources {
            let source_key = (source.stratum, source.feature_name.clone());
            if !type_index.contains_key(&source_key) {
                return Err(GraphFeatureGrammarError::new(
                    GraphFeatureGrammarErrorClass::TypeCollapse,
                    format!("derivation source {} is not declared", source),
                ));
            }
        }

        let target_key = (
            derivation.target.stratum,
            derivation.target.feature_name.clone(),
        );
        if !type_index.contains_key(&target_key) {
            return Err(GraphFeatureGrammarError::new(
                GraphFeatureGrammarErrorClass::TypeCollapse,
                format!("derivation target {} is not declared", derivation.target),
            ));
        }

        let crosses_strata = derivation
            .sources
            .iter()
            .any(|source| source.stratum != derivation.target.stratum);
        if crosses_strata && !derivation.trace_required {
            return Err(GraphFeatureGrammarError::new(
                GraphFeatureGrammarErrorClass::ProvenanceBreak,
                format!(
                    "cross-strata derivation {} requires trace=true",
                    derivation.target
                ),
            ));
        }

        match derivation.kind {
            GraphFeatureDerivationKind::Aggregate => {
                if derivation.target.stratum != GraphFeatureStratum::Graph {
                    return Err(GraphFeatureGrammarError::new(
                        GraphFeatureGrammarErrorClass::ScopeCollapse,
                        format!(
                            "Aggregate target {} must be Graph stratum",
                            derivation.target
                        ),
                    ));
                }
                if derivation
                    .sources
                    .iter()
                    .any(|source| source.stratum == GraphFeatureStratum::Graph)
                {
                    return Err(GraphFeatureGrammarError::new(
                        GraphFeatureGrammarErrorClass::TypeCollapse,
                        format!(
                            "Aggregate source must be Node/Edge, found Graph source for {}",
                            derivation.target
                        ),
                    ));
                }
            }
            GraphFeatureDerivationKind::Propagate => {
                if derivation.target.stratum == GraphFeatureStratum::Graph {
                    return Err(GraphFeatureGrammarError::new(
                        GraphFeatureGrammarErrorClass::ScopeCollapse,
                        format!(
                            "Propagate target {} must be Node or Edge stratum",
                            derivation.target
                        ),
                    ));
                }
            }
            GraphFeatureDerivationKind::Compose => {}
            GraphFeatureDerivationKind::Infer => {
                if form.norms.is_empty() {
                    return Err(GraphFeatureGrammarError::new(
                        GraphFeatureGrammarErrorClass::InvalidGrammar,
                        format!(
                            "Infer derivation for {} requires at least one norm",
                            derivation.target
                        ),
                    ));
                }
            }
        }
    }

    Ok(GraphFeatureGrammarChecked { form })
}

fn stable_fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for b in bytes {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3u64);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(stratum: GraphFeatureStratum, name: &str) -> GraphFeatureAddress {
        GraphFeatureAddress::new(stratum, name)
    }

    fn base_form() -> GraphFeatureGrammarForm {
        GraphFeatureGrammarForm::new("citation_graph", "v1")
            .with_feature_rule(GraphFeatureRule::new(
                GraphFeatureStratum::Graph,
                "density",
                GraphFeatureValueType::Scalar,
                true,
                GraphFeatureCardinality::One,
            ))
            .with_feature_rule(GraphFeatureRule::new(
                GraphFeatureStratum::Node,
                "pagerank",
                GraphFeatureValueType::Scalar,
                false,
                GraphFeatureCardinality::One,
            ))
            .with_feature_rule(GraphFeatureRule::new(
                GraphFeatureStratum::Edge,
                "weight",
                GraphFeatureValueType::Scalar,
                false,
                GraphFeatureCardinality::One,
            ))
    }

    #[test]
    fn reject_duplicate_feature_in_same_stratum() {
        let form = base_form().with_feature_rule(GraphFeatureRule::new(
            GraphFeatureStratum::Node,
            "pagerank",
            GraphFeatureValueType::Scalar,
            false,
            GraphFeatureCardinality::One,
        ));

        let err = validate_graph_feature_grammar(form).expect_err("duplicate should fail");
        assert_eq!(err.class, GraphFeatureGrammarErrorClass::TypeCollapse);
    }

    #[test]
    fn reject_cross_strata_derivation_without_trace() {
        let form = base_form().with_derivation(GraphFeatureDerivationRule::new(
            vec![addr(GraphFeatureStratum::Node, "pagerank")],
            addr(GraphFeatureStratum::Graph, "density"),
            GraphFeatureDerivationKind::Aggregate,
            false,
        ));

        let err = validate_graph_feature_grammar(form).expect_err("missing trace should fail");
        assert_eq!(err.class, GraphFeatureGrammarErrorClass::ProvenanceBreak);
    }

    #[test]
    fn reject_aggregate_target_outside_graph_stratum() {
        let form = base_form().with_derivation(GraphFeatureDerivationRule::new(
            vec![addr(GraphFeatureStratum::Edge, "weight")],
            addr(GraphFeatureStratum::Node, "pagerank"),
            GraphFeatureDerivationKind::Aggregate,
            true,
        ));

        let err = validate_graph_feature_grammar(form)
            .expect_err("aggregate to non-graph target should fail");
        assert_eq!(err.class, GraphFeatureGrammarErrorClass::ScopeCollapse);
    }

    #[test]
    fn digest_is_deterministic_for_same_grammar() {
        let form = base_form().with_derivation(GraphFeatureDerivationRule::new(
            vec![addr(GraphFeatureStratum::Edge, "weight")],
            addr(GraphFeatureStratum::Graph, "density"),
            GraphFeatureDerivationKind::Aggregate,
            true,
        ));

        let checked_a = validate_graph_feature_grammar(form.clone()).expect("valid form");
        let checked_b = validate_graph_feature_grammar(form).expect("valid form");
        assert_eq!(checked_a.rule_graph_digest(), checked_b.rule_graph_digest());
    }

    #[test]
    fn accepts_well_typed_baseline() {
        let form = base_form().with_derivation(GraphFeatureDerivationRule::new(
            vec![addr(GraphFeatureStratum::Edge, "weight")],
            addr(GraphFeatureStratum::Graph, "density"),
            GraphFeatureDerivationKind::Aggregate,
            true,
        ));

        let checked = validate_graph_feature_grammar(form).expect("baseline must pass");
        assert!(checked.rule_graph_digest() > 0);
        assert_eq!(checked.form().name, "citation_graph");
    }
}
