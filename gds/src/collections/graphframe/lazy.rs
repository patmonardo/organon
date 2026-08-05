//! GraphFrame immutable planning surface.
//!
//! This module encodes Form-as-Plan for GraphFrame by carrying a validated
//! GraphStore handle plus graph-view selection state. The resulting
//! GraphExecutionIntent is the handoff boundary between the graph-form layer and
//! the shell-execution layer: GraphFrame provides the form and view semantics,
//! while TaskFrame turns that intent into a workflow executable by the shell.

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use crate::collections::graphframe::agent_plan::GraphAgentPlanInterface;
use crate::collections::graphframe::expr::GraphFrameExpr;
use crate::collections::graphframe::expr::GraphProcedureExpr;
use crate::collections::graphframe::expr::GraphViewExpr;
use crate::collections::graphframe::feature_grammar::GraphFeatureGrammarChecked;
use crate::collections::graphframe::frame::GraphFrame;
use crate::collections::graphframe::frame::GraphFrameError;
use crate::collections::graphframe::frame::SharedGraphStore;
use crate::collections::graphframe::graph_form::build_gdsl_transmission_spec;
use crate::collections::graphframe::graph_form::compile_graph_form;
use crate::collections::graphframe::graph_form::derive_empirical_outflows;
use crate::collections::graphframe::graph_form::synthetic_moments;
use crate::collections::graphframe::graph_form::validate_automation_shell_plan;
use crate::collections::graphframe::graph_form::GraphAutomationProfile;
use crate::collections::graphframe::graph_form::GraphEmpiricalOutflowAspect;
use crate::collections::graphframe::graph_form::GraphExecutionSurface;
use crate::collections::graphframe::graph_form::GraphExecutionSurfaceMode;
use crate::collections::graphframe::graph_form::GraphFormCompilation;
use crate::collections::graphframe::graph_form::GraphFormError;
use crate::collections::graphframe::graph_form::GraphGdslTransmissionSpec;
use crate::collections::graphframe::graph_form::GraphSyntheticMoment;
use crate::collections::graphframe::rational_language::lower_graph_semantic_program;
use crate::collections::graphframe::rational_language::lower_graph_semantics;
use crate::collections::graphframe::rational_language::GraphRationalLanguageError;
use crate::collections::graphframe::rational_language::GraphSemanticLowering;
use crate::collections::graphframe::rational_language::GraphSemanticProgram;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::shell::ShellAddress;
use crate::shell::ShellAlgebra;
use crate::shell::ShellComponentExecutionKind;
use crate::shell::ShellComponentMode;
use crate::shell::ShellComponentPlan;
use crate::shell::ShellPipeline;
use crate::shell::ShellPipelineDescriptor;
use crate::shell::ShellPipelineFacade;
use crate::shell::ShellPureFormReturn;
use crate::shell::ShellRegister;
use crate::task::frame::TaskFrame;
use crate::task::frame::TaskFramePolicy;
use crate::task::frame::TaskObjectiveRef;
use crate::task::frame::TaskReturnContract;
use crate::task::job::TaskJob;
use crate::task::spec::TaskSpec;
use crate::task::spec::TaskSpecError;
use crate::task::spec::TaskWorkflow;
use crate::types::graph_store::GraphViewSpec;

#[derive(Debug, Clone)]
pub struct GraphFramePureFormReciprocity {
    view_spec: GraphViewSpec,
    shell_plan: ShellComponentPlan,
    shell_pipeline: ShellPipelineFacade,
}

/// Objective graph execution intent emitted before Agent workflow constitution.
#[derive(Clone)]
pub struct GraphExecutionIntent {
    store: SharedGraphStore,
    reciprocity: GraphFramePureFormReciprocity,
    objective: TaskObjectiveRef,
    return_contract: TaskReturnContract,
    compute_steps: Vec<String>,
    estimated_volume: usize,
}

