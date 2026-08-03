//! GraphFrame core surface.
//!
//! GraphFrame is an executable, immutable view over a GraphStore.

use std::collections::HashSet;
use std::sync::Arc;

use crate::collections::graphframe::expr::GraphProcedureExpr;
use crate::collections::graphframe::lazy::GraphFramePlan;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::shell::ShellComponentPlanError;
use crate::types::graph::Graph;
use crate::types::graph_store::GraphStoreRead;
use crate::types::graph_store::GraphViewError;
use crate::types::graph_store::GraphViewSpec;

pub type SharedGraphStore = Arc<dyn GraphStoreRead>;

#[derive(Debug, thiserror::Error)]
pub enum GraphFrameError {
    #[error("graph view construction failed: {0}")]
    View(#[from] GraphViewError),

    #[error("PureShell plan compilation failed: {0}")]
    PureShell(#[from] ShellComponentPlanError),

    #[error(transparent)]
    TaskSpec(#[from] crate::task::spec::TaskSpecError),
}

#[derive(Clone)]
pub struct GraphFrame {
    store: SharedGraphStore,
    view_spec: GraphViewSpec,
}

impl GraphFrame {
    pub fn from_store<Store>(store: Arc<Store>) -> Result<Self, GraphFrameError>
    where
        Store: GraphStoreRead + 'static,
    {
        Self::with_view_spec(store, GraphViewSpec::new())
    }

    pub fn new(store: SharedGraphStore) -> Result<Self, GraphFrameError> {
        Self::with_shared_view_spec(store, GraphViewSpec::new())
    }

    pub fn with_view_spec<Store>(
        store: Arc<Store>,
        view_spec: GraphViewSpec,
    ) -> Result<Self, GraphFrameError>
    where
        Store: GraphStoreRead + 'static,
    {
        let store: SharedGraphStore = store;
        Self::with_shared_view_spec(store, view_spec)
    }

    pub fn with_shared_view_spec(
        store: SharedGraphStore,
        view_spec: GraphViewSpec,
    ) -> Result<Self, GraphFrameError> {
        // Validate once at construction time so the frame is always a coherent
        // executable graph view.
        store.get_graph_view(&view_spec)?;
        Ok(Self { store, view_spec })
    }

    pub fn store(&self) -> &SharedGraphStore {
        &self.store
    }

    pub fn view_spec(&self) -> &GraphViewSpec {
        &self.view_spec
    }

    pub fn graph(&self) -> Result<Arc<dyn Graph>, GraphFrameError> {
        Ok(self.store.get_graph_view(&self.view_spec)?)
    }

    pub fn node_count(&self) -> Result<usize, GraphFrameError> {
        Ok(self.graph()?.node_count())
    }

    pub fn relationship_count(&self) -> Result<usize, GraphFrameError> {
        Ok(self.graph()?.relationship_count())
    }

    pub fn relationship_types(&self) -> HashSet<RelationshipType> {
        if self.view_spec.relationship_types().is_empty() {
            self.store.relationship_types()
        } else {
            self.view_spec.relationship_types().clone()
        }
    }

    pub fn plan(&self) -> GraphFramePlan {
        GraphFramePlan::new(Arc::clone(&self.store), self.view_spec.clone())
    }

    pub fn lazy(&self) -> GraphFramePlan {
        self.plan()
    }

    pub fn orient(&self, orientation: Orientation) -> GraphFramePlan {
        self.plan().orient(orientation)
    }

    pub fn select_relationship_types(
        &self,
        relationship_types: HashSet<RelationshipType>,
    ) -> GraphFramePlan {
        self.plan().select_relationship_types(relationship_types)
    }

    pub fn select_relationship_type(&self, relationship_type: RelationshipType) -> GraphFramePlan {
        self.plan().select_relationship_type(relationship_type)
    }

    pub fn select_relationship_property(
        &self,
        relationship_type: RelationshipType,
        property_key: impl Into<String>,
    ) -> GraphFramePlan {
        self.plan()
            .select_relationship_property(relationship_type, property_key)
    }

    pub fn procedure(&self, procedure: GraphProcedureExpr) -> GraphFramePlan {
        self.plan().procedure(procedure)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::Arc;

    use crate::projection::RelationshipType;
    use crate::shell::ShellComponentMode;
    use crate::task::runtime::TaskFrameKind;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::graph_store::GraphStore;
    use crate::types::random::RandomGraphConfig;

    use crate::collections::graphframe::expr::GraphProcedureExpr;

    use super::GraphFrame;

    fn random_store() -> Arc<DefaultGraphStore> {
        Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::seeded(21))
                .expect("seeded random graph store should build"),
        )
    }

    #[test]
    fn graphframe_builds_and_exposes_counts() {
        let store = random_store();
        let frame = GraphFrame::from_store(Arc::clone(&store)).expect("frame should build");

        assert_eq!(frame.node_count().expect("node count"), store.node_count());
        assert_eq!(
            frame.relationship_count().expect("relationship count"),
            store.relationship_count()
        );
    }

    #[test]
    fn graphframe_plan_selects_relationship_types() {
        let store = random_store();
        let frame = GraphFrame::from_store(Arc::clone(&store)).expect("frame should build");

        let first_type = store
            .relationship_types()
            .iter()
            .next()
            .cloned()
            .unwrap_or_else(|| RelationshipType::of("RELATES"));
        let mut selected = HashSet::new();
        selected.insert(first_type.clone());

        let selected_frame = frame
            .select_relationship_types(selected)
            .collect()
            .expect("selected frame should build");

        assert!(selected_frame.relationship_types().contains(&first_type));
        assert_eq!(selected_frame.relationship_types().len(), 1);
    }

    #[test]
    fn graphframe_plan_compiles_procedures_to_pure_shell() {
        let frame = GraphFrame::from_store(random_store()).expect("frame should build");
        let plan = frame.procedure(
            GraphProcedureExpr::new("pagerank", ShellComponentMode::Stream)
                .with_input("maxIterations", 20),
        );

        let shell_plan = plan
            .compile_pure_shell_plan()
            .expect("PureShell plan should compile");

        assert_eq!(shell_plan.len(), 1);
        assert_eq!(
            shell_plan.calls()[0].component.as_str(),
            "gds.algorithms.centrality.pagerank"
        );
        assert_eq!(shell_plan.calls()[0].mode, ShellComponentMode::Stream);
        assert_eq!(shell_plan.calls()[0].inputs["maxIterations"], 20);
    }

    #[test]
    fn graphframe_view_and_procedure_expressions_remain_separate() {
        let frame = GraphFrame::from_store(random_store()).expect("frame should build");
        let relationship_type = RelationshipType::of("RELATES");
        let plan = frame
            .select_relationship_type(relationship_type.clone())
            .procedure(GraphProcedureExpr::new(
                "pagerank",
                ShellComponentMode::Stats,
            ));

        let view_spec = plan.compile_view_spec();
        let shell_plan = plan
            .compile_pure_shell_plan()
            .expect("PureShell plan should compile");

        assert!(view_spec.relationship_types().contains(&relationship_type));
        assert_eq!(shell_plan.len(), 1);
    }

    #[test]
    fn graphframe_objective_identity_distinguishes_selected_views() {
        let frame = GraphFrame::from_store(random_store()).expect("frame should build");
        let follows = frame
            .select_relationship_type(RelationshipType::of("FOLLOWS"))
            .compile_execution_intent()
            .expect("first intent should compile");
        let knows = frame
            .select_relationship_type(RelationshipType::of("KNOWS"))
            .compile_execution_intent()
            .expect("second intent should compile");

        assert_ne!(follows.objective().identity(), knows.objective().identity());
    }

    #[test]
    fn graphframe_compiles_pure_form_reciprocity() {
        let frame = GraphFrame::from_store(random_store()).expect("frame should build");
        let reciprocity = frame
            .select_relationship_type(RelationshipType::of("RELATES"))
            .procedure(GraphProcedureExpr::new(
                "pagerank",
                ShellComponentMode::Stats,
            ))
            .compile_pure_form_reciprocity()
            .expect("pure form reciprocity should compile");

        assert_eq!(reciprocity.shell_plan().len(), 1);
        assert!(
            reciprocity
                .pure_form_return()
                .principle()
                .context
                .dependencies
                .iter()
                .any(|dep| dep == "dataframe")
                || reciprocity
                    .pure_form_return()
                    .principle()
                    .context
                    .dependencies
                    .iter()
                    .any(|dep| dep == "dataset")
        );
    }

    #[test]
    fn graphframe_compiles_task_frame_reciprocity_plan() {
        let frame = GraphFrame::from_store(random_store()).expect("frame should build");
        let task_plan = frame
            .select_relationship_property(RelationshipType::of("RELATES"), "weight")
            .procedure(GraphProcedureExpr::new(
                "pagerank",
                ShellComponentMode::Stream,
            ))
            .compile_task_frame_plan(2)
            .expect("task frame plan should compile");
        let task_frames = task_plan.frames();

        assert_eq!(task_frames.len(), 2);
        assert_eq!(
            task_frames[0].image_spec().kind(),
            TaskFrameKind::ShellProgram
        );
        assert_eq!(
            task_frames[1].image_spec().kind(),
            TaskFrameKind::GraphAlgorithm
        );
    }

    #[test]
    fn graphframe_compiles_task_job_with_procedure_program() {
        let frame = GraphFrame::from_store(random_store()).expect("frame should build");
        let job = frame
            .select_relationship_type(RelationshipType::of("RELATES"))
            .procedure(GraphProcedureExpr::new(
                "pagerank",
                ShellComponentMode::Stats,
            ))
            .compile_job("organon", 2)
            .expect("task job should compile");

        assert_eq!(job.owner(), "organon");
        assert_eq!(job.spec().namespace(), "graphframe");
        assert_eq!(job.spec().workflow().len(), 2);
        assert_eq!(job.spec().objective().source(), "graphstore");
        assert!(!job.spec().return_contract().requires_persistence());
        assert_eq!(job.program().program().len(), 1);
        assert!(job
            .program()
            .view_spec()
            .relationship_types()
            .contains(&RelationshipType::of("RELATES")));
    }

    #[test]
    fn graphframe_write_job_explicitly_requests_persistence() {
        let frame = GraphFrame::from_store(random_store()).expect("frame should build");
        let job = frame
            .select_relationship_type(RelationshipType::of("RELATES"))
            .procedure(GraphProcedureExpr::new(
                "pagerank",
                ShellComponentMode::Write,
            ))
            .compile_job("organon", 2)
            .expect("write task job should compile");

        assert_eq!(job.spec().workflow().len(), 3);
        assert!(job.spec().return_contract().requires_persistence());
        assert_eq!(
            job.spec().workflow().frames()[2].image_spec().kind(),
            TaskFrameKind::ProcedurePipeline
        );
    }
}
