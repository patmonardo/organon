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
    pub simplified: String,
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
                simplified: parsed.simplify().to_string(),
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
}