/// Agent-facing processing contract: persisted GraphForm model plus TaskFrame.
pub struct GraphAgentProcessingContract {
    graph_form: GraphFormCompilation,
    task_frame: TaskFrame<GraphExecutionIntent>,
    synthetic_moments: Vec<GraphSyntheticMoment>,
    empirical_outflows: Vec<GraphEmpiricalOutflowAspect>,
    transmission_spec: GraphGdslTransmissionSpec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphTaskDaemonRoute {
    None,
    Algorithm,
    Pipeline,
    StoreApi,
    Hybrid,
}

impl GraphTaskDaemonRoute {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Algorithm => "algorithm",
            Self::Pipeline => "pipeline",
            Self::StoreApi => "store_api",
            Self::Hybrid => "hybrid",
        }
    }

    fn from_transmission(spec: &GraphGdslTransmissionSpec) -> Self {
        match spec.execution_surface_mode_kind() {
            GraphExecutionSurfaceMode::Hybrid => Self::Hybrid,
            GraphExecutionSurfaceMode::Single => match spec.primary_execution_surface_kind() {
                Some(GraphExecutionSurface::Algorithm) => Self::Algorithm,
                Some(GraphExecutionSurface::Pipeline) => Self::Pipeline,
                Some(GraphExecutionSurface::StoreApi) => Self::StoreApi,
                None => Self::None,
            },
            GraphExecutionSurfaceMode::None => Self::None,
        }
    }
}

#[derive(Clone)]
pub struct GraphTaskDaemonSubmission {
    owner: String,
    route: GraphTaskDaemonRoute,
    task_frame: TaskFrame<GraphExecutionIntent>,
    transmission_spec: GraphGdslTransmissionSpec,
}

impl GraphTaskDaemonSubmission {
    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn route(&self) -> GraphTaskDaemonRoute {
        self.route
    }

    pub fn task_frame(&self) -> &TaskFrame<GraphExecutionIntent> {
        &self.task_frame
    }

    pub fn transmission_spec(&self) -> &GraphGdslTransmissionSpec {
        &self.transmission_spec
    }

    pub fn into_job(self) -> Result<TaskJob<GraphExecutionIntent>, TaskSpecError> {
        self.task_frame.into_job(self.owner)
    }
}

impl GraphAgentProcessingContract {
    pub fn graph_form(&self) -> &GraphFormCompilation {
        &self.graph_form
    }

    pub fn task_frame(&self) -> &TaskFrame<GraphExecutionIntent> {
        &self.task_frame
    }

    pub fn synthetic_moments(&self) -> &[GraphSyntheticMoment] {
        &self.synthetic_moments
    }

    pub fn empirical_outflows(&self) -> &[GraphEmpiricalOutflowAspect] {
        &self.empirical_outflows
    }

    pub fn transmission_spec(&self) -> &GraphGdslTransmissionSpec {
        &self.transmission_spec
    }

    pub fn daemon_route(&self) -> GraphTaskDaemonRoute {
        GraphTaskDaemonRoute::from_transmission(self.transmission_spec())
    }

    pub fn into_task_daemon_submission(
        self,
        owner: impl Into<String>,
    ) -> GraphTaskDaemonSubmission {
        let route = GraphTaskDaemonRoute::from_transmission(&self.transmission_spec);
        GraphTaskDaemonSubmission {
            owner: owner.into(),
            route,
            task_frame: self.task_frame,
            transmission_spec: self.transmission_spec,
        }
    }

    pub fn into_parts(
        self,
    ) -> (
        GraphFormCompilation,
        TaskFrame<GraphExecutionIntent>,
        Vec<GraphSyntheticMoment>,
        Vec<GraphEmpiricalOutflowAspect>,
        GraphGdslTransmissionSpec,
    ) {
        (
            self.graph_form,
            self.task_frame,
            self.synthetic_moments,
            self.empirical_outflows,
            self.transmission_spec,
        )
    }
}

impl GraphExecutionIntent {
    pub fn store(&self) -> &SharedGraphStore {
        &self.store
    }

