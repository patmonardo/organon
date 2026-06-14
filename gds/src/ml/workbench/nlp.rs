use std::collections::HashMap;
use std::sync::Arc;

use crate::collections::dataset::feature::featstruct::format_featstruct;
use crate::collections::dataset::feature::featstruct::parse_featstruct;
use crate::collections::dataset::feature::featstruct::subsumes_featstruct;
use crate::collections::dataset::feature::featstruct::unify_featstruct;
use crate::collections::dataset::feature::featstruct::FeatBindings;
use crate::collections::dataset::feature::featstruct::FeatStruct;
use crate::collections::dataset::feature::featstruct::FeatValue;
use crate::collections::HugeIntArray;
use crate::task::concurrency::Concurrency;
use crate::task::concurrency::TerminationFlag;
use crate::core::utils::progress::TaskProgressTracker;
use crate::core::utils::progress::Tasks;
use crate::ml::decision_tree::ClassifierImpurityCriterionType;
use crate::ml::metrics::ModelSpecificMetricsHandler;
use crate::ml::models::features::DenseFeatures;
use crate::ml::models::random_forest::RandomForestClassifierTrainer;
use crate::ml::models::random_forest::RandomForestClassifierTrainerConfig;
use crate::ml::models::random_forest::RandomForestConfig;
use crate::ml::models::ClassifierTrainer;
use crate::ml::models::Features;
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
            id: "cluster-preview",
            name: "NLP Token Similarity Cluster Preview",
            focus: "small-text DAG-ish affinity clusters",
        },
        NlpWorkbenchExperiment {
            id: "decision-tree-experiment",
            name: "NLP vs ML Decision Tree Experiment",
            focus: "classify methods + dense projection + ml::decision_tree baseline",
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

#[derive(Clone, Debug, Serialize)]
pub struct ClusterCase {
    pub text: String,
    pub token_count: usize,
    pub cluster_id: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClusterEdge {
    pub left: String,
    pub right: String,
    pub shared_tokens: Vec<String>,
    pub weight: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct ClusterPreviewReport {
    pub experiment: &'static str,
    pub threshold: f64,
    pub cluster_count: usize,
    pub cases: Vec<ClusterCase>,
    pub edges: Vec<ClusterEdge>,
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

pub fn run_cluster_preview() -> ClusterPreviewReport {
    let samples: &[&str] = &[
        "dog chases cat in garden",
        "cat sleeps in sunny garden",
        "bird flies over mountain ridge",
        "eagle glides above mountain sky",
        "bank stock market rally",
        "investor studies stock market trends",
        "goal team wins stadium match",
    ];
    let threshold = 0.20;

    let tokenized = samples
        .iter()
        .map(|text| tokenize_cluster_text(text))
        .collect::<Vec<_>>();

    let mut neighbors = vec![Vec::<usize>::new(); samples.len()];
    let mut edges = Vec::<ClusterEdge>::new();

    for left in 0..samples.len() {
        for right in (left + 1)..samples.len() {
            let (weight, mut shared) =
                jaccard_with_shared_tokens(&tokenized[left], &tokenized[right]);
            if weight >= threshold {
                neighbors[left].push(right);
                neighbors[right].push(left);
                shared.sort();
                edges.push(ClusterEdge {
                    left: samples[left].to_string(),
                    right: samples[right].to_string(),
                    shared_tokens: shared,
                    weight,
                });
            }
        }
    }

    let mut cluster_ids = vec![usize::MAX; samples.len()];
    let mut next_cluster_id = 0usize;
    for start in 0..samples.len() {
        if cluster_ids[start] != usize::MAX {
            continue;
        }
        let mut stack = vec![start];
        cluster_ids[start] = next_cluster_id;
        while let Some(node) = stack.pop() {
            for neighbor in &neighbors[node] {
                if cluster_ids[*neighbor] == usize::MAX {
                    cluster_ids[*neighbor] = next_cluster_id;
                    stack.push(*neighbor);
                }
            }
        }
        next_cluster_id += 1;
    }

    let cases = samples
        .iter()
        .enumerate()
        .map(|(index, text)| ClusterCase {
            text: (*text).to_string(),
            token_count: tokenized[index].len(),
            cluster_id: cluster_ids[index],
        })
        .collect::<Vec<_>>();

    ClusterPreviewReport {
        experiment: "cluster-preview",
        threshold,
        cluster_count: next_cluster_id,
        cases,
        edges,
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

#[derive(Clone, Debug, Serialize)]
pub struct DecisionTreeExperimentCase {
    pub text: String,
    pub expected: String,
    pub projected_vector: Vec<f64>,
    pub naive_bayes_predicted: String,
    pub nlp_decision_tree_classify: String,
    pub nlp_decision_tree_classify_many: String,
    pub nlp_decision_tree_prob_top: LabelScore,
    pub ml_decision_tree_predicted: String,
    pub ml_decision_tree_probabilities: Vec<f64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MlDecisionTreeTrainCase {
    pub text: String,
    pub expected: String,
    pub projected_vector: Vec<f64>,
    pub ml_decision_tree_predicted: String,
    pub ml_decision_tree_probabilities: Vec<f64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MlDecisionTreeDiagnostics {
    pub projected_feature_names: Vec<String>,
    pub train_cases: Vec<MlDecisionTreeTrainCase>,
    pub train_accuracy: f64,
    pub predicted_label_counts: Vec<LabelCount>,
    pub predicts_single_label_on_train: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct MlGmlDecisionTreeSweepRun {
    pub id: String,
    pub max_features_ratio: f64,
    pub num_samples_ratio: f64,
    pub max_depth: usize,
    pub min_samples_split: usize,
    pub min_samples_leaf: usize,
    pub seed: u64,
    pub train_accuracy: f64,
    pub test_accuracy: f64,
    pub predicted_label_counts: Vec<LabelCount>,
    pub predicts_single_label_on_train: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct LabelCount {
    pub label: String,
    pub count: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct FixtureCompatibilityReport {
    pub nlp_fixture_shape: &'static str,
    pub ml_fixture_shape: &'static str,
    pub can_share_raw_fixture: bool,
    pub projected_fixture_shape: &'static str,
    pub can_share_via_projection: bool,
    pub notes: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DecisionTreeExperimentReport {
    pub experiment: &'static str,
    pub labels: Vec<&'static str>,
    pub train_size: usize,
    pub test_size: usize,
    pub token_vocabulary: Vec<&'static str>,
    pub naive_bayes_accuracy: f64,
    pub nlp_decision_tree_accuracy: f64,
    pub ml_decision_tree_accuracy: f64,
    pub fixture_compatibility: FixtureCompatibilityReport,
    pub ml_decision_tree_diagnostics: MlDecisionTreeDiagnostics,
    pub ml_gml_decision_tree_sweep: Vec<MlGmlDecisionTreeSweepRun>,
    pub cases: Vec<DecisionTreeExperimentCase>,
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

pub fn run_decision_tree_experiment() -> DecisionTreeExperimentReport {
    let train_specs: &[(&str, &str)] = &[
        ("goal team win stadium", "sports"),
        ("coach team match season", "sports"),
        ("stock market bond trade", "finance"),
        ("bank credit market loan", "finance"),
        ("player score team", "sports"),
        ("fund profit investor", "finance"),
    ];
    let test_specs: &[(&str, &str)] = &[
        ("team goal player", "sports"),
        ("market stock investor", "finance"),
        ("coach trade stadium", "sports"),
    ];

    let train_nlp: Vec<(HashMap<String, FeatureValue>, &'static str)> = train_specs
        .iter()
        .map(|(text, label)| (topic_features(text), *label))
        .collect();
    let test_nlp: Vec<(HashMap<String, FeatureValue>, &'static str)> = test_specs
        .iter()
        .map(|(text, label)| (topic_features(text), *label))
        .collect();
    let test_nlp_features: Vec<HashMap<String, FeatureValue>> = test_nlp
        .iter()
        .map(|(features, _)| features.clone())
        .collect();

    let naive_bayes = NaiveBayesClassifier::train(&train_nlp, 1.0);
    let nlp_tree = DecisionTreeClassifier::train(&train_nlp, 0.0, 3, 0);
    let nb_predictions = naive_bayes.classify_many(&test_nlp_features);
    let nlp_tree_predictions_many = nlp_tree.classify_many(&test_nlp_features);
    let nlp_tree_probs = nlp_tree.prob_classify_many(&test_nlp_features);

    let train_dense = DenseFeatures::new(
        train_specs
            .iter()
            .map(|(text, _)| topic_vector(text))
            .collect::<Vec<_>>(),
    );
    let train_labels = HugeIntArray::from_vec(
        train_specs
            .iter()
            .map(|(_, label)| topic_label_to_class(label))
            .collect::<Vec<_>>(),
    );
    let train_set = Arc::new((0..train_dense.size() as u64).collect::<Vec<u64>>());

    let tree_like_trainer = RandomForestClassifierTrainer::new(
        Concurrency::single_threaded(),
        2,
        RandomForestClassifierTrainerConfig {
            forest: RandomForestConfig {
                max_features_ratio: Some(1.0),
                num_samples_ratio: 0.0,
                num_decision_trees: 1,
                max_depth: 8,
                min_samples_split: 2,
                min_samples_leaf: 1,
            },
            criterion: ClassifierImpurityCriterionType::Gini,
        },
        Some(4242),
        TaskProgressTracker::new(Tasks::leaf("nlp-decision-tree-experiment".to_string())),
        TerminationFlag::default(),
        Arc::new(ModelSpecificMetricsHandler::noop()),
    );
    let ml_tree_like = tree_like_trainer.train(&train_dense, &train_labels, &train_set);

    let test_dense = test_specs
        .iter()
        .map(|(text, _)| topic_vector(text))
        .collect::<Vec<_>>();

    let train_predictions = train_specs
        .iter()
        .enumerate()
        .map(|(index, (_, expected))| {
            let probabilities = ml_tree_like.predict_probabilities(train_dense.get(index));
            let predicted = class_to_topic_label(argmax_index(&probabilities));
            (
                (*expected).to_string(),
                predicted.to_string(),
                probabilities,
            )
        })
        .collect::<Vec<_>>();

    let ml_train_correct = train_predictions
        .iter()
        .filter(|(expected, predicted, _)| expected == predicted)
        .count();

    let predicted_label_counts = label_counts(
        &train_predictions
            .iter()
            .map(|(_, predicted, _)| predicted.clone())
            .collect::<Vec<_>>(),
    );

    let train_cases = train_specs
        .iter()
        .enumerate()
        .map(|(index, (text, expected))| MlDecisionTreeTrainCase {
            text: (*text).to_string(),
            expected: (*expected).to_string(),
            projected_vector: train_dense.get(index).to_vec(),
            ml_decision_tree_predicted: train_predictions[index].1.clone(),
            ml_decision_tree_probabilities: train_predictions[index].2.clone(),
        })
        .collect::<Vec<_>>();

    let ml_tree_probs = test_dense
        .iter()
        .map(|row| ml_tree_like.predict_probabilities(row))
        .collect::<Vec<_>>();
    let ml_tree_predictions = ml_tree_probs
        .iter()
        .map(|probs| class_to_topic_label(argmax_index(probs)).to_string())
        .collect::<Vec<_>>();

    let ml_correct = ml_tree_predictions
        .iter()
        .zip(test_specs.iter())
        .filter(|(predicted, (_, expected))| predicted.as_str() == *expected)
        .count();

    let sweep_specs: &[(&str, f64, f64, usize, usize, usize, u64)] = &[
        ("baseline", 1.0, 0.0, 8, 2, 1, 4242),
        ("bootstrap", 1.0, 0.8, 8, 2, 1, 4242),
        ("feature-bagged", 0.5, 0.8, 8, 2, 1, 4242),
        ("shallow", 1.0, 0.8, 3, 2, 1, 4242),
        ("alt-seed", 1.0, 0.8, 8, 2, 1, 7),
    ];
    let ml_gml_decision_tree_sweep = sweep_specs
        .iter()
        .map(
            |(
                id,
                max_features_ratio,
                num_samples_ratio,
                max_depth,
                min_samples_split,
                min_samples_leaf,
                seed,
            )| {
                let trainer = RandomForestClassifierTrainer::new(
                    Concurrency::single_threaded(),
                    2,
                    RandomForestClassifierTrainerConfig {
                        forest: RandomForestConfig {
                            max_features_ratio: Some(*max_features_ratio),
                            num_samples_ratio: *num_samples_ratio,
                            num_decision_trees: 1,
                            max_depth: *max_depth,
                            min_samples_split: *min_samples_split,
                            min_samples_leaf: *min_samples_leaf,
                        },
                        criterion: ClassifierImpurityCriterionType::Gini,
                    },
                    Some(*seed),
                    TaskProgressTracker::new(Tasks::leaf(format!(
                        "nlp-decision-tree-experiment-sweep-{id}"
                    ))),
                    TerminationFlag::default(),
                    Arc::new(ModelSpecificMetricsHandler::noop()),
                );
                let model = trainer.train(&train_dense, &train_labels, &train_set);

                let train_predictions = (0..train_dense.size())
                    .map(|index| {
                        class_to_topic_label(argmax_index(
                            &model.predict_probabilities(train_dense.get(index)),
                        ))
                        .to_string()
                    })
                    .collect::<Vec<_>>();
                let test_predictions = test_dense
                    .iter()
                    .map(|row| {
                        class_to_topic_label(argmax_index(&model.predict_probabilities(row)))
                    })
                    .collect::<Vec<_>>();

                let train_correct = train_predictions
                    .iter()
                    .zip(train_specs.iter())
                    .filter(|(predicted, (_, expected))| predicted.as_str() == *expected)
                    .count();
                let test_correct = test_predictions
                    .iter()
                    .zip(test_specs.iter())
                    .filter(|(predicted, (_, expected))| **predicted == *expected)
                    .count();

                let counts = label_counts(&train_predictions);
                MlGmlDecisionTreeSweepRun {
                    id: (*id).to_string(),
                    max_features_ratio: *max_features_ratio,
                    num_samples_ratio: *num_samples_ratio,
                    max_depth: *max_depth,
                    min_samples_split: *min_samples_split,
                    min_samples_leaf: *min_samples_leaf,
                    seed: *seed,
                    train_accuracy: train_correct as f64 / train_specs.len() as f64,
                    test_accuracy: test_correct as f64 / test_specs.len() as f64,
                    predicts_single_label_on_train: counts.len() == 1,
                    predicted_label_counts: counts,
                }
            },
        )
        .collect::<Vec<_>>();

    let cases = test_specs
        .iter()
        .enumerate()
        .map(|(index, (text, expected))| {
            let single_prediction = nlp_tree.classify(&test_nlp_features[index]);
            let top_label = nlp_tree_probs[index]
                .max()
                .unwrap_or_else(|| nlp_tree_predictions_many[index]);
            DecisionTreeExperimentCase {
                text: (*text).to_string(),
                expected: (*expected).to_string(),
                projected_vector: test_dense[index].clone(),
                naive_bayes_predicted: nb_predictions[index].to_string(),
                nlp_decision_tree_classify: single_prediction.to_string(),
                nlp_decision_tree_classify_many: nlp_tree_predictions_many[index].to_string(),
                nlp_decision_tree_prob_top: LabelScore {
                    label: top_label.to_string(),
                    prob: *nlp_tree_probs[index]
                        .as_map()
                        .get(top_label)
                        .unwrap_or(&0.0),
                },
                ml_decision_tree_predicted: ml_tree_predictions[index].clone(),
                ml_decision_tree_probabilities: ml_tree_probs[index].clone(),
            }
        })
        .collect::<Vec<_>>();

    DecisionTreeExperimentReport {
        experiment: "decision-tree-experiment",
        labels: vec!["sports", "finance"],
        train_size: train_specs.len(),
        test_size: test_specs.len(),
        token_vocabulary: TOPIC_TOKENS.to_vec(),
        naive_bayes_accuracy: accuracy(&naive_bayes, &test_nlp),
        nlp_decision_tree_accuracy: accuracy(&nlp_tree, &test_nlp),
        ml_decision_tree_accuracy: ml_correct as f64 / test_specs.len() as f64,
        fixture_compatibility: FixtureCompatibilityReport {
            nlp_fixture_shape: "Vec<(HashMap<String, FeatureValue>, &str)>",
            ml_fixture_shape: "DenseFeatures(Vec<Vec<f64>>) + HugeIntArray(Vec<i32>)",
            can_share_raw_fixture: false,
            projected_fixture_shape: "Vec<Vec<f64>> projection from NLP FeatureValue map",
            can_share_via_projection: true,
            notes: vec![
                "NLP classifiers consume typed sparse feature maps keyed by strings.".to_string(),
                "ML/GML (GDS ML) decision_tree path consumes dense numeric vectors plus integer class ids."
                    .to_string(),
                "A deterministic feature projection lets both stacks evaluate equivalent cases."
                    .to_string(),
            ],
        },
        ml_decision_tree_diagnostics: MlDecisionTreeDiagnostics {
            projected_feature_names: projected_feature_names(),
            train_cases,
            train_accuracy: ml_train_correct as f64 / train_specs.len() as f64,
            predicts_single_label_on_train: predicted_label_counts.len() == 1,
            predicted_label_counts,
        },
        ml_gml_decision_tree_sweep,
        cases,
    }
}

const TOPIC_TOKENS: [&str; 16] = [
    "goal", "team", "player", "coach", "stadium", "score", "market", "stock", "bank", "bond",
    "fund", "investor", "trade", "profit", "credit", "loan",
];

fn topic_features(text: &str) -> HashMap<String, FeatureValue> {
    let mut map = HashMap::new();
    let lower = text.to_lowercase();

    map.insert("alwayson".to_string(), FeatureValue::Bool(true));
    map.insert(
        "len_tokens".to_string(),
        FeatureValue::Int(lower.split_whitespace().count() as i64),
    );

    for token in TOPIC_TOKENS {
        map.insert(
            format!("contains({token})"),
            FeatureValue::Bool(lower.contains(token)),
        );
    }

    map
}

fn topic_vector(text: &str) -> Vec<f64> {
    let lower = text.to_lowercase();
    let mut vector = Vec::with_capacity(TOPIC_TOKENS.len() + 1);
    vector.push(lower.split_whitespace().count() as f64);
    for token in TOPIC_TOKENS {
        vector.push(if lower.contains(token) { 1.0 } else { 0.0 });
    }
    vector
}

fn topic_label_to_class(label: &str) -> i32 {
    match label {
        "sports" => 0,
        "finance" => 1,
        _ => 0,
    }
}

fn tokenize_cluster_text(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split_whitespace()
        .map(|token| token.to_string())
        .collect()
}

fn jaccard_with_shared_tokens(left: &[String], right: &[String]) -> (f64, Vec<String>) {
    let left_set = left
        .iter()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let right_set = right
        .iter()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();

    let shared = left_set
        .intersection(&right_set)
        .cloned()
        .collect::<Vec<_>>();
    let union_size = left_set.union(&right_set).count();

    if union_size == 0 {
        return (0.0, shared);
    }
    (shared.len() as f64 / union_size as f64, shared)
}

fn class_to_topic_label(class: usize) -> &'static str {
    match class {
        0 => "sports",
        1 => "finance",
        _ => "sports",
    }
}

fn argmax_index(values: &[f64]) -> usize {
    values
        .iter()
        .enumerate()
        .fold((0usize, f64::NEG_INFINITY), |best, (index, value)| {
            if *value > best.1 {
                (index, *value)
            } else {
                best
            }
        })
        .0
}

fn projected_feature_names() -> Vec<String> {
    let mut names = vec!["len_tokens".to_string()];
    names.extend(
        TOPIC_TOKENS
            .iter()
            .map(|token| format!("contains({token})")),
    );
    names
}

fn label_counts(predictions: &[String]) -> Vec<LabelCount> {
    let mut counts = HashMap::<String, usize>::new();
    for predicted in predictions {
        *counts.entry(predicted.clone()).or_insert(0) += 1;
    }
    let mut counts = counts
        .into_iter()
        .map(|(label, count)| LabelCount { label, count })
        .collect::<Vec<_>>();
    counts.sort_by(|left, right| right.count.cmp(&left.count));
    counts
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
        assert!(experiments.iter().any(|item| item.id == "cluster-preview"));
        assert!(experiments
            .iter()
            .any(|item| item.id == "decision-tree-experiment"));
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
    fn cluster_preview_runs_and_reports_clusters() {
        let report = run_cluster_preview();

        assert_eq!(report.experiment, "cluster-preview");
        assert_eq!(report.cases.len(), 7);
        assert!(report.cluster_count >= 3);
        assert!(report
            .edges
            .iter()
            .all(|edge| edge.weight >= report.threshold));
        assert!(report
            .edges
            .iter()
            .any(|edge| edge.shared_tokens.iter().any(|token| token == "market")));
    }

    #[test]
    fn decision_tree_experiment_runs_and_reports_fixture_compatibility() {
        let report = run_decision_tree_experiment();

        assert_eq!(report.experiment, "decision-tree-experiment");
        assert_eq!(report.train_size, 6);
        assert_eq!(report.test_size, 3);
        assert_eq!(report.cases.len(), 3);
        assert!(!report.fixture_compatibility.can_share_raw_fixture);
        assert!(report.fixture_compatibility.can_share_via_projection);
        assert_eq!(report.ml_decision_tree_diagnostics.train_cases.len(), 6);
        assert_eq!(
            report
                .ml_decision_tree_diagnostics
                .projected_feature_names
                .len(),
            17
        );
        assert!(
            !report
                .ml_decision_tree_diagnostics
                .predicts_single_label_on_train,
            "ML/GML baseline should not collapse to a single class on train"
        );
        assert!(report.cases.iter().all(|case| {
            case.nlp_decision_tree_classify == case.nlp_decision_tree_classify_many
        }));
        assert_eq!(report.ml_gml_decision_tree_sweep.len(), 5);
        assert!(report.ml_gml_decision_tree_sweep.iter().all(|run| {
            run.predicted_label_counts
                .iter()
                .map(|item| item.count)
                .sum::<usize>()
                == 6
        }));
        assert!(report
            .ml_gml_decision_tree_sweep
            .iter()
            .all(|run| !run.predicts_single_label_on_train));
        assert!(report
            .cases
            .iter()
            .all(|case| case.projected_vector.len() == 17));
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
