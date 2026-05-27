use std::collections::HashMap;

use crate::collections::dataset::feature::featstruct::format_featstruct;
use crate::collections::dataset::feature::featstruct::parse_featstruct;
use crate::collections::dataset::feature::featstruct::subsumes_featstruct;
use crate::collections::dataset::feature::featstruct::unify_featstruct;
use crate::collections::dataset::feature::featstruct::FeatBindings;
use crate::collections::dataset::feature::featstruct::FeatStruct;
use crate::collections::dataset::feature::featstruct::FeatValue;
use crate::ml::nlp::classify::classify::Classifier;
use crate::ml::nlp::classify::decision_tree::DecisionTreeClassifier;
use crate::ml::nlp::classify::naive_bayes::NaiveBayesClassifier;
use crate::ml::nlp::classify::util::accuracy;
use crate::ml::nlp::classify::util::log_likelihood;
use crate::ml::nlp::classify::util::names_demo_features;
use crate::ml::nlp::classify::util::FeatureValue;
use crate::ml::nlp::inference::api::Prover;
use crate::ml::nlp::inference::resolution::ResolutionProver;
use crate::ml::nlp::sem::evaluate::Assignment;
use crate::ml::nlp::sem::evaluate::EvalValue;
use crate::ml::nlp::sem::evaluate::Model;
use crate::ml::nlp::sem::evaluate::Valuation;
use crate::ml::nlp::sem::logic;
use crate::ml::nlp::sem::skolemize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct NlpWorkbenchExperiment {
    pub id: &'static str,
    pub name: &'static str,
    pub focus: &'static str,
}

/// A single formula probed against the canonical model.
#[derive(Clone, Debug, Serialize)]
pub struct LogicProbe {
    pub label: &'static str,
    pub expression: String,
    pub parsed: String,
    pub evaluation: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct LogicPreviewReport {
    pub experiment: &'static str,
    /// Individuals in the model: fido → d1, catto → c1.
    pub model_domain: Vec<String>,
    /// Four probes covering universals, existentials, negation, equality.
    pub probes: Vec<LogicProbe>,
    /// Lambda beta-reduction: input → simplified.
    pub beta_input: String,
    pub beta_output: String,
    /// Skolemization: input → output with Skolem function.
    pub skolem_input: String,
    pub skolem_output: String,
}

pub fn available_experiments() -> &'static [NlpWorkbenchExperiment] {
    &[
        NlpWorkbenchExperiment {
            id: "logic-preview",
            name: "First-Order Logic Parse and Evaluate Preview",
            focus: "sem::logic + sem::evaluate + sem::skolemize",
        },
        NlpWorkbenchExperiment {
            id: "inference-preview",
            name: "First-Order Resolution Inference Preview",
            focus: "inference::resolution + sem::logic",
        },
        NlpWorkbenchExperiment {
            id: "featstruct-preview",
            name: "Feature Structure Predicate Algebra Preview",
            focus: "collections::dataset::feature::featstruct",
        },
        NlpWorkbenchExperiment {
            id: "classify-preview",
            name: "NLP Classify FeatureSet Preview",
            focus: "classify::util + naive_bayes + decision_tree",
        },
        NlpWorkbenchExperiment {
            id: "logic-processor-preview",
            name: "Semantic Dataset Logic Processor Preview",
            focus: "featstruct -> classify features -> entailment-style labels",
        },
    ]
}

