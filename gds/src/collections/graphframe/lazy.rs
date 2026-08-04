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

use crate::collections::graphframe::expr::GraphFrameExpr;
use crate::collections::graphframe::expr::GraphProcedureExpr;
use crate::collections::graphframe::expr::GraphViewExpr;
use crate::collections::graphframe::feature_grammar::GraphFeatureGrammarChecked;
use crate::collections::graphframe::frame::GraphFrame;
use crate::collections::graphframe::frame::GraphFrameError;
use crate::collections::graphframe::frame::SharedGraphStore;
use crate::collections::graphframe::rational_language::lower_graph_semantics;
use crate::collections::graphframe::rational_language::GraphRationalLanguageError;
use crate::collections::graphframe::rational_language::GraphSemanticLowering;
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