    pub fn view_spec(&self) -> &GraphViewSpec {
        self.reciprocity.view_spec()
    }

    pub fn program(&self) -> &ShellComponentPlan {
        self.reciprocity.shell_plan()
    }

    pub fn reciprocity(&self) -> &GraphFramePureFormReciprocity {
        &self.reciprocity
    }

    pub fn objective(&self) -> &TaskObjectiveRef {
        &self.objective
    }

    pub fn return_contract(&self) -> &TaskReturnContract {
        &self.return_contract
    }

    pub fn compute_steps(&self) -> &[String] {
        &self.compute_steps
    }

    pub fn estimated_volume(&self) -> usize {
        self.estimated_volume
    }

    pub fn execution_kinds(&self) -> Vec<ShellComponentExecutionKind> {
        self.reciprocity.shell_plan().execution_kinds()
    }

    pub fn has_algorithm_components(&self) -> bool {
        self.reciprocity.shell_plan().has_algorithm_components()
    }

    pub fn has_pipeline_components(&self) -> bool {
        self.reciprocity.shell_plan().has_pipeline_components()
    }
}

impl GraphFramePureFormReciprocity {
    pub fn view_spec(&self) -> &GraphViewSpec {
        &self.view_spec
    }

    pub fn shell_plan(&self) -> &ShellComponentPlan {
        &self.shell_plan
    }

    pub fn shell_pipeline(&self) -> &ShellPipelineFacade {
        &self.shell_pipeline
    }

    pub fn pure_form_return(&self) -> ShellPureFormReturn {
        self.shell_pipeline.pure_form_return()
    }
}

#[derive(Clone)]
pub struct GraphFramePlan {
    store: SharedGraphStore,
    source_spec: GraphViewSpec,
    expressions: Vec<GraphFrameExpr>,
}

impl GraphFramePlan {
    pub fn new(store: SharedGraphStore, view_spec: GraphViewSpec) -> Self {
        Self {
            store,
            source_spec: view_spec,
            expressions: Vec::new(),
        }
    }

    pub fn from_store<Store>(store: Arc<Store>) -> Self
    where
        Store: crate::types::graph_store::GraphStoreRead + 'static,
    {
        let store: SharedGraphStore = store;
        Self::new(store, GraphViewSpec::new())
    }

    pub fn store(&self) -> &SharedGraphStore {
        &self.store
    }

    pub fn view_spec(&self) -> &GraphViewSpec {
        &self.source_spec
    }

    pub fn expressions(&self) -> &[GraphFrameExpr] {
        &self.expressions
    }

    pub fn push_expr(mut self, expr: impl Into<GraphFrameExpr>) -> Self {
        self.expressions.push(expr.into());
        self
    }

    pub fn orient(self, orientation: Orientation) -> Self {
        self.push_expr(GraphViewExpr::Orientation(orientation))
    }

    pub fn select_relationship_types(self, relationship_types: HashSet<RelationshipType>) -> Self {
        self.push_expr(GraphViewExpr::RelationshipTypes(relationship_types))
    }

    pub fn select_relationship_type(self, relationship_type: RelationshipType) -> Self {
        let mut relationship_types = self.compile_view_spec().relationship_types().clone();
        relationship_types.insert(relationship_type);
        self.select_relationship_types(relationship_types)
    }

    pub fn select_relationship_property(
        self,
        relationship_type: RelationshipType,
        property_key: impl Into<String>,
    ) -> Self {
        self.push_expr(GraphViewExpr::RelationshipProperty {
            relationship_type,
            property_key: property_key.into(),
        })
    }

    pub fn procedure(self, procedure: GraphProcedureExpr) -> Self {
        self.push_expr(procedure)
    }

    pub fn call(self, component: impl Into<String>, mode: ShellComponentMode) -> Self {
        self.procedure(GraphProcedureExpr::new(component, mode))
    }

    pub fn agent_plan(self, plan: &GraphAgentPlanInterface) -> Self {
        self.procedure(plan.to_graph_procedure_expr())
    }

