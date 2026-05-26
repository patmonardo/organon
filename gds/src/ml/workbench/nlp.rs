use crate::ml::nlp::sem::evaluate::Assignment;
use crate::ml::nlp::sem::evaluate::EvalValue;
use crate::ml::nlp::sem::evaluate::Model;
use crate::ml::nlp::sem::evaluate::Valuation;
use crate::ml::nlp::sem::logic;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct NlpWorkbenchExperiment {
    pub id: &'static str,
    pub name: &'static str,
    pub focus: &'static str,
}

#[derive(Clone, Debug, Serialize)]
pub struct SemanticBoundaryReport {
    pub experiment: &'static str,
    pub semantic_oriented_modules: Vec<&'static str>,
    pub numeric_oriented_modules: Vec<&'static str>,
}

#[derive(Clone, Debug, Serialize)]
pub struct LogicPreviewReport {
    pub experiment: &'static str,
    pub expression: String,
    pub parsed: String,
    pub simplified: String,
    pub free_variables: Vec<String>,
    pub constants: Vec<String>,
    pub variables: Vec<String>,
    pub evaluation_value: String,
}

pub fn available_experiments() -> &'static [NlpWorkbenchExperiment] {
    &[
        NlpWorkbenchExperiment {
            id: "semantic-boundary-preview",
            name: "Semantic vs Numeric Module Boundary",
            focus: "module-inventory",
        },
        NlpWorkbenchExperiment {
            id: "logic-preview",
            name: "First-Order Logic Parse and Evaluate Preview",
            focus: "sem::logic + sem::evaluate",
        },
    ]
}

pub fn run_semantic_boundary_preview() -> SemanticBoundaryReport {
    SemanticBoundaryReport {
        experiment: "semantic-boundary-preview",
        semantic_oriented_modules: vec![
            "ml::nlp::sem",
            "ml::nlp::parse",
            "ml::nlp::tree",
            "ml::nlp::inference",
        ],
        numeric_oriented_modules: vec![
            "ml::models",
            "ml::gradient_descent",
            "ml::metrics",
            "ml::sampling",
        ],
    }
}

pub fn run_logic_preview() -> LogicPreviewReport {
    let expression = "all x. (dog(x) -> animal(x)) & exists x. dog(x)";
    let parsed = logic::parse(expression).expect("logic preview expression should parse");

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
    let assignment = Assignment::new(domain);
    let value = model
        .satisfy(&parsed, &assignment)
        .expect("logic preview evaluation should succeed");

    LogicPreviewReport {
        experiment: "logic-preview",
        expression: expression.to_string(),
        parsed: parsed.to_string(),
        simplified: parsed.simplify().to_string(),
        free_variables: parsed
            .free()
            .into_iter()
            .map(|variable| variable.name().to_string())
            .collect::<Vec<String>>(),
        constants: parsed.constants().into_iter().collect::<Vec<String>>(),
        variables: parsed
            .variables()
            .into_iter()
            .map(|variable| variable.name().to_string())
            .collect::<Vec<String>>(),
        evaluation_value: eval_value_to_string(&value),
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
    fn nlp_workbench_catalog_has_bootstrap_experiments() {
        let experiments = available_experiments();

        assert!(experiments.iter().any(|item| item.id == "logic-preview"));
        assert!(experiments
            .iter()
            .any(|item| item.id == "semantic-boundary-preview"));
    }

    #[test]
    fn logic_preview_runs_and_reports_semantic_shape() {
        let report = run_logic_preview();

        assert_eq!(report.experiment, "logic-preview");
        assert!(report.parsed.contains("all x"));
        assert!(report.constants.contains(&"animal".to_string()));
        assert!(report.evaluation_value == "true" || report.evaluation_value == "false");
    }

    #[test]
    fn semantic_boundary_preview_separates_domains() {
        let report = run_semantic_boundary_preview();

        assert_eq!(report.experiment, "semantic-boundary-preview");
        assert!(report.semantic_oriented_modules.contains(&"ml::nlp::sem"));
        assert!(report.numeric_oriented_modules.contains(&"ml::models"));
    }
}
