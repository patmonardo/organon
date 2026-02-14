//! Form evaluation entrypoint.
//!
//! Triadic module engineering for Form evaluation:
//! - `concept`: form meaning, contracts, and pre-eval traces.
//! - `mediation`: runtime application and backend execution bindings.
//! - `judgement`: Eval(Form) -> Apply(Form) -> Print orchestration.

pub mod concept;
pub mod judgement;
pub mod mediation;

pub use concept::*;
pub use judgement::*;
pub use mediation::*;

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use std::sync::Arc;

    use serde_json::json;

    use crate::form::{
        ApplicationForm, Context, FormShape, Morph, ProgramSpec, Shape, Specification,
    };
    use crate::types::catalog::InMemoryGraphCatalog;

    fn sample_program() -> ProgramSpec {
        ProgramSpec {
            form: FormShape::new(
                Shape::default(),
                Context::new(vec![], vec![], "kernel".to_string(), vec![]),
                Morph::new(vec!["base.normalize".to_string()]),
            ),
            gdsl: Specification {
                name: "gdsl.core".to_string(),
                version: Some("0.1.0".to_string()),
                attributes: HashMap::new(),
            },
            sdsl: vec![],
            application_forms: vec![ApplicationForm {
                name: "centrality".to_string(),
                domain: "graph-ml".to_string(),
                features: vec!["feature.centrality".to_string()],
                patterns: vec!["algo.pagerank".to_string()],
                specifications: HashMap::new(),
            }],
            selected_forms: vec!["centrality".to_string()],
        }
    }

    #[test]
    fn evaluates_program_into_plan_and_contract() {
        let evaluator = FormEvaluator::new();
        let result = evaluator
            .evaluate(FormEvalRequest::new(sample_program()))
            .expect("evaluation should succeed");

        assert_eq!(
            result.plan.patterns,
            vec!["base.normalize", "algo.pagerank"]
        );
        assert_eq!(result.contract.selected_forms, vec!["centrality"]);
        assert_eq!(result.pre_eval.given_forms.len(), 1);
        assert_eq!(
            result.pre_eval.given_forms[0].effect_entity,
            "entity::centrality"
        );
        assert_eq!(
            result.pre_eval.given_forms[0].monadic_evaluation,
            MonadicEvaluationState::Pending
        );
    }

    #[test]
    fn round_trips_contract_to_program_spec() {
        let program = sample_program();
        let contract = FormContract::from_program_spec(&program);
        let rebuilt = contract.into_program_spec();

        assert_eq!(program, rebuilt);
    }

    #[test]
    fn program_form_api_runs_eval_apply_print() {
        let api = ProgramFormApi::new();
        let mut request = ProgramFormRequest::new(sample_program());
        request.fail_fast = false;

        let catalog = Arc::new(InMemoryGraphCatalog::new());

        let print = api
            .evaluate_apply_print(request, catalog)
            .expect("eval/apply/print should complete");

        assert_eq!(print.eval.patterns, vec!["base.normalize", "algo.pagerank"]);
        assert_eq!(print.backend, ProgramFormApplyBackend::ExecuteSpec);
        assert_eq!(print.apply.skipped, vec!["base.normalize"]);
        assert_eq!(print.apply.executed.len(), 1);
        assert_eq!(print.apply.executed[0].op, "pagerank");
        assert_eq!(print.apply.executed[0].spec_binding, "spec.pagerank");
        assert_eq!(print.apply.failed.len(), 1);
        assert_eq!(print.apply.failed[0].spec_binding, "spec.pagerank");
        assert_eq!(print.pre_eval.given_forms.len(), 1);
        assert_eq!(
            print.pre_eval.given_forms[0].monadic_evaluation,
            MonadicEvaluationState::Failed
        );
        assert_eq!(print.evaluation_slot.state, MonadicEvaluationState::Failed);
        assert_eq!(print.organic_unity.status, OrganicUnityStatus::Coherent);
        assert!(print.organic_unity.reasons.is_empty());
        assert!(!print.ok);
    }

    #[test]
    fn program_form_api_supports_direct_compute_backend() {
        let api = ProgramFormApi::new();
        let mut request = ProgramFormRequest::new(sample_program());
        request.fail_fast = false;
        request.apply_backend = ProgramFormApplyBackend::DirectCompute;

        let catalog = Arc::new(InMemoryGraphCatalog::new());

        let print = api
            .evaluate_apply_print(request, catalog)
            .expect("eval/apply/print should complete");

        assert_eq!(print.backend, ProgramFormApplyBackend::DirectCompute);
        assert_eq!(print.apply.executed.len(), 1);
        assert_eq!(print.apply.executed[0].spec_binding, "direct_compute");
        assert_eq!(print.organic_unity.status, OrganicUnityStatus::Coherent);
    }

    #[test]
    fn organic_unity_check_reports_coherent_when_all_constraints_hold() {
        let eval = ProgramFormEvalPrint {
            selected_forms: vec!["centrality".to_string()],
            patterns: vec!["algo.pagerank".to_string()],
        };

        let apply = ProgramFormApplyPrint {
            executed: vec![ProgramFormExecution {
                pattern: "algo.pagerank".to_string(),
                op: "pagerank".to_string(),
                spec_binding: "spec.pagerank".to_string(),
                response: json!({"ok": true}),
            }],
            failed: vec![],
            skipped: vec![],
        };

        let pre_eval = FormPreEvalTrace {
            given_forms: vec![GivenFormSevenFoldTrace {
                application_form: "centrality".to_string(),
                principle_shape: "principle.shape".to_string(),
                principle_context: "principle.context".to_string(),
                principle_morph: "principle.morph".to_string(),
                effect_entity: "entity::centrality".to_string(),
                effect_property: "property::centrality".to_string(),
                effect_aspect: "aspect::dataset://appearance/main".to_string(),
                monadic_evaluation: MonadicEvaluationState::Succeeded,
            }],
        };

        let report = judgement::OrganicUnityCheck::evaluate(
            ProgramFormApplyBackend::ExecuteSpec,
            &eval,
            &apply,
            &pre_eval,
            &MonadicEvaluationSlot {
                state: MonadicEvaluationState::Succeeded,
            },
            true,
        );

        assert_eq!(report.status, OrganicUnityStatus::Coherent);
        assert!(report.reasons.is_empty());
    }

    #[test]
    fn resolves_dedicated_pattern_bindings() {
        let pagerank = mediation::resolve_spec_binding("algorithm.pagerank", "pagerank");
        assert_eq!(pagerank, "spec.pagerank");

        let leiden = mediation::resolve_spec_binding("algo.leiden", "leiden");
        assert_eq!(leiden, "spec.leiden");

        let generic = mediation::resolve_spec_binding("algo.node2vec", "node2vec");
        assert_eq!(generic, "spec.generic.node2vec");
    }

    #[test]
    fn normalize_algorithm_op_handles_prefixes() {
        assert_eq!(
            mediation::normalize_algorithm_op("algo.pagerank"),
            Some("pagerank".to_string())
        );
        assert_eq!(
            mediation::normalize_algorithm_op("algorithm.leiden"),
            Some("leiden".to_string())
        );
        assert_eq!(
            mediation::normalize_algorithm_op("applications.algorithms.knn"),
            Some("knn".to_string())
        );
        assert_eq!(mediation::normalize_algorithm_op("base.normalize"), None);
    }

    #[test]
    fn request_constructors_encode_world_boundary() {
        let agent = ProgramFormRequest::for_agent_framework(sample_program(), "agent-kernel");
        assert_eq!(agent.username, "agent-kernel");
        assert_eq!(agent.apply_backend, ProgramFormApplyBackend::ExecuteSpec);

        let public = ProgramFormRequest::for_public_facade(sample_program(), "public-api");
        assert_eq!(public.username, "public-api");
        assert_eq!(public.apply_backend, ProgramFormApplyBackend::DirectCompute);
    }

    #[test]
    fn backend_marks_spec_driven_world() {
        assert!(ProgramFormApplyBackend::ExecuteSpec.is_spec_driven());
        assert!(!ProgramFormApplyBackend::DirectCompute.is_spec_driven());
    }

    #[test]
    fn session_executes_program_in_projection_world() {
        let api = ProgramFormApi::new();
        let projection = RootProjectionContext::new("proj-1", "world_graph", "root-agent")
            .with_pipeline_path(vec![
                "codegen".to_string(),
                "factory".to_string(),
                "eval".to_string(),
            ]);

        let mut session = api.start_agent_session(projection, "single-agent-a");
        session.fail_fast = false;

        let catalog = Arc::new(InMemoryGraphCatalog::new());
        let print = api
            .execute_session(&session, sample_program(), catalog)
            .expect("session execution should complete");

        assert_eq!(print.backend, ProgramFormApplyBackend::ExecuteSpec);
        assert_eq!(print.apply.executed.len(), 1);
        assert_eq!(print.apply.executed[0].spec_binding, "spec.pagerank");
    }

    #[test]
    fn public_session_uses_direct_compute_backend() {
        let api = ProgramFormApi::new();
        let projection = RootProjectionContext::new("proj-2", "world_graph", "root-agent");

        let mut session = api.start_public_session(projection, "external-agent");
        session.fail_fast = false;

        let catalog = Arc::new(InMemoryGraphCatalog::new());
        let print = api
            .execute_session(&session, sample_program(), catalog)
            .expect("public session execution should complete");

        assert_eq!(print.backend, ProgramFormApplyBackend::DirectCompute);
        assert_eq!(print.apply.executed.len(), 1);
        assert_eq!(print.apply.executed[0].spec_binding, "direct_compute");
    }
}