    pub fn compile_view_spec(&self) -> GraphViewSpec {
        let mut spec = self.source_spec.clone();

        for expression in &self.expressions {
            let GraphFrameExpr::View(expression) = expression else {
                continue;
            };
            spec = apply_view_expr(&spec, expression);
        }

        spec
    }

    pub fn compile_rational_semantics(
        &self,
        grammar: &GraphFeatureGrammarChecked,
    ) -> Result<GraphSemanticLowering, GraphRationalLanguageError> {
        lower_graph_semantics(&self.expressions, grammar)
    }

    pub fn compile_rational_program(
        &self,
        grammar: &GraphFeatureGrammarChecked,
    ) -> Result<GraphSemanticProgram, GraphRationalLanguageError> {
        lower_graph_semantic_program(&self.expressions, grammar)
    }

    pub fn compile_graph_form(
        &self,
        grammar: &GraphFeatureGrammarChecked,
    ) -> Result<GraphFormCompilation, GraphFormError> {
        compile_graph_form(&self.expressions, grammar)
    }

    pub fn compile_pure_shell_plan(&self) -> Result<ShellComponentPlan, GraphFrameError> {
        let mut plan = ShellComponentPlan::new(pure_form_shell_address());

        for expression in &self.expressions {
            let GraphFrameExpr::Procedure(procedure) = expression else {
                continue;
            };
            let mut builder = plan.component(procedure.component(), procedure.mode())?;
            for (key, value) in procedure.inputs() {
                builder = builder.with_input(key, value.clone());
            }
            plan = builder.finish();
        }

        Ok(plan)
    }

    pub fn compile_pure_form_reciprocity(
        &self,
    ) -> Result<GraphFramePureFormReciprocity, GraphFrameError> {
        let view_spec = self.compile_view_spec();
        let shell_plan = self.compile_pure_shell_plan()?;

        let mut descriptor = ShellPipelineDescriptor::new(pure_form_shell_address());
        if self.has_view_expressions() {
            descriptor = descriptor.with_immediate_body();
        }
        if self.has_procedure_expressions() {
            descriptor = descriptor.with_mediated_body();
        }

        Ok(GraphFramePureFormReciprocity {
            view_spec,
            shell_plan,
            shell_pipeline: ShellPipelineFacade::new(descriptor),
        })
    }

    pub fn compile_execution_intent(&self) -> Result<GraphExecutionIntent, GraphFrameError> {
        let reciprocity = self.compile_pure_form_reciprocity()?;
        let objective =
            TaskObjectiveRef::new("graphstore", graph_view_identity(reciprocity.view_spec()));
        let persisted = self.expressions.iter().any(|expression| {
            matches!(
                expression,
                GraphFrameExpr::Procedure(procedure)
                    if matches!(
                        procedure.mode(),
                        ShellComponentMode::Mutate | ShellComponentMode::Write
                    )
            )
        });
        let return_contract = if persisted {
            TaskReturnContract::persisted(vec![
                "graphframe.dataset.graph".to_string(),
                "graphframe.dataset.node".to_string(),
                "graphframe.dataset.relationship".to_string(),
            ])
        } else {
            TaskReturnContract::ephemeral(vec!["graphframe.compute.result".to_string()])
        };
        let estimated_volume = self
            .expressions
            .iter()
            .filter(|expression| matches!(expression, GraphFrameExpr::Procedure(_)))
            .count()
            .max(1);

        Ok(GraphExecutionIntent {
            store: Arc::clone(&self.store),
            reciprocity,
            objective,
            return_contract,
            compute_steps: self.task_frame_steps(),
            estimated_volume,
        })
    }

    pub fn compile_task_frame_plan(
        &self,
        concurrency: usize,
    ) -> Result<TaskWorkflow, GraphFrameError> {
        Ok(self.compile_task_frame(concurrency)?.workflow().clone())
    }