pub fn run_logic_preview() -> LogicPreviewReport {
    // Canonical model: dog = {d1}, animal = {d1, c1}, fido→d1, catto→c1.
    let valuation = Valuation::from_str(
        r#"
fido => d1
catto => c1
dog => {d1}
animal => {d1, c1}
"#,
    )
    .expect("logic preview valuation should parse");

    let domain = valuation.domain();
    let model = Model::new(domain.clone(), valuation).expect("logic preview model should be valid");
    let assignment = Assignment::new(domain.clone());

    let probe_exprs: &[(&'static str, &str)] = &[
        ("all dogs are animals", "all x. (dog(x) -> animal(x))"),
        (
            "all animals are dogs (false)",
            "all x. (animal(x) -> dog(x))",
        ),
        (
            "some non-dog animal exists",
            "exists x. (animal(x) & -dog(x))",
        ),
        ("fido equals catto (false)", "fido = catto"),
    ];

    let probes = probe_exprs
        .iter()
        .map(|(label, expr)| {
            let parsed = logic::parse(expr).expect("probe expression should parse");
            let value = model
                .satisfy(&parsed, &assignment)
                .expect("probe evaluation should succeed");
            LogicProbe {
                label,
                expression: expr.to_string(),
                parsed: parsed.to_string(),
                evaluation: eval_value_to_string(&value),
            }
        })
        .collect();

    // Beta-reduction: (λx. dog(x))(fido) → dog(fido)
    // Parentheses around the lambda force app(lambda, arg) rather than lambda(x, body(fido)).
    let beta_expr = r"(\x. dog(x))(fido)";
    let beta_parsed = logic::parse(beta_expr).expect("beta expression should parse");
    let beta_output = beta_parsed.simplify().to_string();

    // Skolemization: all x. exists y. loves(x, y) → all x. loves(x, sk(x))
    skolemize::reset_skolem_counter();
    let skolem_expr = "all x. exists y. loves(x, y)";
    let skolem_parsed = logic::parse(skolem_expr).expect("skolem expression should parse");
    let skolem_output = skolemize::skolemize(&skolem_parsed).to_string();

    let mut model_domain: Vec<String> = domain.into_iter().collect();
    model_domain.sort();

    LogicPreviewReport {
        experiment: "logic-preview",
        model_domain,
        probes,
        beta_input: beta_expr.to_string(),
        beta_output,
        skolem_input: skolem_expr.to_string(),
        skolem_output,
    }
}

// ---------------------------------------------------------------------------
// inference-preview
// ---------------------------------------------------------------------------

/// A single proof attempt: goal + assumptions + result.
#[derive(Clone, Debug, Serialize)]
pub struct InferenceProbe {
    pub label: &'static str,
    pub assumptions: Vec<String>,
    pub goal: String,
    pub proved: bool,
    /// Resolution clause trace (each entry is one derived clause).
    pub clauses: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct InferencePreviewReport {
    pub experiment: &'static str,
    pub prover: &'static str,
    pub probes: Vec<InferenceProbe>,
}

pub fn run_inference_preview() -> InferencePreviewReport {
    let prover = ResolutionProver::new();

    let probe_specs: &[(&'static str, &[&str], &str)] = &[
        (
            "socrates is mortal (proved)",
            &["all x. (man(x) -> mortal(x))", "man(socrates)"],
            "mortal(socrates)",
        ),
        (
            "contrapositive: not-mortal implies not-man (proved)",
            &["all x. (man(x) -> mortal(x))", "-mortal(socrates)"],
            "-man(socrates)",
        ),
        (
            "mortal without universal premise (not proved)",
            &["man(socrates)"],
            "mortal(socrates)",
        ),
        (
            "excluded middle tautology (proved)",
            &[],
            "man(socrates) | -man(socrates)",
        ),
    ];

    let probes = probe_specs
        .iter()
        .map(|(label, assumption_strs, goal_str)| {
            let goal = logic::parse(goal_str).expect("inference probe goal should parse");
            let assumptions: Vec<_> = assumption_strs
                .iter()
                .map(|s| logic::parse(s).expect("inference probe assumption should parse"))
                .collect();
            let result = prover
                .prove(Some(&goal), &assumptions, true)
                .expect("inference probe should not error");
            let clauses = result
                .proof
                .lines()
                .map(|line| line.to_string())
                .filter(|line| !line.is_empty())
                .collect();
            InferenceProbe {
                label,
                assumptions: assumption_strs.iter().map(|s| s.to_string()).collect(),
                goal: goal_str.to_string(),
                proved: result.proved,
                clauses,
            }
        })
        .collect();

    InferencePreviewReport {
        experiment: "inference-preview",
        prover: "ResolutionProver",
        probes,
    }
}

// ---------------------------------------------------------------------------
// featstruct-preview
// ---------------------------------------------------------------------------

/// One parse/format round-trip probe.
#[derive(Clone, Debug, Serialize)]
pub struct FeatStructProbe {
    pub label: &'static str,
    pub input: String,
    pub formatted: String,
}

/// One unification probe: two structures unified, result or clash.
#[derive(Clone, Debug, Serialize)]
pub struct UnifyProbe {
    pub label: &'static str,
    pub left: String,
    pub right: String,
    pub result: Option<String>,
    pub unified: bool,
}

/// One subsumption probe: does `general` subsume `specific`?
#[derive(Clone, Debug, Serialize)]
pub struct SubsumeProbe {
    pub label: &'static str,
    pub general: String,
    pub specific: String,
    pub subsumes: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct FeatStructPreviewReport {
    pub experiment: &'static str,
    pub structures: Vec<FeatStructProbe>,
    pub unifications: Vec<UnifyProbe>,
    pub subsumptions: Vec<SubsumeProbe>,
}

pub fn run_featstruct_preview() -> FeatStructPreviewReport {
    // --- parse/format round-trips ---
    let struct_specs: &[(&'static str, &str)] = &[
        ("simple NP", "[CAT=np, NUM=sg]"),
        ("nested agreement", "[CAT=np, AGR=[NUM=sg, PERS=3]]"),
        ("verb phrase", "[CAT=vp, +fin, AGR=[NUM=?n]]"),
    ];
    let structures = struct_specs
        .iter()
        .map(|(label, input)| {
            let parsed = parse_featstruct(input).expect("featstruct probe should parse");
            let formatted = format_featstruct(&parsed);
            FeatStructProbe {
                label,
                input: input.to_string(),
                formatted,
            }
        })
        .collect();

    // --- unification probes ---
    let unify_specs: &[(&'static str, &str, &str)] = &[
        ("merge disjoint features", "[NUM=sg]", "[PERS=3]"),
        ("conflict on NUM", "[NUM=sg]", "[NUM=pl]"),
        ("variable binds to value", "[NUM=?n]", "[NUM=sg]"),
        (
            "full NP merge with agreement",
            "[CAT=np, NUM=sg]",
            "[NUM=sg, PERS=3]",
        ),
    ];
    let unifications = unify_specs
        .iter()
        .map(|(label, left_str, right_str)| {
            let left = parse_featstruct(left_str).expect("unify left should parse");
            let right = parse_featstruct(right_str).expect("unify right should parse");
            let mut bindings = FeatBindings::new();
            let result = unify_featstruct(&left, &right, Some(&mut bindings));
            let result_str = result.as_ref().map(format_featstruct);
            let unified = result.is_some();
            UnifyProbe {
                label,
                left: left_str.to_string(),
                right: right_str.to_string(),
                result: result_str,
                unified,
            }
        })
        .collect();

    // --- subsumption probes ---
    let subsume_specs: &[(&'static str, &str, &str)] = &[
        ("general subsumes specific", "[CAT=np]", "[CAT=np, NUM=sg]"),
        (
            "specific does not subsume general",
            "[CAT=np, NUM=sg]",
            "[CAT=np]",
        ),
        ("variable subsumes value", "[NUM=?n]", "[NUM=sg]"),
    ];
    let subsumptions = subsume_specs
        .iter()
        .map(|(label, general_str, specific_str)| {
            let general = parse_featstruct(general_str).expect("subsume general should parse");
            let specific = parse_featstruct(specific_str).expect("subsume specific should parse");
            let subsumes = subsumes_featstruct(&general, &specific);
            SubsumeProbe {
                label,
                general: general_str.to_string(),
                specific: specific_str.to_string(),
                subsumes,
            }
        })
        .collect();

    FeatStructPreviewReport {
        experiment: "featstruct-preview",
        structures,
        unifications,
        subsumptions,
    }
}

// ---------------------------------------------------------------------------
// classify-preview
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize)]
pub struct LabelScore {
    pub label: String,
    pub prob: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClassifyCase {
    pub text: String,
    pub expected: String,
    pub naive_bayes_predicted: String,
    pub decision_tree_predicted: String,
    pub naive_bayes_probs: Vec<LabelScore>,
    pub feature_keys: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClassifyPreviewReport {
    pub experiment: &'static str,
    pub featureset_shape: &'static str,
    pub algorithms: Vec<&'static str>,
    pub train_size: usize,
    pub test_size: usize,
    pub naive_bayes_accuracy: f64,
    pub decision_tree_accuracy: f64,
    pub naive_bayes_log_likelihood: f64,
    pub decision_tree_log_likelihood: f64,
    pub decision_tree_structure: String,
    pub names_demo_feature_sample: Vec<String>,
    pub cases: Vec<ClassifyCase>,
}

pub fn run_classify_preview() -> ClassifyPreviewReport {
    let train_specs: &[(&str, &str)] = &[
        ("goal team win stadium", "sports"),
        ("coach team match season", "sports"),
        ("stock market bond trade", "finance"),
        ("bank credit market loan", "finance"),
        ("player score team", "sports"),
        ("fund profit investor", "finance"),
    ];

    let train: Vec<(HashMap<String, FeatureValue>, &'static str)> = train_specs
        .iter()
        .map(|(text, label)| (topic_features(text), *label))
        .collect();

    let naive_bayes = NaiveBayesClassifier::train(&train, 1.0);
    let decision_tree = DecisionTreeClassifier::train(&train, 0.0, 3, 0);

    let test_specs: &[(&str, &str)] = &[
        ("team goal player", "sports"),
        ("market stock investor", "finance"),
        ("coach trade stadium", "sports"),
    ];
    let test: Vec<(HashMap<String, FeatureValue>, &'static str)> = test_specs
        .iter()
        .map(|(text, label)| (topic_features(text), *label))
        .collect();

    let nb_predictions = naive_bayes.classify_many(
        &test
            .iter()
            .map(|(features, _)| features.clone())
            .collect::<Vec<_>>(),
    );
    let dt_predictions = decision_tree.classify_many(
        &test
            .iter()
            .map(|(features, _)| features.clone())
            .collect::<Vec<_>>(),
    );
    let nb_probs = naive_bayes.prob_classify_many(
        &test
            .iter()
            .map(|(features, _)| features.clone())
            .collect::<Vec<_>>(),
    );

    let cases = test_specs
        .iter()
        .zip(test.iter())
        .enumerate()
        .map(|(index, ((text, expected), (features, _)))| {
            let mut feature_keys: Vec<String> = features.keys().cloned().collect();
            feature_keys.sort();
            let probs = label_scores(nb_probs[index].as_map());
            ClassifyCase {
                text: (*text).to_string(),
                expected: (*expected).to_string(),
                naive_bayes_predicted: nb_predictions[index].to_string(),
                decision_tree_predicted: dt_predictions[index].to_string(),
                naive_bayes_probs: probs,
                feature_keys,
            }
        })
        .collect();

    let tree_preview = decision_tree.pretty_format(72, "", 3);
    let names_sample = feature_value_sample("Socrates");

    ClassifyPreviewReport {
        experiment: "classify-preview",
        featureset_shape: "HashMap<String, FeatureValue>",
        algorithms: vec!["NaiveBayesClassifier", "DecisionTreeClassifier"],
        train_size: train.len(),
        test_size: test.len(),
        naive_bayes_accuracy: accuracy(&naive_bayes, &test),
        decision_tree_accuracy: accuracy(&decision_tree, &test),
        naive_bayes_log_likelihood: log_likelihood(&naive_bayes, &test),
        decision_tree_log_likelihood: log_likelihood(&decision_tree, &test),
        decision_tree_structure: tree_preview,
        names_demo_feature_sample: names_sample,
        cases,
    }
}

fn topic_features(text: &str) -> HashMap<String, FeatureValue> {
    let mut map = HashMap::new();
    let lower = text.to_lowercase();

    map.insert("alwayson".to_string(), FeatureValue::Bool(true));
    map.insert(
        "len_tokens".to_string(),
        FeatureValue::Int(lower.split_whitespace().count() as i64),
    );

    for token in [
        "goal", "team", "player", "coach", "stadium", "score", "market", "stock", "bank", "bond",
        "fund", "investor", "trade", "profit", "credit", "loan",
    ] {
        map.insert(
            format!("contains({token})"),
            FeatureValue::Bool(lower.contains(token)),
        );
    }

    map
}

fn feature_value_sample(name: &str) -> Vec<String> {
    let features = names_demo_features(name);
    let mut keys: Vec<String> = vec![
        "alwayson".to_string(),
        "startswith".to_string(),
        "endswith".to_string(),
        "count(s)".to_string(),
    ];
    keys.sort();

    keys.into_iter()
        .filter_map(|key| {
            features
                .get(&key)
                .map(|value| format!("{key}={}", feature_value_to_string(value)))
        })
        .collect()
}

fn feature_value_to_string(value: &FeatureValue) -> String {
    match value {
        FeatureValue::Bool(boolean) => boolean.to_string(),
        FeatureValue::Int(integer) => integer.to_string(),
        FeatureValue::Float(float) => format!("{float:.4}"),
        FeatureValue::Text(text) => text.to_string(),
    }
}

fn label_scores(labels: &HashMap<&str, f64>) -> Vec<LabelScore> {
    let mut scores: Vec<LabelScore> = labels
        .iter()
        .map(|(label, prob)| LabelScore {
            label: (*label).to_string(),
            prob: *prob,
        })
        .collect();
    scores.sort_by(|left, right| {
        right
            .prob
            .partial_cmp(&left.prob)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    scores
}

// ---------------------------------------------------------------------------
// logic-processor-preview
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize)]
pub struct LogicProcessorCase {
    pub premise: String,
    pub hypothesis: String,
    pub semantic_record: String,
    pub expected: String,
    pub predicted: String,
    pub probs: Vec<LabelScore>,
    pub derived_features: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct LogicProcessorPreviewReport {
    pub experiment: &'static str,
    pub semantics_shape: &'static str,
    pub classifier: &'static str,
    pub labels: Vec<&'static str>,
    pub train_size: usize,
    pub test_size: usize,
    pub accuracy: f64,
    pub log_likelihood: f64,
    pub cases: Vec<LogicProcessorCase>,
}

pub fn run_logic_processor_preview() -> LogicProcessorPreviewReport {
    let train_specs: &[(&str, &str, &str, &str)] = &[
        (
            "all dogs bark",
            "fido barks",
            "[OVERLAP=true, NEG_MISMATCH=false, HYP_EXTRA=0, MONOTONIC=true]",
            "entails",
        ),
        (
            "dogs bark",
            "dogs do not bark",
            "[OVERLAP=true, NEG_MISMATCH=true, HYP_EXTRA=0, MONOTONIC=true]",
            "contradiction",
        ),
        (
            "some dogs bark",
            "all dogs bark",
            "[OVERLAP=true, NEG_MISMATCH=false, HYP_EXTRA=1, MONOTONIC=false]",
            "unknown",
        ),
        (
            "cats sleep",
            "dogs bark",
            "[OVERLAP=false, NEG_MISMATCH=false, HYP_EXTRA=2, MONOTONIC=true]",
            "unknown",
        ),
        (
            "no dogs bark",
            "some dogs bark",
            "[OVERLAP=true, NEG_MISMATCH=true, HYP_EXTRA=0, MONOTONIC=false]",
            "contradiction",
        ),
        (
            "all mammals breathe",
            "dogs breathe",
            "[OVERLAP=true, NEG_MISMATCH=false, HYP_EXTRA=0, MONOTONIC=true]",
            "entails",
        ),
    ];

    let train: Vec<(HashMap<String, FeatureValue>, &'static str)> = train_specs
        .iter()
        .map(|(_, _, semantic_record, label)| (logic_record_to_features(semantic_record), *label))
        .collect();

    let classifier = NaiveBayesClassifier::train(&train, 1.0);

    let test_specs: &[(&str, &str, &str, &str)] = &[
        (
            "all birds fly",
            "sparrow flies",
            "[OVERLAP=true, NEG_MISMATCH=false, HYP_EXTRA=0, MONOTONIC=true]",
            "entails",
        ),
        (
            "dogs bark",
            "dogs are silent",
            "[OVERLAP=true, NEG_MISMATCH=true, HYP_EXTRA=0, MONOTONIC=true]",
            "contradiction",
        ),
        (
            "some students passed",
            "all students passed",
            "[OVERLAP=true, NEG_MISMATCH=false, HYP_EXTRA=1, MONOTONIC=false]",
            "unknown",
        ),
    ];
    let test: Vec<(HashMap<String, FeatureValue>, &'static str)> = test_specs
        .iter()
        .map(|(_, _, semantic_record, label)| (logic_record_to_features(semantic_record), *label))
        .collect();

    let test_features: Vec<HashMap<String, FeatureValue>> =
        test.iter().map(|(features, _)| features.clone()).collect();
    let predictions = classifier.classify_many(&test_features);
    let prob_dists = classifier.prob_classify_many(&test_features);

    let cases = test_specs
        .iter()
        .zip(test_features.iter())
        .enumerate()
        .map(
            |(index, ((premise, hypothesis, semantic_record, expected), features))| {
                let mut derived_features: Vec<String> = features
                    .iter()
                    .map(|(name, value)| format!("{name}={}", feature_value_to_string(value)))
                    .collect();
                derived_features.sort();

                LogicProcessorCase {
                    premise: (*premise).to_string(),
                    hypothesis: (*hypothesis).to_string(),
                    semantic_record: (*semantic_record).to_string(),
                    expected: (*expected).to_string(),
                    predicted: predictions[index].to_string(),
                    probs: label_scores(prob_dists[index].as_map()),
                    derived_features,
                }
            },
        )
        .collect();

    LogicProcessorPreviewReport {
        experiment: "logic-processor-preview",
        semantics_shape: "FeatStruct Dict -> HashMap<String, FeatureValue>",
        classifier: "NaiveBayesClassifier",
        labels: vec!["entails", "contradiction", "unknown"],
        train_size: train.len(),
        test_size: test.len(),
        accuracy: accuracy(&classifier, &test),
        log_likelihood: log_likelihood(&classifier, &test),
        cases,
    }
}

fn logic_record_to_features(semantic_record: &str) -> HashMap<String, FeatureValue> {
    let parsed = parse_featstruct(semantic_record).expect("semantic record should parse");
    let mut features = HashMap::new();
    features.insert("alwayson".to_string(), FeatureValue::Bool(true));

    let FeatStruct::Dict(entries) = parsed else {
        return features;
    };

    for (name, value) in entries {
        if let Some(mapped) = map_featvalue_to_feature(&value) {
            features.insert(name.to_lowercase(), mapped);
        }
    }

    features
}

fn map_featvalue_to_feature(value: &FeatValue) -> Option<FeatureValue> {
    match value {
        FeatValue::Bool(boolean) => Some(FeatureValue::Bool(*boolean)),
        FeatValue::Number(number) => Some(FeatureValue::Int(*number)),
        FeatValue::Text(text) => Some(FeatureValue::Text(text.clone())),
        FeatValue::Null => Some(FeatureValue::Text("none".to_string())),
        FeatValue::Empty => Some(FeatureValue::Text("empty".to_string())),
        FeatValue::Variable(name) => Some(FeatureValue::Text(format!("?{name}"))),
        FeatValue::BytesRange { start, end } => Some(FeatureValue::Text(format!("{start}:{end}"))),
        FeatValue::Struct(structure) => Some(FeatureValue::Text(format_featstruct(structure))),
        FeatValue::List(values) => Some(FeatureValue::Text(format!("list:{}", values.len()))),
        FeatValue::Reentrant(id) => Some(FeatureValue::Text(format!("reentrant:{}", id.0))),
        FeatValue::ReentranceDef { id, value } => map_featvalue_to_feature(value)
            .or_else(|| Some(FeatureValue::Text(format!("reentrance_def:{}", id.0)))),
    }
}

fn eval_value_to_string(value: &EvalValue) -> String {
    match value {
        EvalValue::Bool(boolean) => boolean.to_string(),
        EvalValue::Individual(individual) => format!("individual:{individual}"),
        EvalValue::Relation(relation) => format!("relation:{}", relation.len()),
        EvalValue::Function(function_table) => format!("function:{}", function_table.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nlp_workbench_catalog_has_logic_preview() {
        let experiments = available_experiments();
        assert!(experiments.iter().any(|item| item.id == "logic-preview"));
        assert!(experiments.iter().any(|item| item.id == "classify-preview"));
        assert!(experiments
            .iter()
            .any(|item| item.id == "logic-processor-preview"));
    }

    #[test]
    fn logic_preview_runs_and_reports_semantic_shape() {
        let report = run_logic_preview();

        assert_eq!(report.experiment, "logic-preview");
        assert_eq!(report.probes.len(), 4);
        assert_eq!(report.probes[0].evaluation, "true");
        assert_eq!(report.probes[1].evaluation, "false");
        assert_eq!(report.probes[2].evaluation, "true");
        assert_eq!(report.probes[3].evaluation, "false");
    }

    #[test]
    fn logic_preview_beta_reduces_lambda() {
        let report = run_logic_preview();
        assert_eq!(report.beta_output, "dog(fido)");
    }

    #[test]
    fn logic_preview_skolemizes_existential() {
        let report = run_logic_preview();
        // skolemize strips universal quantifiers and introduces a Skolem function.
        assert!(!report.skolem_output.contains("exists"));
        // A Skolem function variable (uppercase) should appear in the output.
        assert!(report.skolem_output.chars().any(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn inference_preview_proves_socrates_syllogism() {
        let report = run_inference_preview();
        assert_eq!(report.prover, "ResolutionProver");
        assert_eq!(report.probes.len(), 4);
        assert!(report.probes[0].proved, "socrates should be proved mortal");
        assert!(report.probes[1].proved, "contrapositive should be proved");
        assert!(
            !report.probes[2].proved,
            "missing premise should not be proved"
        );
        assert!(report.probes[3].proved, "excluded middle should be proved");
    }

    #[test]
    fn featstruct_preview_runs_and_reports_shape() {
        let report = run_featstruct_preview();
        assert_eq!(report.experiment, "featstruct-preview");
        assert_eq!(report.structures.len(), 3);
        assert_eq!(report.unifications.len(), 4);
        assert_eq!(report.subsumptions.len(), 3);
        // merge disjoint
        assert!(
            report.unifications[0].unified,
            "disjoint features should unify"
        );
        // conflict
        assert!(!report.unifications[1].unified, "NUM conflict should fail");
        // variable binds
        assert!(
            report.unifications[2].unified,
            "variable should bind to value"
        );
        // general subsumes specific
        assert!(
            report.subsumptions[0].subsumes,
            "general should subsume specific"
        );
        // specific does not subsume general
        assert!(
            !report.subsumptions[1].subsumes,
            "specific should not subsume general"
        );
        // variable subsumes value
        assert!(
            report.subsumptions[2].subsumes,
            "variable should subsume value"
        );
    }

    #[test]
    fn classify_preview_runs_and_reports_shape() {
        let report = run_classify_preview();

        assert_eq!(report.experiment, "classify-preview");
        assert_eq!(report.train_size, 6);
        assert_eq!(report.test_size, 3);
        assert_eq!(report.algorithms.len(), 2);
        assert_eq!(report.cases.len(), 3);
        assert!(
            report.naive_bayes_accuracy >= 0.66,
            "naive bayes should do reasonably on toy data"
        );
        assert!(
            report.decision_tree_accuracy >= 0.66,
            "decision tree should do reasonably on toy data"
        );
        assert!(
            report
                .names_demo_feature_sample
                .iter()
                .any(|entry| entry.starts_with("alwayson=")),
            "names demo sample should include alwayson"
        );
    }

    #[test]
    fn logic_processor_preview_runs_and_reports_shape() {
        let report = run_logic_processor_preview();

        assert_eq!(report.experiment, "logic-processor-preview");
        assert_eq!(report.train_size, 6);
        assert_eq!(report.test_size, 3);
        assert_eq!(report.cases.len(), 3);
        assert!(report.accuracy >= 0.66, "preview should classify toy cases");
        assert!(
            report.cases[0]
                .derived_features
                .iter()
                .any(|feature| feature.starts_with("overlap=")),
            "derived features should contain overlap"
        );
    }
}