    pub fn compile_task_frame(
        &self,
        concurrency: usize,
    ) -> Result<TaskFrame<GraphExecutionIntent>, GraphFrameError> {
        let intent = self.compile_execution_intent()?;
        Ok(TaskFrame::from_graph_intent(
            intent,
            TaskFramePolicy::new(concurrency),
        )?)
    }

    pub fn compile_task_spec(&self, concurrency: usize) -> Result<TaskSpec, GraphFrameError> {
        Ok(self.compile_task_frame(concurrency)?.into_spec()?)
    }

    pub fn compile_agent_task_frame(
        &self,
        profile: GraphAutomationProfile,
        concurrency: usize,
    ) -> Result<TaskFrame<GraphExecutionIntent>, GraphFormError> {
        let intent = self.compile_execution_intent()?;
        validate_automation_shell_plan(intent.program(), profile)?;
        Ok(TaskFrame::from_graph_intent(
            intent,
            TaskFramePolicy::new(concurrency),
        )?)
    }

    pub fn compile_agent_processing_contract(
        &self,
        grammar: &GraphFeatureGrammarChecked,
        profile: GraphAutomationProfile,
        concurrency: usize,
    ) -> Result<GraphAgentProcessingContract, GraphFormError> {
        let graph_form = self.compile_graph_form(grammar)?;
        let task_frame = self.compile_agent_task_frame(profile, concurrency)?;
        let empirical_outflows =
            derive_empirical_outflows(task_frame.program().program(), task_frame.return_contract());
        let synthetic_moments = synthetic_moments().to_vec();
        let transmission_spec =
            build_gdsl_transmission_spec(&task_frame, &synthetic_moments, &empirical_outflows);
        Ok(GraphAgentProcessingContract {
            graph_form,
            task_frame,
            synthetic_moments,
            empirical_outflows,
            transmission_spec,
        })
    }

    pub fn compile_job(
        &self,
        owner: impl Into<String>,
        concurrency: usize,
    ) -> Result<TaskJob<GraphExecutionIntent>, GraphFrameError> {
        Ok(self.compile_task_frame(concurrency)?.into_job(owner)?)
    }

    pub fn collect(self) -> Result<GraphFrame, GraphFrameError> {
        let view_spec = self.compile_view_spec();
        GraphFrame::with_shared_view_spec(self.store, view_spec)
    }

    fn has_view_expressions(&self) -> bool {
        self.expressions
            .iter()
            .any(|expr| matches!(expr, GraphFrameExpr::View(_)))
    }

    fn has_procedure_expressions(&self) -> bool {
        self.expressions
            .iter()
            .any(|expr| matches!(expr, GraphFrameExpr::Procedure(_)))
    }

    fn task_frame_steps(&self) -> Vec<String> {
        if self.expressions.is_empty() {
            return vec!["graphframe.noop".to_string()];
        }

        self.expressions
            .iter()
            .map(|expression| match expression {
                GraphFrameExpr::View(GraphViewExpr::RelationshipTypes(_)) => {
                    "graphframe.view.relationship_types".to_string()
                }
                GraphFrameExpr::View(GraphViewExpr::RelationshipProperty { .. }) => {
                    "graphframe.view.relationship_property".to_string()
                }
                GraphFrameExpr::View(GraphViewExpr::Orientation(_)) => {
                    "graphframe.view.orientation".to_string()
                }
                GraphFrameExpr::Procedure(procedure) => {
                    format!("graphframe.procedure.{}", procedure.component())
                }
                GraphFrameExpr::FeatureGrammar(_) => "graphframe.grammar.bind".to_string(),
                GraphFrameExpr::Model(_) => "graphframe.model.bind".to_string(),
                GraphFrameExpr::Plan(_) => "graphframe.plan.bind".to_string(),
            })
            .collect()
    }
}

fn apply_view_expr(current: &GraphViewSpec, expression: &GraphViewExpr) -> GraphViewSpec {
    match expression {
        GraphViewExpr::RelationshipTypes(relationship_types) => {
            build_spec(current, Some(relationship_types.clone()), None, None)
        }
        GraphViewExpr::RelationshipProperty {
            relationship_type,
            property_key,
        } => {
            let mut selectors = current.relationship_property_selectors().clone();
            selectors.insert(relationship_type.clone(), property_key.clone());
            build_spec(current, None, Some(selectors), None)
        }
        GraphViewExpr::Orientation(orientation) => {
            build_spec(current, None, None, Some(*orientation))
        }
    }
}

fn pure_form_shell_address() -> ShellAddress {
    ShellAddress::new(
        ShellRegister::Unified,
        ShellPipeline::ModelFeaturePlan,
        ShellAlgebra::ProgramFeature,
    )
}

fn graph_view_identity(view_spec: &GraphViewSpec) -> String {
    let mut relationship_types = view_spec
        .relationship_types()
        .iter()
        .map(|relationship_type| relationship_type.name())
        .collect::<Vec<_>>();
    relationship_types.sort_unstable();

    let mut property_selectors = view_spec
        .relationship_property_selectors()
        .iter()
        .map(|(relationship_type, property_key)| {
            format!("{}:{property_key}", relationship_type.name())
        })
        .collect::<Vec<_>>();
    property_selectors.sort_unstable();

    format!(
        "graphframe.view::types=[{}];properties=[{}];orientation={}",
        relationship_types.join(","),
        property_selectors.join(","),
        view_spec.orientation().as_str(),
    )
}

fn build_spec(
    current: &GraphViewSpec,
    relationship_types: Option<HashSet<RelationshipType>>,
    relationship_property_selectors: Option<HashMap<RelationshipType, String>>,
    orientation: Option<Orientation>,
) -> GraphViewSpec {
    let relationship_types =
        relationship_types.unwrap_or_else(|| current.relationship_types().clone());
    let relationship_property_selectors = relationship_property_selectors
        .unwrap_or_else(|| current.relationship_property_selectors().clone());
    let orientation = orientation.unwrap_or_else(|| current.orientation());

    GraphViewSpec::new()
        .with_relationship_types(relationship_types)
        .with_relationship_property_selectors(relationship_property_selectors)
        .with_orientation(orientation)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::collections::graphframe::feature_grammar::validate_graph_feature_grammar;
    use crate::collections::graphframe::feature_grammar::GraphFeatureCardinality;
    use crate::collections::graphframe::feature_grammar::GraphFeatureGrammarForm;
    use crate::collections::graphframe::feature_grammar::GraphFeatureRule;
    use crate::collections::graphframe::feature_grammar::GraphFeatureStratum;
    use crate::collections::graphframe::feature_grammar::GraphFeatureValueType;
    use crate::collections::graphframe::frame::GraphFrame;
    use crate::collections::graphframe::graph_form::GraphAutomationProfile;
    use crate::collections::graphframe::graph_form::GraphExecutionSurface;
    use crate::collections::graphframe::graph_form::GraphExecutionSurfaceMode;
    use crate::collections::graphframe::graph_form::GraphOutflowChannel;
    use crate::collections::graphframe::graph_form::GraphSyntheticMoment;
    use crate::collections::graphframe::graph_form::GraphTransmissionTarget;
    use crate::collections::graphframe::model::GraphFrameModelExt;
    use crate::collections::graphframe::plan::GraphFramePlanExt;
    use crate::collections::graphframe::GraphProcedureExpr;
    use crate::shell::ShellComponentMode;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    use super::GraphTaskDaemonRoute;

    fn deterministic_frame() -> GraphFrame {
        let store = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::seeded(21))
                .expect("seeded graph store should build"),
        );
        GraphFrame::from_store(store).expect("graph frame should build")
    }

    fn density_grammar(
    ) -> crate::collections::graphframe::feature_grammar::GraphFeatureGrammarChecked {
        validate_graph_feature_grammar(
            GraphFeatureGrammarForm::new("graph_theory", "v1").with_feature_rule(
                GraphFeatureRule::new(
                    GraphFeatureStratum::Graph,
                    "density",
                    GraphFeatureValueType::Scalar,
                    true,
                    GraphFeatureCardinality::One,
                ),
            ),
        )
        .expect("density grammar should be valid")
    }

    #[test]
    fn agent_processing_contract_binds_graph_form_and_task_frame() {
        let plan = deterministic_frame()
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("pagerank", ShellComponentMode::Stats);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentAnalytics,
                2,
            )
            .expect("agent processing contract should compile");

        assert!(contract.graph_form().compilation().entrypoints.len() > 0);
        assert_eq!(contract.task_frame().workflow().len(), 2);
        assert!(contract
            .synthetic_moments()
            .contains(&GraphSyntheticMoment::Workflow));
        assert!(contract
            .empirical_outflows()
            .iter()
            .any(|aspect| aspect.channel == GraphOutflowChannel::Output));
        assert_eq!(
            contract.transmission_spec().target,
            GraphTransmissionTarget::TaskDaemon
        );
        assert_eq!(contract.transmission_spec().namespace, "graphframe");
        assert_eq!(
            contract
                .transmission_spec()
                .primary_execution_surface_kind(),
            Some(GraphExecutionSurface::Algorithm)
        );
        assert_eq!(
            contract.transmission_spec().execution_surface_mode_kind(),
            GraphExecutionSurfaceMode::Single
        );
        assert!(!contract.transmission_spec().routes_via_store_api());
        assert_eq!(contract.daemon_route(), GraphTaskDaemonRoute::Algorithm);
    }

    #[test]
    fn store_api_contract_maps_to_store_route_and_submission_job() {
        let plan = deterministic_frame()
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("drop_graph", ShellComponentMode::Mutate);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentHybrid,
                1,
            )
            .expect("store api contract should compile");

        assert_eq!(contract.daemon_route(), GraphTaskDaemonRoute::StoreApi);

        let submission = contract.into_task_daemon_submission("organon");
        assert_eq!(submission.owner(), "organon");
        assert_eq!(submission.route(), GraphTaskDaemonRoute::StoreApi);
        assert!(submission.transmission_spec().routes_via_store_api());

        let job = submission
            .into_job()
            .expect("submission should produce a task job");
        assert_eq!(job.owner(), "organon");
    }

    #[test]
    fn hybrid_contract_maps_to_hybrid_route() {
        let plan = deterministic_frame()
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("pagerank", ShellComponentMode::Stats)
            .call("drop_graph", ShellComponentMode::Mutate);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentHybrid,
                2,
            )
            .expect("hybrid contract should compile");

        assert_eq!(contract.daemon_route(), GraphTaskDaemonRoute::Hybrid);
    }

    #[test]
    fn agent_analytics_profile_rejects_pipeline_invoke_components() {
        let plan = deterministic_frame().procedure(GraphProcedureExpr::new(
            "create_node_classification_pipeline",
            ShellComponentMode::Invoke,
        ));

        assert!(matches!(
            plan.compile_agent_task_frame(GraphAutomationProfile::AgentAnalytics, 1),
            Err(crate::collections::graphframe::graph_form::GraphFormError::InvalidAutomationContract(_))
        ));
    }

    #[test]
    fn graph_store_agent_plan_lowers_into_task_frame_shell_program() {
        use crate::collections::graphframe::agent_plan::GraphAgentPlanInterface;

        let action = GraphAgentPlanInterface::classify("graph_store_catalog", "dropGraph")
            .expect("dropGraph action should classify");
        let plan = deterministic_frame().plan().agent_plan(&action);
        let task_frame = plan
            .compile_task_frame(1)
            .expect("task frame should compile for store action");

        assert_eq!(task_frame.workflow().len(), 3);
        assert!(task_frame.return_contract().requires_persistence());
        assert!(task_frame.program().program().has_store_api_components());
        assert_eq!(
            task_frame.program().program().calls()[0].component.as_str(),
            "gds.store.catalog.drop"
        );
    }
}
